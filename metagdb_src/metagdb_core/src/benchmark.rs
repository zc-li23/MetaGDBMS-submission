//! Fixed system-benchmark adapter shared by every generated implementation.
//!
//! The benchmark owns the tests and expected outcomes.  Implementations only
//! provide their normal components and a projection factory; this prevents a
//! candidate from redefining the oracle used to judge it.

use std::sync::Arc;

use crate::l1_model::GraphSchema;
use crate::l2_storage::StorageEngine;
use crate::l3_txn::{TxnContext, TxnManager};
use crate::l4_compute::algebra::ProjectionSpec;
use crate::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use crate::Result;

/// Candidate-independent access to the components exercised by the fixed
/// system benchmark.
pub struct BenchmarkComponents {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
    pub projection_factory: Arc<ProjectionFactory>,
}

/// Standardized bridge from storage records to the graph projection consumed
/// by analytical algorithms.
pub type ProjectionFactory = dyn Fn(&dyn StorageEngine, &TxnContext, &ProjectionSpec) -> Result<Box<dyn GraphProjection>>
    + Send
    + Sync;
