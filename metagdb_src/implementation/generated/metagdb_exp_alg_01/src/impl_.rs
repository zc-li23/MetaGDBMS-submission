//! The framework-facing implementation crate: `MetagdbImpl`.
//!
//! `scripts/build.py use` renders an assembly that constructs this type and
//! injects its four fields into the framework runtime.  The type therefore has
//! exactly the shape `docs/implementation_contract.md` specifies, and its three
//! entry points carry the three behaviours the framework delegates:
//!
//! * `new()` for an isolated runtime,
//! * `with_base_dir()` for the persistent interactive shell,
//! * `build_from_graph_file()` for `cargo run -- --build <file.graph>`,
//! * `demo_test()` for `cargo run -- --demo`.
//!
//! Everything the demo and the benchmarks exercise goes through the same
//! components: schema, MVCC storage, transaction manager, algorithms, the four
//! query stages, and the CSR projection.

use std::path::PathBuf;
use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::olap::AnalyticalAlgorithm;
use metagdb_core::{Error, Result};

use crate::compute::default_algorithms;
use crate::db::Db;
use crate::schema::SchemaRegistry;
use crate::storage::{import_graph_file, MvccStorage};
use crate::txn::MetagdbTxnManager;

/// The candidate's implementation of the framework contract.
pub struct MetagdbImpl {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
}

impl Default for MetagdbImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl MetagdbImpl {
    /// An isolated, non-persistent runtime.
    pub fn new() -> Self {
        Self::from_db(Db::in_memory())
    }

    /// Open (or create) the database stored under `base_dir`.
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        // The framework's constructor is infallible, so an unusable directory
        // is reported by the first statement rather than by a panic here.
        match Db::open(&base_dir) {
            Ok(db) => Self::from_db(db),
            Err(error) => {
                eprintln!("metagdb_exp_alg_01: cannot open {}: {error}", base_dir.display());
                Self::new()
            }
        }
    }

    /// Import a `.graph` file into `base_dir` and return the opened runtime.
    pub fn build_from_graph_file(graph_path: PathBuf, base_dir: PathBuf) -> Result<Self> {
        let db = Db::open(&base_dir)?;
        import_graph_file(&db, &SchemaRegistry::new(), &graph_path)?;
        Ok(Self::from_db(db))
    }

    fn from_db(db: Db) -> Self {
        Self {
            schema: Arc::new(SchemaRegistry::new()),
            storage: Arc::new(MvccStorage::new(db.clone())),
            txn_manager: Arc::new(MetagdbTxnManager::new(db)),
            algorithms: default_algorithms(),
        }
    }

    /// Query the graph by running one statement through the public runtime.
    ///
    /// This is the same four-stage path the benchmark drives; it exists so the
    /// demo can assert on real results instead of on internal state.
    pub fn query(&self, source: &str) -> Result<metagdb_core::l4_compute::engine::ResultStream> {
        crate::runtime::run_statement(
            source,
            self.schema.clone(),
            self.storage.clone(),
            self.txn_manager.clone(),
            self.algorithms.clone(),
        )
    }

    /// Self-checking end-to-end demo.
    ///
    /// It builds an isolated database, inserts deterministic data, exercises a
    /// read, a traversal, an aggregation, a path, a mutation, all three
    /// algorithms through the public `CALL` syntax, and a reopen, then asserts
    /// every result and returns an error on the first mismatch.
    pub fn demo_test() -> Result<()> {
        let directory = std::env::temp_dir().join(format!(
            "metagdb-exp-alg-01-demo-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or_default()
        ));
        let _ = std::fs::remove_dir_all(&directory);
        let outcome = run_demo(&directory);
        let _ = std::fs::remove_dir_all(&directory);
        outcome
    }
}

fn ensure(condition: bool, message: impl Into<String>) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(Error::Other(message.into()))
    }
}

fn scalar(stream: &metagdb_core::l4_compute::engine::ResultStream) -> Result<&metagdb_core::common::property::ResultValue> {
    stream
        .rows
        .first()
        .and_then(|row| row.first())
        .ok_or_else(|| Error::Execute("the demo query returned no value".into()))
}

fn run_demo(directory: &std::path::Path) -> Result<()> {
    println!("=== metagdb_exp_alg_01 demo ===");
    let implementation = MetagdbImpl::with_base_dir(directory.to_path_buf());

    // 1. Deterministic data through Cypher.
    for statement in [
        "CREATE (:Person {name: 'A', age: 30})",
        "CREATE (:Person {name: 'B', age: 25})",
        "CREATE (:Person {name: 'C', age: 35})",
        "CREATE (:City {name: 'NYC', population: 8000000})",
        "MATCH (a {name: 'A'}), (b {name: 'B'}) CREATE (a)-[:KNOWS {since: 2015}]->(b)",
        "MATCH (a {name: 'B'}), (b {name: 'C'}) CREATE (a)-[:KNOWS {since: 2018}]->(b)",
        "MATCH (a {name: 'A'}), (b {name: 'NYC'}) CREATE (a)-[:LIVES_IN {since: 2010}]->(b)",
    ] {
        implementation.query(statement)?;
    }
    println!("inserted 4 vertices and 3 relationships");

    // 2. Reads, traversal, aggregation and a variable-length path.
    let people = implementation.query("MATCH (n:Person) RETURN count(n)")?;
    ensure(
        scalar(&people)?.eq(&metagdb_core::common::property::ResultValue::Int(3)),
        format!("expected 3 people, got {:?}", people.rows),
    )?;
    println!("MATCH (n:Person) RETURN count(n) -> 3");

    let reached = implementation.query("MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN b.name")?;
    ensure(reached.rows.len() == 1, "A should know exactly one person")?;
    println!("MATCH (a {{name: 'A'}})-[:KNOWS]->(b) RETURN b.name -> B");

    let total = implementation.query("MATCH (n:Person) RETURN sum(n.age)")?;
    ensure(
        scalar(&total)?.eq(&metagdb_core::common::property::ResultValue::Float(90.0)),
        "sum of ages should be 90",
    )?;
    println!("MATCH (n:Person) RETURN sum(n.age) -> 90");

    let hop = implementation.query("MATCH (a {name: 'A'})-[:KNOWS*2]->(b) RETURN b.name")?;
    ensure(hop.rows.len() == 1, "one two-hop path from A")?;
    println!("MATCH (a {{name: 'A'}})-[:KNOWS*2]->(b) RETURN b.name -> C");

    // 3. A mutation, then a read that depends on it.
    implementation.query("MATCH (n:Person {name: 'A'}) SET n.age = 31")?;
    let updated = implementation.query("MATCH (n {name: 'A'}) RETURN n.age")?;
    ensure(
        scalar(&updated)?.eq(&metagdb_core::common::property::ResultValue::Int(31)),
        "the update should be visible",
    )?;
    println!("MATCH (n:Person {{name: 'A'}}) SET n.age = 31 -> 31");

    // 4. Every required algorithm through the public language.
    let bfs = implementation.query("CALL bfs(1) YIELD node, distance RETURN node, distance")?;
    ensure(
        bfs.rows.len() == 4,
        format!("bfs should reach all four vertices, got {}", bfs.rows.len()),
    )?;
    println!("CALL bfs(1) YIELD node, distance -> {} rows", bfs.rows.len());

    let wcc = implementation.query("CALL wcc() YIELD node, component RETURN node, component")?;
    ensure(wcc.rows.len() == 4, "wcc should label every vertex")?;
    println!("CALL wcc() YIELD node, component -> {} rows", wcc.rows.len());

    let ranks = implementation.query("CALL pagerank() YIELD node, rank RETURN node, rank")?;
    let total_rank: f64 = ranks
        .rows
        .iter()
        .filter_map(|row| match row.get(1) {
            Some(metagdb_core::common::property::ResultValue::Float(value)) => Some(*value),
            _ => None,
        })
        .sum();
    ensure(
        (total_rank - 1.0).abs() < 1e-6,
        format!("ranks should sum to 1, got {total_rank}"),
    )?;
    println!("CALL pagerank() YIELD node, rank -> ranks sum to 1");

    // 5. Durability: everything above is committed, so a reopen must see it.
    drop(implementation);
    let reopened = MetagdbImpl::with_base_dir(directory.to_path_buf());
    let after = reopened.query("MATCH (n:Person) RETURN count(n)")?;
    ensure(
        scalar(&after)?.eq(&metagdb_core::common::property::ResultValue::Int(3)),
        "committed data must survive a reopen",
    )?;
    let persisted = reopened.query("MATCH (n {name: 'A'}) RETURN n.age")?;
    ensure(
        scalar(&persisted)?.eq(&metagdb_core::common::property::ResultValue::Int(31)),
        "the committed update must survive a reopen",
    )?;
    println!("reopened the database and re-read the committed graph");

    println!("=== demo passed ===");
    Ok(())
}
