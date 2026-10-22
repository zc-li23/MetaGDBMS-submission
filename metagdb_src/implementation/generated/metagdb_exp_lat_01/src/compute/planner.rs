//! Planner: binds the private Cypher AST against the graph schema and produces a
//! logical plan (a clause pipeline with interned identifiers).
//!
//! Name resolution is the planner's responsibility: labels, edge types, and
//! property keys are interned through the read-only `GraphSchema` trait, and
//! aggregate calls are tagged so the executor can group and fold.

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::{LogicalOp, LogicalPlan, OpaquePayload};
use metagdb_core::l4_compute::engine::{LanguageAst, Planner};
use metagdb_core::Result;

use super::ast::*;
use super::ir::{AggregateKind, BExpr, QuantifierKind, Value};
use super::plan::*;

/// Planner stage.
#[derive(Default)]
pub struct MetagdbPlanner;

impl Planner for MetagdbPlanner {
    fn plan(&self, ast: Box<dyn LanguageAst>, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
        if ast.language() != metagdb_core::l5_interaction::QueryLanguage::Cypher {
            return Err(metagdb_core::Error::Plan(
                "MetagdbPlanner only plans Cypher ASTs".into(),
            ));
        }
        let cypher = ast
            .as_any()
            .downcast_ref::<CypherAst>()
            .ok_or_else(|| metagdb_core::Error::Plan("expected CypherAst".into()))?
            .clone();
        let query = plan_query(&cypher.query, schema)?;
        Ok(LogicalPlan {
            root: LogicalOp::ExtensionOp {
                name: "cypher_pipeline".into(),
                payload: OpaquePayload::new(query),
                input: Box::new(LogicalOp::SingleRow),
            },
        })
    }
}

fn plan_query(query: &Query, schema: &dyn GraphSchema) -> Result<LogicalQuery> {
    let stages = bind_clauses(&query.head, schema)?;
    let mut unions = Vec::new();
    for union in &query.unions {
        unions.push((union.all, bind_clauses(&union.clauses, schema)?));
    }
    Ok(LogicalQuery { stages, unions })
}

fn bind_clauses(clauses: &[Clause], schema: &dyn GraphSchema) -> Result<Vec<Stage>> {
    let mut stages = Vec::new();
    for clause in clauses {
        stages.push(bind_clause(clause, schema)?);
    }
    Ok(stages)
}

fn bind_clause(clause: &Clause, schema: &dyn GraphSchema) -> Result<Stage> {
    Ok(match clause {
        Clause::Match(m) => Stage::Match(MatchStage {
            optional: m.optional,
            parts: m
                .parts
                .iter()
                .map(|part| bind_pattern_part(part, schema))
                .collect::<Result<Vec<_>>>()?,
            where_clause: match &m.where_clause {
                Some(expr) => Some(bind_expr(expr, schema)?),
                None => None,
            },
            id_hint: None,
            end_hint: None,
            shortest_path: false,
        }),
        Clause::With(body) => Stage::Project(bind_projection(body, schema)?),
        Clause::Return(body) => Stage::Project(bind_projection(body, schema)?),
        Clause::Unwind { expr, var } => Stage::Unwind {
            expr: bind_expr(expr, schema)?,
            var: var.clone(),
        },
        Clause::Create(parts) => Stage::Create(
            parts
                .iter()
                .map(|part| bind_pattern_part(part, schema))
                .collect::<Result<Vec<_>>>()?,
        ),
        Clause::Merge(m) => Stage::Merge(MergeStage {
            part: bind_pattern_part(&m.part, schema)?,
            on_create: bind_set_items(&m.on_create, schema)?,
            on_match: bind_set_items(&m.on_match, schema)?,
        }),
        Clause::Delete { detach, exprs } => Stage::Delete {
            detach: *detach,
            exprs: exprs
                .iter()
                .map(|expr| bind_expr(expr, schema))
                .collect::<Result<Vec<_>>>()?,
        },
        Clause::Set(items) => Stage::Set(bind_set_items(items, schema)?),
        Clause::Remove(items) => Stage::Remove(
            items
                .iter()
                .map(|item| bind_remove_item(item, schema))
                .collect::<Result<Vec<_>>>()?,
        ),
        Clause::Call(call) => Stage::Call(CallStage {
            name: call.name.to_ascii_lowercase(),
            args: bind_call_args(call, schema)?,
            yields: call.yields.clone(),
            yield_star: call.yield_star,
        }),
    })
}

fn bind_call_args(call: &CallClause, schema: &dyn GraphSchema) -> Result<Vec<(String, BExpr)>> {
    let mut args = Vec::new();
    let mut positional = 0usize;
    for (name, expr) in &call.args {
        let key = match name {
            Some(name) => name.clone(),
            None => {
                let resolved = super::algorithms::positional_parameter(&call.name, positional)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("arg{positional}"));
                positional += 1;
                resolved
            }
        };
        args.push((key, bind_expr(expr, schema)?));
    }
    Ok(args)
}

fn bind_projection(body: &ProjectionBody, schema: &dyn GraphSchema) -> Result<ProjectStage> {
    let mut items = Vec::new();
    for item in &body.items {
        if let Expr::Var(name) = &item.expr {
            if name == "*" {
                items.push(ProjectItem {
                    expr: BExpr::Var("*".into()),
                    alias: "*".into(),
                    star: true,
                });
                continue;
            }
        }
        let expr = bind_expr(&item.expr, schema)?;
        let alias = match &item.alias {
            Some(alias) => alias.clone(),
            None => ast_alias(&item.expr),
        };
        items.push(ProjectItem {
            expr,
            alias,
            star: false,
        });
    }
    Ok(ProjectStage {
        distinct: body.distinct,
        items,
        order_by: body
            .order_by
            .iter()
            .map(|(expr, desc)| Ok((bind_expr(expr, schema)?, *desc)))
            .collect::<Result<Vec<_>>>()?,
        skip: match &body.skip {
            Some(expr) => Some(bind_expr(expr, schema)?),
            None => None,
        },
        limit: match &body.limit {
            Some(expr) => Some(bind_expr(expr, schema)?),
            None => None,
        },
        where_clause: match &body.where_clause {
            Some(expr) => Some(bind_expr(expr, schema)?),
            None => None,
        },
    })
}

fn bind_set_items(items: &[SetItem], schema: &dyn GraphSchema) -> Result<Vec<BoundSetItem>> {
    items
        .iter()
        .map(|item| {
            Ok(match item {
                SetItem::Property { target, value } => {
                    let (element, key) = split_property_target(target, schema)?;
                    BoundSetItem::Property {
                        element,
                        key,
                        value: bind_expr(value, schema)?,
                    }
                }
                SetItem::Variable { name, value } => BoundSetItem::Variable {
                    name: name.clone(),
                    value: bind_expr(value, schema)?,
                },
                SetItem::Merge { name, value } => BoundSetItem::Merge {
                    name: name.clone(),
                    value: bind_expr(value, schema)?,
                },
                SetItem::Labels { name, labels } => BoundSetItem::Labels {
                    name: name.clone(),
                    labels: labels
                        .iter()
                        .map(|label| intern_label(schema, label))
                        .collect::<Result<Vec<_>>>()?,
                },
            })
        })
        .collect()
}

/// Split `n.key` into the bound element expression and the interned key.
fn split_property_target(
    target: &Expr,
    schema: &dyn GraphSchema,
) -> Result<(BExpr, metagdb_core::common::ids::PropertyKeyId)> {
    match target {
        Expr::Prop(inner, key) => Ok((
            bind_expr(inner, schema)?,
            intern_property_key(schema, key)?,
        )),
        _ => Err(metagdb_core::Error::Plan(
            "SET/REMOVE property target must be a property reference".into(),
        )),
    }
}

fn bind_remove_item(item: &RemoveItem, schema: &dyn GraphSchema) -> Result<BoundRemoveItem> {
    Ok(match item {
        RemoveItem::Property(expr) => {
            let (element, key) = split_property_target(expr, schema)?;
            BoundRemoveItem::Property { element, key }
        }
        RemoveItem::Labels { name, labels } => BoundRemoveItem::Labels {
            name: name.clone(),
            labels: labels
                .iter()
                .map(|label| intern_label(schema, label))
                .collect::<Result<Vec<_>>>()?,
        },
    })
}

fn bind_pattern_part(part: &PatternPart, schema: &dyn GraphSchema) -> Result<BoundPatternPart> {
    Ok(BoundPatternPart {
        path_var: part.path_var.clone(),
        element: bind_element(&part.element, schema)?,
    })
}

fn bind_element(element: &PatternElement, schema: &dyn GraphSchema) -> Result<BoundElement> {
    Ok(BoundElement {
        node: bind_node(&element.node, schema)?,
        chains: element
            .chains
            .iter()
            .map(|(rel, node)| Ok((bind_rel(rel, schema)?, bind_node(node, schema)?)))
            .collect::<Result<Vec<_>>>()?,
    })
}

fn bind_node(node: &NodePattern, schema: &dyn GraphSchema) -> Result<BoundNode> {
    Ok(BoundNode {
        var: node.var.clone(),
        labels: node
            .labels
            .iter()
            .map(|label| intern_label(schema, label))
            .collect::<Result<Vec<_>>>()?,
        properties: node
            .properties
            .iter()
            .map(|(key, value)| {
                Ok((
                    intern_property_key(schema, key)?,
                    bind_expr(value, schema)?,
                ))
            })
            .collect::<Result<Vec<_>>>()?,
    })
}

fn bind_rel(rel: &RelPattern, schema: &dyn GraphSchema) -> Result<BoundRel> {
    Ok(BoundRel {
        var: rel.var.clone(),
        types: rel
            .types
            .iter()
            .map(|name| intern_edge_type(schema, name))
            .collect::<Result<Vec<_>>>()?,
        direction: rel.direction,
        min_hops: rel.min_hops,
        max_hops: rel.max_hops,
        properties: rel
            .properties
            .iter()
            .map(|(key, value)| {
                Ok((
                    intern_property_key(schema, key)?,
                    bind_expr(value, schema)?,
                ))
            })
            .collect::<Result<Vec<_>>>()?,
    })
}

fn intern_label(schema: &dyn GraphSchema, name: &str) -> Result<metagdb_core::common::ids::LabelId> {
    schema
        .label_by_name(name)
        .ok_or_else(|| metagdb_core::Error::Schema(format!("label `{name}` cannot be resolved")))
}

fn intern_edge_type(
    schema: &dyn GraphSchema,
    name: &str,
) -> Result<metagdb_core::common::ids::EdgeTypeId> {
    schema
        .edge_type_by_name(name)
        .ok_or_else(|| metagdb_core::Error::Schema(format!("edge type `{name}` cannot be resolved")))
}

fn intern_property_key(
    schema: &dyn GraphSchema,
    name: &str,
) -> Result<metagdb_core::common::ids::PropertyKeyId> {
    schema.property_key_by_name(name).ok_or_else(|| {
        metagdb_core::Error::Schema(format!("property key `{name}` cannot be resolved"))
    })
}

/// Default output column name derived from the source expression text.
fn ast_alias(expr: &Expr) -> String {
    match expr {
        Expr::Var(name) => name.clone(),
        Expr::Prop(source, key) => format!("{}.{}", ast_alias(source), key),
        Expr::CountStar => "count(*)".into(),
        Expr::Function { name, args, .. } => {
            let inner: Vec<String> = args.iter().map(ast_alias).collect();
            format!("{name}({})", inner.join(", "))
        }
        Expr::Binary(op, lhs, rhs) => format!("{} {:?} {}", ast_alias(lhs), op, ast_alias(rhs)),
        Expr::ListIndex(list, _) => format!("{}[..]", ast_alias(list)),
        _ => "expr".into(),
    }
}

fn const_value(expr: &Expr) -> Option<Value> {
    match expr {
        Expr::Null => Some(Value::Null),
        Expr::Bool(b) => Some(Value::Bool(*b)),
        Expr::Int(i) => Some(Value::Int(*i)),
        Expr::Float(f) => Some(Value::Float(*f)),
        Expr::Str(s) => Some(Value::Text(s.clone())),
        Expr::Neg(inner) => match const_value(inner)? {
            Value::Int(i) => Some(Value::Int(-i)),
            Value::Float(f) => Some(Value::Float(-f)),
            _ => None,
        },
        _ => None,
    }
}

fn bind_expr(expr: &Expr, schema: &dyn GraphSchema) -> Result<BExpr> {
    if let Some(value) = const_value(expr) {
        return Ok(BExpr::Const(value));
    }
    Ok(match expr {
        Expr::Null | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => {
            BExpr::Const(const_value(expr).unwrap_or(Value::Null))
        }
        Expr::Parameter(name) => BExpr::Param(name.clone()),
        Expr::Var(name) => BExpr::Var(name.clone()),
        Expr::Prop(source, key) => BExpr::Prop(
            Box::new(bind_expr(source, schema)?),
            intern_property_key(schema, key)?,
        ),
        Expr::CountStar => BExpr::Aggregate {
            kind: AggregateKind::CountStar,
            distinct: false,
            arg: None,
        },
        Expr::List(items) => BExpr::List(
            items
                .iter()
                .map(|item| bind_expr(item, schema))
                .collect::<Result<Vec<_>>>()?,
        ),
        Expr::Map(entries) => BExpr::Map(
            entries
                .iter()
                .map(|(key, value)| Ok((key.clone(), bind_expr(value, schema)?)))
                .collect::<Result<Vec<_>>>()?,
        ),
        Expr::Neg(inner) => BExpr::Neg(Box::new(bind_expr(inner, schema)?)),
        Expr::Not(inner) => BExpr::Not(Box::new(bind_expr(inner, schema)?)),
        Expr::IsNull(inner) => BExpr::IsNull(Box::new(bind_expr(inner, schema)?)),
        Expr::IsNotNull(inner) => BExpr::IsNotNull(Box::new(bind_expr(inner, schema)?)),
        Expr::Binary(op, lhs, rhs) => {
            let lhs = Box::new(bind_expr(lhs, schema)?);
            let rhs = Box::new(bind_expr(rhs, schema)?);
            match op {
                BinOp::Add => BExpr::Add(lhs, rhs),
                BinOp::Sub => BExpr::Sub(lhs, rhs),
                BinOp::Mul => BExpr::Mul(lhs, rhs),
                BinOp::Div => BExpr::Div(lhs, rhs),
                BinOp::Mod => BExpr::Mod(lhs, rhs),
                BinOp::Pow => BExpr::Pow(lhs, rhs),
                BinOp::Eq => BExpr::Eq(lhs, rhs),
                BinOp::Neq => BExpr::Neq(lhs, rhs),
                BinOp::Lt => BExpr::Lt(lhs, rhs),
                BinOp::Lte => BExpr::Lte(lhs, rhs),
                BinOp::Gt => BExpr::Gt(lhs, rhs),
                BinOp::Gte => BExpr::Gte(lhs, rhs),
                BinOp::And => BExpr::And(lhs, rhs),
                BinOp::Or => BExpr::Or(lhs, rhs),
                BinOp::Xor => BExpr::Xor(lhs, rhs),
                BinOp::In => BExpr::In(lhs, rhs),
                BinOp::StartsWith => BExpr::StartsWith(lhs, rhs),
                BinOp::EndsWith => BExpr::EndsWith(lhs, rhs),
                BinOp::Contains => BExpr::Contains(lhs, rhs),
            }
        }
        Expr::ListIndex(list, index) => BExpr::ListIndex(
            Box::new(bind_expr(list, schema)?),
            Box::new(bind_expr(index, schema)?),
        ),
        Expr::ListSlice { list, start, end } => BExpr::ListSlice {
            list: Box::new(bind_expr(list, schema)?),
            start: match start {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
            end: match end {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
        },
        Expr::Function {
            name,
            distinct,
            args,
        } => bind_function(name, *distinct, args, schema)?,
        Expr::Case {
            test,
            whens,
            else_branch,
        } => BExpr::Case {
            test: match test {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
            whens: whens
                .iter()
                .map(|(when, then)| Ok((bind_expr(when, schema)?, bind_expr(then, schema)?)))
                .collect::<Result<Vec<_>>>()?,
            else_branch: match else_branch {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
        },
        Expr::ListComprehension {
            var,
            list,
            filter,
            map,
        } => BExpr::ListComprehension {
            var: var.clone(),
            list: Box::new(bind_expr(list, schema)?),
            filter: match filter {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
            map: match map {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
        },
        Expr::Quantifier {
            kind,
            var,
            list,
            predicate,
        } => BExpr::Quantifier {
            kind: match kind {
                QuantKind::All => QuantifierKind::All,
                QuantKind::Any => QuantifierKind::Any,
                QuantKind::None => QuantifierKind::None,
                QuantKind::Single => QuantifierKind::Single,
            },
            var: var.clone(),
            list: Box::new(bind_expr(list, schema)?),
            predicate: match predicate {
                Some(expr) => Some(Box::new(bind_expr(expr, schema)?)),
                None => None,
            },
        },
        Expr::LabelTest(inner, labels) => BExpr::LabelTest(
            Box::new(bind_expr(inner, schema)?),
            labels
                .iter()
                .map(|label| intern_label(schema, label))
                .collect::<Result<Vec<_>>>()?,
        ),
    })
}

fn bind_function(
    name: &str,
    distinct: bool,
    args: &[Expr],
    schema: &dyn GraphSchema,
) -> Result<BExpr> {
    let lowered = name.to_ascii_lowercase();
    let bound: Vec<BExpr> = args
        .iter()
        .map(|arg| bind_expr(arg, schema))
        .collect::<Result<Vec<_>>>()?;
    let aggregate = match lowered.as_str() {
        "count" if bound.is_empty() => Some((AggregateKind::CountStar, None)),
        "count" => Some((AggregateKind::Count, Some(bound[0].clone()))),
        "sum" => Some((AggregateKind::Sum, Some(bound[0].clone()))),
        "avg" => Some((AggregateKind::Avg, Some(bound[0].clone()))),
        "min" => Some((AggregateKind::Min, Some(bound[0].clone()))),
        "max" => Some((AggregateKind::Max, Some(bound[0].clone()))),
        "collect" => Some((AggregateKind::Collect, Some(bound[0].clone()))),
        _ => None,
    };
    if let Some((kind, arg)) = aggregate {
        return Ok(BExpr::Aggregate {
            kind,
            distinct,
            arg: arg.map(Box::new),
        });
    }
    match lowered.as_str() {
        "id" if bound.len() == 1 => Ok(BExpr::IdOf(Box::new(bound[0].clone()))),
        "labels" if bound.len() == 1 => Ok(BExpr::LabelsOf(Box::new(bound[0].clone()))),
        _ => Ok(BExpr::Function {
            name: lowered,
            args: bound,
        }),
    }
}
