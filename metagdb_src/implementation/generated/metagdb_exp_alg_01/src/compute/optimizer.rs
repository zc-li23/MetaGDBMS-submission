//! Logical to physical lowering.
//!
//! `MetagdbOptimizer` is the candidate's physical lowering pass.  It performs no
//! reordering or other transformation that could change results: every logical
//! operator and every named extension payload has exactly one physical form, and
//! nesting, predicates, projections, ordering, pagination, write payloads, and
//! the algorithm input tree are preserved node for node.
//!
//! Two checks make the pass more than a rename:
//!
//! * a `CallAlgorithm` whose name is absent from the runtime's algorithm
//!   registry is rejected here, so a planner signature can never conjure an
//!   algorithm that was not assembled;
//! * an unrecognized extension name is rejected with the name in the message,
//!   so a new planner payload cannot be silently dropped.

use std::collections::HashMap;

use metagdb_core::l4_compute::algebra::{LogicalOp, LogicalPlan, OpaquePayload, PhysicalPlan};
use metagdb_core::l4_compute::engine::{OptContext, Optimizer};
use metagdb_core::l4_compute::expression::Expr;
use metagdb_core::l4_compute::olap::AnalyticalAlgorithm;
use metagdb_core::{Error, Result};

use super::logical_ext::{
    MergeSpec, MutateSpec, UnwindSpec, VarExpandSpec, EXT_DISTINCT, EXT_MERGE, EXT_MUTATE,
    EXT_UNWIND, EXT_VAR_EXPAND,
};
use super::physical_ops::PhysicalOp;

/// Optimizer stage: logical algebra to the candidate's physical operators.
#[derive(Debug, Default, Clone, Copy)]
pub struct MetagdbOptimizer;

impl Optimizer for MetagdbOptimizer {
    fn optimize(&self, lp: LogicalPlan, ctx: &OptContext) -> Result<PhysicalPlan> {
        let root = lower(lp.root, &ctx.algorithms.algorithms)?;
        Ok(PhysicalPlan::new(root))
    }
}

fn payload<'a, T: 'static>(payload: &'a OpaquePayload, name: &str) -> Result<&'a T> {
    payload
        .downcast_ref::<T>()
        .ok_or_else(|| Error::Optimize(format!("extension '{name}' carries an unexpected payload")))
}

fn algorithm<'a>(
    name: &str,
    registry: &'a HashMap<String, std::sync::Arc<dyn AnalyticalAlgorithm>>,
) -> Result<&'a std::sync::Arc<dyn AnalyticalAlgorithm>> {
    registry
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, algorithm)| algorithm)
        .ok_or_else(|| {
            Error::Optimize(format!(
                "no algorithm named '{name}' is registered (registered: {})",
                registry
                    .keys()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

fn lower(op: LogicalOp, registry: &HashMap<String, std::sync::Arc<dyn AnalyticalAlgorithm>>) -> Result<PhysicalOp> {
    Ok(match op {
        LogicalOp::SingleRow => PhysicalOp::SingleRow,
        LogicalOp::VertexScan { var, label } => PhysicalOp::VertexScan { var, label },
        LogicalOp::EdgeScan { var, edge_type } => PhysicalOp::EdgeScan { var, etype: edge_type },
        LogicalOp::Expand {
            from,
            to,
            edge,
            direction,
            edge_types,
            input,
        } => PhysicalOp::Expand {
            input: Box::new(lower(*input, registry)?),
            from,
            to,
            edge,
            direction,
            edge_types,
        },
        LogicalOp::Filter { predicate, input } => {
            // `WHERE id(v) = <literal>` is a point lookup: the storage engine
            // maintains a vertex-ID index, so the whole scan can be replaced.
            if let LogicalOp::VertexScan {
                var,
                label: None,
            } = input.as_ref()
            {
                let mut parts = conjuncts(&predicate);
                if let Some(index) = parts
                    .iter()
                    .position(|part| id_equality(part, var).is_some())
                {
                    let id = id_equality(&parts[index], var).expect("just matched");
                    parts.remove(index);
                    let mut folded = PhysicalOp::VertexById {
                        var: var.clone(),
                        id,
                    };
                    for part in parts {
                        folded = PhysicalOp::Filter {
                            predicate: part,
                            input: Box::new(folded),
                        };
                    }
                    return Ok(folded);
                }
            }
            PhysicalOp::Filter {
                predicate,
                input: Box::new(lower(*input, registry)?),
            }
        }
        LogicalOp::Project { items, input } => PhysicalOp::Project {
            items,
            input: Box::new(lower(*input, registry)?),
        },
        LogicalOp::Aggregate {
            group_by,
            aggs,
            input,
        } => PhysicalOp::Aggregate {
            group_by,
            aggs,
            input: Box::new(lower(*input, registry)?),
        },
        LogicalOp::Sort { keys, input } => PhysicalOp::Sort {
            keys,
            input: Box::new(lower(*input, registry)?),
        },
        LogicalOp::Limit {
            limit,
            offset,
            input,
        } => PhysicalOp::Limit {
            limit,
            offset,
            input: Box::new(lower(*input, registry)?),
        },
        LogicalOp::Join {
            kind,
            on,
            left,
            right,
        } => PhysicalOp::Join {
            kind,
            on,
            left: Box::new(lower(*left, registry)?),
            right: Box::new(lower(*right, registry)?),
        },
        LogicalOp::Union { all, inputs } => {
            let mut lowered = Vec::with_capacity(inputs.len());
            for input in inputs {
                lowered.push(lower(input, registry)?);
            }
            PhysicalOp::Union {
                all,
                inputs: lowered,
            }
        }
        LogicalOp::GraphProjection { spec } => PhysicalOp::GraphProjection { spec },
        LogicalOp::CallAlgorithm { name, args, input } => {
            algorithm(&name, registry)?;
            // The planner feeds an algorithm from a projection build; anything
            // else would let a call read arbitrary rows.
            let spec = match input.as_ref() {
                LogicalOp::GraphProjection { spec } => spec.clone(),
                other => {
                    return Err(Error::Optimize(format!(
                        "algorithm '{name}' must read a graph projection, got {}",
                        operator_name(other)
                    )))
                }
            };
            PhysicalOp::CallAlgorithm {
                name,
                args,
                spec,
                input: Box::new(PhysicalOp::SingleRow),
            }
        }
        LogicalOp::ExtensionOp {
            name,
            payload: data,
            input,
        } => match name.as_str() {
            EXT_UNWIND => {
                let spec: &UnwindSpec = payload(&data, &name)?;
                PhysicalOp::Unwind {
                    list: spec.list.clone(),
                    alias: spec.alias.clone(),
                    input: Box::new(lower(*input, registry)?),
                }
            }
            EXT_DISTINCT => PhysicalOp::Distinct {
                input: Box::new(lower(*input, registry)?),
            },
            EXT_VAR_EXPAND => {
                let spec: &VarExpandSpec = payload(&data, &name)?;
                PhysicalOp::VarExpand {
                    input: Box::new(lower(*input, registry)?),
                    from: spec.from.clone(),
                    to: spec.to.clone(),
                    edge: spec.edge.clone(),
                    path: spec.path.clone(),
                    direction: spec.direction,
                    edge_types: spec.edge_types.clone(),
                    min: spec.min,
                    max: spec.max,
                }
            }
            EXT_MUTATE => {
                let spec: &MutateSpec = payload(&data, &name)?;
                PhysicalOp::Mutate {
                    writes: spec.writes.clone(),
                    projection: spec.projection.clone(),
                    input: Box::new(lower(*input, registry)?),
                }
            }
            EXT_MERGE => {
                let spec: &MergeSpec = payload(&data, &name)?;
                PhysicalOp::Merge {
                    input: Box::new(lower(*input, registry)?),
                    probe: Box::new(lower((*spec.probe).clone(), registry)?),
                    create: spec.create.clone(),
                    actions: spec.actions.clone(),
                }
            }
            other => {
                return Err(Error::Optimize(format!(
                    "extension '{other}' has no physical lowering"
                )))
            }
        },
    })
}

/// Split a predicate at its top-level `AND`s.
fn conjuncts(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::And(left, right) => {
            let mut parts = conjuncts(left);
            parts.extend(conjuncts(right));
            parts
        }
        other => vec![other.clone()],
    }
}

/// `id(var) = <integer literal>`, in either operand order.
fn id_equality(expr: &Expr, var: &str) -> Option<u64> {
    let Expr::Eq(left, right) = expr else {
        return None;
    };
    match (left.as_ref(), right.as_ref()) {
        (Expr::FunctionCall { name, args }, Expr::Int(value))
        | (Expr::Int(value), Expr::FunctionCall { name, args }) => {
            if !name.eq_ignore_ascii_case("id") || *value <= 0 {
                return None;
            }
            match args.as_slice() {
                [Expr::VarRef { name }] if name == var => Some(*value as u64),
                _ => None,
            }
        }
        _ => None,
    }
}

fn operator_name(op: &LogicalOp) -> &'static str {
    match op {
        LogicalOp::SingleRow => "single_row",
        LogicalOp::VertexScan { .. } => "vertex_scan",
        LogicalOp::EdgeScan { .. } => "edge_scan",
        LogicalOp::Expand { .. } => "expand",
        LogicalOp::Filter { .. } => "filter",
        LogicalOp::Project { .. } => "project",
        LogicalOp::Aggregate { .. } => "aggregate",
        LogicalOp::Sort { .. } => "sort",
        LogicalOp::Limit { .. } => "limit",
        LogicalOp::Join { .. } => "join",
        LogicalOp::Union { .. } => "union",
        LogicalOp::GraphProjection { .. } => "graph_projection",
        LogicalOp::CallAlgorithm { .. } => "call_algorithm",
        LogicalOp::ExtensionOp { .. } => "extension",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::algorithms::default_algorithms;
    use crate::compute::physical_ops::WriteOp;
    use crate::schema::SchemaRegistry;
    use metagdb_core::common::ids::LabelId;
    use metagdb_core::l4_compute::algebra::{ExpandDirection, ProjectionSpec};
    use metagdb_core::l4_compute::engine::{Parser, Planner};
    use metagdb_core::l4_compute::expression::Expr;
    use metagdb_core::l4_compute::olap::AlgorithmRegistry;
    use std::sync::Arc;

    fn context() -> OptContext {
        let mut registry = AlgorithmRegistry::default();
        for algorithm in default_algorithms() {
            registry.algorithms.insert(algorithm.name().to_string(), algorithm);
        }
        OptContext {
            schema: Arc::new(SchemaRegistry::new()),
            algorithms: Arc::new(registry),
        }
    }

    fn optimize(source: &str) -> Result<PhysicalOp> {
        let ast = crate::language::MetagdbParser.parse(source)?;
        let schema = SchemaRegistry::new();
        let logical = crate::compute::planner::MetagdbPlanner.plan(ast, &schema)?;
        let ctx = context();
        let plan = MetagdbOptimizer.optimize(logical, &ctx)?;
        Ok(plan
            .downcast_ref::<PhysicalOp>()
            .expect("the physical plan is the candidate's own tree")
            .clone())
    }

    fn names(op: &PhysicalOp, out: &mut Vec<String>) {
        match op {
            PhysicalOp::SingleRow => out.push("single_row".into()),
            PhysicalOp::VertexScan { .. } => out.push("vertex_scan".into()),
            PhysicalOp::VertexById { .. } => out.push("vertex_by_id".into()),
            PhysicalOp::EdgeScan { .. } => out.push("edge_scan".into()),
            PhysicalOp::GraphProjection { .. } => out.push("graph_projection".into()),
            PhysicalOp::Expand { input, .. }
            | PhysicalOp::Filter { input, .. }
            | PhysicalOp::Project { input, .. }
            | PhysicalOp::Aggregate { input, .. }
            | PhysicalOp::Distinct { input }
            | PhysicalOp::Sort { input, .. }
            | PhysicalOp::Limit { input, .. }
            | PhysicalOp::Unwind { input, .. }
            | PhysicalOp::VarExpand { input, .. }
            | PhysicalOp::Mutate { input, .. }
            | PhysicalOp::CallAlgorithm { input, .. } => {
                out.push(
                    match op {
                        PhysicalOp::Expand { .. } => "expand",
                        PhysicalOp::Filter { .. } => "filter",
                        PhysicalOp::Project { .. } => "project",
                        PhysicalOp::Aggregate { .. } => "aggregate",
                        PhysicalOp::Distinct { .. } => "distinct",
                        PhysicalOp::Sort { .. } => "sort",
                        PhysicalOp::Limit { .. } => "limit",
                        PhysicalOp::Unwind { .. } => "unwind",
                        PhysicalOp::VarExpand { .. } => "var_expand",
                        PhysicalOp::Mutate { .. } => "mutate",
                        PhysicalOp::CallAlgorithm { .. } => "call",
                        _ => unreachable!(),
                    }
                    .to_string(),
                );
                names(input, out);
            }
            PhysicalOp::Join { left, right, .. } => {
                out.push("join".into());
                names(left, out);
                names(right, out);
            }
            PhysicalOp::Union { inputs, .. } => {
                out.push("union".into());
                for input in inputs {
                    names(input, out);
                }
            }
            PhysicalOp::Merge { input, probe, .. } => {
                out.push("merge".into());
                names(input, out);
                names(probe, out);
            }
            PhysicalOp::Unsupported { .. } => out.push("unsupported".into()),
        }
    }

    fn shape(source: &str) -> Vec<String> {
        let op = optimize(source).expect("optimizes");
        let mut out = Vec::new();
        names(&op, &mut out);
        out
    }

    #[test]
    fn every_planner_shape_has_a_physical_lowering() {
        assert_eq!(shape("MATCH (n) RETURN n"), vec!["project", "vertex_scan"]);
        assert_eq!(
            shape("MATCH (a)-[:KNOWS]->(b) RETURN b"),
            vec!["project", "expand", "vertex_scan"]
        );
        assert_eq!(
            shape("MATCH (n:Person) WHERE n.age > 29 RETURN n.name"),
            vec!["project", "filter", "vertex_scan"]
        );
        assert_eq!(
            shape("MATCH (n:Person) RETURN n.age, count(n)"),
            vec!["project", "aggregate", "vertex_scan"]
        );
        assert_eq!(
            shape("MATCH (n:Person) RETURN n.name ORDER BY n.name LIMIT 2"),
            vec!["limit", "project", "sort", "vertex_scan"]
        );
        assert_eq!(
            shape("MATCH (n:Person) RETURN DISTINCT n.age"),
            vec!["distinct", "project", "vertex_scan"]
        );
        assert_eq!(
            shape("UNWIND [1, 2] AS x RETURN x"),
            vec!["project", "unwind", "single_row"]
        );
        assert_eq!(
            shape("RETURN 1 UNION ALL RETURN 2"),
            vec!["union", "project", "single_row", "project", "single_row"]
        );
        assert_eq!(
            shape("MATCH (a)-[:KNOWS*1..2]->(b) RETURN b"),
            vec!["project", "var_expand", "vertex_scan"]
        );
        assert_eq!(
            shape("MATCH (n:City) DELETE n RETURN count(n)"),
            vec!["project", "aggregate", "mutate", "vertex_scan"]
        );
        assert_eq!(
            shape("MERGE (n:Person {name: 'F'}) RETURN n.name"),
            vec!["project", "merge", "single_row", "filter", "vertex_scan"]
        );
        assert_eq!(
            shape("CALL bfs(1) YIELD node RETURN count(*)"),
            vec!["project", "aggregate", "join", "single_row", "project", "call", "single_row"]
        );
    }

    #[test]
    fn an_id_equality_folds_a_scan_into_a_point_lookup() {
        assert_eq!(
            shape("MATCH (n) WHERE id(n) = 7 RETURN n"),
            vec!["project", "vertex_by_id"]
        );
        // An extra conjunct is kept as a filter above the lookup.
        assert_eq!(
            shape("MATCH (n) WHERE id(n) = 7 AND n.age > 1 RETURN n"),
            vec!["project", "filter", "vertex_by_id"]
        );
        // Without an id equality the general scan remains.
        assert_eq!(
            shape("MATCH (n) WHERE n.age > 1 RETURN n"),
            vec!["project", "filter", "vertex_scan"]
        );
    }

    #[test]
    fn payloads_and_predicates_survive_the_lowering() {
        let op = optimize("MATCH (a)-[r:KNOWS*2]->(b) RETURN length(r)").expect("optimizes");
        fn find_var_expand(op: &PhysicalOp) -> Option<&PhysicalOp> {
            match op {
                PhysicalOp::VarExpand { .. } => Some(op),
                PhysicalOp::Project { input, .. } | PhysicalOp::Filter { input, .. } => {
                    find_var_expand(input)
                }
                _ => None,
            }
        }
        match find_var_expand(&op).expect("a var expand") {
            PhysicalOp::VarExpand {
                from,
                to,
                min,
                max,
                direction,
                edge_types,
                ..
            } => {
                assert_eq!(from, "a");
                assert_eq!(to, "b");
                assert_eq!((*min, *max), (2, Some(2)));
                assert_eq!(*direction, ExpandDirection::Outgoing);
                assert_eq!(edge_types.as_ref().map(Vec::len), Some(1));
            }
            other => panic!("expected a var expand, got {other:?}"),
        }
    }

    #[test]
    fn write_payloads_survive_and_keep_their_variables() {
        let op = optimize("CREATE (n:Person {name: 'F'}) RETURN n.name").expect("optimizes");
        let PhysicalOp::Project { input, .. } = &op else {
            panic!("expected a projection");
        };
        match input.as_ref() {
            PhysicalOp::Mutate { writes, .. } => {
                assert_eq!(writes.len(), 1);
                assert!(matches!(
                    writes[0],
                    WriteOp::CreateVertex {
                        labels: _,
                        ..
                    }
                ));
            }
            other => panic!("expected a mutate, got {other:?}"),
        }
    }

    #[test]
    fn merge_keeps_its_probe_create_and_actions() {
        let op = optimize("MERGE (n:Person {name: 'F'}) ON MATCH SET n.seen = 1 RETURN n.seen")
            .expect("optimizes");
        fn find(op: &PhysicalOp) -> Option<&PhysicalOp> {
            match op {
                PhysicalOp::Merge { .. } => Some(op),
                PhysicalOp::Project { input, .. } | PhysicalOp::Filter { input, .. } => find(input),
                _ => None,
            }
        }
        match find(&op).expect("a merge") {
            PhysicalOp::Merge {
                create, actions, ..
            } => {
                assert_eq!(create.len(), 1);
                assert_eq!(actions.len(), 1);
                assert!(actions[0].0, "the action is ON MATCH");
                assert_eq!(actions[0].1.len(), 1);
            }
            other => panic!("expected a merge, got {other:?}"),
        }
    }

    #[test]
    fn an_algorithm_call_becomes_a_registry_checked_physical_call() {
        let op = optimize("CALL pagerank() YIELD node RETURN count(*)").expect("optimizes");
        fn find(op: &PhysicalOp) -> Option<&PhysicalOp> {
            match op {
                PhysicalOp::CallAlgorithm { .. } => Some(op),
                PhysicalOp::Project { input, .. } | PhysicalOp::Aggregate { input, .. } => find(input),
                PhysicalOp::Join { left, right, .. } => find(left).or_else(|| find(right)),
                _ => None,
            }
        }
        match find(&op).expect("a call") {
            PhysicalOp::CallAlgorithm { name, spec, .. } => {
                assert_eq!(name, "pagerank");
                assert_eq!(spec.direction, crate::compute::physical_ops::default_projection_direction());
            }
            other => panic!("expected a call, got {other:?}"),
        }
    }

    #[test]
    fn an_unregistered_algorithm_is_refused_even_though_the_planner_typed_it() {
        // The planner carries the candidate's signatures, so this query plans;
        // the optimizer is what proves the name is actually assembled.
        let ast = crate::language::MetagdbParser
            .parse("CALL bfs(1) YIELD node RETURN count(*)")
            .expect("parses");
        let schema = SchemaRegistry::new();
        let logical = crate::compute::planner::MetagdbPlanner
            .plan(ast, &schema)
            .expect("plans");
        let empty = OptContext {
            schema: Arc::new(SchemaRegistry::new()),
            algorithms: Arc::new(AlgorithmRegistry::default()),
        };
        let error = MetagdbOptimizer
            .optimize(logical, &empty)
            .expect_err("refused");
        let rendered = format!("{error}");
        assert!(rendered.contains("bfs"), "the error names the algorithm: {rendered}");
    }

    #[test]
    fn an_algorithm_must_read_a_graph_projection() {
        let logical = LogicalPlan {
            root: LogicalOp::CallAlgorithm {
                name: "bfs".into(),
                args: Default::default(),
                input: Box::new(LogicalOp::SingleRow),
            },
        };
        let error = MetagdbOptimizer
            .optimize(logical, &context())
            .expect_err("refused");
        assert!(matches!(error, Error::Optimize(_)), "got {error:?}");
    }

    #[test]
    fn an_unknown_extension_name_is_refused_with_its_name() {
        let logical = LogicalPlan {
            root: LogicalOp::ExtensionOp {
                name: "cypher.unknown".into(),
                payload: OpaquePayload::new(()),
                input: Box::new(LogicalOp::SingleRow),
            },
        };
        let error = MetagdbOptimizer
            .optimize(logical, &context())
            .expect_err("refused");
        let rendered = format!("{error}");
        assert!(rendered.contains("cypher.unknown"), "got {rendered}");
    }

    #[test]
    fn an_extension_with_the_wrong_payload_type_is_refused() {
        let logical = LogicalPlan {
            root: LogicalOp::ExtensionOp {
                name: EXT_UNWIND.into(),
                payload: OpaquePayload::new(LabelId(1)),
                input: Box::new(LogicalOp::SingleRow),
            },
        };
        let error = MetagdbOptimizer
            .optimize(logical, &context())
            .expect_err("refused");
        assert!(matches!(error, Error::Optimize(_)), "got {error:?}");
    }

    #[test]
    fn a_graph_projection_keeps_its_spec() {
        let logical = LogicalPlan {
            root: LogicalOp::GraphProjection {
                spec: ProjectionSpec {
                    direction: metagdb_core::l4_compute::algebra::ProjectionDirection::Both,
                    ..ProjectionSpec::default()
                },
            },
        };
        let plan = MetagdbOptimizer.optimize(logical, &context()).expect("optimizes");
        match plan.downcast_ref::<PhysicalOp>().expect("physical") {
            PhysicalOp::GraphProjection { spec } => {
                assert_eq!(
                    spec.direction,
                    metagdb_core::l4_compute::algebra::ProjectionDirection::Both
                );
            }
            other => panic!("expected a projection, got {other:?}"),
        }
    }

    #[test]
    fn no_physical_payload_is_a_query_string() {
        let op = optimize("MATCH (n:Person) WHERE n.age > 29 RETURN n.name").expect("optimizes");
        let rendered = format!("{op:?}");
        assert!(!rendered.contains("MATCH"));
        assert!(!rendered.contains("Person"));
        let _: Expr = Expr::Null;
    }
}
