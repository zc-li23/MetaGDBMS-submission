//! MVCC transaction manager.
//!
//! The design vector selects `isolation_level = snapshot_isolation` with
//! `concurrency_control = mvcc`.  Each transaction captures the storage's
//! commit counter as its snapshot timestamp at `begin`; reads resolve the
//! newest physical record whose commit timestamp is not newer than that
//! snapshot, and writes are buffered privately until `commit`.  Commit
//! validates every written logical id against the same snapshot, so two
//! concurrent writers of the same element cannot both commit.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use metagdb_core::l3_txn::{IsolationLevel, TxnContext, TxnManager, TxnPolicy};
use metagdb_core::{Error, Result};

use crate::storage::PersistentStorage;

#[derive(Default)]
struct ManagerState {
    next_txn_id: u64,
    active: HashSet<u64>,
}

/// Transaction manager backed by the append-only storage engine.
pub struct MvccTxnManager {
    storage: Arc<PersistentStorage>,
    state: Mutex<ManagerState>,
}

impl MvccTxnManager {
    /// Create a manager over `storage`.
    pub fn new(storage: Arc<PersistentStorage>) -> Self {
        Self {
            storage,
            state: Mutex::new(ManagerState {
                next_txn_id: 1,
                active: HashSet::new(),
            }),
        }
    }

    /// Number of transactions that have begun and not yet completed.
    pub fn active_count(&self) -> usize {
        self.state.lock().unwrap().active.len()
    }

    fn forget(&self, txn_id: u64) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        if state.active.remove(&txn_id) {
            Ok(())
        } else {
            Err(Error::Transaction(format!(
                "transaction {txn_id} is unknown or already completed"
            )))
        }
    }
}

impl TxnManager for MvccTxnManager {
    fn begin(&self, iso: IsolationLevel, read_only: bool) -> Result<TxnContext> {
        let start_ts = self.storage.commit_timestamp();
        let txn_id = {
            let mut state = self.state.lock().unwrap();
            let txn_id = state.next_txn_id;
            state.next_txn_id += 1;
            state.active.insert(txn_id);
            txn_id
        };
        self.storage.begin_txn(txn_id, start_ts, read_only);
        Ok(TxnContext {
            txn_id,
            isolation: iso,
            read_only,
            start_ts,
        })
    }

    fn commit(&self, ctx: TxnContext) -> Result<()> {
        {
            let state = self.state.lock().unwrap();
            if !state.active.contains(&ctx.txn_id) {
                return Err(Error::Transaction(format!(
                    "commit of transaction {} which is not active",
                    ctx.txn_id
                )));
            }
        }
        // Durability happens inside the storage before the transaction is
        // forgotten, so a failed commit cannot be observed as committed.
        self.storage.commit_txn(ctx.txn_id)?;
        self.forget(ctx.txn_id)?;
        Ok(())
    }

    fn abort(&self, ctx: TxnContext) -> Result<()> {
        self.forget(ctx.txn_id)?;
        self.storage.abort_txn(ctx.txn_id)
    }

    fn default_policy(&self) -> TxnPolicy {
        TxnPolicy::new(IsolationLevel::SnapshotIsolation, false)
    }
}

impl std::fmt::Debug for MvccTxnManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MvccTxnManager").finish_non_exhaustive()
    }
}
