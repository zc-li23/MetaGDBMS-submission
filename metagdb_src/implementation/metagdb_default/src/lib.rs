/**
 * @brief: metagdb_default module, providing a default minimal reference graph system implementation.
 *
 * This crate exports five core types according to the unified "implementation crate assembly contract":
 *   MetagdbImpl / MetagdbParser / MetagdbPlanner / MetagdbOptimizer / MetagdbExecutor
 * The framework (metagdb crate) assembly layer only depends on these unified names,
 * allowing scripts to seamlessly switch between different implementation crates.
 */

pub mod compute;
pub mod schema;
pub mod storage;
pub mod test;
pub mod txn;

use std::path::PathBuf;
use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::olap::AnalyticalAlgorithm;

// Assembly slots: uniformly export Metagdb* names (excluding Frontend, provided by the framework layer).
pub use compute::executor::MetagdbExecutor;
pub use compute::optimizer::MetagdbOptimizer;
pub use compute::parser::MetagdbParser;
pub use compute::planner::MetagdbPlanner;

// Internal detail types: keep original names for internal organization within this crate.
pub use compute::algorithms::BfsAlgorithm;
pub use compute::projection::CsrProjection;
pub use schema::DefaultSchema;
pub use storage::DefaultStorage;
pub use txn::{DefaultTxnManager, DefaultVisibility};

/**
 * @brief: Default implementation component package, aggregating specific default implementation classes for each layer.
 *
 * As the implementation entry point for metagdb_default, named MetagdbImpl by unified convention.
 */
pub struct MetagdbImpl {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
}

impl MetagdbImpl {
    /**
     * @brief: Construct a new empty default implementation package, preconfigured with demo metadata model.
     * @return: Returns an initialized MetagdbImpl instance.
     */
    pub fn new() -> Self {
        let schema = Arc::new(DefaultSchema::demo_schema());
        let storage: Arc<DefaultStorage> = Arc::new(DefaultStorage::new(schema.clone()));
        let txn_manager: Arc<dyn TxnManager> =
            Arc::new(DefaultTxnManager::with_storage(storage.clone()));
        let algorithms: Vec<Arc<dyn AnalyticalAlgorithm>> = vec![Arc::new(BfsAlgorithm)];

        Self {
            schema,
            storage: storage as Arc<dyn StorageEngine>,
            txn_manager,
            algorithms,
        }
    }

    /**
     * @brief: End-to-end Demo test entry point (zero arguments, internally self-contained).
     *
     * Delegates to the test module's demo_test() function, executing the full test pipeline:
     *   Create Runtime → Insert demo data → Execute test queries one by one → Verify output → Print results.
     *
     * @return: Returns Ok(()) on success, Error otherwise.
     */
    pub fn demo_test() -> metagdb_core::Result<()> {
        test::demo_test()
    }

    /**
     * @brief: Construct default implementation package using the specified directory as the data storage path.
     *
     * Unlike new(), this constructor allows the caller to specify the directory for persisting data.
     * If the directory already contains data files, they will be automatically opened and read;
     * otherwise a new empty database is created.
     *
     * @param base_dir: Data file storage directory (e.g., "target/data").
     * @return: Returns an initialized MetagdbImpl instance.
     */
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        let schema = Arc::new(DefaultSchema::demo_schema());
        let storage: Arc<DefaultStorage> =
            Arc::new(DefaultStorage::with_base_dir(schema.clone(), base_dir));
        let txn_manager: Arc<dyn TxnManager> =
            Arc::new(DefaultTxnManager::with_storage(storage.clone()));
        let algorithms: Vec<Arc<dyn AnalyticalAlgorithm>> = vec![Arc::new(BfsAlgorithm)];

        Self {
            schema,
            storage: storage as Arc<dyn StorageEngine>,
            txn_manager,
            algorithms,
        }
    }

    /**
     * @brief: Batch-build database from a .graph file.
     *
     * Accepts a .graph file path and data directory, parses the file and writes
     * to the database in an efficient batch manner (bypassing per-record WAL and MVCC overhead).
     *
     * @param graph_path: Path to the .graph file.
     * @param base_dir: Path to the database file storage directory.
     * @return: On success, returns a MetagdbImpl instance populated with data.
     */
    pub fn build_from_graph_file(
        graph_path: PathBuf,
        base_dir: PathBuf,
    ) -> metagdb_core::Result<Self> {
        // Validate that the .graph file exists
        if !graph_path.exists() {
            return Err(metagdb_core::Error::Storage(format!(
                "Graph file does not exist: {}",
                graph_path.display()
            )));
        }

        // Ensure parent directory exists
        if let Some(parent) = base_dir.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                metagdb_core::Error::Storage(format!("Failed to create parent directory: {}", e))
            })?;
        }

        let schema = Arc::new(DefaultSchema::generic_schema());
        let storage: Arc<DefaultStorage> = Arc::new(DefaultStorage::bulk_load_from_graph(
            schema.clone(),
            base_dir,
            &graph_path,
        )?);
        let txn_manager: Arc<dyn TxnManager> =
            Arc::new(DefaultTxnManager::with_storage(storage.clone()));
        let algorithms: Vec<Arc<dyn AnalyticalAlgorithm>> = vec![Arc::new(BfsAlgorithm)];

        Ok(Self {
            schema,
            storage: storage as Arc<dyn StorageEngine>,
            txn_manager,
            algorithms,
        })
    }
}

impl Default for MetagdbImpl {
    /**
     * @brief: Default constructor, equivalent to new().
     */
    fn default() -> Self {
        Self::new()
    }
}
