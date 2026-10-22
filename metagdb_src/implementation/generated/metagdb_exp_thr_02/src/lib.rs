//! Candidate `metagdb_exp_thr_02`: a Cypher graph system with snapshot-isolated
//! MVCC over an append-only physical record segment and a logical-ID
//! redirection index.
//!
//! The crate exports the six names required by
//! `docs/implementation_contract.md`: [`MetagdbImpl`], [`MetagdbParser`],
//! [`MetagdbPlanner`], [`MetagdbOptimizer`], [`MetagdbExecutor`] and
//! [`CsrProjection`].

pub mod compute;
pub mod schema;
pub mod storage;
pub mod test;
pub mod txn;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::olap::AnalyticalAlgorithm;

pub use compute::algorithms::{BfsAlgorithm, PageRankAlgorithm, WccAlgorithm};
pub use compute::executor::MetagdbExecutor;
pub use compute::optimizer::MetagdbOptimizer;
pub use compute::parser::MetagdbParser;
pub use compute::planner::MetagdbPlanner;
pub use compute::projection::CsrProjection;
pub use schema::MetaSchema;
pub use storage::PersistentStorage;
pub use txn::MvccTxnManager;

static INSTANCE: AtomicU64 = AtomicU64::new(1);

/// Implementation component package wired into the framework runtime.
pub struct MetagdbImpl {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
}

impl MetagdbImpl {
    fn from_storage(schema: Arc<MetaSchema>, storage: Arc<PersistentStorage>) -> Self {
        let txn_manager: Arc<dyn TxnManager> = Arc::new(MvccTxnManager::new(storage.clone()));
        Self {
            schema,
            storage: storage as Arc<dyn StorageEngine>,
            txn_manager,
            algorithms: compute::algorithms::required_algorithms(),
        }
    }

    /// A fresh database under a unique temporary directory.
    pub fn new() -> Self {
        let id = INSTANCE.fetch_add(1, Ordering::Relaxed);
        let base_dir = std::env::temp_dir().join(format!(
            "metagdb_exp_thr_02_{}_{id}",
            std::process::id()
        ));
        Self::with_base_dir(base_dir)
    }

    /// Open (or create) a persistent database rooted at `base_dir`.
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        let schema = Arc::new(MetaSchema::demo_schema());
        let storage = Arc::new(
            PersistentStorage::open(schema.clone(), base_dir)
                .expect("candidate storage must open"),
        );
        Self::from_storage(schema, storage)
    }

    /// Bulk import a `.graph` file into a fresh database at `base_dir`.
    pub fn build_from_graph_file(
        graph_path: PathBuf,
        base_dir: PathBuf,
    ) -> metagdb_core::Result<Self> {
        let schema = Arc::new(MetaSchema::generic_schema());
        let storage = Arc::new(PersistentStorage::bulk_load(
            schema.clone(),
            base_dir,
            &graph_path,
        )?);
        Ok(Self::from_storage(schema, storage))
    }

    /// Zero-argument self-checking demo exercised by `cargo run -- --demo`.
    pub fn demo_test() -> metagdb_core::Result<()> {
        test::demo_test()
    }
}

impl Default for MetagdbImpl {
    fn default() -> Self {
        Self::new()
    }
}
