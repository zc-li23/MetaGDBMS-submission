//! `metagdb_exp_alg_01`: a Cypher property-graph system specialized for
//! storage-backed BFS, WCC, and PageRank.
//!
//! The crate is built phase by phase against the frozen design vector in
//! `.synthesis/design_vector.json`; the module map mirrors the component
//! ownership recorded in `.synthesis/capability_matrix.tsv`.

pub mod compute;
pub mod db;
pub mod impl_;
pub mod language;
pub mod runtime;
pub mod schema;
pub mod storage;
#[cfg(test)]
pub mod test;
pub mod txn;

pub use compute::{CsrProjection, MetagdbExecutor, MetagdbOptimizer, MetagdbPlanner};
pub use db::Db;
pub use impl_::MetagdbImpl;
pub use language::MetagdbParser;
pub use schema::SchemaRegistry;
pub use storage::MvccStorage;
pub use txn::MetagdbTxnManager;
