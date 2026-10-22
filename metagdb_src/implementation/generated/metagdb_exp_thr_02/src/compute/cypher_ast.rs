//! Private Cypher AST.
//!
//! The AST is produced by lowering the repository-generated ANTLR CST and is
//! the only input the planner accepts.  It is deliberately free of ANTLR
//! lifetimes and of framework physical types so it can be shaped, stored and
//! moved independently of the parser.

use std::any::Any;
use std::collections::BTreeMap;

use metagdb_core::common::property::PropertyValue;
use metagdb_core::l4_compute::engine::LanguageAst;
use metagdb_core::l5_interaction::QueryLanguage;

/// Top-level Cypher statement.
#[derive(Debug, Clone)]
pub enum CypherAst {
    /// A query: one or more UNION branches of a clause pipeline.
    Query(Query),
    /// A standalone procedure call with no surrounding query.
    Call {
        name: String,
        args: Vec<ProcArg>,
        yields: Option<YieldSpec>,
    },
}

impl LanguageAst for CypherAst {
    fn language(&self) -> QueryLanguage {
        QueryLanguage::Cypher
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A query with one or more UNION branches.
#[derive(Debug, Clone)]
pub struct Query {
    /// `(branch, all)` pairs; `all` is only meaningful from the second branch.
    pub branches: Vec<(Vec<Clause>, bool)>,
}

/// One clause of a linear pipeline.
#[derive(Debug, Clone)]
pub enum Clause {
    Match {
        optional: bool,
        pattern: Pattern,
        predicate: Option<Expr>,
    },
    With {
        distinct: bool,
        items: Vec<ReturnItem>,
        order: Vec<SortItem>,
        skip: Option<Expr>,
        limit: Option<Expr>,
        predicate: Option<Expr>,
    },
    Return {
        distinct: bool,
        items: Vec<ReturnItem>,
        order: Vec<SortItem>,
        skip: Option<Expr>,
        limit: Option<Expr>,
    },
    Unwind {
        expr: Expr,
        var: String,
    },
    Create {
        pattern: Pattern,
    },
    Merge {
        pattern: Pattern,
        on_create: Vec<SetItem>,
        on_match: Vec<SetItem>,
    },
    Set {
        items: Vec<SetItem>,
    },
    Remove {
        items: Vec<RemoveItem>,
    },
    Delete {
        detach: bool,
        exprs: Vec<Expr>,
    },
    /// `CALL proc(args) [YIELD ...] [WHERE ...]`
    Call {
        name: String,
        args: Vec<ProcArg>,
        yields: Option<YieldSpec>,
    },
}

/// Named or positional procedure argument.
#[derive(Debug, Clone)]
pub enum ProcArg {
    Named(String, Expr),
    Positional(Expr),
}

/// `YIELD` mapping: `*` or an explicit field list plus an optional predicate.
#[derive(Debug, Clone)]
pub struct YieldSpec {
    pub star: bool,
    pub items: Vec<(String, String)>,
    pub predicate: Option<Expr>,
}

/// A `RETURN`/`WITH` projection item.
#[derive(Debug, Clone)]
pub struct ReturnItem {
    pub expr: Expr,
    pub alias: Option<String>,
}

/// One `ORDER BY` key.
#[derive(Debug, Clone)]
pub struct SortItem {
    pub expr: Expr,
    pub descending: bool,
}

/// One pattern (possibly several comma-separated parts).
#[derive(Debug, Clone)]
pub struct Pattern {
    pub parts: Vec<PatternPart>,
}

/// A single pattern part: an element, optionally bound to a path variable.
#[derive(Debug, Clone)]
pub struct PatternPart {
    pub path_var: Option<String>,
    pub element: PatternElement,
}

/// A node followed by zero or more relationship/node chains.
#[derive(Debug, Clone)]
pub struct PatternElement {
    pub node: NodePattern,
    pub chains: Vec<(EdgePattern, NodePattern)>,
}

/// Node pattern: `(var:Label:Label2 {props})`.
#[derive(Debug, Clone)]
pub struct NodePattern {
    pub var: Option<String>,
    pub labels: Vec<String>,
    pub properties: Option<Expr>,
    /// Set by the optimizer when a `WHERE id(var) = <literal>` conjunct can be
    /// pushed into this node, turning a full scan into a point lookup.
    pub id_hint: Option<u64>,
}

/// Relationship pattern.
#[derive(Debug, Clone)]
pub struct EdgePattern {
    pub var: Option<String>,
    pub types: Vec<String>,
    pub direction: Direction,
    pub range: Option<RangeLiteral>,
    pub properties: Option<Expr>,
}

/// Relationship direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

/// Variable-length relationship range.
#[derive(Debug, Clone, Copy)]
pub struct RangeLiteral {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

/// `SET` item.
#[derive(Debug, Clone)]
pub enum SetItem {
    Property {
        target: Expr,
        key: String,
        value: Expr,
    },
    Labels {
        var: String,
        labels: Vec<String>,
    },
}

/// `REMOVE` item.
#[derive(Debug, Clone)]
pub enum RemoveItem {
    Property { target: Expr, key: String },
    Labels { var: String, labels: Vec<String> },
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
}

/// Quantifier kind for `all`/`any`/`none`/`single`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Any,
    None,
    Single,
}

/// Scalar expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Parameter(String),
    Variable(String),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    /// Property access `base.key` (also used for temporal field accessors).
    Property(Box<Expr>, String),
    /// List subscript `base[index]`.
    Index(Box<Expr>, Box<Expr>),
    /// List slice `base[start..end]`.
    Slice {
        base: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    Negate(Box<Expr>),
    Not(Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    In(Box<Expr>, Box<Expr>),
    StartsWith(Box<Expr>, Box<Expr>),
    EndsWith(Box<Expr>, Box<Expr>),
    Contains(Box<Expr>, Box<Expr>),
    /// `EXISTS` over a node/relationship pattern.
    PatternExists(Pattern),
    /// `base:Label` label predicate.
    HasLabels(Box<Expr>, Vec<String>),
    /// `[var IN list WHERE filter | map]`
    ListComprehension {
        var: String,
        list: Box<Expr>,
        filter: Option<Box<Expr>>,
        map: Option<Box<Expr>>,
    },
    /// `all|any|none|single(var IN list WHERE filter)`
    Quantifier {
        kind: Quantifier,
        var: String,
        list: Box<Expr>,
        filter: Option<Box<Expr>>,
    },
    Case {
        subject: Option<Box<Expr>>,
        alternatives: Vec<(Expr, Expr)>,
        default: Option<Box<Expr>>,
    },
    Function {
        name: String,
        distinct: bool,
        args: Vec<Expr>,
    },
    /// `COUNT(*)`
    CountAll,
}

impl Expr {
    /// Whether the expression tree contains an aggregate call.
    pub fn is_aggregate(&self) -> bool {
        match self {
            Expr::CountAll => true,
            Expr::Function { name, args, .. } => {
                is_aggregate_name(name) || args.iter().any(Expr::is_aggregate)
            }
            Expr::Property(base, _) => base.is_aggregate(),
            Expr::Index(a, b) => a.is_aggregate() || b.is_aggregate(),
            Expr::Slice { base, start, end } => {
                base.is_aggregate()
                    || start.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
                    || end.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
            }
            Expr::Negate(e) | Expr::Not(e) | Expr::IsNull(e) | Expr::IsNotNull(e) => e.is_aggregate(),
            Expr::Binary(_, a, b) | Expr::In(a, b) | Expr::StartsWith(a, b)
            | Expr::EndsWith(a, b) | Expr::Contains(a, b) => a.is_aggregate() || b.is_aggregate(),
            Expr::HasLabels(e, _) => e.is_aggregate(),
            Expr::List(items) => items.iter().any(Expr::is_aggregate),
            Expr::Map(entries) => entries.iter().any(|(_, value)| value.is_aggregate()),
            Expr::ListComprehension { list, filter, map, .. } => {
                list.is_aggregate()
                    || filter.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
                    || map.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
            }
            Expr::Quantifier { list, filter, .. } => {
                list.is_aggregate()
                    || filter.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
            }
            Expr::Case {
                subject,
                alternatives,
                default,
            } => {
                subject.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
                    || alternatives
                        .iter()
                        .any(|(a, b)| a.is_aggregate() || b.is_aggregate())
                    || default.as_ref().map(|e| e.is_aggregate()).unwrap_or(false)
            }
            _ => false,
        }
    }

    /// Whether the expression is a plain variable reference.
    pub fn as_variable(&self) -> Option<&str> {
        match self {
            Expr::Variable(name) => Some(name),
            _ => None,
        }
    }
}

/// Aggregate function names recognised by the planner.
pub fn is_aggregate_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "count" | "sum" | "avg" | "min" | "max" | "collect" | "stdev" | "percentilecont"
    )
}

/// Algorithm arguments after lowering: named scalars only.
pub type NamedArgs = BTreeMap<String, PropertyValue>;
