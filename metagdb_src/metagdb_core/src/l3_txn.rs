/**
 * @brief: L3 transaction processing layer contract, defining transaction lifecycle
 *         management, visibility determination, and context specifications.
 */

use crate::l2_storage::{EdgeRecord, VertexRecord};
use crate::Result;

/**
 * @brief: Isolation level enumeration, defining the degree of mutual interference
 *         between transactions.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    SnapshotIsolation,
    Serializable,
}

/**
 * @brief: Transaction opening policy, including isolation level and read/write mode.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxnPolicy {
    pub isolation: IsolationLevel,
    pub read_only: bool,
}

impl TxnPolicy {
    /**
     * @brief: Create a new transaction policy.
     * @param isolation: The required isolation level.
     * @param read_only: Whether the transaction is read-only.
     * @return: A TxnPolicy instance.
     */
    pub const fn new(isolation: IsolationLevel, read_only: bool) -> Self {
        Self {
            isolation,
            read_only,
        }
    }
}

/**
 * @brief: Transaction context, carrying runtime information such as transaction
 *         identity, version/timestamp, etc.
 */
#[derive(Debug, Clone)]
pub struct TxnContext {
    pub txn_id: u64,
    pub isolation: IsolationLevel,
    pub read_only: bool,
    pub start_ts: u64,
}

/**
 * @brief: Transaction manager interface, providing the ability to begin, commit,
 *         and abort transactions.
 */
pub trait TxnManager: Send + Sync {
    /**
     * @brief: Begin a new transaction.
     * @param iso: The specified isolation level.
     * @param read_only: Whether the transaction is read-only.
     * @return: A transaction context TxnContext on success, or an Error on failure.
     */
    fn begin(&self, iso: IsolationLevel, read_only: bool) -> Result<TxnContext>;

    /**
     * @brief: Commit the specified transaction, persisting its changes.
     * @param ctx: The transaction context to commit.
     * @return: Ok(()) on success, or an Error on commit conflict or I/O failure.
     */
    fn commit(&self, ctx: TxnContext) -> Result<()>;

    /**
     * @brief: Roll back / abort the specified transaction.
     * @param ctx: The transaction context to abort.
     * @return: Ok(()) on success.
     */
    fn abort(&self, ctx: TxnContext) -> Result<()>;

    /**
     * @brief: Provide the default transaction policy for the current implementation.
     * @return: A TxnPolicy object.
     */
    fn default_policy(&self) -> TxnPolicy;
}

/**
 * @brief: Visibility determination interface, used to decide whether a raw record
 *         is visible to the current transaction after reading.
 */
pub trait Visibility: Send + Sync {
    /**
     * @brief: Determine whether a specific vertex is visible in the current transaction.
     * @param v: The vertex record to check.
     * @param ctx: The current transaction context.
     * @return: true if visible, otherwise false.
     */
    fn is_vertex_visible(&self, v: &VertexRecord, ctx: &TxnContext) -> bool;

    /**
     * @brief: Determine whether a specific edge is visible in the current transaction.
     * @param e: The edge record to check.
     * @param ctx: The current transaction context.
     * @return: true if visible, otherwise false.
     */
    fn is_edge_visible(&self, e: &EdgeRecord, ctx: &TxnContext) -> bool;
}
