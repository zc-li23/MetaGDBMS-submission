/**
 * @brief: L5 interaction layer entry module, wrapping raw query text into an internally
 *         processable query request object.
 *
 * The concrete Frontend implementation (LanguageFrontend) is defined in
 * metagdb_core::l5_interaction; this module is only responsible for orchestration.
 */

use metagdb_core::l5_interaction::{Frontend, QueryRequest};

/**
 * @brief: Drive a frontend component to construct a request object for the given query text.
 * @param frontend: Injected frontend component handler.
 * @param src: Raw query source string.
 * @return: Returns the constructed QueryRequest object.
 */
pub fn run(frontend: &dyn Frontend, src: &str) -> QueryRequest {
    frontend.make_request(src)
}
