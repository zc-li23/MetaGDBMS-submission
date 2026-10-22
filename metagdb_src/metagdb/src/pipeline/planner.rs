/**
 * @brief: Query planning stage orchestration, responsible for converting a language-private AST
 *         into a logical plan (LogicalPlan).
 */

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::LogicalPlan;
use metagdb_core::l4_compute::engine::{LanguageAst, Planner};
use metagdb_core::Result;

/**
 * @brief: Drive the planner to perform logical modeling on the AST.
 * @param planner: Injected planner implementation component.
 * @param ast: Language-private AST to convert, routed and downcast internally by the Planner
 *             according to `language()`.
 * @param schema: Bound metadata Schema, used for semantic analysis.
 * @return: Returns a LogicalPlan on success, or Error otherwise.
 */
pub fn run(
    planner: &dyn Planner,
    ast: Box<dyn LanguageAst>,
    schema: &dyn GraphSchema,
) -> Result<LogicalPlan> {
    planner.plan(ast, schema)
}
