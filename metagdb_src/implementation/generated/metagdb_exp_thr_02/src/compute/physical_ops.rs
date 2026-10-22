//! Candidate physical operators and the logical-plan extension payloads that
//! carry Cypher semantics between the planner and the optimizer.
//!
//! The framework's `LogicalOp` covers relational shape (sources, union, OLAP
//! invocation) but not Cypher's pattern, projection and mutation operators, so
//! those travel as named `ExtensionOp` payloads.  The optimizer lowers every one
//! of them into a variant of `PhysicalOp`, and the executor has a matching
//! evaluator for each variant.

use metagdb_core::l4_compute::algebra::ProjectionSpec;

use super::cypher_ast::*;

/// Payload of `cypher.match`.
#[derive(Debug, Clone)]
pub struct MatchPayload {
    pub optional: bool,
    pub pattern: Pattern,
    pub predicate: Option<Expr>,
}

/// Payload of `cypher.project` (`RETURN` and `WITH` share one operator).
#[derive(Debug, Clone)]
pub struct ProjectPayload {
    pub is_return: bool,
    pub distinct: bool,
    pub items: Vec<ReturnItem>,
    pub order: Vec<SortItem>,
    pub skip: Option<Expr>,
    pub limit: Option<Expr>,
    pub predicate: Option<Expr>,
}

/// Payload of `cypher.unwind`.
#[derive(Debug, Clone)]
pub struct UnwindPayload {
    pub expr: Expr,
    pub var: String,
}

/// Payload of the updating operators.
#[derive(Debug, Clone)]
pub enum MutationPayload {
    Create(Pattern),
    Merge {
        pattern: Pattern,
        on_create: Vec<SetItem>,
        on_match: Vec<SetItem>,
    },
    Set(Vec<SetItem>),
    Remove(Vec<RemoveItem>),
    Delete {
        detach: bool,
        exprs: Vec<Expr>,
    },
}

/// Payload of `cypher.call`.
#[derive(Debug, Clone)]
pub struct CallPayload {
    pub name: String,
    pub args: Vec<ProcArg>,
    pub yields: Option<YieldSpec>,
}

/// Executable physical operator tree.
#[derive(Debug, Clone)]
pub enum PhysicalOp {
    /// A single empty row; the source of every pipeline.
    SingleRow,
    /// A pipeline that produces no rows.
    Empty,
    Match {
        optional: bool,
        pattern: Pattern,
        predicate: Option<Expr>,
        input: Box<PhysicalOp>,
    },
    Unwind {
        expr: Expr,
        var: String,
        input: Box<PhysicalOp>,
    },
    Project {
        is_return: bool,
        distinct: bool,
        items: Vec<ReturnItem>,
        order: Vec<SortItem>,
        skip: Option<Expr>,
        limit: Option<Expr>,
        predicate: Option<Expr>,
        input: Box<PhysicalOp>,
    },
    Create {
        pattern: Pattern,
        input: Box<PhysicalOp>,
    },
    Merge {
        pattern: Pattern,
        on_create: Vec<SetItem>,
        on_match: Vec<SetItem>,
        input: Box<PhysicalOp>,
    },
    Set {
        items: Vec<SetItem>,
        input: Box<PhysicalOp>,
    },
    Remove {
        items: Vec<RemoveItem>,
        input: Box<PhysicalOp>,
    },
    Delete {
        detach: bool,
        exprs: Vec<Expr>,
        input: Box<PhysicalOp>,
    },
    Union {
        all: bool,
        inputs: Vec<PhysicalOp>,
    },
    BuildProjection {
        spec: ProjectionSpec,
    },
    RunAlgorithm {
        name: String,
        params: std::collections::BTreeMap<String, metagdb_core::common::property::PropertyValue>,
        input: Box<PhysicalOp>,
    },
    /// Renames/yields algorithm output columns.
    Yield {
        spec: YieldSpec,
        input: Box<PhysicalOp>,
    },
    /// In-query `CALL` evaluated once per incoming row.
    Call {
        name: String,
        args: Vec<ProcArg>,
        yields: Option<YieldSpec>,
        input: Box<PhysicalOp>,
    },
}

impl PhysicalOp {
    /// Human-readable operator name, used by `EXPLAIN` output and plan tests.
    pub fn operator_name(&self) -> &'static str {
        match self {
            PhysicalOp::SingleRow => "SingleRow",
            PhysicalOp::Empty => "Empty",
            PhysicalOp::Match { .. } => "CypherMatch",
            PhysicalOp::Unwind { .. } => "CypherUnwind",
            PhysicalOp::Project { is_return, .. } => {
                if *is_return {
                    "CypherReturn"
                } else {
                    "CypherWith"
                }
            }
            PhysicalOp::Create { .. } => "CypherCreate",
            PhysicalOp::Merge { .. } => "CypherMerge",
            PhysicalOp::Set { .. } => "CypherSet",
            PhysicalOp::Remove { .. } => "CypherRemove",
            PhysicalOp::Delete { .. } => "CypherDelete",
            PhysicalOp::Union { .. } => "Union",
            PhysicalOp::BuildProjection { .. } => "BuildProjection",
            PhysicalOp::RunAlgorithm { .. } => "RunAlgorithm",
            PhysicalOp::Yield { .. } => "CypherYield",
            PhysicalOp::Call { .. } => "CypherCall",
        }
    }

    /// Depth-first child operators, for plan-shape assertions.
    pub fn children(&self) -> Vec<&PhysicalOp> {
        match self {
            PhysicalOp::Match { input, .. }
            | PhysicalOp::Unwind { input, .. }
            | PhysicalOp::Project { input, .. }
            | PhysicalOp::Create { input, .. }
            | PhysicalOp::Merge { input, .. }
            | PhysicalOp::Set { input, .. }
            | PhysicalOp::Remove { input, .. }
            | PhysicalOp::Delete { input, .. }
            | PhysicalOp::RunAlgorithm { input, .. }
            | PhysicalOp::Yield { input, .. }
            | PhysicalOp::Call { input, .. } => vec![input],
            PhysicalOp::Union { inputs, .. } => inputs.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Whether this plan's root is the final projection of a query.
    pub fn is_return_root(&self) -> bool {
        matches!(self, PhysicalOp::Project { is_return: true, .. })
    }
}
