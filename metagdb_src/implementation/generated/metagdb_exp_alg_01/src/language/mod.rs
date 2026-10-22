//! Cypher language front end.
//!
//! Three artefacts, deliberately separate:
//!
//! 1. [`ast`] — the private semantic representation, free of ANTLR lifetimes;
//! 2. [`lowering`] — converts the repository-generated ANTLR CST into that AST;
//! 3. `parser` — implements the framework `Parser` trait by constructing the
//!    generated lexer/parser, invoking the Cypher entry rule `oC_Cypher()`, and
//!    delegating to lowering.
//!
//! The generated crate `metagdb_cypher_antlr` is the only syntax frontend; this
//! module never tokenizes or parses Cypher itself.

pub mod ast;
pub mod lowering;
pub mod parser;

pub use ast::CypherAst;
pub use lowering::lower_cypher;
pub use parser::MetagdbParser;
