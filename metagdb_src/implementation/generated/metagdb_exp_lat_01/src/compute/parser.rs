//! ANTLR-backed Cypher parser.
//!
//! The repository-generated lexer/parser is the sole syntax frontend: this
//! module builds `CypherLexer` → `CommonTokenStream` → `CypherParser`, invokes
//! the top-level `oC_Cypher()` rule with a bail error strategy, and lowers the
//! resulting CST.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use antlr4rust::common_token_stream::CommonTokenStream;

use antlr4rust::{BailErrorStrategy, InputStream};

use metagdb_cypher_antlr::cypherlexer::CypherLexer;
use metagdb_cypher_antlr::cypherparser::CypherParser;
use metagdb_core::l4_compute::engine::{LanguageAst, Parser};
use metagdb_core::{Error, Result};

use super::ast::CypherAst;
use super::lowering;

/// Capacity of the process-wide prepared-statement cache.
const PARSE_CACHE_CAPACITY: usize = 512;

/// Process-wide prepared-statement cache.
///
/// The fixed assembly constructs `MetagdbParser` as a unit struct, so the cache
/// is process-global. Parsed ASTs are immutable and carry no schema state (name
/// binding happens later in the planner), so caching is safe. The warm
/// point-read and one-hop workloads re-issue identical statements, where
/// re-running the ANTLR frontend dominated query latency.
fn parse_cache() -> &'static Mutex<HashMap<String, CypherAst>> {
    static CACHE: OnceLock<Mutex<HashMap<String, CypherAst>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Cypher parser stage.
#[derive(Default)]
pub struct MetagdbParser;

impl MetagdbParser {
    fn parse_uncached(src: &str) -> Result<CypherAst> {
        let input = InputStream::new(src);
        let lexer = CypherLexer::new(input);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = CypherParser::with_strategy(token_stream, Box::new(BailErrorStrategy::new()));
        let cypher_ctx = parser
            .oC_Cypher()
            .map_err(|e| Error::Parse(format!("ANTLR parse failure: {e:?}")))?;
        lowering::lower_cypher(cypher_ctx.as_ref())
    }
}

impl Parser for MetagdbParser {
    fn parse(&self, src: &str) -> Result<Box<dyn LanguageAst>> {
        if let Some(ast) = parse_cache().lock().unwrap().get(src) {
            return Ok(Box::new(ast.clone()));
        }
        let ast = Self::parse_uncached(src)?;
        let mut cache = parse_cache().lock().unwrap();
        if cache.len() < PARSE_CACHE_CAPACITY {
            cache.insert(src.to_string(), ast.clone());
        }
        Ok(Box::new(ast))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::ast::{BinOp, Clause, CypherAst, Expr};

    fn parse(src: &str) -> CypherAst {
        let ast = MetagdbParser.parse(src).expect("parse must succeed");
        ast.as_any()
            .downcast_ref::<CypherAst>()
            .expect("parser must produce CypherAst")
            .clone()
    }

    fn _assert_text(ast: &CypherAst) -> String {
        format!("{ast:?}")
    }

    #[test]
    fn lowers_match_return() {
        let ast = parse("MATCH (n:Person) WHERE n.age > 30 RETURN n.name");
        assert_eq!(ast.query.head.len(), 2);
        match &ast.query.head[0] {
            Clause::Match(m) => {
                assert_eq!(m.parts.len(), 1);
                assert_eq!(m.parts[0].element.node.labels, vec!["Person".to_string()]);
                assert!(m.where_clause.is_some());
            }
            other => panic!("expected MATCH, got {other:?}"),
        }
        match &ast.query.head[1] {
            Clause::Return(r) => assert_eq!(r.items.len(), 1),
            other => panic!("expected RETURN, got {other:?}"),
        }
    }

    #[test]
    fn lowers_relationship_directions_and_ranges() {
        let ast = parse("MATCH (a)-[:KNOWS*2..3]->(b) RETURN b");
        match &ast.query.head[0] {
            Clause::Match(m) => {
                let (rel, _) = &m.parts[0].element.chains[0];
                assert_eq!(rel.min_hops, 2);
                assert_eq!(rel.max_hops, Some(3));
            }
            other => panic!("expected MATCH, got {other:?}"),
        }

        let ast = parse("MATCH (a)<-[:KNOWS]-(b) RETURN b");
        match &ast.query.head[0] {
            Clause::Match(m) => {
                let (rel, _) = &m.parts[0].element.chains[0];
                assert_eq!(rel.direction, crate::compute::ast::Direction::Incoming);
            }
            other => panic!("expected MATCH, got {other:?}"),
        }
    }

    #[test]
    fn lowers_arithmetic_precedence() {
        let ast = parse("RETURN 1 + 2 * 3");
        match &ast.query.head[0] {
            Clause::Return(r) => match &r.items[0].expr {
                Expr::Binary(BinOp::Add, _, rhs) => {
                    assert!(matches!(**rhs, Expr::Binary(BinOp::Mul, _, _)));
                }
                other => panic!("expected add at root, got {other:?}"),
            },
            other => panic!("expected RETURN, got {other:?}"),
        }
    }

    #[test]
    fn lowers_unions_and_unwind() {
        let ast = parse("UNWIND [1, 2, 3] AS x RETURN x UNION ALL UNWIND [4] AS y RETURN y");
        assert_eq!(ast.query.head.len(), 2);
        assert_eq!(ast.query.unions.len(), 1);
        assert!(ast.query.unions[0].all);
    }

    #[test]
    fn rejects_invalid_syntax() {
        assert!(MetagdbParser.parse("MATCH (n RETURN n").is_err());
        assert!(MetagdbParser.parse("g.V().count()").is_err());
        assert!(MetagdbParser.parse("RETURN 1 +").is_err());
    }

    #[test]
    fn lowers_not_predicate() {
        let ast = parse("MATCH (n) WHERE NOT n.age = 30 RETURN n");
        match &ast.query.head[0] {
            Clause::Match(m) => {
                eprintln!("WHERE={:?}", m.where_clause);
                match m.where_clause.as_ref().expect("where") {
                    Expr::Not(inner) => assert!(matches!(**inner, Expr::Binary(BinOp::Eq, _, _)), "inner {inner:?}"),
                    other => panic!("expected NOT, got {other:?}"),
                }
            }
            other => panic!("expected MATCH, got {other:?}"),
        }
    }

    #[test]
    fn lowers_detach_delete() {
        let ast = parse("MATCH (n) DETACH DELETE n");
        match &ast.query.head[1] {
            Clause::Delete { detach, .. } => assert!(*detach, "DETACH flag lost"),
            other => panic!("expected DELETE, got {other:?}"),
        }
    }

    #[test]
    fn lowers_multi_part_with_ordering() {
        let ast = parse("MATCH (n) WITH collect(n.age) AS ages UNWIND ages AS age RETURN age");
        let kinds: Vec<&str> = ast
            .query
            .head
            .iter()
            .map(|clause| match clause {
                Clause::Match(_) => "match",
                Clause::With(_) => "with",
                Clause::Unwind { .. } => "unwind",
                Clause::Return(_) => "return",
                _ => "other",
            })
            .collect();
        assert_eq!(kinds, vec!["match", "with", "unwind", "return"]);
    }
}
