/**
 * @brief: Query execution stage orchestration, responsible for transaction lifecycle management and
 *         operator execution context construction.
 */

use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::algebra::PhysicalPlan;
use metagdb_core::l4_compute::engine::{ExecContext, Executor, ResultStream};
use metagdb_core::l4_compute::olap::AlgorithmRegistry;
use metagdb_core::Result;

/**
 * @brief: Execute a query physical plan. Opens a transaction, drives the executor, and commits or
 *         rolls back the transaction based on the result.
 * @param executor: Physical operator executor.
 * @param pp: Physical plan to execute.
 * @param txn_manager: Transaction manager.
 * @param storage: Storage engine handle.
 * @param schema: Graph metadata handle.
 * @param algorithms: Registered algorithm library.
 * @return: Returns a ResultStream on success, or Error otherwise.
 */
pub fn run(
    executor: &dyn Executor,
    pp: &PhysicalPlan,
    txn_manager: &dyn TxnManager,
    storage: Arc<dyn StorageEngine>,
    schema: Arc<dyn GraphSchema>,
    algorithms: Arc<AlgorithmRegistry>,
) -> Result<ResultStream> {
    let policy = txn_manager.default_policy();
    let txn = txn_manager.begin(policy.isolation, policy.read_only)?;
    let exec_ctx = ExecContext {
        txn: txn.clone(),
        storage,
        schema,
        algorithms,
    };
    match executor.execute(pp, &exec_ctx) {
        Ok(r) => {
            txn_manager.commit(txn)?;
            Ok(r)
        }
        Err(e) => {
            let _ = txn_manager.abort(txn);
            Err(e)
        }
    }
}
