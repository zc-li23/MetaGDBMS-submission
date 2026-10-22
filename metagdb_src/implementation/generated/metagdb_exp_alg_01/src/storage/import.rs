//! Bulk import of the metaGDB `.graph` exchange format.
//!
//! The format is a one-line header followed by vertex and edge lines:
//!
//! ```text
//! t <vertex_count> <edge_count>
//! v <zero_based_vertex_id> <label_index> <community>
//! e <zero_based_source> <zero_based_destination>
//! ```
//!
//! The framework contract fixes the ID mapping: `.graph` files are zero-based
//! and `VertexId(0)` is reserved for storage-assigned IDs, so external vertex
//! `v` becomes `VertexId(v + 1)`.  Edge endpoints use the same mapping.
//!
//! The vertex pass runs before the edge pass because an edge may only reference
//! a vertex that already exists; the file is read twice rather than buffering
//! several million endpoints.  The whole import is one transaction: a failed
//! import leaves no partial graph behind.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::l3_txn::IsolationLevel;
use metagdb_core::{Error, Result};

use crate::db::Db;
use crate::schema::SchemaRegistry;

/// Label prefix used for the numeric label index carried by `.graph` files.
pub const IMPORT_LABEL_PREFIX: &str = "L";
/// Property key holding the community index carried by `.graph` files.
pub const IMPORT_COMMUNITY_KEY: &str = "community";
/// Edge type used for every imported edge, which carries no type of its own.
pub const IMPORT_EDGE_TYPE: &str = "E";

/// Outcome of one import call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSummary {
    pub vertices: usize,
    pub edges: usize,
    /// False when the database already held data and the import was skipped.
    pub imported: bool,
}

/// Import `path` into an empty database.
pub fn import_graph_file(db: &Db, schema: &SchemaRegistry, path: &Path) -> Result<ImportSummary> {
    // Validate the header before touching the database, so a malformed file
    // fails without opening a transaction at all.
    read_header(path)?;
    let (existing_vertices, existing_edges) = db.record_counts();
    if existing_vertices != 0 || existing_edges != 0 {
        return Ok(ImportSummary {
            vertices: existing_vertices,
            edges: existing_edges,
            imported: false,
        });
    }

    let community_key = schema.resolve_property_key(IMPORT_COMMUNITY_KEY);
    let edge_type = schema.resolve_edge_type(IMPORT_EDGE_TYPE);

    let ctx = db.begin(IsolationLevel::SnapshotIsolation, false)?;
    let outcome = import_within(&db, schema, &ctx, path, community_key, edge_type);
    match outcome {
        Ok(summary) => {
            db.commit(&ctx)?;
            Ok(summary)
        }
        Err(error) => {
            let _ = db.abort(&ctx);
            Err(error)
        }
    }
}

fn import_within(
    db: &Db,
    schema: &SchemaRegistry,
    ctx: &metagdb_core::l3_txn::TxnContext,
    path: &Path,
    community_key: PropertyKeyId,
    edge_type: EdgeTypeId,
) -> Result<ImportSummary> {
    let mut vertices = 0usize;
    for line in BufReader::new(File::open(path).map_err(io_error)?).lines() {
        let line = line.map_err(io_error)?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("v") {
            continue;
        }
        let external = parse_u64(fields.next(), "vertex id", &line)?;
        let label = parse_u64(fields.next(), "label index", &line)?;
        let community = parse_i64(fields.next(), "community", &line)?;
        let mut properties = PropertyMap::new();
        properties.insert(community_key, PropertyValue::Int(community));
        let record = VertexRecord {
            id: VertexId(external + 1),
            labels: vec![schema.resolve_vertex_label(&format!("{IMPORT_LABEL_PREFIX}{label}"))],
            properties,
        };
        db.insert_vertex(ctx, record)?;
        vertices += 1;
    }

    let mut edges = 0usize;
    for line in BufReader::new(File::open(path).map_err(io_error)?).lines() {
        let line = line.map_err(io_error)?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("e") {
            continue;
        }
        let src = parse_u64(fields.next(), "edge source", &line)?;
        let dst = parse_u64(fields.next(), "edge destination", &line)?;
        let record = EdgeRecord {
            id: EdgeId(0),
            src: VertexId(src + 1),
            dst: VertexId(dst + 1),
            etype: edge_type,
            properties: PropertyMap::new(),
        };
        db.insert_edge(ctx, record)?;
        edges += 1;
    }

    let summary = ImportSummary {
        vertices,
        edges,
        imported: true,
    };
    let (expected_vertices, expected_edges) = read_header(path)?;
    if summary.vertices != expected_vertices || summary.edges != expected_edges {
        return Err(Error::Storage(format!(
            "graph file declares {expected_vertices} vertices and {expected_edges} edges, \
             but {} vertices and {} edges were read",
            summary.vertices, summary.edges
        )));
    }
    Ok(summary)
}

/// Read the `t <vertices> <edges>` header.
pub fn read_header(path: &Path) -> Result<(usize, usize)> {
    let mut reader = BufReader::new(File::open(path).map_err(io_error)?);
    let mut header = String::new();
    reader.read_line(&mut header).map_err(io_error)?;
    let mut fields = header.split_whitespace();
    if fields.next() != Some("t") {
        return Err(Error::InvalidQuery(
            "graph file must start with a 't <vertices> <edges>' header".into(),
        ));
    }
    let vertices = parse_u64(fields.next(), "vertex count", &header)? as usize;
    let edges = parse_u64(fields.next(), "edge count", &header)? as usize;
    Ok((vertices, edges))
}

fn parse_u64(value: Option<&str>, field: &str, line: &str) -> Result<u64> {
    value
        .ok_or_else(|| Error::InvalidQuery(format!("graph line is missing its {field}: {line}")))?
        .parse::<u64>()
        .map_err(|_| Error::InvalidQuery(format!("graph line has a non-numeric {field}: {line}")))
}

fn parse_i64(value: Option<&str>, field: &str, line: &str) -> Result<i64> {
    value
        .ok_or_else(|| Error::InvalidQuery(format!("graph line is missing its {field}: {line}")))?
        .parse::<i64>()
        .map_err(|_| Error::InvalidQuery(format!("graph line has a non-numeric {field}: {line}")))
}

fn io_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch_dir(tag: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "metagdb-exp-alg-01-import-{tag}-{}-{nanos}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    /// A four-vertex, four-edge graph with two labels, hand-checkable by eye.
    const SAMPLE: &str = "t 4 4\nv 0 0 7\nv 1 0 7\nv 2 1 9\nv 3 1 9\ne 0 1\ne 1 2\ne 2 3\ne 0 3\n";

    #[test]
    fn import_maps_external_ids_and_builds_both_adjacency_directions() {
        let dir = scratch_dir("basic");
        let graph = dir.join("sample.graph");
        fs::create_dir_all(&dir).unwrap();
        fs::write(&graph, SAMPLE).unwrap();

        let db = Db::open(&dir.join("db")).unwrap();
        let schema = SchemaRegistry::new();
        let summary = import_graph_file(&db, &schema, &graph).unwrap();
        assert_eq!(summary, ImportSummary { vertices: 4, edges: 4, imported: true });

        let ctx = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        // External vertex 0 maps to VertexId(1).
        let first = db.read_vertex(&ctx, VertexId(1)).unwrap().unwrap();
        assert_eq!(first.labels.len(), 1);
        assert_eq!(
            first.properties.get(&schema.resolve_property_key("community")),
            Some(&PropertyValue::Int(7))
        );
        assert_eq!(db.out_edges(&ctx, VertexId(1), None).unwrap().len(), 2);
        assert_eq!(db.in_edges(&ctx, VertexId(1), None).unwrap().len(), 0);
        assert_eq!(db.in_edges(&ctx, VertexId(2), None).unwrap().len(), 1);
        assert_eq!(db.scan_vertices(&ctx, None).unwrap().len(), 4);
        db.commit(&ctx).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_second_import_into_a_populated_database_is_skipped() {
        let dir = scratch_dir("skip");
        let graph = dir.join("sample.graph");
        fs::create_dir_all(&dir).unwrap();
        fs::write(&graph, SAMPLE).unwrap();

        let db = Db::open(&dir.join("db")).unwrap();
        let schema = SchemaRegistry::new();
        import_graph_file(&db, &schema, &graph).unwrap();
        let again = import_graph_file(&db, &schema, &graph).unwrap();
        assert!(!again.imported);
        assert_eq!(again.vertices, 4);
        assert_eq!(again.edges, 4);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_header_that_disagrees_with_the_body_fails_and_imports_nothing() {
        let dir = scratch_dir("mismatch");
        let graph = dir.join("bad.graph");
        fs::create_dir_all(&dir).unwrap();
        fs::write(&graph, "t 4 5\nv 0 0 7\nv 1 0 7\nv 2 1 9\nv 3 1 9\ne 0 1\ne 1 2\ne 2 3\ne 0 3\n").unwrap();

        let db = Db::open(&dir.join("db")).unwrap();
        let schema = SchemaRegistry::new();
        assert!(import_graph_file(&db, &schema, &graph).is_err());
        assert_eq!(db.record_counts(), (0, 0));
        fs::remove_dir_all(&dir).unwrap();
    }

    /// The tracked manifest names the synthetic datasets; the files themselves
    /// are local experiment inputs, so this smoke test skips when they are absent.
    #[test]
    fn imports_the_local_uniform_dataset_when_it_is_available() {
        // `cargo test` runs with the package directory as the working directory,
        // so the tracked manifest's dataset path is resolved from the workspace
        // root instead.
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
        let graph = workspace.join("data/raw/uniform_standard.graph");
        if !graph.is_file() {
            return;
        }
        let graph = graph.as_path();
        let dir = scratch_dir("uniform");
        let db = Db::open(&dir.join("db")).unwrap();
        let schema = SchemaRegistry::new();
        let summary = import_graph_file(&db, &schema, graph).unwrap();
        assert_eq!(summary, ImportSummary { vertices: 10_000, edges: 160_000, imported: true });

        // Every declared edge must be reachable from its source's adjacency.
        let ctx = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        let mut edges = 0usize;
        for record in db.scan_vertices(&ctx, None).unwrap() {
            edges += db.out_edges(&ctx, record.id, None).unwrap().len();
        }
        assert_eq!(edges, 160_000);
        db.commit(&ctx).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_edge_to_a_missing_vertex_is_rejected() {
        let dir = scratch_dir("dangling");
        let graph = dir.join("dangling.graph");
        fs::create_dir_all(&dir).unwrap();
        fs::write(&graph, "t 2 1\nv 0 0 1\nv 1 0 1\ne 0 9\n").unwrap();

        let db = Db::open(&dir.join("db")).unwrap();
        let schema = SchemaRegistry::new();
        assert!(import_graph_file(&db, &schema, &graph).is_err());
        assert_eq!(db.record_counts(), (0, 0));
        fs::remove_dir_all(&dir).unwrap();
    }
}
