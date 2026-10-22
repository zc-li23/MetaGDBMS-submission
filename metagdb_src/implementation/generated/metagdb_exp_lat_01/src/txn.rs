//! MVCC transaction manager.
//!
//! Timestamps are allocated from the storage engine so that they survive a
//! reopen (the `next_ts`/`next_txn` counters are part of the snapshot). Each
//! transaction receives an increasing `start_ts`; commits receive a fresh
//! timestamp that ends the previous version and begins the new one. Write
//! conflicts are detected by the storage write-lock set when the mutation is
//! applied, so a loser fails before commit.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext, TxnManager, TxnPolicy, Visibility};
use metagdb_core::Result;

use crate::storage::GraphStore;

pub const DEFAULT_ISOLATION: IsolationLevel = IsolationLevel::SnapshotIsolation;

/// Snapshot-isolation transaction manager over a persistent store.
pub struct MvccTxnManager {
    storage: Arc<GraphStore>,
    active: Mutex<HashSet<u64>>,
    policy: TxnPolicy,
}

impl MvccTxnManager {
    /// Build a manager over the given store using the design's snapshot policy.
    pub fn with_storage(storage: Arc<GraphStore>) -> Self {
        Self {
            storage,
            active: Mutex::new(HashSet::new()),
            policy: TxnPolicy::new(DEFAULT_ISOLATION, false),
        }
    }

    fn deactivate(&self, txn_id: u64) -> bool {
        self.active.lock().unwrap().remove(&txn_id)
    }
}

impl TxnManager for MvccTxnManager {
    fn begin(&self, iso: IsolationLevel, read_only: bool) -> Result<TxnContext> {
        let txn_id = self.storage.alloc_txn_id();
        let start_ts = self.storage.alloc_ts();
        self.active.lock().unwrap().insert(txn_id);
        Ok(TxnContext {
            txn_id,
            isolation: iso,
            read_only,
            start_ts,
        })
    }

    fn commit(&self, ctx: TxnContext) -> Result<()> {
        if !self.deactivate(ctx.txn_id) {
            return Err(metagdb_core::Error::Transaction(format!(
                "transaction {} is not active (double commit or unknown id)",
                ctx.txn_id
            )));
        }
        let commit_ts = self.storage.alloc_ts();
        self.storage.commit_txn(ctx.txn_id, commit_ts)
    }

    fn abort(&self, ctx: TxnContext) -> Result<()> {
        self.deactivate(ctx.txn_id);
        self.storage.abort_txn(ctx.txn_id)
    }

    fn default_policy(&self) -> TxnPolicy {
        self.policy
    }
}

/// Visibility shim required by the framework contract.
///
/// Records returned by [`crate::storage::GraphStore`] are already filtered by
/// the MVCC visibility rule (which needs the raw version header, not just the
/// record), so the trait-level check is a no-op.
pub struct SnapshotVisibility;

impl Visibility for SnapshotVisibility {
    fn is_vertex_visible(&self, _v: &VertexRecord, _ctx: &TxnContext) -> bool {
        true
    }

    fn is_edge_visible(&self, _e: &EdgeRecord, _ctx: &TxnContext) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{LabelId, PropertyKeyId, VertexId};
    use metagdb_core::common::property::{PropertyMap, PropertyValue};
    use metagdb_core::l2_storage::StorageEngine;

    #[test]
    fn double_commit_is_rejected() {
        let dir = std::env::temp_dir().join(format!("mgdb_txn_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let schema = Arc::new(crate::schema::DynamicSchema::schemaless());
        let storage = Arc::new(GraphStore::with_base_dir(schema, dir.clone()).unwrap());
        let manager = MvccTxnManager::with_storage(storage.clone());

        let tx = manager.begin(DEFAULT_ISOLATION, false).unwrap();
        let clone = tx.clone();
        manager.commit(tx).unwrap();
        assert!(manager.commit(clone).is_err());
        drop(storage);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_only_rejects_mutation() {
        let dir = std::env::temp_dir().join(format!("mgdb_txn_ro_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let schema = Arc::new(crate::schema::DynamicSchema::schemaless());
        let storage = Arc::new(GraphStore::with_base_dir(schema, dir.clone()).unwrap());
        let manager = MvccTxnManager::with_storage(storage.clone());

        let tx = manager.begin(DEFAULT_ISOLATION, true).unwrap();
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Int(1));
        let result = storage.insert_vertex(
            VertexRecord {
                id: VertexId(0),
                labels: vec![LabelId(1)],
                properties,
            },
            &tx,
        );
        assert!(result.is_err());
        manager.abort(tx).unwrap();
        drop(storage);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
