/**
 * @brief: Default transaction manager and visibility determination implementation.
 *
 * Provides MVCC-based Snapshot Isolation transaction semantics:
 * - begin allocates start_ts and registers active transactions
 * - commit allocates commit_ts and coordinates with the storage layer to commit (in-place update of MVCC header)
 * - abort coordinates with the storage layer to clean up the write set
 */

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext, TxnManager, TxnPolicy, Visibility};
use metagdb_core::Result;

use crate::storage::DefaultStorage;

/**
 * @brief: Default transaction manager, responsible for generating transaction IDs and timestamps,
 *         and coordinating with the storage layer to complete commit/abort.
 *
 * Injects Arc<DefaultStorage> via constructor, enabling commit/abort to
 * synchronously update on-disk MVCC headers and flush.
 */
pub struct DefaultTxnManager {
    next_txn_id: AtomicU64,
    next_ts: AtomicU64,
    default_policy: TxnPolicy,
    /// Active transaction set, used to prevent re-entrant commit/abort and state tracking.
    active_txns: Mutex<HashSet<u64>>,
    /// Associated storage engine instance, used to update MVCC headers during commit/abort.
    storage: Option<Arc<DefaultStorage>>,
}

impl DefaultTxnManager {
    pub const DEFAULT_POLICY: TxnPolicy =
        TxnPolicy::new(IsolationLevel::SnapshotIsolation, false);

    /**
     * @brief: Create a transaction manager not linked to storage (only for backward compatibility or test stubs).
     */
    pub fn new() -> Self {
        Self::with_default_policy(Self::DEFAULT_POLICY)
    }

    /**
     * @brief: Create a transaction manager linked to the specified storage instance.
     * @param storage: Associated storage engine; commit/abort will synchronously update on-disk MVCC headers.
     */
    pub fn with_storage(storage: Arc<DefaultStorage>) -> Self {
        // Recover counters from persistent storage to ensure monotonic increase across restarts
        let start_ts = storage.current_ts().max(1);
        let start_txn = storage.current_txn_id().max(1);
        Self {
            next_txn_id: AtomicU64::new(start_txn),
            next_ts: AtomicU64::new(start_ts),
            default_policy: Self::DEFAULT_POLICY,
            active_txns: Mutex::new(HashSet::new()),
            storage: Some(storage),
        }
    }

    /**
     * @brief: Construct transaction manager with a specific default policy.
     * @param default_policy: The specified default transaction policy.
     */
    pub fn with_default_policy(default_policy: TxnPolicy) -> Self {
        Self {
            next_txn_id: AtomicU64::new(1),
            next_ts: AtomicU64::new(1),
            default_policy,
            active_txns: Mutex::new(HashSet::new()),
            storage: None,
        }
    }
}

impl Default for DefaultTxnManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TxnManager for DefaultTxnManager {
    /**
     * @brief: Begin a transaction, allocate txn_id and start_ts, register in active transaction table.
     * @param iso: Isolation level.
     * @param read_only: Whether read-only.
     * @return: Returns the constructed TxnContext.
     */
    fn begin(&self, iso: IsolationLevel, read_only: bool) -> Result<TxnContext> {
        // Prefer persistent storage counters, fall back to local counters when no storage
        let txn_id = match &self.storage {
            Some(s) => s.alloc_txn_id(),
            None => self.next_txn_id.fetch_add(1, Ordering::SeqCst),
        };
        let start_ts = match &self.storage {
            Some(s) => s.alloc_ts(),
            None => self.next_ts.fetch_add(1, Ordering::SeqCst),
        };

        let mut active = self.active_txns.lock().map_err(|_| {
            metagdb_core::Error::Storage("Active transaction table lock poisoned".into())
        })?;
        active.insert(txn_id);

        Ok(TxnContext {
            txn_id,
            isolation: iso,
            read_only,
            start_ts,
        })
    }

    /**
     * @brief: Commit transaction: allocate commit_ts, coordinate with storage layer to update MVCC headers and flush.
     * @param ctx: Transaction context to be committed.
     */
    fn commit(&self, ctx: TxnContext) -> Result<()> {
        // Deregister from active transaction table
        {
            let mut active = self.active_txns.lock().map_err(|_| {
                metagdb_core::Error::Storage("Active transaction table lock poisoned".into())
            })?;
            if !active.remove(&ctx.txn_id) {
                return Err(metagdb_core::Error::Storage(format!(
                    "Transaction {} is not in the active transaction table, may have already been committed or rolled back",
                    ctx.txn_id
                )));
            }
        }

        let commit_ts = match &self.storage {
            Some(s) => s.alloc_ts(),
            None => self.next_ts.fetch_add(1, Ordering::SeqCst),
        };

        // Coordinate with storage layer: in-place update MVCC headers and flush
        if let Some(ref storage) = self.storage {
            storage.commit_txn(ctx.txn_id, commit_ts)?;
        }

        Ok(())
    }

    /**
     * @brief: Abort transaction: clean up active transaction state, coordinate with storage layer to clean up write set.
     * @param ctx: Transaction context to be aborted.
     */
    fn abort(&self, ctx: TxnContext) -> Result<()> {
        // Deregister from active transaction table
        {
            let mut active = self.active_txns.lock().map_err(|_| {
                metagdb_core::Error::Storage("Active transaction table lock poisoned".into())
            })?;
            active.remove(&ctx.txn_id);
        }

        // Coordinate with storage layer: clean up write set
        if let Some(ref storage) = self.storage {
            storage.abort_txn(ctx.txn_id)?;
        }

        Ok(())
    }

    /**
     * @brief: Get the default transaction policy.
     */
    fn default_policy(&self) -> TxnPolicy {
        self.default_policy
    }
}

/**
 * @brief: Visibility determination based on MVCC header.
 */
pub struct DefaultVisibility;

impl Visibility for DefaultVisibility {
    /**
     * @brief: Vertex visibility determination.
     *
     * Since VertexRecord / EdgeRecord do not carry MVCC information,
     * the actual determination is done in the storage layer read path by parsing MvccHeader from raw bytes.
     * This is kept as always-visible here, serving as a framework compatibility layer.
     */
    fn is_vertex_visible(&self, _v: &VertexRecord, _ctx: &TxnContext) -> bool {
        true
    }
    /**
     * @brief: Edge visibility determination. Default implementation always returns true.
     */
    fn is_edge_visible(&self, _e: &EdgeRecord, _ctx: &TxnContext) -> bool {
        true
    }
}
