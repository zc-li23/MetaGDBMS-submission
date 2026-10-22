//! Benchmark v2 runner: the MetaGDB Core Language Profile.
//!
//! Benchmark data lives outside implementation crates, so candidate-authored
//! tests cannot redefine the expected behavior. The runner is language
//! agnostic: it drives whatever Parser/Planner/Optimizer/Executor the active
//! implementation crate assembled and only depends on the public pipeline.
//!
//! Each case gets
//!
//! * a freshly created database directory, so a failed mutation cannot leak
//!   into a later case,
//! * its own fixture setup and per-case setup statements,
//! * an independent wall-clock budget, enforced by running the whole case in a
//!   child process that the parent can terminate without leaking work.
//!
//! Two on-disk formats are supported:
//!
//! * `core.jsonl` — Benchmark v2 (typed expectations, fixtures, per-case setup,
//!   negative cases, postconditions). This is the graded Core Profile.
//! * `setup.txt` + `cases.tsv` — the older v1 format. The runner can read an
//!   externally supplied v1 dataset, but no v1 dataset is included here.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use metagdb_core::{Error, Result};

use metagdb::assembly;
use metagdb::pipeline::Runtime;

use crate::canon::{self, Val};
use crate::json::Json;

const DEFAULT_TIMEOUT_MS: u64 = 5_000;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

pub struct Config {
    /// Directory holding `core.jsonl` (v2) or `setup.txt` + `cases.tsv` (v1).
    pub dataset_dir: PathBuf,
    /// Fixture catalogue; defaults to `<dataset_dir>/../fixtures.jsonl`.
    pub fixtures_path: Option<PathBuf>,
    pub min_pass_rate: f64,
    /// Overrides every case-level budget when greater than zero.
    pub timeout_ms: u64,
    pub output_dir: PathBuf,
    /// Only run cases whose id contains this substring.
    pub id_filter: Option<String>,
    /// Only run cases carrying one of these capability tags.
    pub capabilities: Vec<String>,
    /// Also require this fraction of cases to execute without any failure.
    pub min_execution_rate: f64,
    /// Reject cases naming an unknown fixture.
    pub strict_fixtures: bool,
    /// Run only cases whose position satisfies `index % shards == shard`.
    pub shard: usize,
    pub shards: usize,
    /// Skip cases already present in the output journal and append to it.
    ///
    /// An implementation can abort the whole process (a parser that overflows
    /// the stack, for example) rather than returning an error, so progress is
    /// journalled after every case and a run can be resumed instead of lost.
    pub resume: bool,
    pub quiet: bool,
}

// ---------------------------------------------------------------------------
// Case model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Parse,
    Plan,
    Optimize,
    Execute,
    /// Failure class the implementation did not attribute to a pipeline stage.
    Unknown,
}

impl Stage {
    fn from_error(error: &Error) -> Stage {
        match error {
            Error::Parse(_) | Error::InvalidQuery(_) => Stage::Parse,
            Error::Plan(_) | Error::Schema(_) | Error::NotFound(_) | Error::Unsupported(_) => {
                Stage::Plan
            }
            Error::Optimize(_) => Stage::Optimize,
            Error::Execute(_) | Error::Transaction(_) | Error::Storage(_) => Stage::Execute,
            Error::Other(_) => Stage::Unknown,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Stage::Parse => "parse",
            Stage::Plan => "plan",
            Stage::Optimize => "optimize",
            Stage::Execute => "execute",
            Stage::Unknown => "unknown",
        }
    }

    fn parse(text: &str) -> Option<Stage> {
        match text {
            "parse" => Some(Stage::Parse),
            "plan" => Some(Stage::Plan),
            "optimize" => Some(Stage::Optimize),
            "execute" => Some(Stage::Execute),
            "unknown" => Some(Stage::Unknown),
            _ => None,
        }
    }

    /// True when `expected` accepts an observed failure at `self`.
    fn matches(self, expected: &str) -> bool {
        expected == "any" || Stage::parse(expected) == Some(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pass,
    /// A shape-only oracle matched. Diagnostic evidence, never a semantic pass.
    ShapeMatch,
    SetupFailure,
    ParseFailure,
    PlanFailure,
    OptimizeFailure,
    ExecuteFailure,
    Timeout,
    Mismatch,
    /// A negative case that unexpectedly succeeded.
    UnexpectedSuccess,
    /// The mutation itself succeeded but its follow-up state check did not.
    PostconditionFailure,
    /// The profile cannot normalize this case; excluded from the pass rate.
    Unscored,
    /// The case needs a capability the entry point does not provide.
    Inapplicable,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Pass => "PASS",
            Status::ShapeMatch => "SHAPE_MATCH",
            Status::SetupFailure => "SETUP_FAILURE",
            Status::ParseFailure => "PARSE_FAILURE",
            Status::PlanFailure => "PLAN_FAILURE",
            Status::OptimizeFailure => "OPTIMIZE_FAILURE",
            Status::ExecuteFailure => "EXECUTE_FAILURE",
            Status::Timeout => "TIMEOUT",
            Status::Mismatch => "MISMATCH",
            Status::UnexpectedSuccess => "UNEXPECTED_SUCCESS",
            Status::PostconditionFailure => "POSTCONDITION_FAILURE",
            Status::Unscored => "UNSCORED",
            Status::Inapplicable => "INAPPLICABLE",
        }
    }

    /// Statuses that mean "the statement ran and produced the right answer".
    pub fn is_pass(&self) -> bool {
        matches!(self, Status::Pass)
    }

    /// Statuses that mean "the statement ran at all".
    pub fn is_executed(&self) -> bool {
        !matches!(
            self,
            Status::SetupFailure
                | Status::ParseFailure
                | Status::PlanFailure
                | Status::OptimizeFailure
                | Status::ExecuteFailure
                | Status::Timeout
                | Status::Inapplicable
        )
    }

    fn parse(label: &str) -> Option<Self> {
        Some(match label {
            "PASS" => Status::Pass,
            "SHAPE_MATCH" => Status::ShapeMatch,
            "SETUP_FAILURE" => Status::SetupFailure,
            "PARSE_FAILURE" => Status::ParseFailure,
            "PLAN_FAILURE" => Status::PlanFailure,
            "OPTIMIZE_FAILURE" => Status::OptimizeFailure,
            "EXECUTE_FAILURE" => Status::ExecuteFailure,
            "TIMEOUT" => Status::Timeout,
            "MISMATCH" => Status::Mismatch,
            "UNEXPECTED_SUCCESS" => Status::UnexpectedSuccess,
            "POSTCONDITION_FAILURE" => Status::PostconditionFailure,
            "UNSCORED" => Status::Unscored,
            "INAPPLICABLE" => Status::Inapplicable,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Expectation {
    Rows {
        rows: Vec<Vec<Val>>,
        columns: Vec<String>,
    },
    Error {
        stage: String,
        pattern: Option<String>,
    },
    /// v1 string expectation, compared against the legacy canonical rendering.
    LegacyText(String),
}

#[derive(Debug, Clone)]
pub struct Case {
    pub id: String,
    pub language: String,
    pub category: String,
    pub features: Vec<String>,
    pub capabilities: Vec<String>,
    pub source_kind: String,
    pub source_detail: String,
    pub fixture: String,
    pub setup: Vec<String>,
    pub query: String,
    pub ordered: bool,
    pub timeout_ms: u64,
    pub scored: bool,
    pub applicable: bool,
    pub inapplicable_reason: Option<String>,
    /// `exact` compares values; `shape` only compares row count and arity.
    pub comparison: String,
    /// TinkerPop's `action=next`: the traversal is iterated once, so only the
    /// first result is observed.
    pub first_only: bool,
    /// Set while resuming a run whose process died on this case.
    pub aborted: bool,
    pub expectation: Expectation,
    /// Follow-up reads that verify the graph state a mutation left behind.
    pub postconditions: Vec<(String, Expectation)>,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Execution engine
// ---------------------------------------------------------------------------

struct Execution {
    rows: Vec<Vec<Val>>,
    columns: Vec<String>,
    /// One entry per postcondition, in declaration order.
    post_rows: Vec<Vec<Vec<Val>>>,
    elapsed_ms: u128,
}

enum ExecError {
    Setup { command: String, error: String },
    Query { stage: Stage, error: String },
    Post { stage: Stage, error: String },
}

fn attribute_stage(runtime: &Runtime, query: &str, error: &Error) -> Stage {
    let primary = Stage::from_error(error);
    if primary != Stage::Unknown {
        return primary;
    }
    // `Error::Other` is the catch-all variant used by implementations that do
    // not attribute their own failures. Re-running the front half tells us
    // whether the statement ever reached the executor; a passing statement is
    // never affected because this only runs after a failure.
    match runtime.explain(query) {
        Ok(_) => Stage::Execute,
        Err(front) => match Stage::from_error(&front) {
            Stage::Unknown => Stage::Unknown,
            stage => stage,
        },
    }
}

fn run_worker(
    case_dir: &Path,
    setup: &[String],
    query: &str,
    posts: &[String],
    first_only: bool,
) -> std::result::Result<Execution, ExecError> {
    let started = Instant::now();
    let runtime = assembly::build_runtime_clean(&case_dir.to_string_lossy()).map_err(|error| {
        ExecError::Setup {
            command: "<open database>".into(),
            error: error.to_string(),
        }
    })?;

    for command in setup {
        if let Err(error) = runtime.execute_query(command) {
            return Err(ExecError::Setup {
                command: command.clone(),
                error: error.to_string(),
            });
        }
    }

    let stream = match runtime.execute_query(query) {
        Ok(stream) => stream,
        Err(error) => {
            return Err(ExecError::Query {
                stage: attribute_stage(&runtime, query, &error),
                error: error.to_string(),
            })
        }
    };
    let mut rows = canon::from_stream(&stream);
    if first_only {
        // `next` consumes a single result; anything after it is not observed.
        rows.truncate(1);
    }
    let columns = stream.schema.clone();

    let mut post_rows = Vec::with_capacity(posts.len());
    for text in posts {
        match runtime.execute_query(text) {
            Ok(stream) => post_rows.push(canon::from_stream(&stream)),
            Err(error) => {
                return Err(ExecError::Post {
                    stage: attribute_stage(&runtime, text, &error),
                    error: error.to_string(),
                })
            }
        }
    }

    Ok(Execution {
        rows,
        columns,
        post_rows,
        elapsed_ms: started.elapsed().as_millis(),
    })
}

/// Run one case in a child process under a wall-clock budget.
///
/// Candidate parsers and executors are untrusted experimental code: they can
/// loop forever, overflow the stack, or abort the process. A thread cannot be
/// stopped safely after a timeout, so every case crosses a small JSON boundary
/// and the parent kills the complete child process when the budget expires.
fn run_isolated(case_dir: PathBuf, case: &Case, budget: Duration) -> (Outcome, bool) {
    let request_path = case_dir.with_extension("request.json");
    let response_path = case_dir.with_extension("response.json");
    let stderr_path = case_dir.with_extension("stderr.log");
    drop(fs::remove_file(&request_path));
    drop(fs::remove_file(&response_path));
    drop(fs::remove_file(&stderr_path));

    if let Err(error) = fs::write(&request_path, child_request_json(case).to_string()) {
        return (
            isolation_failure(format!("cannot write child request: {error}")),
            false,
        );
    }
    let executable = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            drop(fs::remove_file(&request_path));
            return (
                isolation_failure(format!("cannot locate benchmark executable: {error}")),
                false,
            );
        }
    };
    let stderr_file = match fs::File::create(&stderr_path) {
        Ok(file) => file,
        Err(error) => {
            drop(fs::remove_file(&request_path));
            return (
                isolation_failure(format!("cannot create child error log: {error}")),
                false,
            );
        }
    };
    let mut child = match Command::new(executable)
        .arg("--benchmark-child")
        .arg(&request_path)
        .arg(&response_path)
        .arg(&case_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::from(stderr_file))
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            drop(fs::remove_file(&request_path));
            drop(fs::remove_file(&stderr_path));
            return (
                isolation_failure(format!("cannot start benchmark child: {error}")),
                false,
            );
        }
    };

    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let outcome = if status.success() {
                    read_child_outcome(&response_path).unwrap_or_else(isolation_failure)
                } else {
                    isolation_failure(format!(
                        "benchmark child exited with {status}{}",
                        child_error_suffix(&stderr_path)
                    ))
                };
                drop(fs::remove_file(&request_path));
                drop(fs::remove_file(&response_path));
                drop(fs::remove_file(&stderr_path));
                return (outcome, false);
            }
            Ok(None) if started.elapsed() < budget => {
                thread::sleep(Duration::from_millis(2));
            }
            Ok(None) => {
                drop(child.kill());
                drop(child.wait());
                drop(fs::remove_file(&request_path));
                drop(fs::remove_file(&response_path));
                let stderr = child_error_suffix(&stderr_path);
                drop(fs::remove_file(&stderr_path));
                return (
                    Outcome::new(
                        Status::Timeout,
                        String::new(),
                        format!("no result within {} ms{}", budget.as_millis(), stderr),
                        budget.as_millis(),
                    ),
                    true,
                );
            }
            Err(error) => {
                drop(child.kill());
                drop(child.wait());
                drop(fs::remove_file(&request_path));
                drop(fs::remove_file(&response_path));
                drop(fs::remove_file(&stderr_path));
                return (
                    isolation_failure(format!("cannot inspect benchmark child: {error}")),
                    false,
                );
            }
        }
    }
}

fn isolation_failure(detail: String) -> Outcome {
    Outcome::new(Status::ExecuteFailure, String::new(), detail, 0)
}

fn child_error_suffix(path: &Path) -> String {
    let Ok(text) = fs::read_to_string(path) else {
        return String::new();
    };
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    // Candidate panics can include very large backtraces. Keep the diagnostic
    // useful without allowing one case to dominate the report.
    let start = text
        .char_indices()
        .rev()
        .nth(3_999)
        .map(|(index, _)| index)
        .unwrap_or(0);
    let tail = &text[start..];
    format!("; child stderr: {tail}")
}

fn outcome_for(
    case: &Case,
    result: std::result::Result<Execution, ExecError>,
    timed_out: bool,
) -> Outcome {
    if timed_out {
        return Outcome::new(
            Status::Timeout,
            String::new(),
            format!("no result within {} ms", case.timeout_ms),
            0,
        );
    }
    match result {
        Err(ExecError::Setup { command, error }) => Outcome::new(
            Status::SetupFailure,
            String::new(),
            format!("`{command}` failed: {error}"),
            0,
        ),
        Err(ExecError::Query { stage, error }) => {
            match negative_verdict(&case.expectation, stage, &error) {
                Some(Ok(())) => Outcome::new(Status::Pass, String::new(), error, 0),
                Some(Err(detail)) => Outcome::new(Status::Mismatch, String::new(), detail, 0),
                None => Outcome::new(stage_status(stage), String::new(), error, 0),
            }
        }
        Err(ExecError::Post { stage, error }) => Outcome::new(
            Status::PostconditionFailure,
            String::new(),
            format!("postcondition {} failure: {error}", stage.label()),
            0,
        ),
        Ok(execution) => {
            let elapsed = execution.elapsed_ms;
            let columns = execution.columns.clone();
            match &case.expectation {
                Expectation::Error { stage, pattern } => {
                    if let Some(pattern) = pattern {
                        Outcome::new(
                            Status::UnexpectedSuccess,
                            canon::render_rows(&execution.rows, case.ordered),
                            format!("expected a {stage} error containing {pattern:?}"),
                            elapsed,
                        )
                    } else {
                        Outcome::new(
                            Status::UnexpectedSuccess,
                            canon::render_rows(&execution.rows, case.ordered),
                            format!("expected a {stage} error"),
                            elapsed,
                        )
                    }
                }
                expectation => {
                    let actual = render_expectation_result(&execution, expectation, case.ordered);
                    let matched = expectation_matches(&execution, expectation, case);
                    let post_matched = check_postconditions(case, &execution);
                    if !matched {
                        Outcome::new(Status::Mismatch, actual, String::new(), elapsed)
                    } else if let Some(detail) = post_matched {
                        Outcome::new(Status::PostconditionFailure, actual, detail, elapsed)
                    } else {
                        Outcome::new(
                            if case.comparison == "shape" {
                                Status::ShapeMatch
                            } else {
                                Status::Pass
                            },
                            actual,
                            String::new(),
                            elapsed,
                        )
                    }
                    .with_columns(columns)
                }
            }
        }
    }
}

/// Verdict for a statement that failed while the case expected an error.
///
/// `None` means the case was not a negative case, so the failure keeps its
/// pipeline-stage status. `Some(Ok(()))` means the failure matched the
/// expected class and the case passes; `Some(Err(detail))` means it failed for
/// the wrong reason, which is a mismatch rather than a stage failure.
fn negative_verdict(
    expectation: &Expectation,
    observed: Stage,
    error: &str,
) -> Option<std::result::Result<(), String>> {
    let Expectation::Error { stage, pattern } = expectation else {
        return None;
    };
    if !observed.matches(stage) {
        return Some(Err(format!(
            "expected a {stage} error but the statement failed at {}",
            observed.label()
        )));
    }
    if let Some(pattern) = pattern {
        if !error.contains(pattern.as_str()) {
            return Some(Err(format!(
                "expected the {stage} error to mention {pattern:?}, got: {error}"
            )));
        }
    }
    Some(Ok(()))
}

/// True when a case carries an oracle the profile refuses to normalize.
///
/// Such a case is reported as `UNSCORED` and excluded from every rate: running
/// it would only prove that *something* happened, which is not a semantic
/// result.
fn expectation_unresolved(case: &Case) -> bool {
    let check = |expectation: &Expectation| match expectation {
        Expectation::Rows { rows, .. } => canon::rows_unresolved(rows),
        _ => false,
    };
    check(&case.expectation)
        || case
            .postconditions
            .iter()
            .any(|(_, expectation)| check(expectation))
}

fn stage_status(stage: Stage) -> Status {
    match stage {
        Stage::Parse => Status::ParseFailure,
        Stage::Plan => Status::PlanFailure,
        Stage::Optimize => Status::OptimizeFailure,
        Stage::Execute | Stage::Unknown => Status::ExecuteFailure,
    }
}

fn render_expectation_result(
    execution: &Execution,
    expectation: &Expectation,
    ordered: bool,
) -> String {
    match expectation {
        Expectation::Rows { .. } => canon::render_rows(&execution.rows, ordered),
        Expectation::LegacyText(_) => legacy_render(&execution.rows, ordered),
        Expectation::Error { .. } => String::new(),
    }
}

fn expectation_matches(execution: &Execution, expectation: &Expectation, case: &Case) -> bool {
    match expectation {
        Expectation::Rows { rows, .. } => {
            if case.comparison == "shape" {
                shape_matches(&execution.rows, rows)
            } else {
                canon::rows_match(&execution.rows, rows, case.ordered)
            }
        }
        Expectation::LegacyText(text) => legacy_render(&execution.rows, case.ordered) == *text,
        Expectation::Error { .. } => false,
    }
}

/// Shape comparison: the same number of rows with the same per-row arity.
///
/// Used for structural diagnostics when value-level comparison is unavailable.
/// It never counts as a semantic pass.
fn shape_matches(actual: &[Vec<Val>], expected: &[Vec<Val>]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    let mut actual_widths = canon::shape_of(actual);
    let mut expected_widths = canon::shape_of(expected);
    actual_widths.sort_unstable();
    expected_widths.sort_unstable();
    actual_widths == expected_widths
}

/// Verify every postcondition; the first failure is reported.
fn check_postconditions(case: &Case, execution: &Execution) -> Option<String> {
    for (index, (query, expectation)) in case.postconditions.iter().enumerate() {
        let rows = execution.post_rows.get(index)?;
        let matched = match expectation {
            Expectation::Rows { rows: expected, .. } => {
                canon::rows_match(rows, expected, case.ordered)
            }
            Expectation::LegacyText(text) => legacy_render(rows, case.ordered) == *text,
            Expectation::Error { .. } => false,
        };
        if !matched {
            return Some(format!(
                "postcondition `{query}` produced {}",
                canon::render_rows(rows, case.ordered)
            ));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Outcome and reporting
// ---------------------------------------------------------------------------

pub struct Outcome {
    pub status: Status,
    pub actual: String,
    pub detail: String,
    pub elapsed_ms: u128,
    /// Column labels the executor produced, kept for diagnosis only; column
    /// *names* are engine-specific and deliberately not part of the oracle.
    pub observed_columns: Vec<String>,
}

impl Outcome {
    fn new(status: Status, actual: String, detail: String, elapsed_ms: u128) -> Self {
        Self {
            status,
            actual,
            detail,
            elapsed_ms,
            observed_columns: Vec::new(),
        }
    }

    fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.observed_columns = columns;
        self
    }
}

#[derive(Default, Clone)]
pub struct Tally {
    pub passed: usize,
    pub total: usize,
}

pub struct Report {
    pub outcomes: Vec<Outcome>,
    pub status_counts: BTreeMap<String, usize>,
    pub category: BTreeMap<String, Tally>,
    pub feature: BTreeMap<String, Tally>,
    pub source: BTreeMap<String, Tally>,
    pub total: usize,
    pub inapplicable: usize,
    pub scored: usize,
    pub passed: usize,
    pub executed: usize,
    pub shape_comparable: usize,
    pub shape_matched: usize,
    pub exact_comparable: usize,
    pub strict_passed: usize,
    pub timeouts: usize,
    pub elapsed_ms: u128,
}

fn tally(entries: &mut BTreeMap<String, Tally>, key: &str, passed: bool) {
    let entry = entries.entry(key.to_string()).or_default();
    entry.total += 1;
    if passed {
        entry.passed += 1;
    }
}

// ---------------------------------------------------------------------------
// Dataset loading
// ---------------------------------------------------------------------------

pub fn run(config: Config) -> Result<()> {
    let (cases, format, report) = execute_all(&config)?;
    let failures = collect_failures(&cases, &report);

    write_reports(&config, &cases, &report)?;
    summarize(&config, &report, &failures);

    let pass_rate = ratio(report.passed, report.scored);
    let execution_rate = ratio(report.executed, report.total - report.inapplicable);

    if pass_rate < config.min_pass_rate {
        return Err(Error::Other(format!(
            "Core Profile pass rate {:.1}% is below required {:.1}% (format {format})",
            pass_rate * 100.0,
            config.min_pass_rate * 100.0
        )));
    }
    if execution_rate < config.min_execution_rate {
        return Err(Error::Other(format!(
            "execution rate {:.1}% is below required {:.1}%",
            execution_rate * 100.0,
            config.min_execution_rate * 100.0
        )));
    }
    Ok(())
}

/// Load, select and execute a dataset, returning the cases and their tally.
///
pub fn execute_all(config: &Config) -> Result<(Vec<Case>, String, Report)> {
    let fixtures = load_fixtures(config)?;
    let (mut cases, format) = load_cases(&config.dataset_dir, &fixtures, config.strict_fixtures)?;
    apply_selection(&mut cases, config);

    if cases.is_empty() {
        return Err(Error::Other(format!(
            "benchmark contains no cases after filtering: {}",
            config.dataset_dir.display()
        )));
    }

    fs::create_dir_all(&config.output_dir).map_err(io_error)?;

    if !config.quiet {
        println!("=== metaGDB language benchmark ===");
        println!("Dataset: {}", config.dataset_dir.display());
        println!("Format:  {format}");
        println!("Cases:   {}", cases.len());
        println!();
    }

    // Journal every outcome as it happens: a case may abort the process, and a
    // lost run would otherwise have to start over from the first case.
    let journal_path = config.output_dir.join("report.jsonl");
    if config.resume && journal_path.is_file() {
        let (done, aborted) = read_journal_state(&journal_path)?;
        let before = cases.len();
        cases.retain(|case| !done.contains(&case.id));
        for case in cases.iter_mut() {
            if aborted.contains(&case.id) {
                // A `STARTING` record without an outcome means the process died
                // on this case. Record it as one failure and move past it.
                case.aborted = true;
            }
        }
        if !config.quiet {
            println!(
                "Resuming: {} of {before} cases already journalled, {} previously aborted",
                before - cases.len(),
                aborted.len()
            );
        }
        if cases.is_empty() {
            return Err(Error::Other(format!(
                "every case is already journalled in {}",
                journal_path.display()
            )));
        }
    }
    let mut journal = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .truncate(false)
        .open(&journal_path)
        .map_err(io_error)?;
    if !config.resume {
        journal.set_len(0).map_err(io_error)?;
    }

    let mut report = Report::new();
    let started = Instant::now();

    for (index, case) in cases.iter().enumerate() {
        if case.aborted {
            let outcome = Outcome::new(
                Status::ExecuteFailure,
                String::new(),
                "the implementation aborted the process before returning a result;                  re-run with a fixed implementation to score this case"
                    .into(),
                0,
            );
            record_case(&mut report, case, outcome);
            if let Some(last) = report.outcomes.last() {
                journal
                    .write_all(case_record_json(case, last).to_string().as_bytes())
                    .map_err(io_error)?;
                journal.write_all(b"\n").map_err(io_error)?;
                journal.flush().map_err(io_error)?;
            }
            continue;
        }
        if let Some(started) = start_record_json(case) {
            journal.write_all(started.as_bytes()).map_err(io_error)?;
            journal.write_all(b"\n").map_err(io_error)?;
            journal.flush().map_err(io_error)?;
        }
        let outcome = if !case.applicable {
            Outcome::new(
                Status::Inapplicable,
                String::new(),
                case.inapplicable_reason.clone().unwrap_or_default(),
                0,
            )
        } else if expectation_unresolved(case) {
            Outcome::new(
                Status::Unscored,
                String::new(),
                "the oracle contains a cell this profile cannot normalize".into(),
                0,
            )
        } else {
            let case_dir = config.output_dir.join(format!("case-{index:04}"));
            if case_dir.exists() {
                fs::remove_dir_all(&case_dir).map_err(io_error)?;
            }
            let budget = Duration::from_millis(if config.timeout_ms > 0 {
                config.timeout_ms
            } else {
                case.timeout_ms
            });
            let (outcome, timed_out) = run_isolated(case_dir.clone(), case, budget);
            if timed_out {
                report.timeouts += 1;
            }
            drop(fs::remove_dir_all(&case_dir));
            outcome
        };

        record_case(&mut report, case, outcome);
        if let Some(last) = report.outcomes.last() {
            let record = case_record_json(case, last);
            journal
                .write_all(record.to_string().as_bytes())
                .map_err(io_error)?;
            journal.write_all(b"\n").map_err(io_error)?;
            journal.flush().map_err(io_error)?;
        }
    }

    report.elapsed_ms = started.elapsed().as_millis();

    Ok((cases, format, report))
}

impl Report {
    pub fn new() -> Self {
        Self {
            outcomes: Vec::new(),
            status_counts: BTreeMap::new(),
            category: BTreeMap::new(),
            feature: BTreeMap::new(),
            source: BTreeMap::new(),
            total: 0,
            inapplicable: 0,
            scored: 0,
            passed: 0,
            executed: 0,
            shape_comparable: 0,
            shape_matched: 0,
            exact_comparable: 0,
            strict_passed: 0,
            timeouts: 0,
            elapsed_ms: 0,
        }
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

fn record_case(report: &mut Report, case: &Case, outcome: Outcome) {
    let status = outcome.status.clone();
    *report
        .status_counts
        .entry(status.label().to_string())
        .or_insert(0) += 1;
    report.total += 1;

    if !case.applicable {
        report.inapplicable += 1;
        report.outcomes.push(outcome);
        return;
    }

    // Shape-only observations are diagnostic: matching row counts/arity is not
    // semantic correctness and must not raise any graded pass rate.
    let countable = case.scored && status != Status::Unscored && case.comparison != "shape";
    if countable {
        report.scored += 1;
        if status.is_pass() {
            report.passed += 1;
        }
        tally(&mut report.category, &case.category, status.is_pass());
        for feature in &case.features {
            tally(&mut report.feature, feature, status.is_pass());
        }
        tally(&mut report.source, &case.source_kind, status.is_pass());
    }
    if status.is_executed() && status != Status::Unscored {
        report.executed += 1;
    }
    if case.scored && status != Status::Unscored && case.comparison == "shape" {
        report.shape_comparable += 1;
        if status == Status::ShapeMatch {
            report.shape_matched += 1;
        }
    } else if countable {
        report.exact_comparable += 1;
        if status.is_pass() {
            report.strict_passed += 1;
        }
    }

    report.outcomes.push(outcome);
}

pub fn collect_failures(cases: &[Case], report: &Report) -> Vec<String> {
    let mut failures = Vec::new();
    for (case, outcome) in cases.iter().zip(&report.outcomes) {
        if outcome.status.is_pass()
            || outcome.status == Status::ShapeMatch
            || !case.applicable
            || !case.scored
        {
            continue;
        }
        failures.push(format!(
            "{} [{}] {}\n  query:    {}\n  expected: {}\n  observed: {} {}",
            case.id,
            case.category,
            outcome.status.label(),
            case.query,
            describe_expectation(case),
            outcome.actual,
            outcome.detail
        ));
    }
    failures
}

/// Declared column labels of a case expectation, when the profile states them.
fn expected_columns(case: &Case) -> &[String] {
    match &case.expectation {
        Expectation::Rows { columns, .. } => columns,
        _ => &[],
    }
}

/// One case outcome as a JSON object.
///
/// Used both for the incremental journal, which must survive an implementation
/// that aborts the process, and for the final report.
fn case_record_json(case: &Case, outcome: &Outcome) -> Json {
    Json::Object(BTreeMap::from([
        ("id".to_string(), Json::Text(case.id.clone())),
        ("language".to_string(), Json::Text(case.language.clone())),
        ("category".to_string(), Json::Text(case.category.clone())),
        (
            "features".to_string(),
            Json::Array(case.features.iter().cloned().map(Json::Text).collect()),
        ),
        (
            "status".to_string(),
            Json::Text(outcome.status.label().into()),
        ),
        ("query".to_string(), Json::Text(case.query.clone())),
        (
            "expected".to_string(),
            Json::Text(describe_expectation(case)),
        ),
        ("actual".to_string(), Json::Text(outcome.actual.clone())),
        ("detail".to_string(), Json::Text(outcome.detail.clone())),
        ("fixture".to_string(), Json::Text(case.fixture.clone())),
        (
            "expected_columns".to_string(),
            Json::Array(
                expected_columns(case)
                    .iter()
                    .cloned()
                    .map(Json::Text)
                    .collect(),
            ),
        ),
        (
            "observed_columns".to_string(),
            Json::Array(
                outcome
                    .observed_columns
                    .iter()
                    .cloned()
                    .map(Json::Text)
                    .collect(),
            ),
        ),
        (
            "notes".to_string(),
            Json::Text(case.notes.clone().unwrap_or_default()),
        ),
        (
            "comparison".to_string(),
            Json::Text(case.comparison.clone()),
        ),
        ("first_result".to_string(), Json::Bool(case.first_only)),
        (
            "source".to_string(),
            Json::Text(format!("{}:{}", case.source_kind, case.source_detail)),
        ),
        (
            "elapsed_ms".to_string(),
            Json::Int(outcome.elapsed_ms as i64),
        ),
    ]))
}

fn child_request_json(case: &Case) -> Json {
    Json::Object(BTreeMap::from([
        ("id".to_string(), Json::Text(case.id.clone())),
        (
            "setup".to_string(),
            Json::Array(case.setup.iter().cloned().map(Json::Text).collect()),
        ),
        ("query".to_string(), Json::Text(case.query.clone())),
        ("ordered".to_string(), Json::Bool(case.ordered)),
        (
            "comparison".to_string(),
            Json::Text(case.comparison.clone()),
        ),
        ("first_result".to_string(), Json::Bool(case.first_only)),
        (
            "expectation".to_string(),
            child_expectation_json(&case.expectation),
        ),
        (
            "postconditions".to_string(),
            Json::Array(
                case.postconditions
                    .iter()
                    .map(|(query, expectation)| {
                        Json::Object(BTreeMap::from([
                            ("query".to_string(), Json::Text(query.clone())),
                            (
                                "expectation".to_string(),
                                child_expectation_json(expectation),
                            ),
                        ]))
                    })
                    .collect(),
            ),
        ),
    ]))
}

fn child_expectation_json(expectation: &Expectation) -> Json {
    match expectation {
        Expectation::Rows { rows, columns } => Json::Object(BTreeMap::from([
            ("type".to_string(), Json::Text("rows".into())),
            (
                "rows".to_string(),
                Json::Array(
                    rows.iter()
                        .map(|row| Json::Array(row.iter().map(canon::to_json).collect()))
                        .collect(),
                ),
            ),
            (
                "columns".to_string(),
                Json::Array(columns.iter().cloned().map(Json::Text).collect()),
            ),
        ])),
        Expectation::Error { stage, pattern } => {
            let mut value = BTreeMap::from([
                ("type".to_string(), Json::Text("error".into())),
                ("stage".to_string(), Json::Text(stage.clone())),
            ]);
            if let Some(pattern) = pattern {
                value.insert("pattern".to_string(), Json::Text(pattern.clone()));
            }
            Json::Object(value)
        }
        Expectation::LegacyText(text) => Json::Object(BTreeMap::from([
            ("type".to_string(), Json::Text("legacy".into())),
            ("text".to_string(), Json::Text(text.clone())),
        ])),
    }
}

fn parse_child_expectation(value: &Json) -> std::result::Result<Expectation, String> {
    let kind = value
        .get("type")
        .and_then(Json::as_str)
        .ok_or("child expectation has no type")?;
    match kind {
        "rows" => {
            let mut rows = Vec::new();
            for row in value
                .get("rows")
                .and_then(Json::as_array)
                .ok_or("child row expectation has no rows")?
            {
                let mut cells = Vec::new();
                for cell in row.as_array().ok_or("child expected row is not an array")? {
                    cells.push(canon::from_json(cell)?);
                }
                rows.push(cells);
            }
            Ok(Expectation::Rows {
                rows,
                columns: child_string_array(value, "columns")?,
            })
        }
        "error" => Ok(Expectation::Error {
            stage: value
                .get("stage")
                .and_then(Json::as_str)
                .ok_or("child error expectation has no stage")?
                .to_string(),
            pattern: value
                .get("pattern")
                .and_then(Json::as_str)
                .map(str::to_string),
        }),
        "legacy" => Ok(Expectation::LegacyText(
            value
                .get("text")
                .and_then(Json::as_str)
                .ok_or("child legacy expectation has no text")?
                .to_string(),
        )),
        other => Err(format!("unknown child expectation type {other:?}")),
    }
}

fn child_string_array(value: &Json, key: &str) -> std::result::Result<Vec<String>, String> {
    value
        .get(key)
        .and_then(Json::as_array)
        .ok_or_else(|| format!("child request has no {key} array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("child request {key} entry is not a string"))
        })
        .collect()
}

fn parse_child_case(value: &Json) -> std::result::Result<Case, String> {
    let text = |key: &str| {
        value
            .get(key)
            .and_then(Json::as_str)
            .map(str::to_string)
            .ok_or_else(|| format!("child request has no string field {key}"))
    };
    let mut postconditions = Vec::new();
    for post in value
        .get("postconditions")
        .and_then(Json::as_array)
        .ok_or("child request has no postconditions array")?
    {
        postconditions.push((
            post.get("query")
                .and_then(Json::as_str)
                .ok_or("child postcondition has no query")?
                .to_string(),
            parse_child_expectation(
                post.get("expectation")
                    .ok_or("child postcondition has no expectation")?,
            )?,
        ));
    }
    Ok(Case {
        id: text("id")?,
        language: "child".into(),
        category: "child".into(),
        features: Vec::new(),
        capabilities: Vec::new(),
        source_kind: "child".into(),
        source_detail: String::new(),
        fixture: "inline".into(),
        setup: child_string_array(value, "setup")?,
        query: text("query")?,
        ordered: value
            .get("ordered")
            .and_then(Json::as_bool)
            .unwrap_or(false),
        timeout_ms: DEFAULT_TIMEOUT_MS,
        scored: true,
        applicable: true,
        inapplicable_reason: None,
        comparison: text("comparison")?,
        first_only: value
            .get("first_result")
            .and_then(Json::as_bool)
            .unwrap_or(false),
        aborted: false,
        expectation: parse_child_expectation(
            value
                .get("expectation")
                .ok_or("child request has no expectation")?,
        )?,
        postconditions,
        notes: None,
    })
}

fn child_outcome_json(outcome: &Outcome) -> Json {
    Json::Object(BTreeMap::from([
        (
            "status".to_string(),
            Json::Text(outcome.status.label().into()),
        ),
        ("actual".to_string(), Json::Text(outcome.actual.clone())),
        ("detail".to_string(), Json::Text(outcome.detail.clone())),
        (
            "elapsed_ms".to_string(),
            Json::Int(outcome.elapsed_ms as i64),
        ),
        (
            "observed_columns".to_string(),
            Json::Array(
                outcome
                    .observed_columns
                    .iter()
                    .cloned()
                    .map(Json::Text)
                    .collect(),
            ),
        ),
    ]))
}

fn read_child_outcome(path: &Path) -> std::result::Result<Outcome, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("benchmark child produced no readable result: {error}"))?;
    let value = crate::json::parse(&text).map_err(|error| error.to_string())?;
    let status_text = value
        .get("status")
        .and_then(Json::as_str)
        .ok_or("benchmark child result has no status")?;
    Ok(Outcome {
        status: Status::parse(status_text)
            .ok_or_else(|| format!("benchmark child returned unknown status {status_text:?}"))?,
        actual: value
            .get("actual")
            .and_then(Json::as_str)
            .unwrap_or_default()
            .to_string(),
        detail: value
            .get("detail")
            .and_then(Json::as_str)
            .unwrap_or_default()
            .to_string(),
        elapsed_ms: value
            .get("elapsed_ms")
            .and_then(Json::as_i64)
            .unwrap_or(0)
            .max(0) as u128,
        observed_columns: child_string_array(&value, "observed_columns")?,
    })
}

/// Hidden child-process entry used by [`run_isolated`].
pub fn run_child(request_path: &Path, response_path: &Path, case_dir: &Path) -> Result<()> {
    let request = fs::read_to_string(request_path).map_err(io_error)?;
    let value = crate::json::parse(&request)?;
    let case = parse_child_case(&value).map_err(Error::Other)?;
    let result = run_worker(
        case_dir,
        &case.setup,
        &case.query,
        &case
            .postconditions
            .iter()
            .map(|(query, _)| query.clone())
            .collect::<Vec<_>>(),
        case.first_only,
    );
    let outcome = outcome_for(&case, result, false);
    fs::write(response_path, child_outcome_json(&outcome).to_string()).map_err(io_error)
}

/// The `STARTING` marker written before a case runs.
///
/// Its presence without a following outcome is how a resumed run learns which
/// case killed the process.
fn start_record_json(case: &Case) -> Option<String> {
    Some(
        Json::Object(BTreeMap::from([
            ("id".to_string(), Json::Text(case.id.clone())),
            ("status".to_string(), Json::Text("STARTING".into())),
        ]))
        .to_string(),
    )
}

/// Split a journal into cases that produced an outcome and cases that did not.
fn read_journal_state(path: &Path) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
    let text = fs::read_to_string(path).map_err(io_error)?;
    let mut done = BTreeSet::new();
    let mut started: BTreeSet<String> = BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let Ok(value) = crate::json::parse(line) else {
            continue;
        };
        let Some(id) = value.get("id").and_then(Json::as_str) else {
            continue;
        };
        match value.get("status").and_then(Json::as_str) {
            Some("STARTING") => {
                started.insert(id.to_string());
            }
            Some(_) => {
                started.remove(id);
                done.insert(id.to_string());
            }
            None => {}
        }
    }
    Ok((done, started))
}

pub fn describe_expectation(case: &Case) -> String {
    match &case.expectation {
        Expectation::Rows { rows, .. } if case.comparison == "shape" => {
            format!("shape of {} row(s)", rows.len())
        }
        Expectation::Rows { rows, .. } => canon::render_rows(rows, case.ordered),
        Expectation::LegacyText(text) => text.clone(),
        Expectation::Error { stage, pattern } => match pattern {
            Some(pattern) => format!("a {stage} error matching {pattern:?}"),
            None => format!("a {stage} error"),
        },
    }
}

// ---------------------------------------------------------------------------
// Report writing
// ---------------------------------------------------------------------------

pub fn write_reports(config: &Config, cases: &[Case], report: &Report) -> Result<()> {
    let mut tsv = String::from(
        "id\tlanguage\tcategory\tfeatures\tcapabilities\tstatus\tdetail\tsource\tfixture\telapsed_ms\tquery\texpected\tactual\n",
    );
    for (case, outcome) in cases.iter().zip(&report.outcomes) {
        let cells = [
            case.id.clone(),
            case.language.clone(),
            case.category.clone(),
            case.features.join(","),
            case.capabilities.join(","),
            outcome.status.label().to_string(),
            outcome.detail.clone(),
            format!("{}:{}", case.source_kind, case.source_detail),
            case.fixture.clone(),
            outcome.elapsed_ms.to_string(),
            case.query.clone(),
            describe_expectation(case),
            outcome.actual.clone(),
        ];
        tsv.push_str(
            &cells
                .iter()
                .map(|cell| sanitize(cell))
                .collect::<Vec<_>>()
                .join("\t"),
        );
        tsv.push('\n');
    }
    fs::write(config.output_dir.join("report.tsv"), tsv).map_err(io_error)?;

    let mut summary = BTreeMap::new();
    let format = format_version(&config.dataset_dir);
    summary.insert("format".to_string(), Json::Text(format));
    summary.insert(
        "dataset".to_string(),
        Json::Text(config.dataset_dir.display().to_string()),
    );
    summary.insert("cases".to_string(), Json::Int(report.total as i64));
    summary.insert(
        "inapplicable".to_string(),
        Json::Int(report.inapplicable as i64),
    );
    summary.insert("timeouts".to_string(), Json::Int(report.timeouts as i64));
    summary.insert(
        "elapsed_ms".to_string(),
        Json::Int(report.elapsed_ms as i64),
    );
    summary.insert(
        "status_counts".to_string(),
        Json::Object(
            report
                .status_counts
                .iter()
                .map(|(key, value)| (key.clone(), Json::Int(*value as i64)))
                .collect(),
        ),
    );
    summary.insert("scored".to_string(), Json::Int(report.scored as i64));
    summary.insert("passed".to_string(), Json::Int(report.passed as i64));
    summary.insert("executed".to_string(), Json::Int(report.executed as i64));
    summary.insert(
        "micro_pass_rate".to_string(),
        Json::Float(ratio(report.passed, report.scored)),
    );
    summary.insert(
        "macro_category_pass_rate".to_string(),
        Json::Float(macro_average(&report.category)),
    );
    summary.insert(
        "macro_feature_pass_rate".to_string(),
        Json::Float(macro_average(&report.feature)),
    );
    summary.insert(
        "categories".to_string(),
        Json::Array(
            report
                .category
                .iter()
                .map(|(name, score)| tally_json(name, score))
                .collect(),
        ),
    );
    summary.insert(
        "features".to_string(),
        Json::Array(
            report
                .feature
                .iter()
                .map(|(name, score)| tally_json(name, score))
                .collect(),
        ),
    );
    summary.insert(
        "sources".to_string(),
        Json::Array(
            report
                .source
                .iter()
                .map(|(name, score)| tally_json(name, score))
                .collect(),
        ),
    );
    if let Ok(fixtures) = load_fixtures(config) {
        summary.insert(
            "fixtures".to_string(),
            Json::Array(
                fixtures
                    .values()
                    .map(|fixture| {
                        Json::Object(BTreeMap::from([
                            ("name".to_string(), Json::Text(fixture.name.clone())),
                            ("language".to_string(), Json::Text(fixture.language.clone())),
                            (
                                "description".to_string(),
                                Json::Text(fixture.description.clone()),
                            ),
                            (
                                "setup_statements".to_string(),
                                Json::Int(fixture.setup.len() as i64),
                            ),
                        ]))
                    })
                    .collect(),
            ),
        );
    }
    fs::write(
        config.output_dir.join("summary.json"),
        Json::Object(summary).to_string(),
    )
    .map_err(io_error)
}

fn tally_json(name: &str, score: &Tally) -> Json {
    Json::Object(BTreeMap::from([
        ("name".to_string(), Json::Text(name.into())),
        ("passed".to_string(), Json::Int(score.passed as i64)),
        ("total".to_string(), Json::Int(score.total as i64)),
        (
            "rate".to_string(),
            Json::Float(ratio(score.passed, score.total)),
        ),
    ]))
}

fn format_version(dataset_dir: &Path) -> String {
    if dataset_dir.join("core.jsonl").is_file() {
        "benchmark-v2-jsonl".into()
    } else {
        "benchmark-v1-tsv".into()
    }
}

pub fn ratio(passed: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        passed as f64 / total as f64
    }
}

/// Unweighted mean of the per-category pass rates.
///
/// Micro average counts every case equally, so a large category dominates it.
/// The macro average counts every *capability* equally and is the number that
/// should be quoted when arguing about breadth of language support.
pub fn macro_average(tallies: &BTreeMap<String, Tally>) -> f64 {
    let scored: Vec<f64> = tallies
        .values()
        .filter(|score| score.total > 0)
        .map(|score| ratio(score.passed, score.total))
        .collect();
    if scored.is_empty() {
        0.0
    } else {
        scored.iter().sum::<f64>() / scored.len() as f64
    }
}

fn summarize(config: &Config, report: &Report, failures: &[String]) {
    println!("Status breakdown:");
    for (status, count) in &report.status_counts {
        println!("  {status:<22} {count}");
    }

    println!("\nCategory pass rates:");
    for (category, score) in &report.category {
        println!(
            "  {:<28} {:>3}/{:<3} {:>6.1}%",
            category,
            score.passed,
            score.total,
            ratio(score.passed, score.total) * 100.0
        );
    }

    println!("\nFeature pass rates (lowest 20):");
    let mut features: Vec<(&String, &Tally)> = report.feature.iter().collect();
    features.sort_by(|a, b| {
        ratio(a.1.passed, a.1.total)
            .partial_cmp(&ratio(b.1.passed, b.1.total))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(b.0))
    });
    for (feature, score) in features.iter().take(20) {
        println!(
            "  {:<32} {:>3}/{:<3} {:>6.1}%",
            feature,
            score.passed,
            score.total,
            ratio(score.passed, score.total) * 100.0
        );
    }

    println!(
        "\nMicro pass rate: {:.1}% ({}/{})",
        ratio(report.passed, report.scored) * 100.0,
        report.passed,
        report.scored
    );
    println!(
        "Macro category average: {:.1}%",
        macro_average(&report.category) * 100.0
    );
    println!("Report: {}", config.output_dir.join("report.tsv").display());

    if !failures.is_empty() {
        println!("\nFirst {} failures:", failures.len().min(20));
        for failure in failures.iter().take(20) {
            println!("{failure}");
        }
    }
}

fn sanitize(text: &str) -> String {
    text.replace(['\t', '\n'], " ")
}

fn io_error(error: std::io::Error) -> Error {
    Error::Other(error.to_string())
}

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct Fixture {
    pub name: String,
    pub language: String,
    pub description: String,
    pub setup: Vec<String>,
}

pub fn load_fixtures(config: &Config) -> Result<BTreeMap<(String, String), Fixture>> {
    let path = config
        .fixtures_path
        .clone()
        .unwrap_or_else(|| config.dataset_dir.join("..").join("fixtures.jsonl"));
    let mut out = BTreeMap::new();
    if !path.is_file() {
        return Ok(out);
    }
    let text = fs::read_to_string(&path).map_err(io_error)?;
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let value = crate::json::parse(line)
            .map_err(|error| Error::Other(format!("{}:{}: {error}", path.display(), number + 1)))?;
        let name = string_field(&value, "name", &path, number)?;
        let language = string_field(&value, "language", &path, number)?;
        let setup = value
            .get("setup")
            .and_then(Json::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        out.insert(
            (language.clone(), name.clone()),
            Fixture {
                name,
                language,
                description: value
                    .get("description")
                    .and_then(Json::as_str)
                    .unwrap_or_default()
                    .to_string(),
                setup,
            },
        );
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Case loading
// ---------------------------------------------------------------------------

fn load_cases(
    dataset_dir: &Path,
    fixtures: &BTreeMap<(String, String), Fixture>,
    strict_fixtures: bool,
) -> Result<(Vec<Case>, String)> {
    let core = dataset_dir.join("core.jsonl");
    if core.is_file() {
        return Ok((
            load_v2_cases(&core, fixtures, strict_fixtures)?,
            "benchmark-v2-jsonl".into(),
        ));
    }
    let legacy = dataset_dir.join("cases.tsv");
    if legacy.is_file() {
        return Ok((load_v1_cases(dataset_dir)?, "benchmark-v1-tsv".into()));
    }
    Err(Error::Other(format!(
        "no benchmark dataset found in {}: expected core.jsonl (v2) or cases.tsv (v1)",
        dataset_dir.display()
    )))
}

fn load_v2_cases(
    path: &Path,
    fixtures: &BTreeMap<(String, String), Fixture>,
    strict_fixtures: bool,
) -> Result<Vec<Case>> {
    let text = fs::read_to_string(path).map_err(io_error)?;
    let mut cases = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let value = crate::json::parse(line)
            .map_err(|error| Error::Other(format!("{}:{}: {error}", path.display(), number + 1)))?;
        cases.push(parse_case(&value, path, number, fixtures, strict_fixtures)?);
    }
    Ok(cases)
}

fn string_field(value: &Json, key: &str, path: &Path, line: usize) -> Result<String> {
    value
        .get(key)
        .and_then(Json::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            Error::Other(format!(
                "{}:{}: missing string field `{key}`",
                path.display(),
                line + 1
            ))
        })
}

fn parse_expectation(value: &Json, path: &Path, line: usize) -> Result<Expectation> {
    let kind = value.get("kind").and_then(Json::as_str).unwrap_or("rows");
    match kind {
        "rows" => {
            let rows_json = value
                .get("rows")
                .and_then(Json::as_array)
                .ok_or_else(|| bad(path, line, "rows expectation requires a `rows` array"))?;
            let mut rows = Vec::with_capacity(rows_json.len());
            for row in rows_json {
                let cells = row
                    .as_array()
                    .ok_or_else(|| bad(path, line, "each expected row must be an array"))?;
                let mut out = Vec::with_capacity(cells.len());
                for cell in cells {
                    out.push(canon::from_json(cell).map_err(|error| bad(path, line, &error))?);
                }
                rows.push(out);
            }
            Ok(Expectation::Rows {
                rows,
                columns: value
                    .get("columns")
                    .and_then(Json::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|item| item.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
            })
        }
        other => Err(bad(
            path,
            line,
            &format!("unsupported expectation kind {other:?}"),
        )),
    }
}

fn parse_error_expectation(value: &Json, path: &Path, line: usize) -> Result<Expectation> {
    let stage = value
        .get("stage")
        .and_then(Json::as_str)
        .ok_or_else(|| bad(path, line, "expect_error requires a `stage`"))?;
    if stage != "any" && Stage::parse(stage).is_none() {
        return Err(bad(path, line, &format!("unknown error stage {stage:?}")));
    }
    Ok(Expectation::Error {
        stage: stage.to_string(),
        pattern: value
            .get("match")
            .and_then(Json::as_str)
            .map(str::to_string),
    })
}

fn bad(path: &Path, line: usize, message: &str) -> Error {
    Error::Other(format!("{}:{}: {message}", path.display(), line + 1))
}

fn parse_case(
    value: &Json,
    path: &Path,
    line: usize,
    fixtures: &BTreeMap<(String, String), Fixture>,
    strict_fixtures: bool,
) -> Result<Case> {
    let language = string_field(value, "language", path, line)?;
    let fixture_name = value
        .get("fixture")
        .and_then(Json::as_str)
        .unwrap_or("empty")
        .to_string();
    let mut setup: Vec<String> = Vec::new();
    if fixture_name != "empty" {
        match fixtures.get(&(language.clone(), fixture_name.clone())) {
            Some(fixture) => setup.extend(fixture.setup.iter().cloned()),
            None if strict_fixtures => {
                return Err(bad(
                    path,
                    line,
                    &format!("unknown fixture {fixture_name:?} for language {language:?}"),
                ))
            }
            None => {}
        }
    }
    if let Some(items) = value.get("setup").and_then(Json::as_array) {
        for item in items {
            setup.push(
                item.as_str()
                    .ok_or_else(|| bad(path, line, "setup entries must be strings"))?
                    .to_string(),
            );
        }
    }

    let expectation = match value.get("expect_error") {
        Some(error) => parse_error_expectation(error, path, line)?,
        None => parse_expectation(
            value
                .get("expected")
                .ok_or_else(|| bad(path, line, "case requires `expected` or `expect_error`"))?,
            path,
            line,
        )?,
    };

    let mut postconditions: Vec<(String, Expectation)> = Vec::new();
    let single = value.get("postcondition").map(|post| vec![post.clone()]);
    let many = value
        .get("postconditions")
        .and_then(Json::as_array)
        .map(<[Json]>::to_vec);
    for post in single
        .into_iter()
        .flatten()
        .chain(many.into_iter().flatten())
    {
        let query = string_field(&post, "query", path, line)?;
        let expected = post
            .get("expected")
            .ok_or_else(|| bad(path, line, "a postcondition requires an `expected`"))?;
        postconditions.push((query, parse_expectation(expected, path, line)?));
    }

    let source = value.get("source").cloned().unwrap_or(Json::Null);
    Ok(Case {
        id: string_field(value, "id", path, line)?,
        language,
        category: string_field(value, "category", path, line)?,
        features: string_list(value, "features"),
        capabilities: string_list(value, "capabilities"),
        source_kind: source
            .get("kind")
            .and_then(Json::as_str)
            .unwrap_or("project")
            .to_string(),
        source_detail: source
            .get("id")
            .and_then(Json::as_str)
            .or_else(|| source.get("oracle").and_then(Json::as_str))
            .or_else(|| source.get("corpus").and_then(Json::as_str))
            .unwrap_or_default()
            .to_string(),
        fixture: fixture_name,
        setup,
        query: string_field(value, "query", path, line)?,
        ordered: value
            .get("ordered")
            .and_then(Json::as_bool)
            .unwrap_or(false),
        timeout_ms: value
            .get("timeout_ms")
            .and_then(Json::as_i64)
            .map(|value| value.max(1) as u64)
            .unwrap_or(DEFAULT_TIMEOUT_MS),
        scored: value.get("scored").and_then(Json::as_bool).unwrap_or(true),
        applicable: value
            .get("applicable")
            .and_then(Json::as_bool)
            .unwrap_or(true),
        inapplicable_reason: value
            .get("inapplicable_reason")
            .and_then(Json::as_str)
            .map(str::to_string),
        comparison: value
            .get("comparison")
            .and_then(Json::as_str)
            .unwrap_or("exact")
            .to_string(),
        first_only: value
            .get("first_result")
            .and_then(Json::as_bool)
            .unwrap_or(false),
        aborted: false,
        expectation,
        postconditions,
        notes: value
            .get("notes")
            .and_then(Json::as_str)
            .map(str::to_string),
    })
}

fn string_list(value: &Json, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Json::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn apply_selection(cases: &mut Vec<Case>, config: &Config) {
    if let Some(filter) = &config.id_filter {
        cases.retain(|case| case.id.contains(filter.as_str()));
    }
    if !config.capabilities.is_empty() {
        cases.retain(|case| {
            case.capabilities
                .iter()
                .any(|capability| config.capabilities.contains(capability))
        });
    }
    if config.shards > 1 {
        let shard = config.shard.min(config.shards - 1);
        let mut index = 0usize;
        cases.retain(|_| {
            let keep = index % config.shards == shard;
            index += 1;
            keep
        });
    }
}

// ---------------------------------------------------------------------------
// v1 (TSV) compatibility
// ---------------------------------------------------------------------------

fn load_v1_cases(dataset_dir: &Path) -> Result<Vec<Case>> {
    let language = dataset_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();
    let setup = read_legacy_setup(&dataset_dir.join("setup.txt"))?;
    let text = fs::read_to_string(dataset_dir.join("cases.tsv")).map_err(io_error)?;
    let mut cases = Vec::new();
    for (number, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.splitn(5, '\t').collect();
        if fields.len() != 5 {
            return Err(Error::Other(format!(
                "{}:{}: expected five tab-separated fields",
                dataset_dir.join("cases.tsv").display(),
                number + 1
            )));
        }
        let ordered = match fields[2] {
            "ordered" => true,
            "unordered" => false,
            other => {
                return Err(Error::Other(format!(
                    "{}:{}: invalid comparison mode `{other}`",
                    dataset_dir.join("cases.tsv").display(),
                    number + 1
                )))
            }
        };
        cases.push(Case {
            id: fields[0].to_string(),
            language: language.clone(),
            category: fields[1].to_string(),
            features: Vec::new(),
            capabilities: vec!["legacy-v1".into()],
            source_kind: "legacy-v1".into(),
            source_detail: String::new(),
            fixture: "legacy-setup".into(),
            setup: setup.clone(),
            query: fields[3].to_string(),
            ordered,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            scored: true,
            applicable: true,
            inapplicable_reason: None,
            comparison: "exact".into(),
            first_only: false,
            aborted: false,
            expectation: Expectation::LegacyText(fields[4].to_string()),
            postconditions: Vec::new(),
            notes: None,
        });
    }
    Ok(cases)
}

fn read_legacy_setup(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(io_error)?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect())
}

/// Canonical rendering used by the frozen v1 datasets.
///
/// v1 sorted nested list elements when comparing unordered results. That is
/// preserved here so historical scores stay reproducible, but it is *not* what
/// Benchmark v2 does: see [`canon::rows_match`].
fn legacy_render(rows: &[Vec<Val>], ordered: bool) -> String {
    if rows.iter().all(|row| row.len() == 1) {
        let mut values: Vec<String> = rows
            .iter()
            .map(|row| legacy_canon(&row[0], !ordered))
            .collect();
        if !ordered {
            values.sort();
        }
        format!("[{}]", values.join(", "))
    } else {
        let mut rendered: Vec<String> = rows
            .iter()
            .map(|row| {
                format!(
                    "[{}]",
                    row.iter()
                        .map(|value| legacy_canon(value, !ordered))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect();
        if !ordered {
            rendered.sort();
        }
        format!("[{}]", rendered.join(", "))
    }
}

fn legacy_canon(value: &Val, sort_lists: bool) -> String {
    match value {
        Val::List(items) => {
            let mut rendered: Vec<String> = items
                .iter()
                .map(|item| legacy_canon(item, sort_lists))
                .collect();
            if sort_lists {
                rendered.sort();
            }
            format!("[{}]", rendered.join(", "))
        }
        Val::Map(entries) => {
            let mut rendered: Vec<(String, String)> = entries
                .iter()
                .map(|(key, item)| (format!("\"{key}\""), legacy_canon(item, sort_lists)))
                .collect();
            rendered.sort();
            format!(
                "{{{}}}",
                rendered
                    .iter()
                    .map(|(key, item)| format!("{key}:{item}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Val::Set(items) => {
            let mut rendered: Vec<String> = items
                .iter()
                .map(|item| legacy_canon(item, sort_lists))
                .collect();
            rendered.sort();
            format!("{{{}}}", rendered.join(", "))
        }
        Val::Property { key, value, .. } => {
            format!("p{{{key}={}}}", legacy_canon(value, sort_lists))
        }
        other => canon::render(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case_with(expectation: Expectation, ordered: bool) -> Case {
        Case {
            id: "T-1".into(),
            language: "test".into(),
            category: "read.access".into(),
            features: vec!["test.feature".into()],
            capabilities: vec!["core".into()],
            source_kind: "project".into(),
            source_detail: String::new(),
            fixture: "empty".into(),
            setup: Vec::new(),
            query: "q".into(),
            ordered,
            timeout_ms: 1000,
            scored: true,
            applicable: true,
            inapplicable_reason: None,
            comparison: "exact".into(),
            first_only: false,
            aborted: false,
            expectation,
            postconditions: Vec::new(),
            notes: None,
        }
    }

    #[test]
    fn negative_case_reports_unexpected_success() {
        let case = case_with(
            Expectation::Error {
                stage: "parse".into(),
                pattern: None,
            },
            false,
        );
        let execution = Execution {
            rows: vec![vec![Val::Int(1)]],
            columns: vec!["x".into()],
            post_rows: Vec::new(),
            elapsed_ms: 1,
        };
        let outcome = outcome_for(&case, Ok(execution), false);
        assert_eq!(outcome.status, Status::UnexpectedSuccess);
    }

    #[test]
    fn failing_postcondition_is_not_a_pass() {
        let mut case = case_with(
            Expectation::Rows {
                rows: vec![vec![Val::Int(1)]],
                columns: Vec::new(),
            },
            false,
        );
        case.postconditions = vec![(
            "check".into(),
            Expectation::Rows {
                rows: vec![vec![Val::Int(9)]],
                columns: Vec::new(),
            },
        )];
        let execution = Execution {
            rows: vec![vec![Val::Int(1)]],
            columns: Vec::new(),
            post_rows: vec![vec![vec![Val::Int(2)]]],
            elapsed_ms: 1,
        };
        let outcome = outcome_for(&case, Ok(execution), false);
        assert_eq!(outcome.status, Status::PostconditionFailure);
    }

    #[test]
    fn timeout_is_reported_without_a_result() {
        let case = case_with(
            Expectation::Rows {
                rows: vec![vec![Val::Int(1)]],
                columns: Vec::new(),
            },
            false,
        );
        let outcome = outcome_for(
            &case,
            Err(ExecError::Query {
                stage: Stage::Execute,
                error: "budget".into(),
            }),
            true,
        );
        assert_eq!(outcome.status, Status::Timeout);
    }

    #[test]
    fn v2_row_comparison_keeps_list_order_when_unordered() {
        let actual = vec![vec![Val::List(vec![Val::Int(2), Val::Int(1)])]];
        let expected = vec![vec![Val::List(vec![Val::Int(1), Val::Int(2)])]];
        assert!(!canon::rows_match(&actual, &expected, false));
    }

    #[test]
    fn legacy_render_still_sorts_nested_lists() {
        let rows = vec![vec![Val::List(vec![
            Val::Text("b".into()),
            Val::Text("a".into()),
        ])]];
        assert_eq!(legacy_render(&rows, false), "[[\"a\", \"b\"]]");
    }

    #[test]
    fn parse_error_stage_accepts_any() {
        assert!(Stage::Parse.matches("any"));
        assert!(Stage::Parse.matches("parse"));
        assert!(!Stage::Parse.matches("execute"));
    }

    #[test]
    fn shape_comparison_ignores_values_but_not_arity() {
        let actual = vec![vec![Val::Int(1)], vec![Val::Int(2)]];
        let expected = vec![vec![Val::Vertex(4)], vec![Val::Vertex(7)]];
        assert!(shape_matches(&actual, &expected));
        assert!(!shape_matches(
            &actual,
            &vec![vec![Val::Int(1), Val::Int(2)]]
        ));
    }

    #[test]
    fn shape_match_is_diagnostic_and_never_a_graded_pass() {
        let mut case = case_with(
            Expectation::Rows {
                rows: vec![vec![Val::Vertex(1)]],
                columns: Vec::new(),
            },
            false,
        );
        case.comparison = "shape".into();
        let execution = Execution {
            rows: vec![vec![Val::Int(99)]],
            columns: Vec::new(),
            post_rows: Vec::new(),
            elapsed_ms: 1,
        };
        let outcome = outcome_for(&case, Ok(execution), false);
        assert_eq!(outcome.status, Status::ShapeMatch);

        let mut report = Report::new();
        record_case(&mut report, &case, outcome);
        assert_eq!(report.passed, 0);
        assert_eq!(report.scored, 0);
        assert_eq!(report.strict_passed, 0);
        assert_eq!(report.shape_comparable, 1);
        assert_eq!(report.shape_matched, 1);
    }

    #[test]
    fn benchmark_child_boundary_preserves_typed_expectations() {
        let mut case = case_with(
            Expectation::Rows {
                rows: vec![vec![
                    Val::List(vec![Val::Int(1), Val::Text("x".into())]),
                    Val::Set(vec![Val::Int(2), Val::Int(1)]),
                    Val::Map(vec![("k".into(), Val::Bool(true))]),
                ]],
                columns: vec!["a".into(), "b".into(), "c".into()],
            },
            true,
        );
        case.setup = vec!["setup".into()];
        case.postconditions = vec![(
            "post".into(),
            Expectation::Rows {
                rows: vec![vec![Val::Int(1)]],
                columns: Vec::new(),
            },
        )];
        let decoded = parse_child_case(&child_request_json(&case)).unwrap();
        assert_eq!(decoded.setup, case.setup);
        assert_eq!(decoded.query, case.query);
        assert_eq!(decoded.ordered, case.ordered);
        assert_eq!(decoded.postconditions.len(), 1);
        match decoded.expectation {
            Expectation::Rows { rows, columns } => {
                assert_eq!(
                    rows,
                    match case.expectation {
                        Expectation::Rows { rows, .. } => rows,
                        _ => unreachable!(),
                    }
                );
                assert_eq!(columns, vec!["a", "b", "c"]);
            }
            _ => panic!("row expectation changed across child boundary"),
        }
    }
}
