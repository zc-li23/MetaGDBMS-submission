//! Public-pipeline integration harness and requirement tests.
//!
//! Every test here drives the candidate through the same four stages the fixed
//! assembly wires together — `Parser` -> `Planner` -> `Optimizer` -> `Executor`
//! — and opens and commits a transaction exactly the way the framework's
//! `pipeline::executor` does.  No test reaches into a component directly, so a
//! passing test is evidence about the runtime the benchmark will drive.
//!
//! Expected values are derived from the fixture by hand (see
//! [`SOCIAL9_SETUP`]) rather than recorded from a first run.

use std::collections::HashMap;
use std::sync::Arc;

use metagdb_core::common::ids::{EdgeId, VertexId};
use metagdb_core::common::property::{PropertyValue, ResultValue};
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::Result;

use crate::compute::CsrProjection;
use crate::db::Db;
use crate::schema::SchemaRegistry;
use crate::storage::MvccStorage;
use crate::txn::MetagdbTxnManager;

/// The nine-vertex fixture shared with the Core Language Profile.
pub const SOCIAL9_SETUP: &[&str] = &[
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
];

/// One assembled runtime: the shared pipeline plus its storage.
pub struct Harness {
    pub pipeline: crate::runtime::Pipeline,
    db: Db,
}

impl Default for Harness {
    fn default() -> Self {
        Self::new()
    }
}

impl Harness {
    pub fn new() -> Self {
        Self::from_db(Db::in_memory()).expect("the in-memory database opens")
    }

    pub fn from_dir(directory: &std::path::Path) -> Result<Self> {
        Self::from_db(Db::open(directory)?)
    }

    fn from_db(db: Db) -> Result<Self> {
        let pipeline = crate::runtime::Pipeline::new(
            Arc::new(SchemaRegistry::new()),
            Arc::new(MvccStorage::new(db.clone())),
            Arc::new(MetagdbTxnManager::new(db.clone())),
            crate::compute::default_algorithms(),
        );
        Ok(Self { pipeline, db })
    }

    /// Run one query through all four stages inside one transaction.
    pub fn run(&self, source: &str) -> Result<ResultStream> {
        self.pipeline.run(source)
    }

    /// Run the fixture setup, one statement per transaction.
    pub fn load_social9(&self) -> Result<()> {
        for statement in SOCIAL9_SETUP {
            self.run(statement)?;
        }
        Ok(())
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    /// The projection factory the fixed benchmark uses, for kernel checks.
    pub fn projection(
        &self,
        direction: metagdb_core::l4_compute::algebra::ProjectionDirection,
    ) -> Result<CsrProjection> {
        let txn = self
            .pipeline
            .txn_manager
            .begin(metagdb_core::l3_txn::IsolationLevel::SnapshotIsolation, true)?;
        let spec = metagdb_core::l4_compute::algebra::ProjectionSpec {
            direction,
            ..Default::default()
        };
        let projection = CsrProjection::build(self.pipeline.storage.as_ref(), &txn, &spec)?;
        self.pipeline.txn_manager.commit(txn)?;
        Ok(projection)
    }
}

/// Rows as plain tuples of display strings, so an assertion reads like the
/// benchmark's expected value.
pub fn rows(stream: &ResultStream) -> Vec<Vec<String>> {
    stream
        .rows
        .iter()
        .map(|row| row.iter().map(render).collect())
        .collect()
}

pub fn render(value: &ResultValue) -> String {
    match value {
        ResultValue::Null => "null".into(),
        ResultValue::Bool(v) => v.to_string(),
        ResultValue::Int(v) => v.to_string(),
        ResultValue::Float(v) => {
            if v.fract() == 0.0 && v.is_finite() {
                format!("{}", *v as i64)
            } else {
                format!("{v}")
            }
        }
        ResultValue::Text(v) => v.clone(),
        ResultValue::VertexRef(v) => format!("v{}", v.0),
        ResultValue::EdgeRef(v) => format!("e{}", v.0),
        ResultValue::Property(inner) => render_property(inner),
        ResultValue::List(items) => format!(
            "[{}]",
            items.iter().map(render).collect::<Vec<_>>().join(", ")
        ),
        ResultValue::Set(items) => {
            let mut rendered: Vec<String> = items.iter().map(render).collect();
            rendered.sort();
            format!("set{{{}}}", rendered.join(", "))
        }
        ResultValue::Map(entries) => {
            let mut rendered: Vec<String> = entries
                .iter()
                .map(|(key, value)| format!("{}: {}", render(key), render(value)))
                .collect();
            rendered.sort();
            format!("{{{}}}", rendered.join(", "))
        }
        ResultValue::Path(path) => format!(
            "path[{}]",
            path.objects
                .iter()
                .map(render)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ResultValue::PropertyElement(element) => {
            format!("{}={}", element.key, render(&element.value))
        }
    }
}

fn render_property(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(v) => v.to_string(),
        PropertyValue::Int(v) => v.to_string(),
        PropertyValue::Float(v) => format!("{v}"),
        PropertyValue::Text(v) => v.clone(),
    }
}

/// Rows sorted for assertions on unordered results.
pub fn sorted_rows(stream: &ResultStream) -> Vec<Vec<String>> {
    let mut rows = rows(stream);
    rows.sort();
    rows
}

/// `(name -> value)` for a single-column `name, value` result.
pub fn pairs(stream: &ResultStream) -> HashMap<String, String> {
    stream
        .rows
        .iter()
        .map(|row| (render(&row[0]), render(&row[1])))
        .collect()
}

/// One column of a result, in row order.
pub fn column(stream: &ResultStream, index: usize) -> Vec<String> {
    stream.rows.iter().map(|row| render(&row[index])).collect()
}

#[allow(dead_code)]
fn vertex_id(id: u64) -> ResultValue {
    ResultValue::VertexRef(VertexId(id))
}

#[allow(dead_code)]
fn edge_id(id: u64) -> ResultValue {
    ResultValue::EdgeRef(EdgeId(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loaded() -> Harness {
        let harness = Harness::new();
        harness.load_social9().expect("the fixture loads");
        harness
    }

    fn one(harness: &Harness, source: &str) -> String {
        let stream = harness.run(source).unwrap_or_else(|error| panic!("{source}: {error}"));
        assert_eq!(stream.rows.len(), 1, "{source} produced {:?}", rows(&stream));
        assert_eq!(stream.rows[0].len(), 1, "{source} produced extra columns");
        render(&stream.rows[0][0])
    }

    // -- read.access ------------------------------------------------------

    #[test]
    fn counting_and_scanning_match_the_fixture() {
        let harness = loaded();
        assert_eq!(one(&harness, "MATCH (n) RETURN count(n)"), "9");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN count(n)"), "5");
        assert_eq!(one(&harness, "MATCH (n:City) RETURN count(n)"), "3");
        assert_eq!(one(&harness, "MATCH ()-[r]->() RETURN count(r)"), "14");
        assert_eq!(one(&harness, "MATCH ()-[r:KNOWS]->() RETURN count(r)"), "6");
        assert_eq!(
            sorted_rows(&harness.run("MATCH (n:Person) RETURN n.name").unwrap()),
            vec![
                vec!["A".to_string()],
                vec!["B".into()],
                vec!["C".into()],
                vec!["D".into()],
                vec!["E".into()]
            ]
        );
    }

    #[test]
    fn element_ids_are_the_fixture_insertion_ordinals() {
        let harness = loaded();
        assert_eq!(one(&harness, "MATCH (n {name: 'A'}) RETURN id(n)"), "1");
        assert_eq!(one(&harness, "MATCH (n {name: 'Acme'}) RETURN id(n)"), "9");
    }

    #[test]
    fn labels_properties_and_keys_read_back() {
        let harness = loaded();
        assert_eq!(one(&harness, "MATCH (n {name: 'A'}) RETURN size(keys(n))"), "3");
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) RETURN labels(n)"),
            "[Person]"
        );
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) RETURN properties(n)"),
            "{age: 30, joined: 2015, name: A}"
        );
        assert_eq!(
            one(&harness, "MATCH ()-[r:KNOWS]->() RETURN DISTINCT type(r)"),
            "KNOWS"
        );
    }

    // -- read.filter ------------------------------------------------------

    #[test]
    fn predicates_compare_numerically_and_textually() {
        let harness = loaded();
        assert_eq!(
            column(&harness.run("MATCH (n:Person) WHERE n.age > 28 RETURN n.name").unwrap(), 0),
            vec!["A", "C", "D"]
        );
        assert_eq!(
            column(
                &harness
                    .run("MATCH (n) WHERE n.name STARTS WITH 'A' RETURN n.name")
                    .unwrap(),
                0
            ),
            vec!["A", "Acme"]
        );
        assert_eq!(
            sorted_rows(&harness.run("MATCH (n:Person) WHERE n.name IN ['A', 'C'] RETURN n.name").unwrap()),
            vec![vec!["A".to_string()], vec!["C".into()]]
        );
    }

    #[test]
    fn null_comparisons_exclude_rows_and_exists_handles_missing_properties() {
        let harness = loaded();
        // `NOT n.age = 30` is null for vertices without the property.
        assert_eq!(
            sorted_rows(&harness.run("MATCH (n) WHERE NOT n.age = 30 RETURN n.name").unwrap())
                .len(),
            3
        );
        assert_eq!(one(&harness, "MATCH (n) WHERE exists(n.age) RETURN count(n)"), "5");
        assert_eq!(one(&harness, "MATCH (n) WHERE NOT exists(n.age) RETURN count(n)"), "4");
        assert_eq!(one(&harness, "MATCH (n) WHERE NOT (n.age IS NULL) RETURN count(n)"), "5");
    }

    // -- read.navigation --------------------------------------------------

    #[test]
    fn traversal_follows_and_reverses_relationships() {
        let harness = loaded();
        assert_eq!(
            sorted_rows(&harness.run("MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN b.name").unwrap()),
            vec![vec!["B".to_string()], vec!["C".into()]]
        );
        assert_eq!(
            one(&harness, "MATCH (a {name: 'A'})<-[:KNOWS]-(b) RETURN b.name"),
            "E"
        );
        assert_eq!(
            one(&harness, "MATCH (a:Person {name: 'E'})-[:KNOWS]->(b) RETURN b.name"),
            "A"
        );
    }

    #[test]
    fn multi_hop_and_type_restricted_patterns_compose() {
        let harness = loaded();
        assert_eq!(
            sorted_rows(
                &harness
                    .run("MATCH (a {name: 'A'})-[:KNOWS]->()-[:KNOWS]->(c) RETURN c.name")
                    .unwrap()
            ),
            vec![vec!["C".to_string()], vec!["D".into()]]
        );
        assert_eq!(
            one(&harness, "MATCH (a:Person)-[:KNOWS]->(b:Person) RETURN count(b)"),
            "6"
        );
        assert_eq!(
            one(&harness, "MATCH (a:LivesIn)-[r]->(b) RETURN count(r)"),
            "0"
        );
        assert_eq!(
            one(&harness, "MATCH (a:Person)-[:LIVES_IN]->(c:City)<-[:LIVES_IN]-(d:Person) RETURN count(d)"),
            "13"
        );
    }

    // -- read.projection --------------------------------------------------

    #[test]
    fn projections_shape_values_and_maps() {
        let harness = loaded();
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) RETURN n.name + toString(n.age)"),
            "A30"
        );
        assert_eq!(
            one(&harness, "MATCH (n:Person) WHERE n.name = 'A' RETURN [n.name, n.age]"),
            "[A, 30]"
        );
        assert_eq!(
            one(&harness, "MATCH (n:Person) WHERE n.name = 'A' RETURN {name: n.name, age: n.age}"),
            "{age: 30, name: A}"
        );
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) RETURN n"),
            "v1"
        );
        assert_eq!(
            column(&harness.run("MATCH (n:Person) RETURN CASE WHEN n.age >= 30 THEN 'senior' ELSE 'junior' END").unwrap(), 0),
            vec!["senior", "junior", "senior", "senior", "junior"]
        );
    }

    // -- read.aggregation -------------------------------------------------

    #[test]
    fn aggregation_computes_the_fixtures_totals() {
        let harness = loaded();
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN sum(n.age)"), "148");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN min(n.age)"), "25");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN max(n.age)"), "35");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN avg(n.age)"), "29.6");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN count(DISTINCT n.age)"), "4");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN sum(n.age) + count(n)"), "153");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN avg(n.age) * 2"), "59.2");
    }

    #[test]
    fn grouping_keeps_first_occurrence_order_inside_collected_lists() {
        let harness = loaded();
        let stream = harness
            .run("MATCH (n:Person) RETURN n.joined, collect(n.name)")
            .unwrap();
        let grouped = pairs(&stream);
        assert_eq!(grouped.get("2015").map(String::as_str), Some("[A, D]"));
        assert_eq!(grouped.get("2010").map(String::as_str), Some("[C]"));
        assert_eq!(grouped.get("2017").map(String::as_str), Some("[E]"));
        assert_eq!(grouped.get("2018").map(String::as_str), Some("[B]"));
    }

    // -- read.ordering ----------------------------------------------------

    #[test]
    fn ordering_pagination_and_distinct_follow_the_expected_rows() {
        let harness = loaded();
        assert_eq!(
            column(&harness.run("MATCH (n:Person) RETURN n.name ORDER BY n.age, n.name").unwrap(), 0),
            vec!["B", "E", "A", "D", "C"]
        );
        assert_eq!(
            column(
                &harness
                    .run("MATCH (n:Person) RETURN n.name ORDER BY n.name SKIP 1 LIMIT 2")
                    .unwrap(),
                0
            ),
            vec!["B", "C"]
        );
        assert_eq!(
            column(&harness.run("MATCH (n:Person) RETURN DISTINCT n.age ORDER BY n.age").unwrap(), 0),
            vec!["25", "28", "30", "35"]
        );
        assert_eq!(
            column(&harness.run("MATCH (n:Person) WITH n.age AS age LIMIT 2 RETURN age").unwrap(), 0),
            vec!["30", "25"]
        );
    }

    // -- read.path --------------------------------------------------------

    #[test]
    fn variable_length_traversal_returns_every_path() {
        let harness = loaded();
        // Two paths of length two leave A, so `length(p)` has two rows.
        assert_eq!(
            column(&harness.run("MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN length(p)").unwrap(), 0),
            vec!["2", "2"]
        );
        assert_eq!(
            sorted_rows(
                &harness
                    .run("MATCH (a {name: 'A'})-[:KNOWS*2]->(b) RETURN b.name")
                    .unwrap()
            ),
            vec![vec!["C".to_string()], vec!["D".into()]]
        );
        // Length 0 contributes the start vertex itself.
        assert_eq!(
            one(&harness, "MATCH (a {name: 'A'})-[:KNOWS*0..1]->(b) RETURN count(b)"),
            "3"
        );
        assert_eq!(
            one(&harness, "MATCH (a {name: 'A'})-[:KNOWS*1..2]->(b) RETURN count(b)"),
            "4"
        );
        assert_eq!(
            sorted_rows(
                &harness
                    .run("MATCH (a {name: 'A'})-[:KNOWS*1..3]->(b) RETURN DISTINCT b.name")
                    .unwrap()
            ),
            vec![
                vec!["B".to_string()],
                vec!["C".into()],
                vec!["D".into()],
                vec!["E".into()]
            ]
        );
    }

    #[test]
    fn a_named_path_exposes_its_nodes_and_relationships() {
        let harness = loaded();
        assert_eq!(
            column(&harness.run("MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN size(nodes(p))").unwrap(), 0),
            vec!["3", "3"]
        );
        assert_eq!(
            column(&harness.run("MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN size(relationships(p))").unwrap(), 0),
            vec!["2", "2"]
        );
        let stream = harness
            .run("MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN nodes(p)")
            .unwrap();
        assert_eq!(rows(&stream), vec![vec!["[v1, v2, v3]".to_string()], vec!["[v1, v3, v6]".into()]]);
    }

    // -- read.collection and read.function --------------------------------

    #[test]
    fn collection_functions_cover_the_profile_forms() {
        let harness = loaded();
        assert_eq!(one(&harness, "RETURN range(1, 3)"), "[1, 2, 3]");
        assert_eq!(one(&harness, "RETURN range(0, 5, 2)"), "[0, 2, 4]");
        assert_eq!(one(&harness, "RETURN [x IN [1, 2, 3] WHERE x > 1]"), "[2, 3]");
        assert_eq!(one(&harness, "RETURN [x IN [1, 2, 3] | x * 2]"), "[2, 4, 6]");
        assert_eq!(one(&harness, "RETURN [1, 2, 3][-1]"), "3");
        assert_eq!(one(&harness, "RETURN [1, 2, 3, 4][1..3]"), "[2, 3]");
        assert_eq!(one(&harness, "RETURN [1, 2, 3, 4][..2]"), "[1, 2]");
        assert_eq!(one(&harness, "RETURN [1, 2, 3, 4][2..]"), "[3, 4]");
        assert_eq!(one(&harness, "RETURN [1, 2] + [3]"), "[1, 2, 3]");
        assert_eq!(one(&harness, "RETURN [1, 2, 3] + 4"), "[1, 2, 3, 4]");
        assert_eq!(one(&harness, "RETURN head([])"), "null");
        assert_eq!(one(&harness, "RETURN tail([1, 2, 3])"), "[2, 3]");
        assert_eq!(one(&harness, "RETURN reverse([1, 2, 3])"), "[3, 2, 1]");
    }

    #[test]
    fn scalar_functions_cover_the_profile_forms() {
        let harness = loaded();
        assert_eq!(one(&harness, "RETURN toString(30)"), "30");
        assert_eq!(one(&harness, "RETURN toString(1.5)"), "1.5");
        assert_eq!(one(&harness, "RETURN toInteger(3.7)"), "3");
        assert_eq!(one(&harness, "RETURN toInteger('42')"), "42");
        assert_eq!(one(&harness, "RETURN toFloat('1.5')"), "1.5");
        assert_eq!(one(&harness, "RETURN toBoolean('true')"), "true");
        assert_eq!(one(&harness, "RETURN abs(-3)"), "3");
        assert_eq!(one(&harness, "RETURN ceil(1.2)"), "2");
        assert_eq!(one(&harness, "RETURN floor(1.8)"), "1");
        assert_eq!(one(&harness, "RETURN round(1.5)"), "2");
        assert_eq!(one(&harness, "RETURN sign(-4)"), "-1");
        assert_eq!(one(&harness, "RETURN sqrt(9)"), "3");
        assert_eq!(one(&harness, "MATCH (n {name: 'NYC'}) RETURN size(n.name)"), "3");
        assert_eq!(one(&harness, "MATCH (n {name: 'Acme'}) RETURN substring(n.name, 1, 2)"), "cm");
        assert_eq!(one(&harness, "MATCH (n {name: 'Acme'}) RETURN left(n.name, 2)"), "Ac");
        assert_eq!(one(&harness, "MATCH (n {name: 'Acme'}) RETURN right(n.name, 2)"), "me");
        assert_eq!(one(&harness, "MATCH (n {name: 'NYC'}) RETURN replace(n.name, 'N', 'B')"), "BYC");
        assert_eq!(one(&harness, "MATCH (n {name: 'Acme'}) RETURN split(n.name, 'c')"), "[A, me]");
        assert_eq!(one(&harness, "MATCH (n {name: 'NYC'}) RETURN size(toString(n.population))"), "7");
        assert_eq!(one(&harness, "MATCH (n {name: 'B'}) RETURN toString(n.joined) + '-' + n.name"), "2018-B");
    }

    // -- read.advanced ----------------------------------------------------

    #[test]
    fn quantified_predicates_and_temporal_comparisons_evaluate() {
        let harness = loaded();
        assert_eq!(one(&harness, "RETURN all(x IN [1, 2, 3] WHERE x > 0)"), "true");
        assert_eq!(one(&harness, "RETURN all(x IN [1, 2, 3] WHERE x > 2)"), "false");
        assert_eq!(one(&harness, "RETURN any(x IN [1, 2, 3] WHERE x > 2)"), "true");
        assert_eq!(one(&harness, "RETURN none(x IN [] WHERE x > 0)"), "true");
        assert_eq!(one(&harness, "RETURN single(x IN [1, 2, 3] WHERE x = 2)"), "true");
        assert_eq!(one(&harness, "RETURN all(x IN [] WHERE x > 0)"), "true");
        assert_eq!(one(&harness, "RETURN date('2020-01-01') < date('2020-01-02')"), "true");
        assert_eq!(one(&harness, "RETURN date('2020-05-06').year = 2020"), "true");
        assert_eq!(one(&harness, "RETURN date('2020-05-06').day = 6"), "true");
        assert_eq!(
            one(&harness, "RETURN datetime('2020-01-01T00:00:00Z') = datetime('2020-01-01T00:00:00Z')"),
            "true"
        );
        // The text order is the opposite of the length order.
        assert_eq!(one(&harness, "RETURN duration('P1D') > duration('PT12H')"), "true");
        assert_eq!(one(&harness, "RETURN time('12:00:00') < time('13:00:00')"), "true");
    }

    // -- read.composition -------------------------------------------------

    #[test]
    fn union_unwind_and_with_compose_in_pipeline_order() {
        let harness = loaded();
        assert_eq!(
            sorted_rows(
                &harness
                    .run("MATCH (n:City) RETURN n.name UNION MATCH (n:Company) RETURN n.name")
                    .unwrap()
            ),
            vec![
                vec!["Acme".to_string()],
                vec!["LA".into()],
                vec!["NYC".into()],
                vec!["SF".into()]
            ]
        );
        assert_eq!(
            rows(&harness.run("MATCH (n:Person {name: 'A'}) RETURN n.name UNION ALL MATCH (n:Person {name: 'A'}) RETURN n.name").unwrap()),
            vec![vec!["A".to_string()], vec!["A".into()]]
        );
        assert_eq!(
            rows(&harness.run("UNWIND [1, 2, 3] AS x RETURN x").unwrap()),
            vec![vec!["1".to_string()], vec!["2".into()], vec!["3".into()]]
        );
        assert!(harness.run("UNWIND [] AS x RETURN x").unwrap().rows.is_empty());
        assert_eq!(
            rows(&harness.run("UNWIND [1, 2] AS x UNWIND [10, 20] AS y RETURN x, y").unwrap()),
            vec![
                vec!["1".to_string(), "10".into()],
                vec!["1".into(), "20".into()],
                vec!["2".into(), "10".into()],
                vec!["2".into(), "20".into()]
            ]
        );
        assert_eq!(
            one(&harness, "MATCH (n:Person) WITH count(n) AS total RETURN total"),
            "5"
        );
    }

    // -- algorithms through the public language ---------------------------

    /// Required by the algorithm evidence plan: a public-pipeline test distinct
    /// from the phase-03 core test, with an independently reasoned expectation.
    #[test]
    fn language_bfs_public_pipeline_matches_core_result() {
        let harness = loaded();
        let stream = harness
            .run("CALL bfs(1) YIELD node, distance RETURN node, distance")
            .expect("CALL bfs runs through the public pipeline");
        // The traversal runs over every relationship in the projection, not
        // just KNOWS. From vertex 1 (A): B, C, NYC and Acme are one hop away;
        // SF (via B) and D (via C) are two; E is three. LA (vertex 8) has no
        // relationships at all, so it is unreachable and must be absent.
        let mut found = pairs(&stream);
        assert_eq!(found.remove("v1").as_deref(), Some("0"));
        assert_eq!(found.remove("v2").as_deref(), Some("1"));
        assert_eq!(found.remove("v3").as_deref(), Some("1"));
        assert_eq!(found.remove("v4").as_deref(), Some("1"));
        assert_eq!(found.remove("v9").as_deref(), Some("1"));
        assert_eq!(found.remove("v5").as_deref(), Some("2"));
        assert_eq!(found.remove("v6").as_deref(), Some("2"));
        assert_eq!(found.remove("v7").as_deref(), Some("3"));
        assert!(found.is_empty(), "unexpected rows: {found:?}");
    }

    #[test]
    fn language_wcc_public_pipeline_matches_core_result() {
        let harness = loaded();
        let stream = harness
            .run("CALL wcc() YIELD node, component RETURN node, component")
            .expect("CALL wcc runs through the public pipeline");
        // The KNOWS cycle joins A, B, C, D and E; LIVES_IN attaches NYC and SF
        // and WORKS_AT attaches Acme, so vertices 1..7 and 9 form one component
        // whose smallest global ID is 1. LA (vertex 8) has no relationships at
        // all, so it is a component of its own.
        assert_eq!(stream.rows.len(), 9);
        let found = pairs(&stream);
        assert_eq!(found.get("v1").map(String::as_str), Some("1"));
        assert_eq!(found.get("v2").map(String::as_str), Some("1"));
        assert_eq!(found.get("v9").map(String::as_str), Some("1"));
        assert_eq!(found.get("v8").map(String::as_str), Some("8"));
    }

    #[test]
    fn language_pagerank_public_pipeline_matches_core_result() {
        let harness = loaded();
        let stream = harness
            .run("CALL pagerank() YIELD node, rank RETURN node, rank")
            .expect("CALL pagerank runs through the public pipeline");
        assert_eq!(stream.rows.len(), 9);
        let total: f64 = stream
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Float(value) => value,
                ref other => panic!("unexpected rank cell {other:?}"),
            })
            .sum();
        assert!((total - 1.0).abs() < 1e-6, "ranks summed to {total}");
        // Vertices 8 (LA) and 9 (Acme) both have no outgoing relationships, so
        // they receive the same dangling share; Acme additionally has three
        // WORKS_AT in-edges and must therefore outrank LA.
        let ranks = pairs(&stream);
        let rank = |key: &str| ranks[key].parse::<f64>().unwrap();
        assert!(
            rank("v9") > rank("v8"),
            "Acme {} should outrank LA {}",
            rank("v9"),
            rank("v8")
        );
    }

    // -- mutations --------------------------------------------------------

    #[test]
    fn create_set_remove_and_delete_change_the_graph() {
        let harness = loaded();
        assert_eq!(
            one(&harness, "CREATE (n:Person {name: 'F', age: 40}) RETURN n.name"),
            "F"
        );
        assert_eq!(one(&harness, "MATCH (n) RETURN count(n)"), "10");

        // The new vertex is reachable through a fresh relationship.
        harness
            .run("MATCH (a {name: 'A'}), (b {name: 'E'}) CREATE (a)-[:KNOWS {since: 2022}]->(b)")
            .unwrap();
        assert_eq!(
            one(&harness, "MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN count(b)"),
            "3"
        );

        assert_eq!(
            one(&harness, "MATCH (n:Person {name: 'A'}) SET n.age = 31 RETURN n.age"),
            "31"
        );
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) RETURN n.age"),
            "31"
        );
        assert_eq!(
            one(&harness, "MATCH (n:Person {name: 'A'}) REMOVE n.age RETURN n.age"),
            "null"
        );
        assert_eq!(
            one(&harness, "MATCH (n {name: 'A'}) WHERE n.age IS NULL RETURN count(n)"),
            "1"
        );

        assert_eq!(
            one(&harness, "MATCH (n:Person {name: 'A'}) SET n:Admin RETURN size(labels(n))"),
            "2"
        );
        assert_eq!(one(&harness, "MATCH (n:Admin) RETURN count(n)"), "1");

        // DETACH DELETE removes the company and its three relationships.
        assert_eq!(
            one(&harness, "MATCH (n:Company) DETACH DELETE n RETURN count(n)"),
            "1"
        );
        assert_eq!(one(&harness, "MATCH (n:Company) RETURN count(n)"), "0");
        assert_eq!(one(&harness, "MATCH ()-[r:WORKS_AT]->() RETURN count(r)"), "0");
    }

    #[test]
    fn plain_delete_removes_only_an_unconnected_vertex() {
        let harness = loaded();
        assert_eq!(
            one(&harness, "MATCH (n:City {name: 'LA'}) DELETE n RETURN count(n)"),
            "1"
        );
        assert_eq!(one(&harness, "MATCH (n:City) RETURN count(n)"), "2");
    }

    #[test]
    fn merge_matches_before_it_creates_and_runs_its_actions() {
        let harness = loaded();
        assert_eq!(
            one(&harness, "MERGE (n:Person {name: 'A'}) RETURN n.name"),
            "A"
        );
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN count(n)"), "5");

        assert_eq!(
            one(&harness, "MERGE (n:Person {name: 'A'}) ON MATCH SET n.seen = 1 RETURN n.seen"),
            "1"
        );
        assert_eq!(
            one(&harness, "MERGE (n:Person {name: 'F'}) ON CREATE SET n.age = 40 RETURN n.age"),
            "40"
        );
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN count(n)"), "6");
    }

    #[test]
    fn update_uses_the_current_value_and_filters_before_writing() {
        let harness = loaded();
        assert_eq!(
            one(&harness, "MATCH (n:Person {name: 'B'}) SET n.age = n.age + 1 RETURN n.age"),
            "26"
        );
        assert_eq!(
            one(&harness, "MATCH (n:Person) WHERE n.age > 29 SET n.senior = true RETURN count(n)"),
            "3"
        );
        assert_eq!(
            one(&harness, "MATCH (n) WHERE n.senior = true RETURN count(n)"),
            "3"
        );
    }

    // -- error and empty paths --------------------------------------------

    #[test]
    fn grammar_invalid_statements_fail_before_execution() {
        let harness = loaded();
        let error = harness.run("MATCH (n RETURN n").expect_err("must fail");
        assert!(matches!(error, metagdb_core::Error::Parse(_)), "got {error}");
    }

    #[test]
    fn empty_matches_produce_empty_results_and_one_aggregate_row() {
        let harness = loaded();
        assert!(harness
            .run("MATCH (a:Person {name: 'B'})-[:WORKS_AT]->(c) RETURN c.name")
            .unwrap()
            .rows
            .is_empty());
        assert_eq!(one(&harness, "MATCH (n:Person {name: 'Z'}) RETURN count(n)"), "0");
    }

    // -- durability -------------------------------------------------------

    #[test]
    fn committed_data_survives_a_clean_reopen() {
        let dir = std::env::temp_dir().join(format!(
            "metagdb-exp-alg-01-pipeline-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        {
            let harness = Harness::from_dir(&dir).expect("opens");
            harness.load_social9().expect("loads");
            assert_eq!(one(&harness, "MATCH (n) RETURN count(n)"), "9");
        }
        let reopened = Harness::from_dir(&dir).expect("reopens");
        assert_eq!(one(&reopened, "MATCH (n) RETURN count(n)"), "9");
        assert_eq!(one(&reopened, "MATCH ()-[r]->() RETURN count(r)"), "14");
        assert_eq!(
            one(&reopened, "MATCH (n {name: 'A'})-[:KNOWS]->(b) RETURN count(b)"),
            "2"
        );
        // A write after reopening keeps allocating fresh IDs.
        assert_eq!(
            one(&reopened, "CREATE (n:Person {name: 'F'}) RETURN id(n)"),
            "10"
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    // -- regressions found by the external benchmark ----------------------

    #[test]
    fn aggregates_nested_inside_other_expressions_projected() {
        let harness = loaded();
        // `collect` inside `size`, inside a subscript, and twice in one row.
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN collect(n.age)[0]"), "30");
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN size(collect(n.name))"), "5");
        assert_eq!(
            rows(&harness
                .run("MATCH (n:Person) RETURN collect(n.name) AS names, size(collect(n.name)) AS total")
                .unwrap()),
            vec![vec!["[A, B, C, D, E]".to_string(), "5".into()]]
        );
    }

    #[test]
    fn a_with_predicate_sees_the_projection_aliases() {
        let harness = loaded();
        assert_eq!(
            column(&harness.run("MATCH (n:Person) WITH n.age AS age WHERE age >= 30 RETURN age").unwrap(), 0),
            vec!["30", "35", "30"]
        );
        assert_eq!(
            column(
                &harness
                    .run("MATCH (a:Person)-[:KNOWS]->(b) WITH a, count(b) AS degree WHERE degree > 1 RETURN a.name")
                    .unwrap(),
                0
            ),
            vec!["A"]
        );
        assert_eq!(
            one(
                &harness,
                "MATCH (n:Person) WITH collect(n.age) AS ages WHERE all(a IN ages WHERE a >= 25) RETURN size(ages)"
            ),
            "5"
        );
        assert_eq!(
            one(
                &harness,
                "MATCH (n:Person) WITH collect(n.age) AS ages WHERE none(a IN ages WHERE a > 40) RETURN size(ages)"
            ),
            "5"
        );
    }

    #[test]
    fn an_order_by_key_may_mix_an_alias_with_an_input_expression() {
        let harness = loaded();
        assert_eq!(
            rows(&harness
                .run("MATCH (a:Person)-[:KNOWS]->(b) RETURN a.name, count(b) AS degree ORDER BY degree DESC, a.name")
                .unwrap()),
            vec![
                vec!["A".to_string(), "2".into()],
                vec!["B".into(), "1".into()],
                vec!["C".into(), "1".into()],
                vec!["D".into(), "1".into()],
                vec!["E".into(), "1".into()]
            ]
        );
    }

    #[test]
    fn a_named_path_over_several_chains_assembles_the_whole_path() {
        let harness = loaded();
        assert_eq!(
            column(&harness.run("MATCH p = (a {name: 'A'})-[:KNOWS]->(b)-[:KNOWS]->(c) RETURN length(p)").unwrap(), 0),
            vec!["2", "2"]
        );
        assert_eq!(
            column(&harness.run("MATCH p = (a {name: 'A'})-[:KNOWS]->(b)-[:KNOWS]->(c) RETURN size(nodes(p))").unwrap(), 0),
            vec!["3", "3"]
        );
    }

    #[test]
    fn a_create_may_target_an_anonymous_vertex() {
        let harness = loaded();
        assert_eq!(
            one(
                &harness,
                "MATCH (a {name: 'A'}) CREATE (a)-[:KNOWS {since: 2022}]->(:Person {name: 'F'}) RETURN a.name"
            ),
            "A"
        );
        assert_eq!(one(&harness, "MATCH (n:Person) RETURN count(n)"), "6");
        assert_eq!(
            one(&harness, "MATCH (a {name: 'A'})-[:KNOWS]->(b) RETURN count(b)"),
            "3"
        );
    }

    #[test]
    fn a_point_lookup_by_id_reads_exactly_that_vertex() {
        let harness = loaded();
        assert_eq!(one(&harness, "MATCH (n) WHERE id(n) = 1 RETURN n.name"), "A");
        assert_eq!(one(&harness, "MATCH (n) WHERE id(n) = 9 RETURN n.name"), "Acme");
        // An id that does not exist yields no row rather than a fabricated one.
        assert!(harness
            .run("MATCH (n) WHERE id(n) = 99 RETURN n")
            .unwrap()
            .rows
            .is_empty());
        // Additional conjuncts still apply.
        assert!(harness
            .run("MATCH (n) WHERE id(n) = 1 AND n.age > 100 RETURN n")
            .unwrap()
            .rows
            .is_empty());
        // The one-hop form the performance workload uses.
        assert_eq!(
            sorted_rows(&harness.run("MATCH (a)-[e]->(b) WHERE id(a) = 1 RETURN b.name").unwrap()),
            vec![
                vec!["Acme".to_string()],
                vec!["B".into()],
                vec!["C".into()],
                vec!["NYC".into()]
            ]
        );
    }

    #[test]
    fn the_self_checking_demo_passes() {
        // `cargo run -- --demo` reaches this through the assembly, so the demo
        // itself is exercised by the crate's own test gate.
        crate::impl_::MetagdbImpl::demo_test().expect("the demo asserts its own results");
    }

    #[test]
    fn an_aborted_statement_leaves_no_trace() {
        let harness = loaded();
        // A statement that fails during execution must not commit its writes.
        let failing = "MATCH (n:Person) SET n.age = 1 WITH n MATCH (m:Company) SET m.missing = n.missing + 1 RETURN count(m)";
        let _ = harness.run(failing);
        assert_eq!(one(&harness, "MATCH (n:Company {name: 'Acme'}) RETURN count(n)"), "1");
    }
}
