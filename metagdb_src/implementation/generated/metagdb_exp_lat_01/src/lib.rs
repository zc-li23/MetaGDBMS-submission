//! metaGDB candidate `metagdb_exp_lat_01`.
//!
//! A schemaless, persistent, snapshot-isolated Cypher property-graph system
//! optimized for warm vertex-ID point reads and one-hop traversals while
//! retaining BFS, WCC, and PageRank analytical capability.

pub mod compute;
pub mod schema;
pub mod storage;
pub mod txn;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use metagdb_core::common::property::ResultValue;
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::engine::{ExecContext, Executor, OptContext, Optimizer, Parser, Planner, ResultStream};
use metagdb_core::l4_compute::olap::{AlgorithmRegistry, AnalyticalAlgorithm};
use metagdb_core::{Error, Result};

pub use compute::algorithms::{BfsAlgorithm, PageRankAlgorithm, WccAlgorithm};
pub use compute::executor::MetagdbExecutor;
pub use compute::optimizer::MetagdbOptimizer;
pub use compute::parser::MetagdbParser;
pub use compute::planner::MetagdbPlanner;
pub use compute::projection::CsrProjection;
pub use schema::DynamicSchema;
pub use storage::GraphStore;
pub use txn::MvccTxnManager;

/// Owner of the schema, storage engine, transaction manager, and algorithms.
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
    /// Open the default on-disk database directory.
    pub fn new() -> Self {
        Self::with_base_dir(PathBuf::from("data/gdb"))
    }

    /// Open (or create) a persistent database rooted at `base_dir`.
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = match GraphStore::with_base_dir(schema.clone(), base_dir) {
            Ok(store) => Arc::new(store),
            Err(error) => panic!("cannot open metaGDB database: {error}"),
        };
        Self::assemble(schema, store)
    }

    /// Import a `.graph` file (zero-based external ids) into a persistent store.
    pub fn build_from_graph_file(graph_path: PathBuf, base_dir: PathBuf) -> Result<Self> {
        if !graph_path.exists() {
            return Err(Error::Storage(format!(
                "graph file does not exist: {}",
                graph_path.display()
            )));
        }
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = Arc::new(GraphStore::bulk_load_from_graph(
            schema.clone(),
            base_dir,
            &graph_path,
        )?);
        Ok(Self::assemble(schema, store))
    }

    fn assemble(schema: Arc<DynamicSchema>, store: Arc<GraphStore>) -> Self {
        let txn_manager = Arc::new(MvccTxnManager::with_storage(store.clone()));
        Self {
            schema: schema as Arc<dyn GraphSchema>,
            storage: store,
            txn_manager,
            algorithms: compute::algorithms::required_algorithms(),
        }
    }

    fn algorithm_registry(&self) -> Arc<AlgorithmRegistry> {
        let mut registry = AlgorithmRegistry::default();
        for algorithm in &self.algorithms {
            registry
                .algorithms
                .insert(algorithm.name().to_string(), algorithm.clone());
        }
        Arc::new(registry)
    }

    /// Execute one Cypher statement through the full public pipeline.
    pub fn execute(&self, source: &str) -> Result<ResultStream> {
        let parser = MetagdbParser;
        let ast = parser.parse(source)?;
        let planner = MetagdbPlanner;
        let logical = planner.plan(ast, &*self.schema)?;
        let algorithms = self.algorithm_registry();
        let optimizer = MetagdbOptimizer;
        let physical = optimizer.optimize(
            logical,
            &OptContext {
                schema: self.schema.clone(),
                algorithms: algorithms.clone(),
            },
        )?;
        let policy = self.txn_manager.default_policy();
        let txn = self.txn_manager.begin(policy.isolation, policy.read_only)?;
        let executor = MetagdbExecutor;
        let context = ExecContext {
            txn: txn.clone(),
            storage: self.storage.clone(),
            schema: self.schema.clone(),
            algorithms,
        };
        match executor.execute(&physical, &context) {
            Ok(stream) => {
                self.txn_manager.commit(txn)?;
                Ok(stream)
            }
            Err(error) => {
                let _ = self.txn_manager.abort(txn);
                Err(error)
            }
        }
    }

    /// Self-contained end-to-end demo used by `cargo run -- --demo`.
    pub fn demo_test() -> Result<()> {
        let base_dir = std::env::temp_dir().join(format!(
            "metagdb_exp_lat_01_demo_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&base_dir);
        let outcome = run_demo(&base_dir);
        let _ = std::fs::remove_dir_all(&base_dir);
        outcome
    }
}

fn run_demo(base_dir: &Path) -> Result<()> {
    let db = MetagdbImpl::with_base_dir(base_dir.to_path_buf());

    db.execute("CREATE (:Person {name: 'Alice', age: 30})")?;
    db.execute("CREATE (:Person {name: 'Bob', age: 25})")?;
    db.execute("CREATE (:City {name: 'Berlin'})")?;
    db.execute("MATCH (a {name: 'Alice'}), (b {name: 'Bob'}) CREATE (a)-[:KNOWS {since: 2020}]->(b)")?;
    db.execute("MATCH (a {name: 'Alice'}), (c {name: 'Berlin'}) CREATE (a)-[:LIVES_IN]->(c)")?;

    let people = db.execute("MATCH (n:Person) RETURN n.name ORDER BY n.name")?;
    if people.rows.len() != 2 {
        return Err(Error::Other(format!(
            "demo expected 2 people, found {}",
            people.rows.len()
        )));
    }
    if people.rows[0][0] != ResultValue::Text("Alice".into()) {
        return Err(Error::Other(format!(
            "demo ordering failed: {:?}",
            people.rows[0][0]
        )));
    }

    let neighbors = db.execute("MATCH (a {name: 'Alice'})-[:KNOWS]->(b) RETURN b.name")?;
    if neighbors.rows.len() != 1 || neighbors.rows[0][0] != ResultValue::Text("Bob".into()) {
        return Err(Error::Other("demo one-hop traversal failed".into()));
    }

    let counts = db.execute("MATCH (n) RETURN count(n)")?;
    if counts.rows.len() != 1 || counts.rows[0][0] != ResultValue::Int(3) {
        return Err(Error::Other(format!(
            "demo count failed: {:?}",
            counts.rows
        )));
    }

    let round_trip = db.execute("MATCH (n) WHERE n.age IS NOT NULL RETURN count(n)")?;
    if round_trip.rows[0][0] != ResultValue::Int(2) {
        return Err(Error::Other("demo null filter failed".into()));
    }

    // Reopen the persistent store and verify committed data survives.
    drop(db);
    let reopened = MetagdbImpl::with_base_dir(base_dir.to_path_buf());
    let after = reopened.execute("MATCH (n:Person) RETURN count(n)")?;
    if after.rows[0][0] != ResultValue::Int(2) {
        return Err(Error::Other("demo persistence failed".into()));
    }

    // Run a registered analytical algorithm through the public language path.
    let bfs = reopened.execute("CALL bfs(source=1)")?;
    if bfs.rows.is_empty() {
        return Err(Error::Other("demo BFS returned no rows".into()));
    }

    println!("demo: created 3 vertices and 2 edges; point, one-hop, aggregate, and reopen checks passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::l3_txn::IsolationLevel;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mgdb_lib_{tag}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn public_pipeline_crud_and_adjacency() {
        let dir = temp_dir("crud");
        let db = MetagdbImpl::with_base_dir(dir.clone());
        db.execute("CREATE (:Person {name: 'A', age: 30})").unwrap();
        db.execute("CREATE (:Person {name: 'B', age: 25})").unwrap();
        db.execute("MATCH (a {name: 'A'}), (b {name: 'B'}) CREATE (a)-[:KNOWS {since: 2020}]->(b)")
            .unwrap();

        let result = db
            .execute("MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN b.name")
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], ResultValue::Text("B".into()));

        let updated = db
            .execute("MATCH (a {name: 'A'}) SET a.age = 31 RETURN a.age")
            .unwrap();
        assert_eq!(updated.rows[0][0], ResultValue::Int(31));

        let follow_up = db
            .execute("MATCH (a {name: 'A'}) RETURN a.age")
            .unwrap();
        assert_eq!(follow_up.rows[0][0], ResultValue::Int(31));

        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn public_pipeline_aggregation_and_unwind() {
        let dir = temp_dir("agg");
        let db = MetagdbImpl::with_base_dir(dir.clone());
        db.execute("UNWIND [1, 2, 3] AS x RETURN x").unwrap();
        let sum = db.execute("UNWIND [1, 2, 3] AS x RETURN sum(x)").unwrap();
        assert_eq!(sum.rows[0][0], ResultValue::Int(6));
        let collected = db
            .execute("MATCH (n:Missing) RETURN collect(n)")
            .unwrap();
        assert_eq!(collected.rows[0][0], ResultValue::List(Vec::new()));
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn detach_delete_removes_incident_edges() {
        let dir = temp_dir("detach");
        let db = MetagdbImpl::with_base_dir(dir.clone());
        db.execute("CREATE (:Person {name: 'A'})").unwrap();
        db.execute("CREATE (:Person {name: 'B'})").unwrap();
        db.execute("MATCH (a {name: 'A'}), (b {name: 'B'}) CREATE (a)-[:KNOWS]->(b)")
            .unwrap();
        let deleted = db
            .execute("MATCH (a {name: 'A'}) DETACH DELETE a RETURN count(a)")
            .unwrap();
        assert_eq!(deleted.rows[0][0], ResultValue::Int(1));
        let edges = db.execute("MATCH ()-[r]->() RETURN count(r)").unwrap();
        assert_eq!(edges.rows[0][0], ResultValue::Int(0));
        let remaining = db.execute("MATCH (n) RETURN count(n)").unwrap();
        assert_eq!(remaining.rows[0][0], ResultValue::Int(1));
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn not_null_semantics_exclude_unknown() {
        let dir = temp_dir("notnull");
        let db = MetagdbImpl::with_base_dir(dir.clone());
        db.execute("CREATE (:Person {name: 'A', age: 30})").unwrap();
        db.execute("CREATE (:Person {name: 'B'})").unwrap();
        let result = db
            .execute("MATCH (n:Person) WHERE NOT n.age = 30 RETURN n.name")
            .unwrap();
        assert_eq!(result.rows.len(), 0, "null comparison under NOT must be filtered");
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn graph_db(tag: &str) -> (MetagdbImpl, PathBuf) {
        let dir = temp_dir(tag);
        let db = MetagdbImpl::with_base_dir(dir.clone());
        db.execute("CREATE (a:Node {name: 'a'})").unwrap();
        db.execute("CREATE (b:Node {name: 'b'})").unwrap();
        db.execute("CREATE (c:Node {name: 'c'})").unwrap();
        db.execute("MATCH (a {name: 'a'}), (b {name: 'b'}) CREATE (a)-[:LINK]->(b)")
            .unwrap();
        db.execute("MATCH (b {name: 'b'}), (c {name: 'c'}) CREATE (b)-[:LINK]->(c)")
            .unwrap();
        (db, dir)
    }

    #[test]
    fn bfs_language_pipeline() {
        let (db, dir) = graph_db("bfs_lang");
        let result = db.execute("CALL bfs(source=1)").unwrap();
        assert_eq!(result.schema, vec!["node".to_string(), "distance".to_string()]);
        let mut distances: Vec<i64> = result
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Int(distance) => distance,
                ref other => panic!("unexpected distance {other:?}"),
            })
            .collect();
        distances.sort_unstable();
        assert_eq!(distances, vec![0, 1, 2], "BFS distances through the public pipeline");
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn wcc_language_pipeline() {
        let (db, dir) = graph_db("wcc_lang");
        let result = db.execute("CALL wcc()").unwrap();
        assert_eq!(result.rows.len(), 3);
        let components: Vec<i64> = result
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Int(component) => component,
                ref other => panic!("unexpected component {other:?}"),
            })
            .collect();
        assert!(components.iter().all(|component| *component == components[0]));
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pagerank_language_pipeline() {
        let (db, dir) = graph_db("pr_lang");
        let result = db.execute("CALL pagerank()").unwrap();
        assert_eq!(result.rows.len(), 3);
        let total: f64 = result
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Float(rank) => rank,
                ref other => panic!("unexpected rank {other:?}"),
            })
            .sum();
        assert!((total - 1.0).abs() < 1e-6, "PageRank mass {total}");
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn demo_test_passes() {
        MetagdbImpl::demo_test().unwrap();
    }

    #[test]
    fn required_exports_construct() {
        let _parser = MetagdbParser;
        let _planner = MetagdbPlanner;
        let _optimizer = MetagdbOptimizer;
        let _executor = MetagdbExecutor;
        let algorithms = compute::algorithms::required_algorithms();
        assert_eq!(algorithms.len(), 3);
        assert_eq!(IsolationLevel::SnapshotIsolation, txn::DEFAULT_ISOLATION);
    }
}
