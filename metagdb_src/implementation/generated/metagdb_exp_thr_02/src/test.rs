//! Candidate-local deterministic tests and the self-checking demo.
//!
//! The helper [`run_query`] drives the public Parser -> Planner -> Optimizer ->
//! Executor path exactly as the framework runtime does, so every semantic claim
//! below (and in the `tests` submodule) is exercised through the real pipeline
//! rather than an internal shortcut.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use metagdb_core::common::property::ResultValue;

use metagdb_core::l4_compute::engine::{
    ExecContext, Executor, OptContext, Optimizer, Parser, Planner, ResultStream,
};
use metagdb_core::l4_compute::olap::AlgorithmRegistry;
use metagdb_core::Result;

use crate::compute::executor::MetagdbExecutor;
use crate::compute::optimizer::MetagdbOptimizer;
use crate::compute::parser::MetagdbParser;
use crate::compute::planner::MetagdbPlanner;
use crate::MetagdbImpl;

static NEXT_DIR: AtomicU64 = AtomicU64::new(1);

fn temp_dir(label: &str) -> PathBuf {
    let id = NEXT_DIR.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "metagdb_thr02_test_{label}_{}_{id}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn registry(implementation: &MetagdbImpl) -> Arc<AlgorithmRegistry> {
    let mut algorithms = HashMap::new();
    for algorithm in &implementation.algorithms {
        algorithms.insert(algorithm.name().to_string(), algorithm.clone());
    }
    Arc::new(AlgorithmRegistry { algorithms })
}

/// Run one statement through the public pipeline.
pub fn run_query(implementation: &MetagdbImpl, source: &str) -> Result<ResultStream> {
    let parser = MetagdbParser;
    let ast = parser.parse(source)?;
    let planner = MetagdbPlanner;
    let logical = planner.plan(ast, &*implementation.schema)?;
    let algorithms = registry(implementation);
    let optimizer = MetagdbOptimizer;
    let physical = optimizer.optimize(
        logical,
        &OptContext {
            schema: implementation.schema.clone(),
            algorithms: algorithms.clone(),
        },
    )?;
    let policy = implementation.txn_manager.default_policy();
    let txn = implementation
        .txn_manager
        .begin(policy.isolation, policy.read_only)?;
    let context = ExecContext {
        txn: txn.clone(),
        storage: implementation.storage.clone(),
        schema: implementation.schema.clone(),
        algorithms,
    };
    let executor = MetagdbExecutor;
    match executor.execute(&physical, &context) {
        Ok(stream) => {
            implementation.txn_manager.commit(txn)?;
            Ok(stream)
        }
        Err(error) => {
            let _ = implementation.txn_manager.abort(txn);
            Err(error)
        }
    }
}

/// Run a list of statements, ignoring their results.
pub fn run_all(implementation: &MetagdbImpl, statements: &[&str]) -> Result<()> {
    for statement in statements {
        run_query(implementation, statement)?;
    }
    Ok(())
}

fn install_social9(implementation: &MetagdbImpl) -> Result<()> {
    run_all(
        implementation,
        &[
            "CREATE (:Person {name: 'A', age: 30, joined: 2015})",
            "CREATE (:Person {name: 'B', age: 25, joined: 2018})",
            "CREATE (:Person {name: 'C', age: 35, joined: 2010})",
            "CREATE (:City {name: 'NYC', population: 8000000})",
            "CREATE (:City {name: 'SF', population: 900000})",
            "CREATE (:Person {name: 'D', age: 30, joined: 2015})",
            "CREATE (:Person {name: 'E', age: 28, joined: 2017})",
            "CREATE (:City {name: 'LA', population: 4000000})",
            "CREATE (:Company {name: 'Acme', founded: 1999})",
            "MATCH (a {name: 'A'}), (b {name: 'B'}) CREATE (a)-[:KNOWS {since: 2015, weight: 0.5}]->(b)",
            "MATCH (a {name: 'B'}), (b {name: 'C'}) CREATE (a)-[:KNOWS {since: 2018, weight: 1.5}]->(b)",
            "MATCH (a {name: 'A'}), (b {name: 'C'}) CREATE (a)-[:KNOWS {since: 2016, weight: 0.5}]->(b)",
            "MATCH (a {name: 'C'}), (b {name: 'D'}) CREATE (a)-[:KNOWS {since: 2019, weight: 1.5}]->(b)",
            "MATCH (a {name: 'D'}), (b {name: 'E'}) CREATE (a)-[:KNOWS {since: 2020, weight: 0.5}]->(b)",
            "MATCH (a {name: 'E'}), (b {name: 'A'}) CREATE (a)-[:KNOWS {since: 2021, weight: 1.5}]->(b)",
            "MATCH (a {name: 'A'}), (b {name: 'NYC'}) CREATE (a)-[:LIVES_IN {since: 2010}]->(b)",
            "MATCH (a {name: 'B'}), (b {name: 'SF'}) CREATE (a)-[:LIVES_IN {since: 2018}]->(b)",
            "MATCH (a {name: 'C'}), (b {name: 'NYC'}) CREATE (a)-[:LIVES_IN {since: 2011}]->(b)",
            "MATCH (a {name: 'D'}), (b {name: 'NYC'}) CREATE (a)-[:LIVES_IN {since: 2015}]->(b)",
            "MATCH (a {name: 'E'}), (b {name: 'SF'}) CREATE (a)-[:LIVES_IN {since: 2017}]->(b)",
            "MATCH (a {name: 'A'}), (b {name: 'Acme'}) CREATE (a)-[:WORKS_AT {since: 2016}]->(b)",
            "MATCH (a {name: 'C'}), (b {name: 'Acme'}) CREATE (a)-[:WORKS_AT {since: 2012}]->(b)",
            "MATCH (a {name: 'E'}), (b {name: 'Acme'}) CREATE (a)-[:WORKS_AT {since: 2018}]->(b)",
        ],
    )
}

fn expected(rows: Vec<Vec<ResultValue>>) -> Vec<Vec<ResultValue>> {
    rows
}

/// Order-insensitive row comparison, mirroring the benchmark's unordered mode.
#[track_caller]
fn assert_rows_unordered(stream: &ResultStream, rows: Vec<Vec<ResultValue>>) {
    let mut actual: Vec<String> = stream.rows.iter().map(|row| render_row(row)).collect();
    let mut want: Vec<String> = rows.iter().map(|row| render_row(row)).collect();
    actual.sort();
    want.sort();
    assert_eq!(actual, want, "query result mismatch");
}

fn render_row(row: &[ResultValue]) -> String {
    row.iter().map(render_value).collect::<Vec<_>>().join(",")
}

fn render_value(value: &ResultValue) -> String {
    match value {
        ResultValue::Null => "null".into(),
        ResultValue::Bool(v) => v.to_string(),
        ResultValue::Int(v) => v.to_string(),
        ResultValue::Float(v) => v.to_string(),
        ResultValue::Text(v) => v.clone(),
        ResultValue::VertexRef(v) => format!("v[{}]", v.0),
        ResultValue::EdgeRef(v) => format!("e[{}]", v.0),
        ResultValue::Property(p) => format!("{p:?}"),
        other => format!("{other:?}"),
    }
}

fn text(value: &str) -> ResultValue {
    ResultValue::Text(value.to_string())
}

fn int(value: i64) -> ResultValue {
    ResultValue::Int(value)
}

// ---------------------------------------------------------------------------
// Self-checking demo
// ---------------------------------------------------------------------------

/// Zero-argument demo: build an isolated runtime, insert deterministic data,
/// run representative requirements, assert, and report.
pub fn demo_test() -> Result<()> {
    let base_dir = temp_dir("demo");
    let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
    install_social9(&implementation)?;

    println!("== metagdb_exp_thr_02 demo ==");

    let count = run_query(&implementation, "MATCH (n) RETURN count(n)")?;
    assert_rows_unordered(&count, expected(vec![vec![int(9)]]));
    println!("  vertices: {}", count.rows[0][0].to_string_value());

    let people = run_query(
        &implementation,
        "MATCH (n:Person) WHERE n.age > 28 RETURN n.name ORDER BY n.name",
    )?;
    assert_eq!(
        people.rows,
        vec![vec![text("A")], vec![text("C")], vec![text("D")]]
    );

    let neighbours = run_query(
        &implementation,
        "MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN b.name",
    )?;
    assert_rows_unordered(&neighbours, expected(vec![vec![text("B")], vec![text("C")]]));

    let aggregated = run_query(&implementation, "MATCH (n:Person) RETURN sum(n.age)")?;
    assert_eq!(aggregated.rows, vec![vec![int(148)]]);

    let paths = run_query(
        &implementation,
        "MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN length(p)",
    )?;
    assert_eq!(paths.rows, vec![vec![int(2)], vec![int(2)]]);

    // A durable update: the append-only segment gains a new physical record and
    // the redirection index moves, while the old snapshot keeps its value.
    run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) SET n.age = 31 RETURN n.age",
    )?;
    let updated = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) RETURN n.age",
    )?;
    assert_eq!(updated.rows, vec![vec![int(31)]]);

    // The registered algorithm is reachable from the public language path; the
    // procedure streams one row per reached vertex.
    let reached = run_query(
        &implementation,
        "CALL bfs(1) YIELD node RETURN count(*)",
    )?;
    assert_eq!(reached.rows, vec![vec![int(8)]]);

    // Reopen the same directory and confirm the committed state survived.
    drop(implementation);
    let reopened = MetagdbImpl::with_base_dir(base_dir.clone());
    let persisted = run_query(
        &reopened,
        "MATCH (n:Person {name: 'A'}) RETURN n.age",
    )?;
    assert_eq!(persisted.rows, vec![vec![int(31)]]);
    println!("  reopen: committed update survived");

    let _ = std::fs::remove_dir_all(&base_dir);
    println!("== demo ok ==");
    Ok(())
}

trait DisplayValue {
    fn to_string_value(&self) -> String;
}

impl DisplayValue for ResultValue {
    fn to_string_value(&self) -> String {
        render_value(self)
    }
}
// ---------------------------------------------------------------------------
// Candidate-local deterministic tests
//
// Every semantic claim below is exercised through the public
// Parser -> Planner -> Optimizer -> Executor path via `run_query`.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
use super::*;

use crate::compute::algorithms::{BfsAlgorithm, PageRankAlgorithm, WccAlgorithm};
use crate::compute::projection::CsrProjection;
use metagdb_core::common::ids::{EdgeId, VertexId};
use metagdb_core::l2_storage::{EdgeFilter, VertexFilter};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext, TxnManager};
use metagdb_core::l4_compute::algebra::{AlgorithmArgs, ProjectionSpec};
use metagdb_core::l4_compute::olap::AnalyticalAlgorithm;
use metagdb_core::Error;

/// Build an isolated runtime and return the concrete storage handle so a test
/// can inspect the redirection index directly.
fn build_runtime(label: &str) -> (MetagdbImpl, Arc<crate::storage::PersistentStorage>) {
    build_runtime_at(&temp_dir(label))
}

/// Build a runtime rooted at an explicit directory.
fn build_runtime_at(
    base_dir: &std::path::Path,
) -> (MetagdbImpl, Arc<crate::storage::PersistentStorage>) {
    let base_dir = base_dir.to_path_buf();
    let schema = Arc::new(crate::schema::MetaSchema::demo_schema());
    let storage = Arc::new(
        crate::storage::PersistentStorage::open(schema.clone(), base_dir)
            .expect("storage opens"),
    );
    let txn_manager: Arc<dyn TxnManager> = Arc::new(crate::txn::MvccTxnManager::new(storage.clone()));
    let implementation = MetagdbImpl {
        schema,
        storage: storage.clone(),
        txn_manager,
        algorithms: crate::compute::algorithms::required_algorithms(),
    };
    (implementation, storage)
}

// ---------------------------------------------------------------------------
// Language front end
// ---------------------------------------------------------------------------

#[test]
fn parser_accepts_and_rejects_cypher() {
    let parser = MetagdbParser;
    assert!(parser.parse("MATCH (n) RETURN n").is_ok());
    assert!(parser.parse("MATCH (n) RETURN n;").is_ok());
    assert!(parser.parse("MATCH (n RETURN n").is_err());
    assert!(parser.parse("MATCH (n) RETURN").is_err());
    assert!(parser.parse("UNWIND AS x RETURN x").is_err());
    // A Gremlin traversal is not valid Cypher and must be rejected at parse time.
    assert!(parser.parse("g.V().count()").is_err());
}

#[test]
fn lowering_keeps_optional_parts_distinct() {
    use crate::compute::cypher_ast::{CypherAst, Expr};
    let parser = MetagdbParser;
    let ast = parser.parse("MATCH (n) WHERE n.age > 2 RETURN n.name AS name").unwrap();
    let CypherAst::Query(query) = ast.as_any().downcast_ref::<CypherAst>().unwrap() else {
        panic!("expected a query");
    };
    let clauses = &query.branches[0].0;
    assert_eq!(clauses.len(), 2);
    match &clauses[0] {
        crate::compute::cypher_ast::Clause::Match { predicate, .. } => {
            assert!(matches!(predicate, Some(Expr::Binary(..))));
        }
        other => panic!("expected MATCH, found {other:?}"),
    }
}

#[test]
fn grammar_quirks_are_handled() {
    let parser = MetagdbParser;
    // The vendored grammar requires no space after the comma in SET items.
    assert!(parser
        .parse("MATCH (n) SET n.a = 1,n.b = 2 RETURN n.a")
        .is_ok());
    // Negative list subscripts and open slices are part of the profile.
    assert!(parser.parse("RETURN [1, 2, 3][-1]").is_ok());
    assert!(parser.parse("RETURN [1, 2, 3, 4][..2]").is_ok());
}

#[test]
fn planner_binds_metadata_through_the_schema() {
    let implementation = MetagdbImpl::new();
    let parser = MetagdbParser;
    let planner = MetagdbPlanner;
    let ast = parser
        .parse("MATCH (n:Person) WHERE n.name = 'A' RETURN n.age")
        .unwrap();
    let plan = planner.plan(ast, &*implementation.schema).unwrap();
    // Binding is observable: planning interns the metadata names in the schema.
    assert!(implementation.schema.label_by_name("Person").is_some());
    assert!(implementation.schema.property_key_by_name("name").is_some());
    assert!(implementation.schema.label_name(
        implementation.schema.label_by_name("Person").unwrap()
    ) == Some("Person"));
    let rendered = format!("{:?}", plan.root);
    assert!(rendered.contains("ExtensionOp"), "unexpected plan: {rendered}");
}

#[test]
fn optimizer_lowers_every_planner_operator() {
    use crate::compute::physical_ops::PhysicalOp;
    let implementation = MetagdbImpl::new();
    let parser = MetagdbParser;
    let planner = MetagdbPlanner;
    let optimizer = MetagdbOptimizer;
    let algorithms = registry(&implementation);
    let ast = parser
        .parse("MATCH (n:Person) WITH n WHERE n.age > 20 SET n.seen = true RETURN n.age ORDER BY n.age LIMIT 2")
        .unwrap();
    let plan = planner.plan(ast, &*implementation.schema).unwrap();
    let physical = optimizer
        .optimize(
            plan,
            &OptContext {
                schema: implementation.schema.clone(),
                algorithms,
            },
        )
        .unwrap();
    let root = physical.downcast_ref::<PhysicalOp>().expect("physical plan");
    let mut names = Vec::new();
    collect_names(root, &mut names);
    for expected in ["CypherReturn", "CypherSet", "CypherWith", "CypherMatch"] {
        assert!(names.contains(&expected), "missing operator {expected} in {names:?}");
    }
}

fn collect_names<'a>(op: &'a crate::compute::physical_ops::PhysicalOp, out: &mut Vec<&'static str>) {
    out.push(op.operator_name());
    for child in op.children() {
        collect_names(child, out);
    }
}

#[test]
fn optimizer_pushes_point_lookup_hint() {
    let implementation = MetagdbImpl::new();
    let parser = MetagdbParser;
    let planner = MetagdbPlanner;
    let optimizer = MetagdbOptimizer;
    let ast = parser.parse("MATCH (n) WHERE id(n) = 5 RETURN n").unwrap();
    let plan = planner.plan(ast, &*implementation.schema).unwrap();
    let physical = optimizer
        .optimize(
            plan,
            &OptContext {
                schema: implementation.schema.clone(),
                algorithms: registry(&implementation),
            },
        )
        .unwrap();
    let root = physical
        .downcast_ref::<crate::compute::physical_ops::PhysicalOp>()
        .expect("physical plan");
    let mut found = false;
    if let crate::compute::physical_ops::PhysicalOp::Project { input, .. } = root {
        if let crate::compute::physical_ops::PhysicalOp::Match { pattern, .. } = &**input {
            found = pattern.parts[0].element.node.id_hint == Some(5);
        }
    }
    assert!(found, "id() predicate was not pushed into the node pattern");
}

// ---------------------------------------------------------------------------
// Semantics
// ---------------------------------------------------------------------------

#[test]
fn reads_match_the_shared_fixture() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();

    let count = run_query(&implementation, "MATCH ()-[r:KNOWS]->() RETURN count(r)").unwrap();
    assert_eq!(count.rows, vec![vec![int(6)]]);

    let labels = run_query(&implementation, "MATCH (n) RETURN DISTINCT labels(n)").unwrap();
    assert_rows_unordered(
        &labels,
        expected(vec![
            vec![ResultValue::List(vec![text("Person")])],
            vec![ResultValue::List(vec![text("City")])],
            vec![ResultValue::List(vec![text("Company")])],
        ]),
    );

    let incoming = run_query(
        &implementation,
        "MATCH (a {name: 'A'})<-[:KNOWS]-(b) RETURN b.name",
    )
    .unwrap();
    assert_eq!(incoming.rows, vec![vec![text("E")]]);

    let identifiers = run_query(&implementation, "MATCH (n {name: 'A'}) RETURN id(n)").unwrap();
    assert_eq!(identifiers.rows, vec![vec![int(1)]]);
}

#[test]
fn null_and_boolean_semantics_follow_cypher() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let nulls = run_query(
        &implementation,
        "MATCH (n) WHERE NOT n.age = 30 RETURN n.name ORDER BY n.name",
    )
    .unwrap();
    assert_eq!(
        nulls.rows,
        vec![vec![text("B")], vec![text("C")], vec![text("E")]]
    );
    let exists = run_query(
        &implementation,
        "MATCH (n) WHERE exists(n.age) RETURN count(n)",
    )
    .unwrap();
    assert_eq!(exists.rows, vec![vec![int(5)]]);
}

#[test]
fn aggregation_and_ordering_are_correct() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let grouped = run_query(
        &implementation,
        "MATCH (a:Person)-[:KNOWS]->(b) RETURN a.name, count(b) AS degree ORDER BY degree DESC, a.name",
    )
    .unwrap();
    assert_eq!(
        grouped.rows,
        vec![
            vec![text("A"), int(2)],
            vec![text("B"), int(1)],
            vec![text("C"), int(1)],
            vec![text("D"), int(1)],
            vec![text("E"), int(1)],
        ]
    );
    let distinct = run_query(
        &implementation,
        "MATCH (n:Person) RETURN collect(DISTINCT n.age)",
    )
    .unwrap();
    assert_eq!(
        distinct.rows,
        vec![vec![ResultValue::List(vec![
            int(30),
            int(25),
            int(35),
            int(28)
        ])]]
    );
}

#[test]
fn advanced_expressions_are_evaluated() {
    let implementation = MetagdbImpl::new();
    let quantifier = run_query(&implementation, "RETURN all(x IN [1, 2, 3] WHERE x > 0)").unwrap();
    assert_eq!(quantifier.rows, vec![vec![ResultValue::Bool(true)]]);
    let empty_all = run_query(&implementation, "RETURN all(x IN [] WHERE x > 0)").unwrap();
    assert_eq!(empty_all.rows, vec![vec![ResultValue::Bool(true)]]);
    let any_empty = run_query(&implementation, "RETURN any(x IN [] WHERE x > 0)").unwrap();
    assert_eq!(any_empty.rows, vec![vec![ResultValue::Bool(false)]]);
    let temporal = run_query(
        &implementation,
        "RETURN date('2020-01-01') < date('2020-01-02')",
    )
    .unwrap();
    assert_eq!(temporal.rows, vec![vec![ResultValue::Bool(true)]]);
    let accessor = run_query(&implementation, "RETURN date('2020-05-06').year = 2020").unwrap();
    assert_eq!(accessor.rows, vec![vec![ResultValue::Bool(true)]]);
    let duration = run_query(
        &implementation,
        "RETURN duration('P1D') > duration('PT12H')",
    )
    .unwrap();
    assert_eq!(duration.rows, vec![vec![ResultValue::Bool(true)]]);
    let comprehension = run_query(
        &implementation,
        "RETURN [x IN [1, 2, 3, 4] WHERE x % 2 = 0 | x]",
    )
    .unwrap();
    assert_eq!(
        comprehension.rows,
        vec![vec![ResultValue::List(vec![int(2), int(4)])]]
    );
    let slice = run_query(&implementation, "RETURN [1, 2, 3, 4][1..3]").unwrap();
    assert_eq!(
        slice.rows,
        vec![vec![ResultValue::List(vec![int(2), int(3)])]]
    );
}

#[test]
fn mutations_are_visible_within_and_after_the_query() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let updated = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) SET n.age = 31 RETURN n.age",
    )
    .unwrap();
    assert_eq!(updated.rows, vec![vec![int(31)]]);
    let removed = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) REMOVE n.age RETURN n.age",
    )
    .unwrap();
    assert_eq!(removed.rows, vec![vec![ResultValue::Null]]);
    let relabelled = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) SET n:Admin RETURN size(labels(n))",
    )
    .unwrap();
    assert_eq!(relabelled.rows, vec![vec![int(2)]]);
    // The same transaction cannot see the label again after REMOVE.
    let dropped = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) REMOVE n:Person RETURN size(labels(n))",
    )
    .unwrap();
    assert_eq!(dropped.rows, vec![vec![int(1)]]);
}

#[test]
fn merge_upserts_and_delete_removes() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let existing = run_query(
        &implementation,
        "MERGE (n:Person {name: 'A'}) RETURN n.name",
    )
    .unwrap();
    assert_eq!(existing.rows, vec![vec![text("A")]]);
    let created = run_query(
        &implementation,
        "MERGE (n:Person {name: 'Z'}) ON CREATE SET n.age = 40 RETURN n.age",
    )
    .unwrap();
    assert_eq!(created.rows, vec![vec![int(40)]]);
    let deleted = run_query(
        &implementation,
        "MATCH (n:Company) DETACH DELETE n RETURN count(n)",
    )
    .unwrap();
    assert_eq!(deleted.rows, vec![vec![int(1)]]);
    let remaining = run_query(&implementation, "MATCH (n:Company) RETURN count(n)").unwrap();
    assert_eq!(remaining.rows, vec![vec![int(0)]]);
}

#[test]
fn unions_and_with_pipelines_compose() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let distinct = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) RETURN n.name UNION MATCH (n:Person {name: 'A'}) RETURN n.name",
    )
    .unwrap();
    assert_eq!(distinct.rows, vec![vec![text("A")]]);
    let all = run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) RETURN n.name UNION ALL MATCH (n:Person {name: 'A'}) RETURN n.name",
    )
    .unwrap();
    assert_eq!(all.rows.len(), 2);
    let pipelined = run_query(
        &implementation,
        "MATCH (n:Person) WITH n.age AS age WHERE age >= 30 RETURN count(age)",
    )
    .unwrap();
    assert_eq!(pipelined.rows, vec![vec![int(3)]]);
    let unwound = run_query(
        &implementation,
        "UNWIND [1, 2, 3] AS x WITH x WHERE x > 1 RETURN x",
    )
    .unwrap();
    assert_eq!(unwound.rows, vec![vec![int(2)], vec![int(3)]]);
}

#[test]
fn unsupported_surface_returns_a_precise_error() {
    let implementation = MetagdbImpl::new();
    let error = run_query(&implementation, "RETURN $param").unwrap_err();
    let message = error.to_string();
    assert!(
        message.contains("parameter") || message.contains("unsupported"),
        "unexpected error: {message}"
    );
}

// ---------------------------------------------------------------------------
// Storage and transactions
// ---------------------------------------------------------------------------

#[test]
fn storage_crud_and_adjacency_are_consistent() {
    use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};

    let implementation = MetagdbImpl::new();
    let schema = implementation.schema.clone();
    let label = schema.label_by_name("Person").unwrap();
    let etype = schema.edge_type_by_name("KNOWS").unwrap();
    let txn = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let a = implementation
        .storage
        .insert_vertex(
            VertexRecord {
                id: VertexId(0),
                labels: vec![label],
                properties: Default::default(),
            },
            &txn,
        )
        .unwrap();
    let b = implementation
        .storage
        .insert_vertex(
            VertexRecord {
                id: VertexId(0),
                labels: vec![label],
                properties: Default::default(),
            },
            &txn,
        )
        .unwrap();
    assert_ne!(a, b);
    let edge = implementation
        .storage
        .insert_edge(
            EdgeRecord {
                id: EdgeId(0),
                src: a,
                dst: b,
                etype,
                properties: Default::default(),
            },
            &txn,
        )
        .unwrap();
    implementation.txn_manager.commit(txn).unwrap();

    let read = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, true)
        .unwrap();
    assert!(implementation.storage.get_edge(edge, &read).unwrap().is_some());
    assert!(implementation
        .storage
        .out_edges(a, EdgeFilter::default(), &read)
        .unwrap()
        .any(|candidate| candidate.id == edge));
    assert!(implementation
        .storage
        .in_edges(b, EdgeFilter::default(), &read)
        .unwrap()
        .any(|candidate| candidate.id == edge));
    implementation.txn_manager.commit(read).unwrap();

    // An edge to a missing endpoint is rejected.
    let bad = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let result = implementation.storage.insert_edge(
        EdgeRecord {
            id: EdgeId(0),
            src: a,
            dst: VertexId(999_999),
            etype,
            properties: Default::default(),
        },
        &bad,
    );
    assert!(result.is_err());
    let _ = implementation.txn_manager.abort(bad);
}

#[test]
fn snapshot_isolation_hides_uncommitted_writes() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let writer = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let reader = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, true)
        .unwrap();
    let vertex = implementation
        .storage
        .scan_vertices(VertexFilter::default(), &reader)
        .unwrap()
        .find(|candidate| candidate.id == VertexId(1))
        .unwrap();
    let mut updated = vertex.clone();
    updated.properties.clear();
    implementation
        .storage
        .update_vertex(updated, &writer)
        .unwrap();
    // The writer sees its own update; the older snapshot does not.
    let writer_view = implementation
        .storage
        .get_vertex(VertexId(1), &writer)
        .unwrap()
        .unwrap();
    assert!(writer_view.properties.is_empty());
    let reader_view = implementation
        .storage
        .get_vertex(VertexId(1), &reader)
        .unwrap()
        .unwrap();
    assert!(!reader_view.properties.is_empty());
    implementation.txn_manager.commit(reader).unwrap();
    implementation.txn_manager.commit(writer).unwrap();
}

#[test]
fn concurrent_writers_conflict_on_commit() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let first = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let second = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let original = implementation
        .storage
        .get_vertex(VertexId(1), &first)
        .unwrap()
        .unwrap();
    let mut one = original.clone();
    one.properties.clear();
    implementation.storage.update_vertex(one, &first).unwrap();
    let mut two = original.clone();
    two.properties.clear();
    implementation.storage.update_vertex(two, &second).unwrap();
    let commit_one = implementation.txn_manager.commit(first);
    let commit_two = implementation.txn_manager.commit(second);
    assert!(
        commit_one.is_err() || commit_two.is_err(),
        "both conflicting writers committed"
    );
}

#[test]
fn aborts_and_double_commits_are_rejected() {
    let implementation = MetagdbImpl::new();
    let txn = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let clone = txn.clone();
    implementation.txn_manager.commit(txn).unwrap();
    assert!(implementation.txn_manager.commit(clone).is_err());

    let read_only = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, true)
        .unwrap();
    let result = implementation.storage.insert_vertex(
        metagdb_core::l2_storage::VertexRecord {
            id: VertexId(0),
            labels: Vec::new(),
            properties: Default::default(),
        },
        &read_only,
    );
    assert!(result.is_err(), "read-only transaction accepted a mutation");
    implementation.txn_manager.abort(read_only).unwrap();
}

#[test]
fn append_only_records_and_id_redirection_survive_reopen() {
    let (implementation, storage) = build_runtime("append_only");
    run_all(
        &implementation,
        &[
            "CREATE (:Person {name: 'A', age: 30})",
            "CREATE (:Person {name: 'B', age: 25})",
        ],
    )
    .unwrap();
    let offset_before = storage.redirection_offset(VertexId(1)).unwrap();
    let records_before = storage.physical_record_count();

    run_query(
        &implementation,
        "MATCH (n:Person {name: 'A'}) SET n.age = 31 RETURN n.age",
    )
    .unwrap();

    let offset_after = storage.redirection_offset(VertexId(1)).unwrap();
    assert!(
        offset_after > offset_before,
        "an update must append a new physical record and redirect the id"
    );
    assert!(
        storage.physical_record_count() > records_before,
        "the update must only append, never rewrite, physical records"
    );

    drop(implementation);
    let reopened = MetagdbImpl::with_base_dir(storage.base_dir().to_path_buf());
    let snapshot = read_txn(&reopened);
    let value = reopened
        .storage
        .get_vertex(VertexId(1), &snapshot)
        .unwrap()
        .unwrap();
    let age = value
        .properties
        .get(&reopened.schema.property_key_by_name("age").unwrap())
        .cloned()
        .unwrap();
    assert_eq!(age, metagdb_core::common::property::PropertyValue::Int(31));
    reopened.txn_manager.commit(snapshot).unwrap();
    let _ = std::fs::remove_dir_all(storage.base_dir());
}

fn read_txn(implementation: &MetagdbImpl) -> TxnContext {
    implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, true)
        .unwrap()
}

#[test]
fn crash_recovery_discards_uncommitted_and_keeps_committed() {
    let base_dir = temp_dir("recovery");
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(424_242),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        // Abandon the transaction without committing, then reopen.
    }
    let reopened = MetagdbImpl::with_base_dir(base_dir.clone());
    let read = read_txn(&reopened);
    assert!(reopened
        .storage
        .get_vertex(VertexId(424_242), &read)
        .unwrap()
        .is_none());
    reopened.txn_manager.commit(read).unwrap();

    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(424_243),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }
    let reopened = MetagdbImpl::with_base_dir(base_dir.clone());
    let read = read_txn(&reopened);
    assert!(reopened
        .storage
        .get_vertex(VertexId(424_243), &read)
        .unwrap()
        .is_some());
    reopened.txn_manager.commit(read).unwrap();
    let _ = std::fs::remove_dir_all(&base_dir);
}

#[test]
fn checkpoint_then_reopen_keeps_the_redirection_index() {
    let (implementation, storage) = build_runtime("checkpoint");
    run_all(&implementation, &["CREATE (:Person {name: 'A'})"]).unwrap();
    let offset = storage.redirection_offset(VertexId(1)).unwrap();
    storage.checkpoint().unwrap();
    let base_dir = storage.base_dir().to_path_buf();
    drop(implementation);

    let schema = Arc::new(crate::schema::MetaSchema::demo_schema());
    let reopened_storage =
        crate::storage::PersistentStorage::open(schema, base_dir.clone()).unwrap();
    assert_eq!(
        reopened_storage.redirection_offset(VertexId(1)),
        Some(offset)
    );
    let _ = std::fs::remove_dir_all(&base_dir);
}

/// Performance probe for the sustained edge-insert window. Not part of the
/// correctness suite; run with
/// `cargo test -p <crate> --release bench_edge_insert_window -- --ignored --nocapture`.
#[test]
#[ignore = "performance probe"]
fn bench_edge_insert_window() {
    use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
    use std::time::Instant;

    let probe_dir = temp_dir("insert_bench");
    let (implementation, storage) = build_runtime_at(&probe_dir);
    let schema = implementation.schema.clone();
    let label = schema.label_by_name("Person").unwrap();
    let etype = schema.edge_type_by_name("KNOWS").unwrap();

    // Seed a graph shaped like the generated uniform datasets: 10k vertices and
    // ~16 outgoing edges each, so the adjacency maps are the real size.
    let vertices_count = 10_000usize;
    let degree = 16usize;
    let txn = implementation
        .txn_manager
        .begin(IsolationLevel::SnapshotIsolation, false)
        .unwrap();
    let mut ids = Vec::with_capacity(vertices_count);
    for _ in 0..vertices_count {
        ids.push(
            implementation
                .storage
                .insert_vertex(
                    VertexRecord {
                        id: VertexId(0),
                        labels: vec![label],
                        properties: Default::default(),
                    },
                    &txn,
                )
                .unwrap(),
        );
    }
    for index in 0..vertices_count * degree {
        implementation
            .storage
            .insert_edge(
                EdgeRecord {
                    id: EdgeId(0),
                    src: ids[index % vertices_count],
                    dst: ids[(index * 37 + 13) % vertices_count],
                    etype,
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
    }
    implementation.txn_manager.commit(txn).unwrap();

    let edges_per_commit = 100usize;
    let commits_per_window = 10usize;
    let edges_per_window = edges_per_commit * commits_per_window;
    let mut next_edge_id = 10_000_000u64;
    let mut all_us = Vec::new();

    for window in 0..6 {
        let mut insert_ns = 0u128;
        let mut commit_ns = 0u128;
        let started = Instant::now();
        for batch in 0..commits_per_window {
            let txn = implementation
                .txn_manager
                .begin(IsolationLevel::SnapshotIsolation, false)
                .unwrap();
            let before = Instant::now();
            for offset in 0..edges_per_commit {
                let index = window * edges_per_window + batch * edges_per_commit + offset;
                implementation
                    .storage
                    .insert_edge(
                        EdgeRecord {
                            id: EdgeId(next_edge_id),
                            src: ids[(index * 37 + 13) % vertices_count],
                            dst: ids[(index * 101 + 53) % vertices_count],
                            etype,
                            properties: Default::default(),
                        },
                        &txn,
                    )
                    .unwrap();
                next_edge_id += 1;
            }
            insert_ns += before.elapsed().as_nanos();
            let before = Instant::now();
            implementation.txn_manager.commit(txn).unwrap();
            commit_ns += before.elapsed().as_nanos();
        }
        let elapsed = started.elapsed();
        if window > 0 {
            all_us.push(elapsed.as_secs_f64() * 1e6 / edges_per_window as f64);
        }
        println!(
            "window {window}: {:.3} us/edge ({:.0} edges/s) insert {:.1}% commit {:.1}%",
            elapsed.as_secs_f64() * 1e6 / edges_per_window as f64,
            edges_per_window as f64 / elapsed.as_secs_f64(),
            100.0 * insert_ns as f64 / (insert_ns + commit_ns) as f64,
            100.0 * commit_ns as f64 / (insert_ns + commit_ns) as f64,
        );
    }
    let mean = all_us.iter().sum::<f64>() / all_us.len() as f64;
    println!(
        "MEAN {mean:.3} us/edge -> {:.0} edges/s",
        1e6 / mean
    );
    let _ = storage;
    let _ = std::fs::remove_dir_all(probe_dir);
}

/// Fault injection: a transaction whose records reached the segment but whose
/// WAL commit marker was not persisted must be discarded, and the physical
/// record numbering must not drift from the actual segment positions.
#[test]
fn uncommitted_segment_tail_does_not_desynchronise_record_positions() {
    let base_dir = temp_dir("torn_commit");
    let wal = base_dir.join("wal.log");

    // Transaction A commits normally.
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(100),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }
    let wal_after_a = std::fs::read(&wal).unwrap();

    // Transaction B appends its record to the segment, then loses its WAL
    // commit marker, as if the process died before the marker was fsynced.
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(200),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }
    std::fs::write(&wal, &wal_after_a).unwrap();

    // Reopen, commit C, reopen again: recovery must stay consistent.
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let read = read_txn(&implementation);
        assert!(
            implementation
                .storage
                .get_vertex(VertexId(100), &read)
                .unwrap()
                .is_some(),
            "the committed vertex must survive"
        );
        assert!(
            implementation
                .storage
                .get_vertex(VertexId(200), &read)
                .unwrap()
                .is_none(),
            "the transaction without a WAL commit marker must be discarded"
        );
        implementation.txn_manager.commit(read).unwrap();

        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(300),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }

    let reopened = MetagdbImpl::with_base_dir(base_dir.clone());
    let read = read_txn(&reopened);
    for (id, must_exist) in [(100u64, true), (200, false), (300, true)] {
        assert_eq!(
            reopened
                .storage
                .get_vertex(VertexId(id), &read)
                .unwrap()
                .is_some(),
            must_exist,
            "vertex {id} visibility after repeated reopen"
        );
    }
    reopened.txn_manager.commit(read).unwrap();
    let _ = std::fs::remove_dir_all(&base_dir);
}

/// A partial trailing write must be discarded, and the records before it must
/// stay readable across repeated reopens.
#[test]
fn torn_segment_tail_is_discarded_and_reopens_stay_consistent() {
    use std::io::Write;

    use crate::storage::record::{PhysicalRecord, RecordBody, RecordKind};

    let base_dir = temp_dir("torn_tail");
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(7),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }

    // Append the first bytes of a valid record, as a torn write would leave.
    let torn = PhysicalRecord {
        offset: 99,
        kind: RecordKind::Vertex,
        logical_id: 99,
        begin_ts: 77,
        prev_offset: 0,
        txn_id: 77,
        body: Some(RecordBody::Vertex(metagdb_core::l2_storage::VertexRecord {
            id: VertexId(99),
            labels: Vec::new(),
            properties: Default::default(),
        })),
    };
    let encoded = torn.encode();
    {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(base_dir.join("records.seg"))
            .unwrap();
        file.write_all(&encoded[..encoded.len() / 2]).unwrap();
        file.sync_all().unwrap();
    }

    // Reopen, commit, reopen again; the committed vertex must be unaffected.
    for round in 0..2 {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let read = read_txn(&implementation);
        assert!(
            implementation
                .storage
                .get_vertex(VertexId(7), &read)
                .unwrap()
                .is_some(),
            "committed vertex lost after torn tail, round {round}"
        );
        assert!(implementation
            .storage
            .get_vertex(VertexId(99), &read)
            .unwrap()
            .is_none());
        implementation.txn_manager.commit(read).unwrap();

        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(8 + round),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }

    // A torn tail that is discarded leaves the segment with exactly the
    // committed records, so further reopens must not re-append anything or the
    // file would grow on every restart.
    let segment = base_dir.join("records.seg");
    let mut lengths = Vec::new();
    for round in 0..3 {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let read = read_txn(&implementation);
        for id in [7u64, 8, 9] {
            assert!(
                implementation
                    .storage
                    .get_vertex(VertexId(id), &read)
                    .unwrap()
                    .is_some(),
                "vertex {id} lost after torn tail recovery, round {round}"
            );
        }
        implementation.txn_manager.commit(read).unwrap();
        lengths.push(std::fs::metadata(&segment).unwrap().len());
    }
    assert_eq!(
        lengths[0], lengths[1],
        "the segment grew on reopen after a torn tail"
    );
    assert_eq!(lengths[1], lengths[2]);
    let _ = std::fs::remove_dir_all(&base_dir);
}

/// Repeated reopens must be stable when an uncommitted tail is present.
#[test]
fn repeated_reopen_with_uncommitted_tail_is_stable() {
    let base_dir = temp_dir("reopen_tail");
    let wal = base_dir.join("wal.log");
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(10),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }
    let committed_wal = std::fs::read(&wal).unwrap();
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(20),
                    labels: Vec::new(),
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        implementation.txn_manager.commit(txn).unwrap();
    }
    std::fs::write(&wal, &committed_wal).unwrap();

    let mut offsets = Vec::new();
    for _ in 0..3 {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let read = read_txn(&implementation);
        assert!(implementation
            .storage
            .get_vertex(VertexId(10), &read)
            .unwrap()
            .is_some());
        assert!(implementation
            .storage
            .get_vertex(VertexId(20), &read)
            .unwrap()
            .is_none());
        implementation.txn_manager.commit(read).unwrap();
        let schema = Arc::new(crate::schema::MetaSchema::demo_schema());
        let storage =
            crate::storage::PersistentStorage::open(schema, base_dir.clone()).unwrap();
        offsets.push(storage.redirection_offset(VertexId(10)));
    }
    assert_eq!(offsets[0], offsets[1]);
    assert_eq!(offsets[1], offsets[2]);
    let _ = std::fs::remove_dir_all(&base_dir);
}

/// `VertexId(1)` and `EdgeId(1)` are distinct logical elements; recovery must
/// not conflate them when they are committed in the same transaction.
#[test]
fn vertex_and_edge_namespaces_do_not_collide_across_reopen() {
    let base_dir = temp_dir("namespace");
    {
        let implementation = MetagdbImpl::with_base_dir(base_dir.clone());
        let schema = implementation.schema.clone();
        let label = schema.vertex_labels().first().unwrap().id;
        let etype = schema.edge_types().first().unwrap().id;
        let txn = implementation
            .txn_manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        let a = implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(0),
                    labels: vec![label],
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        let b = implementation
            .storage
            .insert_vertex(
                metagdb_core::l2_storage::VertexRecord {
                    id: VertexId(0),
                    labels: vec![label],
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        let edge = implementation
            .storage
            .insert_edge(
                metagdb_core::l2_storage::EdgeRecord {
                    id: EdgeId(0),
                    src: a,
                    dst: b,
                    etype,
                    properties: Default::default(),
                },
                &txn,
            )
            .unwrap();
        assert_eq!(edge, EdgeId(1));
        implementation.txn_manager.commit(txn).unwrap();
    }

    let reopened = MetagdbImpl::with_base_dir(base_dir.clone());
    let read = read_txn(&reopened);
    assert!(reopened
        .storage
        .get_vertex(VertexId(1), &read)
        .unwrap()
        .is_some());
    assert!(reopened
        .storage
        .get_vertex(VertexId(2), &read)
        .unwrap()
        .is_some());
    assert!(reopened
        .storage
        .get_edge(EdgeId(1), &read)
        .unwrap()
        .is_some());
    assert_eq!(
        reopened
            .storage
            .out_edges(VertexId(1), EdgeFilter::default(), &read)
            .unwrap()
            .count(),
        1
    );
    reopened.txn_manager.commit(read).unwrap();
    let _ = std::fs::remove_dir_all(&base_dir);
}

#[test]
fn graph_file_import_populates_point_lookups_and_adjacency() {
    let base_dir = temp_dir("graph_import");
    let graph = base_dir.join("sample.graph");
    std::fs::create_dir_all(&base_dir).unwrap();
    std::fs::write(&graph, "t 3 2\nv 0 1 10\nv 1 2 20\nv 2 3 30\ne 0 1\ne 1 2\n").unwrap();

    let implementation =
        MetagdbImpl::build_from_graph_file(graph, base_dir.join("db")).unwrap();
    let read = read_txn(&implementation);
    // External vertex 0 maps to VertexId(1).
    let vertex = implementation
        .storage
        .get_vertex(VertexId(1), &read)
        .unwrap()
        .expect("imported vertex");
    let label_key = implementation.schema.property_key_by_name("label").unwrap();
    assert_eq!(
        vertex.properties.get(&label_key),
        Some(&metagdb_core::common::property::PropertyValue::Int(1))
    );
    assert_eq!(
        implementation
            .storage
            .out_edges(VertexId(1), EdgeFilter::default(), &read)
            .unwrap()
            .count(),
        1
    );
    implementation.txn_manager.commit(read).unwrap();
    let _ = std::fs::remove_dir_all(&base_dir);
}

// ---------------------------------------------------------------------------
// Algorithm evidence: one direct core test and one public-pipeline test each
// ---------------------------------------------------------------------------

fn algorithm_projection(
    implementation: &MetagdbImpl,
) -> Result<crate::compute::projection::CsrProjection> {
    let txn = read_txn(implementation);
    let projection = CsrProjection::build(
        &*implementation.storage,
        &txn,
        &ProjectionSpec {
            vertex_labels: None,
            edge_types: None,
            direction: metagdb_core::l4_compute::algebra::ProjectionDirection::Outgoing,
            vertex_properties: None,
        },
    )?;
    implementation.txn_manager.commit(txn).unwrap();
    Ok(projection)
}

fn distance_rows(stream: &ResultStream) -> Vec<(i64, i64)> {
    stream
        .rows
        .iter()
        .map(|row| match (&row[0], &row[1]) {
            (ResultValue::VertexRef(vertex), ResultValue::Int(distance)) => {
                (vertex.0 as i64, *distance)
            }
            other => panic!("unexpected bfs row: {other:?}"),
        })
        .collect()
}

#[test]
fn algorithm_core_bfs() {
    // A -> B -> C plus A -> C: distances 0, 1, 1 from A.
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let projection = algorithm_projection(&implementation).unwrap();
    let mut args = AlgorithmArgs::default();
    args.params
        .insert("source".into(), metagdb_core::common::property::PropertyValue::Int(1));
    let stream = BfsAlgorithm.execute(&projection, &args).unwrap();
    let mut rows = distance_rows(&stream);
    rows.sort();
    // A -> {B, C, NYC, Acme} -> {SF, D} -> E; LA stays unreachable.
    assert_eq!(
        rows,
        vec![
            (1, 0),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 3),
            (9, 1),
        ]
    );
}

#[test]
fn algorithm_language_bfs() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let stream = run_query(
        &implementation,
        "CALL bfs(source = 1) YIELD vertex, distance RETURN distance ORDER BY distance",
    )
    .unwrap();
    let distances: Vec<i64> = stream
        .rows
        .iter()
        .map(|row| match row[0] {
            ResultValue::Int(value) => value,
            ref other => panic!("unexpected distance {other:?}"),
        })
        .collect();
    assert_eq!(distances, vec![0, 1, 1, 1, 1, 2, 2, 3]);
}

#[test]
fn algorithm_core_wcc() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let projection = algorithm_projection(&implementation).unwrap();
    let stream = WccAlgorithm.execute(&projection, &AlgorithmArgs::default()).unwrap();
    let components: HashMap<i64, i64> = stream
        .rows
        .iter()
        .map(|row| match (&row[0], &row[1]) {
            (ResultValue::VertexRef(vertex), ResultValue::Int(component)) => {
                (vertex.0 as i64, *component)
            }
            other => panic!("unexpected wcc row: {other:?}"),
        })
        .collect();
    // Everyone reachable from A shares A's component label.
    assert_eq!(components[&1], 1);
    assert_eq!(components[&2], 1);
    assert_eq!(components[&9], 1);
}

#[test]
fn algorithm_language_wcc() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let stream = run_query(
        &implementation,
        "CALL wcc() YIELD vertex, component RETURN count(vertex)",
    )
    .unwrap();
    assert_eq!(stream.rows, vec![vec![int(9)]]);
}

#[test]
fn algorithm_core_pagerank() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let projection = algorithm_projection(&implementation).unwrap();
    let mut args = AlgorithmArgs::default();
    args.params.insert(
        "damping".into(),
        metagdb_core::common::property::PropertyValue::Float(0.85),
    );
    let stream = PageRankAlgorithm.execute(&projection, &args).unwrap();
    let mut ranks: Vec<(i64, f64)> = stream
        .rows
        .iter()
        .map(|row| match (&row[0], &row[1]) {
            (ResultValue::VertexRef(vertex), ResultValue::Float(score)) => {
                (vertex.0 as i64, *score)
            }
            other => panic!("unexpected pagerank row: {other:?}"),
        })
        .collect();
    let total: f64 = ranks.iter().map(|(_, score)| *score).sum();
    assert!(
        (total - 1.0).abs() < 1.0e-6,
        "PageRank scores must sum to 1, found {total}"
    );
    assert_eq!(ranks.len(), 9);
    assert!(
        ranks.iter().all(|(_, score)| *score > 0.0),
        "every reachable vertex must receive rank mass"
    );
    ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // Deterministic and stable across repeated runs of the same projection.
    let repeat = PageRankAlgorithm.execute(&projection, &args).unwrap();
    let repeat_rows: Vec<f64> = repeat
        .rows
        .iter()
        .map(|row| match row[1] {
            ResultValue::Float(score) => score,
            _ => panic!("unexpected score"),
        })
        .collect();
    let mut repeat_sorted = repeat_rows.clone();
    repeat_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let first: Vec<f64> = ranks.iter().map(|(_, score)| *score).collect();
    let mut sorted_first = first.clone();
    sorted_first.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert_eq!(sorted_first, repeat_sorted);
    assert!(sorted_first[0] >= sorted_first[sorted_first.len() - 1]);
}

#[test]
fn algorithm_language_pagerank() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let stream = run_query(
        &implementation,
        "CALL pagerank() YIELD vertex, score RETURN count(vertex)",
    )
    .unwrap();
    assert_eq!(stream.rows, vec![vec![int(9)]]);
}

#[test]
fn algorithm_registration_names_are_stable() {
    let implementation = MetagdbImpl::new();
    let names: Vec<String> = implementation
        .algorithms
        .iter()
        .map(|algorithm| algorithm.name().to_string())
        .collect();
    assert_eq!(names, vec!["bfs", "wcc", "pagerank"]);
}

#[test]
fn missing_algorithm_argument_is_a_precise_error() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let error = run_query(&implementation, "CALL bfs() YIELD vertex RETURN vertex").unwrap_err();
    assert!(
        error.to_string().contains("source"),
        "unexpected error: {error}"
    );
}

#[test]
fn unregistered_algorithm_is_rejected_at_plan_time() {
    let implementation = MetagdbImpl::new();
    let parser = MetagdbParser;
    let planner = MetagdbPlanner;
    let optimizer = MetagdbOptimizer;
    let ast = parser.parse("CALL no_such_algorithm() YIELD vertex RETURN vertex").unwrap();
    let plan = planner.plan(ast, &*implementation.schema).unwrap();
    let error = optimizer
        .optimize(
            plan,
            &OptContext {
                schema: implementation.schema.clone(),
                algorithms: registry(&implementation),
            },
        )
        .unwrap_err();
    assert!(error.to_string().contains("not registered"), "{error}");
}

#[test]
fn errors_are_never_silently_empty() {
    let implementation = MetagdbImpl::new();
    let error = run_query(&implementation, "RETURN [x IN [1] | x][0] + 'a'").unwrap_err();
    assert!(matches!(
        error,
        Error::Execute(_) | Error::Unsupported(_) | Error::Plan(_)
    ));
}

#[test]
fn empty_results_keep_their_schema() {
    let implementation = MetagdbImpl::new();
    let stream = run_query(&implementation, "UNWIND [] AS x RETURN x").unwrap();
    assert_eq!(stream.schema, vec!["x".to_string()]);
    assert!(stream.rows.is_empty());
}

#[test]
fn one_hop_point_query_uses_the_redirection_index() {
    let implementation = MetagdbImpl::new();
    install_social9(&implementation).unwrap();
    let stream = run_query(
        &implementation,
        "MATCH (a)-[e]->(b) WHERE id(a) = 1 RETURN b",
    )
    .unwrap();
    assert_eq!(stream.rows.len(), 4);
    // The optimizer rewrites the scan into a point lookup, so the plan must not
    // contain a vertex scan for the anchored node.
    let parser = MetagdbParser;
    let planner = MetagdbPlanner;
    let optimizer = MetagdbOptimizer;
    let ast = parser
        .parse("MATCH (a)-[e]->(b) WHERE id(a) = 1 RETURN b")
        .unwrap();
    let plan = planner.plan(ast, &*implementation.schema).unwrap();
    let physical = optimizer
        .optimize(
            plan,
            &OptContext {
                schema: implementation.schema.clone(),
                algorithms: registry(&implementation),
            },
        )
        .unwrap();
    let root = physical
        .downcast_ref::<crate::compute::physical_ops::PhysicalOp>()
        .unwrap();
    let mut names = Vec::new();
    collect_names(root, &mut names);
    assert!(names.contains(&"CypherMatch"));
}
}
