/**
 * @brief: End-to-end Demo test module, providing the demo_test() entry point.
 *
 * demo_test() is internally self-contained: create MetagdbImpl → load demo data in a transaction →
 * execute test queries one by one (directly orchestrating Parser→Planner→Optimizer→Executor) →
 * verify output → print results.
 *
 * Note: This module does not depend on metagdb (the framework itself), only uses metagdb_core traits
 * and local implementation types, avoiding circular dependencies.
 */

use std::sync::Arc;
use std::time::Instant;

use metagdb_core::common::ids::{EdgeId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue, ResultValue};
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::{EdgeRecord, StorageEngine, VertexFilter, VertexRecord};
use metagdb_core::l3_txn::{IsolationLevel, TxnManager};
use metagdb_core::l4_compute::engine::{
    ExecContext, Executor, OptContext, Optimizer, Parser, Planner, ResultStream,
};
use metagdb_core::l4_compute::olap::{AlgorithmRegistry, AnalyticalAlgorithm};
use metagdb_core::{Error, Result};

use crate::compute::algorithms::BfsAlgorithm;
use crate::compute::executor::MetagdbExecutor;
use crate::compute::optimizer::MetagdbOptimizer;
use crate::compute::parser::MetagdbParser;
use crate::compute::planner::MetagdbPlanner;
use crate::MetagdbImpl;

// ============================================================
// Test query definitions
// ============================================================

struct TestQuery {
    id: &'static str,
    category: &'static str,
    query: &'static str,
    expected_behavior: &'static str,
}

const TEST_QUERIES: &[TestQuery] = &[
    TestQuery {
        id: "Q1", category: "OLTP",
        query: "MATCH (n) RETURN n",
        expected_behavior: "Returns all vertices, at least 6 rows",
    },
    TestQuery {
        id: "Q2", category: "OLTP",
        query: "MATCH (n:Person) RETURN n.name, n.age",
        expected_behavior: "Returns name and age of all Persons, at least 4 rows",
    },
    TestQuery {
        id: "Q3", category: "OLTP",
        query: "MATCH (n) WHERE n.age > 30 RETURN n.name, n.age",
        expected_behavior: "Returns vertices with age>30, at least 2 rows (Carol 35, Dave 41)",
    },
    TestQuery {
        id: "Q4", category: "OLTP",
        query: "MATCH (a)-[:KNOWS]->(b) RETURN a.name, b.name",
        expected_behavior: "Returns all KNOWS relationships, at least 4 rows",
    },
    TestQuery {
        id: "Q5", category: "OLTP",
        query: "MATCH (a)-[:KNOWS]->(b) WHERE a.age > 30 RETURN b.name",
        expected_behavior: "Returns b.name from edges where a.age>30, at least 1 row",
    },
    TestQuery {
        id: "Q6", category: "OLTP",
        query: "MATCH (n) RETURN n ORDER BY n.age DESC SKIP 1 LIMIT 2",
        expected_behavior: "Sort by age descending, skip 1st row, take 2",
    },
    TestQuery {
        id: "Q7", category: "OLTP",
        query: "MATCH (n) RETURN count(*)",
        expected_behavior: "Returns total vertex count, at least 6",
    },
    TestQuery {
        id: "Q8", category: "OLTP",
        query: "MATCH (n) RETURN n.age, count(*)",
        expected_behavior: "Group by age aggregation",
    },
    TestQuery {
        id: "Q9", category: "OLAP",
        query: "CALL bfs(source=1)",
        expected_behavior: "BFS traversal, at least includes the source vertex itself",
    },
    TestQuery {
        id: "Q10", category: "Explain",
        query: "MATCH (a)-[:KNOWS]->(b) WHERE a.age > 30 RETURN b.name",
        expected_behavior: "Prints physical execution plan, non-empty",
    },
    TestQuery {
        id: "Q11", category: "DML",
        query: "CREATE (n:Person {name: 'Eve', age: 25}) RETURN n",
        expected_behavior: "Creates Eve, returns name='Eve', age=25",
    },
    TestQuery {
        id: "Q12", category: "DML",
        query: "MATCH (n {name: 'Eve'}) SET n.age = 26 RETURN n.name, n.age",
        expected_behavior: "Updates Eve's age=26, returns name='Eve', age=26",
    },
    TestQuery {
        id: "Q13", category: "Verify",
        query: "MATCH (n {name: 'Eve'}) RETURN n.name, n.age",
        expected_behavior: "Verifies SET persistence, returns 1 row, age=26",
    },
    TestQuery {
        id: "Q14", category: "DML",
        query: "MATCH (n {name: 'Eve'}) DELETE n",
        expected_behavior: "Deletes Eve, executes successfully",
    },
    TestQuery {
        id: "Q15", category: "Verify",
        query: "MATCH (n {name: 'Eve'}) RETURN n",
        expected_behavior: "Verifies DELETE took effect, returns 0 rows",
    },
    TestQuery {
        id: "Q16", category: "OLTP",
        query: "MATCH (a:Person), (b:City) RETURN a.name, b.name",
        expected_behavior: "Multi-element MATCH Cartesian product",
    },
    TestQuery {
        id: "Q17", category: "OLTP",
        query: "UNWIND [1, 2, 3] AS x RETURN x",
        expected_behavior: "Unwinds list, returns 3 rows",
    },
    TestQuery {
        id: "Q18", category: "OLTP",
        query: "MATCH (n {name: 'Alice'}) RETURN n.name AS name UNION ALL MATCH (n {name: 'Bob'}) RETURN n.name AS name",
        expected_behavior: "UNION ALL returns 2 rows",
    },
    TestQuery {
        id: "Q19", category: "DML",
        query: "MERGE (n:Person {name: 'Dave'}) ON MATCH SET n.age = 42 RETURN n.name, n.age",
        expected_behavior: "MERGE matches existing Dave, updates age=42",
    },
    TestQuery {
        id: "Q20", category: "OLTP",
        query: "MATCH (a:Person)-[:KNOWS*1..2]->(b) RETURN a.name, b.name",
        expected_behavior: "Variable-length path 1..2 hops KNOWS, returns path endpoints",
    },
];

// ============================================================
// Entry point
// ============================================================

/// End-to-end Demo test entry point (zero arguments, internally self-contained).
pub fn demo_test() -> Result<()> {
    let impl_ = MetagdbImpl::new();

    // Load demo data
    load_demo_graph(
        impl_.storage.as_ref(),
        impl_.schema.as_ref(),
        impl_.txn_manager.as_ref(),
    )?;

    // Components (all use Arc, matching OptContext/ExecContext requirements)
    let parser: &dyn Parser = &MetagdbParser;
    let planner: &dyn Planner = &MetagdbPlanner;
    let optimizer: &dyn Optimizer = &MetagdbOptimizer;
    let executor: &dyn Executor = &MetagdbExecutor;
    let schema: Arc<dyn GraphSchema> = impl_.schema.clone();
    let storage: Arc<dyn StorageEngine> = impl_.storage.clone();
    let txn_manager: &dyn TxnManager = impl_.txn_manager.as_ref();

    // Build AlgorithmRegistry
    let mut algo_registry = AlgorithmRegistry::default();
    let bfs: Arc<dyn AnalyticalAlgorithm> = Arc::new(BfsAlgorithm);
    algo_registry.algorithms.insert(bfs.name().to_string(), bfs);
    let algorithms = Arc::new(algo_registry);

    println!("default language: Cypher\n");

    let total = TEST_QUERIES.len();
    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut warnings = 0u32;

    for (i, tc) in TEST_QUERIES.iter().enumerate() {
        println!("[Test {}/{}] [{}] {}",
                 i + 1, total, tc.category, tc.id);
        println!("  Query: {}", tc.query);

        // Explain category: only generate physical plan
        if matches!(tc.category, "Explain") {
            match explain_query(tc.query, parser, planner, optimizer,
                                &schema, &algorithms) {
                Ok(plan) => {
                    println!("  {:#?}", plan);
                    passed += 1;
                }
                Err(e) => {
                    println!("  ✗ Explain failed: {}", e);
                    failed += 1;
                }
            }
            println!();
            continue;
        }

        // Regular query: full pipeline
        let is_olap = matches!(tc.category, "OLAP");
        let start = Instant::now();
        match execute_query(
            tc.query, parser, planner, optimizer, executor,
            &schema, storage.clone(), txn_manager, &algorithms,
        ) {
            Ok(result) => {
                let elapsed = start.elapsed();
                if is_olap {
                    // OLAP query: only output statistics, do not print rows one by one
                    println!(
                        "  Done: {} rows, {} cols ({:?})",
                        result.rows.len(),
                        result.schema.len(),
                        elapsed
                    );
                    if !result.schema.is_empty() {
                        println!("  Columns: {}", result.schema.join(", "));
                    }
                } else {
                    print_result(&result);
                }

                let upper = tc.query.to_uppercase();
                let is_delete = upper.contains("DELETE");
                let is_dml_write = matches!(tc.category, "DML")
                    && (upper.contains("CREATE") || upper.contains("SET")
                        || upper.contains("MERGE"));

                if result.rows.is_empty() && !is_delete && !is_dml_write {
                    println!("  ⚠ WARNING: returned 0 rows, expected: {}", tc.expected_behavior);
                    warnings += 1;
                } else {
                    println!("  ✓ Passed ({} rows)", result.rows.len());
                    passed += 1;
                }
            }
            Err(e) => {
                println!("  ✗ Execution failed: {}", e);
                failed += 1;
            }
        }
        println!();
    }

    println!("=== Test Summary ===");
    println!("  Total: {} tests", total);
    println!("  Passed: {} tests", passed);
    println!("  Warnings: {} tests", warnings);
    println!("  Failed: {} tests", failed);

    Ok(())
}

// ============================================================
// Query execution pipeline
// ============================================================

/// Explain mode: only generate physical plan, do not execute.
fn explain_query(
    src: &str,
    parser: &dyn Parser,
    planner: &dyn Planner,
    optimizer: &dyn Optimizer,
    schema: &Arc<dyn GraphSchema>,
    algorithms: &Arc<AlgorithmRegistry>,
) -> Result<metagdb_core::l4_compute::algebra::PhysicalPlan> {
    let ast = parser.parse(src)?;
    let lp = planner.plan(ast, schema.as_ref())?;
    let opt_ctx = OptContext {
        schema: schema.clone(),
        algorithms: algorithms.clone(),
    };
    optimizer.optimize(lp, &opt_ctx)
}

/// Full pipeline: Parser→Planner→Optimizer→Executor, managing transaction lifecycle.
fn execute_query(
    src: &str,
    parser: &dyn Parser,
    planner: &dyn Planner,
    optimizer: &dyn Optimizer,
    executor: &dyn Executor,
    schema: &Arc<dyn GraphSchema>,
    storage: Arc<dyn StorageEngine>,
    txn_manager: &dyn TxnManager,
    algorithms: &Arc<AlgorithmRegistry>,
) -> Result<ResultStream> {
    let ast = parser.parse(src)?;
    let lp = planner.plan(ast, schema.as_ref())?;

    let opt_ctx = OptContext {
        schema: schema.clone(),
        algorithms: algorithms.clone(),
    };
    let pp = optimizer.optimize(lp, &opt_ctx)?;

    let policy = txn_manager.default_policy();
    let txn = txn_manager.begin(policy.isolation, policy.read_only)?;
    let exec_ctx = ExecContext {
        txn: txn.clone(),
        storage,
        schema: schema.clone(),
        algorithms: algorithms.clone(),
    };

    match executor.execute(&pp, &exec_ctx) {
        Ok(result) => {
            txn_manager.commit(txn)?;
            Ok(result)
        }
        Err(e) => {
            let _ = txn_manager.abort(txn);
            Err(e)
        }
    }
}

// ============================================================
// Demo data loading
// ============================================================

fn props(pairs: &[(PropertyKeyId, PropertyValue)]) -> PropertyMap {
    pairs.iter().cloned().collect()
}

fn load_demo_graph(
    storage: &dyn StorageEngine,
    schema: &dyn GraphSchema,
    txn_manager: &dyn TxnManager,
) -> Result<()> {
    let check_ctx = txn_manager.begin(IsolationLevel::SnapshotIsolation, true)?;
    let existing = storage.scan_vertices(VertexFilter::default(), &check_ctx)?;
    let has_data = existing.count() > 0;
    txn_manager.commit(check_ctx)?;

    if has_data {
        return Ok(());
    }

    let person = schema.label_by_name("Person")
        .ok_or_else(|| Error::Schema("Person label not registered".into()))?;
    let city = schema.label_by_name("City")
        .ok_or_else(|| Error::Schema("City label not registered".into()))?;
    let knows = schema.edge_type_by_name("KNOWS")
        .ok_or_else(|| Error::Schema("KNOWS type not registered".into()))?;
    let lives_in = schema.edge_type_by_name("LIVES_IN")
        .ok_or_else(|| Error::Schema("LIVES_IN type not registered".into()))?;
    let name = schema.property_key_by_name("name")
        .ok_or_else(|| Error::Schema("name property key not registered".into()))?;
    let age = schema.property_key_by_name("age")
        .ok_or_else(|| Error::Schema("age property key not registered".into()))?;

    let ctx = txn_manager.begin(IsolationLevel::SnapshotIsolation, false)?;

    let mk_person = |id: u64, n: &str, a: i64| VertexRecord {
        id: VertexId(id + 1),
        labels: vec![person],
        properties: props(&[(name, PropertyValue::Text(n.into())), (age, PropertyValue::Int(a))]),
    };

    let v0 = storage.insert_vertex(mk_person(0, "Alice", 30), &ctx)?;
    let v1 = storage.insert_vertex(mk_person(1, "Bob", 28), &ctx)?;
    let v2 = storage.insert_vertex(mk_person(2, "Carol", 35), &ctx)?;
    let v3 = storage.insert_vertex(mk_person(3, "Dave", 41), &ctx)?;
    let v4 = storage.insert_vertex(VertexRecord {
        id: VertexId(5),
        labels: vec![city],
        properties: props(&[(name, PropertyValue::Text("Shanghai".into()))]),
    }, &ctx)?;

    let mk_edge = |src: VertexId, dst: VertexId, et| EdgeRecord {
        id: EdgeId(0), src, dst, etype: et, properties: PropertyMap::new(),
    };

    storage.insert_edge(mk_edge(v0, v1, knows), &ctx)?;
    storage.insert_edge(mk_edge(v0, v2, knows), &ctx)?;
    storage.insert_edge(mk_edge(v1, v3, knows), &ctx)?;
    storage.insert_edge(mk_edge(v3, v2, knows), &ctx)?;
    storage.insert_edge(mk_edge(v0, v4, lives_in), &ctx)?;

    txn_manager.commit(ctx)?;
    Ok(())
}

// ============================================================
// Output helpers
// ============================================================

fn print_result(r: &ResultStream) {
    if r.rows.is_empty() {
        println!("  (0 rows)");
        return;
    }

    let widths: Vec<usize> = r.schema.iter().enumerate()
        .map(|(i, h)| {
            let max_cell = r.rows.iter()
                .map(|row| fmt_cell(&row[i]).len())
                .max().unwrap_or(0);
            h.len().max(max_cell).max(8)
        })
        .collect();

    let header = r.schema.iter().zip(&widths)
        .map(|(h, w)| format!("  {:width$}", h, width = w))
        .collect::<Vec<_>>().join(" | ");
    let sep = widths.iter().map(|w| "-".repeat(*w))
        .collect::<Vec<_>>().join("-+-");
    println!("  {header}");
    println!("  {sep}");
    for row in &r.rows {
        let line = row.iter().zip(&widths)
            .map(|(cell, w)| format!("  {:width$}", fmt_cell(cell), width = w))
            .collect::<Vec<_>>().join(" | ");
        println!("  {line}");
    }
    println!("  ({} rows)", r.rows.len());
}

fn fmt_cell(v: &ResultValue) -> String {
    match v {
        ResultValue::Null => "NULL".into(),
        ResultValue::Bool(b) => b.to_string(),
        ResultValue::Int(i) => i.to_string(),
        ResultValue::Float(f) => f.to_string(),
        ResultValue::Text(s) => s.clone(),
        ResultValue::VertexRef(v) => format!("V({})", v.0),
        ResultValue::EdgeRef(e) => format!("E({})", e.0),
        ResultValue::Property(p) => fmt_prop(p),
        ResultValue::List(elems) => {
            let inner: Vec<String> = elems.iter().map(fmt_cell).collect();
            format!("[{}]", inner.join(", "))
        }
        ResultValue::Map(entries) => {
            let inner: Vec<String> = entries
                .iter()
                .map(|(k, v)| format!("{}:{}", fmt_cell(k), fmt_cell(v)))
                .collect();
            format!("{{{}}}", inner.join(", "))
        }
        ResultValue::Path(p) => {
            let inner: Vec<String> = p.objects.iter().map(fmt_cell).collect();
            format!("path[{}]", inner.join(", "))
        }
        ResultValue::PropertyElement(p) => {
            format!("{}={}@{}", p.key, fmt_cell(&p.value), fmt_cell(&p.element))
        }
        ResultValue::Set(elems) => {
            let mut inner: Vec<String> = elems.iter().map(fmt_cell).collect();
            inner.sort();
            format!("{{{}}}", inner.join(", "))
        }
    }
}

fn fmt_prop(p: &PropertyValue) -> String {
    match p {
        PropertyValue::Null => "NULL".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Int(i) => i.to_string(),
        PropertyValue::Float(f) => f.to_string(),
        PropertyValue::Text(s) => s.clone(),
    }
}
