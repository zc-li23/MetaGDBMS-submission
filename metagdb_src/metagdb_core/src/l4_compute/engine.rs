/**
 * @brief: L4.b query engine contract, defining the standard interfaces for the four
 *         stages: parsing, planning, optimization, and execution.
 */

use std::any::Any;
use std::sync::Arc;

use crate::common::property::ResultValue;
use crate::l1_model::GraphSchema;
use crate::l2_storage::StorageEngine;
use crate::l3_txn::TxnContext;
use crate::l4_compute::algebra::{LogicalPlan, PhysicalPlan};
use crate::l4_compute::olap::AlgorithmRegistry;
use crate::l5_interaction::QueryLanguage;
use crate::Result;

/**
 * @brief: Stub contract for the language abstract syntax tree (AST).
 *
 * core does not prescribe a specific node shape for the AST -- Cypher, GQL, and
 * Gremlin syntax trees have naturally significant differences. The AST serves as
 * the Planner's input, can be passed across stages, and can be moved across threads.
 * Concrete node definitions are provided by each implementation.
 */
pub trait LanguageAst: Any + Send + Sync {
    /**
     * @brief: Declare the query language to which this AST belongs, used by the
     *         Planner to route to the corresponding binder.
     * @return: A QueryLanguage enum value.
     */
    fn language(&self) -> QueryLanguage;

    /**
     * @brief: Downcast hook, enabling the Planner to retrieve the concrete AST type
     *         from `&dyn LanguageAst`.
     * @return: A `&dyn Any` view of the current object.
     */
    fn as_any(&self) -> &dyn Any;
}

pub trait Parser: Send + Sync {
    /**
     * @brief: Parse source text into a language-private AST object.
     * @param src: The query source string.
     * @return: An opaque AST implementing `LanguageAst` on success, or an Error on
     *          failure.
     */
    fn parse(&self, src: &str) -> Result<Box<dyn LanguageAst>>;
}

pub trait Planner: Send + Sync {
    /**
     * @brief: Map an AST to a logical query plan.
     * @param ast: The language-private AST to process; the Planner routes and
     *             downcasts based on `language()`.
     * @param schema: Schema view used for semantic binding.
     * @return: A logical plan on success, or an Error on failure.
     */
    fn plan(&self, ast: Box<dyn LanguageAst>, schema: &dyn GraphSchema) -> Result<LogicalPlan>;
}

pub trait Optimizer: Send + Sync {
    /**
     * @brief: Optimize a logical plan into a physical plan based on optimization
     *         rules and statistical context.
     * @param lp: The input logical plan.
     * @param ctx: The optimization context (including metadata, etc.).
     * @return: A physical plan on success.
     */
    fn optimize(&self, lp: LogicalPlan, ctx: &OptContext) -> Result<PhysicalPlan>;
}

pub trait Executor: Send + Sync {
    /**
     * @brief: Drive the execution of physical operators and produce a result stream.
     * @param pp: The physical plan to execute.
     * @param ctx: The runtime context (including transaction, storage access, etc.).
     * @return: A ResultStream object.
     */
    fn execute(&self, pp: &PhysicalPlan, ctx: &ExecContext) -> Result<ResultStream>;
}

/**
 * @brief: Optimization phase reference context, containing Schema and algorithm
 *         library information.
 */
#[derive(Clone)]
pub struct OptContext {
    pub schema: Arc<dyn GraphSchema>,
    pub algorithms: Arc<AlgorithmRegistry>,
}

/**
 * @brief: Execution phase runtime context, containing full access to transaction,
 *         storage, metadata, and algorithm library.
 */
#[derive(Clone)]
pub struct ExecContext {
    pub txn: TxnContext,
    pub storage: Arc<dyn StorageEngine>,
    pub schema: Arc<dyn GraphSchema>,
    pub algorithms: Arc<AlgorithmRegistry>,
}

/**
 * @brief: Query result stream container, containing result column metadata (Schema)
 *         and row data.
 */
#[derive(Debug, Clone)]
pub struct ResultStream {
    pub schema: Vec<String>,
    pub rows: Vec<Vec<ResultValue>>,
}

impl ResultStream {
    pub fn empty() -> Self {
        Self {
            schema: Vec::new(),
            rows: Vec::new(),
        }
    }
}
