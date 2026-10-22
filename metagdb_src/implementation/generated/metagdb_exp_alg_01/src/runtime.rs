//! The four-stage query pipeline, assembled in the same order as the framework
//! runtime.
//!
//! The framework's own `Runtime` is built by the generated assembly out of the
//! components this crate exports; the candidate cannot depend on `metagdb`, so
//! it wires the stages itself.  Keeping the wiring in one place means the demo,
//! the integration tests, and the entry points all exercise the identical
//! sequence the benchmark will drive:
//!
//! ```text
//! Frontend -> Parser -> Planner -> Optimizer -> Executor -> commit/abort
//! ```

use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::engine::{
    ExecContext, Executor, OptContext, Optimizer, Parser, Planner, ResultStream,
};
use metagdb_core::l4_compute::olap::{AlgorithmRegistry, AnalyticalAlgorithm};
use metagdb_core::Result;

use crate::compute::executor::MetagdbExecutor;
use crate::compute::optimizer::MetagdbOptimizer;
use crate::compute::planner::MetagdbPlanner;
use crate::language::MetagdbParser;

/// One assembled runtime: the four stages plus the components they read.
pub struct Pipeline {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Arc<AlgorithmRegistry>,
}

impl Pipeline {
    pub fn new(
        schema: Arc<dyn GraphSchema>,
        storage: Arc<dyn StorageEngine>,
        txn_manager: Arc<dyn TxnManager>,
        algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
    ) -> Self {
        let mut registry = AlgorithmRegistry::default();
        for algorithm in algorithms {
            registry
                .algorithms
                .insert(algorithm.name().to_string(), algorithm);
        }
        Self {
            schema,
            storage,
            txn_manager,
            algorithms: Arc::new(registry),
        }
    }

    /// Run one statement end to end inside one transaction.
    pub fn run(&self, source: &str) -> Result<ResultStream> {
        let ast = MetagdbParser.parse(source)?;
        let logical = MetagdbPlanner.plan(ast, self.schema.as_ref())?;
        let opt = OptContext {
            schema: self.schema.clone(),
            algorithms: self.algorithms.clone(),
        };
        let physical = MetagdbOptimizer.optimize(logical, &opt)?;

        // The transaction wraps execution exactly as the framework's executor
        // stage does: a successful statement commits, a failed one aborts.
        let policy = self.txn_manager.default_policy();
        let txn = self.txn_manager.begin(policy.isolation, policy.read_only)?;
        let exec = ExecContext {
            txn: txn.clone(),
            storage: self.storage.clone(),
            schema: self.schema.clone(),
            algorithms: self.algorithms.clone(),
        };
        match MetagdbExecutor.execute(&physical, &exec) {
            Ok(stream) => {
                self.txn_manager.commit(txn)?;
                Ok(stream)
            }
            Err(error) => {
                let _ = self.txn_manager.abort(txn);
                Err(error)
            }
        }
    }
}

/// Convenience wrapper for callers that hold components rather than a pipeline.
pub fn run_statement(
    source: &str,
    schema: Arc<dyn GraphSchema>,
    storage: Arc<dyn StorageEngine>,
    txn_manager: Arc<dyn TxnManager>,
    algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
) -> Result<ResultStream> {
    Pipeline::new(schema, storage, txn_manager, algorithms).run(source)
}
