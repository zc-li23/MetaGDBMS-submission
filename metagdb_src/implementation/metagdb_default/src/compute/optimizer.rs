/**
 * @brief: MetagdbOptimizer: optimizer that lowers logical plans into implementation-side physical operators.
 *
 * Maps LogicalOp one-to-one to DefaultPhysicalOp, and wraps into core's PhysicalPlan abstraction.
 * New operators such as DML are mapped through ExtensionOp for backward compatibility.
 */

use metagdb_core::l4_compute::algebra::{JoinKind, LogicalOp, LogicalPlan, PhysicalPlan};
use metagdb_core::l4_compute::expression::Expr;
use metagdb_core::l4_compute::engine::{OptContext, Optimizer};
use metagdb_core::{Error, Result};

use super::physical_ops::{
    CreateEdgePayload, CreateNodePayload, DefaultPhysicalOp, DeletePayload, MergePayload,
    RemoveLabelsPayload, RemovePropertyPayload, SetLabelsPayload, SetPropertyPayload,
    UnwindPayload, VarLengthExpandPayload,
};

/**
 * @brief: Default optimizer implementation, performing one-to-one LogicalOp → DefaultPhysicalOp lowering.
 */
#[derive(Default)]
pub struct MetagdbOptimizer;

impl Optimizer for MetagdbOptimizer {
    /**
     * @brief: Lower logical plan to opaque physical plan.
     */
    fn optimize(&self, lp: LogicalPlan, ctx: &OptContext) -> Result<PhysicalPlan> {
        let root = lower(lp.root, ctx)?;
        Ok(PhysicalPlan::new(root))
    }
}

/**
 * @brief: Recursively lower logical operators to implementation-side physical operators.
 */
fn lower(op: LogicalOp, ctx: &OptContext) -> Result<DefaultPhysicalOp> {
    match op {
        // ── Data sources ──
        LogicalOp::SingleRow => Ok(DefaultPhysicalOp::SingleRow),
        LogicalOp::VertexScan { var, label } => Ok(DefaultPhysicalOp::VertexScan { var, label }),
        LogicalOp::EdgeScan { var, edge_type } => Ok(DefaultPhysicalOp::ExtensionOp {
            name: "EdgeScan".into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(format!(
                "edge_scan(var={}, edge_type={:?})", var, edge_type
            )),
            input: Box::new(DefaultPhysicalOp::SingleRow),
        }),

        // ── Graph traversal ──
        LogicalOp::Expand {
            from,
            to,
            edge,
            direction,
            edge_types,
            input,
        } => Ok(DefaultPhysicalOp::Expand {
            from,
            to,
            edge,
            direction,
            edge_types,
            input: Box::new(lower(*input, ctx)?),
        }),

        // ── Relational algebra ──
        LogicalOp::Filter { predicate, input } => Ok(DefaultPhysicalOp::Filter {
            predicate,
            input: Box::new(lower(*input, ctx)?),
        }),
        LogicalOp::Project { items, input } => Ok(DefaultPhysicalOp::Project {
            items,
            input: Box::new(lower(*input, ctx)?),
        }),
        LogicalOp::Aggregate {
            group_by,
            aggs,
            input,
        } => {
            let gb_items: Vec<(Expr, String)> = group_by
                .iter()
                .enumerate()
                .map(|(i, e)| (e.clone(), format!("__gb_{i}")))
                .collect();
            Ok(DefaultPhysicalOp::Aggregate {
                group_by: gb_items,
                aggs,
                input: Box::new(lower(*input, ctx)?),
            })
        }
        LogicalOp::Sort { keys, input } => Ok(DefaultPhysicalOp::Sort {
            keys,
            input: Box::new(lower(*input, ctx)?),
        }),
        LogicalOp::Limit {
            limit,
            offset,
            input,
        } => Ok(DefaultPhysicalOp::Limit {
            limit,
            offset,
            input: Box::new(lower(*input, ctx)?),
        }),
        LogicalOp::Join {
            kind,
            on: _,
            left,
            right,
        } => {
            let left_op = lower(*left, ctx)?;
            let right_op = lower(*right, ctx)?;
            match kind {
                JoinKind::Cross => Ok(DefaultPhysicalOp::CrossJoin {
                    left: Box::new(left_op),
                    right: Box::new(right_op),
                }),
                // Other JoinKind not yet reachable, kept as ExtensionOp
                _ => Ok(DefaultPhysicalOp::ExtensionOp {
                    name: "Join".into(),
                    payload: metagdb_core::l4_compute::algebra::OpaquePayload::new("join"),
                    input: Box::new(DefaultPhysicalOp::Union {
                        all: true,
                        inputs: vec![left_op, right_op],
                    }),
                }),
            }
        }
        LogicalOp::Union { all, inputs } => {
            let mut lowered_inputs = Vec::with_capacity(inputs.len());
            for input in inputs {
                lowered_inputs.push(lower(input, ctx)?);
            }
            Ok(DefaultPhysicalOp::Union {
                all,
                inputs: lowered_inputs,
            })
        }

        // ── OLAP path ──
        LogicalOp::GraphProjection { spec } => Ok(DefaultPhysicalOp::BuildProjection { spec }),
        LogicalOp::CallAlgorithm { name, args, input } => {
            if !ctx.algorithms.algorithms.contains_key(&name) {
                return Err(Error::Optimize(format!("Unregistered algorithm: `{name}`")));
            }
            Ok(DefaultPhysicalOp::RunAlgorithm {
                name,
                args,
                input: Box::new(lower(*input, ctx)?),
            })
        }

        // ── DML / Extension operators ──
        LogicalOp::ExtensionOp {
            name,
            payload,
            input,
        } => {
            let child = lower(*input, ctx)?;
            match name.as_str() {
                "CreateNode" => {
                    let p = payload.downcast_ref::<CreateNodePayload>()
                        .ok_or_else(|| Error::Optimize("CreateNode payload type error".into()))?;
                    Ok(DefaultPhysicalOp::CreateNode {
                        var: p.var.clone(),
                        label_id: p.label_id,
                        properties: p.properties.clone(),
                        input: Box::new(child),
                    })
                }
                "CreateEdge" => {
                    let p = payload.downcast_ref::<CreateEdgePayload>()
                        .ok_or_else(|| Error::Optimize("CreateEdge payload type error".into()))?;
                    Ok(DefaultPhysicalOp::CreateEdge {
                        from_var: p.from_var.clone(),
                        to_var: p.to_var.clone(),
                        edge_var: p.edge_var.clone(),
                        edge_type_ids: p.edge_type_ids.clone(),
                        properties: p.properties.clone(),
                        input: Box::new(child),
                    })
                }
                "SetProperty" => {
                    let p = payload.downcast_ref::<SetPropertyPayload>()
                        .ok_or_else(|| Error::Optimize("SetProperty payload type error".into()))?;
                    Ok(DefaultPhysicalOp::SetProperty {
                        var: p.var.clone(),
                        key_id: p.key_id,
                        value: p.value.clone(),
                        input: Box::new(child),
                    })
                }
                "SetLabels" => {
                    let p = payload.downcast_ref::<SetLabelsPayload>()
                        .ok_or_else(|| Error::Optimize("SetLabels payload type error".into()))?;
                    Ok(DefaultPhysicalOp::SetLabels {
                        var: p.var.clone(),
                        label_ids: p.label_ids.clone(),
                        input: Box::new(child),
                    })
                }
                "RemoveProperty" => {
                    let p = payload.downcast_ref::<RemovePropertyPayload>()
                        .ok_or_else(|| Error::Optimize("RemoveProperty payload type error".into()))?;
                    Ok(DefaultPhysicalOp::RemoveProperty {
                        var: p.var.clone(),
                        key_id: p.key_id,
                        input: Box::new(child),
                    })
                }
                "RemoveLabels" => {
                    let p = payload.downcast_ref::<RemoveLabelsPayload>()
                        .ok_or_else(|| Error::Optimize("RemoveLabels payload type error".into()))?;
                    Ok(DefaultPhysicalOp::RemoveLabels {
                        var: p.var.clone(),
                        label_ids: p.label_ids.clone(),
                        input: Box::new(child),
                    })
                }
                "Delete" => {
                    let p = payload.downcast_ref::<DeletePayload>()
                        .ok_or_else(|| Error::Optimize("Delete payload type error".into()))?;
                    Ok(DefaultPhysicalOp::Delete {
                        detach: p.detach,
                        targets: p.target_exprs.clone(),
                        input: Box::new(child),
                    })
                }
                "Unwind" => {
                    let p = payload.downcast_ref::<UnwindPayload>()
                        .ok_or_else(|| Error::Optimize("Unwind payload type error".into()))?;
                    Ok(DefaultPhysicalOp::Unwind {
                        expr: p.expr.clone(),
                        var: p.var.clone(),
                        input: Box::new(child),
                    })
                }
                "Merge" => {
                    let p = payload.downcast_ref::<MergePayload>()
                        .ok_or_else(|| Error::Optimize("Merge payload type error".into()))?;
                    Ok(DefaultPhysicalOp::Merge {
                        payload: p.clone(),
                        input: Box::new(child),
                    })
                }
                "VarLengthExpand" => {
                    let p = payload.downcast_ref::<VarLengthExpandPayload>()
                        .ok_or_else(|| Error::Optimize("VarLengthExpand payload type error".into()))?;
                    Ok(DefaultPhysicalOp::VarLengthExpand {
                        from: p.from.clone(),
                        to: p.to.clone(),
                        edge: p.edge.clone(),
                        direction: p.direction,
                        edge_types: p.edge_types.clone(),
                        min_hops: p.min_hops,
                        max_hops: p.max_hops,
                        input: Box::new(child),
                    })
                }
                // Unrecognized ExtensionOp passed through as-is (Merge / Aggregate, etc.)
                _ => Ok(DefaultPhysicalOp::ExtensionOp {
                    name,
                    payload,
                    input: Box::new(child),
                }),
            }
        }
    }
}
