//! Rebuildable CSR analytical projection.
//!
//! The projection is an immutable, contiguous snapshot of the storage-backed
//! graph: a `row_ptr`/`col_idx` CSR adjacency plus per-vertex properties. It is
//! rebuilt on demand after commits and is the only graph view passed to OLAP
//! algorithms.

use std::collections::HashMap;

use metagdb_core::common::ids::{LocalVid, PropertyKeyId, VertexId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l2_storage::{EdgeFilter, StorageEngine, VertexFilter};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{ProjectionDirection, ProjectionSpec};
use metagdb_core::l4_compute::olap::GraphProjection;
use metagdb_core::Result;

/// Standard CSR projection used by the fixed algorithm benchmark.
pub struct CsrProjection {
    local_to_global: Vec<VertexId>,
    global_to_local: HashMap<VertexId, LocalVid>,
    row_ptr: Vec<usize>,
    col_idx: Vec<LocalVid>,
    vertex_props: Vec<HashMap<PropertyKeyId, PropertyValue>>,
}

impl CsrProjection {
    /// Build the projection from a storage snapshot visible to `ctx`.
    pub fn build(
        storage: &dyn StorageEngine,
        ctx: &TxnContext,
        spec: &ProjectionSpec,
    ) -> Result<Self> {
        let filter = VertexFilter {
            labels: spec.vertex_labels.clone(),
        };
        let records: Vec<_> = storage.scan_vertices(filter, ctx)?.collect();

        let mut local_to_global = Vec::with_capacity(records.len());
        let mut global_to_local = HashMap::with_capacity(records.len());
        let mut vertex_props = Vec::with_capacity(records.len());
        for (index, record) in records.iter().enumerate() {
            let local = LocalVid(index as u32);
            local_to_global.push(record.id);
            global_to_local.insert(record.id, local);
            let mut properties = HashMap::new();
            for (key, value) in &record.properties {
                if let Some(only) = &spec.vertex_properties {
                    if !only.contains(key) {
                        continue;
                    }
                }
                properties.insert(*key, value.clone());
            }
            vertex_props.push(properties);
        }

        let edge_filter = EdgeFilter {
            types: spec.edge_types.clone(),
        };
        let mut row_ptr = Vec::with_capacity(records.len() + 1);
        let mut col_idx = Vec::new();
        row_ptr.push(0);
        for record in &records {
            let mut push = |neighbor: VertexId| {
                if let Some(local) = global_to_local.get(&neighbor) {
                    col_idx.push(*local);
                }
            };
            match spec.direction {
                ProjectionDirection::Outgoing => {
                    for edge in storage.out_edges(record.id, edge_filter.clone(), ctx)? {
                        push(edge.dst);
                    }
                }
                ProjectionDirection::Incoming => {
                    for edge in storage.in_edges(record.id, edge_filter.clone(), ctx)? {
                        push(edge.src);
                    }
                }
                ProjectionDirection::Both => {
                    for edge in storage.out_edges(record.id, edge_filter.clone(), ctx)? {
                        push(edge.dst);
                    }
                    for edge in storage.in_edges(record.id, edge_filter.clone(), ctx)? {
                        push(edge.src);
                    }
                }
            }
            row_ptr.push(col_idx.len());
        }

        Ok(Self {
            local_to_global,
            global_to_local,
            row_ptr,
            col_idx,
            vertex_props,
        })
    }
}

impl GraphProjection for CsrProjection {
    fn num_vertices(&self) -> usize {
        self.local_to_global.len()
    }

    fn num_edges(&self) -> usize {
        self.col_idx.len()
    }

    fn neighbors(&self, v: LocalVid) -> &[LocalVid] {
        let start = self.row_ptr[v.0 as usize];
        let end = self.row_ptr[v.0 as usize + 1];
        &self.col_idx[start..end]
    }

    fn vertex_property(&self, v: LocalVid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.vertex_props
            .get(v.0 as usize)
            .and_then(|properties| properties.get(&key))
    }

    fn edge_property(&self, _e: metagdb_core::common::ids::LocalEid, _key: PropertyKeyId) -> Option<&PropertyValue> {
        None
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
    use crate::schema::DynamicSchema;
    use crate::storage::GraphStore;
    use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId};
    use metagdb_core::common::property::PropertyMap;
    use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
    use metagdb_core::l3_txn::IsolationLevel;
    use std::sync::Arc;

    #[test]
    fn known_answer_csr_adjacency() {
        let dir = std::env::temp_dir().join(format!("mgdb_csr_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = GraphStore::with_base_dir(schema, dir.clone()).unwrap();
        let ctx = TxnContext {
            txn_id: 1,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: false,
            start_ts: 1,
        };
        let mut ids = Vec::new();
        for _ in 0..3 {
            ids.push(
                store
                    .insert_vertex(
                        VertexRecord {
                            id: VertexId(0),
                            labels: vec![LabelId(1)],
                            properties: PropertyMap::new(),
                        },
                        &ctx,
                    )
                    .unwrap(),
            );
        }
        store
            .insert_edge(
                EdgeRecord {
                    id: EdgeId(0),
                    src: ids[0],
                    dst: ids[1],
                    etype: EdgeTypeId(1),
                    properties: PropertyMap::new(),
                },
                &ctx,
            )
            .unwrap();
        store
            .insert_edge(
                EdgeRecord {
                    id: EdgeId(0),
                    src: ids[0],
                    dst: ids[2],
                    etype: EdgeTypeId(1),
                    properties: PropertyMap::new(),
                },
                &ctx,
            )
            .unwrap();
        store.commit_txn(1, 2).unwrap();
        let read = TxnContext {
            txn_id: 2,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: true,
            start_ts: 3,
        };

        let spec = ProjectionSpec {
            vertex_labels: None,
            edge_types: None,
            direction: ProjectionDirection::Outgoing,
            vertex_properties: None,
        };
        let projection = CsrProjection::build(&store, &read, &spec).unwrap();
        assert_eq!(projection.num_vertices(), 3);
        assert_eq!(projection.num_edges(), 2);
        let first = projection.local_vid_of(ids[0]).unwrap();
        let neighbors = projection.neighbors(first);
        assert_eq!(neighbors.len(), 2);
        let originals: Vec<VertexId> = neighbors
            .iter()
            .map(|local| projection.original_vertex_id(*local).unwrap())
            .collect();
        assert!(originals.contains(&ids[1]));
        assert!(originals.contains(&ids[2]));

        drop(store);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
