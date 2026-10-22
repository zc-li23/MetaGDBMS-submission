//! Rebuildable in-memory CSR analytical projection.
//!
//! The projection is deliberately *derived* state: it is rebuilt from the
//! storage engine on demand and is never the durable source of truth.  The
//! layout is a conventional CSR neighbour array plus a global/local id map, so
//! the analytical algorithms stay independent of the storage layout.

use std::collections::HashMap;

use metagdb_core::common::ids::{LocalEid, LocalVid, PropertyKeyId, VertexId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l2_storage::{EdgeFilter, StorageEngine, VertexFilter};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{ProjectionDirection, ProjectionSpec};
use metagdb_core::l4_compute::olap::GraphProjection;
use metagdb_core::Result;

/// Compressed-sparse-row graph projection.
pub struct CsrProjection {
    offsets: Vec<usize>,
    neighbors: Vec<LocalVid>,
    edge_ids: Vec<LocalEid>,
    local_to_global: Vec<VertexId>,
    global_to_local: HashMap<VertexId, LocalVid>,
    vertex_properties: HashMap<(u32, PropertyKeyId), PropertyValue>,
    edge_properties: HashMap<(u32, PropertyKeyId), PropertyValue>,
    num_edges: usize,
}

impl CsrProjection {
    /// Build the projection selected by `spec` over the transaction's snapshot.
    pub fn build(
        storage: &dyn StorageEngine,
        ctx: &TxnContext,
        spec: &ProjectionSpec,
    ) -> Result<Self> {
        let filter = VertexFilter {
            labels: spec.vertex_labels.clone(),
        };
        let vertices: Vec<_> = storage.scan_vertices(filter, ctx)?.collect();

        let mut local_to_global = Vec::with_capacity(vertices.len());
        let mut global_to_local = HashMap::with_capacity(vertices.len());
        let mut vertex_properties = HashMap::new();
        for (index, vertex) in vertices.iter().enumerate() {
            let local = LocalVid(index as u32);
            local_to_global.push(vertex.id);
            global_to_local.insert(vertex.id, local);
            for (key, value) in &vertex.properties {
                if let Some(wanted) = &spec.vertex_properties {
                    if !wanted.contains(key) {
                        continue;
                    }
                }
                vertex_properties.insert((local.0, *key), value.clone());
            }
        }

        let edge_filter = EdgeFilter {
            types: spec.edge_types.clone(),
        };
        let mut offsets = Vec::with_capacity(vertices.len() + 1);
        let mut neighbors = Vec::new();
        let mut edge_ids = Vec::new();
        let mut edge_properties = HashMap::new();
        let mut edge_counter = 0u32;

        for vertex in &vertices {
            offsets.push(neighbors.len());
            let mut collected: Vec<metagdb_core::l2_storage::EdgeRecord> = Vec::new();
            if matches!(
                spec.direction,
                ProjectionDirection::Outgoing | ProjectionDirection::Both
            ) {
                collected.extend(storage.out_edges(vertex.id, edge_filter.clone(), ctx)?);
            }
            if matches!(
                spec.direction,
                ProjectionDirection::Incoming | ProjectionDirection::Both
            ) {
                collected.extend(storage.in_edges(vertex.id, edge_filter.clone(), ctx)?);
            }
            for edge in collected {
                let neighbor = match spec.direction {
                    ProjectionDirection::Incoming => edge.src,
                    _ => edge.dst,
                };
                let Some(local) = global_to_local.get(&neighbor) else {
                    continue;
                };
                let local_edge = LocalEid(edge_counter);
                edge_counter += 1;
                neighbors.push(*local);
                edge_ids.push(local_edge);
                for (key, value) in &edge.properties {
                    edge_properties.insert((local_edge.0, *key), value.clone());
                }
            }
        }
        offsets.push(neighbors.len());

        Ok(Self {
            num_edges: neighbors.len(),
            offsets,
            neighbors,
            edge_ids,
            local_to_global,
            global_to_local,
            vertex_properties,
            edge_properties,
        })
    }

    /// Local vertex ids in projection order.
    pub fn local_ids(&self) -> impl Iterator<Item = LocalVid> + '_ {
        (0..self.local_to_global.len() as u32).map(LocalVid)
    }

    /// Outgoing edge local ids parallel to `neighbors`.
    pub fn edge_local_ids(&self) -> &[LocalEid] {
        &self.edge_ids
    }
}

impl GraphProjection for CsrProjection {
    fn num_vertices(&self) -> usize {
        self.local_to_global.len()
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn neighbors(&self, v: LocalVid) -> &[LocalVid] {
        let index = v.0 as usize;
        if index + 1 >= self.offsets.len() {
            return &[];
        }
        &self.neighbors[self.offsets[index]..self.offsets[index + 1]]
    }

    fn vertex_property(&self, v: LocalVid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.vertex_properties.get(&(v.0, key))
    }

    fn edge_property(&self, e: LocalEid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.edge_properties.get(&(e.0, key))
    }

    fn original_vertex_id(&self, v: LocalVid) -> Option<VertexId> {
        self.local_to_global.get(v.0 as usize).copied()
    }

    fn local_vid_of(&self, vid: VertexId) -> Option<LocalVid> {
        self.global_to_local.get(&vid).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{EdgeId, LabelId, VertexId};
    use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
    use metagdb_core::l3_txn::{IsolationLevel, TxnManager};
    use std::sync::Arc;

    use crate::schema::MetaSchema;
    use crate::storage::PersistentStorage;
    use crate::txn::MvccTxnManager;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "metagdb_thr02_projection_{name}_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn runtime(name: &str) -> (Arc<PersistentStorage>, MvccTxnManager) {
        let schema = Arc::new(MetaSchema::demo_schema());
        let storage = Arc::new(
            PersistentStorage::open(schema, temp_dir(name)).expect("storage opens"),
        );
        let manager = MvccTxnManager::new(storage.clone());
        (storage, manager)
    }

    #[test]
    fn projection_matches_a_hand_checkable_graph() {
        // 1 -> 2 -> 3, plus 1 -> 3
        let (storage, manager) = runtime("known_answer");
        let schema = storage.schema().clone();
        let label = schema.label_by_name("Person").unwrap();
        let etype = schema.edge_type_by_name("KNOWS").unwrap();
        let tx = manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        let a = storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![label],
                    properties: Default::default(),
                },
                &tx,
            )
            .unwrap();
        let b = storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![label],
                    properties: Default::default(),
                },
                &tx,
            )
            .unwrap();
        let c = storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![label],
                    properties: Default::default(),
                },
                &tx,
            )
            .unwrap();
        for (src, dst) in [(a, b), (b, c), (a, c)] {
            storage
                .insert_edge(
                    EdgeRecord {
                        id: EdgeId(0),
                        src,
                        dst,
                        etype,
                        properties: Default::default(),
                    },
                    &tx,
                )
                .unwrap();
        }
        manager.commit(tx).unwrap();

        let read = manager
            .begin(IsolationLevel::SnapshotIsolation, true)
            .unwrap();
        let projection = CsrProjection::build(
            &*storage,
            &read,
            &ProjectionSpec {
                vertex_labels: Some(vec![label]),
                edge_types: Some(vec![etype]),
                direction: ProjectionDirection::Outgoing,
                vertex_properties: None,
            },
        )
        .unwrap();

        assert_eq!(projection.num_vertices(), 3);
        assert_eq!(projection.num_edges(), 3);
        assert_eq!(projection.local_vid_of(a), Some(LocalVid(0)));
        assert_eq!(projection.original_vertex_id(LocalVid(1)), Some(b));
        let first: Vec<u32> = projection
            .neighbors(LocalVid(0))
            .iter()
            .map(|v| v.0)
            .collect();
        assert_eq!(first, vec![1, 2]);
        assert_eq!(projection.neighbors(LocalVid(1)).len(), 1);
        assert!(projection.neighbors(LocalVid(2)).is_empty());
    }

    #[test]
    fn label_filter_excludes_other_vertices() {
        let (storage, manager) = runtime("label_filter");
        let schema = storage.schema().clone();
        let person = schema.label_by_name("Person").unwrap();
        let city = schema.label_by_name("City").unwrap();
        let tx = manager
            .begin(IsolationLevel::SnapshotIsolation, false)
            .unwrap();
        let a = storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![person],
                    properties: Default::default(),
                },
                &tx,
            )
            .unwrap();
        let _b = storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![city],
                    properties: Default::default(),
                },
                &tx,
            )
            .unwrap();
        manager.commit(tx).unwrap();

        let read = manager
            .begin(IsolationLevel::SnapshotIsolation, true)
            .unwrap();
        let projection = CsrProjection::build(
            &*storage,
            &read,
            &ProjectionSpec {
                vertex_labels: Some(vec![person]),
                edge_types: None,
                direction: ProjectionDirection::Both,
                vertex_properties: None,
            },
        )
        .unwrap();
        assert_eq!(projection.num_vertices(), 1);
        assert_eq!(projection.local_vid_of(a), Some(LocalVid(0)));
    }

    #[test]
    fn unused_label_id_type_is_referenced() {
        let _ = LabelId(0);
    }
}
