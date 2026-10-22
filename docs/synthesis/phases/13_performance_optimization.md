# Phase 13 — measured performance optimization

**Goal:** improve the design vector's primary workload on its intended graph
shapes without weakening correctness, durability, or other graph scenarios.

**Read:** frozen design, `benchmark/performance/README.md`, current candidate,
phase-12 correctness evidence, and measured performance reports. Read a storage
or execution playbook only when a measured bottleneck points to it.

**Prerequisite:** phase 12 passed. Never optimize a semantically incorrect system.

**Actions:**

1. Run the full selected dataset baseline into stable report directories. A
   quick iteration may begin with one run, but final evidence requires two fresh,
   independent measurements of the same baseline revision, for example
   `target/perf-baseline-1` and `target/perf-baseline-2`.
   The runner reads `.synthesis/algorithm_plan.json` and measures every
   required algorithm as both kernel and public-language end-to-end metrics.
   Every report directory gets a `run.json` source fingerprint.
2. The candidate already has isolated history initialized by `synthesis.py init`
   and named checkpoints for every passed phase. Mark the phase-12 correct
   baseline with `python3 scripts/snapshot.py mark-best <crate>`. Its Git directory
   lives under ignored `generated/.snapshots/`; it cannot commit, reset, stash,
   or change the outer research repository.
3. Rank opportunities by `dataset_weights[dataset] * workload[operation]`. Use
   the report to form one concrete implementation hypothesis. Change one coherent
   mechanism at a time; never remove fsync, validation, locking, recovery, or
   required semantics to gain speed.
   For full-set candidate decisions, inspect `topology_robust_gain` and its
   per-dataset `topology_log_gain` alongside `overall_ratio`. The design fixes
   `topology_downside_penalty` before measurement; do not tune it after seeing
   candidate results.
4. Checkpoint the candidate with `python3 scripts/snapshot.py checkpoint <crate>
   --name perf_<iteration> --label "<hypothesis>"`. Run a focused dataset first
   using `--datasets <name>`, then run `verify`, the language benchmark, and the
   non-diagnostic system benchmark.
   Keep the measurement budget in mind: one dataset at a time, one measurement
   at a time, and a focused subset while iterating. Full-set runs belong to the
   comparisons that decide something. Add `--reuse-data` to re-measure the *same*
   revision: the runner then reuses an already imported scratch database whose
   graph, source fingerprint and design hash all match, and reports the import
   metric as `reused` instead of re-measuring it. A reused metric is not
   acceptance evidence, and the comparator refuses one unless `--allow-reused`
   says the comparison is exploratory.
   `scripts/snapshot.py restore`/`restore-best` refuse to run while a
   micro-benchmark is in flight, because restoring the work tree under a running
   measurement silently reports a number for code that no longer exists. Both
   commands use one atomic candidate lock, held from compilation through the
   final report.
5. If correctness regresses, run `scripts/snapshot.py restore-best <crate>`.
   Otherwise run every design dataset into a new directory and compare it with
   the original baseline using `build.py compare-performance`.
6. Read the comparison as a measurement, not a verdict:

   - `ACCEPT` — the candidate has cleared the fixed baseline and all guardrails.
     Compare its `topology_robust_gain` with the current best under the same
     frozen design and dataset set; mark the checkpoint with
     `python3 scripts/snapshot.py mark-best <crate>` only if the score improves.
     Preserve both comparison reports in `decisions.md`.
   - `PROVISIONAL` (exit code 3) — the reported failures are larger than the
     configured cap but the comparator has not seen them reproduced. Measure
     again: run the same baseline and candidate revisions into new directories
     and compare with both sides' directories passed (repeat `--baseline` and
     `--candidate`) plus `--confirm-with <previous comparison json>`. The
     comparator rejects a confirmation that reuses a prior directory or changes
     the revision, design, repeat count, crate, or dataset contents. Replicate directories also
     give every metric a measured noise floor, so `~ within noise floor` notes
     mean the difference is inside what this machine reproduces for the same
     code and is not evidence of a regression. Only failures that survive that
     independent measurement are regressions.
     Every directory on one side must name the same revision in its `run.json`.
     Replicates are measurements of *one* binary; the comparator refuses a side
     that mixes revisions instead of averaging two different programs, so
     re-measure rather than reusing a directory from an earlier hypothesis.
   - `REJECT` — restore `best` without spending a `no_improvement_patience`
     iteration on it; a confirmed regression is a rejected experiment.

   `INCONCLUSIVE` (exit code 4) means a required metric is
   `noise-limited`: the same code varies more than its configured cap. First
   improve the measurement conditions or repetition count and measure again. If
   two fresh runs per side still produce only noise-limited metrics, with no
   confirmed, structural, or tentative failure, record the spread and state that
   the experiment does not establish a performance improvement. The workflow may
   then close phase 13 as `completed_with_limitations`; it is never an
   accepted performance result. Record every hypothesis, focused result, full
   score, exit status, and decision in `decisions.md`.

   Wait for a background measurement by checking its output file or by process
   name (`pgrep -x metagdb`); never poll with `pgrep -f "<pattern>"` whose
   pattern also appears in the polling command line, because that matches the
   waiter itself and never becomes false.
7. Stop after `max_optimization_iterations`, after
   `no_improvement_patience` consecutive rejected hypotheses, or when all absolute
   targets pass and no remaining measured bottleneck has a safe candidate-side
   change. Restore `best`, rerun all correctness gates and the complete performance
   suite, and write the final evidence.

**Edit scope:** candidate crate and candidate `.synthesis/` reports only. `target/`
reports and the ignored snapshot Git are runtime evidence, not research commits.

**Artifacts:** baseline reports, final reports, comparison JSON, isolated snapshot
history, final correctness rerun, and completed `final_report.md`.

**Gate:** the candidate is at the `best` snapshot; all correctness gates still
pass; two fresh baseline runs and two fresh final-candidate runs use one design,
identical datasets and one revision per side; every configured absolute target
passes. An `ACCEPTED` comparison completes the phase normally. A comparison
that is solely `INCONCLUSIVE` from noise-limited metrics may complete as
`completed_with_limitations` only when no real or tentative failure remains
and the Performance section explicitly says it cannot claim an improvement.
Both outcomes require current structured evidence in
`.synthesis/performance_evidence.json`.

**Phase snapshot:** after the final gate passes, run `python3
scripts/synthesis.py advance <crate> --phase 13_performance_optimization
--review <candidate>/.synthesis/reviews/13_performance_optimization.json`. The successful
transition records the completed report and final state as
`checkpoints/13_performance_optimization`. Use only `scripts/snapshot.py` for
intra-phase experiments; never use the outer research Git.

**Next:** complete.
