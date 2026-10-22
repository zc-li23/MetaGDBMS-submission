//! Candidate-owned physical operator tree and DML payloads.
//!
//! `metagdb_core` deliberately keeps `PhysicalPlan` opaque: an implementation
//! lowers the framework's `LogicalOp` IR into its own executable tree and
//! downcasts it inside `Executor::execute`.  This module is that tree.
//!
//! The payload types are framework types (`LabelId`, `EdgeTypeId`,
//! `PropertyKeyId`, `Expr`, `AggregateExpr`, `AlgorithmArgs`, `ProjectionSpec`,
//! `ResultValue`, …) rather than candidate redefinitions, so the physical layer
//! stays a lowering of the shared IR instead of a parallel one.  Variable
//! bindings are addressed by name, matching `LogicalOp`, and the executor
//! resolves them to row slots once per query.

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, JoinKind, ProjectionDirection, ProjectionSpec, SortOrder,
};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};

/// One row of a push-based execution stream, addressed by slot.
pub type Row = Vec<ResultValue>;

/// A single graph or property mutation performed by a write operator.
#[derive(Debug, Clone)]
pub enum WriteOp {
    /// Create one vertex; the planner binds it to `var` after insertion.
    CreateVertex {
        var: String,
        labels: Vec<LabelId>,
        properties: Vec<(PropertyKeyId, Expr)>,
    },
    /// Create one edge between two already bound endpoints.
    CreateEdge {
        from: String,
        to: String,
        etype: EdgeTypeId,
        properties: Vec<(PropertyKeyId, Expr)>,
        var: Option<String>,
        reversed: bool,
    },
    /// Overwrite one property value on a bound element.
    SetProperty {
        target: Expr,
        key: PropertyKeyId,
        value: Expr,
    },
    /// Add labels to a bound vertex.
    SetLabels {
        target: Expr,
        labels: Vec<LabelId>,
    },
    /// Remove one property from a bound element.
    RemoveProperty { target: Expr, key: PropertyKeyId },
    /// Remove labels from a bound vertex.
    RemoveLabels {
        target: Expr,
        labels: Vec<LabelId>,
    },
    /// Delete the element bound to `target`; `detach` also removes its edges.
    Delete { target: Expr, detach: bool },
}

/// Executable operator tree produced by [`crate::optimizer::MetagdbOptimizer`].
#[derive(Debug, Clone)]
pub enum PhysicalOp {
    /// A single empty row: the seed for pattern-free statements.
    SingleRow,
    /// Scan every visible vertex, optionally restricted to one label.
    VertexScan {
        var: String,
        label: Option<LabelId>,
    },
    /// Read one vertex by its stable ID.
    ///
    /// This is the physical form of `WHERE id(v) = <literal>`: the storage
    /// engine maintains a vertex-ID index, so the general scan is only needed
    /// when the predicate cannot be folded into a point lookup.
    VertexById {
        var: String,
        id: u64,
    },
    /// Scan every visible edge, optionally restricted to one type.
    EdgeScan {
        var: String,
        etype: Option<EdgeTypeId>,
    },
    /// Build the storage-backed analytical projection of the current graph.
    GraphProjection { spec: ProjectionSpec },
    /// Traverse from an already bound vertex to its neighbours.
    Expand {
        input: Box<PhysicalOp>,
        from: String,
        to: String,
        edge: Option<String>,
        direction: ExpandDirection,
        edge_types: Option<Vec<EdgeTypeId>>,
    },
    /// Keep only the rows whose predicate is `true`.
    Filter {
        predicate: Expr,
        input: Box<PhysicalOp>,
    },
    /// Evaluate projected expressions into named output columns.
    Project {
        items: Vec<(Expr, String)>,
        input: Box<PhysicalOp>,
    },
    /// Group rows and evaluate the aggregate expressions per group.
    Aggregate {
        group_by: Vec<Expr>,
        aggs: Vec<(AggregateExpr, String)>,
        input: Box<PhysicalOp>,
    },
    /// Remove duplicate rows by projected key.
    Distinct { input: Box<PhysicalOp> },
    /// Order rows by the requested keys.
    Sort {
        keys: Vec<(Expr, SortOrder)>,
        input: Box<PhysicalOp>,
    },
    /// Skip `offset` rows and emit at most `limit`.
    Limit {
        limit: usize,
        offset: usize,
        input: Box<PhysicalOp>,
    },
    /// Expand a list-valued expression into one row per element.
    Unwind {
        list: Expr,
        alias: String,
        input: Box<PhysicalOp>,
    },
    /// Concatenate the inputs, deduplicating unless `all`.
    Union {
        all: bool,
        inputs: Vec<PhysicalOp>,
    },
    /// Join two independent sub-plans.
    Join {
        kind: JoinKind,
        on: Vec<(Expr, Expr)>,
        left: Box<PhysicalOp>,
        right: Box<PhysicalOp>,
    },
    /// Perform the statement's mutations, then stream the projection rows.
    Mutate {
        writes: Vec<WriteOp>,
        input: Box<PhysicalOp>,
        projection: Vec<(Expr, String)>,
    },
    /// Traverse a relationship pattern of variable length along `min..max`
    /// hops, emitting one row per path and optionally binding the path.
    VarExpand {
        input: Box<PhysicalOp>,
        from: String,
        to: String,
        edge: Option<String>,
        path: Option<String>,
        direction: ExpandDirection,
        edge_types: Option<Vec<EdgeTypeId>>,
        min: u32,
        max: Option<u32>,
    },
    /// For every input row: probe `probe`; when it yields nothing, run the
    /// `create` writes.  `actions` are the `ON MATCH` / `ON CREATE` writes,
    /// keyed by whether they apply when the pattern matched.
    Merge {
        input: Box<PhysicalOp>,
        probe: Box<PhysicalOp>,
        create: Vec<WriteOp>,
        actions: Vec<(bool, Vec<WriteOp>)>,
    },
    /// Build the analytical projection and run a registered algorithm.
    CallAlgorithm {
        name: String,
        args: AlgorithmArgs,
        spec: ProjectionSpec,
        input: Box<PhysicalOp>,
    },
    /// Reject statements the candidate deliberately does not implement.
    Unsupported { reason: String },
}

/// Internal column name the executor gives the `index`-th grouping key.
///
/// `Aggregate` emits its grouping keys positionally; the optimizer renames them
/// with this convention so the projection above the aggregate can address each
/// key by name without the framework IR needing named group keys.
/// Direction the candidate builds every analytical projection with.
///
/// `Outgoing` for all three algorithms: BFS and PageRank are directed by
/// definition, and WCC symmetrizes whatever adjacency it is given, so one
/// projection shape serves the language path and the fixed kernel benchmark
/// alike.
pub fn default_projection_direction() -> ProjectionDirection {
    ProjectionDirection::Outgoing
}

pub fn group_key_name(index: usize) -> String {
    format!("$group{index}")
}

/// Output column shape of a physical plan, used by the executor to size rows.
#[derive(Debug, Clone, Default)]
pub struct PlanShape {
    pub columns: Vec<String>,
}

impl PhysicalOp {
    /// Collect every variable name bound anywhere in the operator tree.
    pub fn bound_variables(&self, out: &mut Vec<String>) {
        match self {
            PhysicalOp::SingleRow | PhysicalOp::Unsupported { .. } => {}
            PhysicalOp::VertexScan { var, .. }
            | PhysicalOp::EdgeScan { var, .. }
            | PhysicalOp::VertexById { var, .. } => {
                push_unique(out, var);
            }
            PhysicalOp::Expand {
                input,
                from,
                to,
                edge,
                ..
            }
            | PhysicalOp::VarExpand {
                input,
                from,
                to,
                edge,
                ..
            } => {
                input.bound_variables(out);
                push_unique(out, from);
                push_unique(out, to);
                if let Some(edge) = edge {
                    push_unique(out, edge);
                }
            }
            PhysicalOp::GraphProjection { .. } => {}
            PhysicalOp::Filter { input, .. }
            | PhysicalOp::Distinct { input }
            | PhysicalOp::Limit { input, .. } => input.bound_variables(out),
            PhysicalOp::Project { items, input } => {
                input.bound_variables(out);
                for (_, alias) in items {
                    push_unique(out, alias);
                }
            }
            PhysicalOp::Aggregate {
                group_by,
                aggs,
                input,
            } => {
                input.bound_variables(out);
                for (_, alias) in aggs {
                    push_unique(out, alias);
                }
                let _ = group_by;
            }
            PhysicalOp::Sort { input, .. } => input.bound_variables(out),
            PhysicalOp::Unwind { alias, input, .. } => {
                input.bound_variables(out);
                push_unique(out, alias);
            }
            PhysicalOp::Union { inputs, .. } => {
                for input in inputs {
                    input.bound_variables(out);
                }
            }
            PhysicalOp::Join { left, right, .. } => {
                left.bound_variables(out);
                right.bound_variables(out);
            }
            PhysicalOp::Mutate {
                input, projection, ..
            } => {
                input.bound_variables(out);
                for (_, alias) in projection {
                    push_unique(out, alias);
                }
            }
            PhysicalOp::Merge { input, probe, .. } => {
                input.bound_variables(out);
                probe.bound_variables(out);
            }
            PhysicalOp::CallAlgorithm { input, .. } => input.bound_variables(out),
        }
    }
}

fn push_unique(out: &mut Vec<String>, name: &str) {
    if !out.iter().any(|existing| existing == name) {
        out.push(name.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_are_collected_once_per_name() {
        let plan = PhysicalOp::Expand {
            input: Box::new(PhysicalOp::VertexScan {
                var: "a".into(),
                label: None,
            }),
            from: "a".into(),
            to: "b".into(),
            edge: Some("r".into()),
            direction: ExpandDirection::Outgoing,
            edge_types: None,
        };
        let mut names = Vec::new();
        plan.bound_variables(&mut names);
        assert_eq!(names, vec!["a".to_string(), "b".into(), "r".into()]);
    }

    #[test]
    fn grouping_keys_have_a_stable_internal_name() {
        assert_eq!(group_key_name(0), "$group0");
        assert_eq!(group_key_name(3), "$group3");
    }

    #[test]
    fn write_payloads_reuse_framework_identifiers() {
        let op = WriteOp::CreateVertex {
            var: "n".into(),
            labels: vec![LabelId(1)],
            properties: vec![(PropertyKeyId(3), Expr::Int(7))],
        };
        match op {
            WriteOp::CreateVertex { labels, properties, .. } => {
                assert_eq!(labels[0], LabelId(1));
                assert_eq!(properties[0].0, PropertyKeyId(3));
            }
            _ => unreachable!("constructed a create payload"),
        }
    }
}
