//! Framework `Parser` implementation.
//!
//! The only syntax frontend is the repository-generated ANTLR crate.  The
//! pipeline is exactly
//!
//! ```text
//! InputStream -> CypherLexer -> CommonTokenStream -> CypherParser
//!             -> oC_Cypher() -> lower_cypher(root)
//! ```
//!
//! and `BailErrorStrategy` turns the first syntax error into `Error::Parse`.

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::{BailErrorStrategy, InputStream};

use metagdb_cypher_antlr::cypherlexer::CypherLexer;
use metagdb_cypher_antlr::cypherparser::CypherParser;

use metagdb_core::l4_compute::engine::{LanguageAst, Parser};
use metagdb_core::{Error, Result};

use super::cypher_lowering;

/// Cypher parser backed by the generated ANTLR lexer/parser.
#[derive(Default)]
pub struct MetagdbParser;

impl Parser for MetagdbParser {
    fn parse(&self, src: &str) -> Result<Box<dyn LanguageAst>> {
        let input = InputStream::new(src);
        let lexer = CypherLexer::new(input);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser =
            CypherParser::with_strategy(token_stream, Box::new(BailErrorStrategy::new()));
        let root = parser
            .oC_Cypher()
            .map_err(|error| Error::Parse(format!("ANTLR parse failure: {error:?}")))?;
        let ast = cypher_lowering::lower_cypher(root.as_ref())?;
        Ok(Box::new(ast))
    }
}
