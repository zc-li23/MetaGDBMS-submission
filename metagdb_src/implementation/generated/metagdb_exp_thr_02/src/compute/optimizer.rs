//! Physical-plan lowering and semantics-preserving optimization.
//!
//! Every logical operator the planner can emit is lowered to a physical
//! operator.  The single rewrite applied here pushes a `WHERE id(v) = <literal>`
//! conjunct into the node pattern it constrains, which turns the full vertex
//! scan of a point lookup into a direct redirection-index lookup without
//! changing the result set.

use metagdb_core::l4_compute::algebra::{LogicalOp, LogicalPlan, PhysicalPlan};
use metagdb_core::l4_compute::engine::{OptContext, Optimizer};
use metagdb_core::{Error, Result};

use super::cypher_ast::*;
use super::physical_ops::*;

/// Framework optimizer producing the candidate's physical operator tree.
#[derive(Default)]
pub struct MetagdbOptimizer;

impl Optimizer for MetagdbOptimizer {
    fn optimize(&self, lp: LogicalPlan, ctx: &OptContext) -> Result<PhysicalPlan> {
        let physical = lower(lp.root, ctx)?;
        Ok(PhysicalPlan::new(physical))
    }
}

fn lower(op: LogicalOp, ctx: &OptContext) -> Result<PhysicalOp> {
    match op {
        LogicalOp::SingleRow => Ok(PhysicalOp::SingleRow),
        LogicalOp::Union { all, inputs } => Ok(PhysicalOp::Union {
            all,
            inputs: inputs
                .into_iter()
                .map(|input| lower(input, ctx))
                .collect::<Result<Vec<_>>>()?,
        }),
        LogicalOp::GraphProjection { spec } => Ok(PhysicalOp::BuildProjection { spec }),
        LogicalOp::CallAlgorithm { name, args, input } => {
            if !ctx.algorithms.algorithms.contains_key(&name) {
                return Err(Error::Plan(format!(
                    "algorithm `{name}` is not registered in this runtime"
                )));
            }
            Ok(PhysicalOp::RunAlgorithm {
                name,
                params: args.params,
                input: Box::new(lower(*input, ctx)?),
            })
        }
        LogicalOp::ExtensionOp {
            name, payload, input,
        } => lower_extension(&name, payload, *input, ctx),
        other => Err(Error::Unsupported(format!(
            "logical operator {} is not produced by this planner",
            describe(&other)
        ))),
    }
}

fn lower_extension(
    name: &str,
    payload: metagdb_core::l4_compute::algebra::OpaquePayload,
    input: LogicalOp,
    ctx: &OptContext,
) -> Result<PhysicalOp> {
    let input = Box::new(lower(input, ctx)?);
    match name {
        "cypher.match" => {
            let mut match_payload = payload
                .downcast_ref::<MatchPayload>()
                .ok_or_else(|| Error::Optimize("malformed cypher.match payload".into()))?
                .clone();
            push_id_hints(&mut match_payload);
            Ok(PhysicalOp::Match {
                optional: match_payload.optional,
                pattern: match_payload.pattern,
                predicate: match_payload.predicate,
                input,
            })
        }
        "cypher.project" => {
            let project = payload
                .downcast_ref::<ProjectPayload>()
                .ok_or_else(|| Error::Optimize("malformed cypher.project payload".into()))?
                .clone();
            Ok(PhysicalOp::Project {
                is_return: project.is_return,
                distinct: project.distinct,
                items: project.items,
                order: project.order,
                skip: project.skip,
                limit: project.limit,
                predicate: project.predicate,
                input,
            })
        }
        "cypher.unwind" => {
            let unwind = payload
                .downcast_ref::<UnwindPayload>()
                .ok_or_else(|| Error::Optimize("malformed cypher.unwind payload".into()))?
                .clone();
            Ok(PhysicalOp::Unwind {
                expr: unwind.expr,
                var: unwind.var,
                input,
            })
        }
        "cypher.mutate" => {
            let mutation = payload
                .downcast_ref::<MutationPayload>()
                .ok_or_else(|| Error::Optimize("malformed cypher.mutate payload".into()))?
                .clone();
            Ok(match mutation {
                MutationPayload::Create(pattern) => PhysicalOp::Create { pattern, input },
                MutationPayload::Merge {
                    pattern,
                    on_create,
                    on_match,
                } => PhysicalOp::Merge {
                    pattern,
                    on_create,
                    on_match,
                    input,
                },
                MutationPayload::Set(items) => PhysicalOp::Set { items, input },
                MutationPayload::Remove(items) => PhysicalOp::Remove { items, input },
                MutationPayload::Delete { detach, exprs } => PhysicalOp::Delete {
                    detach,
                    exprs,
                    input,
                },
            })
        }
        "cypher.yield" => {
            let spec = payload
                .downcast_ref::<YieldSpec>()
                .ok_or_else(|| Error::Optimize("malformed cypher.yield payload".into()))?
                .clone();
            Ok(PhysicalOp::Yield { spec, input })
        }
        "cypher.call" => {
            let call = payload
                .downcast_ref::<CallPayload>()
                .ok_or_else(|| Error::Optimize("malformed cypher.call payload".into()))?
                .clone();
            if !ctx.algorithms.algorithms.contains_key(&call.name) {
                return Err(Error::Plan(format!(
                    "algorithm `{}` is not registered in this runtime",
                    call.name
                )));
            }
            Ok(PhysicalOp::Call {
                name: call.name,
                args: call.args,
                yields: call.yields,
                input,
            })
        }
        other => Err(Error::Unsupported(format!(
            "logical extension operator `{other}` has no physical lowering"
        ))),
    }
}

/// Push `id(v) = <positive literal>` conjuncts into the matching node pattern.
///
/// This is semantics-preserving: the hint only limits the candidate set to the
/// single vertex the predicate already required, and the predicate itself is
/// still applied afterwards.
fn push_id_hints(payload: &mut MatchPayload) {
    let Some(predicate) = payload.predicate.clone() else {
        return;
    };
    let mut hints: Vec<(String, u64)> = Vec::new();
    collect_id_hints(&predicate, &mut hints);
    if hints.is_empty() {
        return;
    }
    apply_hints(&mut payload.pattern, &hints);
}

fn collect_id_hints(expr: &Expr, out: &mut Vec<(String, u64)>) {
    match expr {
        Expr::Binary(BinOp::And, left, right) => {
            collect_id_hints(left, out);
            collect_id_hints(right, out);
        }
        Expr::Binary(BinOp::Eq, left, right) => {
            if let Some(hint) = id_equality(left, right).or_else(|| id_equality(right, left)) {
                out.push(hint);
            }
        }
        _ => {}
    }
}

fn id_equality(left: &Expr, right: &Expr) -> Option<(String, u64)> {
    let Expr::Function { name, args, .. } = left else {
        return None;
    };
    if !name.eq_ignore_ascii_case("id") || args.len() != 1 {
        return None;
    }
    let Expr::Variable(variable) = &args[0] else {
        return None;
    };
    let Expr::Int(value) = right else {
        return None;
    };
    if *value <= 0 {
        return None;
    }
    Some((variable.clone(), *value as u64))
}

fn apply_hints(pattern: &mut Pattern, hints: &[(String, u64)]) {
    for part in &mut pattern.parts {
        let mut nodes: Vec<&mut NodePattern> = Vec::new();
        nodes.push(&mut part.element.node);
        for (_, node) in &mut part.element.chains {
            nodes.push(node);
        }
        for node in nodes {
            let Some(variable) = node.var.clone() else {
                continue;
            };
            if let Some((_, value)) = hints.iter().find(|(name, _)| *name == variable) {
                node.id_hint = Some(*value);
            }
        }
    }
}

fn describe(op: &LogicalOp) -> &'static str {
    match op {
        LogicalOp::SingleRow => "SingleRow",
        LogicalOp::VertexScan { .. } => "VertexScan",
        LogicalOp::EdgeScan { .. } => "EdgeScan",
        LogicalOp::Expand { .. } => "Expand",
        LogicalOp::Filter { .. } => "Filter",
        LogicalOp::Project { .. } => "Project",
        LogicalOp::Aggregate { .. } => "Aggregate",
        LogicalOp::Sort { .. } => "Sort",
        LogicalOp::Limit { .. } => "Limit",
        LogicalOp::Join { .. } => "Join",
        LogicalOp::Union { .. } => "Union",
        LogicalOp::GraphProjection { .. } => "GraphProjection",
        LogicalOp::CallAlgorithm { .. } => "CallAlgorithm",
        LogicalOp::ExtensionOp { .. } => "ExtensionOp",
    }
}
