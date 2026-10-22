//! Logical plan: the language-level clause pipeline with interned identifiers.
//!
//! Produced by the planner (AST → logical plan), lowered to the executable
//! physical pipeline by the optimizer.

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};

use super::ast::Direction;
use super::ir::BExpr;

#[derive(Clone, Debug)]
pub struct BoundNode {
    pub var: Option<String>,
    pub labels: Vec<LabelId>,
    pub properties: Vec<(PropertyKeyId, BExpr)>,
}

#[derive(Clone, Debug)]
pub struct BoundRel {
    pub var: Option<String>,
    pub types: Vec<EdgeTypeId>,
    pub direction: Direction,
    pub min_hops: i64,
    pub max_hops: Option<i64>,
    pub properties: Vec<(PropertyKeyId, BExpr)>,
}

#[derive(Clone, Debug)]
pub struct BoundElement {
    pub node: BoundNode,
    pub chains: Vec<(BoundRel, BoundNode)>,
}

#[derive(Clone, Debug)]
pub struct BoundPatternPart {
    pub path_var: Option<String>,
    pub element: BoundElement,
}

#[derive(Clone, Debug)]
pub struct MatchStage {
    pub optional: bool,
    pub parts: Vec<BoundPatternPart>,
    pub where_clause: Option<BExpr>,
    /// Optimizer hint: `(variable, vertex id)` extracted from an `id(v) = k`
    /// predicate, letting the anchor use a point lookup instead of a scan.
    pub id_hint: Option<(String, u64)>,
    /// Optimizer hint: `(variable, vertex id)` for the node at the end of a
    /// single variable-length chain.  It is set only together with
    /// `shortest_path`, and lets the executor stop once that node is reached.
    pub end_hint: Option<(String, u64)>,
    /// Set when the only use of the path variable is `min(length(path))`.
    /// The executor may then answer the pattern with a target-aware BFS
    /// instead of enumerating every trail.
    pub shortest_path: bool,
}

#[derive(Clone, Debug)]
pub struct ProjectItem {
    pub expr: BExpr,
    /// Output column name.
    pub alias: String,
    /// `true` for `RETURN *`: expand every currently bound variable.
    pub star: bool,
}

#[derive(Clone, Debug)]
pub struct ProjectStage {
    pub distinct: bool,
    pub items: Vec<ProjectItem>,
    pub order_by: Vec<(BExpr, bool)>,
    pub skip: Option<BExpr>,
    pub limit: Option<BExpr>,
    pub where_clause: Option<BExpr>,
}

#[derive(Clone, Debug)]
pub enum BoundSetItem {
    Property {
        element: BExpr,
        key: PropertyKeyId,
        value: BExpr,
    },
    Variable {
        name: String,
        value: BExpr,
    },
    Merge {
        name: String,
        value: BExpr,
    },
    Labels {
        name: String,
        labels: Vec<LabelId>,
    },
}

#[derive(Clone, Debug)]
pub enum BoundRemoveItem {
    Property {
        element: BExpr,
        key: PropertyKeyId,
    },
    Labels {
        name: String,
        labels: Vec<LabelId>,
    },
}

#[derive(Clone, Debug)]
pub struct MergeStage {
    pub part: BoundPatternPart,
    pub on_create: Vec<BoundSetItem>,
    pub on_match: Vec<BoundSetItem>,
}

#[derive(Clone, Debug)]
pub struct CallStage {
    pub name: String,
    pub args: Vec<(String, BExpr)>,
    pub yields: Vec<(String, String)>,
    pub yield_star: bool,
}

#[derive(Clone, Debug)]
pub enum Stage {
    Match(MatchStage),
    Unwind { expr: BExpr, var: String },
    Project(ProjectStage),
    Create(Vec<BoundPatternPart>),
    Merge(MergeStage),
    Delete { detach: bool, exprs: Vec<BExpr> },
    Set(Vec<BoundSetItem>),
    Remove(Vec<BoundRemoveItem>),
    Call(CallStage),
}

/// A complete logical query: a head pipeline plus UNION branches.
#[derive(Clone, Debug)]
pub struct LogicalQuery {
    pub stages: Vec<Stage>,
    pub unions: Vec<(bool, Vec<Stage>)>,
}
