//! Storage-backed CSR analytical projection.
//!
//! [`CsrProjection`] materializes a compact, rebuildable in-memory view of the
//! committed graph: vertex local IDs are dense `0..n` indices in scan order,
//! adjacency is a compressed-sparse-row array in the direction selected by the
//! [`ProjectionSpec`], and every materialized edge keeps a local ID so edge
//! properties can be read back by request.
//!
//! Property materialization follows `ProjectionSpec::vertex_properties`: the
//! spec has a single property selector, so it governs vertex *and* edge
//! properties.  `None` (the default, used by the fixed performance runner)
//! selects no properties and keeps the projection small on million-vertex
//! graphs; `Some(keys)` materializes exactly those keys.

use std::collections::HashMap;

use metagdb_core::common::ids::{EdgeId, LocalEid, LocalVid, PropertyKeyId, VertexId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l2_storage::{EdgeFilter, StorageEngine, VertexFilter};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{ProjectionDirection, ProjectionSpec};
use metagdb_core::l4_compute::olap::GraphProjection;
use metagdb_core::Result;

/// Compressed-sparse-row projection of a storage-backed property graph.
pub struct CsrProjection {
    globals: Vec<VertexId>,
    local_of: HashMap<VertexId, LocalVid>,
    offsets: Vec<u32>,
    adjacency: Vec<LocalVid>,
    edge_locals: Vec<EdgeId>,
    edge_of: HashMap<EdgeId, LocalEid>,
    vertex_properties: HashMap<(u32, u32), PropertyValue>,
    edge_properties: HashMap<(u32, u32), PropertyValue>,
}

impl CsrProjection {
    /// Build the projection from the visible graph state of `storage`.
    pub fn build(
        storage: &dyn StorageEngine,
        ctx: &TxnContext,
        spec: &ProjectionSpec,
    ) -> Result<Self> {
        let filter = VertexFilter {
            labels: spec.vertex_labels.clone(),
        };
        let globals: Vec<VertexId> = storage
            .scan_vertices(filter, ctx)?
            .map(|record| record.id)
            .collect();
        let mut local_of = HashMap::with_capacity(globals.len());
        for (index, id) in globals.iter().enumerate() {
            local_of.insert(*id, LocalVid(index as u32));
        }

        let edge_filter = EdgeFilter {
            types: spec.edge_types.clone(),
        };
        let property_keys: Option<Vec<PropertyKeyId>> = spec.vertex_properties.clone();

        let mut offsets = Vec::with_capacity(globals.len() + 1);
        let mut adjacency: Vec<LocalVid> = Vec::new();
        let mut edge_locals: Vec<EdgeId> = Vec::new();
        let mut edge_of: HashMap<EdgeId, LocalEid> = HashMap::new();
        let mut vertex_properties = HashMap::new();
        let mut edge_properties = HashMap::new();

        offsets.push(0u32);
        for (index, global) in globals.iter().enumerate() {
            let local = LocalVid(index as u32);
            if let Some(keys) = property_keys.as_ref() {
                if let Some(record) = storage.get_vertex(*global, ctx)? {
                    for key in keys {
                        if let Some(value) = record.properties.get(key) {
                            vertex_properties.insert((local.0, key.0), value.clone());
                        }
                    }
                }
            }

            let mut outgoing: Vec<_> = match spec.direction {
                ProjectionDirection::Outgoing | ProjectionDirection::Both => storage
                    .out_edges(*global, edge_filter.clone(), ctx)?
                    .collect(),
                ProjectionDirection::Incoming => Vec::new(),
            };
            let mut incoming: Vec<_> = match spec.direction {
                ProjectionDirection::Incoming | ProjectionDirection::Both => storage
                    .in_edges(*global, edge_filter.clone(), ctx)?
                    .collect(),
                ProjectionDirection::Outgoing => Vec::new(),
            };
            outgoing.sort_unstable_by_key(|edge| edge.id);
            incoming.sort_unstable_by_key(|edge| edge.id);

            // An edge can be reached from both endpoints only in the `Both`
            // direction; otherwise each edge is visited exactly once.
            let deduplicated = matches!(spec.direction, ProjectionDirection::Both)
                || property_keys.is_some();
            let mut sink = EdgeSink {
                locals: &mut edge_locals,
                index: deduplicated.then_some(&mut edge_of),
                properties: &mut edge_properties,
                property_keys: &property_keys,
            };
            for edge in &outgoing {
                if let Some(target) = local_of.get(&edge.dst) {
                    sink.record(edge);
                    adjacency.push(*target);
                }
            }
            for edge in &incoming {
                if let Some(target) = local_of.get(&edge.src) {
                    sink.record(edge);
                    adjacency.push(*target);
                }
            }
            offsets.push(adjacency.len() as u32);
        }

        Ok(Self {
            globals,
            local_of,
            offsets,
            adjacency,
            edge_locals,
            edge_of,
            vertex_properties,
            edge_properties,
        })
    }

    /// Number of distinct edges materialized by the projection.
    pub fn materialized_edges(&self) -> usize {
        self.edge_locals.len()
    }

    /// Global ID of a materialized edge.
    pub fn edge_id(&self, edge: LocalEid) -> Option<EdgeId> {
        self.edge_locals.get(edge.0 as usize).copied()
    }

    /// Projection-local ID of a materialized edge.
    ///
    /// The reverse index is only maintained when a projection could visit an
    /// edge twice or when properties were selected, so this returns `None` for
    /// a plain single-direction structural projection.
    pub fn local_eid_of(&self, edge: EdgeId) -> Option<LocalEid> {
        self.edge_of.get(&edge).copied()
    }
}

struct EdgeSink<'a> {
    locals: &'a mut Vec<EdgeId>,
    /// Reverse edge index. `None` when the projection traverses each edge from
    /// exactly one endpoint and materializes no properties, in which case no
    /// lookup can ever hit it and maintaining it would cost two hash probes per
    /// edge of a million-edge graph.
    index: Option<&'a mut HashMap<EdgeId, LocalEid>>,
    properties: &'a mut HashMap<(u32, u32), PropertyValue>,
    property_keys: &'a Option<Vec<PropertyKeyId>>,
}

impl EdgeSink<'_> {
    /// Register a materialized edge once and remember its selected properties.
    fn record(&mut self, record: &metagdb_core::l2_storage::EdgeRecord) -> LocalEid {
        if let Some(index) = self.index.as_mut() {
            if let Some(existing) = index.get(&record.id) {
                return *existing;
            }
            let local = LocalEid(self.locals.len() as u32);
            self.locals.push(record.id);
            index.insert(record.id, local);
            self.record_properties(record, local);
            return local;
        }
        let local = LocalEid(self.locals.len() as u32);
        self.locals.push(record.id);
        self.record_properties(record, local);
        local
    }

    fn record_properties(
        &mut self,
        record: &metagdb_core::l2_storage::EdgeRecord,
        local: LocalEid,
    ) {
        if let Some(keys) = self.property_keys.as_ref() {
            for key in keys {
                if let Some(value) = record.properties.get(key) {
                    self.properties.insert((local.0, key.0), value.clone());
                }
            }
        }
    }
}

impl GraphProjection for CsrProjection {
    fn num_vertices(&self) -> usize {
        self.globals.len()
    }

    fn num_edges(&self) -> usize {
        self.edge_locals.len()
    }

    fn neighbors(&self, v: LocalVid) -> &[LocalVid] {
        let index = v.0 as usize;
        if index + 1 >= self.offsets.len() {
            return &[];
        }
        let start = self.offsets[index] as usize;
        let end = self.offsets[index + 1] as usize;
        &self.adjacency[start..end]
    }

    fn vertex_property(&self, v: LocalVid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.vertex_properties.get(&(v.0, key.0))
    }

    fn edge_property(&self, e: LocalEid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.edge_properties.get(&(e.0, key.0))
    }

    fn original_vertex_id(&self, v: LocalVid) -> Option<VertexId> {
        self.globals.get(v.0 as usize).copied()
    }

    fn local_vid_of(&self, vid: VertexId) -> Option<LocalVid> {
        self.local_of.get(&vid).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::test_support::MemoryStorage;

    /// 1 -> 2 -> 3 and 4 -> 2: a hand-checkable four-vertex directed graph.
    fn chain() -> MemoryStorage {
        MemoryStorage::new()
            .vertex(1, &[1], PropertyKeyId(1), 10)
            .vertex(2, &[1], PropertyKeyId(1), 20)
            .vertex(3, &[1], PropertyKeyId(1), 30)
            .vertex(4, &[1], PropertyKeyId(1), 40)
            .edge(1, 1, 2, 1)
            .edge_property(PropertyKeyId(1), 100)
            .edge(2, 2, 3, 1)
            .edge_property(PropertyKeyId(1), 200)
            .edge(3, 4, 2, 1)
            .edge_property(PropertyKeyId(1), 300)
    }

    fn outgoing() -> ProjectionSpec {
        ProjectionSpec {
            direction: ProjectionDirection::Outgoing,
            ..ProjectionSpec::default()
        }
    }

    fn neighbors_of(projection: &CsrProjection, id: u64) -> Vec<u64> {
        let local = projection
            .local_vid_of(VertexId(id))
            .expect("vertex is in the projection");
        projection
            .neighbors(local)
            .iter()
            .map(|v| projection.original_vertex_id(*v).unwrap().0)
            .collect()
    }

    #[test]
    fn outgoing_projection_matches_hand_checked_adjacency() {
        let storage = chain();
        let projection =
            CsrProjection::build(&storage, &MemoryStorage::context(), &outgoing()).expect("projection");
        assert_eq!(projection.num_vertices(), 4);
        assert_eq!(projection.num_edges(), 3);

        assert_eq!(neighbors_of(&projection, 1), vec![2]);
        assert_eq!(neighbors_of(&projection, 2), vec![3]);
        assert_eq!(neighbors_of(&projection, 3), Vec::<u64>::new());
        assert_eq!(neighbors_of(&projection, 4), vec![2]);
    }

    #[test]
    fn incoming_projection_reverses_every_edge() {
        let storage = chain();
        let spec = ProjectionSpec {
            direction: ProjectionDirection::Incoming,
            ..ProjectionSpec::default()
        };
        let projection =
            CsrProjection::build(&storage, &MemoryStorage::context(), &spec).expect("projection");
        assert_eq!(neighbors_of(&projection, 2), vec![1, 4]);
        assert_eq!(neighbors_of(&projection, 3), vec![2]);
        assert_eq!(neighbors_of(&projection, 1), Vec::<u64>::new());
    }

    #[test]
    fn both_direction_ignores_edge_orientation() {
        let storage = chain();
        let spec = ProjectionSpec {
            direction: ProjectionDirection::Both,
            ..ProjectionSpec::default()
        };
        let projection =
            CsrProjection::build(&storage, &MemoryStorage::context(), &spec).expect("projection");
        // 1 -> 2 -> 3 and 4 -> 2 form one weakly connected component.
        let start = projection.local_vid_of(VertexId(1)).unwrap();
        let mut seen = vec![start];
        let mut frontier = vec![start];
        while let Some(current) = frontier.pop() {
            for next in projection.neighbors(current) {
                if !seen.contains(next) {
                    seen.push(*next);
                    frontier.push(*next);
                }
            }
        }
        assert_eq!(seen.len(), 4);
    }

    #[test]
    fn selected_property_keys_are_readable_for_edges_too() {
        let storage = chain();
        let key = PropertyKeyId(1);
        let projection = CsrProjection::build(
            &storage,
            &MemoryStorage::context(),
            &ProjectionSpec {
                vertex_properties: Some(vec![key]),
                ..outgoing()
            },
        )
        .unwrap();
        let local = projection.local_eid_of(EdgeId(2)).expect("edge 2 projected");
        assert_eq!(projection.edge_id(local), Some(EdgeId(2)));
        assert_eq!(
            projection.edge_property(local, key),
            Some(&PropertyValue::Int(200))
        );
        assert!(projection.local_eid_of(EdgeId(99)).is_none());
    }

    #[test]
    fn properties_are_materialized_only_when_selected() {
        let storage = chain();
        let key = PropertyKeyId(1);
        let without =
            CsrProjection::build(&storage, &MemoryStorage::context(), &outgoing()).unwrap();
        let first = without.local_vid_of(VertexId(1)).unwrap();
        assert!(without.vertex_property(first, key).is_none());

        let with = CsrProjection::build(
            &storage,
            &MemoryStorage::context(),
            &ProjectionSpec {
                vertex_properties: Some(vec![key]),
                ..outgoing()
            },
        )
        .unwrap();
        assert_eq!(
            with.vertex_property(first, key),
            Some(&PropertyValue::Int(10))
        );
    }

    #[test]
    fn an_empty_graph_projects_to_empty_arrays() {
        let storage = MemoryStorage::new();
        let projection =
            CsrProjection::build(&storage, &MemoryStorage::context(), &outgoing()).unwrap();
        assert_eq!(projection.num_vertices(), 0);
        assert_eq!(projection.num_edges(), 0);
        assert!(projection.neighbors(LocalVid(0)).is_empty());
    }
}
