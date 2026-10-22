/**
 * @brief: MetagdbPlanner: component providing generation of logical execution plans, provided by metagdb_default.
 *
 * Translates CypherAst to LogicalPlan (logical algebra tree). Contains a binder sub-phase
 * responsible for resolving string names to schema IDs, and binding AstExpr to Expr.
 * DML operators temporarily carry semantics through ExtensionOp.
 */

use std::collections::BTreeMap;

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, JoinKind, LogicalOp, LogicalPlan, OpaquePayload,
    ProjectionDirection, ProjectionSpec, SortOrder as CoreSortOrder,
};
use metagdb_core::l4_compute::engine::{LanguageAst, Planner};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};
use metagdb_core::l5_interaction::QueryLanguage;
use metagdb_core::{Error, Result};

use super::cypher_ast::{
    AstExpr, Clause, CreateClause, CypherAst, DeleteClause, EdgeDirection, MatchClause,
    MatchPattern, MergeAction, MergeClause, PatternEdge, PatternElement, RemoveClause,
    RemoveItem, ReturnClause, ReturnItem, SetClause, SetItem, SortItem, UnwindClause,
    WithClause,
};
use super::physical_ops::{
    CreateEdgePayload, CreateNodePayload, DeletePayload, MergePayload, MergeTarget,
    RemoveLabelsPayload, RemovePropertyPayload, SetLabelsPayload, SetPropertyPayload,
    UnwindPayload, VarLengthExpandPayload,
};

/**
 * @brief: Default execution plan generator.
 */
#[derive(Default)]
pub struct MetagdbPlanner;

impl Planner for MetagdbPlanner {
    /**
     * @brief: Generate a logical plan. First routes by AST language, then downcasts to the concrete AST.
     */
    fn plan(&self, ast: Box<dyn LanguageAst>, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
        match ast.language() {
            QueryLanguage::Cypher => plan_cypher(&*ast, schema),
            other => Err(Error::Plan(format!(
                "MetagdbPlanner language not yet supported: {other:?}"
            ))),
        }
    }
}

/**
 * @brief: Cypher AST routing entry point.
 */
fn plan_cypher(ast: &dyn LanguageAst, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
    let cy = ast
        .as_any()
        .downcast_ref::<CypherAst>()
        .ok_or_else(|| Error::Plan("AST declared as Cypher but actual type is not CypherAst".into()))?;
    match cy {
        CypherAst::Query(query) => plan_query(query, schema),
        CypherAst::CallAlgorithm { algo, args } => Ok(plan_call(algo.clone(), args.clone())),
    }
}

/**
 * @brief: Build LogicalOp sequentially in clause chain order.
 */
fn plan_query(
    query: &super::cypher_ast::Query,
    schema: &dyn GraphSchema,
) -> Result<LogicalPlan> {
    let mut root: Option<LogicalOp> = None;

    for clause in &query.clauses {
        let child = root.map(Box::new);
        root = Some(plan_clause(clause, child, schema)?);
    }

    if let Some(ref union_clause) = query.unions {
        // Conservative strategy: if any branch uses UNION (non-ALL), treat the whole as requiring dedup
        let any_distinct = union_clause.branches.iter().any(|b| !b.all);
        let mut inputs: Vec<LogicalOp> = Vec::new();
        if let Some(main) = root {
            inputs.push(main);
        }
        for branch in &union_clause.branches {
            let branch_plan = plan_query(&branch.query, schema)?;
            inputs.push(branch_plan.root);
        }
        root = Some(LogicalOp::Union {
            all: !any_distinct,
            inputs,
        });
    }

    match root {
        Some(op) => Ok(LogicalPlan { root: op }),
        None => Err(Error::Plan("No valid clauses in the query".into())),
    }
}

/**
 * @brief: Clause dispatch.
 */
fn plan_clause(
    clause: &Clause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    match clause {
        Clause::Match(mc) => plan_match_clause(mc, input, schema),
        Clause::OptionalMatch(mc) => plan_optional_match(mc, input, schema),
        Clause::With(wc) => plan_with(wc, input, schema),
        Clause::Return(rc) => plan_return(rc, input, schema),
        Clause::Unwind(uc) => plan_unwind(uc, input, schema),
        Clause::Create(cc) => plan_create(cc, input, schema),
        Clause::Merge(mc) => plan_merge(mc, input, schema),
        Clause::Delete(dc) => plan_delete(dc, input, schema),
        Clause::Set(sc) => plan_set(sc, input, schema),
        Clause::Remove(rc) => plan_remove(rc, input, schema),
        Clause::CallSubquery(call) => {
            if input.is_some() {
                return Err(Error::Plan("CALL subquery does not support a preceding operator".into()));
            }
            Ok(plan_call(call.algo.clone(), call.args.clone()).root)
        }
    }
}

// ============================================================
// MATCH
// ============================================================

fn plan_match_clause(
    mc: &MatchClause,
    _input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    plan_match_pattern(&mc.pattern, mc.where_clause.as_ref(), schema)
}

fn plan_optional_match(
    mc: &MatchClause,
    _input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let inner = plan_match_pattern(&mc.pattern, mc.where_clause.as_ref(), schema)?;
    Ok(LogicalOp::ExtensionOp {
        name: "OptionalMatch".into(),
        payload: OpaquePayload::new("optional_match"),
        input: Box::new(inner),
    })
}

fn plan_match_pattern(
    pattern: &MatchPattern,
    where_clause: Option<&AstExpr>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    if pattern.elements.is_empty() {
        return Err(Error::Plan("MATCH pattern is empty".into()));
    }

    let mut element_ops: Vec<LogicalOp> = Vec::new();
    for element in &pattern.elements {
        let op = plan_pattern_element(element, where_clause, schema)?;
        element_ops.push(op);
    }

    if element_ops.len() == 1 {
        Ok(element_ops.into_iter().next().unwrap())
    } else {
        // Multiple MATCH elements → build left-deep Cartesian product tree
        let mut root = element_ops.remove(0);
        for right in element_ops {
            root = LogicalOp::Join {
                kind: JoinKind::Cross,
                on: vec![],
                left: Box::new(root),
                right: Box::new(right),
            };
        }
        Ok(root)
    }
}

fn plan_pattern_element(
    element: &PatternElement,
    where_clause: Option<&AstExpr>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let vertex_var = element.node.var.clone().unwrap_or_else(|| "__anon".into());

    let label_id = match &element.node.label {
        Some(name) => Some(
            schema
                .label_by_name(name)
                .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?,
        ),
        None => None,
    };

    let mut root = LogicalOp::VertexScan {
        var: vertex_var.clone(),
        label: label_id,
    };

    // Inline vertex property filter: convert {key: val} to Filter(n.key = val)
    if let Some(ref node_props) = element.node.properties {
        for (key, val) in node_props {
            let key_id = schema
                .property_key_by_name(key)
                .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
            let prop_expr = Expr::PropRef {
                source: Box::new(Expr::VarRef {
                    name: vertex_var.clone(),
                }),
                key: key_id,
            };
            let val_expr = bind_expr(val, schema)?;
            let predicate = Expr::Eq(Box::new(prop_expr), Box::new(val_expr));
            root = LogicalOp::Filter {
                predicate,
                input: Box::new(root),
            };
        }
    }

    let mut current_from = vertex_var;
    for edge in &element.edges {
        root = plan_expand(edge, &current_from, root, schema)?;
        current_from = edge.target.var.clone().unwrap_or_else(|| "__anon".into());
    }

    if let Some(wc) = where_clause {
        let predicate = bind_expr(wc, schema)?;
        root = LogicalOp::Filter {
            predicate,
            input: Box::new(root),
        };
    }

    Ok(root)
}

fn plan_expand(
    edge: &PatternEdge,
    from_var: &str,
    input: LogicalOp,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let to_var = edge.target.var.clone().unwrap_or_else(|| "__anon".into());
    let edge_types = bind_edge_types(&edge.edge_types, schema)?;
    let direction = convert_direction(edge.direction);

    // Variable-length path: directly use VarLengthExpand (replaces basic Expand)
    if let Some(ref range) = edge.range {
        let min_hops = range.start.unwrap_or(1) as usize;
        let max_hops = range.end.map(|e| e as usize);
        return Ok(LogicalOp::ExtensionOp {
            name: "VarLengthExpand".into(),
            payload: OpaquePayload::new(VarLengthExpandPayload {
                from: from_var.to_string(),
                to: to_var.clone(),
                edge: edge.edge_var.clone(),
                direction,
                edge_types,
                min_hops,
                max_hops,
            }),
            input: Box::new(input),
        });
    }

    // Fixed-length single hop: use basic Expand
    let mut root = LogicalOp::Expand {
        from: from_var.to_string(),
        to: to_var.clone(),
        edge: edge.edge_var.clone(),
        direction,
        edge_types: edge_types.clone(),
        input: Box::new(input),
    };

    // Inline edge property filter
    if let Some(ref edge_props) = edge.properties {
        let edge_filter_var = edge.edge_var.clone().unwrap_or_else(|| "__anon_edge".into());
        for (key, val) in edge_props {
            let key_id = schema
                .property_key_by_name(key)
                .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
            let prop_expr = Expr::PropRef {
                source: Box::new(Expr::VarRef {
                    name: edge_filter_var.clone(),
                }),
                key: key_id,
            };
            let val_expr = bind_expr(val, schema)?;
            let predicate = Expr::Eq(Box::new(prop_expr), Box::new(val_expr));
            root = LogicalOp::Filter {
                predicate,
                input: Box::new(root),
            };
        }
    }

    // Inline target vertex property filter
    if let Some(ref target_props) = edge.target.properties {
        for (key, val) in target_props {
            let key_id = schema
                .property_key_by_name(key)
                .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
            let prop_expr = Expr::PropRef {
                source: Box::new(Expr::VarRef {
                    name: to_var.clone(),
                }),
                key: key_id,
            };
            let val_expr = bind_expr(val, schema)?;
            let predicate = Expr::Eq(Box::new(prop_expr), Box::new(val_expr));
            root = LogicalOp::Filter {
                predicate,
                input: Box::new(root),
            };
        }
    }

    Ok(root)
}

// ============================================================
// WITH / RETURN
// ============================================================

// ============================================================
// Aggregate detection helpers
// ============================================================

/// Set of built-in aggregate function names
const AGGREGATE_FNS: &[&str] = &["count", "sum", "avg", "min", "max", "collect"];

/**
 * @brief: Determine whether a function name is an aggregate function.
 */
fn is_aggregate_fn(name: &str) -> bool {
    AGGREGATE_FNS.contains(&name.to_lowercase().as_str())
}

/**
 * @brief: Recursively check whether the expression tree contains aggregate function calls.
 */
fn contains_aggregate(expr: &AstExpr) -> bool {
    match expr {
        AstExpr::FunctionCall { name, .. } if is_aggregate_fn(name) => true,
        AstExpr::Neg(e)
        | AstExpr::Not(e)
        | AstExpr::IsNull(e)
        | AstExpr::IsNotNull(e) => contains_aggregate(e),
        AstExpr::Add(l, r)
        | AstExpr::Sub(l, r)
        | AstExpr::Mul(l, r)
        | AstExpr::Div(l, r)
        | AstExpr::Mod(l, r)
        | AstExpr::Pow(l, r)
        | AstExpr::Eq(l, r)
        | AstExpr::Neq(l, r)
        | AstExpr::Gt(l, r)
        | AstExpr::Gte(l, r)
        | AstExpr::Lt(l, r)
        | AstExpr::Lte(l, r)
        | AstExpr::And(l, r)
        | AstExpr::Or(l, r)
        | AstExpr::Xor(l, r)
        | AstExpr::StartsWith(l, r)
        | AstExpr::EndsWith(l, r)
        | AstExpr::Contains(l, r)
        | AstExpr::RegexMatch(l, r)
        | AstExpr::Concat(l, r)
        | AstExpr::In(l, r) => contains_aggregate(l) || contains_aggregate(r),
        AstExpr::List(items) => items.iter().any(contains_aggregate),
        AstExpr::ListIndex { list, index } => contains_aggregate(list) || contains_aggregate(index),
        AstExpr::Case { test, cases, else_branch } => {
            test.as_ref().map(|t| contains_aggregate(t)).unwrap_or(false)
                || cases.iter().any(|(w, t)| contains_aggregate(w) || contains_aggregate(t))
                || else_branch.as_ref().map(|e| contains_aggregate(e)).unwrap_or(false)
        }
        _ => false,
    }
}

/**
 * @brief: Convert aggregate function call to AggregateExpr.
 * @param name: Function name.
 * @param distinct: Whether DISTINCT.
 * @param arg: Function argument expression.
 * @param schema: Schema reference.
 * @return: The corresponding AggregateExpr.
 */
fn ast_agg_to_aggregate_expr(
    name: &str,
    distinct: bool,
    arg: Option<&AstExpr>,
    schema: &dyn GraphSchema,
) -> Result<AggregateExpr> {
    let bind = |e: &AstExpr| bind_expr(e, schema);
    match (name.to_lowercase().as_str(), distinct) {
        ("count", false) if arg.is_none() => Ok(AggregateExpr::CountAll),
        ("count", false) => Ok(AggregateExpr::Count(bind(arg.unwrap())?)),
        ("count", true) => Ok(AggregateExpr::CountDistinct(bind(arg.unwrap())?)),
        ("sum", false) => Ok(AggregateExpr::Sum(bind(arg.unwrap())?)),
        ("sum", true) => Ok(AggregateExpr::SumDistinct(bind(arg.unwrap())?)),
        ("avg", false) => Ok(AggregateExpr::Avg(bind(arg.unwrap())?)),
        ("min", false) => Ok(AggregateExpr::Min(bind(arg.unwrap())?)),
        ("max", false) => Ok(AggregateExpr::Max(bind(arg.unwrap())?)),
        ("collect", false) => Ok(AggregateExpr::Collect(bind(arg.unwrap())?)),
        ("collect", true) => Ok(AggregateExpr::CollectDistinct(bind(arg.unwrap())?)),
        _ => Err(Error::Plan(format!("Unsupported aggregate function: {name}"))),
    }
}

/**
 * @brief: Classify RETURN/WITH ReturnItems into group_by and aggs.
 *
 * If any expression contains an aggregate function, all non-aggregate expressions become GROUP BY keys;
 * the aggregate parts are extracted as AggregateExpr.
 *
 * @param items: List of projection items.
 * @param schema: Schema reference.
 * @return: (group_by list, aggs list, whether aggregation exists).
 */
fn classify_return_items(
    items: &[ReturnItem],
    schema: &dyn GraphSchema,
) -> Result<(Vec<(Expr, String)>, Vec<(AggregateExpr, String)>, bool)> {
    let has_agg = items.iter().any(|item| {
        match item {
            ReturnItem::Var(_) | ReturnItem::Property { .. } => false,
            ReturnItem::Alias { expr, .. } => contains_aggregate(expr),
            ReturnItem::Expression(expr) => contains_aggregate(expr),
        }
    });

    if !has_agg {
        return Ok((Vec::new(), Vec::new(), false));
    }

    let mut group_by = Vec::new();
    let mut aggs = Vec::new();

    for (i, item) in items.iter().enumerate() {
        match item {
            ReturnItem::Var(v) => {
                group_by.push((Expr::VarRef { name: v.clone() }, v.clone()));
            }
            ReturnItem::Property { var, key } => {
                let key_id = schema
                    .property_key_by_name(key)
                    .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                group_by.push((
                    Expr::PropRef {
                        source: Box::new(Expr::VarRef { name: var.clone() }),
                        key: key_id,
                    },
                    format!("{var}.{key}"),
                ));
            }
            ReturnItem::Alias { expr, alias } => {
                extract_agg_from_ast(expr, alias, schema, &mut group_by, &mut aggs)?;
            }
            ReturnItem::Expression(expr) => {
                let name = format!("expr_{i}");
                extract_agg_from_ast(expr, &name, schema, &mut group_by, &mut aggs)?;
            }
        }
    }

    Ok((group_by, aggs, true))
}

/**
 * @brief: Extract aggregate functions from AstExpr.
 *
 * If the expression is an aggregate call → add to aggs.
 * If the expression contains aggregate sub-expressions → extract recursively, preserving non-aggregate parts.
 *
 * @param expr: Source AST expression.
 * @param alias: Output alias.
 * @param schema: Schema reference.
 * @param group_by: group_by output list (mutable).
 * @param aggs: aggs output list (mutable).
 */
fn extract_agg_from_ast(
    expr: &AstExpr,
    alias: &str,
    schema: &dyn GraphSchema,
    group_by: &mut Vec<(Expr, String)>,
    aggs: &mut Vec<(AggregateExpr, String)>,
) -> Result<()> {
    match expr {
        AstExpr::FunctionCall { name, distinct, args } if is_aggregate_fn(name) => {
            let agg_expr = ast_agg_to_aggregate_expr(name, *distinct, args.first(), schema)?;
            aggs.push((agg_expr, alias.to_string()));
        }
        other if contains_aggregate(other) => {
            // Compound expression containing aggregates: add the whole expression to group_by, extract aggregate parts separately
            // Simplified handling: traverse sub-expressions, aggregate parts → aggs, non-aggregate parts → group_by
            extract_nested_agg(other, alias, schema, group_by, aggs)?;
        }
        other => {
            // Pure non-aggregate expression
            group_by.push((bind_expr(other, schema)?, alias.to_string()));
        }
    }
    Ok(())
}

/**
 * @brief: Recursively extract aggregate sub-expressions from compound expressions.
 */
fn extract_nested_agg(
    expr: &AstExpr,
    _alias: &str,
    schema: &dyn GraphSchema,
    group_by: &mut Vec<(Expr, String)>,
    aggs: &mut Vec<(AggregateExpr, String)>,
) -> Result<()> {
    match expr {
        AstExpr::FunctionCall { name, distinct, args } if is_aggregate_fn(name) => {
            let agg_name = format!("__agg_{}", aggs.len());
            let agg_expr = ast_agg_to_aggregate_expr(name, *distinct, args.first(), schema)?;
            aggs.push((agg_expr, agg_name));
        }
        AstExpr::Add(l, r) | AstExpr::Sub(l, r) | AstExpr::Mul(l, r)
        | AstExpr::Div(l, r) | AstExpr::Mod(l, r) => {
            extract_nested_agg(l, "", schema, group_by, aggs)?;
            extract_nested_agg(r, "", schema, group_by, aggs)?;
        }
        _ => {
            // Non-aggregate sub-expression → add to group_by (auto-assign column name)
            let gb_name = format!("__gb_{}", group_by.len());
            group_by.push((bind_expr(expr, schema)?, gb_name));
        }
    }
    Ok(())
}

fn plan_with(
    wc: &WithClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let mut root = input.map(|b| *b).unwrap_or(LogicalOp::SingleRow);

    // Detect aggregation
    let (group_by_exprs, aggs, has_agg) = classify_return_items(&wc.items, schema)?;
    if has_agg {
        // Aggregation layer
        root = LogicalOp::Aggregate {
            group_by: group_by_exprs.iter().map(|(e, _)| e.clone()).collect(),
            aggs: aggs.clone(),
            input: Box::new(root),
        };
        // Project rename
        let proj_items = build_agg_output_columns(&group_by_exprs, &aggs);
        root = LogicalOp::Project {
            items: proj_items,
            input: Box::new(root),
        };
    } else {
        root = plan_projection(&wc.items, root, schema)?;
    }

    if let Some(ref where_expr) = wc.where_clause {
        let predicate = bind_expr(where_expr, schema)?;
        root = LogicalOp::Filter {
            predicate,
            input: Box::new(root),
        };
    }
    if let Some(ref order_by) = wc.order_by {
        root = plan_order_by(&order_by.items, root, schema)?;
    }
    root = apply_skip_limit(&wc.skip, &wc.limit, root, schema)?;
    Ok(root)
}

fn plan_return(
    rc: &ReturnClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let mut root = input.map(|b| *b).unwrap_or(LogicalOp::SingleRow);

    // Detect aggregation
    let (group_by_exprs, aggs, has_agg) = classify_return_items(&rc.items, schema)?;
    if has_agg {
        // Aggregation layer
        root = LogicalOp::Aggregate {
            group_by: group_by_exprs.iter().map(|(e, _)| e.clone()).collect(),
            aggs: aggs.clone(),
            input: Box::new(root),
        };
        // Project rename
        let proj_items = build_agg_output_columns(&group_by_exprs, &aggs);
        root = LogicalOp::Project {
            items: proj_items,
            input: Box::new(root),
        };
    } else {
        root = plan_projection(&rc.items, root, schema)?;
    }

    if let Some(ref order_by) = rc.order_by {
        root = plan_order_by(&order_by.items, root, schema)?;
    }
    root = apply_skip_limit(&rc.skip, &rc.limit, root, schema)?;
    Ok(root)
}

fn plan_projection(
    items: &[ReturnItem],
    input: LogicalOp,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let mut proj_items = Vec::with_capacity(items.len());
    for item in items {
        match item {
            ReturnItem::Var(v) => {
                proj_items.push((Expr::VarRef { name: v.clone() }, v.clone()));
            }
            ReturnItem::Property { var, key } => {
                let key_id = schema
                    .property_key_by_name(key)
                    .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                let expr = Expr::PropRef {
                    source: Box::new(Expr::VarRef { name: var.clone() }),
                    key: key_id,
                };
                proj_items.push((expr, format!("{var}.{key}")));
            }
            ReturnItem::Alias { expr, alias } => {
                let bound = bind_expr(expr, schema)?;
                proj_items.push((bound, alias.clone()));
            }
            ReturnItem::Expression(expr) => {
                let bound = bind_expr(expr, schema)?;
                let alias = format!("expr_{}", proj_items.len());
                proj_items.push((bound, alias));
            }
        }
    }
    Ok(LogicalOp::Project {
        items: proj_items,
        input: Box::new(input),
    })
}

fn plan_order_by(
    items: &[SortItem],
    input: LogicalOp,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let mut keys = Vec::with_capacity(items.len());
    for si in items {
        let expr = bind_expr(&si.expr, schema)?;
        let order = match si.order {
            Some(super::cypher_ast::SortOrder::Desc) => CoreSortOrder::Desc,
            _ => CoreSortOrder::Asc,
        };
        keys.push((expr, order));
    }
    Ok(LogicalOp::Sort {
        keys,
        input: Box::new(input),
    })
}

fn apply_skip_limit(
    skip: &Option<AstExpr>,
    limit: &Option<AstExpr>,
    input: LogicalOp,
    _schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let offset = match skip {
        Some(s) => eval_const_int(s)?,
        None => 0,
    };
    let lim = match limit {
        Some(l) => eval_const_int(l)?,
        None => usize::MAX,
    };
    if offset == 0 && lim == usize::MAX {
        Ok(input)
    } else {
        Ok(LogicalOp::Limit {
            limit: lim,
            offset,
            input: Box::new(input),
        })
    }
}

// ============================================================
// UNWIND
// ============================================================

fn plan_unwind(
    uc: &UnwindClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.unwrap_or_else(|| Box::new(LogicalOp::SingleRow));
    let expr = bind_expr(&uc.expr, schema)?;
    Ok(LogicalOp::ExtensionOp {
        name: "Unwind".into(),
        payload: OpaquePayload::new(UnwindPayload {
            expr,
            var: uc.var.clone(),
        }),
        input: child,
    })
}

// ============================================================
// CREATE
// ============================================================

fn plan_create(
    cc: &CreateClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.unwrap_or_else(|| Box::new(LogicalOp::SingleRow));
    let mut root = *child;
    for element in &cc.pattern.elements {
        root = plan_create_element(element, root, schema)?;
    }
    Ok(root)
}

fn plan_create_element(
    element: &PatternElement,
    input: LogicalOp,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let vertex_var = element.node.var.clone().unwrap_or_else(|| "__anon".into());

    // Resolve label
    let label_id = match &element.node.label {
        Some(name) => Some(
            schema
                .label_by_name(name)
                .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?,
        ),
        None => None,
    };

    // Resolve inline properties (NodePattern.properties: Option<BTreeMap<String, AstExpr>>)
    let node_properties = bind_property_map(&element.node.properties, schema)?;

    let create_node_payload = CreateNodePayload {
        var: vertex_var.clone(),
        label_id,
        properties: node_properties,
    };
    let mut root = LogicalOp::ExtensionOp {
        name: "CreateNode".into(),
        payload: OpaquePayload::new(create_node_payload),
        input: Box::new(input),
    };

    let mut current_from = vertex_var;
    for edge in &element.edges {
        let to_var = edge.target.var.clone().unwrap_or_else(|| "__anon".into());

        // If the target node has a label or properties, create the target node first (before creating the edge)
        let target_label_id = match &edge.target.label {
            Some(name) => Some(
                schema
                    .label_by_name(name)
                    .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?,
            ),
            None => None,
        };
        let target_properties = bind_property_map(&edge.target.properties, schema)?;

        if target_label_id.is_some() || !target_properties.is_empty() {
            let target_node_payload = CreateNodePayload {
                var: to_var.clone(),
                label_id: target_label_id,
                properties: target_properties,
            };
            root = LogicalOp::ExtensionOp {
                name: "CreateNode".into(),
                payload: OpaquePayload::new(target_node_payload),
                input: Box::new(root),
            };
        }

        // Resolve edge type and properties
        let edge_type_ids = bind_edge_types(&edge.edge_types, schema)?;
        let edge_properties = bind_property_map(&edge.properties, schema)?;

        let create_edge_payload = CreateEdgePayload {
            from_var: current_from.clone(),
            to_var: to_var.clone(),
            edge_var: edge.edge_var.clone(),
            edge_type_ids,
            properties: edge_properties,
        };
        root = LogicalOp::ExtensionOp {
            name: "CreateEdge".into(),
            payload: OpaquePayload::new(create_edge_payload),
            input: Box::new(root),
        };
        current_from = to_var;
    }
    Ok(root)
}

// ============================================================
// MERGE
// ============================================================

fn plan_merge(
    mc: &MergeClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.unwrap_or_else(|| Box::new(LogicalOp::SingleRow));

    let mut targets = Vec::new();

    // Traverse each element of the pattern to build the MergeTarget list
    for element in &mc.pattern.elements {
        // Node target
        let node_var = element.node.var.clone().unwrap_or_else(|| "__anon".into());
        let label_id = match &element.node.label {
            Some(name) => Some(
                schema
                    .label_by_name(name)
                    .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?,
            ),
            None => None,
        };
        let node_properties = bind_property_map(&element.node.properties, schema)?;
        targets.push(MergeTarget::Node {
            var: node_var.clone(),
            label_id,
            properties: node_properties,
        });

        // Edge targets
        let mut current_from = node_var;
        for edge in &element.edges {
            let to_var = edge.target.var.clone().unwrap_or_else(|| "__anon".into());

            // Also add target node to targets (if it has a label or properties)
            let target_label_id = match &edge.target.label {
                Some(name) => Some(
                    schema
                        .label_by_name(name)
                        .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?,
                ),
                None => None,
            };
            let target_properties = bind_property_map(&edge.target.properties, schema)?;
            if target_label_id.is_some() || !target_properties.is_empty() {
                targets.push(MergeTarget::Node {
                    var: to_var.clone(),
                    label_id: target_label_id,
                    properties: target_properties,
                });
            }

            // Edge target
            let edge_var = edge.edge_var.clone().unwrap_or_else(|| "__anon_edge".into());
            let edge_type_ids = bind_edge_types(&edge.edge_types, schema)?
                .unwrap_or_default();
            let edge_properties = bind_property_map(&edge.properties, schema)?;
            targets.push(MergeTarget::Edge {
                from_var: current_from.clone(),
                to_var: to_var.clone(),
                edge_var,
                edge_type_ids,
                properties: edge_properties,
            });

            current_from = to_var;
        }
    }

    // Build ON CREATE / ON MATCH action lists
    let mut on_create = Vec::new();
    let mut on_match = Vec::new();
    for action in &mc.actions {
        match action {
            MergeAction::OnCreate(items) | MergeAction::OnMatch(items) => {
                for item in items {
                    if let SetItem::Property { var, key, value } = item {
                        let key_id = schema
                            .property_key_by_name(key)
                            .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                        let val_expr = bind_expr(value, schema)?;
                        match action {
                            MergeAction::OnCreate(_) => {
                                on_create.push((var.clone(), key_id, val_expr));
                            }
                            MergeAction::OnMatch(_) => {
                                on_match.push((var.clone(), key_id, val_expr));
                            }
                        }
                    }
                }
            }
        }
    }

    let merge_payload = MergePayload {
        targets,
        on_create,
        on_match,
    };
    Ok(LogicalOp::ExtensionOp {
        name: "Merge".into(),
        payload: OpaquePayload::new(merge_payload),
        input: child,
    })
}

// ============================================================
// DELETE
// ============================================================

fn plan_delete(
    dc: &DeleteClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.ok_or_else(|| Error::Plan("DELETE missing preceding data source".into()))?;
    let (targets, detach) = match dc {
        DeleteClause::Delete(vars) => (vars, false),
        DeleteClause::DetachDelete(vars) => (vars, true),
    };
    let target_exprs: Vec<Expr> = targets
        .iter()
        .map(|e| bind_expr(e, schema))
        .collect::<Result<Vec<_>>>()?;
    Ok(LogicalOp::ExtensionOp {
        name: "Delete".into(),
        payload: OpaquePayload::new(DeletePayload {
            detach,
            target_exprs,
        }),
        input: child,
    })
}

// ============================================================
// SET
// ============================================================

fn plan_set(
    sc: &SetClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.ok_or_else(|| Error::Plan("SET missing preceding data source".into()))?;
    let mut root = *child;
    for item in &sc.items {
        match item {
            SetItem::Property { var, key, value } => {
                let key_id = schema
                    .property_key_by_name(key)
                    .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                let val_expr = bind_expr(value, schema)?;
                root = LogicalOp::ExtensionOp {
                    name: "SetProperty".into(),
                    payload: OpaquePayload::new(SetPropertyPayload {
                        var: var.clone(),
                        key_id,
                        value: val_expr,
                    }),
                    input: Box::new(root),
                };
            }
            SetItem::Labels { var, labels } => {
                let label_ids = bind_label_names(labels, schema)?;
                root = LogicalOp::ExtensionOp {
                    name: "SetLabels".into(),
                    payload: OpaquePayload::new(SetLabelsPayload {
                        var: var.clone(),
                        label_ids,
                    }),
                    input: Box::new(root),
                };
            }
        }
    }
    Ok(root)
}

// ============================================================
// REMOVE
// ============================================================

fn plan_remove(
    rc: &RemoveClause,
    input: Option<Box<LogicalOp>>,
    schema: &dyn GraphSchema,
) -> Result<LogicalOp> {
    let child = input.ok_or_else(|| Error::Plan("REMOVE missing preceding data source".into()))?;
    let mut root = *child;
    for item in &rc.items {
        match item {
            RemoveItem::Property { var, key } => {
                let key_id = schema
                    .property_key_by_name(key)
                    .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                root = LogicalOp::ExtensionOp {
                    name: "RemoveProperty".into(),
                    payload: OpaquePayload::new(RemovePropertyPayload {
                        var: var.clone(),
                        key_id,
                    }),
                    input: Box::new(root),
                };
            }
            RemoveItem::Labels { var, labels } => {
                let label_ids = bind_label_names(labels, schema)?;
                root = LogicalOp::ExtensionOp {
                    name: "RemoveLabels".into(),
                    payload: OpaquePayload::new(RemoveLabelsPayload {
                        var: var.clone(),
                        label_ids,
                    }),
                    input: Box::new(root),
                };
            }
        }
    }
    Ok(root)
}

// ============================================================
// CALL
// ============================================================

fn plan_call(algo: String, args: BTreeMap<String, PropertyValue>) -> LogicalPlan {
    let spec = ProjectionSpec {
        vertex_labels: None,
        edge_types: None,
        direction: ProjectionDirection::Outgoing,
        vertex_properties: None,
    };
    let root = LogicalOp::CallAlgorithm {
        name: algo,
        args: AlgorithmArgs { params: args },
        input: Box::new(LogicalOp::GraphProjection { spec }),
    };
    LogicalPlan { root }
}

// ============================================================
// Binder: AstExpr -> Expr
// ============================================================

fn bind_expr(ast: &AstExpr, schema: &dyn GraphSchema) -> Result<Expr> {
    match ast {
        AstExpr::Int(v) => Ok(Expr::Int(*v)),
        AstExpr::Float(v) => Ok(Expr::Float(*v)),
        AstExpr::Str(v) => Ok(Expr::String(v.clone())),
        AstExpr::Bool(v) => Ok(Expr::Bool(*v)),
        AstExpr::Null => Ok(Expr::Null),
        AstExpr::Parameter(_) => Err(Error::Plan("Parameters not yet supported".into())),

        AstExpr::Var(name) => Ok(Expr::VarRef { name: name.clone() }),
        AstExpr::Prop { var, key } => {
            let key_id = schema
                .property_key_by_name(key)
                .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
            Ok(Expr::PropRef {
                source: Box::new(Expr::VarRef { name: var.clone() }),
                key: key_id,
            })
        }

        AstExpr::Neg(inner) => Ok(Expr::Neg(Box::new(bind_expr(inner, schema)?))),
        AstExpr::Add(l, r) => Ok(Expr::Add(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Sub(l, r) => Ok(Expr::Sub(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Mul(l, r) => Ok(Expr::Mul(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Div(l, r) => Ok(Expr::Div(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Mod(l, r) => Ok(Expr::Mod(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Pow(l, r) => Ok(Expr::FunctionCall { name: "pow".into(), args: vec![bind_expr(l, schema)?, bind_expr(r, schema)?] }),

        AstExpr::Eq(l, r) => Ok(Expr::Eq(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Neq(l, r) => Ok(Expr::Neq(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Gt(l, r) => Ok(Expr::Gt(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Gte(l, r) => Ok(Expr::Gte(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Lt(l, r) => Ok(Expr::Lt(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Lte(l, r) => Ok(Expr::Lte(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),

        AstExpr::IsNull(inner) => Ok(Expr::IsNull(Box::new(bind_expr(inner, schema)?))),
        AstExpr::IsNotNull(inner) => Ok(Expr::IsNotNull(Box::new(bind_expr(inner, schema)?))),

        AstExpr::And(l, r) => Ok(Expr::And(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Or(l, r) => Ok(Expr::Or(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Xor(l, r) => {
            let lb = bind_expr(l, schema)?;
            let rb = bind_expr(r, schema)?;
            Ok(Expr::Or(
                Box::new(Expr::And(Box::new(lb.clone()), Box::new(Expr::Not(Box::new(rb.clone()))))),
                Box::new(Expr::And(Box::new(Expr::Not(Box::new(lb))), Box::new(rb))),
            ))
        }
        AstExpr::Not(inner) => Ok(Expr::Not(Box::new(bind_expr(inner, schema)?))),

        AstExpr::In(l, r) => Ok(Expr::FunctionCall { name: "in".into(), args: vec![bind_expr(l, schema)?, bind_expr(r, schema)?] }),

        AstExpr::StartsWith(l, r) => Ok(Expr::StartsWith(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::EndsWith(l, r) => Ok(Expr::EndsWith(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::Contains(l, r) => Ok(Expr::Contains(Box::new(bind_expr(l, schema)?), Box::new(bind_expr(r, schema)?))),
        AstExpr::RegexMatch(l, r) => Ok(Expr::FunctionCall { name: "regex_match".into(), args: vec![bind_expr(l, schema)?, bind_expr(r, schema)?] }),
        AstExpr::Concat(l, r) => Ok(Expr::Concat(vec![bind_expr(l, schema)?, bind_expr(r, schema)?])),

        AstExpr::List(items) => Ok(Expr::List(items.iter().map(|e| bind_expr(e, schema)).collect::<Result<Vec<_>>>()?)),
        AstExpr::ListIndex { list, index } => Ok(Expr::ListIndex { list: Box::new(bind_expr(list, schema)?), index: Box::new(bind_expr(index, schema)?) }),
        AstExpr::ListSlice { .. } => Err(Error::Plan("ListSlice not yet supported".into())),

        AstExpr::Map(_) | AstExpr::MapAccess { .. } => Err(Error::Plan("Map expressions not yet supported".into())),

        AstExpr::FunctionCall { name, args, .. } => Ok(Expr::FunctionCall { name: name.clone(), args: args.iter().map(|e| bind_expr(e, schema)).collect::<Result<Vec<_>>>()? }),

        AstExpr::Case { cases, else_branch, .. } => {
            let mut case_pairs = Vec::new();
            for (cond, then_val) in cases {
                case_pairs.push((bind_expr(cond, schema)?, bind_expr(then_val, schema)?));
            }
            let else_val = match else_branch {
                Some(e) => Some(Box::new(bind_expr(e, schema)?)),
                None => None,
            };
            Ok(Expr::Case { cases: case_pairs, else_branch: else_val })
        }

        AstExpr::ExistsPattern(_) => Err(Error::Plan("EXISTS pattern not yet supported".into())),
    }
}

// ============================================================
// Helpers
// ============================================================

fn bind_edge_types(types: &Option<Vec<String>>, schema: &dyn GraphSchema) -> Result<Option<Vec<EdgeTypeId>>> {
    match types {
        None => Ok(None),
        Some(names) => {
            let mut ids = Vec::with_capacity(names.len());
            for name in names {
                let id = schema
                    .edge_type_by_name(name)
                    .ok_or_else(|| Error::Plan(format!("Unknown edge type: {name}")))?;
                ids.push(id);
            }
            Ok(Some(ids))
        }
    }
}

/**
 * @brief: Resolve a list of label names to LabelId list.
 * @param names: Label name list.
 * @param schema: Used for resolution.
 * @return: LabelId list.
 */
fn bind_label_names(names: &[String], schema: &dyn GraphSchema) -> Result<Vec<LabelId>> {
    let mut ids = Vec::with_capacity(names.len());
    for name in names {
        let id = schema
            .label_by_name(name)
            .ok_or_else(|| Error::Plan(format!("Unknown label: {name}")))?;
        ids.push(id);
    }
    Ok(ids)
}

/**
 * @brief: Bind an optional property map (BTreeMap<String, AstExpr>) to a list of (PropertyKeyId, Expr).
 * @param props: Optional property map.
 * @param schema: Used to resolve property key names.
 * @return: List of bound property key-value pairs.
 */
fn bind_property_map(
    props: &Option<std::collections::BTreeMap<String, AstExpr>>,
    schema: &dyn GraphSchema,
) -> Result<Vec<(PropertyKeyId, Expr)>> {
    match props {
        None => Ok(Vec::new()),
        Some(map) => {
            let mut result = Vec::with_capacity(map.len());
            for (key, val) in map {
                let key_id = schema
                    .property_key_by_name(key)
                    .ok_or_else(|| Error::Plan(format!("Unknown property key: {key}")))?;
                let expr = bind_expr(val, schema)?;
                result.push((key_id, expr));
            }
            Ok(result)
        }
    }
}

fn convert_direction(d: EdgeDirection) -> ExpandDirection {
    match d {
        EdgeDirection::Outgoing => ExpandDirection::Outgoing,
        EdgeDirection::Incoming => ExpandDirection::Incoming,
        EdgeDirection::Both => ExpandDirection::Both,
    }
}

/**
 * @brief: Build Project output columns from aggregate classification results.
 *
 * group_by and aggs each have internal column names (__gb_N, __agg_N),
 * this function maps them back to user-specified aliases.
 */
fn build_agg_output_columns(
    group_by: &[(Expr, String)],
    aggs: &[(AggregateExpr, String)],
) -> Vec<(Expr, String)> {
    let mut items = Vec::with_capacity(group_by.len() + aggs.len());
    // optimizer renames group_by aliases to __gb_N, Project needs to match
    for (i, (_, alias)) in group_by.iter().enumerate() {
        let internal = format!("__gb_{i}");
        items.push((Expr::VarRef { name: internal }, alias.clone()));
    }
    for (_, alias) in aggs {
        // aggs aliases are output directly by Aggregate, no renaming needed
        items.push((Expr::VarRef { name: alias.clone() }, alias.clone()));
    }
    items
}

fn eval_const_int(expr: &AstExpr) -> Result<usize> {
    match expr {
        AstExpr::Int(v) if *v >= 0 => Ok(*v as usize),
        AstExpr::Int(v) => Err(Error::Plan(format!("Negative number cannot be used as limit/offset: {v}"))),
        AstExpr::Parameter(_) => Err(Error::Plan("limit/skip parameters not yet supported".into())),
        other => Err(Error::Plan(format!("limit/skip requires integer constant, got: {other:?}"))),
    }
}
