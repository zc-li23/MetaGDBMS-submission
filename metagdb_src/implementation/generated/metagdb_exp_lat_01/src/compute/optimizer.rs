//! Optimizer: lowers the logical clause pipeline into an executable physical
//! query and applies a point-read optimization.
//!
//! Rule: a top-level conjunct `id(v) = <integer literal>` in a MATCH whose first
//! pattern element is anchored on `v` is removed from the filter and recorded as
//! a scan hint, so the anchor becomes a direct vertex point lookup. This is the
//! optimization the candidate's warm point-read/one-hop profile depends on.

use metagdb_core::l4_compute::algebra::{LogicalOp, LogicalPlan, PhysicalPlan};
use metagdb_core::l4_compute::engine::{OptContext, Optimizer};
use metagdb_core::{Error, Result};

use super::ir::{AggregateKind, BExpr, Value};
use super::physical::PhysicalQuery;
use super::plan::{LogicalQuery, MatchStage, ProjectStage, Stage};

/// Optimizer stage.
#[derive(Default)]
pub struct MetagdbOptimizer;

impl Optimizer for MetagdbOptimizer {
    fn optimize(&self, lp: LogicalPlan, _ctx: &OptContext) -> Result<PhysicalPlan> {
        let query = match &lp.root {
            LogicalOp::ExtensionOp { name, payload, .. } if name == "cypher_pipeline" => payload
                .downcast_ref::<LogicalQuery>()
                .ok_or_else(|| Error::Optimize("cypher plan payload has the wrong type".into()))?
                .clone(),
            _ => {
                return Err(Error::Optimize(
                    "MetagdbOptimizer expected a cypher_pipeline logical plan".into(),
                ))
            }
        };
        let optimized = optimize_query(query);
        Ok(PhysicalPlan::new(PhysicalQuery(optimized)))
    }
}

fn optimize_query(mut query: LogicalQuery) -> LogicalQuery {
    query.stages = optimize_stages(query.stages);
    query.unions = query
        .unions
        .into_iter()
        .map(|(all, stages)| (all, optimize_stages(stages)))
        .collect();
    query
}

fn optimize_stages(stages: Vec<Stage>) -> Vec<Stage> {
    let mut stages: Vec<Stage> = stages.into_iter().map(optimize_stage).collect();
    for index in 0..stages.len().saturating_sub(1) {
        let hint = match (&stages[index], &stages[index + 1]) {
            (Stage::Match(m), Stage::Project(p)) => detect_shortest_path_hint(m, p),
            _ => None,
        };
        if let Some((variable, id)) = hint {
            if let Stage::Match(m) = &mut stages[index] {
                m.end_hint = Some((variable, id));
                m.shortest_path = true;
            }
        }
    }
    stages
}

fn optimize_stage(stage: Stage) -> Stage {
    match stage {
        Stage::Match(mut m) => {
            if let Some(hint) = extract_point_hint(&m) {
                m.id_hint = Some(hint);
            }
            Stage::Match(m)
        }
        other => other,
    }
}

fn extract_point_hint(m: &MatchStage) -> Option<(String, u64)> {
    let anchor = m.parts.first()?.element.node.var.clone()?;
    let predicate = m.where_clause.as_ref()?;
    let conjuncts = split_conjuncts(predicate);
    for conjunct in &conjuncts {
        if let Some((variable, id)) = match_point_lookup(conjunct) {
            if variable == anchor {
                return Some((variable, id));
            }
        }
    }
    None
}

/// Recognize the exact shape of a shortest-path question:
///
/// `MATCH p = (s)-[*1..N]->(t) WHERE id(s) = S AND id(t) = T`
/// `RETURN min(length(p))`
///
/// The result is the terminal node's `id` hint.  The fast path is deliberately
/// narrow: one pattern part, one variable-length chain, every predicate an
/// `id(v) = k` equality, and the only projection a minimum path length.  Under
/// those conditions a BFS from `s` to `t` returns exactly the same answer as
/// enumerating every trail, because only the shortest length is observed.
fn detect_shortest_path_hint(m: &MatchStage, p: &ProjectStage) -> Option<(String, u64)> {
    if m.optional || m.parts.len() != 1 || p.items.len() != 1 || !p.order_by.is_empty() {
        return None;
    }
    let part = &m.parts[0];
    let path_var = part.path_var.as_ref()?;
    if part.element.chains.len() != 1 {
        return None;
    }
    let (rel, terminal_node) = &part.element.chains[0];
    if rel.min_hops != 1 || rel.max_hops.is_none() || !rel.properties.is_empty() {
        return None;
    }
    let BExpr::Aggregate {
        kind: AggregateKind::Min,
        arg: Some(argument),
        ..
    } = &p.items[0].expr
    else {
        return None;
    };
    let BExpr::Function { name, args } = &**argument else {
        return None;
    };
    if name != "length" || args.len() != 1 {
        return None;
    }
    let BExpr::Var(projected) = &args[0] else {
        return None;
    };
    if projected != path_var {
        return None;
    }
    let anchor = part.element.node.var.as_ref()?;
    let terminal = terminal_node.var.as_ref()?;
    let conjuncts = split_conjuncts(m.where_clause.as_ref()?);
    let mut anchor_id = None;
    let mut terminal_id = None;
    for conjunct in &conjuncts {
        // Anything that is not an id equality -- a label, a property, or a
        // predicate over the path -- is outside this fast path.
        let (variable, id) = match_point_lookup(conjunct)?;
        if variable == *anchor {
            anchor_id = Some(id);
        } else if variable == *terminal {
            terminal_id = Some(id);
        } else {
            return None;
        }
    }
    let anchor_id = anchor_id?;
    let terminal_id = terminal_id?;
    // `*1..N` cannot answer an empty walk, so an anchor that is also the
    // terminal is left to the general expansion rather than special-cased here.
    if anchor_id == terminal_id {
        return None;
    }
    Some((terminal.clone(), terminal_id))
}

fn split_conjuncts(expr: &BExpr) -> Vec<BExpr> {
    match expr {
        BExpr::And(left, right) => {
            let mut conjuncts = split_conjuncts(left);
            conjuncts.extend(split_conjuncts(right));
            conjuncts
        }
        other => vec![other.clone()],
    }
}

fn match_point_lookup(expr: &BExpr) -> Option<(String, u64)> {
    let (lhs, rhs) = match expr {
        BExpr::Eq(lhs, rhs) => (lhs, rhs),
        _ => return None,
    };
    if let (BExpr::IdOf(inner), BExpr::Const(Value::Int(id))) = (&**lhs, &**rhs) {
        if let BExpr::Var(name) = &**inner {
            if *id >= 0 {
                return Some((name.clone(), *id as u64));
            }
        }
    }
    if let (BExpr::Const(Value::Int(id)), BExpr::IdOf(inner)) = (&**lhs, &**rhs) {
        if let BExpr::Var(name) = &**inner {
            if *id >= 0 {
                return Some((name.clone(), *id as u64));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::l4_compute::algebra::OpaquePayload;
    use metagdb_core::l4_compute::olap::AlgorithmRegistry;
    use std::sync::Arc;

    fn schema() -> Arc<dyn metagdb_core::l1_model::GraphSchema> {
        Arc::new(crate::schema::DynamicSchema::schemaless())
    }

    #[test]
    fn extracts_point_lookup_hint() {
        let m = MatchStage {
            optional: false,
            parts: vec![super::super::plan::BoundPatternPart {
                path_var: None,
                element: super::super::plan::BoundElement {
                    node: super::super::plan::BoundNode {
                        var: Some("n".into()),
                        labels: Vec::new(),
                        properties: Vec::new(),
                    },
                    chains: Vec::new(),
                },
            }],
            where_clause: Some(BExpr::Eq(
                Box::new(BExpr::IdOf(Box::new(BExpr::Var("n".into())))),
                Box::new(BExpr::Const(Value::Int(42))),
            )),
            id_hint: None,
            end_hint: None,
            shortest_path: false,
        };
        assert_eq!(extract_point_hint(&m), Some(("n".to_string(), 42)));
    }

    #[test]
    fn detects_shortest_path_hint() {
        use super::super::ast::Direction;
        use super::super::plan::{
            BoundElement, BoundNode, BoundPatternPart, BoundRel, ProjectItem,
        };

        let m = MatchStage {
            optional: false,
            parts: vec![BoundPatternPart {
                path_var: Some("p".into()),
                element: BoundElement {
                    node: BoundNode {
                        var: Some("s".into()),
                        labels: Vec::new(),
                        properties: Vec::new(),
                    },
                    chains: vec![(
                        BoundRel {
                            var: None,
                            types: Vec::new(),
                            direction: Direction::Outgoing,
                            min_hops: 1,
                            max_hops: Some(4),
                            properties: Vec::new(),
                        },
                        BoundNode {
                            var: Some("t".into()),
                            labels: Vec::new(),
                            properties: Vec::new(),
                        },
                    )],
                },
            }],
            where_clause: Some(BExpr::And(
                Box::new(BExpr::Eq(
                    Box::new(BExpr::IdOf(Box::new(BExpr::Var("s".into())))),
                    Box::new(BExpr::Const(Value::Int(10))),
                )),
                Box::new(BExpr::Eq(
                    Box::new(BExpr::IdOf(Box::new(BExpr::Var("t".into())))),
                    Box::new(BExpr::Const(Value::Int(20))),
                )),
            )),
            id_hint: None,
            end_hint: None,
            shortest_path: false,
        };
        let p = ProjectStage {
            distinct: false,
            items: vec![ProjectItem {
                expr: BExpr::Aggregate {
                    kind: AggregateKind::Min,
                    distinct: false,
                    arg: Some(Box::new(BExpr::Function {
                        name: "length".into(),
                        args: vec![BExpr::Var("p".into())],
                    })),
                },
                alias: "min(length(p))".into(),
                star: false,
            }],
            order_by: Vec::new(),
            skip: None,
            limit: None,
            where_clause: None,
        };
        assert_eq!(
            detect_shortest_path_hint(&m, &p),
            Some(("t".to_string(), 20))
        );
    }

    #[test]
    fn optimizer_marks_physical_plan() {
        let optimizer = MetagdbOptimizer;
        let query = LogicalQuery {
            stages: vec![Stage::Match(MatchStage {
                optional: false,
                parts: Vec::new(),
                where_clause: None,
                id_hint: None,
                end_hint: None,
                shortest_path: false,
            })],
            unions: Vec::new(),
        };
        let lp = LogicalPlan {
            root: LogicalOp::ExtensionOp {
                name: "cypher_pipeline".into(),
                payload: OpaquePayload::new(query),
                input: Box::new(LogicalOp::SingleRow),
            },
        };
        let ctx = OptContext {
            schema: schema(),
            algorithms: Arc::new(AlgorithmRegistry::default()),
        };
        let pp = optimizer.optimize(lp, &ctx).unwrap();
        assert!(pp.downcast_ref::<PhysicalQuery>().is_some());
    }
}
