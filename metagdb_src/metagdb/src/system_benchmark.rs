//! Language-independent system correctness and smoke-performance benchmark.
//!
//! Every correctness case owns a fresh database directory.  The expected
//! outcomes are fixed in the framework and selected by the finalized design
//! vector, while the generated candidate only supplies normal framework
//! components through `assembly::build_benchmark_components`. It checks required
//! algorithm registration; open algorithm semantics live in agent-authored tests.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use metagdb::assembly;
use metagdb_core::benchmark::BenchmarkComponents;
use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l1_model::SchemaMode;
use metagdb_core::l2_storage::{EdgeFilter, EdgeRecord, VertexFilter, VertexRecord};
use metagdb_core::l3_txn::IsolationLevel;
use metagdb_core::{Error, Result};

const RUN_ROOT: &str = "target/system-benchmark";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Durability {
    Restart,
    CrashSafe,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub isolation: IsolationLevel,
    pub concurrency_control: String,
    pub durability: Durability,
    pub schema_mode: SchemaMode,
    pub vertex_labels: Vec<String>,
    pub edge_types: Vec<String>,
    pub property_keys: Vec<String>,
    pub required_algorithms: Vec<String>,
    pub diagnostic: bool,
}

struct Outcome {
    id: &'static str,
    category: &'static str,
    status: &'static str,
    duration_us: u128,
    detail: String,
}

struct Fixture {
    components: BenchmarkComponents,
    label: LabelId,
    edge_type: EdgeTypeId,
    property: PropertyKeyId,
}

impl Fixture {
    fn open(path: &Path) -> Self {
        let components = assembly::build_benchmark_components(path);
        let label = components
            .schema
            .vertex_labels()
            .first()
            .map(|x| x.id)
            .unwrap_or(LabelId(1));
        let edge_type = components
            .schema
            .edge_types()
            .first()
            .map(|x| x.id)
            .unwrap_or(EdgeTypeId(1));
        let property = components
            .schema
            .property_keys()
            .first()
            .map(|x| x.id)
            .unwrap_or(PropertyKeyId(1));
        Self {
            components,
            label,
            edge_type,
            property,
        }
    }

    fn begin(
        &self,
        isolation: IsolationLevel,
        read_only: bool,
    ) -> Result<metagdb_core::l3_txn::TxnContext> {
        self.components.txn_manager.begin(isolation, read_only)
    }

    fn vertex(&self, id: u64, value: i64) -> VertexRecord {
        let mut properties = PropertyMap::new();
        properties.insert(self.property, PropertyValue::Int(value));
        VertexRecord {
            id: VertexId(id),
            labels: vec![self.label],
            properties,
        }
    }

    fn edge(&self, id: u64, src: VertexId, dst: VertexId, value: i64) -> EdgeRecord {
        let mut properties = PropertyMap::new();
        properties.insert(self.property, PropertyValue::Int(value));
        EdgeRecord {
            id: EdgeId(id),
            src,
            dst,
            etype: self.edge_type,
            properties,
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    let root = PathBuf::from(RUN_ROOT);
    if root.exists() {
        fs::remove_dir_all(&root).map_err(io_error)?;
    }
    fs::create_dir_all(&root).map_err(io_error)?;

    let mut outcomes = Vec::new();
    macro_rules! case {
        ($id:literal, $category:literal, $body:expr) => {{
            outcomes.push(run_case(&root, $id, $category, $body));
        }};
    }

    // ---------- Schema and graph-model consistency ----------
    let expected_mode = config.schema_mode;
    let expected_labels = config.vertex_labels.clone();
    let expected_edge_types = config.edge_types.clone();
    let expected_property_keys = config.property_keys.clone();
    case!("C01", "consistency", move |dir| {
        let f = Fixture::open(dir);
        ensure(
            f.components.schema.schema_mode() == expected_mode,
            format!(
                "schema mode mismatch: expected {expected_mode:?}, got {:?}",
                f.components.schema.schema_mode()
            ),
        )?;
        for name in &expected_labels {
            ensure(
                f.components.schema.label_by_name(name).is_some(),
                format!("required vertex label is missing: {name}"),
            )?;
        }
        for name in &expected_edge_types {
            ensure(
                f.components.schema.edge_type_by_name(name).is_some(),
                format!("required edge type is missing: {name}"),
            )?;
        }
        for name in &expected_property_keys {
            ensure(
                f.components.schema.property_key_by_name(name).is_some(),
                format!("required property key is missing: {name}"),
            )?;
        }
        for item in f.components.schema.vertex_labels() {
            ensure(
                f.components.schema.label_by_name(&item.name) == Some(item.id),
                format!("vertex label does not round-trip: {}", item.name),
            )?;
            ensure(
                f.components.schema.label_name(item.id) == Some(item.name.as_str()),
                format!("vertex label id does not round-trip: {:?}", item.id),
            )?;
        }
        for item in f.components.schema.edge_types() {
            ensure(
                f.components.schema.edge_type_by_name(&item.name) == Some(item.id),
                format!("edge type does not round-trip: {}", item.name),
            )?;
            ensure(
                f.components.schema.edge_type_name(item.id) == Some(item.name.as_str()),
                format!("edge type id does not round-trip: {:?}", item.id),
            )?;
        }
        for item in f.components.schema.property_keys() {
            ensure(
                f.components.schema.property_key_by_name(&item.name) == Some(item.id),
                format!("property key does not round-trip: {}", item.name),
            )?;
            ensure(
                f.components.schema.property_key_name(item.id) == Some(item.name.as_str()),
                format!("property key id does not round-trip: {:?}", item.id),
            )?;
        }
        Ok(())
    });

    let iso = config.isolation;
    let single_writer = config.concurrency_control == "single_writer";
    case!("C02", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 10), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 20), &tx)?;
        ensure(a != b, "auto-allocated vertex IDs must be unique")?;
        f.components.txn_manager.commit(tx)?;
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_vertex(a, &r)?.is_some(),
            "first committed vertex missing",
        )?;
        ensure(
            f.components.storage.get_vertex(b, &r)?.is_some(),
            "second committed vertex missing",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("C03", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        let e = f.components.storage.insert_edge(f.edge(0, a, b, 7), &tx)?;
        f.components.txn_manager.commit(tx)?;
        let r = f.begin(iso, true)?;
        let direct = f
            .components
            .storage
            .get_edge(e, &r)?
            .ok_or_else(|| Error::Other("edge point lookup failed".into()))?;
        ensure(direct.src == a && direct.dst == b, "edge endpoints changed")?;
        let out: Vec<_> = f
            .components
            .storage
            .out_edges(a, EdgeFilter::default(), &r)?
            .collect();
        let incoming: Vec<_> = f
            .components
            .storage
            .in_edges(b, EdgeFilter::default(), &r)?
            .collect();
        ensure(
            out.iter().any(|x| x.id == e),
            "out adjacency missing committed edge",
        )?;
        ensure(
            incoming.iter().any(|x| x.id == e),
            "in adjacency missing committed edge",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("C04", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        let e = f.components.storage.insert_edge(f.edge(0, a, b, 3), &tx)?;
        f.components.txn_manager.commit(tx)?;
        let d = f.begin(iso, false)?;
        f.components.storage.delete_edge(e, &d)?;
        f.components.txn_manager.commit(d)?;
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_edge(e, &r)?.is_none(),
            "deleted edge remains visible by ID",
        )?;
        ensure(
            !f.components
                .storage
                .out_edges(a, EdgeFilter::default(), &r)?
                .any(|x| x.id == e),
            "deleted edge remains in out adjacency",
        )?;
        ensure(
            !f.components
                .storage
                .in_edges(b, EdgeFilter::default(), &r)?
                .any(|x| x.id == e),
            "deleted edge remains in in adjacency",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("C05", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        f.components.txn_manager.commit(tx)?;
        let bad = f.begin(iso, false)?;
        let result = f
            .components
            .storage
            .insert_edge(f.edge(0, a, VertexId(b.0 + 999_999), 1), &bad);
        let _ = f.components.txn_manager.abort(bad);
        ensure(
            result.is_err(),
            "storage accepted an edge whose destination vertex does not exist",
        )
    });

    case!("C06", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        let e = f.components.storage.insert_edge(f.edge(0, a, b, 3), &tx)?;
        f.components.txn_manager.commit(tx)?;
        let u = f.begin(iso, false)?;
        let mut changed = f
            .components
            .storage
            .get_edge(e, &u)?
            .ok_or_else(|| Error::Other("edge missing before update".into()))?;
        changed.dst = VertexId(b.0 + 1);
        let result = f.components.storage.update_edge(changed, &u);
        let _ = f.components.txn_manager.abort(u);
        ensure(result.is_err(), "update_edge changed immutable endpoints")
    });

    case!("C07", "consistency", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        let e = f.components.storage.insert_edge(f.edge(0, a, b, 3), &tx)?;
        f.components.txn_manager.commit(tx)?;
        let d = f.begin(iso, false)?;
        match f.components.storage.delete_vertex(a, &d) {
            Err(_) => {
                let _ = f.components.txn_manager.abort(d);
                Ok(())
            }
            Ok(()) => {
                f.components.txn_manager.commit(d)?;
                let r = f.begin(iso, true)?;
                ensure(
                    f.components.storage.get_edge(e, &r)?.is_none(),
                    "successful vertex deletion left a dangling incident edge",
                )?;
                f.components.txn_manager.commit(r)?;
                Ok(())
            }
        }
    });

    if expected_mode == SchemaMode::Strict {
        case!("C08", "consistency", move |dir| {
            let f = Fixture::open(dir);
            let tx = f.begin(iso, false)?;
            let mut record = f.vertex(0, 1);
            record.labels = vec![LabelId(u32::MAX)];
            record
                .properties
                .insert(PropertyKeyId(u32::MAX), PropertyValue::Int(1));
            let result = f.components.storage.insert_vertex(record, &tx);
            let _ = f.components.txn_manager.abort(tx);
            ensure(
                result.is_err(),
                "strict schema accepted undeclared vertex metadata",
            )
        });

        case!("C09", "consistency", move |dir| {
            let f = Fixture::open(dir);
            let seed = f.begin(iso, false)?;
            let a = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
            let b = f.components.storage.insert_vertex(f.vertex(0, 2), &seed)?;
            f.components.txn_manager.commit(seed)?;
            let tx = f.begin(iso, false)?;
            let mut edge = f.edge(0, a, b, 1);
            edge.etype = EdgeTypeId(u32::MAX);
            let result = f.components.storage.insert_edge(edge, &tx);
            let _ = f.components.txn_manager.abort(tx);
            ensure(
                result.is_err(),
                "strict schema accepted an undeclared edge type",
            )
        });
    }

    // ---------- Atomicity ----------
    case!("A01", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, true)?;
        let result = f.components.storage.insert_vertex(f.vertex(0, 1), &tx);
        let _ = f.components.txn_manager.abort(tx);
        ensure(result.is_err(), "read-only transaction accepted a mutation")
    });

    case!("A02", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
        let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
        f.components.txn_manager.abort(tx)?;
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_vertex(a, &r)?.is_none(),
            "aborted insert A became visible",
        )?;
        ensure(
            f.components.storage.get_vertex(b, &r)?.is_none(),
            "aborted insert B became visible",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("A03", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let tx = f.begin(iso, false)?;
        f.components.storage.update_vertex(f.vertex(id.0, 9), &tx)?;
        f.components.txn_manager.abort(tx)?;
        let r = f.begin(iso, true)?;
        ensure(
            value_of(
                &f.components.storage.get_vertex(id, &r)?.ok_or_else(|| {
                    Error::Other("vertex disappeared after aborted update".into())
                })?,
                f.property,
            ) == Some(1),
            "aborted update changed committed value",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("A04", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let tx = f.begin(iso, false)?;
        f.components.storage.delete_vertex(id, &tx)?;
        f.components.txn_manager.abort(tx)?;
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_vertex(id, &r)?.is_some(),
            "aborted delete removed committed vertex",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("A05", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let tx = f.begin(iso, false)?;
        let ctx2 = tx.clone();
        f.components.txn_manager.commit(tx)?;
        ensure(
            f.components.txn_manager.commit(ctx2).is_err(),
            "double commit succeeded",
        )
    });

    case!("A06", "atomicity", move |dir| {
        let f = Fixture::open(dir);
        let mut model = BTreeMap::<VertexId, i64>::new();
        for step in 0..24i64 {
            let tx = f.begin(iso, false)?;
            let inserted = f
                .components
                .storage
                .insert_vertex(f.vertex(0, 100 + step), &tx)?;
            let existing = model.keys().next().copied();
            if let Some(id) = existing {
                f.components
                    .storage
                    .update_vertex(f.vertex(id.0, 1_000 + step), &tx)?;
            }
            let commit = step % 4 != 0;
            if commit {
                f.components.txn_manager.commit(tx)?;
                model.insert(inserted, 100 + step);
                if let Some(id) = existing {
                    model.insert(id, 1_000 + step);
                }
            } else {
                f.components.txn_manager.abort(tx)?;
            }
            let r = f.begin(iso, true)?;
            let actual: BTreeMap<_, _> = f
                .components
                .storage
                .scan_vertices(VertexFilter::default(), &r)?
                .filter_map(|v| value_of(&v, f.property).map(|value| (v.id, value)))
                .collect();
            f.components.txn_manager.commit(r)?;
            ensure(actual == model, format!("reference-model mismatch after step {step}: expected {model:?}, got {actual:?}"))?;
        }
        Ok(())
    });

    // ---------- Isolation (logical histories work even on one OS thread) ----------
    case!("I01", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let w = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &w)?;
        ensure(
            f.components.storage.get_vertex(id, &w)?.is_some(),
            "transaction cannot point-read its own insert",
        )?;
        ensure(
            f.components
                .storage
                .scan_vertices(VertexFilter::default(), &w)?
                .any(|v| v.id == id),
            "transaction scan cannot see its own insert",
        )?;
        f.components.txn_manager.abort(w)?;
        Ok(())
    });

    case!("I02", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let w = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &w)?;
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_vertex(id, &r)?.is_none(),
            "dirty insert visible by point lookup",
        )?;
        ensure(
            !f.components
                .storage
                .scan_vertices(VertexFilter::default(), &r)?
                .any(|v| v.id == id),
            "dirty insert visible by scan",
        )?;
        f.components.txn_manager.commit(r)?;
        f.components.txn_manager.abort(w)?;
        Ok(())
    });

    case!("I03", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let w = f.begin(iso, false)?;
        f.components.storage.update_vertex(f.vertex(id.0, 2), &w)?;
        let point = f
            .components
            .storage
            .get_vertex(id, &w)?
            .ok_or_else(|| Error::Other("updated vertex missing".into()))?;
        ensure(
            value_of(&point, f.property) == Some(2),
            "point lookup cannot see own update",
        )?;
        let scanned = f
            .components
            .storage
            .scan_vertices(VertexFilter::default(), &w)?
            .find(|v| v.id == id)
            .ok_or_else(|| Error::Other("scan lost updated vertex".into()))?;
        ensure(
            value_of(&scanned, f.property) == Some(2),
            "scan cannot see own update",
        )?;
        f.components.txn_manager.abort(w)?;
        Ok(())
    });

    case!("I04", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let w = f.begin(iso, false)?;
        f.components.storage.delete_vertex(id, &w)?;
        ensure(
            f.components.storage.get_vertex(id, &w)?.is_none(),
            "point lookup sees own deletion",
        )?;
        ensure(
            !f.components
                .storage
                .scan_vertices(VertexFilter::default(), &w)?
                .any(|v| v.id == id),
            "scan sees own deletion",
        )?;
        f.components.txn_manager.abort(w)?;
        Ok(())
    });

    case!("I05", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let old = f.begin(iso, true)?;
        let w = f.begin(iso, false)?;
        f.components.storage.update_vertex(f.vertex(id.0, 2), &w)?;
        f.components.txn_manager.commit(w)?;
        let observed = value_of(
            &f.components
                .storage
                .get_vertex(id, &old)?
                .ok_or_else(|| Error::Other("snapshot lost vertex".into()))?,
            f.property,
        );
        if iso != IsolationLevel::ReadCommitted {
            ensure(
                observed == Some(1),
                format!("stable-snapshot violation: expected old value 1, got {observed:?}"),
            )?;
        } else {
            ensure(
                matches!(observed, Some(1 | 2)),
                format!("read-committed transaction observed invalid value: {observed:?}"),
            )?;
        }
        f.components.txn_manager.commit(old)?;
        Ok(())
    });

    case!("I06", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 0), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let t1 = f.begin(iso, false)?;
        let t2 = match f.begin(iso, false) {
            Ok(tx) => tx,
            Err(_) if single_writer => {
                f.components.txn_manager.abort(t1)?;
                return Ok(());
            }
            Err(error) => return Err(error),
        };
        let u1 = f.components.storage.update_vertex(f.vertex(id.0, 1), &t1);
        let u2 = f.components.storage.update_vertex(f.vertex(id.0, 2), &t2);
        let c1 = u1.and_then(|_| f.components.txn_manager.commit(t1));
        let c2 = u2.and_then(|_| f.components.txn_manager.commit(t2));
        ensure(
            c1.is_err() || c2.is_err(),
            "both conflicting writers committed",
        )
    });

    case!("I07", "isolation", move |dir| {
        let f = Fixture::open(dir);
        let seed = f.begin(iso, false)?;
        let id = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
        f.components.txn_manager.commit(seed)?;
        let w = f.begin(iso, false)?;
        f.components.storage.update_vertex(f.vertex(id.0, 2), &w)?;
        let r = f.begin(iso, true)?;
        let observed = f.components.storage.get_vertex(id, &r)?.ok_or_else(|| {
            Error::Other("reader lost committed version during dirty update".into())
        })?;
        ensure(
            value_of(&observed, f.property) == Some(1),
            "reader observed an uncommitted update",
        )?;
        f.components.txn_manager.commit(r)?;
        f.components.txn_manager.abort(w)?;
        Ok(())
    });

    if iso == IsolationLevel::Serializable {
        case!("I08", "isolation", move |dir| {
            let f = Fixture::open(dir);
            let seed = f.begin(iso, false)?;
            let a = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
            let b = f.components.storage.insert_vertex(f.vertex(0, 1), &seed)?;
            f.components.txn_manager.commit(seed)?;
            let t1 = f.begin(iso, false)?;
            let t2 = match f.begin(iso, false) {
                Ok(tx) => tx,
                Err(_) if single_writer => {
                    f.components.txn_manager.abort(t1)?;
                    return Ok(());
                }
                Err(error) => return Err(error),
            };
            ensure(
                f.components.storage.get_vertex(b, &t1)?.is_some(),
                "T1 read failed",
            )?;
            ensure(
                f.components.storage.get_vertex(a, &t2)?.is_some(),
                "T2 read failed",
            )?;
            f.components.storage.update_vertex(f.vertex(a.0, 0), &t1)?;
            f.components.storage.update_vertex(f.vertex(b.0, 0), &t2)?;
            let c1 = f.components.txn_manager.commit(t1);
            let c2 = f.components.txn_manager.commit(t2);
            ensure(
                c1.is_err() || c2.is_err(),
                "serializable mode allowed write skew",
            )
        });
    }

    // ---------- Durability ----------
    case!("D01", "durability", move |dir| {
        let id = {
            let f = Fixture::open(dir);
            let tx = f.begin(iso, false)?;
            let id = f.components.storage.insert_vertex(f.vertex(0, 41), &tx)?;
            f.components.txn_manager.commit(tx)?;
            id
        };
        let reopened = Fixture::open(dir);
        let r = reopened.begin(iso, true)?;
        ensure(
            reopened.components.storage.get_vertex(id, &r)?.is_some(),
            "committed vertex missing after clean reopen",
        )?;
        reopened.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("D02", "durability", move |dir| {
        let id = {
            let f = Fixture::open(dir);
            let tx = f.begin(iso, false)?;
            let id = f.components.storage.insert_vertex(f.vertex(0, 41), &tx)?;
            f.components.txn_manager.abort(tx)?;
            id
        };
        let reopened = Fixture::open(dir);
        let r = reopened.begin(iso, true)?;
        ensure(
            reopened.components.storage.get_vertex(id, &r)?.is_none(),
            "aborted vertex appeared after reopen",
        )?;
        reopened.components.txn_manager.commit(r)?;
        Ok(())
    });

    case!("D03", "durability", move |dir| {
        let first = {
            let f = Fixture::open(dir);
            let tx = f.begin(iso, false)?;
            let id = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
            f.components.txn_manager.commit(tx)?;
            id
        };
        let reopened = Fixture::open(dir);
        let tx = reopened.begin(iso, false)?;
        let second = reopened
            .components
            .storage
            .insert_vertex(reopened.vertex(0, 2), &tx)?;
        reopened.components.txn_manager.commit(tx)?;
        ensure(
            second != first && second.0 > first.0,
            "vertex ID counter was reused or moved backwards after reopen",
        )
    });

    if config.durability == Durability::CrashSafe {
        case!("D04", "durability", move |dir| run_crash_case(
            dir,
            iso,
            "uncommitted",
            false
        ));
        case!("D05", "durability", move |dir| run_crash_case(
            dir,
            iso,
            "committed",
            true
        ));
    }

    case!("D06", "durability", move |dir| {
        let (a, b, e) = {
            let f = Fixture::open(dir);
            let tx = f.begin(iso, false)?;
            let a = f.components.storage.insert_vertex(f.vertex(0, 1), &tx)?;
            let b = f.components.storage.insert_vertex(f.vertex(0, 2), &tx)?;
            let e = f.components.storage.insert_edge(f.edge(0, a, b, 3), &tx)?;
            f.components.txn_manager.commit(tx)?;
            (a, b, e)
        };
        let f = Fixture::open(dir);
        let r = f.begin(iso, true)?;
        ensure(
            f.components.storage.get_edge(e, &r)?.is_some(),
            "committed edge missing after reopen",
        )?;
        ensure(
            f.components
                .storage
                .out_edges(a, EdgeFilter::default(), &r)?
                .any(|x| x.id == e),
            "out adjacency missing after reopen",
        )?;
        ensure(
            f.components
                .storage
                .in_edges(b, EdgeFilter::default(), &r)?
                .any(|x| x.id == e),
            "in adjacency missing after reopen",
        )?;
        f.components.txn_manager.commit(r)?;
        Ok(())
    });

    // ---------- Required analytical capabilities ----------
    let required = config.required_algorithms.clone();
    case!("G01", "algorithms", move |dir| {
        let f = Fixture::open(dir);
        let names: BTreeSet<_> = f
            .components
            .algorithms
            .iter()
            .map(|a| a.name().to_ascii_lowercase())
            .collect();
        for name in &required {
            ensure(
                names.contains(&name.to_ascii_lowercase()),
                format!("required algorithm is not registered: {name}"),
            )?;
        }
        Ok(())
    });

    write_correctness_report(&root, &outcomes)?;
    write_performance_smoke(&root, iso)?;
    print_summary(&outcomes, &root);

    let failed = outcomes.iter().filter(|x| x.status != "PASS").count();
    if failed > 0 && !config.diagnostic {
        return Err(Error::Other(format!(
            "system benchmark failed: {failed} correctness case(s); see {RUN_ROOT}/report.tsv"
        )));
    }
    Ok(())
}

/// Entry used only by a parent benchmark process to create a real abrupt-exit
/// recovery history. `process::abort` skips Rust destructors.
pub fn run_crash_child(scenario: &str, dir: &Path, isolation: IsolationLevel) -> Result<()> {
    let f = Fixture::open(dir);
    let tx = f.begin(isolation, false)?;
    let first = f
        .components
        .storage
        .insert_vertex(f.vertex(777_777, 77), &tx)?;
    let second = f
        .components
        .storage
        .insert_vertex(f.vertex(777_778, 78), &tx)?;
    ensure(
        first == VertexId(777_777) && second == VertexId(777_778),
        "crash child could not preserve explicit test IDs",
    )?;
    match scenario {
        "uncommitted" => std::process::abort(),
        "committed" => {
            f.components.txn_manager.commit(tx)?;
            std::process::abort();
        }
        _ => Err(Error::Other(format!(
            "unknown crash-child scenario: {scenario}"
        ))),
    }
}

fn run_crash_case(
    dir: &Path,
    isolation: IsolationLevel,
    scenario: &str,
    should_exist: bool,
) -> Result<()> {
    let exe = std::env::current_exe().map_err(io_error)?;
    let status = Command::new(exe)
        .arg("--system-benchmark-child")
        .arg(scenario)
        .arg(dir)
        .arg(isolation_name(isolation))
        .status()
        .map_err(io_error)?;
    ensure(
        !status.success(),
        "crash child exited normally; test did not exercise abrupt termination",
    )?;
    let reopened = Fixture::open(dir);
    let r = reopened.begin(isolation, true)?;
    let first = reopened
        .components
        .storage
        .get_vertex(VertexId(777_777), &r)?
        .is_some();
    let second = reopened
        .components
        .storage
        .get_vertex(VertexId(777_778), &r)?
        .is_some();
    reopened.components.txn_manager.commit(r)?;
    ensure(first == should_exist && second == should_exist, format!("after crash/recovery expected both vertices existence={should_exist}, got ({first}, {second})"))
}

fn run_case<F>(root: &Path, id: &'static str, category: &'static str, body: F) -> Outcome
where
    F: FnOnce(&Path) -> Result<()> + std::panic::UnwindSafe,
{
    let dir = root.join(format!("case-{id}"));
    let _ = fs::remove_dir_all(&dir);
    let started = Instant::now();
    let result = catch_unwind(AssertUnwindSafe(|| body(&dir)));
    let (status, detail) = match result {
        Ok(Ok(())) => ("PASS", String::new()),
        Ok(Err(error)) => ("FAIL", error.to_string()),
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .map(|x| (*x).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "non-string panic".into());
            ("PANIC", message)
        }
    };
    Outcome {
        id,
        category,
        status,
        duration_us: started.elapsed().as_micros(),
        detail: clean(&detail),
    }
}

fn write_correctness_report(root: &Path, outcomes: &[Outcome]) -> Result<()> {
    let mut text = String::from("id\tcategory\tstatus\tduration_us\tdetail\n");
    for x in outcomes {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            x.id, x.category, x.status, x.duration_us, x.detail
        ));
    }
    fs::write(root.join("report.tsv"), text).map_err(io_error)
}

fn write_performance_smoke(root: &Path, isolation: IsolationLevel) -> Result<()> {
    let dir = root.join("performance-smoke");
    let _ = fs::remove_dir_all(&dir);
    let result = catch_unwind(AssertUnwindSafe(
        || -> Result<Vec<(&'static str, u128, usize)>> {
            let f = Fixture::open(&dir);
            let tx = f.begin(isolation, false)?;
            let started = Instant::now();
            let mut ids = Vec::new();
            for i in 0..200 {
                ids.push(f.components.storage.insert_vertex(f.vertex(0, i), &tx)?);
            }
            for i in 0..800 {
                let src = ids[i % ids.len()];
                let dst = ids[(i * 17 + 1) % ids.len()];
                f.components
                    .storage
                    .insert_edge(f.edge(0, src, dst, i as i64), &tx)?;
            }
            f.components.txn_manager.commit(tx)?;
            let load = started.elapsed().as_micros();
            let r = f.begin(isolation, true)?;
            let started = Instant::now();
            for id in &ids {
                let _ = f.components.storage.get_vertex(*id, &r)?;
            }
            let lookup = started.elapsed().as_micros();
            let started = Instant::now();
            let count = f
                .components
                .storage
                .scan_vertices(VertexFilter::default(), &r)?
                .count();
            let scan = started.elapsed().as_micros();
            let started = Instant::now();
            for id in &ids {
                let _ = f
                    .components
                    .storage
                    .out_edges(*id, EdgeFilter::default(), &r)?
                    .count();
            }
            let adjacency = started.elapsed().as_micros();
            let rows = vec![
                ("load_200v_800e", load, 1000),
                ("point_lookup", lookup, ids.len()),
                ("vertex_scan", scan, count),
                ("out_adjacency", adjacency, ids.len()),
            ];
            f.components.txn_manager.commit(r)?;
            Ok(rows)
        },
    ));
    let mut text = String::from("operation\tstatus\tduration_us\toperations\tdetail\n");
    match result {
        Ok(Ok(rows)) => {
            for (name, elapsed, operations) in rows {
                text.push_str(&format!("{name}\tPASS\t{elapsed}\t{operations}\t\n"));
            }
        }
        Ok(Err(error)) => text.push_str(&format!(
            "smoke\tFAIL\t0\t0\t{}\n",
            clean(&error.to_string())
        )),
        Err(_) => text.push_str("smoke\tPANIC\t0\t0\tperformance smoke panicked\n"),
    }
    fs::write(root.join("performance.tsv"), text).map_err(io_error)
}

fn print_summary(outcomes: &[Outcome], root: &Path) {
    let mut categories: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for x in outcomes {
        let score = categories.entry(x.category).or_default();
        score.1 += 1;
        if x.status == "PASS" {
            score.0 += 1;
        }
    }
    println!("=== metaGDB system benchmark ===");
    for (category, (passed, total)) in categories {
        println!("  {category:<14} {passed:>2}/{total:<2}");
    }
    for x in outcomes.iter().filter(|x| x.status != "PASS") {
        println!("  {} [{}] {}: {}", x.id, x.category, x.status, x.detail);
    }
    println!("Correctness report: {}", root.join("report.tsv").display());
    println!(
        "Performance report: {}",
        root.join("performance.tsv").display()
    );
}

fn value_of(record: &VertexRecord, key: PropertyKeyId) -> Option<i64> {
    record.properties.get(&key).and_then(PropertyValue::as_i64)
}

fn ensure(condition: bool, message: impl Into<String>) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(Error::Other(message.into()))
    }
}

fn isolation_name(value: IsolationLevel) -> &'static str {
    match value {
        IsolationLevel::ReadUncommitted => "read_uncommitted",
        IsolationLevel::ReadCommitted => "read_committed",
        IsolationLevel::RepeatableRead => "repeatable_read",
        IsolationLevel::SnapshotIsolation => "snapshot_isolation",
        IsolationLevel::Serializable => "serializable",
    }
}

pub fn parse_isolation(value: &str) -> Result<IsolationLevel> {
    match value {
        "read_committed" => Ok(IsolationLevel::ReadCommitted),
        "repeatable_read" => Ok(IsolationLevel::RepeatableRead),
        "snapshot_isolation" => Ok(IsolationLevel::SnapshotIsolation),
        "serializable" => Ok(IsolationLevel::Serializable),
        _ => Err(Error::Other(format!(
            "unsupported benchmark isolation level: {value}"
        ))),
    }
}

pub fn parse_schema_mode(value: &str) -> Result<SchemaMode> {
    match value {
        "strict" => Ok(SchemaMode::Strict),
        "hybrid" => Ok(SchemaMode::Hybrid),
        "schemaless" => Ok(SchemaMode::Schemaless),
        _ => Err(Error::Other(format!(
            "unsupported benchmark schema mode: {value}"
        ))),
    }
}

fn clean(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}
fn io_error(error: std::io::Error) -> Error {
    Error::Other(error.to_string())
}
