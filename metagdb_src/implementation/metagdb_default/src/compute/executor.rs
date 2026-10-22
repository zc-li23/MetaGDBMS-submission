/**
 * @brief: MetagdbExecutor: implementation based on the volcano model executor.
 *
 * Downcasts DefaultPhysicalOp from core's PhysicalPlan, recursively materializes intermediate results.
 * Includes expression evaluator, row binding model, and DML operator executors.
 */

use std::sync::Arc;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue, ResultValue};
use metagdb_core::l2_storage::{EdgeFilter, EdgeRecord, VertexFilter, VertexRecord};
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, PhysicalPlan, ProjectionSpec, SortOrder,
};
use metagdb_core::l4_compute::engine::{ExecContext, Executor, ResultStream};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};
use metagdb_core::l4_compute::olap::GraphProjection;
use metagdb_core::{Error, Result};

use super::physical_ops::{DefaultPhysicalOp, MergePayload};
use super::projection::CsrProjection;

// ============================================================
// Intermediate value container
// ============================================================

enum ExecValue {
    Rows(Vec<String>, Vec<Row>),
    Projection(Arc<dyn GraphProjection>),
}

type Row = Vec<ResultValue>;

// ============================================================
// Executor entry point
// ============================================================

#[derive(Default)]
pub struct MetagdbExecutor;

impl Executor for MetagdbExecutor {
    /**
     * @brief: Execute a physical plan.
     */
    fn execute(&self, pp: &PhysicalPlan, ctx: &ExecContext) -> Result<ResultStream> {
        let root = pp
            .downcast_ref::<DefaultPhysicalOp>()
            .ok_or_else(|| {
                Error::Execute("PhysicalPlan does not contain DefaultPhysicalOp".into())
            })?;
        let value = eval(root, ctx)?;
        match value {
            ExecValue::Rows(schema, rows) => Ok(ResultStream { schema, rows }),
            ExecValue::Projection(_) => Err(Error::Execute(
                "PhysicalPlan root node is GraphProjection, but no top-level algorithm consumes it".into(),
            )),
        }
    }
}

// ============================================================
// Operator evaluation dispatch
// ============================================================

fn eval(op: &DefaultPhysicalOp, ctx: &ExecContext) -> Result<ExecValue> {
    match op {
        DefaultPhysicalOp::SingleRow => Ok(ExecValue::Rows(
            Vec::new(),
            vec![Vec::new()],
        )),
        DefaultPhysicalOp::VertexScan { var, label } => exec_vertex_scan(var, *label, ctx),
        DefaultPhysicalOp::Expand {
            from,
            to,
            edge,
            direction,
            edge_types,
            input,
        } => {
            let child = eval(input, ctx)?;
            exec_expand(
                from,
                to,
                edge.as_deref(),
                *direction,
                edge_types.as_deref(),
                child,
                ctx,
            )
        }
        DefaultPhysicalOp::VarLengthExpand {
            from,
            to,
            edge,
            direction,
            edge_types,
            min_hops,
            max_hops,
            input,
        } => {
            let child = eval(input, ctx)?;
            exec_var_length_expand(
                from,
                to,
                edge.as_deref(),
                *direction,
                edge_types.as_deref(),
                *min_hops,
                *max_hops,
                child,
                ctx,
            )
        }
        DefaultPhysicalOp::Filter { predicate, input } => {
            let child = eval(input, ctx)?;
            exec_filter(predicate, child, ctx)
        }
        DefaultPhysicalOp::Project { items, input } => {
            let child = eval(input, ctx)?;
            exec_project(items, child, ctx)
        }
        DefaultPhysicalOp::Sort { keys, input } => {
            let child = eval(input, ctx)?;
            exec_sort(keys, child, ctx)
        }
        DefaultPhysicalOp::Aggregate { group_by, aggs, input } => {
            let child = eval(input, ctx)?;
            exec_aggregate(group_by, aggs, child, ctx)
        }
        DefaultPhysicalOp::Limit {
            limit,
            offset,
            input,
        } => {
            let child = eval(input, ctx)?;
            exec_limit(*limit, *offset, child)
        }
        DefaultPhysicalOp::Union { all, inputs } => exec_union(*all, inputs, ctx),
        DefaultPhysicalOp::CrossJoin { left, right } => exec_cross_join(left, right, ctx),
        DefaultPhysicalOp::BuildProjection { spec } => exec_build_projection(spec, ctx),
        DefaultPhysicalOp::RunAlgorithm { name, args, input } => {
            let child = eval(input, ctx)?;
            exec_run_algorithm(name, args, child, ctx)
        }
        DefaultPhysicalOp::CreateNode { var, label_id, properties, input } => {
            let child = eval(input, ctx)?;
            exec_create_node(var, *label_id, properties, child, ctx)
        }
        DefaultPhysicalOp::CreateEdge { from_var, to_var, edge_var, edge_type_ids, properties, input } => {
            let child = eval(input, ctx)?;
            exec_create_edge(from_var, to_var, edge_var.as_deref(), edge_type_ids.as_deref(), properties, child, ctx)
        }
        DefaultPhysicalOp::SetProperty { var, key_id, value, input } => {
            let child = eval(input, ctx)?;
            exec_set_property(var, *key_id, value, child, ctx)
        }
        DefaultPhysicalOp::SetLabels { var, label_ids, input } => {
            let child = eval(input, ctx)?;
            exec_set_labels(var, label_ids, child, ctx)
        }
        DefaultPhysicalOp::RemoveProperty { var, key_id, input } => {
            let child = eval(input, ctx)?;
            exec_remove_property(var, *key_id, child, ctx)
        }
        DefaultPhysicalOp::RemoveLabels { var, label_ids, input } => {
            let child = eval(input, ctx)?;
            exec_remove_labels(var, label_ids, child, ctx)
        }
        DefaultPhysicalOp::Delete { detach, targets, input } => {
            let child = eval(input, ctx)?;
            exec_delete(*detach, targets, child, ctx)
        }
        DefaultPhysicalOp::Unwind { expr, var, input } => {
            let child = eval(input, ctx)?;
            exec_unwind(expr, var, child, ctx)
        }
        DefaultPhysicalOp::Merge { payload, input } => {
            let child = eval(input, ctx)?;
            exec_merge(payload, child, ctx)
        }
        DefaultPhysicalOp::ExtensionOp { name, payload: _, input } => {
            let _child = eval(input, ctx)?;
            Err(Error::Execute(format!(
                "ExtensionOp `{name}` execution logic not yet implemented"
            )))
        }
    }
}

// ============================================================
// SingleRow
// ============================================================

// (inline in eval)

// ============================================================
// VertexScan
// ============================================================

fn exec_vertex_scan(var: &str, label: Option<LabelId>, ctx: &ExecContext) -> Result<ExecValue> {
    let filter = VertexFilter {
        labels: label.map(|l| vec![l]),
    };
    let iter = ctx.storage.scan_vertices(filter, &ctx.txn)?;
    let mut rows = Vec::new();
    for v in iter {
        rows.push(vec![ResultValue::VertexRef(v.id)]);
    }
    Ok(ExecValue::Rows(vec![var.to_string()], rows))
}

// ============================================================
// Expand
// ============================================================

fn exec_expand(
    from: &str,
    to: &str,
    edge: Option<&str>,
    direction: ExpandDirection,
    edge_types: Option<&[EdgeTypeId]>,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (in_schema, in_rows) = match child {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("Expand input cannot be GraphProjection".into()))
        }
    };

    let from_idx = col_index(&in_schema, from)?;

    let mut out_schema = in_schema.clone();
    if let Some(e) = edge {
        out_schema.push(e.to_string());
    }
    out_schema.push(to.to_string());

    let efilter = EdgeFilter {
        types: edge_types.map(|t| t.to_vec()),
    };

    let mut out_rows = Vec::new();
    for in_row in &in_rows {
        let src_vid = match &in_row[from_idx] {
            ResultValue::VertexRef(v) => *v,
            other => {
                return Err(Error::Execute(format!(
                    "Expand: from column expected VertexRef, got {other:?}"
                )))
            }
        };

        let edges_iter: Vec<_> = match direction {
            ExpandDirection::Outgoing => ctx
                .storage
                .out_edges(src_vid, efilter.clone(), &ctx.txn)?
                .collect(),
            ExpandDirection::Incoming => ctx
                .storage
                .in_edges(src_vid, efilter.clone(), &ctx.txn)?
                .collect(),
            ExpandDirection::Both => {
                let mut out: Vec<_> = ctx
                    .storage
                    .out_edges(src_vid, efilter.clone(), &ctx.txn)?
                    .collect();
                let inc: Vec<_> = ctx
                    .storage
                    .in_edges(src_vid, efilter.clone(), &ctx.txn)?
                    .collect();
                out.extend(inc);
                out
            }
        };

        for edge_rec in &edges_iter {
            let target_vid = match direction {
                ExpandDirection::Outgoing => edge_rec.dst,
                ExpandDirection::Incoming => edge_rec.src,
                ExpandDirection::Both => {
                    if edge_rec.src == src_vid {
                        edge_rec.dst
                    } else {
                        edge_rec.src
                    }
                }
            };
            let mut out_row = in_row.clone();
            if edge.is_some() {
                out_row.push(ResultValue::EdgeRef(edge_rec.id));
            }
            out_row.push(ResultValue::VertexRef(target_vid));
            out_rows.push(out_row);
        }
    }

    Ok(ExecValue::Rows(out_schema, out_rows))
}

// ============================================================
// VarLengthExpand (BFS multi-hop expansion)
// ============================================================

/**
 * @brief: Execute variable-length path expansion operator (BFS multi-hop traversal).
 *
 * For each input row, starting from the source vertex specified by the from column,
 * BFS-expand for min_hops to max_hops hops.
 * Matches edge_types filtering and direction semantics.
 *
 * @param from: Source vertex variable name.
 * @param to: Target vertex variable name.
 * @param edge: Optional edge variable name.
 * @param direction: Expansion direction.
 * @param edge_types: Optional edge type filter.
 * @param min_hops: Minimum hop count.
 * @param max_hops: Maximum hop count (defaults to 10 when None).
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Expanded row set.
 */
fn exec_var_length_expand(
    from: &str,
    to: &str,
    edge: Option<&str>,
    direction: ExpandDirection,
    edge_types: Option<&[EdgeTypeId]>,
    min_hops: usize,
    max_hops: Option<usize>,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    use std::collections::{HashSet, VecDeque};

    let (in_schema, in_rows) = unpack_rows(child)?;
    let from_idx = col_index(&in_schema, from)?;
    let max_effective = max_hops.unwrap_or(10);

    let mut out_schema = in_schema.clone();
    if edge.is_some() {
        out_schema.push(edge.unwrap().to_string());
    }
    out_schema.push(to.to_string());

    let efilter = EdgeFilter {
        types: edge_types.map(|t| t.to_vec()),
    };

    let mut out_rows = Vec::new();
    for in_row in &in_rows {
        let src_vid = extract_vid(&in_row[from_idx])?;

        // BFS: (vertex_id, hops)
        let mut visited: HashSet<VertexId> = HashSet::new();
        let mut queue: VecDeque<(VertexId, usize)> = VecDeque::new();
        visited.insert(src_vid);
        queue.push_back((src_vid, 0));

        while let Some((current_vid, hops)) = queue.pop_front() {
            if hops >= max_effective {
                continue;
            }

            let edges: Vec<_> = match direction {
                ExpandDirection::Outgoing => ctx
                    .storage
                    .out_edges(current_vid, efilter.clone(), &ctx.txn)?
                    .collect(),
                ExpandDirection::Incoming => ctx
                    .storage
                    .in_edges(current_vid, efilter.clone(), &ctx.txn)?
                    .collect(),
                ExpandDirection::Both => {
                    let mut out: Vec<_> = ctx
                        .storage
                        .out_edges(current_vid, efilter.clone(), &ctx.txn)?
                        .collect();
                    let inc: Vec<_> = ctx
                        .storage
                        .in_edges(current_vid, efilter.clone(), &ctx.txn)?
                        .collect();
                    out.extend(inc);
                    out
                }
            };

            for edge_rec in &edges {
                let neighbor = match direction {
                    ExpandDirection::Outgoing => edge_rec.dst,
                    ExpandDirection::Incoming => edge_rec.src,
                    ExpandDirection::Both => {
                        if edge_rec.src == current_vid {
                            edge_rec.dst
                        } else {
                            edge_rec.src
                        }
                    }
                };

                let new_hops = hops + 1;

                // Produce results when minimum hop count is reached
                if new_hops >= min_hops {
                    let mut out_row = in_row.clone();
                    if edge.is_some() {
                        out_row.push(ResultValue::EdgeRef(edge_rec.id));
                    }
                    out_row.push(ResultValue::VertexRef(neighbor));
                    out_rows.push(out_row);
                }

                // Continue expanding (not yet at limit and not visited)
                if new_hops < max_effective && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, new_hops));
                }
            }
        }
    }

    Ok(ExecValue::Rows(out_schema, out_rows))
}

// ============================================================
// Filter
// ============================================================

fn exec_filter(predicate: &Expr, child: ExecValue, ctx: &ExecContext) -> Result<ExecValue> {
    let (schema, in_rows) = match child {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("Filter input cannot be GraphProjection".into()))
        }
    };

    let mut out_rows = Vec::with_capacity(in_rows.len());
    for row in &in_rows {
        let val = eval_expr(predicate, &schema, row, ctx)?;
        if is_truthy(&val) {
            out_rows.push(row.clone());
        }
    }

    Ok(ExecValue::Rows(schema, out_rows))
}

// ============================================================
// Project
// ============================================================

fn exec_project(
    items: &[(Expr, String)],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (in_schema, in_rows) = match child {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("Project input cannot be GraphProjection".into()))
        }
    };

    let mut out_schema = Vec::with_capacity(items.len());
    let mut out_rows: Vec<Row> = vec![Vec::with_capacity(items.len()); in_rows.len()];

    for (expr, alias) in items {
        out_schema.push(alias.clone());
        for (i, in_row) in in_rows.iter().enumerate() {
            let val = eval_expr(expr, &in_schema, in_row, ctx)?;
            out_rows[i].push(val);
        }
    }

    Ok(ExecValue::Rows(out_schema, out_rows))
}

// ============================================================
// Sort
// ============================================================

fn exec_sort(
    keys: &[(Expr, SortOrder)],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, mut rows) = match child {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("Sort input cannot be GraphProjection".into()))
        }
    };

    rows.sort_by(|a, b| {
        for (expr, order) in keys {
            let va = eval_expr(expr, &schema, a, ctx).unwrap_or(ResultValue::Null);
            let vb = eval_expr(expr, &schema, b, ctx).unwrap_or(ResultValue::Null);
            let cmp = compare_values(&va, &vb);
            match cmp {
                Some(std::cmp::Ordering::Equal) => continue,
                Some(o) => {
                    return match order {
                        SortOrder::Desc => o.reverse(),
                        SortOrder::Asc => o,
                    }
                }
                None => continue,
            }
        }
        std::cmp::Ordering::Equal
    });

    Ok(ExecValue::Rows(schema, rows))
}

// ============================================================
// Limit (SKIP + LIMIT)
// ============================================================

fn exec_limit(limit: usize, offset: usize, child: ExecValue) -> Result<ExecValue> {
    let (schema, mut rows) = match child {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("Limit input cannot be GraphProjection".into()))
        }
    };

    let start = offset.min(rows.len());
    let end = (offset + limit).min(rows.len());
    let limited = rows.drain(start..end).collect();

    Ok(ExecValue::Rows(schema, limited))
}

// ============================================================
// Aggregate
// ============================================================

/**
 * @brief: Execute aggregate operator (GROUP BY + aggregate functions).
 *
 * If no group_by, treat all rows as a single group for full-table aggregation.
 * If group_by present, group by keys then compute aggregates per group.
 *
 * @param group_by: Group key expression + alias list.
 * @param aggs: Aggregate expression + alias list.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Aggregated row set.
 */
fn exec_aggregate(
    group_by: &[(Expr, String)],
    aggs: &[(AggregateExpr, String)],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (in_schema, rows) = unpack_rows(child)?;

    if rows.is_empty() && group_by.is_empty() {
        // Empty table full-table aggregate: count(*) = 0
        let mut out_schema: Vec<String> = group_by.iter().map(|(_, a)| a.clone()).collect();
        for (_, alias) in aggs {
            out_schema.push(alias.clone());
        }
        let mut out_row = Vec::new();
        for _ in group_by {
            out_row.push(ResultValue::Null);
        }
        for agg in aggs.iter().map(|(a, _)| a) {
            out_row.push(eval_aggregate_empty(agg));
        }
        return Ok(ExecValue::Rows(out_schema, vec![out_row]));
    }

    if group_by.is_empty() {
        // Full-table aggregate
        let out_schema: Vec<String> = aggs.iter().map(|(_, a)| a.clone()).collect();
        let mut out_row = Vec::with_capacity(aggs.len());
        for (agg_expr, _) in aggs {
            out_row.push(eval_aggregate(agg_expr, &rows, &in_schema, ctx)?);
        }
        Ok(ExecValue::Rows(out_schema, vec![out_row]))
    } else {
        // Has GROUP BY
        use std::collections::HashMap;
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();

        for (row_idx, row) in rows.iter().enumerate() {
            let mut key_parts = Vec::with_capacity(group_by.len());
            for (expr, _) in group_by {
                let val = eval_expr(expr, &in_schema, row, ctx)?;
                key_parts.push(format!("{val:?}"));
            }
            let key = key_parts.join("\x00");
            groups.entry(key).or_default().push(row_idx);
        }

        let mut out_schema: Vec<String> = group_by.iter().map(|(_, a)| a.clone()).collect();
        for (_, alias) in aggs {
            out_schema.push(alias.clone());
        }

        let mut out_rows = Vec::with_capacity(groups.len());
        for (_key, row_indices) in &groups {
            let group_rows: Vec<Row> = row_indices.iter().map(|&i| rows[i].clone()).collect();
            // Reconstruct group key values (take values from the first row's group_by columns)
            let first_row = &rows[row_indices[0]];
            let mut out_row = Vec::with_capacity(group_by.len() + aggs.len());
            for (expr, _) in group_by {
                out_row.push(eval_expr(expr, &in_schema, first_row, ctx)?);
            }
            for (agg_expr, _) in aggs {
                out_row.push(eval_aggregate(agg_expr, &group_rows, &in_schema, ctx)?);
            }
            out_rows.push(out_row);
        }

        Ok(ExecValue::Rows(out_schema, out_rows))
    }
}

/**
 * @brief: Compute the value of a single aggregate expression over a set of rows.
 */
fn eval_aggregate(
    agg: &AggregateExpr,
    rows: &[Row],
    schema: &[String],
    ctx: &ExecContext,
) -> Result<ResultValue> {
    use std::collections::HashSet;

    match agg {
        AggregateExpr::CountAll => Ok(ResultValue::Int(rows.len() as i64)),
        AggregateExpr::Count(expr) => {
            let cnt = rows.iter().filter(|r| {
                !matches!(eval_expr(expr, schema, r, ctx), Ok(ResultValue::Null))
            }).count();
            Ok(ResultValue::Int(cnt as i64))
        }
        AggregateExpr::CountDistinct(expr) => {
            let mut seen = HashSet::new();
            for row in rows {
                if let Ok(val) = eval_expr(expr, schema, row, ctx) {
                    if !matches!(val, ResultValue::Null) {
                        seen.insert(format!("{val:?}"));
                    }
                }
            }
            Ok(ResultValue::Int(seen.len() as i64))
        }
        AggregateExpr::Sum(expr) => {
            let mut total: f64 = 0.0;
            let mut has_value = false;
            for row in rows {
                match eval_expr(expr, schema, row, ctx)? {
                    ResultValue::Int(i) => { total += i as f64; has_value = true; }
                    ResultValue::Float(f) => { total += f; has_value = true; }
                    _ => {}
                }
            }
            if has_value { Ok(ResultValue::Float(total)) } else { Ok(ResultValue::Null) }
        }
        AggregateExpr::SumDistinct(expr) => {
            let mut seen: HashSet<String> = HashSet::new();
            let mut total: f64 = 0.0;
            for row in rows {
                match eval_expr(expr, schema, row, ctx)? {
                    ResultValue::Int(i) => {
                        let key = format!("{i}");
                        if seen.insert(key) { total += i as f64; }
                    }
                    ResultValue::Float(f) => {
                        let key = format!("{f}");
                        if seen.insert(key) { total += f; }
                    }
                    _ => {}
                }
            }
            if seen.is_empty() { Ok(ResultValue::Null) } else { Ok(ResultValue::Float(total)) }
        }
        AggregateExpr::Avg(expr) => {
            let mut total: f64 = 0.0;
            let mut cnt: i64 = 0;
            for row in rows {
                match eval_expr(expr, schema, row, ctx)? {
                    ResultValue::Int(i) => { total += i as f64; cnt += 1; }
                    ResultValue::Float(f) => { total += f; cnt += 1; }
                    _ => {}
                }
            }
            if cnt > 0 { Ok(ResultValue::Float(total / cnt as f64)) } else { Ok(ResultValue::Null) }
        }
        AggregateExpr::Min(expr) => {
            let mut best: Option<ResultValue> = None;
            for row in rows {
                let val = eval_expr(expr, schema, row, ctx)?;
                if matches!(val, ResultValue::Null) { continue; }
                match &best {
                    None => best = Some(val),
                    Some(b) if compare_values(&val, b) == Some(std::cmp::Ordering::Less) => best = Some(val),
                    _ => {}
                }
            }
            Ok(best.unwrap_or(ResultValue::Null))
        }
        AggregateExpr::Max(expr) => {
            let mut best: Option<ResultValue> = None;
            for row in rows {
                let val = eval_expr(expr, schema, row, ctx)?;
                if matches!(val, ResultValue::Null) { continue; }
                match &best {
                    None => best = Some(val),
                    Some(b) if compare_values(&val, b) == Some(std::cmp::Ordering::Greater) => best = Some(val),
                    _ => {}
                }
            }
            Ok(best.unwrap_or(ResultValue::Null))
        }
        AggregateExpr::Collect(expr) => {
            let mut items = Vec::new();
            for row in rows {
                items.push(eval_expr(expr, schema, row, ctx)?);
            }
            Ok(ResultValue::List(items))
        }
        AggregateExpr::CollectDistinct(expr) => {
            let mut seen = HashSet::new();
            let mut items = Vec::new();
            for row in rows {
                let val = eval_expr(expr, schema, row, ctx)?;
                let key = format!("{val:?}");
                if !seen.contains(&key) && !matches!(val, ResultValue::Null) {
                    seen.insert(key);
                    items.push(val);
                }
            }
            Ok(ResultValue::List(items))
        }
    }
}

/**
 * @brief: Default value returned by aggregate functions on empty tables.
 */
fn eval_aggregate_empty(agg: &AggregateExpr) -> ResultValue {
    match agg {
        AggregateExpr::CountAll | AggregateExpr::Count(_) | AggregateExpr::CountDistinct(_) => {
            ResultValue::Int(0)
        }
        AggregateExpr::Collect(_) | AggregateExpr::CollectDistinct(_) => {
            ResultValue::List(Vec::new())
        }
        _ => ResultValue::Null,
    }
}

// ============================================================
// Union
// ============================================================

fn exec_union(
    all: bool,
    inputs: &[DefaultPhysicalOp],
    ctx: &ExecContext,
) -> Result<ExecValue> {
    if inputs.is_empty() {
        return Ok(ExecValue::Rows(Vec::new(), Vec::new()));
    }

    let mut all_schema = None;
    let mut all_rows = Vec::new();

    for input in inputs {
        let value = eval(input, ctx)?;
        let (schema, rows) = match value {
            ExecValue::Rows(s, r) => (s, r),
            ExecValue::Projection(_) => {
                return Err(Error::Execute("Union input cannot be GraphProjection".into()))
            }
        };

        match &all_schema {
            None => all_schema = Some(schema),
            Some(_s) => {
                // Check schema compatibility
            }
        }

        all_rows.extend(rows);
    }

    if !all {
        // UNION (deduplicate)
        all_rows.sort_by(|a, b| {
            for (i, _col) in a.iter().enumerate() {
                let cmp = compare_values(&a[i], &b[i]);
                if cmp != Some(std::cmp::Ordering::Equal) {
                    return cmp.unwrap_or(std::cmp::Ordering::Equal);
                }
            }
            std::cmp::Ordering::Equal
        });
        all_rows.dedup_by(|a, b| {
            a.iter()
                .zip(b.iter())
                .all(|(x, y)| compare_values(x, y) == Some(std::cmp::Ordering::Equal))
        });
    }

    Ok(ExecValue::Rows(
        all_schema.unwrap_or_default(),
        all_rows,
    ))
}

// ============================================================
// CrossJoin (Cartesian product)
// ============================================================

/**
 * @brief: Execute Cartesian product operator.
 *
 * Performs nested-loop Cartesian product on all rows from left and right subtrees.
 *
 * @param left: Left subtree physical operator.
 * @param right: Right subtree physical operator.
 * @param ctx: Execution context.
 * @return: Returns the concatenated row set.
 */
fn exec_cross_join(
    left: &DefaultPhysicalOp,
    right: &DefaultPhysicalOp,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (left_schema, left_rows) = match eval(left, ctx)? {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("CrossJoin input cannot be GraphProjection".into()))
        }
    };
    let (right_schema, right_rows) = match eval(right, ctx)? {
        ExecValue::Rows(s, r) => (s, r),
        ExecValue::Projection(_) => {
            return Err(Error::Execute("CrossJoin input cannot be GraphProjection".into()))
        }
    };

    // If either side is empty, Cartesian product result is empty
    if left_rows.is_empty() || right_rows.is_empty() {
        let mut schema = left_schema;
        schema.extend(right_schema);
        return Ok(ExecValue::Rows(schema, Vec::new()));
    }

    let mut out_schema = left_schema.clone();
    out_schema.extend(right_schema.clone());

    let mut out_rows = Vec::with_capacity(left_rows.len() * right_rows.len());
    for l_row in &left_rows {
        for r_row in &right_rows {
            let mut joined = l_row.clone();
            joined.extend(r_row.clone());
            out_rows.push(joined);
        }
    }

    Ok(ExecValue::Rows(out_schema, out_rows))
}

// ============================================================
// OLAP: BuildProjection / RunAlgorithm
// ============================================================

fn exec_build_projection(spec: &ProjectionSpec, ctx: &ExecContext) -> Result<ExecValue> {
    let proj = CsrProjection::build(&*ctx.storage, &ctx.txn, spec)?;
    Ok(ExecValue::Projection(Arc::new(proj)))
}

fn exec_run_algorithm(
    name: &str,
    args: &AlgorithmArgs,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let proj: Arc<dyn GraphProjection> = match child {
        ExecValue::Projection(p) => p,
        ExecValue::Rows(_, _) => {
            return Err(Error::Execute("RunAlgorithm input must be GraphProjection".into()))
        }
    };

    let algo = ctx
        .algorithms
        .algorithms
        .get(name)
        .ok_or_else(|| Error::Execute(format!("Unregistered algorithm: `{name}`")))?
        .clone();

    let result = algo.execute(&*proj, args)?;
    Ok(ExecValue::Rows(result.schema, result.rows))
}

// ============================================================
// Expression evaluator
// ============================================================

fn eval_expr(
    expr: &Expr,
    schema: &[String],
    row: &Row,
    ctx: &ExecContext,
) -> Result<ResultValue> {
    match expr {
        // Literals
        Expr::Int(v) => Ok(ResultValue::Int(*v)),
        Expr::Float(v) => Ok(ResultValue::Float(*v)),
        Expr::String(v) => Ok(ResultValue::Text(v.clone())),
        Expr::Bool(v) => Ok(ResultValue::Bool(*v)),
        Expr::Null => Ok(ResultValue::Null),

        // Variable and property references
        Expr::VarRef { name } => {
            let idx = col_index(schema, name)?;
            Ok(row[idx].clone())
        }
        Expr::PropRef { source, key } => {
            let src_val = eval_expr(source, schema, row, ctx)?;
            let vid = match src_val {
                ResultValue::VertexRef(v) => v,
                other => {
                    return Err(Error::Execute(format!(
                        "PropRef: expected VertexRef, got {other:?}"
                    )))
                }
            };
            let vrec = ctx
                .storage
                .get_vertex(vid, &ctx.txn)?
                .ok_or_else(|| Error::Execute(format!("Vertex {vid:?} no longer exists")))?;
            Ok(vrec
                .properties
                .get(key)
                .cloned()
                .map(property_to_result)
                .unwrap_or(ResultValue::Null))
        }

        // Arithmetic
        Expr::Neg(inner) => {
            let v = eval_expr(inner, schema, row, ctx)?;
            match v {
                ResultValue::Int(i) => Ok(ResultValue::Int(-i)),
                ResultValue::Float(f) => Ok(ResultValue::Float(-f)),
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Add(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            Ok(arith_op(&lv, &rv, |a, b| a + b, |a, b| a + b))
        }
        Expr::Sub(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            Ok(arith_op(&lv, &rv, |a, b| a - b, |a, b| a - b))
        }
        Expr::Mul(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            Ok(arith_op(&lv, &rv, |a, b| a * b, |a, b| a * b))
        }
        Expr::Div(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Int(a), ResultValue::Int(b)) if *b != 0 => {
                    Ok(ResultValue::Int(a / b))
                }
                (ResultValue::Float(a), ResultValue::Float(b)) if *b != 0.0 => {
                    Ok(ResultValue::Float(a / b))
                }
                (ResultValue::Int(a), ResultValue::Float(b)) if *b != 0.0 => {
                    Ok(ResultValue::Float(*a as f64 / b))
                }
                (ResultValue::Float(a), ResultValue::Int(b)) if *b != 0 => {
                    Ok(ResultValue::Float(a / *b as f64))
                }
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Mod(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Int(a), ResultValue::Int(b)) if *b != 0 => {
                    Ok(ResultValue::Int(a % b))
                }
                _ => Ok(ResultValue::Null),
            }
        }

        // Comparison operations
        Expr::Gt(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            Ok(ResultValue::Bool(
                compare_values(&lv, &rv) == Some(std::cmp::Ordering::Greater),
            ))
        }
        Expr::Gte(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match compare_values(&lv, &rv) {
                Some(o) => Ok(ResultValue::Bool(o != std::cmp::Ordering::Less)),
                None => Ok(ResultValue::Null),
            }
        }
        Expr::Lt(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            Ok(ResultValue::Bool(
                compare_values(&lv, &rv) == Some(std::cmp::Ordering::Less),
            ))
        }
        Expr::Lte(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match compare_values(&lv, &rv) {
                Some(o) => Ok(ResultValue::Bool(o != std::cmp::Ordering::Greater)),
                None => Ok(ResultValue::Null),
            }
        }
        Expr::Eq(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match compare_values(&lv, &rv) {
                Some(o) => Ok(ResultValue::Bool(o == std::cmp::Ordering::Equal)),
                None => Ok(ResultValue::Null),
            }
        }
        Expr::Neq(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match compare_values(&lv, &rv) {
                Some(o) => Ok(ResultValue::Bool(o != std::cmp::Ordering::Equal)),
                None => Ok(ResultValue::Null),
            }
        }

        // Logical operations
        Expr::And(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            if let ResultValue::Bool(false) = lv {
                return Ok(ResultValue::Bool(false));
            }
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Bool(false), _) => Ok(ResultValue::Bool(false)),
                (_, ResultValue::Bool(false)) => Ok(ResultValue::Bool(false)),
                (ResultValue::Null, _) | (_, ResultValue::Null) => Ok(ResultValue::Null),
                (ResultValue::Bool(_), ResultValue::Bool(_)) => Ok(ResultValue::Bool(true)),
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Or(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            if let ResultValue::Bool(true) = lv {
                return Ok(ResultValue::Bool(true));
            }
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Bool(true), _) => Ok(ResultValue::Bool(true)),
                (_, ResultValue::Bool(true)) => Ok(ResultValue::Bool(true)),
                (ResultValue::Null, _) | (_, ResultValue::Null) => Ok(ResultValue::Null),
                (ResultValue::Bool(false), ResultValue::Bool(false)) => {
                    Ok(ResultValue::Bool(false))
                }
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Not(inner) => {
            let v = eval_expr(inner, schema, row, ctx)?;
            match v {
                ResultValue::Bool(b) => Ok(ResultValue::Bool(!b)),
                ResultValue::Null => Ok(ResultValue::Null),
                _ => Ok(ResultValue::Null),
            }
        }

        // Null testing
        Expr::IsNull(inner) => {
            let v = eval_expr(inner, schema, row, ctx)?;
            Ok(ResultValue::Bool(matches!(v, ResultValue::Null)))
        }
        Expr::IsNotNull(inner) => {
            let v = eval_expr(inner, schema, row, ctx)?;
            Ok(ResultValue::Bool(!matches!(v, ResultValue::Null)))
        }

        // String operations (simplified implementation)
        Expr::StartsWith(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Text(a), ResultValue::Text(b)) => {
                    Ok(ResultValue::Bool(a.starts_with(b.as_str())))
                }
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::EndsWith(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Text(a), ResultValue::Text(b)) => {
                    Ok(ResultValue::Bool(a.ends_with(b.as_str())))
                }
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Contains(l, r) => {
            let lv = eval_expr(l, schema, row, ctx)?;
            let rv = eval_expr(r, schema, row, ctx)?;
            match (&lv, &rv) {
                (ResultValue::Text(a), ResultValue::Text(b)) => {
                    Ok(ResultValue::Bool(a.contains(b.as_str())))
                }
                _ => Ok(ResultValue::Null),
            }
        }
        Expr::Concat(args) => {
            let mut result = String::new();
            for arg in args {
                if let ResultValue::Text(s) = eval_expr(arg, schema, row, ctx)? {
                    result.push_str(&s);
                }
            }
            Ok(ResultValue::Text(result))
        }

        // ── List / function calls ──
        Expr::List(items) => {
            let elems: Vec<ResultValue> = items
                .iter()
                .map(|e| eval_expr(e, schema, row, ctx))
                .collect::<Result<Vec<_>>>()?;
            Ok(ResultValue::List(elems))
        }
        Expr::ListIndex { list, index } => {
            let _lv = eval_expr(list, schema, row, ctx)?;
            let _iv = eval_expr(index, schema, row, ctx)?;
            Ok(ResultValue::Null) // List operations not yet supported
        }
        Expr::Size { value: _ } => Ok(ResultValue::Null),
        Expr::FunctionCall { name, args } => exec_function(name, args, schema, row, ctx),
        Expr::Case {
            cases,
            else_branch,
        } => {
            for (cond, then_val) in cases {
                let cv = eval_expr(cond, schema, row, ctx)?;
                if is_truthy(&cv) {
                    return eval_expr(then_val, schema, row, ctx);
                }
            }
            match else_branch {
                Some(e) => eval_expr(e, schema, row, ctx),
                None => Ok(ResultValue::Null),
            }
        }
        Expr::Coalesce(args) => {
            for arg in args {
                let v = eval_expr(arg, schema, row, ctx)?;
                if !matches!(v, ResultValue::Null) {
                    return Ok(v);
                }
            }
            Ok(ResultValue::Null)
        }
    }
}

/**
 * @brief: Execute built-in function calls.
 */
fn exec_function(
    name: &str,
    args: &[Expr],
    schema: &[String],
    row: &Row,
    ctx: &ExecContext,
) -> Result<ResultValue> {
    match name.to_lowercase().as_str() {
        "count" => {
            if args.is_empty() {
                Ok(ResultValue::Int(1))
            } else {
                let v = eval_expr(&args[0], schema, row, ctx)?;
                Ok(ResultValue::Int(if matches!(v, ResultValue::Null) { 0 } else { 1 }))
            }
        }
        "coalesce" => {
            for arg in args {
                let v = eval_expr(arg, schema, row, ctx)?;
                if !matches!(v, ResultValue::Null) {
                    return Ok(v);
                }
            }
            Ok(ResultValue::Null)
        }
        "size" => {
            if let Some(arg) = args.first() {
                let v = eval_expr(arg, schema, row, ctx)?;
                match v {
                    ResultValue::Text(s) => Ok(ResultValue::Int(s.len() as i64)),
                    _ => Ok(ResultValue::Null),
                }
            } else {
                Ok(ResultValue::Null)
            }
        }
        "tointeger" => {
            if let Some(arg) = args.first() {
                let v = eval_expr(arg, schema, row, ctx)?;
                match v {
                    ResultValue::Int(i) => Ok(ResultValue::Int(i)),
                    ResultValue::Float(f) => Ok(ResultValue::Int(f as i64)),
                    ResultValue::Text(s) => Ok(s.parse::<i64>().map(ResultValue::Int).unwrap_or(ResultValue::Null)),
                    _ => Ok(ResultValue::Null),
                }
            } else {
                Ok(ResultValue::Null)
            }
        }
        "tostring" => {
            if let Some(arg) = args.first() {
                let v = eval_expr(arg, schema, row, ctx)?;
                match v {
                    ResultValue::Text(s) => Ok(ResultValue::Text(s)),
                    ResultValue::Int(i) => Ok(ResultValue::Text(i.to_string())),
                    ResultValue::Float(f) => Ok(ResultValue::Text(f.to_string())),
                    ResultValue::Bool(b) => Ok(ResultValue::Text(b.to_string())),
                    _ => Ok(ResultValue::Null),
                }
            } else {
                Ok(ResultValue::Null)
            }
        }
        _ => Err(Error::Execute(format!("Unregistered function: `{name}`"))),
    }
}

// ============================================================
// Arithmetic helpers
// ============================================================

fn arith_op<FInt, FFloat>(a: &ResultValue, b: &ResultValue, f_int: FInt, f_float: FFloat) -> ResultValue
where
    FInt: Fn(i64, i64) -> i64,
    FFloat: Fn(f64, f64) -> f64,
{
    match (a, b) {
        (ResultValue::Int(x), ResultValue::Int(y)) => ResultValue::Int(f_int(*x, *y)),
        (ResultValue::Float(x), ResultValue::Float(y)) => ResultValue::Float(f_float(*x, *y)),
        (ResultValue::Int(x), ResultValue::Float(y)) => ResultValue::Float(f_float(*x as f64, *y)),
        (ResultValue::Float(x), ResultValue::Int(y)) => ResultValue::Float(f_float(*x, *y as f64)),
        _ => ResultValue::Null,
    }
}

// ============================================================
// DML operators
// ============================================================

/**
 * @brief: Execute CreateNode physical operator.
 * @param var: Bound variable name for the new vertex.
 * @param label_id: Optional label ID.
 * @param properties: List of properties to set (PropertyKeyId, Expr).
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Result with VertexRef column appended to input rows.
 */
fn exec_create_node(
    var: &str,
    label_id: Option<LabelId>,
    properties: &[(PropertyKeyId, Expr)],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (mut schema, rows) = unpack_rows(child)?;
    let labels: Vec<LabelId> = label_id.map(|l| vec![l]).unwrap_or_default();

    let mut out_rows = Vec::with_capacity(rows.len());
    for row in &rows {
        // Evaluate each property
        let mut prop_map = PropertyMap::new();
        for (key_id, expr) in properties {
            let val = eval_expr(expr, &schema, row, ctx)?;
            prop_map.insert(*key_id, result_to_property(val)?);
        }

        let record = VertexRecord {
            id: VertexId(0), // Assigned by storage engine
            labels: labels.clone(),
            properties: prop_map,
        };
        let vid = ctx.storage.insert_vertex(record, &ctx.txn)?;

        let mut out_row = row.clone();
        out_row.push(ResultValue::VertexRef(vid));
        out_rows.push(out_row);
    }

    schema.push(var.to_string());
    Ok(ExecValue::Rows(schema, out_rows))
}

/**
 * @brief: Execute CreateEdge physical operator.
 * @param from_var: Source vertex variable name.
 * @param to_var: Target vertex variable name.
 * @param edge_var: Optional edge variable name.
 * @param edge_type_ids: Optional edge type ID list.
 * @param properties: List of edge properties to set.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Result with EdgeRef column appended to input rows.
 */
fn exec_create_edge(
    from_var: &str,
    to_var: &str,
    edge_var: Option<&str>,
    edge_type_ids: Option<&[EdgeTypeId]>,
    properties: &[(PropertyKeyId, Expr)],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (mut schema, rows) = unpack_rows(child)?;
    let from_idx = col_index(&schema, from_var)?;
    let to_idx = col_index(&schema, to_var)?;

    // Take the first edge type (current storage model only supports single EdgeTypeId)
    let etype = match edge_type_ids.and_then(|t| t.first().copied()) {
        Some(et) => et,
        None => {
            return Err(Error::Execute(
                "CreateEdge must specify at least one edge type".into(),
            ))
        }
    };

    let mut out_rows = Vec::with_capacity(rows.len());
    for row in &rows {
        let src_vid = extract_vid(&row[from_idx])?;
        let dst_vid = extract_vid(&row[to_idx])?;

        let mut prop_map = PropertyMap::new();
        for (key_id, expr) in properties {
            let val = eval_expr(expr, &schema, row, ctx)?;
            prop_map.insert(*key_id, result_to_property(val)?);
        }

        let record = EdgeRecord {
            id: EdgeId(0), // Assigned by storage engine
            src: src_vid,
            dst: dst_vid,
            etype,
            properties: prop_map,
        };
        let eid = ctx.storage.insert_edge(record, &ctx.txn)?;

        let mut out_row = row.clone();
        // Always append edge reference column to keep schema aligned with row data
        out_row.push(ResultValue::EdgeRef(eid));
        out_rows.push(out_row);
    }

    // Always append schema column (anonymous edges use reserved name to ensure column alignment)
    schema.push(edge_var.unwrap_or("__anon_edge").to_string());
    Ok(ExecValue::Rows(schema, out_rows))
}

/**
 * @brief: Execute SetProperty physical operator.
 * @param var: Target vertex variable name.
 * @param key_id: Property key ID.
 * @param value: New value expression.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Pass through input rows unchanged.
 */
fn exec_set_property(
    var: &str,
    key_id: PropertyKeyId,
    value: &Expr,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, rows) = unpack_rows(child)?;
    let var_idx = col_index(&schema, var)?;

    for row in &rows {
        let vid = extract_vid(&row[var_idx])?;
        let mut vrec = ctx
            .storage
            .get_vertex(vid, &ctx.txn)?
            .ok_or_else(|| Error::Execute(format!("Vertex {vid:?} does not exist")))?;

        let val = eval_expr(value, &schema, row, ctx)?;
        vrec.properties.insert(key_id, result_to_property(val)?);
        ctx.storage.update_vertex(vrec, &ctx.txn)?;
    }

    Ok(ExecValue::Rows(schema, rows))
}

/**
 * @brief: Execute SetLabels physical operator.
 * @param var: Target vertex variable name.
 * @param label_ids: List of label IDs to add.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Pass through input rows unchanged.
 */
fn exec_set_labels(
    var: &str,
    label_ids: &[LabelId],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, rows) = unpack_rows(child)?;
    let var_idx = col_index(&schema, var)?;

    for row in &rows {
        let vid = extract_vid(&row[var_idx])?;
        let mut vrec = ctx
            .storage
            .get_vertex(vid, &ctx.txn)?
            .ok_or_else(|| Error::Execute(format!("Vertex {vid:?} does not exist")))?;

        // Merge labels (deduplicate)
        for &lid in label_ids {
            if !vrec.labels.contains(&lid) {
                vrec.labels.push(lid);
            }
        }
        ctx.storage.update_vertex(vrec, &ctx.txn)?;
    }

    Ok(ExecValue::Rows(schema, rows))
}

/**
 * @brief: Execute RemoveProperty physical operator.
 * @param var: Target vertex variable name.
 * @param key_id: Property key ID to remove.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Pass through input rows unchanged.
 */
fn exec_remove_property(
    var: &str,
    key_id: PropertyKeyId,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, rows) = unpack_rows(child)?;
    let var_idx = col_index(&schema, var)?;

    for row in &rows {
        let vid = extract_vid(&row[var_idx])?;
        let mut vrec = ctx
            .storage
            .get_vertex(vid, &ctx.txn)?
            .ok_or_else(|| Error::Execute(format!("Vertex {vid:?} does not exist")))?;

        vrec.properties.remove(&key_id);
        ctx.storage.update_vertex(vrec, &ctx.txn)?;
    }

    Ok(ExecValue::Rows(schema, rows))
}

/**
 * @brief: Execute RemoveLabels physical operator.
 * @param var: Target vertex variable name.
 * @param label_ids: List of label IDs to remove.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Pass through input rows unchanged.
 */
fn exec_remove_labels(
    var: &str,
    label_ids: &[LabelId],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, rows) = unpack_rows(child)?;
    let var_idx = col_index(&schema, var)?;

    for row in &rows {
        let vid = extract_vid(&row[var_idx])?;
        let mut vrec = ctx
            .storage
            .get_vertex(vid, &ctx.txn)?
            .ok_or_else(|| Error::Execute(format!("Vertex {vid:?} does not exist")))?;

        vrec.labels.retain(|l| !label_ids.contains(l));
        ctx.storage.update_vertex(vrec, &ctx.txn)?;
    }

    Ok(ExecValue::Rows(schema, rows))
}

/**
 * @brief: Execute Delete physical operator (including DETACH DELETE).
 * @param detach: Whether to cascade delete adjacent edges.
 * @param targets: List of target expressions to delete.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Pass through input rows unchanged.
 */
fn exec_delete(
    detach: bool,
    targets: &[Expr],
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (schema, rows) = unpack_rows(child)?;

    for row in &rows {
        for target in targets {
            let val = eval_expr(target, &schema, row, ctx)?;
            match val {
                ResultValue::VertexRef(vid) => {
                    if detach {
                        // Collect and delete all adjacent edges
                        let out_edges: Vec<_> = ctx
                            .storage
                            .out_edges(vid, EdgeFilter::default(), &ctx.txn)?
                            .collect();
                        for e in &out_edges {
                            ctx.storage.delete_edge(e.id, &ctx.txn)?;
                        }
                        let in_edges: Vec<_> = ctx
                            .storage
                            .in_edges(vid, EdgeFilter::default(), &ctx.txn)?
                            .collect();
                        for e in &in_edges {
                            ctx.storage.delete_edge(e.id, &ctx.txn)?;
                        }
                    }
                    ctx.storage.delete_vertex(vid, &ctx.txn)?;
                }
                ResultValue::EdgeRef(eid) => {
                    ctx.storage.delete_edge(eid, &ctx.txn)?;
                }
                other => {
                    return Err(Error::Execute(format!(
                        "DELETE target must be a vertex or edge reference, got: {other:?}"
                    )));
                }
            }
        }
    }

    Ok(ExecValue::Rows(schema, rows))
}

/**
 * @brief: Execute Unwind physical operator.
 * @param expr: List expression to unwind.
 * @param var: Variable name to bind to each unwound element.
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Unwound row set.
 */
fn exec_unwind(
    expr: &Expr,
    var: &str,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    let (mut schema, rows) = unpack_rows(child)?;

    let mut out_rows = Vec::new();
    for row in &rows {
        let val = eval_expr(expr, &schema, row, ctx)?;
        match val {
            ResultValue::List(elems) => {
                for elem in elems {
                    let mut out_row = row.clone();
                    out_row.push(elem);
                    out_rows.push(out_row);
                }
            }
            other => {
                // Non-list value: implicitly wrap as single-element list
                let mut out_row = row.clone();
                out_row.push(other);
                out_rows.push(out_row);
            }
        }
    }

    schema.push(var.to_string());
    Ok(ExecValue::Rows(schema, out_rows))
}

// ============================================================
// MERGE (match-or-create)
// ============================================================

/**
 * @brief: Execute MERGE operator (match-or-create semantics).
 *
 * For each target (node or edge) in the payload, first look for a match in storage;
 * if found, execute ON MATCH SET actions; if not found, create and execute ON CREATE SET actions.
 *
 * @param payload: MERGE payload (targets + actions).
 * @param child: Child operator execution result.
 * @param ctx: Execution context.
 * @return: Result row set (with variable reference columns for each target appended).
 */
fn exec_merge(
    payload: &MergePayload,
    child: ExecValue,
    ctx: &ExecContext,
) -> Result<ExecValue> {
    use super::physical_ops::MergeTarget;

    let (mut schema, rows) = unpack_rows(child)?;

    // If no input rows, drive with a single row
    let rows = if rows.is_empty() {
        vec![Vec::new()]
    } else {
        rows
    };

    let mut out_rows = Vec::new();
    for row in &rows {
        let mut out_row = row.clone();
        let mut binding: std::collections::HashMap<String, ResultValue> =
            std::collections::HashMap::new();

        // Inherit existing variable bindings from input rows
        for (i, col) in schema.iter().enumerate() {
            if i < out_row.len() {
                binding.insert(col.clone(), out_row[i].clone());
            }
        }

        let mut any_created = false;

        for target in &payload.targets {
            match target {
                MergeTarget::Node {
                    var,
                    label_id,
                    properties,
                } => {
                    // Try to match an existing node
                    let found = find_matching_vertex(*label_id, properties, &schema, row, ctx)?;

                    match found {
                        Some(vid) => {
                            binding.insert(var.clone(), ResultValue::VertexRef(vid));
                        }
                        None => {
                            // Create a new node
                            let labels: Vec<LabelId> =
                                label_id.map(|l| vec![l]).unwrap_or_default();
                            let mut prop_map = PropertyMap::new();
                            for (key_id, expr) in properties {
                                let val = eval_expr(expr, &schema, row, ctx)?;
                                prop_map.insert(*key_id, result_to_property(val)?);
                            }
                            let record = VertexRecord {
                                id: VertexId(0),
                                labels,
                                properties: prop_map,
                            };
                            let vid = ctx.storage.insert_vertex(record, &ctx.txn)?;
                            binding.insert(var.clone(), ResultValue::VertexRef(vid));
                            any_created = true;
                        }
                    }
                }
                MergeTarget::Edge {
                    from_var,
                    to_var,
                    edge_var,
                    edge_type_ids,
                    properties,
                } => {
                    let from_vid = match binding.get(from_var) {
                        Some(ResultValue::VertexRef(v)) => *v,
                        other => {
                            return Err(Error::Execute(format!(
                                "MERGE edge: from_var '{from_var}' is not VertexRef: {other:?}"
                            )));
                        }
                    };
                    let to_vid = match binding.get(to_var) {
                        Some(ResultValue::VertexRef(v)) => *v,
                        other => {
                            return Err(Error::Execute(format!(
                                "MERGE edge: to_var '{to_var}' is not VertexRef: {other:?}"
                            )));
                        }
                    };

                    let etype = edge_type_ids.first().copied().unwrap_or_else(|| {
                        // Having a type list but empty shouldn't happen, give a placeholder
                        metagdb_core::common::ids::EdgeTypeId(0)
                    });

                    // Look up an existing edge
                    let found = find_matching_edge(
                        from_vid, to_vid, etype, properties, &schema, row, ctx,
                    )?;

                    match found {
                        Some(eid) => {
                            binding.insert(edge_var.clone(), ResultValue::EdgeRef(eid));
                        }
                        None => {
                            let mut prop_map = PropertyMap::new();
                            for (key_id, expr) in properties {
                                let val = eval_expr(expr, &schema, row, ctx)?;
                                prop_map.insert(*key_id, result_to_property(val)?);
                            }
                            let record = EdgeRecord {
                                id: EdgeId(0),
                                src: from_vid,
                                dst: to_vid,
                                etype,
                                properties: prop_map,
                            };
                            let eid = ctx.storage.insert_edge(record, &ctx.txn)?;
                            binding.insert(edge_var.clone(), ResultValue::EdgeRef(eid));
                            any_created = true;
                        }
                    }
                }
            }
        }

        // Execute ON CREATE / ON MATCH SET actions
        let actions = if any_created {
            &payload.on_create
        } else {
            &payload.on_match
        };
        for (var, key_id, expr) in actions {
            let vid = match binding.get(var) {
                Some(ResultValue::VertexRef(v)) => *v,
                other => {
                    return Err(Error::Execute(format!(
                        "MERGE SET: variable '{var}' is not VertexRef: {other:?}"
                    )));
                }
            };
            let mut vrec = ctx
                .storage
                .get_vertex(vid, &ctx.txn)?
                .ok_or_else(|| Error::Execute(format!("MERGE SET: vertex {vid:?} does not exist")))?;
            let val = eval_expr(expr, &schema, row, ctx)?;
            vrec.properties.insert(*key_id, result_to_property(val)?);
            ctx.storage.update_vertex(vrec, &ctx.txn)?;
        }

        // Append all bound variables to the output row
        for target in &payload.targets {
            match target {
                MergeTarget::Node { var, .. } => {
                    if let Some(rv) = binding.get(var) {
                        out_row.push(rv.clone());
                    }
                }
                MergeTarget::Edge { edge_var, .. } => {
                    if let Some(rv) = binding.get(edge_var) {
                        out_row.push(rv.clone());
                    }
                }
            }
        }
        out_rows.push(out_row);
    }

    // Build output schema
    for target in &payload.targets {
        match target {
            MergeTarget::Node { var, .. } => schema.push(var.clone()),
            MergeTarget::Edge { edge_var, .. } => schema.push(edge_var.clone()),
        }
    }

    Ok(ExecValue::Rows(schema, out_rows))
}

/**
 * @brief: Find a matching vertex (filter by label + properties).
 */
fn find_matching_vertex(
    label_id: Option<LabelId>,
    properties: &[(PropertyKeyId, Expr)],
    schema: &[String],
    row: &Row,
    ctx: &ExecContext,
) -> Result<Option<VertexId>> {
    let filter = VertexFilter {
        labels: label_id.map(|l| vec![l]),
    };
    let iter = ctx.storage.scan_vertices(filter, &ctx.txn)?;
    for vrec in iter {
        let mut matched = true;
        for (key_id, expr) in properties {
            let expected = eval_expr(expr, schema, row, ctx)?;
            let actual = vrec
                .properties
                .get(key_id)
                .cloned()
                .map(property_to_result)
                .unwrap_or(ResultValue::Null);
            if compare_values(&expected, &actual) != Some(std::cmp::Ordering::Equal) {
                matched = false;
                break;
            }
        }
        if matched {
            return Ok(Some(vrec.id));
        }
    }
    Ok(None)
}

/**
 * @brief: Find a matching edge (filter by endpoints + type + properties).
 */
fn find_matching_edge(
    from_vid: VertexId,
    to_vid: VertexId,
    etype: EdgeTypeId,
    properties: &[(PropertyKeyId, Expr)],
    schema: &[String],
    row: &Row,
    ctx: &ExecContext,
) -> Result<Option<EdgeId>> {
    let efilter = EdgeFilter {
        types: Some(vec![etype]),
    };
    let edges: Vec<_> = ctx
        .storage
        .out_edges(from_vid, efilter, &ctx.txn)?
        .collect();
    for erec in edges {
        if erec.dst != to_vid {
            continue;
        }
        let mut matched = true;
        for (key_id, expr) in properties {
            let expected = eval_expr(expr, schema, row, ctx)?;
            let actual = erec
                .properties
                .get(key_id)
                .cloned()
                .map(property_to_result)
                .unwrap_or(ResultValue::Null);
            if compare_values(&expected, &actual) != Some(std::cmp::Ordering::Equal) {
                matched = false;
                break;
            }
        }
        if matched {
            return Ok(Some(erec.id));
        }
    }
    Ok(None)
}

// ============================================================
// DML helper functions
// ============================================================

/**
 * @brief: Unpack (schema, rows) from ExecValue.
 */
fn unpack_rows(value: ExecValue) -> Result<(Vec<String>, Vec<Row>)> {
    match value {
        ExecValue::Rows(s, r) => Ok((s, r)),
        ExecValue::Projection(_) => Err(Error::Execute("DML operator input cannot be GraphProjection".into())),
    }
}

/**
 * @brief: Extract VertexId from ResultValue.
 */
fn extract_vid(val: &ResultValue) -> Result<VertexId> {
    match val {
        ResultValue::VertexRef(v) => Ok(*v),
        other => Err(Error::Execute(format!("Expected VertexRef, got: {other:?}"))),
    }
}

/**
 * @brief: Convert ResultValue to PropertyValue.
 */
fn result_to_property(val: ResultValue) -> Result<PropertyValue> {
    match val {
        ResultValue::Null => Ok(PropertyValue::Null),
        ResultValue::Bool(b) => Ok(PropertyValue::Bool(b)),
        ResultValue::Int(i) => Ok(PropertyValue::Int(i)),
        ResultValue::Float(f) => Ok(PropertyValue::Float(f)),
        ResultValue::Text(s) => Ok(PropertyValue::Text(s)),
        ResultValue::Property(p) => Ok(p),
        other => Err(Error::Execute(format!(
            "Cannot convert {other:?} to PropertyValue"
        ))),
    }
}

// ============================================================
// General helpers
// ============================================================

fn col_index(schema: &[String], name: &str) -> Result<usize> {
    schema
        .iter()
        .position(|c| c == name)
        .ok_or_else(|| Error::Execute(format!("Column '{name}' not in schema {schema:?}")))
}

fn compare_values(a: &ResultValue, b: &ResultValue) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (ResultValue::Int(x), ResultValue::Int(y)) => x.partial_cmp(y),
        (ResultValue::Float(x), ResultValue::Float(y)) => x.partial_cmp(y),
        (ResultValue::Int(x), ResultValue::Float(y)) => (*x as f64).partial_cmp(y),
        (ResultValue::Float(x), ResultValue::Int(y)) => x.partial_cmp(&(*y as f64)),
        (ResultValue::Text(x), ResultValue::Text(y)) => x.partial_cmp(y),
        (ResultValue::Bool(x), ResultValue::Bool(y)) => x.partial_cmp(y),
        (ResultValue::VertexRef(x), ResultValue::VertexRef(y)) => x.0.partial_cmp(&y.0),
        (ResultValue::Null, _) | (_, ResultValue::Null) => None,
        _ => None,
    }
}

fn property_to_result(v: PropertyValue) -> ResultValue {
    match v {
        PropertyValue::Null => ResultValue::Null,
        PropertyValue::Bool(b) => ResultValue::Bool(b),
        PropertyValue::Int(i) => ResultValue::Int(i),
        PropertyValue::Float(f) => ResultValue::Float(f),
        PropertyValue::Text(s) => ResultValue::Text(s),
    }
}

fn is_truthy(v: &ResultValue) -> bool {
    match v {
        ResultValue::Bool(true) => true,
        ResultValue::Null => false,
        ResultValue::Bool(false) => false,
        _ => false,
    }
}
