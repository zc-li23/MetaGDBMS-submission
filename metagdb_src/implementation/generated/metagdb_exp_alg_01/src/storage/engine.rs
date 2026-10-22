//! `StorageEngine` implementation over the MVCC version store.
//!
//! The design vector's storage layout is "persistent per-vertex adjacency
//! records; delta updates with periodic merge and compaction; rebuildable CSR
//! analytical projection; record versions with begin/end visibility timestamps;
//! write-set conflict detection", with the indexes `out adjacency`,
//! `in adjacency`, `edge id`, and `vertex id`.
//!
//! This type is the thin engine surface over [`crate::db::Db`]: every accessor
//! forwards to the version store, which already owns visibility, the two
//! adjacency indexes, the ID indexes, and the log. Keeping the engine thin means
//! the semantics the system benchmark observes are the ones the transaction
//! phase tested, not a second implementation.

use metagdb_core::common::ids::{EdgeId, VertexId};
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeIter, EdgeRecord, IndexHandle, IndexKey, StorageEngine, VertexFilter,
    VertexIter, VertexRecord,
};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::common::property::PropertyValue;
use metagdb_core::{Error, Result};

use crate::db::Db;

/// Handle of the vertex-ID index: the key is the integer vertex ID.
pub const VERTEX_ID_INDEX: IndexHandle = IndexHandle(0);

/// Storage engine backed by the shared MVCC version store.
#[derive(Clone)]
pub struct MvccStorage {
    db: Db,
}

impl MvccStorage {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    /// The version store this engine reads and writes.
    pub fn db(&self) -> &Db {
        &self.db
    }
}

impl StorageEngine for MvccStorage {
    fn get_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        self.db.read_vertex(ctx, vid)
    }

    fn scan_vertices(&self, filter: VertexFilter, ctx: &TxnContext) -> Result<VertexIter> {
        let rows = self.db.scan_vertices(ctx, filter.labels.as_deref())?;
        Ok(Box::new(rows.into_iter()))
    }

    fn get_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        self.db.read_edge(ctx, eid)
    }

    fn out_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let rows = self.db.out_edges(ctx, vid, filter.types.as_deref())?;
        Ok(Box::new(rows.into_iter()))
    }

    fn in_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let rows = self.db.in_edges(ctx, vid, filter.types.as_deref())?;
        Ok(Box::new(rows.into_iter()))
    }

    fn index_lookup(
        &self,
        idx: IndexHandle,
        key: &IndexKey,
        ctx: &TxnContext,
    ) -> Result<VertexIter> {
        if idx != VERTEX_ID_INDEX {
            return Err(Error::Unsupported(format!(
                "index handle {} is not maintained by this storage engine",
                idx.0
            )));
        }
        let Some(id) = key.as_i64() else {
            return Err(Error::InvalidQuery(
                "the vertex-ID index requires an integer key".into(),
            ));
        };
        if id <= 0 {
            return Ok(Box::new(std::iter::empty()));
        }
        let found = self.db.read_vertex(ctx, VertexId(id as u64))?;
        Ok(Box::new(found.into_iter()))
    }

    fn insert_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<VertexId> {
        self.db.insert_vertex(ctx, record)
    }

    fn insert_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<EdgeId> {
        self.db.insert_edge(ctx, record)
    }

    fn update_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<()> {
        self.db.update_vertex(ctx, record)
    }

    fn delete_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<()> {
        self.db.delete_vertex(ctx, vid)
    }

    fn update_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<()> {
        self.db.update_edge(ctx, record)
    }

    fn delete_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<()> {
        self.db.delete_edge(ctx, eid)
    }
}

/// Convenience: resolve the first visible vertex matching a property value.
///
/// The design vector declares no secondary property index, so this is a scan;
/// it exists so the planner has one honest path for equality predicates rather
/// than a fabricated index handle.
pub fn lookup_by_property(
    storage: &dyn StorageEngine,
    ctx: &TxnContext,
    key: metagdb_core::common::ids::PropertyKeyId,
    value: &PropertyValue,
) -> Result<Vec<VertexRecord>> {
    let rows = storage.scan_vertices(VertexFilter::default(), ctx)?;
    Ok(rows
        .filter(|record| record.properties.get(&key) == Some(value))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
    use metagdb_core::common::property::PropertyMap;
    use metagdb_core::l3_txn::IsolationLevel;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch_dir(tag: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "metagdb-exp-alg-01-storage-{tag}-{}-{nanos}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn vertex(id: u64, value: i64) -> VertexRecord {
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Int(value));
        VertexRecord {
            id: VertexId(id),
            labels: vec![LabelId(1)],
            properties,
        }
    }

    fn edge(src: VertexId, dst: VertexId) -> EdgeRecord {
        EdgeRecord {
            id: EdgeId(0),
            src,
            dst,
            etype: EdgeTypeId(1),
            properties: PropertyMap::new(),
        }
    }

    fn value_of(record: &VertexRecord) -> Option<i64> {
        record
            .properties
            .get(&PropertyKeyId(1))
            .and_then(PropertyValue::as_i64)
    }

    #[test]
    fn crud_round_trips_through_the_engine_surface() {
        let db = Db::in_memory();
        let storage = MvccStorage::new(db.clone());
        let ctx = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let a = storage.insert_vertex(vertex(0, 1), &ctx).unwrap();
        let b = storage.insert_vertex(vertex(0, 2), &ctx).unwrap();
        let e = storage.insert_edge(edge(a, b), &ctx).unwrap();
        db.commit(&ctx).unwrap();

        let read = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        assert_eq!(
            storage.get_vertex(a, &read).unwrap().as_ref().and_then(value_of),
            Some(1)
        );
        assert_eq!(storage.get_edge(e, &read).unwrap().unwrap().dst, b);
        assert_eq!(storage.out_edges(a, EdgeFilter::default(), &read).unwrap().count(), 1);
        assert_eq!(storage.in_edges(b, EdgeFilter::default(), &read).unwrap().count(), 1);

        let mut updated = storage.get_vertex(a, &read).unwrap().unwrap();
        updated.properties.insert(PropertyKeyId(1), PropertyValue::Int(9));
        let write = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        storage.update_vertex(updated, &write).unwrap();
        storage.delete_edge(e, &write).unwrap();
        storage.delete_vertex(b, &write).unwrap();
        db.commit(&write).unwrap();

        let after = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        assert_eq!(
            storage.get_vertex(a, &after).unwrap().as_ref().and_then(value_of),
            Some(9)
        );
        assert!(storage.get_edge(e, &after).unwrap().is_none());
        assert!(storage.get_vertex(b, &after).unwrap().is_none());
        assert_eq!(storage.out_edges(a, EdgeFilter::default(), &after).unwrap().count(), 0);
        db.commit(&after).unwrap();
    }

    #[test]
    fn scans_filter_by_label_and_survive_a_reopen() {
        let dir = scratch_dir("scan");
        {
            let db = Db::open(&dir).unwrap();
            let storage = MvccStorage::new(db.clone());
            let ctx = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
            for value in 0..5 {
                let mut record = vertex(0, value);
                record.labels = vec![LabelId(if value % 2 == 0 { 1 } else { 2 })];
                storage.insert_vertex(record, &ctx).unwrap();
            }
            db.commit(&ctx).unwrap();
        }
        let db = Db::open(&dir).unwrap();
        let storage = MvccStorage::new(db.clone());
        let ctx = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        let all = storage
            .scan_vertices(VertexFilter::default(), &ctx)
            .unwrap()
            .count();
        assert_eq!(all, 5);
        let evens = storage
            .scan_vertices(
                VertexFilter {
                    labels: Some(vec![LabelId(1)]),
                },
                &ctx,
            )
            .unwrap()
            .count();
        assert_eq!(evens, 3);
        db.commit(&ctx).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_vertex_id_index_resolves_integer_keys_and_rejects_others() {
        let db = Db::in_memory();
        let storage = MvccStorage::new(db.clone());
        let ctx = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let id = storage.insert_vertex(vertex(0, 3), &ctx).unwrap();
        db.commit(&ctx).unwrap();

        let read = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        let found = storage
            .index_lookup(VERTEX_ID_INDEX, &PropertyValue::Int(id.0 as i64), &read)
            .unwrap()
            .count();
        assert_eq!(found, 1);
        assert!(storage
            .index_lookup(VERTEX_ID_INDEX, &PropertyValue::Text("x".into()), &read)
            .is_err());
        assert!(storage
            .index_lookup(IndexHandle(9), &PropertyValue::Int(1), &read)
            .is_err());
        db.commit(&read).unwrap();
    }

    #[test]
    fn updates_and_deletes_maintain_both_adjacency_indexes() {
        let db = Db::in_memory();
        let storage = MvccStorage::new(db.clone());
        let ctx = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let a = storage.insert_vertex(vertex(0, 1), &ctx).unwrap();
        let b = storage.insert_vertex(vertex(0, 2), &ctx).unwrap();
        let c = storage.insert_vertex(vertex(0, 3), &ctx).unwrap();
        let ab = storage.insert_edge(edge(a, b), &ctx).unwrap();
        let ac = storage.insert_edge(edge(a, c), &ctx).unwrap();
        db.commit(&ctx).unwrap();

        let read = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        let out: Vec<_> = storage
            .out_edges(a, EdgeFilter::default(), &read)
            .unwrap()
            .map(|e| e.id)
            .collect();
        assert_eq!(out, vec![ab, ac]);
        db.commit(&read).unwrap();

        let write = db.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        storage.delete_edge(ab, &write).unwrap();
        db.commit(&write).unwrap();

        let read = db.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        assert_eq!(
            storage.out_edges(a, EdgeFilter::default(), &read).unwrap().count(),
            1
        );
        assert_eq!(storage.in_edges(b, EdgeFilter::default(), &read).unwrap().count(), 0);
        db.commit(&read).unwrap();
    }
}
