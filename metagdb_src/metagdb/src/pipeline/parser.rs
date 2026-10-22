/**
 * @brief: Query parsing stage orchestration, responsible for parsing a query source string into a
 *         language-private AST object.
 */

use metagdb_core::l4_compute::engine::{LanguageAst, Parser};
use metagdb_core::l5_interaction::QueryRequest;
use metagdb_core::Result;

/**
 * @brief: Drive the parser to parse a query request.
 * @param parser: Injected parser implementation component.
 * @param req: Request object containing the query text.
 * @return: Returns a language-private AST on success, or Error if parsing fails.
 */
pub fn run(parser: &dyn Parser, req: &QueryRequest) -> Result<Box<dyn LanguageAst>> {
    parser.parse(&req.source)
}
