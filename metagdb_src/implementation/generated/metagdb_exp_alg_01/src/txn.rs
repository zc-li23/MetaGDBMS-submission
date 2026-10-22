//! Transaction manager over the shared MVCC state.
//!
//! The frozen design vector selects `snapshot_isolation` with MVCC and
//! write-set conflict detection.  [`MetagdbTxnManager`] is the thin lifecycle
//! façade: it hands out snapshot timestamps, forwards commit and abort to the
//! version store, and reports the default policy that the framework's executor
//! uses to open the transaction behind every statement.
//!
//! Concurrency semantics live in [`crate::db`]; this type owns identity and
//! lifecycle so that a committed, aborted, or unknown transaction is rejected
//! precisely instead of silently succeeding.

use metagdb_core::l3_txn::{IsolationLevel, TxnContext, TxnManager, TxnPolicy};
use metagdb_core::Result;

use crate::db::Db;

/// Snapshot-isolation transaction manager backed by [`Db`].
#[derive(Clone)]
pub struct MetagdbTxnManager {
    db: Db,
}

impl MetagdbTxnManager {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    /// The shared version store the storage engine and algorithms read.
    pub fn db(&self) -> &Db {
        &self.db
    }
}

impl TxnManager for MetagdbTxnManager {
    fn begin(&self, iso: IsolationLevel, read_only: bool) -> Result<TxnContext> {
        self.db.begin(iso, read_only)
    }

    fn commit(&self, ctx: TxnContext) -> Result<()> {
        self.db.commit(&ctx)
    }

    fn abort(&self, ctx: TxnContext) -> Result<()> {
        self.db.abort(&ctx)
    }

    fn default_policy(&self) -> TxnPolicy {
        TxnPolicy::new(IsolationLevel::SnapshotIsolation, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{LabelId, PropertyKeyId, VertexId};
    use metagdb_core::common::property::PropertyValue;
    use metagdb_core::l2_storage::VertexRecord;

    use crate::db::property_map;

    fn vertex(id: u64, value: i64) -> VertexRecord {
        VertexRecord {
            id: VertexId(id),
            labels: vec![LabelId(1)],
            properties: property_map(&[(PropertyKeyId(1), PropertyValue::Int(value))]),
        }
    }

    fn value_of(record: &VertexRecord) -> Option<i64> {
        record
            .properties
            .get(&PropertyKeyId(1))
            .and_then(PropertyValue::as_i64)
    }

    #[test]
    fn default_policy_is_snapshot_isolation_with_writes() {
        let manager = MetagdbTxnManager::new(Db::in_memory());
        let policy = manager.default_policy();
        assert_eq!(policy.isolation, IsolationLevel::SnapshotIsolation);
        assert!(!policy.read_only);
    }

    #[test]
    fn a_snapshot_reader_retains_the_view_it_started_with() {
        let manager = MetagdbTxnManager::new(Db::in_memory());
        let seed = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let id = manager.db().insert_vertex(&seed, vertex(0, 1)).unwrap();
        manager.commit(seed).unwrap();

        let reader = manager.begin(IsolationLevel::SnapshotIsolation, true).unwrap();
        let writer = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        manager.db().update_vertex(&writer, vertex(id.0, 2)).unwrap();
        manager.commit(writer).unwrap();

        let observed = manager
            .db()
            .read_vertex(&reader, id)
            .unwrap()
            .as_ref()
            .and_then(value_of);
        assert_eq!(observed, Some(1));
        manager.commit(reader).unwrap();
    }

    #[test]
    fn two_writers_of_one_key_cannot_both_commit_through_the_manager() {
        let manager = MetagdbTxnManager::new(Db::in_memory());
        let seed = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let id = manager.db().insert_vertex(&seed, vertex(0, 0)).unwrap();
        manager.commit(seed).unwrap();

        let first = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        let second = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        manager.db().update_vertex(&first, vertex(id.0, 1)).unwrap();
        manager.db().update_vertex(&second, vertex(id.0, 2)).unwrap();
        assert!(manager.commit(first).is_ok());
        assert!(manager.commit(second).is_err());
    }

    #[test]
    fn lifecycle_errors_are_precise() {
        let manager = MetagdbTxnManager::new(Db::in_memory());
        let txn = manager.begin(IsolationLevel::SnapshotIsolation, false).unwrap();
        manager.commit(txn.clone()).unwrap();
        let error = manager.commit(txn.clone()).unwrap_err().to_string();
        assert!(error.contains("already finished"), "got {error}");
        let error = manager.abort(txn).unwrap_err().to_string();
        assert!(error.contains("already finished"), "got {error}");
    }
}
