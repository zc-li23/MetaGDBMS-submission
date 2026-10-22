//! Compute layer: analytical projections and algorithms, the Cypher planner,
//! physical-plan lowering, and execution.

pub mod algorithms;
pub mod executor;
pub mod logical_ext;
pub mod optimizer;
pub mod physical_ops;
pub mod planner;
pub mod projection;
#[cfg(test)]
pub mod test_support;
pub mod temporal;
pub mod value;

pub use algorithms::default_algorithms;
pub use executor::MetagdbExecutor;
pub use optimizer::MetagdbOptimizer;
pub use planner::MetagdbPlanner;
pub use projection::CsrProjection;
