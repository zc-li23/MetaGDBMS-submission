//! Private Cypher AST. Produced by CST lowering, consumed by the planner.
//!
//! This tree has no ANTLR lifetimes and keeps names as strings; the planner is
//! responsible for resolving them through `GraphSchema`.

use std::any::Any;

use metagdb_core::l4_compute::engine::LanguageAst;
use metagdb_core::l5_interaction::QueryLanguage;

#[derive(Debug, Clone, PartialEq)]
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
    In,
    StartsWith,
    EndsWith,
    Contains,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuantKind {
    All,
    Any,
    None,
    Single,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Parameter(String),
    Var(String),
    Prop(Box<Expr>, String),
    CountStar,
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    Neg(Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    ListIndex(Box<Expr>, Box<Expr>),
    ListSlice {
        list: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    Function {
        name: String,
        distinct: bool,
        args: Vec<Expr>,
    },
    Case {
        test: Option<Box<Expr>>,
        whens: Vec<(Expr, Expr)>,
        else_branch: Option<Box<Expr>>,
    },
    ListComprehension {
        var: String,
        list: Box<Expr>,
        filter: Option<Box<Expr>>,
        map: Option<Box<Expr>>,
    },
    Quantifier {
        kind: QuantKind,
        var: String,
        list: Box<Expr>,
        predicate: Option<Box<Expr>>,
    },
    LabelTest(Box<Expr>, Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

#[derive(Debug, Clone)]
pub struct NodePattern {
    pub var: Option<String>,
    pub labels: Vec<String>,
    pub properties: Vec<(String, Expr)>,
}

#[derive(Debug, Clone)]
pub struct RelPattern {
    pub var: Option<String>,
    pub types: Vec<String>,
    pub direction: Direction,
    /// Variable-length walk bounds. `(1, Some(1))` is a single hop.
    pub min_hops: i64,
    pub max_hops: Option<i64>,
    pub properties: Vec<(String, Expr)>,
}

#[derive(Debug, Clone)]
pub struct PatternElement {
    pub node: NodePattern,
    pub chains: Vec<(RelPattern, NodePattern)>,
}

#[derive(Debug, Clone)]
pub struct PatternPart {
    /// `Some(name)` for `name = (a)-[...]->(b)` path bindings.
    pub path_var: Option<String>,
    pub element: PatternElement,
}

#[derive(Debug, Clone)]
pub struct MatchClause {
    pub optional: bool,
    pub parts: Vec<PatternPart>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct ProjectionItem {
    pub expr: Expr,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectionBody {
    pub distinct: bool,
    pub items: Vec<ProjectionItem>,
    pub order_by: Vec<(Expr, bool)>,
    pub skip: Option<Expr>,
    pub limit: Option<Expr>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum SetItem {
    Property { target: Expr, value: Expr },
    Variable { name: String, value: Expr },
    Merge { name: String, value: Expr },
    Labels { name: String, labels: Vec<String> },
}

#[derive(Debug, Clone)]
pub enum RemoveItem {
    Property(Expr),
    Labels { name: String, labels: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct MergeClause {
    pub part: PatternPart,
    pub on_create: Vec<SetItem>,
    pub on_match: Vec<SetItem>,
}

#[derive(Debug, Clone)]
pub struct CallClause {
    pub name: String,
    pub args: Vec<(Option<String>, Expr)>,
    pub yields: Vec<(String, String)>,
    pub yield_star: bool,
}

#[derive(Debug, Clone)]
pub enum Clause {
    Match(MatchClause),
    With(ProjectionBody),
    Return(ProjectionBody),
    Unwind { expr: Expr, var: String },
    Create(Vec<PatternPart>),
    Merge(MergeClause),
    Delete { detach: bool, exprs: Vec<Expr> },
    Set(Vec<SetItem>),
    Remove(Vec<RemoveItem>),
    Call(CallClause),
}

#[derive(Debug, Clone)]
pub struct Union {
    pub all: bool,
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub head: Vec<Clause>,
    pub unions: Vec<Union>,
}

/// Language AST handed to the planner.
#[derive(Debug, Clone)]
pub struct CypherAst {
    pub query: Query,
}

impl LanguageAst for CypherAst {
    fn language(&self) -> QueryLanguage {
        QueryLanguage::Cypher
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
