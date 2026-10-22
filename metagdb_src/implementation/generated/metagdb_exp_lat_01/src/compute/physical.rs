//! Physical plan: the optimizer's executable output.
//!
//! The pipeline stages are the same shape as the logical plan but carry the
//! physical annotations produced by optimization (for example the point-read
//! hint extracted from `id(v) = k`). The executor interprets this structure as
//! a push-based operator pipeline.

use super::plan::LogicalQuery;

/// Executable physical query.
#[derive(Clone, Debug)]
pub struct PhysicalQuery(pub LogicalQuery);

impl PhysicalQuery {
    pub fn stages(&self) -> &[super::plan::Stage] {
        &self.0.stages
    }

    pub fn unions(&self) -> &[(bool, Vec<super::plan::Stage>)] {
        &self.0.unions
    }
}
