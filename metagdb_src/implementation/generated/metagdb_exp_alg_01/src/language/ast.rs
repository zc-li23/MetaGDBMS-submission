//! Private Cypher AST.
//!
//! This is the candidate's own semantic representation of a Cypher statement,
//! shaped after `data/language/Cypher.g4` but free of ANTLR lifetimes and of the
//! framework's physical types.  Lowering (`crate::language::lowering`) builds
//! it from the generated parser's CST, and the planner
//! (`crate::planner`) turns it into `l4_compute::algebra` IR.
//!
//! Grammar shape is preserved where it matters to planning:
//!
//! * `Query` distinguishes a regular query from a standalone procedure call;
//! * a regular query is a list of `UNION` branches, each a flat clause sequence
//!   (a multi-part query is exactly a clause sequence containing `With`);
//! * optional grammar parts are `Option<T>` rather than sentinels;
//! * pattern direction, relationship types, and variable-length ranges are kept
//!   as the grammar distinguishes them.
//!
//! The algorithm-call form ([`Clause::Call`] and [`AstBody::Call`]) carries any
//! registered algorithm name plus scalar arguments, so the three algorithms the
//! design vector requires are expressed by the same node as any other procedure.

use std::any::Any;

use metagdb_core::l4_compute::engine::LanguageAst;
use metagdb_core::l5_interaction::QueryLanguage;

/// Root of a parsed statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CypherAst {
    pub body: AstBody,
}

impl LanguageAst for CypherAst {
    fn language(&self) -> QueryLanguage {
        QueryLanguage::Cypher
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Top-level statement shape.
#[derive(Debug, Clone, PartialEq)]
pub enum AstBody {
    /// `RETURN …` / `MATCH … RETURN …`, optionally joined by `UNION`.
    Union {
        all: bool,
        branches: Vec<Vec<Clause>>,
    },
    /// `CALL name(args) [YIELD …]` with no surrounding clause.
    Call(ProcedureCall),
}

/// One clause of a query branch, in source order.
#[derive(Debug, Clone, PartialEq)]
pub enum Clause {
    /// `[OPTIONAL] MATCH pattern [WHERE predicate]`
    Match {
        optional: bool,
        pattern: Pattern,
        predicate: Option<AstExpr>,
    },
    /// `UNWIND expression AS variable`
    Unwind { expr: AstExpr, alias: String },
    /// `WITH projection [WHERE predicate]`
    With {
        body: ProjectionBody,
        predicate: Option<AstExpr>,
    },
    /// `RETURN projection`
    Return(ProjectionBody),
    /// `CREATE pattern`
    Create(Pattern),
    /// `MERGE pattern [ON MATCH SET …] [ON CREATE SET …]`
    Merge {
        pattern: PatternPart,
        actions: Vec<MergeAction>,
    },
    /// `SET item [, item]*`
    Set(Vec<SetItem>),
    /// `REMOVE item [, item]*`
    Remove(Vec<RemoveItem>),
    /// `[DETACH] DELETE expr [, expr]*`
    Delete {
        detach: bool,
        targets: Vec<AstExpr>,
    },
    /// `CALL name(args) [YIELD …]` used inside a clause sequence.
    Call(ProcedureCall),
}

/// `RETURN`/`WITH` projection list and modifiers.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectionBody {
    pub distinct: bool,
    pub items: Vec<ProjectionItem>,
    pub order: Vec<SortItem>,
    pub skip: Option<AstExpr>,
    pub limit: Option<AstExpr>,
}

/// One projected expression and its optional `AS` alias.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionItem {
    pub expr: AstExpr,
    pub alias: Option<String>,
}

/// One `ORDER BY` key.
#[derive(Debug, Clone, PartialEq)]
pub struct SortItem {
    pub expr: AstExpr,
    pub descending: bool,
}

/// `MERGE`'s `ON MATCH`/`ON CREATE` action.
#[derive(Debug, Clone, PartialEq)]
pub struct MergeAction {
    pub on_match: bool,
    pub items: Vec<SetItem>,
}

/// One `SET` item.
#[derive(Debug, Clone, PartialEq)]
pub enum SetItem {
    /// `n.prop = value`
    Property {
        target: AstExpr,
        value: AstExpr,
    },
    /// `n = value` — replace every property of the element.
    Replace { variable: String, value: AstExpr },
    /// `n += value` — merge the given map into the element's properties.
    Merge { variable: String, value: AstExpr },
    /// `n:Label:Other`
    Labels { variable: String, labels: Vec<String> },
}

/// One `REMOVE` item.
#[derive(Debug, Clone, PartialEq)]
pub enum RemoveItem {
    /// `n.prop`
    Property(AstExpr),
    /// `n:Label`
    Labels { variable: String, labels: Vec<String> },
}

/// A pattern: one or more comma-separated parts.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pattern {
    pub parts: Vec<PatternPart>,
}

/// A pattern part, optionally bound to a path variable.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternPart {
    pub variable: Option<String>,
    pub element: PatternElement,
}

/// A start node followed by relationship/node chains.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternElement {
    pub start: NodePattern,
    pub chains: Vec<(RelationshipPattern, NodePattern)>,
}

/// `(variable?:Label:Other {properties})`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub labels: Vec<String>,
    pub properties: Option<Box<AstExpr>>,
}

/// Relationship direction as the grammar's arrow heads express it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Outgoing,
    Incoming,
    Undirected,
}

/// `-[variable?:TYPE|OTHER*range {properties}]->`
#[derive(Debug, Clone, PartialEq)]
pub struct RelationshipPattern {
    pub variable: Option<String>,
    pub types: Vec<String>,
    pub direction: Direction,
    /// `*`, `*n`, `*n..`, `*..m`, `*n..m` — absent means exactly one hop.
    pub range: Option<Range>,
    pub properties: Option<Box<AstExpr>>,
}

/// Variable-length relationship bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

impl Range {
    /// Bounds with the grammar's defaults applied: `*` is `1..`, `*n` is `n..n`.
    pub fn bounds(&self) -> (u32, Option<u32>) {
        match (self.min, self.max) {
            (Some(min), Some(max)) => (min, Some(max)),
            (Some(min), None) => (min, None),
            (None, Some(max)) => (1, Some(max)),
            (None, None) => (1, None),
        }
    }
}

/// `CALL name(args) [YIELD items]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureCall {
    /// Fully qualified name as written, e.g. `bfs` or `db.labels`.
    pub name: String,
    pub args: Vec<AstExpr>,
    pub yields: Option<YieldItems>,
}

/// `YIELD * [WHERE …]` or `YIELD item AS alias, … [WHERE …]`.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldItems {
    pub all: bool,
    pub items: Vec<YieldItem>,
    pub predicate: Option<AstExpr>,
}

/// One yielded column.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldItem {
    /// Result column produced by the procedure, when it differs from `alias`.
    pub field: Option<String>,
    pub alias: String,
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
    Plus,
    Not,
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Or,
    Xor,
    And,
    Eq,
    NotEq,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

/// String predicate operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringOp {
    StartsWith,
    EndsWith,
    Contains,
}

/// Quantifier predicate over a list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantifierKind {
    All,
    Any,
    None,
    Single,
}

/// Cypher scalar expression.
#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    List(Vec<AstExpr>),
    /// Map literal; keys keep source order.
    Map(Vec<(String, AstExpr)>),
    Variable(String),
    /// `$name` query parameter.
    Parameter(String),
    /// `base.key`
    Property {
        base: Box<AstExpr>,
        key: String,
    },
    /// `base:Label` used as a predicate.
    Labels {
        base: Box<AstExpr>,
        labels: Vec<String>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<AstExpr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<AstExpr>,
        right: Box<AstExpr>,
    },
    /// `operand IS [NOT] NULL`
    IsNull {
        operand: Box<AstExpr>,
        negated: bool,
    },
    /// `list[index]`
    ListIndex {
        list: Box<AstExpr>,
        index: Box<AstExpr>,
    },
    /// `list[from..to]`, with either bound optional.
    ListSlice {
        list: Box<AstExpr>,
        from: Option<Box<AstExpr>>,
        to: Option<Box<AstExpr>>,
    },
    /// `item IN list`
    In {
        item: Box<AstExpr>,
        list: Box<AstExpr>,
    },
    /// `left STARTS WITH right`, `ENDS WITH`, `CONTAINS`
    StringOp {
        op: StringOp,
        left: Box<AstExpr>,
        right: Box<AstExpr>,
    },
    /// `CASE [operand] WHEN … THEN … [ELSE …] END`
    Case {
        operand: Option<Box<AstExpr>>,
        alternatives: Vec<(AstExpr, AstExpr)>,
        fallback: Option<Box<AstExpr>>,
    },
    /// A function call; `distinct` reflects `f(DISTINCT x)`.
    Function {
        name: String,
        distinct: bool,
        args: Vec<AstExpr>,
    },
    /// `count(*)`
    CountAll,
    /// `[variable IN list [WHERE filter] [| projection]]`
    ListComprehension {
        variable: String,
        list: Box<AstExpr>,
        filter: Option<Box<AstExpr>>,
        projection: Option<Box<AstExpr>>,
    },
    /// `[pattern [WHERE filter] | projection]`
    PatternComprehension {
        variable: Option<String>,
        pattern: Pattern,
        filter: Option<Box<AstExpr>>,
        projection: Box<AstExpr>,
    },
    /// `all|any|none|single(variable IN list WHERE filter)`
    Quantifier {
        kind: QuantifierKind,
        variable: String,
        list: Box<AstExpr>,
        filter: Box<AstExpr>,
    },
    /// A relationship pattern used as an expression, e.g. inside `exists()`.
    Path(Pattern),
}

impl AstExpr {
    /// The variable an expression is bound to, when it is a plain variable.
    pub fn as_variable(&self) -> Option<&str> {
        match self {
            AstExpr::Variable(name) => Some(name),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(variable: &str) -> NodePattern {
        NodePattern {
            variable: Some(variable.to_string()),
            labels: Vec::new(),
            properties: None,
        }
    }

    #[test]
    fn the_ast_declares_the_cypher_language() {
        let ast = CypherAst {
            body: AstBody::Union {
                all: false,
                branches: vec![vec![Clause::Return(ProjectionBody::default())]],
            },
        };
        assert_eq!(ast.language(), QueryLanguage::Cypher);
        assert!(ast.as_any().is::<CypherAst>());
    }

    #[test]
    fn optional_grammar_parts_are_optional_fields() {
        let element = PatternElement {
            start: NodePattern::default(),
            chains: Vec::new(),
        };
        assert!(element.start.variable.is_none());
        assert!(element.start.properties.is_none());

        let relationship = RelationshipPattern {
            variable: None,
            types: Vec::new(),
            direction: Direction::Undirected,
            range: None,
            properties: None,
        };
        assert!(relationship.range.is_none());
        assert!(relationship.types.is_empty());
    }

    #[test]
    fn range_bounds_apply_the_grammar_defaults() {
        assert_eq!(Range { min: None, max: None }.bounds(), (1, None));
        assert_eq!(Range { min: Some(2), max: None }.bounds(), (2, None));
        assert_eq!(Range { min: None, max: Some(3) }.bounds(), (1, Some(3)));
        assert_eq!(Range { min: Some(2), max: Some(2) }.bounds(), (2, Some(2)));
    }

    #[test]
    fn an_algorithm_call_carries_any_registered_name_and_scalar_arguments() {
        let call = ProcedureCall {
            name: "bfs".to_string(),
            args: vec![AstExpr::Int(1)],
            yields: Some(YieldItems {
                all: false,
                items: vec![YieldItem {
                    field: None,
                    alias: "node".to_string(),
                }],
                predicate: None,
            }),
        };
        assert_eq!(call.name, "bfs");
        assert_eq!(call.args, vec![AstExpr::Int(1)]);
        let yields = call.yields.expect("yield items");
        assert_eq!(yields.items[0].alias, "node");

        let pagerank = ProcedureCall {
            name: "pagerank".to_string(),
            args: vec![AstExpr::Float(0.85)],
            yields: None,
        };
        assert!(pagerank.yields.is_none());
    }

    #[test]
    fn a_pattern_part_can_be_bound_to_a_path_variable() {
        let part = PatternPart {
            variable: Some("p".to_string()),
            element: PatternElement {
                start: node("a"),
                chains: vec![(
                    RelationshipPattern {
                        variable: Some("r".to_string()),
                        types: vec!["KNOWS".to_string()],
                        direction: Direction::Outgoing,
                        range: Some(Range {
                            min: Some(2),
                            max: Some(2),
                        }),
                        properties: None,
                    },
                    node("b"),
                )],
            },
        };
        assert_eq!(part.variable.as_deref(), Some("p"));
        assert_eq!(part.element.chains.len(), 1);
        assert_eq!(part.element.chains[0].0.direction, Direction::Outgoing);
    }

    #[test]
    fn every_clause_form_the_profile_uses_is_representable() {
        let clauses = vec![
            Clause::Match {
                optional: true,
                pattern: Pattern::default(),
                predicate: Some(AstExpr::Bool(true)),
            },
            Clause::Unwind {
                expr: AstExpr::List(vec![AstExpr::Int(1)]),
                alias: "x".to_string(),
            },
            Clause::With {
                body: ProjectionBody::default(),
                predicate: None,
            },
            Clause::Return(ProjectionBody::default()),
            Clause::Create(Pattern::default()),
            Clause::Merge {
                pattern: PatternPart {
                    variable: None,
                    element: PatternElement {
                        start: NodePattern::default(),
                        chains: Vec::new(),
                    },
                },
                actions: vec![MergeAction {
                    on_match: false,
                    items: vec![SetItem::Labels {
                        variable: "n".to_string(),
                        labels: vec!["Person".to_string()],
                    }],
                }],
            },
            Clause::Set(vec![SetItem::Replace {
                variable: "n".to_string(),
                value: AstExpr::Map(Vec::new()),
            }]),
            Clause::Remove(vec![RemoveItem::Property(AstExpr::Variable("n".into()))]),
            Clause::Delete {
                detach: true,
                targets: vec![AstExpr::Variable("n".into())],
            },
            Clause::Call(ProcedureCall {
                name: "bfs".to_string(),
                args: Vec::new(),
                yields: None,
            }),
        ];
        assert_eq!(clauses.len(), 10);
    }

    #[test]
    fn expression_forms_cover_the_profile_feature_list() {
        let expressions = vec![
            AstExpr::Case {
                operand: None,
                alternatives: vec![(AstExpr::Bool(true), AstExpr::Int(1))],
                fallback: Some(Box::new(AstExpr::Null)),
            },
            AstExpr::ListComprehension {
                variable: "x".to_string(),
                list: Box::new(AstExpr::List(vec![AstExpr::Int(1)])),
                filter: Some(Box::new(AstExpr::Bool(true))),
                projection: Some(Box::new(AstExpr::Variable("x".into()))),
            },
            AstExpr::PatternComprehension {
                variable: Some("p".to_string()),
                pattern: Pattern::default(),
                filter: None,
                projection: Box::new(AstExpr::Variable("p".into())),
            },
            AstExpr::Quantifier {
                kind: QuantifierKind::Single,
                variable: "x".to_string(),
                list: Box::new(AstExpr::List(Vec::new())),
                filter: Box::new(AstExpr::Bool(true)),
            },
            AstExpr::ListSlice {
                list: Box::new(AstExpr::List(Vec::new())),
                from: Some(Box::new(AstExpr::Int(1))),
                to: None,
            },
            AstExpr::StringOp {
                op: StringOp::StartsWith,
                left: Box::new(AstExpr::String("a".into())),
                right: Box::new(AstExpr::String("b".into())),
            },
            AstExpr::CountAll,
        ];
        assert_eq!(expressions.len(), 7);
    }
}
