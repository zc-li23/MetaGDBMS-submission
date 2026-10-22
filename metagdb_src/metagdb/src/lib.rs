/**
 * @brief: metaGDB framework body, defining control flow orchestration (pipeline) and component
 *         assembly (assembly).
 */

pub mod assembly;
pub mod pipeline;

pub use pipeline::{Runtime, RuntimeBuilder};
