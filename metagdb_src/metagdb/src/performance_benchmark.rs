//! Fixed single-thread micro-benchmark for storage, queries, and open algorithms.

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use metagdb::assembly;
use metagdb_core::benchmark::BenchmarkComponents;
use metagdb_core::common::ids::{EdgeId, EdgeTypeId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeFilter, EdgeRecord, VertexFilter};
use metagdb_core::l4_compute::algebra::{AlgorithmArgs, ProjectionDirection, ProjectionSpec};
use metagdb_core::{Error, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub dataset: String,
    pub graph: PathBuf,
    pub repeat: usize,
    pub output: PathBuf,
    pub algorithm_plan: Option<PathBuf>,
    /// Fingerprint of the implementation source this run compiles.
    pub revision: String,
    /// Fingerprint of the design vector that configured this run.
    pub design_sha: String,
    /// Fingerprint of the exact graph input used by this run.
    pub graph_sha: String,
    /// Grammar-validated public-language workload templates.
    pub point_query_template: String,
    pub one_hop_query_template: String,
    /// Reuse a scratch database whose stamp matches this run's identity.
    pub reuse_data: bool,
    /// Run the fixed sustained edge-insertion window for write-specialized designs.
    pub edge_insert_window: bool,
}

#[derive(Debug, Clone)]
struct AlgorithmPerfSpec {
    name: String,
    allow_empty_result: bool,
    query_template: String,
    arguments: Vec<(String, PlanValue)>,
}

#[derive(Debug, Clone)]
enum PlanValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
struct Metric {
    name: String,
    status: &'static str,
    samples: usize,
    operations: usize,
    p50_us: f64,
    p95_us: f64,
    p99_us: f64,
    throughput: f64,
    value: f64,
    detail: String,
}

impl Metric {
    fn error(name: &str, error: impl ToString) -> Self {
        Self {
            name: name.into(),
            status: "ERROR",
            samples: 0,
            operations: 0,
            p50_us: 0.0,
            p95_us: 0.0,
            p99_us: 0.0,
            throughput: 0.0,
            value: 0.0,
            detail: clean(&error.to_string()),
        }
    }

    fn scalar(name: &str, value: f64, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: "PASS",
            samples: 1,
            operations: 1,
            p50_us: 0.0,
            p95_us: 0.0,
            p99_us: 0.0,
            throughput: 0.0,
            value,
            detail: detail.into(),
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    if config.repeat < 1 {
        return Err(Error::Other(
            "micro-benchmark repeat must be positive".into(),
        ));
    }
    if !config.graph.is_file() {
        return Err(Error::Other(format!(
            "micro-benchmark graph is missing: {}",
            config.graph.display()
        )));
    }
    let algorithm_specs = match &config.algorithm_plan {
        Some(path) => load_algorithm_plan(path)?,
        None => vec![AlgorithmPerfSpec {
            name: "bfs".into(),
            allow_empty_result: false,
            query_template: String::new(),
            arguments: vec![("source".into(), PlanValue::String("$hotspot_vertex".into()))],
        }],
    };
    if config.dataset.is_empty()
        || !config
            .dataset
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        return Err(Error::Other(
            "micro-benchmark dataset name may contain only ASCII letters, digits, '_' and '-'"
                .into(),
        ));
    }
    let output_dir = config
        .output
        .parent()
        .ok_or_else(|| Error::Other("micro-benchmark output needs a parent directory".into()))?;
    fs::create_dir_all(output_dir).map_err(io_error)?;
    if config.reuse_data && config.revision.is_empty() {
        return Err(Error::Other(
            "--reuse-data needs --revision: reusing a scratch database is only sound \
             when the measurement can prove the code is unchanged"
                .into(),
        ));
    }
    // The scratch database is a fixed path per dataset, and the runner deletes it
    // before importing. Two runs on one dataset would therefore destroy each
    // other's data mid-measurement, so the directory has exactly one owner.
    let scratch_dir = PathBuf::from("target").join("performance-data");
    fs::create_dir_all(&scratch_dir).map_err(io_error)?;
    let _scratch_lock = ScratchLock::acquire(&scratch_dir, &config.dataset)?;
    let data_dir = scratch_dir.join(&config.dataset);
    let stamp_path = scratch_dir.join(format!("{}.stamp", config.dataset));
    let identity = ScratchIdentity::new(&config)?;
    let stamp = fs::read_to_string(&stamp_path).ok();
    let reusing = config.reuse_data
        && data_dir.is_dir()
        && stamp
            .as_deref()
            .map(|text| identity.matches(text))
            .unwrap_or(false);

    println!("=== metaGDB micro-benchmark: {} ===", config.dataset);
    let (components, import_metric) = if reusing {
        let import_us = identity
            .stamp_import_us(stamp.as_deref().unwrap_or(""))
            .unwrap_or(0.0);
        println!(
            "=== reusing {} (import not re-measured; stamp {}) ===",
            data_dir.display(),
            stamp_path.display()
        );
        (
            assembly::build_benchmark_components(&data_dir),
            Metric::scalar(
                "storage.import.us",
                import_us,
                format!(
                    "reused: not measured in this run, recorded value from {}",
                    stamp_path.display()
                ),
            ),
        )
    } else {
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir).map_err(io_error)?;
        }
        let import_started = Instant::now();
        let components =
            assembly::build_benchmark_components_from_graph_file(&config.graph, &data_dir)?;
        let import_us = import_started.elapsed().as_secs_f64() * 1_000_000.0;
        write_stamp(&stamp_path, &identity, import_us)?;
        (
            components,
            Metric::scalar("storage.import.us", import_us, "dataset preparation"),
        )
    };
    let mut metrics = vec![import_metric];

    let isolation = components.txn_manager.default_policy().isolation;
    let read = components.txn_manager.begin(isolation, true)?;
    let mut all_vertices: Vec<_> = components
        .storage
        .scan_vertices(VertexFilter::default(), &read)?
        .map(|vertex| vertex.id)
        .collect();
    if all_vertices.is_empty() {
        return Err(Error::Other(format!(
            "micro-benchmark database {} holds no vertices; the import failed or a \
             reused scratch directory is incomplete",
            data_dir.display()
        )));
    }
    // Keep the benchmark request sequence independent of each implementation's scan order.
    all_vertices.sort_unstable_by_key(|id| id.0);
    let sampled_vertices = evenly_sample(&all_vertices, 2048);
    let (hotspot_vertices, maximum_degree, input_edge_count) =
        highest_degree_vertices(&config.graph, 64)?;
    let hotspot_source = hotspot_vertices[0].0;
    let point_vertices = evenly_sample(&all_vertices, 1200);
    let mut hop_vertices = Vec::new();
    for id in evenly_sample(&all_vertices, 8192) {
        if components
            .storage
            .out_edges(id, EdgeFilter::default(), &read)?
            .next()
            .is_some()
        {
            hop_vertices.push(id);
            if hop_vertices.len() == 1000 {
                break;
            }
        }
    }
    if hop_vertices.is_empty() {
        return Err(Error::Other("graph has no nonempty one-hop source".into()));
    }
    for index in 0..200 {
        hop_vertices.push(hotspot_vertices[index % hotspot_vertices.len()].0);
    }
    let mut sampled_edges = Vec::<EdgeId>::new();
    for vertex in &sampled_vertices {
        for edge in components
            .storage
            .out_edges(*vertex, EdgeFilter::default(), &read)?
            .take(8)
        {
            sampled_edges.push(edge.id);
            if sampled_edges.len() >= 2048 {
                break;
            }
        }
        if sampled_edges.len() >= 2048 {
            break;
        }
    }

    metrics.push(measure(
        "storage.vertex_point_read",
        config.repeat,
        sampled_vertices.len(),
        |index| {
            let id = sampled_vertices[index % sampled_vertices.len()];
            match components.storage.get_vertex(id, &read)? {
                Some(_) => Ok(()),
                None => Err(Error::Other(format!("sampled vertex {} disappeared", id.0))),
            }
        },
    ));
    if sampled_edges.is_empty() {
        metrics.push(Metric::error(
            "storage.edge_point_read",
            "sample contains no edges",
        ));
    } else {
        metrics.push(measure(
            "storage.edge_point_read",
            config.repeat,
            sampled_edges.len(),
            |index| {
                let id = sampled_edges[index % sampled_edges.len()];
                match components.storage.get_edge(id, &read)? {
                    Some(_) => Ok(()),
                    None => Err(Error::Other(format!("sampled edge {} disappeared", id.0))),
                }
            },
        ));
    }

    let degree_total = sampled_vertices.iter().try_fold(0usize, |total, id| {
        Ok::<_, Error>(
            total.saturating_add(
                components
                    .storage
                    .out_edges(*id, EdgeFilter::default(), &read)?
                    .count(),
            ),
        )
    })?;
    metrics.push(measure(
        "storage.out_adjacency",
        config.repeat,
        sampled_vertices.len(),
        |index| {
            let id = sampled_vertices[index % sampled_vertices.len()];
            components
                .storage
                .out_edges(id, EdgeFilter::default(), &read)?
                .count();
            Ok(())
        },
    ));
    metrics.push(measure(
        "storage.hotspot_out_adjacency",
        config.repeat,
        hotspot_vertices.len(),
        |index| {
            let (id, expected) = hotspot_vertices[index % hotspot_vertices.len()];
            if components.storage.get_vertex(id, &read)?.is_none() {
                return Err(Error::Other(format!(
                    "hotspot vertex {} is missing after import",
                    id.0
                )));
            }
            let actual = components
                .storage
                .out_edges(id, EdgeFilter::default(), &read)?
                .count();
            if actual != expected {
                return Err(Error::Other(format!(
                    "hotspot {} has {} outgoing edges after import, expected {}",
                    id.0, actual, expected
                )));
            }
            Ok(())
        },
    ));

    metrics.push(measure("storage.scan", config.repeat, 1, |_| {
        let count = components
            .storage
            .scan_vertices(VertexFilter::default(), &read)?
            .count();
        if count != all_vertices.len() {
            return Err(Error::Other(format!(
                "scan count changed: expected {}, got {count}",
                all_vertices.len()
            )));
        }
        Ok(())
    }));

    let projection_spec = ProjectionSpec {
        vertex_labels: None,
        edge_types: None,
        direction: ProjectionDirection::Outgoing,
        vertex_properties: None,
    };
    let projection_started = Instant::now();
    let projection = (components.projection_factory)(&*components.storage, &read, &projection_spec);
    match projection {
        Ok(projection) => {
            metrics.push(Metric::scalar(
                "algorithm.projection_build.us",
                projection_started.elapsed().as_secs_f64() * 1_000_000.0,
                format!(
                    "vertices={} edges={}",
                    projection.num_vertices(),
                    projection.num_edges()
                ),
            ));
            for spec in &algorithm_specs {
                let slug = metric_slug(&spec.name);
                let metric =
                    if config.algorithm_plan.is_none() && spec.name.eq_ignore_ascii_case("bfs") {
                        "algorithm.bfs".to_string()
                    } else {
                        format!("algorithm.{slug}.kernel")
                    };
                match components
                    .algorithms
                    .iter()
                    .find(|algorithm| algorithm.name().eq_ignore_ascii_case(&spec.name))
                {
                    Some(algorithm) => {
                        let args = resolve_arguments(
                            spec,
                            hotspot_source,
                            all_vertices[0],
                            all_vertices.len(),
                        );
                        match args {
                            Ok(args) => metrics.push(measure(&metric, config.repeat, 1, |_| {
                                let result = algorithm.execute(&*projection, &args)?;
                                if !spec.allow_empty_result && result.rows.is_empty() {
                                    return Err(Error::Other(format!(
                                        "{} returned no rows",
                                        spec.name
                                    )));
                                }
                                Ok(())
                            })),
                            Err(error) => metrics.push(Metric::error(&metric, error)),
                        }
                    }
                    None => metrics.push(Metric::error(
                        &metric,
                        format!("{} is not registered", spec.name),
                    )),
                }
            }
        }
        Err(error) => {
            metrics.push(Metric::error("algorithm.projection_build.us", error));
            for spec in &algorithm_specs {
                let metric =
                    if config.algorithm_plan.is_none() && spec.name.eq_ignore_ascii_case("bfs") {
                        "algorithm.bfs".to_string()
                    } else {
                        format!("algorithm.{}.kernel", metric_slug(&spec.name))
                    };
                metrics.push(Metric::error(&metric, "projection construction failed"));
            }
        }
    }
    components.txn_manager.commit(read)?;

    let source = hotspot_source;
    drop(components);
    let reopen_started = Instant::now();
    let reopened = assembly::build_benchmark_components(&data_dir);
    let reopen_result = (|| -> Result<()> {
        let reopen_read = reopened.txn_manager.begin(isolation, true)?;
        let reopened_source = reopened.storage.get_vertex(source, &reopen_read)?;
        reopened.txn_manager.commit(reopen_read)?;
        if reopened_source.is_none() {
            return Err(Error::Other(format!(
                "reopened database lost hotspot vertex {}",
                source.0
            )));
        }
        Ok(())
    })();
    metrics.push(match reopen_result {
        Ok(()) => Metric::scalar(
            "storage.reopen",
            reopen_started.elapsed().as_secs_f64() * 1_000_000.0,
            "constructor reopen plus persisted point read",
        ),
        Err(error) => Metric::error("storage.reopen", error),
    });
    drop(reopened);

    let runtime = assembly::build_runtime_clean(&data_dir.to_string_lossy())?;
    let point_queries: Vec<_> = point_vertices
        .iter()
        .map(|id| render_query_template("point_read", &config.point_query_template, *id))
        .collect::<Result<_>>()?;
    let hop_queries: Vec<_> = hop_vertices
        .iter()
        .map(|id| render_query_template("one_hop", &config.one_hop_query_template, *id))
        .collect::<Result<_>>()?;
    metrics.push(measure(
        "query.point_read",
        config.repeat,
        point_queries.len(),
        |index| {
            let result = runtime.execute_query(&point_queries[index % point_queries.len()])?;
            if result.rows.is_empty() {
                return Err(Error::Other("point query returned no rows".into()));
            }
            Ok(())
        },
    ));
    metrics.push(measure(
        "query.one_hop",
        config.repeat,
        hop_queries.len(),
        |index| {
            let result = runtime.execute_query(&hop_queries[index % hop_queries.len()])?;
            if result.rows.is_empty() {
                return Err(Error::Other("one-hop query returned no rows".into()));
            }
            Ok(())
        },
    ));
    for spec in &algorithm_specs {
        if spec.query_template.is_empty() {
            continue;
        }
        let query = resolve_query_template(
            &spec.query_template,
            hotspot_source,
            all_vertices[0],
            all_vertices.len(),
        );
        let metric = format!("algorithm.{}.e2e", metric_slug(&spec.name));
        metrics.push(measure(&metric, config.repeat, 1, |_| {
            let result = runtime.execute_query(&query)?;
            if !spec.allow_empty_result && result.rows.is_empty() {
                return Err(Error::Other(format!(
                    "{} language query returned no rows",
                    spec.name
                )));
            }
            Ok(())
        }));
    }
    drop(runtime);

    let write_components = assembly::build_benchmark_components(&data_dir);
    let edge_type = {
        let read = write_components.txn_manager.begin(isolation, true)?;
        let sample_id = sampled_edges
            .first()
            .ok_or_else(|| Error::Other("commit benchmark needs a sampled edge type".into()))?;
        let edge = write_components
            .storage
            .get_edge(*sample_id, &read)?
            .ok_or_else(|| {
                Error::Other("sampled edge disappeared before commit benchmark".into())
            })?;
        write_components.txn_manager.commit(read)?;
        edge.etype
    };
    let src = all_vertices[0];
    let dst = all_vertices[if all_vertices.len() > 1 { 1 } else { 0 }];
    let mut next_edge_id = input_edge_count
        .checked_add(1)
        .ok_or_else(|| Error::Other("edge ID range is exhausted".into()))?;
    let mut commit_metric = measure("transaction.commit", config.repeat, 32, |_| {
        let txn = write_components.txn_manager.begin(isolation, false)?;
        write_components.storage.insert_edge(
            EdgeRecord {
                id: EdgeId(next_edge_id),
                src,
                dst,
                etype: edge_type,
                properties: PropertyMap::new(),
            },
            &txn,
        )?;
        next_edge_id += 1;
        write_components.txn_manager.commit(txn)
    });
    commit_metric.detail = "one edge insertion and durable commit per transaction".into();
    metrics.push(commit_metric);
    let mut inserted_window_edges = Vec::new();
    if config.edge_insert_window {
        let name = "transaction.edge_insert_window.us_per_edge";
        let result = measure_edge_insert_window(
            &write_components,
            &all_vertices,
            edge_type,
            next_edge_id,
            config.repeat,
        );
        metrics.push(match result {
            Ok((metric, edges)) => {
                inserted_window_edges = edges;
                metric
            }
            Err(error) => Metric::error(name, error),
        });
    }
    drop(write_components);
    if !inserted_window_edges.is_empty() {
        if let Err(error) = verify_inserted_edges(&data_dir, &inserted_window_edges) {
            if let Some(metric) = metrics
                .iter_mut()
                .find(|metric| metric.name == "transaction.edge_insert_window.us_per_edge")
            {
                *metric = Metric::error("transaction.edge_insert_window.us_per_edge", error);
            }
        }
    }

    metrics.push(Metric::scalar(
        "storage.disk_bytes",
        directory_size(&data_dir)? as f64,
        "database directory after the fixed workload",
    ));
    let average_degree = if sampled_vertices.is_empty() {
        0.0
    } else {
        degree_total as f64 / sampled_vertices.len() as f64
    };
    metrics.push(Metric::scalar(
        "dataset.sample_average_out_degree",
        average_degree,
        format!("{} sampled vertices", sampled_vertices.len()),
    ));
    metrics.push(Metric::scalar(
        "dataset.maximum_out_degree",
        maximum_degree as f64,
        "maximum source degree parsed from the input graph",
    ));

    write_report(&config, &metrics)?;
    let errors = metrics
        .iter()
        .filter(|metric| metric.status == "ERROR")
        .count();
    // Scratch databases are deleted after their report so baseline and candidate
    // runs leave evidence rather than duplicate graphs behind. A run that asked
    // for reuse keeps the database and its stamp so the next measurement of the
    // same revision can skip the import.
    if config.reuse_data {
        println!(
            "Kept {} for reuse; remove it (or re-run without --reuse-data) to reclaim the space",
            data_dir.display()
        );
    } else {
        fs::remove_dir_all(&data_dir).map_err(io_error)?;
        let _ = fs::remove_file(&stamp_path);
    }
    println!("Report: {}", config.output.display());
    println!("Metrics: {}, errors: {}", metrics.len(), errors);
    Ok(())
}

fn load_algorithm_plan(path: &Path) -> Result<Vec<AlgorithmPerfSpec>> {
    let text = fs::read_to_string(path).map_err(io_error)?;
    let mut lines = text.lines();
    if lines.next() != Some("metagdb-algorithm-plan-v1") {
        return Err(Error::Other("invalid runtime algorithm plan header".into()));
    }
    let mut specs = Vec::<AlgorithmPerfSpec>::new();
    for (line_number, line) in lines.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        match fields.as_slice() {
            ["A", name, allow_empty, query] => {
                let name = decode_hex(name)?;
                if specs
                    .iter()
                    .any(|spec| spec.name.eq_ignore_ascii_case(&name))
                {
                    return Err(Error::Other(format!("algorithm plan repeats {name}")));
                }
                specs.push(AlgorithmPerfSpec {
                    name,
                    allow_empty_result: match *allow_empty {
                        "0" => false,
                        "1" => true,
                        _ => return Err(Error::Other("invalid allow-empty flag".into())),
                    },
                    query_template: decode_hex(query)?,
                    arguments: Vec::new(),
                });
            }
            ["P", name, key, kind, value] => {
                let name = decode_hex(name)?;
                let spec = specs
                    .iter_mut()
                    .find(|spec| spec.name == name)
                    .ok_or_else(|| Error::Other(format!("parameter precedes algorithm {name}")))?;
                let rendered = decode_hex(value)?;
                let value =
                    match *kind {
                        "null" => PlanValue::Null,
                        "bool" => PlanValue::Bool(rendered.parse::<bool>().map_err(|_| {
                            Error::Other("invalid boolean algorithm parameter".into())
                        })?),
                        "int" => PlanValue::Int(rendered.parse::<i64>().map_err(|_| {
                            Error::Other("invalid integer algorithm parameter".into())
                        })?),
                        "float" => PlanValue::Float(rendered.parse::<f64>().map_err(|_| {
                            Error::Other("invalid float algorithm parameter".into())
                        })?),
                        "string" => PlanValue::String(rendered),
                        _ => return Err(Error::Other(format!("unknown parameter kind {kind}"))),
                    };
                spec.arguments.push((decode_hex(key)?, value));
            }
            _ => {
                return Err(Error::Other(format!(
                    "invalid runtime algorithm plan line {}",
                    line_number + 2
                )))
            }
        }
    }
    Ok(specs)
}

fn decode_hex(value: &str) -> Result<String> {
    if value.len() % 2 != 0 {
        return Err(Error::Other("invalid hex value in algorithm plan".into()));
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for index in (0..value.len()).step_by(2) {
        bytes.push(
            u8::from_str_radix(&value[index..index + 2], 16)
                .map_err(|_| Error::Other("invalid hex value in algorithm plan".into()))?,
        );
    }
    String::from_utf8(bytes)
        .map_err(|_| Error::Other("algorithm plan contains invalid UTF-8".into()))
}

fn resolve_arguments(
    spec: &AlgorithmPerfSpec,
    hotspot: VertexId,
    first: VertexId,
    vertex_count: usize,
) -> Result<AlgorithmArgs> {
    let mut args = AlgorithmArgs::default();
    for (key, value) in &spec.arguments {
        let resolved = match value {
            PlanValue::Null => PropertyValue::Null,
            PlanValue::Bool(value) => PropertyValue::Bool(*value),
            PlanValue::Int(value) => PropertyValue::Int(*value),
            PlanValue::Float(value) => PropertyValue::Float(*value),
            PlanValue::String(value) if value == "$hotspot_vertex" => {
                PropertyValue::Int(hotspot.0 as i64)
            }
            PlanValue::String(value) if value == "$first_vertex" => {
                PropertyValue::Int(first.0 as i64)
            }
            PlanValue::String(value) if value == "$vertex_count" => {
                PropertyValue::Int(vertex_count as i64)
            }
            PlanValue::String(value) if value.starts_with('$') => {
                return Err(Error::Other(format!(
                    "unsupported algorithm benchmark placeholder: {value}"
                )))
            }
            PlanValue::String(value) => PropertyValue::Text(value.clone()),
        };
        args.params.insert(key.clone(), resolved);
    }
    Ok(args)
}

fn resolve_query_template(
    template: &str,
    hotspot: VertexId,
    first: VertexId,
    vertex_count: usize,
) -> String {
    template
        .replace("{{hotspot_vertex}}", &hotspot.0.to_string())
        .replace("{{first_vertex}}", &first.0.to_string())
        .replace("{{vertex_count}}", &vertex_count.to_string())
}

fn metric_slug(name: &str) -> String {
    let mut slug = String::new();
    let mut underscore = false;
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            underscore = false;
        } else if !underscore && !slug.is_empty() {
            slug.push('_');
            underscore = true;
        }
    }
    while slug.ends_with('_') {
        slug.pop();
    }
    slug
}

// ---------------------------------------------------------------------------
// Scratch-directory ownership and reuse
// ---------------------------------------------------------------------------

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

/// Linux process start tick, used with the PID to reject PID reuse.
fn process_start(pid: u32) -> Option<String> {
    let text = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let tail = text.get(text.rfind(')')? + 2..)?;
    tail.split_whitespace().nth(19).map(str::to_owned)
}

/// Sole ownership of one dataset's scratch directory.
///
/// The runner deletes `target/performance-data/<dataset>` before importing, so a
/// second measurement of the same dataset would delete the first one's database
/// mid-run: it would not fail, it would silently report numbers from a database
/// that no longer exists. The lock makes that a refusal instead.
struct ScratchLock {
    _file: File,
}

impl ScratchLock {
    fn acquire(scratch_dir: &Path, dataset: &str) -> Result<Self> {
        let path = scratch_dir.join(format!("{dataset}.lock"));
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)
            .map_err(io_error)?;
        if let Err(error) = file.try_lock() {
            let owner = fs::read_to_string(&path).unwrap_or_default();
            let pid = owner
                .lines()
                .find_map(|line| line.strip_prefix("pid="))
                .unwrap_or("?");
            return Err(Error::Other(format!(
                "another micro-benchmark owns dataset {dataset} (pid {pid}): {error}; \
                 wait for it to finish"
            )));
        }
        let start = process_start(std::process::id()).ok_or_else(|| {
            Error::Other(
                "cannot read this process identity from /proc; refusing an unsafe lock".into(),
            )
        })?;
        file.set_len(0).map_err(io_error)?;
        file.write_all(
            format!(
                "pid={}\nprocess_start={}\nstarted_unix={}\n",
                std::process::id(),
                start,
                unix_now()
            )
            .as_bytes(),
        )
        .map_err(io_error)?;
        file.sync_all().map_err(io_error)?;
        Ok(Self { _file: file })
    }
}

/// What a scratch database was built from.
///
/// Reuse is only sound when the graph, the implementation source and the design
/// vector are all unchanged, so all three are part of the identity.
struct ScratchIdentity {
    dataset: String,
    graph_sha: String,
    revision: String,
    design_sha: String,
}

impl ScratchIdentity {
    fn new(config: &Config) -> Result<Self> {
        if config.graph_sha.is_empty() {
            return Err(Error::Other(
                "micro-benchmark requires --graph-sha for reproducible scratch reuse".into(),
            ));
        }
        Ok(Self {
            dataset: config.dataset.clone(),
            graph_sha: config.graph_sha.clone(),
            revision: config.revision.clone(),
            design_sha: config.design_sha.clone(),
        })
    }

    fn encode(&self) -> String {
        format!(
            "dataset={}\ngraph_sha={}\nrevision={}\ndesign_sha={}\n",
            self.dataset, self.graph_sha, self.revision, self.design_sha
        )
    }

    fn matches(&self, stamp: &str) -> bool {
        stamp.starts_with(&self.encode())
    }

    fn stamp_import_us(&self, stamp: &str) -> Option<f64> {
        stamp
            .lines()
            .find_map(|line| line.strip_prefix("import_us="))
            .and_then(|value| value.trim().parse::<f64>().ok())
    }
}

fn write_stamp(path: &Path, identity: &ScratchIdentity, import_us: f64) -> Result<()> {
    fs::write(
        path,
        format!(
            "{}import_us={import_us:.1}\nwritten_unix={}\n",
            identity.encode(),
            unix_now()
        ),
    )
    .map_err(io_error)
}

fn measure<F>(name: &str, repeat: usize, operations_per_repeat: usize, mut operation: F) -> Metric
where
    F: FnMut(usize) -> Result<()>,
{
    let warmups = operations_per_repeat.min(128);
    for index in 0..warmups {
        if let Err(error) = operation(index) {
            return Metric::error(name, error);
        }
    }
    let mut samples = Vec::with_capacity(repeat * operations_per_repeat);
    let total_started = Instant::now();
    for iteration in 0..repeat {
        for index in 0..operations_per_repeat {
            let started = Instant::now();
            if let Err(error) = operation(iteration * operations_per_repeat + index) {
                return Metric::error(name, error);
            }
            samples.push(started.elapsed().as_nanos() as u64);
        }
    }
    let elapsed = total_started.elapsed().as_secs_f64();
    samples.sort_unstable();
    let operations = samples.len();
    Metric {
        name: name.into(),
        status: "PASS",
        samples: operations,
        operations,
        p50_us: percentile(&samples, 0.50) / 1000.0,
        p95_us: percentile(&samples, 0.95) / 1000.0,
        p99_us: percentile(&samples, 0.99) / 1000.0,
        throughput: if elapsed > 0.0 {
            operations as f64 / elapsed
        } else {
            0.0
        },
        value: 0.0,
        detail: String::new(),
    }
}

/// A fixed-count sustained write window. The scalar is elapsed microseconds per
/// committed edge, so the standard lower-is-better comparator directly optimizes
/// the reciprocal of the reported edge throughput.
fn measure_edge_insert_window(
    components: &BenchmarkComponents,
    vertices: &[VertexId],
    edge_type: EdgeTypeId,
    mut next_edge_id: u64,
    repeat: usize,
) -> Result<(Metric, Vec<(EdgeId, VertexId, VertexId)>)> {
    const EDGES_PER_WINDOW: usize = 1000;
    const EDGES_PER_COMMIT: usize = 100;
    let isolation = components.txn_manager.default_policy().isolation;
    let mut per_edge_ns = Vec::with_capacity(repeat);
    let mut inserted = Vec::with_capacity(repeat * EDGES_PER_WINDOW);
    let total_started = Instant::now();
    for window in 0..repeat {
        let started = Instant::now();
        for batch in 0..(EDGES_PER_WINDOW / EDGES_PER_COMMIT) {
            let txn = components.txn_manager.begin(isolation, false)?;
            for offset in 0..EDGES_PER_COMMIT {
                let index = window * EDGES_PER_WINDOW + batch * EDGES_PER_COMMIT + offset;
                let sweep = index / vertices.len();
                let src = vertices[(index * 37 + sweep * 17 + 13) % vertices.len()];
                let dst = vertices[(index * 101 + sweep * 53 + 53) % vertices.len()];
                let id = EdgeId(next_edge_id);
                components.storage.insert_edge(
                    EdgeRecord {
                        id,
                        src,
                        dst,
                        etype: edge_type,
                        properties: PropertyMap::new(),
                    },
                    &txn,
                )?;
                inserted.push((id, src, dst));
                next_edge_id = next_edge_id.checked_add(1).ok_or_else(|| {
                    Error::Other("edge ID range is exhausted during write window".into())
                })?;
            }
            components.txn_manager.commit(txn)?;
        }
        per_edge_ns.push((started.elapsed().as_nanos() / EDGES_PER_WINDOW as u128) as u64);
    }
    let elapsed_s = total_started.elapsed().as_secs_f64();
    if elapsed_s <= 0.0 {
        return Err(Error::Other(
            "edge insert window elapsed time is zero".into(),
        ));
    }
    per_edge_ns.sort_unstable();
    let edge_count = inserted.len();
    Ok((Metric {
        name: "transaction.edge_insert_window.us_per_edge".into(),
        status: "PASS",
        samples: repeat,
        operations: edge_count,
        p50_us: percentile(&per_edge_ns, 0.50) / 1000.0,
        p95_us: percentile(&per_edge_ns, 0.95) / 1000.0,
        p99_us: percentile(&per_edge_ns, 0.99) / 1000.0,
        throughput: edge_count as f64 / elapsed_s,
        value: elapsed_s * 1_000_000.0 / edge_count as f64,
        detail: format!(
            "{} windows, {} inserted edges/window, {} edges/commit; includes begin, insert and durable commit; reopen/readback verified after timing",
            repeat, EDGES_PER_WINDOW, EDGES_PER_COMMIT
        ),
    }, inserted))
}

fn verify_inserted_edges(data_dir: &Path, expected: &[(EdgeId, VertexId, VertexId)]) -> Result<()> {
    let reopened = assembly::build_benchmark_components(data_dir);
    let isolation = reopened.txn_manager.default_policy().isolation;
    let read = reopened.txn_manager.begin(isolation, true)?;
    for &(id, src, dst) in expected {
        let edge = reopened
            .storage
            .get_edge(id, &read)?
            .ok_or_else(|| Error::Other(format!("committed edge {} missing after reopen", id.0)))?;
        if edge.src != src || edge.dst != dst {
            return Err(Error::Other(format!(
                "committed edge {} has wrong endpoints after reopen",
                id.0
            )));
        }
    }
    reopened.txn_manager.commit(read)
}

fn percentile(values: &[u64], percentile: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let index = ((values.len() - 1) as f64 * percentile).round() as usize;
    values[index] as f64
}

fn evenly_sample(values: &[VertexId], maximum: usize) -> Vec<VertexId> {
    if values.len() <= maximum {
        return values.to_vec();
    }
    (0..maximum)
        .map(|index| values[index * values.len() / maximum])
        .collect()
}

fn highest_degree_vertices(
    path: &Path,
    maximum: usize,
) -> Result<(Vec<(VertexId, usize)>, usize, u64)> {
    let input = File::open(path).map_err(io_error)?;
    let mut degrees = HashMap::<u64, usize>::new();
    let mut edge_count = 0u64;
    for line in BufReader::new(input).lines() {
        let line = line.map_err(io_error)?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("e") {
            continue;
        }
        edge_count += 1;
        let source = fields
            .next()
            .ok_or_else(|| Error::Other("edge line is missing its source vertex".into()))?
            .parse::<u64>()
            .map_err(|_| Error::Other("edge source vertex is not an unsigned integer".into()))?;
        *degrees.entry(source).or_default() += 1;
    }
    let mut ranked: Vec<_> = degrees.into_iter().collect();
    ranked.sort_unstable_by(|(left_id, left_degree), (right_id, right_degree)| {
        right_degree
            .cmp(left_degree)
            .then_with(|| left_id.cmp(right_id))
    });
    let maximum_degree = ranked.first().map(|(_, degree)| *degree).unwrap_or(0);
    let selected: Vec<_> = ranked
        .into_iter()
        .take(maximum)
        .map(|(id, degree)| {
            id.checked_add(1)
                .map(|storage_id| (VertexId(storage_id), degree))
                .ok_or_else(|| Error::Other("graph vertex ID cannot be mapped to storage".into()))
        })
        .collect::<Result<_>>()?;
    if selected.is_empty() {
        return Err(Error::Other(
            "micro-benchmark graph contains no edges for hotspot sampling".into(),
        ));
    }
    Ok((selected, maximum_degree, edge_count))
}

fn render_query_template(name: &str, template: &str, source: VertexId) -> Result<String> {
    if template.matches("{source_id}").count() != 1 {
        return Err(Error::Other(format!(
            "performance template {name} must contain exactly one {{source_id}} placeholder"
        )));
    }
    Ok(template.replace("{source_id}", &source.0.to_string()))
}

fn write_report(config: &Config, metrics: &[Metric]) -> Result<()> {
    let mut text = String::from(
        "dataset\tmetric\tstatus\tsamples\toperations\tp50_us\tp95_us\tp99_us\tthroughput_ops_s\tvalue\tdetail\n",
    );
    for metric in metrics {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}\n",
            config.dataset,
            metric.name,
            metric.status,
            metric.samples,
            metric.operations,
            metric.p50_us,
            metric.p95_us,
            metric.p99_us,
            metric.throughput,
            metric.value,
            clean(&metric.detail),
        ));
    }
    fs::write(&config.output, text).map_err(io_error)
}

fn directory_size(path: &Path) -> Result<u64> {
    let mut total = 0u64;
    for entry in fs::read_dir(path).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let metadata = entry.metadata().map_err(io_error)?;
        if metadata.is_dir() {
            total = total.saturating_add(directory_size(&entry.path())?);
        } else {
            total = total.saturating_add(metadata.len());
        }
    }
    Ok(total)
}

fn clean(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

fn io_error(error: std::io::Error) -> Error {
    Error::Other(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratch_lock_is_exclusive_and_ignores_stale_metadata() {
        let directory = std::env::temp_dir().join(format!(
            "metagdb-scratch-lock-{}-{}",
            std::process::id(),
            unix_now()
        ));
        fs::create_dir_all(&directory).unwrap();
        let first = ScratchLock::acquire(&directory, "graph").unwrap();
        assert!(ScratchLock::acquire(&directory, "graph").is_err());
        drop(first);

        let path = directory.join("graph.lock");
        fs::write(
            &path,
            format!(
                "pid={}\nprocess_start={}-stale\n",
                std::process::id(),
                process_start(std::process::id()).unwrap()
            ),
        )
        .unwrap();
        let reacquired = ScratchLock::acquire(&directory, "graph").unwrap();
        drop(reacquired);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn open_algorithm_plan_preserves_arbitrary_names_arguments_and_query() {
        let path =
            std::env::temp_dir().join(format!("metagdb-algorithm-plan-{}.txt", std::process::id()));
        fs::write(
            &path,
            concat!(
                "metagdb-algorithm-plan-v1\n",
                "A\t6d795f637573746f6d2e616c676f\t0\t672e63616c6c28276d795f637573746f6d2e616c676f272c205b27736f75726365273a207b7b686f7473706f745f7665727465787d7d5d29\n",
                "P\t6d795f637573746f6d2e616c676f\t736f75726365\tstring\t24686f7473706f745f766572746578\n",
                "P\t6d795f637573746f6d2e616c676f\t616c706861\tfloat\t302e3835\n",
            ),
        )
        .unwrap();
        let plan = load_algorithm_plan(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].name, "my_custom.algo");
        assert_eq!(metric_slug(&plan[0].name), "my_custom_algo");
        let args = resolve_arguments(&plan[0], VertexId(9), VertexId(1), 20).unwrap();
        assert_eq!(args.get_i64("source"), Some(9));
        assert!(
            matches!(args.get("alpha"), Some(PropertyValue::Float(v)) if (*v - 0.85).abs() < 1e-9)
        );
        assert_eq!(
            resolve_query_template(&plan[0].query_template, VertexId(9), VertexId(1), 20),
            "g.call('my_custom.algo', ['source': 9])"
        );
    }
}
