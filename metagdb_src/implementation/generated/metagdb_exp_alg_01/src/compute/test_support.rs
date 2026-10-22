//! Read-only storage double shared by component unit tests.
//!
//! Projection and algorithm tests need a graph with an adjacency list that can
//! be checked by eye.  Using the real MVCC engine would make those tests depend
//! on the transaction phase rather than on the component under test, so this
//! module provides the smallest `StorageEngine` that answers reads honestly and
//! rejects writes: a component that tried to mutate during a read-only unit test
//! would fail loudly instead of passing by accident.

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeIter, EdgeRecord, IndexHandle, IndexKey, StorageEngine, VertexFilter,
    VertexIter, VertexRecord,
};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext};
use metagdb_core::{Error, Result};

/// A fixed vertex/edge list with no versions and no writes.
pub struct MemoryStorage {
    vertices: Vec<VertexRecord>,
    edges: Vec<EdgeRecord>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a vertex whose only property is `key = value`.
    pub fn vertex(mut self, id: u64, labels: &[u32], key: PropertyKeyId, value: i64) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert(key, PropertyValue::Int(value));
        self.vertices.push(VertexRecord {
            id: VertexId(id),
            labels: labels.iter().map(|label| LabelId(*label)).collect(),
            properties,
        });
        self
    }

    /// Add a vertex with no properties.
    pub fn bare_vertex(mut self, id: u64) -> Self {
        self.vertices.push(VertexRecord {
            id: VertexId(id),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        });
        self
    }

    pub fn edge(mut self, id: u64, src: u64, dst: u64, etype: u32) -> Self {
        self.edges.push(EdgeRecord {
            id: EdgeId(id),
            src: VertexId(src),
            dst: VertexId(dst),
            etype: EdgeTypeId(etype),
            properties: PropertyMap::new(),
        });
        self
    }

    /// Attach an integer property to the most recently added edge.
    pub fn edge_property(mut self, key: PropertyKeyId, value: i64) -> Self {
        let edge = self
            .edges
            .last_mut()
            .expect("edge_property requires a previously added edge");
        edge.properties.insert(key, PropertyValue::Int(value));
        self
    }

    pub fn context() -> TxnContext {
        TxnContext {
            txn_id: 1,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: true,
            start_ts: 1,
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

fn read_only() -> Error {
    Error::Unsupported("the unit-test storage double is read-only".into())
}

impl StorageEngine for MemoryStorage {
    fn get_vertex(&self, vid: VertexId, _ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        Ok(self.vertices.iter().find(|row| row.id == vid).cloned())
    }

    fn scan_vertices(&self, filter: VertexFilter, _ctx: &TxnContext) -> Result<VertexIter> {
        let rows: Vec<VertexRecord> = self
            .vertices
            .iter()
            .filter(|row| match &filter.labels {
                Some(labels) => row.labels.iter().any(|label| labels.contains(label)),
                None => true,
            })
            .cloned()
            .collect();
        Ok(Box::new(rows.into_iter()))
    }

    fn get_edge(&self, eid: EdgeId, _ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        Ok(self.edges.iter().find(|row| row.id == eid).cloned())
    }

    fn out_edges(&self, vid: VertexId, filter: EdgeFilter, _ctx: &TxnContext) -> Result<EdgeIter> {
        let rows: Vec<EdgeRecord> = self
            .edges
            .iter()
            .filter(|row| row.src == vid && matches_type(&filter, row))
            .cloned()
            .collect();
        Ok(Box::new(rows.into_iter()))
    }

    fn in_edges(&self, vid: VertexId, filter: EdgeFilter, _ctx: &TxnContext) -> Result<EdgeIter> {
        let rows: Vec<EdgeRecord> = self
            .edges
            .iter()
            .filter(|row| row.dst == vid && matches_type(&filter, row))
            .cloned()
            .collect();
        Ok(Box::new(rows.into_iter()))
    }

    fn index_lookup(
        &self,
        _idx: IndexHandle,
        _key: &IndexKey,
        _ctx: &TxnContext,
    ) -> Result<VertexIter> {
        Err(Error::Unsupported(
            "the unit-test storage double maintains no secondary index".into(),
        ))
    }

    fn insert_vertex(&self, _record: VertexRecord, _ctx: &TxnContext) -> Result<VertexId> {
        Err(read_only())
    }

    fn insert_edge(&self, _record: EdgeRecord, _ctx: &TxnContext) -> Result<EdgeId> {
        Err(read_only())
    }

    fn update_vertex(&self, _record: VertexRecord, _ctx: &TxnContext) -> Result<()> {
        Err(read_only())
    }

    fn delete_vertex(&self, _vid: VertexId, _ctx: &TxnContext) -> Result<()> {
        Err(read_only())
    }

    fn update_edge(&self, _record: EdgeRecord, _ctx: &TxnContext) -> Result<()> {
        Err(read_only())
    }

    fn delete_edge(&self, _eid: EdgeId, _ctx: &TxnContext) -> Result<()> {
        Err(read_only())
    }
}

fn matches_type(filter: &EdgeFilter, edge: &EdgeRecord) -> bool {
    match &filter.types {
        Some(types) => types.contains(&edge.etype),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_double_answers_reads_and_refuses_writes() {
        let storage = MemoryStorage::new().vertex(1, &[1], PropertyKeyId(1), 5).edge(1, 1, 1, 1);
        let ctx = MemoryStorage::context();
        assert!(storage.get_vertex(VertexId(1), &ctx).unwrap().is_some());
        assert_eq!(storage.get_edge(EdgeId(1), &ctx).unwrap().unwrap().src, VertexId(1));
        assert!(storage
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: Vec::new(),
                    properties: PropertyMap::new(),
                },
                &ctx,
            )
            .is_err());
    }
}
