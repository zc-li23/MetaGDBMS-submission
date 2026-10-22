#![allow(dead_code)]
#![allow(unused)]
/**
 * @brief: MetagdbParser: Cypher parser implementation provided by metagdb_default.
 *
 * Uses ANTLR-generated Cypher Lexer/Parser, translating ANTLR CST to CypherAst via lowering functions.
 * Adopts BailErrorStrategy to ensure termination on first error, avoiding generation of half-broken CST.
 */

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::InputStream;
use antlr4rust::BailErrorStrategy;

use metagdb_cypher_antlr::cypherlexer::CypherLexer;
use metagdb_cypher_antlr::cypherparser::*;

use metagdb_core::l4_compute::engine::{LanguageAst, Parser};
use metagdb_core::{Error, Result};

use super::cypher_lowering;

// ============================================================
// Parser entry point
// ============================================================

/**
 * @brief: Default Cypher parser (ANTLR-driven).
 */
#[derive(Default)]
pub struct MetagdbParser;

impl Parser for MetagdbParser {
    /**
     * @brief: Parse a query string. Uses the ANTLR pipeline:
     *   InputStream -> CypherLexer -> CommonTokenStream -> CypherParser -> oC_Cypher()
     *
     * Adopts BailErrorStrategy: throws exception on first syntax error, avoiding generation of half-broken CST.
     * Errors are converted to Error::Parse via ANTLR exception messages and propagated upward.
     *
     * @param src: Cypher query script.
     * @return: On success, returns CypherAst wrapped as `Box<dyn LanguageAst>`.
     */
    fn parse(&self, src: &str) -> Result<Box<dyn LanguageAst>> {
        // Build ANTLR parsing pipeline
        let input = InputStream::new(src);
        let lexer = CypherLexer::new(input);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = CypherParser::with_strategy(
            token_stream,
            Box::new(BailErrorStrategy::new()),
        );

        // Parse
        let cypher_ctx = parser
            .oC_Cypher()
            .map_err(|e| Error::Parse(format!("ANTLR parse failure: {e:?}")))?;

        // Lower CST to CypherAst
        let ast = cypher_lowering::lower_cypher(cypher_ctx.as_ref())?;
        Ok(Box::new(ast))
    }
}

// ============================================================
// Re-exports for backward compatibility (test dependencies)
// ============================================================

pub use cypher_lowering::{
    extract_label_name, extract_literal_from_add_sub, extract_property_key_name,
    extract_schema_name_text, extract_var_from_add_sub, extract_variable_name,
    lower_add_or_subtract_expression, lower_and_expression, lower_atom, lower_call_argument,
    lower_comparison_expression, lower_expression, lower_literal, lower_match,
    lower_not_expression, lower_or_expression, lower_property_or_labels_expression,
    lower_property_value, lower_return, lower_string_list_null_expression, lower_xor_expression,
};

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cypher_ast::{CypherAst, Clause, EdgeDirection};

    /**
     * @brief: Test helper: parse and downcast `Box<dyn LanguageAst>` back to CypherAst.
     */
    fn parse_cypher(src: &str) -> CypherAst {
        let ast = MetagdbParser.parse(src).unwrap();
        ast.as_any()
            .downcast_ref::<CypherAst>()
            .expect("MetagdbParser must produce CypherAst")
            .clone()
    }

    // ============================================================
    // MATCH tests
    // ============================================================

    #[test]
    fn parse_match_all() {
        let ast = parse_cypher("MATCH (v) RETURN v");
        match ast {
            CypherAst::Query(ref q) => {
                assert_eq!(q.clauses.len(), 2);
                match &q.clauses[0] {
                    Clause::Match(mc) => {
                        assert_eq!(mc.pattern.elements.len(), 1);
                        let el = &mc.pattern.elements[0];
                        assert_eq!(el.node.var.as_deref(), Some("v"));
                        assert!(el.node.label.is_none());
                        assert!(el.edges.is_empty());
                        assert!(mc.where_clause.is_none());
                    }
                    other => panic!("expected Match clause, got {other:?}"),
                }
                match &q.clauses[1] {
                    Clause::Return(rc) => {
                        assert_eq!(rc.items.len(), 1);
                    }
                    other => panic!("expected Return clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_match_label_props() {
        let ast = parse_cypher("MATCH (v:Person) RETURN v.name, v.age");
        match ast {
            CypherAst::Query(ref q) => {
                assert_eq!(q.clauses.len(), 2);
                match &q.clauses[0] {
                    Clause::Match(mc) => {
                        let el = &mc.pattern.elements[0];
                        assert_eq!(el.node.label.as_deref(), Some("Person"));
                    }
                    other => panic!("expected Match clause"),
                }
                match &q.clauses[1] {
                    Clause::Return(rc) => {
                        assert_eq!(rc.items.len(), 2);
                    }
                    other => panic!("expected Return clause"),
                }
            }
            other => panic!("expected Query"),
        }
    }

    #[test]
    fn parse_match_with_edge() {
        let ast = parse_cypher("MATCH (a)-[:KNOWS]->(b) RETURN b");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[0] {
                    Clause::Match(mc) => {
                        let el = &mc.pattern.elements[0];
                        assert_eq!(el.node.var.as_deref(), Some("a"));
                        assert_eq!(el.edges.len(), 1);
                        let e = &el.edges[0];
                        assert_eq!(e.direction, EdgeDirection::Outgoing);
                        assert_eq!(e.target.var.as_deref(), Some("b"));
                        assert_eq!(
                            e.edge_types.as_deref(),
                            Some(&vec!["KNOWS".to_string()][..])
                        );
                    }
                    other => panic!("expected Match clause"),
                }
            }
            other => panic!("expected Query"),
        }
    }

    #[test]
    fn parse_match_with_where() {
        let ast = parse_cypher("MATCH (a)-[:KNOWS]->(b) WHERE a.age > 30 RETURN b.name");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[0] {
                    Clause::Match(mc) => {
                        assert!(mc.where_clause.is_some());
                    }
                    other => panic!("expected Match clause"),
                }
            }
            other => panic!("expected Query"),
        }
    }

    #[test]
    fn parse_call_bfs() {
        let ast = parse_cypher("CALL bfs(source=1)");
        match ast {
            CypherAst::CallAlgorithm { algo, args } => {
                assert_eq!(algo, "bfs");
                assert_eq!(
                    args.get("source"),
                    Some(&metagdb_core::common::property::PropertyValue::Int(1))
                );
            }
            other => panic!("expected CallAlgorithm, got {other:?}"),
        }
    }

    // ============================================================
    // WITH tests
    // ============================================================

    #[test]
    fn parse_with_clause() {
        let ast = parse_cypher("MATCH (a) WITH a.name AS name, a.age AS age WHERE name = 'Alice' RETURN name");
        match ast {
            CypherAst::Query(ref q) => {
                assert!(q.clauses.len() >= 3);
                match &q.clauses[1] {
                    Clause::With(wc) => {
                        assert_eq!(wc.items.len(), 2);
                        assert!(wc.where_clause.is_some());
                    }
                    other => panic!("expected With clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    // ============================================================
    // DML tests
    // ============================================================

    #[test]
    fn parse_create() {
        let ast = parse_cypher("CREATE (n:Person {name: 'Bob'})");
        match ast {
            CypherAst::Query(ref q) => {
                assert_eq!(q.clauses.len(), 1);
                match &q.clauses[0] {
                    Clause::Create(_) => {}
                    other => panic!("expected Create clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_delete() {
        let ast = parse_cypher("MATCH (n) DELETE n");
        match ast {
            CypherAst::Query(ref q) => {
                assert_eq!(q.clauses.len(), 2);
                match &q.clauses[1] {
                    Clause::Delete(_) => {}
                    other => panic!("expected Delete clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_detach_delete() {
        let ast = parse_cypher("MATCH (n) DETACH DELETE n");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[1] {
                    Clause::Delete(dc) => {
                        assert!(matches!(dc, super::super::cypher_ast::DeleteClause::DetachDelete(_)));
                    }
                    other => panic!("expected Delete clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_set() {
        let ast = parse_cypher("MATCH (n) SET n.name = 'Alice'");
        match ast {
            CypherAst::Query(ref q) => {
                assert_eq!(q.clauses.len(), 2);
                match &q.clauses[1] {
                    Clause::Set(sc) => {
                        assert_eq!(sc.items.len(), 1);
                    }
                    other => panic!("expected Set clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_remove() {
        let ast = parse_cypher("MATCH (n) REMOVE n.age");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[1] {
                    Clause::Remove(_) => {}
                    other => panic!("expected Remove clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_unwind() {
        let ast = parse_cypher("UNWIND [1,2,3] AS x RETURN x");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[0] {
                    Clause::Unwind(uc) => {
                        assert_eq!(uc.var, "x");
                    }
                    other => panic!("expected Unwind clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_return_order_limit() {
        let ast = parse_cypher("MATCH (n) RETURN n.name ORDER BY n.age DESC SKIP 5 LIMIT 10");
        match ast {
            CypherAst::Query(ref q) => {
                match &q.clauses[1] {
                    Clause::Return(rc) => {
                        assert!(rc.order_by.is_some());
                        assert!(rc.skip.is_some());
                        assert!(rc.limit.is_some());
                    }
                    other => panic!("expected Return clause, got {other:?}"),
                }
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    // ============================================================
    // UNION tests
    // ============================================================

    #[test]
    fn parse_union_all() {
        let ast = parse_cypher("MATCH (n) RETURN n UNION ALL MATCH (m) RETURN m");
        match ast {
            CypherAst::Query(ref q) => {
                let uc = q.unions.as_ref().expect("expected UNION clause");
                assert_eq!(uc.branches.len(), 1);
                assert!(uc.branches[0].all);
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_union_distinct() {
        let ast = parse_cypher("MATCH (n) RETURN n UNION MATCH (m) RETURN m");
        match ast {
            CypherAst::Query(ref q) => {
                let uc = q.unions.as_ref().expect("expected UNION clause");
                assert_eq!(uc.branches.len(), 1);
                assert!(!uc.branches[0].all);
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }

    #[test]
    fn parse_union_multi_branch() {
        let ast = parse_cypher(
            "MATCH (a) RETURN a UNION ALL MATCH (b) RETURN b UNION MATCH (c) RETURN c",
        );
        match ast {
            CypherAst::Query(ref q) => {
                let uc = q.unions.as_ref().expect("expected UNION clause");
                assert_eq!(uc.branches.len(), 2);
                assert!(uc.branches[0].all); // UNION ALL
                assert!(!uc.branches[1].all); // UNION (distinct)
            }
            other => panic!("expected Query, got {other:?}"),
        }
    }
}
