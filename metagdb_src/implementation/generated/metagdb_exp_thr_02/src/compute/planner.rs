//! Logical planner: private Cypher AST -> framework `LogicalPlan`.
//!
//! Each Cypher clause becomes one logical operator.  The relational shape uses
//! framework operators (`SingleRow`, `Union`, `GraphProjection`,
//! `CallAlgorithm`) and the Cypher-specific operators travel as named
//! `ExtensionOp` payloads defined in `physical_ops`.  Names are bound through
//! the `GraphSchema` while planning, so a plan never carries an unvalidated
//! label, edge type or property key.

use std::collections::BTreeMap;

use metagdb_core::common::property::PropertyValue;
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, LogicalOp, LogicalPlan, OpaquePayload, ProjectionDirection,
    ProjectionSpec,
};
use metagdb_core::l4_compute::engine::{LanguageAst, Planner};
use metagdb_core::{Error, Result};

use super::algorithms::parameter_name;
use super::cypher_ast::*;
use super::physical_ops::*;

/// Framework planner producing a Cypher logical plan.
#[derive(Default)]
pub struct MetagdbPlanner;

impl Planner for MetagdbPlanner {
    fn plan(&self, ast: Box<dyn LanguageAst>, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
        let cypher = ast
            .as_any()
            .downcast_ref::<CypherAst>()
            .ok_or_else(|| Error::Plan("planner received a non-Cypher AST".into()))?;
        match cypher {
            CypherAst::Query(query) => plan_query(query, schema),
            CypherAst::Call { name, args, yields } => Ok(LogicalPlan {
                root: plan_call(name, args, yields.clone())?,
            }),
        }
    }
}

fn plan_query(query: &Query, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
    let mut branches = Vec::new();
    let mut any_distinct = false;
    for (index, (clauses, all)) in query.branches.iter().enumerate() {
        let plan = plan_branch(clauses, schema)?;
        if index > 0 && !*all {
            any_distinct = true;
        }
        branches.push(plan);
    }
    bind_query(query, schema)?;
    if branches.len() == 1 {
        return Ok(LogicalPlan {
            root: branches.into_iter().next().expect("one branch"),
        });
    }
    Ok(LogicalPlan {
        root: LogicalOp::Union {
            all: !any_distinct,
            inputs: branches,
        },
    })
}

fn plan_branch(clauses: &[Clause], schema: &dyn GraphSchema) -> Result<LogicalOp> {
    let mut root = LogicalOp::SingleRow;
    let mut index = 0;
    while index < clauses.len() {
        let clause = &clauses[index];
        // Group consecutive updating clauses so they share one physical pass.
        if let Some(payload) = mutation_of(clause) {
            let mut payloads = vec![payload];
            let mut next = index + 1;
            while next < clauses.len() {
                match mutation_of(&clauses[next]) {
                    Some(payload) => {
                        payloads.push(payload);
                        next += 1;
                    }
                    None => break,
                }
            }
            for payload in payloads {
                root = LogicalOp::ExtensionOp {
                    name: "cypher.mutate".into(),
                    payload: OpaquePayload::new(payload),
                    input: Box::new(root),
                };
            }
            index = next;
            continue;
        }
        root = plan_clause(clause, root, schema)?;
        index += 1;
    }
    Ok(root)
}

fn mutation_of(clause: &Clause) -> Option<MutationPayload> {
    match clause {
        Clause::Create { pattern } => Some(MutationPayload::Create(pattern.clone())),
        Clause::Merge {
            pattern,
            on_create,
            on_match,
        } => Some(MutationPayload::Merge {
            pattern: pattern.clone(),
            on_create: on_create.clone(),
            on_match: on_match.clone(),
        }),
        Clause::Set { items } => Some(MutationPayload::Set(items.clone())),
        Clause::Remove { items } => Some(MutationPayload::Remove(items.clone())),
        Clause::Delete { detach, exprs } => Some(MutationPayload::Delete {
            detach: *detach,
            exprs: exprs.clone(),
        }),
        _ => None,
    }
}

fn plan_clause(clause: &Clause, input: LogicalOp, _schema: &dyn GraphSchema) -> Result<LogicalOp> {
    match clause {
        Clause::Match {
            optional,
            pattern,
            predicate,
        } => Ok(LogicalOp::ExtensionOp {
            name: "cypher.match".into(),
            payload: OpaquePayload::new(MatchPayload {
                optional: *optional,
                pattern: pattern.clone(),
                predicate: predicate.clone(),
            }),
            input: Box::new(input),
        }),
        Clause::Unwind { expr, var } => Ok(LogicalOp::ExtensionOp {
            name: "cypher.unwind".into(),
            payload: OpaquePayload::new(UnwindPayload {
                expr: expr.clone(),
                var: var.clone(),
            }),
            input: Box::new(input),
        }),
        Clause::With {
            distinct,
            items,
            order,
            skip,
            limit,
            predicate,
        } => Ok(LogicalOp::ExtensionOp {
            name: "cypher.project".into(),
            payload: OpaquePayload::new(ProjectPayload {
                is_return: false,
                distinct: *distinct,
                items: items.clone(),
                order: order.clone(),
                skip: skip.clone(),
                limit: limit.clone(),
                predicate: predicate.clone(),
            }),
            input: Box::new(input),
        }),
        Clause::Return {
            distinct,
            items,
            order,
            skip,
            limit,
        } => Ok(LogicalOp::ExtensionOp {
            name: "cypher.project".into(),
            payload: OpaquePayload::new(ProjectPayload {
                is_return: true,
                distinct: *distinct,
                items: items.clone(),
                order: order.clone(),
                skip: skip.clone(),
                limit: limit.clone(),
                predicate: None,
            }),
            input: Box::new(input),
        }),
        Clause::Call { name, args, yields } => {
            let call = plan_call(name, args, yields.clone())?;
            Ok(match input {
                // A standalone CALL is already the whole pipeline.
                LogicalOp::SingleRow => call,
                other => LogicalOp::ExtensionOp {
                    name: "cypher.call".into(),
                    payload: OpaquePayload::new(CallPayload {
                        name: name.clone(),
                        args: args.clone(),
                        yields: yields.clone(),
                    }),
                    input: Box::new(other),
                },
            })
        }
        Clause::Create { .. }
        | Clause::Merge { .. }
        | Clause::Set { .. }
        | Clause::Remove { .. }
        | Clause::Delete { .. } => unreachable!("mutations are grouped by plan_branch"),
    }
}

/// Lower a procedure call into `GraphProjection` + `CallAlgorithm` (+ `cypher.yield`).
fn plan_call(
    name: &str,
    args: &[ProcArg],
    yields: Option<YieldSpec>,
) -> Result<LogicalOp> {
    let params = bind_arguments(name, args)?;
    let mut root = LogicalOp::CallAlgorithm {
        name: name.to_string(),
        args: AlgorithmArgs { params },
        input: Box::new(LogicalOp::GraphProjection {
            spec: ProjectionSpec {
                vertex_labels: None,
                edge_types: None,
                direction: ProjectionDirection::Outgoing,
                vertex_properties: None,
            },
        }),
    };
    if let Some(spec) = yields {
        root = LogicalOp::ExtensionOp {
            name: "cypher.yield".into(),
            payload: OpaquePayload::new(spec),
            input: Box::new(root),
        };
    }
    Ok(root)
}

/// Positional arguments map onto the algorithm's declared parameter order.
fn bind_arguments(name: &str, args: &[ProcArg]) -> Result<BTreeMap<String, PropertyValue>> {
    let mut params = BTreeMap::new();
    let mut position = 0usize;
    for arg in args {
        match arg {
            ProcArg::Named(key, expr) => {
                params.insert(key.clone(), constant_of(expr)?);
            }
            ProcArg::Positional(expr) => {
                let key = parameter_name(name, position).ok_or_else(|| {
                    Error::Plan(format!(
                        "procedure `{name}` does not accept a positional argument at position {position}"
                    ))
                })?;
                params.insert(key.to_string(), constant_of(expr)?);
                position += 1;
            }
        }
    }
    Ok(params)
}

fn constant_of(expr: &Expr) -> Result<PropertyValue> {
    match expr {
        Expr::Null => Ok(PropertyValue::Null),
        Expr::Bool(value) => Ok(PropertyValue::Bool(*value)),
        Expr::Int(value) => Ok(PropertyValue::Int(*value)),
        Expr::Float(value) => Ok(PropertyValue::Float(*value)),
        Expr::Str(value) => Ok(PropertyValue::Text(value.clone())),
        other => Err(Error::Plan(format!(
            "algorithm arguments must be scalar literals, found {other:?}"
        ))),
    }
}

/// Resolve every metadata name in the query through the schema.
fn bind_query(query: &Query, schema: &dyn GraphSchema) -> Result<()> {
    for (clauses, _) in &query.branches {
        for clause in clauses {
            bind_clause(clause, schema)?;
        }
    }
    Ok(())
}

fn bind_clause(clause: &Clause, schema: &dyn GraphSchema) -> Result<()> {
    match clause {
        Clause::Match {
            pattern, predicate, ..
        } => {
            bind_pattern(pattern, schema)?;
            if let Some(predicate) = predicate {
                bind_expr(predicate, schema)?;
            }
        }
        Clause::Create { pattern } => bind_pattern(pattern, schema)?,
        Clause::Merge {
            pattern,
            on_create,
            on_match,
        } => {
            bind_pattern(pattern, schema)?;
            for item in on_create.iter().chain(on_match.iter()) {
                bind_set_item(item, schema)?;
            }
        }
        Clause::Set { items } => {
            for item in items {
                bind_set_item(item, schema)?;
            }
        }
        Clause::Remove { items } => {
            for item in items {
                match item {
                    RemoveItem::Property { target, key } => {
                        bind_expr(target, schema)?;
                        require_property_key(schema, key)?;
                    }
                    RemoveItem::Labels { labels, .. } => {
                        for label in labels {
                            require_label(schema, label)?;
                        }
                    }
                }
            }
        }
        Clause::Delete { exprs, .. } => {
            for expr in exprs {
                bind_expr(expr, schema)?;
            }
        }
        Clause::Unwind { expr, .. } => bind_expr(expr, schema)?,
        Clause::Call { args, .. } => {
            for arg in args {
                match arg {
                    ProcArg::Named(_, expr) | ProcArg::Positional(expr) => {
                        bind_expr(expr, schema)?
                    }
                }
            }
        }
        Clause::With {
            items,
            order,
            skip,
            limit,
            predicate,
            ..
        } => {
            bind_projection(items, order, skip, limit, schema)?;
            if let Some(predicate) = predicate {
                bind_expr(predicate, schema)?;
            }
        }
        Clause::Return {
            items,
            order,
            skip,
            limit,
            ..
        } => bind_projection(items, order, skip, limit, schema)?,
    }
    Ok(())
}

fn bind_projection(
    items: &[ReturnItem],
    order: &[SortItem],
    skip: &Option<Expr>,
    limit: &Option<Expr>,
    schema: &dyn GraphSchema,
) -> Result<()> {
    for item in items {
        bind_expr(&item.expr, schema)?;
    }
    for sort in order {
        bind_expr(&sort.expr, schema)?;
    }
    if let Some(skip) = skip {
        bind_expr(skip, schema)?;
    }
    if let Some(limit) = limit {
        bind_expr(limit, schema)?;
    }
    Ok(())
}

fn bind_set_item(item: &SetItem, schema: &dyn GraphSchema) -> Result<()> {
    match item {
        SetItem::Property { target, key, value } => {
            bind_expr(target, schema)?;
            bind_expr(value, schema)?;
            require_property_key(schema, key)?;
        }
        SetItem::Labels { labels, .. } => {
            for label in labels {
                require_label(schema, label)?;
            }
        }
    }
    Ok(())
}

fn bind_pattern(pattern: &Pattern, schema: &dyn GraphSchema) -> Result<()> {
    for part in &pattern.parts {
        let element = &part.element;
        bind_node(&element.node, schema)?;
        for (edge, node) in &element.chains {
            for label in &edge.types {
                require_edge_type(schema, label)?;
            }
            bind_properties(&edge.properties, schema)?;
            bind_node(node, schema)?;
        }
    }
    Ok(())
}

fn bind_node(node: &NodePattern, schema: &dyn GraphSchema) -> Result<()> {
    for label in &node.labels {
        require_label(schema, label)?;
    }
    bind_properties(&node.properties, schema)
}

fn bind_properties(properties: &Option<Expr>, schema: &dyn GraphSchema) -> Result<()> {
    if let Some(Expr::Map(entries)) = properties {
        for (key, value) in entries {
            require_property_key(schema, key)?;
            bind_expr(value, schema)?;
        }
    } else if let Some(expr) = properties {
        bind_expr(expr, schema)?;
    }
    Ok(())
}

fn bind_expr(expr: &Expr, schema: &dyn GraphSchema) -> Result<()> {
    match expr {
        Expr::Property(base, key) => {
            bind_expr(base, schema)?;
            require_property_key(schema, key)?;
        }
        Expr::Map(entries) => {
            for (key, value) in entries {
                require_property_key(schema, key)?;
                bind_expr(value, schema)?;
            }
        }
        Expr::List(items) => {
            for item in items {
                bind_expr(item, schema)?;
            }
        }
        Expr::Index(a, b) | Expr::In(a, b) | Expr::StartsWith(a, b) | Expr::EndsWith(a, b)
        | Expr::Contains(a, b) | Expr::Binary(_, a, b) => {
            bind_expr(a, schema)?;
            bind_expr(b, schema)?;
        }
        Expr::Slice { base, start, end } => {
            bind_expr(base, schema)?;
            if let Some(start) = start {
                bind_expr(start, schema)?;
            }
            if let Some(end) = end {
                bind_expr(end, schema)?;
            }
        }
        Expr::Negate(inner) | Expr::Not(inner) | Expr::IsNull(inner) | Expr::IsNotNull(inner) => {
            bind_expr(inner, schema)?
        }
        Expr::HasLabels(inner, labels) => {
            bind_expr(inner, schema)?;
            for label in labels {
                require_label(schema, label)?;
            }
        }
        Expr::ListComprehension {
            list, filter, map, ..
        } => {
            bind_expr(list, schema)?;
            if let Some(filter) = filter {
                bind_expr(filter, schema)?;
            }
            if let Some(map) = map {
                bind_expr(map, schema)?;
            }
        }
        Expr::Quantifier { list, filter, .. } => {
            bind_expr(list, schema)?;
            if let Some(filter) = filter {
                bind_expr(filter, schema)?;
            }
        }
        Expr::Case {
            subject,
            alternatives,
            default,
        } => {
            if let Some(subject) = subject {
                bind_expr(subject, schema)?;
            }
            for (when, then) in alternatives {
                bind_expr(when, schema)?;
                bind_expr(then, schema)?;
            }
            if let Some(default) = default {
                bind_expr(default, schema)?;
            }
        }
        Expr::Function { args, .. } => {
            for arg in args {
                bind_expr(arg, schema)?;
            }
        }
        Expr::PatternExists(pattern) => bind_pattern(pattern, schema)?,
        _ => {}
    }
    Ok(())
}

fn require_label(schema: &dyn GraphSchema, label: &str) -> Result<()> {
    schema
        .label_by_name(label)
        .map(|_| ())
        .ok_or_else(|| Error::Schema(format!("vertex label `{label}` is not declared")))
}

fn require_edge_type(schema: &dyn GraphSchema, edge_type: &str) -> Result<()> {
    schema
        .edge_type_by_name(edge_type)
        .map(|_| ())
        .ok_or_else(|| Error::Schema(format!("edge type `{edge_type}` is not declared")))
}

fn require_property_key(schema: &dyn GraphSchema, key: &str) -> Result<()> {
    schema
        .property_key_by_name(key)
        .map(|_| ())
        .ok_or_else(|| Error::Schema(format!("property key `{key}` is not declared")))
}

/// Framework traversal direction corresponding to a pattern's arrow syntax.
pub fn expand_direction(direction: Direction) -> ExpandDirection {
    match direction {
        Direction::Outgoing => ExpandDirection::Outgoing,
        Direction::Incoming => ExpandDirection::Incoming,
        Direction::Both => ExpandDirection::Both,
    }
}
