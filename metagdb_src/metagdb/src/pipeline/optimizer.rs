/**
 * @brief: Query optimization stage orchestration, responsible for converting a logical plan into
 *         an optimized physical plan.
 */

use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::{LogicalPlan, PhysicalPlan};
use metagdb_core::l4_compute::engine::{OptContext, Optimizer};
use metagdb_core::l4_compute::olap::AlgorithmRegistry;
use metagdb_core::Result;

/**
 * @brief: Run the optimizer. Constructs an optimization context and optimizes the logical plan
 *         into a physical plan.
 * @param optimizer: Query optimizer component.
 * @param lp: Input logical plan.
 * @param schema: Graph metadata handle.
 * @param algorithms: Algorithm registry.
 * @return: Returns the generated PhysicalPlan on success, or Error otherwise.
 */
pub fn run(
    optimizer: &dyn Optimizer,
    lp: LogicalPlan,
    schema: Arc<dyn GraphSchema>,
    algorithms: Arc<AlgorithmRegistry>,
) -> Result<PhysicalPlan> {
    let ctx = OptContext { schema, algorithms };
    optimizer.optimize(lp, &ctx)
}
