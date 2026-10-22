//! Push-based executor.
//!
//! Each clause stage consumes the input row set and emits the next row set; the
//! pipeline is driven forward one stage at a time. OLAP algorithms are invoked
//! through the framework registry with a freshly built CSR projection.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, ResultValue};
use metagdb_core::l2_storage::{EdgeFilter, EdgeRecord, VertexFilter, VertexRecord};
use metagdb_core::l4_compute::algebra::{AlgorithmArgs, PhysicalPlan, ProjectionDirection, ProjectionSpec};
use metagdb_core::l4_compute::engine::{ExecContext, Executor, ResultStream};
use metagdb_core::l4_compute::olap::AlgorithmRegistry;
use metagdb_core::{Error, Result};

use super::ast::Direction;
use super::functions::{call as call_function, EvalCtx};
use super::ir::{
    row_get, row_put, value_from_property, value_to_property, value_to_result, AggregateKind, BExpr,
    QuantifierKind, Row, Value,
};
use super::physical::PhysicalQuery;
use super::plan::{
    BoundElement, BoundNode, BoundPatternPart, BoundRel, BoundRemoveItem, BoundSetItem, CallStage,
    MatchStage, MergeStage, ProjectStage, Stage,
};
use super::projection::CsrProjection;

/// Executor stage.
#[derive(Default)]
pub struct MetagdbExecutor;

impl Executor for MetagdbExecutor {
    fn execute(&self, pp: &PhysicalPlan, ctx: &ExecContext) -> Result<ResultStream> {
        let query = pp
            .downcast_ref::<PhysicalQuery>()
            .ok_or_else(|| Error::Execute("MetagdbExecutor received a foreign physical plan".into()))?;
        let context = EvalCtx {
            storage: &*ctx.storage,
            txn: &ctx.txn,
            schema: &*ctx.schema,
        };
        let (head_rows, columns) = run_pipeline(&context, &ctx.algorithms, query.stages())?;
        let mut value_rows = rows_to_values(&head_rows, &columns);
        if !query.unions().is_empty() {
            let all_all = query.unions().iter().all(|(all, _)| *all);
            for (_, stages) in query.unions() {
                let (branch_rows, _) = run_pipeline(&context, &ctx.algorithms, stages)?;
                let branch_values = rows_to_values(&branch_rows, &columns);
                value_rows.extend(branch_values);
            }
            if !all_all {
                let mut seen = std::collections::HashSet::new();
                value_rows.retain(|row| {
                    let key = row
                        .iter()
                        .map(canonical_result)
                        .collect::<Vec<_>>()
                        .join("|");
                    seen.insert(key)
                });
            }
        }
        Ok(ResultStream {
            schema: columns,
            rows: value_rows,
        })
    }
}

fn canonical_result(value: &ResultValue) -> String {
    format!("{value:?}")
}

fn rows_to_values(rows: &[Row], columns: &[String]) -> Vec<Vec<ResultValue>> {
    rows.iter()
        .map(|row| {
            columns
                .iter()
                .map(|column| {
                    row_get(row, column)
                        .map(value_to_result)
                        .unwrap_or(ResultValue::Null)
                })
                .collect()
        })
        .collect()
}

fn run_pipeline(
    ctx: &EvalCtx<'_>,
    algorithms: &Arc<AlgorithmRegistry>,
    stages: &[Stage],
) -> Result<(Vec<Row>, Vec<String>)> {
    let mut rows: Vec<Row> = vec![Vec::new()];
    let mut columns: Vec<String> = Vec::new();
    for stage in stages {
        rows = apply_stage(ctx, algorithms, stage, rows, &mut columns)?;
    }
    if columns.is_empty() {
        columns = rows
            .first()
            .map(|row| row.iter().map(|(name, _)| name.clone()).collect())
            .unwrap_or_default();
    }
    Ok((rows, columns))
}

fn apply_stage(
    ctx: &EvalCtx<'_>,
    algorithms: &Arc<AlgorithmRegistry>,
    stage: &Stage,
    rows: Vec<Row>,
    columns: &mut Vec<String>,
) -> Result<Vec<Row>> {
    match stage {
        Stage::Match(m) => match_stage(ctx, m, rows),
        Stage::Unwind { expr, var } => unwind_stage(ctx, expr, var, rows),
        Stage::Project(p) => project_stage(ctx, p, rows, columns),
        Stage::Create(parts) => create_stage(ctx, parts, rows),
        Stage::Merge(m) => merge_stage(ctx, m, rows),
        Stage::Delete { detach, exprs } => delete_stage(ctx, *detach, exprs, rows),
        Stage::Set(items) => set_stage(ctx, items, rows),
        Stage::Remove(items) => remove_stage(ctx, items, rows),
        Stage::Call(call) => call_stage(ctx, algorithms, call, rows, columns),
    }
}

// ---------------------------------------------------------------------------
// Match
// ---------------------------------------------------------------------------

fn match_stage(ctx: &EvalCtx<'_>, stage: &MatchStage, input: Vec<Row>) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        let mut matched: Vec<Row> = vec![row.clone()];
        for (index, part) in stage.parts.iter().enumerate() {
            let hint = if index == 0 {
                stage.id_hint.as_ref()
            } else {
                None
            };
            let end_hint = if index == 0 {
                stage.end_hint.as_ref()
            } else {
                None
            };
            let shortest = index == 0 && stage.shortest_path;
            let mut next = Vec::new();
            for current in &matched {
                next.extend(match_element(
                    ctx,
                    &part.element,
                    part.path_var.as_deref(),
                    current,
                    hint,
                    end_hint,
                    shortest,
                )?);
            }
            matched = next;
            if matched.is_empty() {
                break;
            }
        }
        if matched.is_empty() {
            if stage.optional {
                let mut nulls = row.clone();
                bind_nulls(&mut nulls, stage);
                output.push(nulls);
            }
            continue;
        }
        for matched_row in matched {
            if let Some(predicate) = &stage.where_clause {
                let value = eval(ctx, predicate, &matched_row, None)?;
                if !value.is_true() {
                    continue;
                }
            }
            output.push(matched_row);
        }
    }
    Ok(output)
}

fn bind_nulls(row: &mut Row, stage: &MatchStage) {
    for part in &stage.parts {
        collect_nulls(&part.element, row);
    }
}

fn collect_nulls(element: &BoundElement, row: &mut Row) {
    if let Some(var) = &element.node.var {
        row_put(row, var, Value::Null);
    }
    for (rel, node) in &element.chains {
        if let Some(var) = &rel.var {
            row_put(row, var, Value::Null);
        }
        if let Some(var) = &node.var {
            row_put(row, var, Value::Null);
        }
    }
}

fn match_element(
    ctx: &EvalCtx<'_>,
    element: &BoundElement,
    path_var: Option<&str>,
    row: &Row,
    hint: Option<&(String, u64)>,
    end_hint: Option<&(String, u64)>,
    shortest_path: bool,
) -> Result<Vec<Row>> {
    let anchor = &element.node;
    let candidates: Vec<(Option<String>, VertexId)> = if let Some(var) = &anchor.var {
        if let Some(value) = row_get(row, var) {
            match value {
                Value::Node(id) => vec![(Some(var.clone()), *id)],
                _ => Vec::new(),
            }
        } else if let Some((hint_var, id)) = hint {
            if hint_var == var {
                match ctx.storage.get_vertex(VertexId(*id), ctx.txn)? {
                    Some(record) => vec![(Some(var.clone()), record.id)],
                    None => Vec::new(),
                }
            } else {
                scan_candidates(ctx, anchor)?
            }
        } else {
            scan_candidates(ctx, anchor)?
        }
    } else {
        scan_candidates(ctx, anchor)?
    };

    let mut results = Vec::new();
    for (var, id) in candidates {
        let mut bound = row.clone();
        if let Some(var) = &var {
            row_put(&mut bound, var, Value::Node(id));
        }
        if !node_predicates(ctx, anchor, id, &bound)? {
            continue;
        }
        let path = vec![Value::Node(id)];
        expand_chains(
            ctx,
            element,
            0,
            id,
            bound,
            path,
            path_var,
            &mut results,
            end_hint,
            shortest_path,
        )?;
    }
    Ok(results)
}

fn scan_candidates(ctx: &EvalCtx<'_>, node: &BoundNode) -> Result<Vec<(Option<String>, VertexId)>> {
    let filter = VertexFilter {
        labels: if node.labels.is_empty() {
            None
        } else {
            Some(node.labels.clone())
        },
    };
    let mut candidates = Vec::new();
    for record in ctx.storage.scan_vertices(filter, ctx.txn)? {
        candidates.push((node.var.clone(), record.id));
    }
    Ok(candidates)
}

fn node_predicates(ctx: &EvalCtx<'_>, node: &BoundNode, id: VertexId, row: &Row) -> Result<bool> {
    if node.labels.is_empty() && node.properties.is_empty() {
        return Ok(true);
    }
    let record = match ctx.storage.get_vertex(id, ctx.txn)? {
        Some(record) => record,
        None => return Ok(false),
    };
    for label in &node.labels {
        if !record.labels.contains(label) {
            return Ok(false);
        }
    }
    for (key, expr) in &node.properties {
        let expected = eval(ctx, expr, row, None)?;
        let actual = record
            .properties
            .get(key)
            .map(value_from_property)
            .unwrap_or(Value::Null);
        if !property_matches(&actual, &expected) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn property_matches(actual: &Value, expected: &Value) -> bool {
    match expected {
        Value::Null => actual.is_null(),
        _ => actual.equals(expected),
    }
}

fn rel_predicates(
    ctx: &EvalCtx<'_>,
    rel: &BoundRel,
    edge_id: EdgeId,
    row: &Row,
) -> Result<bool> {
    if rel.properties.is_empty() {
        return Ok(true);
    }
    let record = match ctx.storage.get_edge(edge_id, ctx.txn)? {
        Some(record) => record,
        None => return Ok(false),
    };
    for (key, expr) in &rel.properties {
        let mut bound = row.clone();
        if let Some(var) = &rel.var {
            row_put(&mut bound, var, Value::Rel(edge_id));
        }
        let expected = eval(ctx, expr, &bound, None)?;
        let actual = record
            .properties
            .get(key)
            .map(value_from_property)
            .unwrap_or(Value::Null);
        if !property_matches(&actual, &expected) {
            return Ok(false);
        }
    }
    Ok(true)
}

struct Walk {
    end: VertexId,
    rels: Vec<EdgeId>,
    nodes: Vec<VertexId>,
}

fn expand_chains(
    ctx: &EvalCtx<'_>,
    element: &BoundElement,
    index: usize,
    current: VertexId,
    row: Row,
    path: Vec<Value>,
    path_var: Option<&str>,
    results: &mut Vec<Row>,
    end_hint: Option<&(String, u64)>,
    shortest_path: bool,
) -> Result<()> {
    if index == element.chains.len() {
        let mut finished = row;
        if let Some(path_var) = path_var {
            row_put(&mut finished, path_var, Value::Path(path));
        }
        results.push(finished);
        return Ok(());
    }
    let (rel, node) = &element.chains[index];
    let single = rel.min_hops == 1 && rel.max_hops == Some(1);
    // The optimizer marks a single variable-length chain whose only observed
    // property is `min(length(path))`, and supplies the terminal node's id.
    // Under that shape the shortest walk is the whole answer, so a BFS can
    // stop at the target instead of materialising every trail up to max_hops.
    let goal = if shortest_path && index == 0 && element.chains.len() == 1 {
        end_hint.and_then(|(variable, id)| {
            (node.var.as_deref() == Some(variable.as_str())).then_some(VertexId(*id))
        })
    } else {
        None
    };
    let walks: Vec<Walk> = if let Some(goal) = goal {
        match bfs_shortest(ctx, current, rel, goal, rel.max_hops.unwrap_or(8))? {
            Some(walk) => vec![walk],
            None => Vec::new(),
        }
    } else if single {
        single_step(ctx, current, rel)?
            .into_iter()
            .map(|(edge, target)| Walk {
                end: target,
                rels: vec![edge],
                nodes: vec![target],
            })
            .collect()
    } else {
        let mut walks = Vec::new();
        let mut rels = Vec::new();
        let mut nodes = Vec::new();
        var_length(ctx, current, rel, 0, &mut rels, &mut nodes, &mut walks)?;
        walks
    };

    for walk in walks {
        // Relationship property filters apply to every traversed edge.
        let mut ok = true;
        for edge in &walk.rels {
            if !rel_predicates(ctx, rel, *edge, &row)? {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }
        let mut next = row.clone();
        if let Some(var) = &rel.var {
            if single {
                row_put(&mut next, var, Value::Rel(walk.rels[0]));
            } else {
                row_put(
                    &mut next,
                    var,
                    Value::List(walk.rels.iter().map(|id| Value::Rel(*id)).collect()),
                );
            }
        }
        if let Some(var) = &node.var {
            if let Some(existing) = row_get(&next, var) {
                match existing {
                    Value::Node(id) if *id == walk.end => {}
                    _ => continue,
                }
            } else {
                row_put(&mut next, var, Value::Node(walk.end));
            }
        }
        if !node_predicates(ctx, node, walk.end, &next)? {
            continue;
        }
        let mut next_path = path.clone();
        for (edge, target) in walk.rels.iter().zip(walk.nodes.iter()) {
            next_path.push(Value::Rel(*edge));
            next_path.push(Value::Node(*target));
        }
        expand_chains(
            ctx,
            element,
            index + 1,
            walk.end,
            next,
            next_path,
            path_var,
            results,
            None,
            false,
        )?;
    }
    Ok(())
}

fn single_step(ctx: &EvalCtx<'_>, current: VertexId, rel: &BoundRel) -> Result<Vec<(EdgeId, VertexId)>> {
    let filter = EdgeFilter {
        types: if rel.types.is_empty() {
            None
        } else {
            Some(rel.types.clone())
        },
    };
    let mut steps = Vec::new();
    if matches!(rel.direction, Direction::Outgoing | Direction::Both) {
        for edge in ctx.storage.out_edges(current, filter.clone(), ctx.txn)? {
            steps.push((edge.id, edge.dst));
        }
    }
    if matches!(rel.direction, Direction::Incoming | Direction::Both) {
        for edge in ctx.storage.in_edges(current, filter.clone(), ctx.txn)? {
            steps.push((edge.id, edge.src));
        }
    }
    Ok(steps)
}

/// Shortest directed walk from `start` to `goal` of at most `max_hops` edges.
///
/// The caller has already proved that only the path's length is observed and
/// that every predicate is an `id` equality, so the first time BFS reaches
/// `goal` is the complete answer.  Visiting a vertex once is sufficient: any
/// shortest walk can be chosen simple, and revisiting a vertex cannot shorten
/// it.  This is what replaces the trail enumeration's exponential frontier on
/// high-degree starts.
fn bfs_shortest(
    ctx: &EvalCtx<'_>,
    start: VertexId,
    rel: &BoundRel,
    goal: VertexId,
    max_hops: i64,
) -> Result<Option<Walk>> {
    let max_hops = max_hops.max(1);
    let mut visited: HashSet<VertexId> = HashSet::new();
    visited.insert(start);
    let mut parent: HashMap<VertexId, (VertexId, EdgeId)> = HashMap::new();
    let mut frontier = vec![start];
    for _depth in 1..=max_hops {
        let mut next = Vec::new();
        for current in frontier {
            for (edge, target) in single_step(ctx, current, rel)? {
                if !visited.insert(target) {
                    continue;
                }
                parent.insert(target, (current, edge));
                if target == goal {
                    let mut rels = Vec::new();
                    let mut nodes = Vec::new();
                    let mut node = target;
                    while node != start {
                        let (previous, edge) = parent[&node];
                        rels.push(edge);
                        nodes.push(node);
                        node = previous;
                    }
                    rels.reverse();
                    nodes.reverse();
                    return Ok(Some(Walk { end: target, rels, nodes }));
                }
                next.push(target);
            }
        }
        if next.is_empty() {
            return Ok(None);
        }
        frontier = next;
    }
    Ok(None)
}

fn var_length(
    ctx: &EvalCtx<'_>,
    current: VertexId,
    rel: &BoundRel,
    depth: i64,
    rels: &mut Vec<EdgeId>,
    nodes: &mut Vec<VertexId>,
    out: &mut Vec<Walk>,
) -> Result<()> {
    if depth >= rel.min_hops {
        out.push(Walk {
            end: current,
            rels: rels.clone(),
            nodes: nodes.clone(),
        });
    }
    let limit = rel.max_hops.unwrap_or(8);
    if depth >= limit {
        return Ok(());
    }
    if out.len() > 100_000 {
        return Err(Error::Execute(
            "variable-length expansion exceeded the traversal budget".into(),
        ));
    }
    for (edge, target) in single_step(ctx, current, rel)? {
        if rels.contains(&edge) {
            continue;
        }
        rels.push(edge);
        nodes.push(target);
        var_length(ctx, target, rel, depth + 1, rels, nodes, out)?;
        rels.pop();
        nodes.pop();
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Unwind
// ---------------------------------------------------------------------------

fn unwind_stage(ctx: &EvalCtx<'_>, expr: &BExpr, var: &str, input: Vec<Row>) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        match eval(ctx, expr, &row, None)? {
            Value::List(items) => {
                for item in items {
                    let mut next = row.clone();
                    row_put(&mut next, var, item);
                    output.push(next);
                }
            }
            Value::Null => {}
            other => {
                return Err(Error::Execute(format!(
                    "UNWIND expects a list but received {other:?}"
                )))
            }
        }
    }
    Ok(output)
}

// ---------------------------------------------------------------------------
// Project / aggregate
// ---------------------------------------------------------------------------

fn project_stage(
    ctx: &EvalCtx<'_>,
    stage: &ProjectStage,
    input: Vec<Row>,
    columns: &mut Vec<String>,
) -> Result<Vec<Row>> {
    let has_aggregate = stage
        .items
        .iter()
        .any(|item| !item.star && item.expr.contains_aggregate());

    let star_keys: Vec<String> = input
        .first()
        .map(|row| row.iter().map(|(name, _)| name.clone()).collect())
        .unwrap_or_default();

    let mut produced: Vec<(Row, Vec<Value>)> = Vec::new();
    if !has_aggregate {
        for row in &input {
            let mut out: Row = Vec::new();
            for item in &stage.items {
                if item.star {
                    for (name, value) in row {
                        if row_get(&out, name).is_none() {
                            row_put(&mut out, name, value.clone());
                        }
                    }
                    continue;
                }
                let value = eval(ctx, &item.expr, row, None)?;
                row_put(&mut out, &item.alias, value);
            }
            if let Some(predicate) = &stage.where_clause {
                if !eval(ctx, predicate, &out, None)?.is_true() {
                    continue;
                }
            }
            let merged = merge_rows(row, &out);
            let keys = order_keys(ctx, &stage.order_by, &merged, None)?;
            produced.push((out, keys));
        }
        if stage.distinct {
            let mut seen = std::collections::HashSet::new();
            produced.retain(|(row, _)| {
                let key = row
                    .iter()
                    .map(|(_, value)| value.canonical_key())
                    .collect::<Vec<_>>()
                    .join("|");
                seen.insert(key)
            });
        }
    } else {
        let mut groups: Vec<(Vec<Value>, Vec<Row>)> = Vec::new();
        let mut index_of: HashMap<String, usize> = HashMap::new();
        for row in &input {
            let mut key_values = Vec::new();
            for item in &stage.items {
                if item.star || item.expr.contains_aggregate() {
                    continue;
                }
                key_values.push(eval(ctx, &item.expr, row, None)?);
            }
            let key = key_values
                .iter()
                .map(|value| value.canonical_key())
                .collect::<Vec<_>>()
                .join("|");
            match index_of.get(&key) {
                Some(index) => groups[*index].1.push(row.clone()),
                None => {
                    index_of.insert(key, groups.len());
                    groups.push((key_values, vec![row.clone()]));
                }
            }
        }
        // Aggregation without any grouping key over an empty input still emits
        // one row (COUNT -> 0, COLLECT -> []), matching Cypher semantics.
        let has_group_keys = stage
            .items
            .iter()
            .any(|item| !item.star && !item.expr.contains_aggregate());
        if groups.is_empty() && !has_group_keys {
            groups.push((Vec::new(), Vec::new()));
        }
        for (_, group_rows) in &groups {
            let representative = group_rows.first().cloned().unwrap_or_default();
            let representative = &representative;
            let mut out: Row = Vec::new();
            for item in &stage.items {
                if item.star {
                    for (name, value) in representative {
                        if row_get(&out, name).is_none() {
                            row_put(&mut out, name, value.clone());
                        }
                    }
                    continue;
                }
                let value = eval(ctx, &item.expr, representative, Some(group_rows))?;
                row_put(&mut out, &item.alias, value);
            }
            if let Some(predicate) = &stage.where_clause {
                if !eval(ctx, predicate, &out, None)?.is_true() {
                    continue;
                }
            }
            let merged = merge_rows(representative, &out);
            let keys = order_keys(ctx, &stage.order_by, &merged, Some(group_rows))?;
            produced.push((out, keys));
        }
    }

    // ORDER BY
    if !stage.order_by.is_empty() {
        let desc: Vec<bool> = stage.order_by.iter().map(|(_, desc)| *desc).collect();
        produced.sort_by(|a, b| compare_keys(&a.1, &b.1, &desc));
    }

    // SKIP / LIMIT
    let skip = match &stage.skip {
        Some(expr) => eval(ctx, expr, &Row::new(), None)?.as_i64().unwrap_or(0).max(0) as usize,
        None => 0,
    };
    let limit = match &stage.limit {
        Some(expr) => {
            let value = eval(ctx, expr, &Row::new(), None)?.as_i64().unwrap_or(0);
            Some(value.max(0) as usize)
        }
        None => None,
    };
    let mut rows: Vec<Row> = produced
        .into_iter()
        .skip(skip)
        .map(|(row, _)| row)
        .collect();
    if let Some(limit) = limit {
        rows.truncate(limit);
    }

    // The projection defines the output schema of the whole pipeline, so the
    // last Project stage always replaces any earlier column list.
    columns.clear();
    for item in &stage.items {
        if item.star {
            columns.extend(star_keys.iter().cloned());
        } else {
            columns.push(item.alias.clone());
        }
    }
    if columns.is_empty() {
        if let Some(first) = rows.first() {
            columns.extend(first.iter().map(|(name, _)| name.clone()));
        }
    }
    Ok(rows)
}

fn merge_rows(base: &Row, overrides: &Row) -> Row {
    let mut merged = base.clone();
    for (name, value) in overrides {
        row_put(&mut merged, name, value.clone());
    }
    merged
}

fn order_keys(
    ctx: &EvalCtx<'_>,
    order_by: &[(BExpr, bool)],
    row: &Row,
    group: Option<&[Row]>,
) -> Result<Vec<Value>> {
    order_by
        .iter()
        .map(|(expr, _)| eval(ctx, expr, row, group))
        .collect()
}

fn compare_keys(a: &[Value], b: &[Value], desc: &[bool]) -> std::cmp::Ordering {
    for (index, (left, right)) in a.iter().zip(b.iter()).enumerate() {
        let mut ordering = left.total_cmp(right);
        if desc.get(index).copied().unwrap_or(false) {
            ordering = ordering.reverse();
        }
        if ordering != std::cmp::Ordering::Equal {
            return ordering;
        }
    }
    std::cmp::Ordering::Equal
}

// ---------------------------------------------------------------------------
// DML
// ---------------------------------------------------------------------------

fn create_stage(
    ctx: &EvalCtx<'_>,
    parts: &[BoundPatternPart],
    input: Vec<Row>,
) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        let mut current_row = row;
        for part in parts {
            create_element(ctx, &part.element, &mut current_row)?;
        }
        output.push(current_row);
    }
    Ok(output)
}

fn create_element(ctx: &EvalCtx<'_>, element: &BoundElement, row: &mut Row) -> Result<VertexId> {
    let anchor_id = materialize_node(ctx, &element.node, row)?;
    let mut current = anchor_id;
    for (rel, node) in &element.chains {
        let target = materialize_node(ctx, node, row)?;
        let (src, dst) = match rel.direction {
            Direction::Incoming => (target, current),
            _ => (current, target),
        };
        let etype = rel.types.first().copied().unwrap_or(EdgeTypeId(0));
        let properties = evaluate_properties(ctx, &rel.properties, row)?;
        let edge_id = ctx.storage.insert_edge(
            EdgeRecord {
                id: EdgeId(0),
                src,
                dst,
                etype,
                properties,
            },
            ctx.txn,
        )?;
        if let Some(var) = &rel.var {
            row_put(row, var, Value::Rel(edge_id));
        }
        current = target;
    }
    Ok(current)
}

fn materialize_node(ctx: &EvalCtx<'_>, node: &BoundNode, row: &mut Row) -> Result<VertexId> {
    if let Some(var) = &node.var {
        if let Some(value) = row_get(row, var) {
            if let Value::Node(id) = value {
                return Ok(*id);
            }
            return Err(Error::Execute(format!(
                "variable `{var}` is bound to a non-node value in CREATE"
            )));
        }
    }
    let properties = evaluate_properties(ctx, &node.properties, row)?;
    let id = ctx.storage.insert_vertex(
        VertexRecord {
            id: VertexId(0),
            labels: node.labels.clone(),
            properties,
        },
        ctx.txn,
    )?;
    if let Some(var) = &node.var {
        row_put(row, var, Value::Node(id));
    }
    Ok(id)
}

fn evaluate_properties(
    ctx: &EvalCtx<'_>,
    properties: &[(PropertyKeyId, BExpr)],
    row: &Row,
) -> Result<PropertyMap> {
    let mut map = PropertyMap::new();
    for (key, expr) in properties {
        let value = eval(ctx, expr, row, None)?;
        map.insert(*key, value_to_property(&value));
    }
    Ok(map)
}

fn merge_stage(ctx: &EvalCtx<'_>, stage: &MergeStage, input: Vec<Row>) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        let matches = match_element(
            ctx,
            &stage.part.element,
            stage.part.path_var.as_deref(),
            &row,
            None,
            None,
            false,
        )?;
        if let Some(mut matched) = matches.into_iter().next() {
            apply_set_items(ctx, &stage.on_match, &mut matched)?;
            output.push(matched);
        } else {
            let mut created = row;
            create_element(ctx, &stage.part.element, &mut created)?;
            apply_set_items(ctx, &stage.on_create, &mut created)?;
            output.push(created);
        }
    }
    Ok(output)
}

fn set_stage(ctx: &EvalCtx<'_>, items: &[BoundSetItem], input: Vec<Row>) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for mut row in input {
        apply_set_items(ctx, items, &mut row)?;
        output.push(row);
    }
    Ok(output)
}

fn apply_set_items(ctx: &EvalCtx<'_>, items: &[BoundSetItem], row: &mut Row) -> Result<()> {
    for item in items {
        match item {
            BoundSetItem::Property {
                element,
                key,
                value,
            } => {
                let target = eval(ctx, element, row, None)?;
                let new_value = eval(ctx, value, row, None)?;
                update_property(ctx, &target, *key, new_value)?;
            }
            BoundSetItem::Variable { name, value } => {
                let target = eval(ctx, &BExpr::Var(name.clone()), row, None)?;
                let replacement = eval(ctx, value, row, None)?;
                if let Value::Map(entries) = replacement {
                    let map = entries
                        .into_iter()
                        .filter_map(|(key, value)| {
                            property_key_by_name(ctx, &key).map(|id| (id, value_to_property(&value)))
                        })
                        .collect();
                    replace_properties(ctx, &target, map)?;
                }
            }
            BoundSetItem::Merge { name, value } => {
                let target = eval(ctx, &BExpr::Var(name.clone()), row, None)?;
                if let Value::Map(entries) = eval(ctx, value, row, None)? {
                    for (key, value) in entries {
                        if let Some(id) = property_key_by_name(ctx, &key) {
                            update_property(ctx, &target, id, value)?;
                        }
                    }
                }
            }
            BoundSetItem::Labels { name, labels } => {
                let target = eval(ctx, &BExpr::Var(name.clone()), row, None)?;
                if let Value::Node(id) = target {
                    if let Some(mut record) = ctx.storage.get_vertex(id, ctx.txn)? {
                        for label in labels {
                            if !record.labels.contains(label) {
                                record.labels.push(*label);
                            }
                        }
                        ctx.storage.update_vertex(record, ctx.txn)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn property_key_by_name(ctx: &EvalCtx<'_>, name: &str) -> Option<PropertyKeyId> {
    ctx.schema.property_key_by_name(name)
}

fn update_property(
    ctx: &EvalCtx<'_>,
    target: &Value,
    key: PropertyKeyId,
    value: Value,
) -> Result<()> {
    match target {
        Value::Node(id) => {
            if let Some(mut record) = ctx.storage.get_vertex(*id, ctx.txn)? {
                if value.is_null() {
                    record.properties.remove(&key);
                } else {
                    record.properties.insert(key, value_to_property(&value));
                }
                ctx.storage.update_vertex(record, ctx.txn)?;
            }
            Ok(())
        }
        Value::Rel(id) => {
            if let Some(mut record) = ctx.storage.get_edge(*id, ctx.txn)? {
                if value.is_null() {
                    record.properties.remove(&key);
                } else {
                    record.properties.insert(key, value_to_property(&value));
                }
                ctx.storage.update_edge(record, ctx.txn)?;
            }
            Ok(())
        }
        Value::Null => Ok(()),
        other => Err(Error::Execute(format!(
            "cannot set a property on {other:?}"
        ))),
    }
}

fn replace_properties(ctx: &EvalCtx<'_>, target: &Value, properties: PropertyMap) -> Result<()> {
    match target {
        Value::Node(id) => {
            if let Some(mut record) = ctx.storage.get_vertex(*id, ctx.txn)? {
                record.properties = properties;
                ctx.storage.update_vertex(record, ctx.txn)?;
            }
        }
        Value::Rel(id) => {
            if let Some(mut record) = ctx.storage.get_edge(*id, ctx.txn)? {
                record.properties = properties;
                ctx.storage.update_edge(record, ctx.txn)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn remove_stage(ctx: &EvalCtx<'_>, items: &[BoundRemoveItem], input: Vec<Row>) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        for item in items {
            match item {
                BoundRemoveItem::Property { element, key } => {
                    let target = eval(ctx, element, &row, None)?;
                    update_property(ctx, &target, *key, Value::Null)?;
                }
                BoundRemoveItem::Labels { name, labels } => {
                    let target = eval(ctx, &BExpr::Var(name.clone()), &row, None)?;
                    if let Value::Node(id) = target {
                        if let Some(mut record) = ctx.storage.get_vertex(id, ctx.txn)? {
                            record.labels.retain(|label| !labels.contains(label));
                            ctx.storage.update_vertex(record, ctx.txn)?;
                        }
                    }
                }
            }
        }
        output.push(row);
    }
    Ok(output)
}

fn delete_stage(
    ctx: &EvalCtx<'_>,
    detach: bool,
    exprs: &[BExpr],
    input: Vec<Row>,
) -> Result<Vec<Row>> {
    let mut output = Vec::new();
    for row in input {
        for expr in exprs {
            match eval(ctx, expr, &row, None)? {
                Value::Node(id) => {
                    if detach {
                        for edge in ctx
                            .storage
                            .out_edges(id, EdgeFilter::default(), ctx.txn)?
                            .collect::<Vec<_>>()
                        {
                            ctx.storage.delete_edge(edge.id, ctx.txn)?;
                        }
                        for edge in ctx
                            .storage
                            .in_edges(id, EdgeFilter::default(), ctx.txn)?
                            .collect::<Vec<_>>()
                        {
                            ctx.storage.delete_edge(edge.id, ctx.txn)?;
                        }
                    }
                    ctx.storage.delete_vertex(id, ctx.txn)?;
                }
                Value::Rel(id) => {
                    ctx.storage.delete_edge(id, ctx.txn)?;
                }
                Value::Null => {}
                other => {
                    return Err(Error::Execute(format!("DELETE cannot remove {other:?}")))
                }
            }
        }
        output.push(row);
    }
    Ok(output)
}

// ---------------------------------------------------------------------------
// CALL
// ---------------------------------------------------------------------------

fn call_stage(
    ctx: &EvalCtx<'_>,
    algorithms: &Arc<AlgorithmRegistry>,
    call: &CallStage,
    input: Vec<Row>,
    columns: &mut Vec<String>,
) -> Result<Vec<Row>> {
    let mut params = std::collections::BTreeMap::new();
    for (name, expr) in &call.args {
        let value = eval(ctx, expr, &Row::new(), None)?;
        params.insert(name.clone(), value_to_property(&value));
    }
    let direction = match call.name.as_str() {
        "wcc" => ProjectionDirection::Both,
        _ => ProjectionDirection::Outgoing,
    };
    let spec = ProjectionSpec {
        vertex_labels: None,
        edge_types: None,
        direction,
        vertex_properties: None,
    };
    let projection = CsrProjection::build(ctx.storage, ctx.txn, &spec)?;
    let algorithm = algorithms
        .algorithms
        .get(&call.name)
        .ok_or_else(|| Error::Execute(format!("algorithm `{}` is not registered", call.name)))?
        .clone();
    let result = algorithm.execute(&projection, &AlgorithmArgs { params })?;

    let output_columns: Vec<String> = if call.yields.is_empty() && !call.yield_star {
        result.schema.clone()
    } else {
        call.yields.iter().map(|(_, alias)| alias.clone()).collect()
    };
    let mut algorithm_rows = Vec::new();
    for row in &result.rows {
        let mut bound = Row::new();
        for (index, column) in result.schema.iter().enumerate() {
            if index < row.len() {
                row_put(&mut bound, column, result_to_value(&row[index]));
            }
        }
        if call.yields.is_empty() {
            algorithm_rows.push(bound);
        } else {
            let mut mapped = Row::new();
            for (field, alias) in &call.yields {
                let value = row_get(&bound, field).cloned().ok_or_else(|| {
                    Error::Execute(format!(
                        "YIELD field `{field}` is not produced by algorithm `{}`",
                        call.name
                    ))
                })?;
                row_put(&mut mapped, alias, value);
            }
            algorithm_rows.push(mapped);
        }
    }

    if columns.is_empty() {
        columns.extend(output_columns);
    }

    // Cross join with the input rows; the common case is a single empty row.
    let mut output = Vec::new();
    for input_row in &input {
        for algorithm_row in &algorithm_rows {
            output.push(merge_rows(input_row, algorithm_row));
        }
    }
    Ok(output)
}

fn result_to_value(value: &ResultValue) -> Value {
    match value {
        ResultValue::Null => Value::Null,
        ResultValue::Bool(value) => Value::Bool(*value),
        ResultValue::Int(value) => Value::Int(*value),
        ResultValue::Float(value) => Value::Float(*value),
        ResultValue::Text(value) => Value::Text(value.clone()),
        ResultValue::VertexRef(id) => Value::Node(*id),
        ResultValue::EdgeRef(id) => Value::Rel(*id),
        ResultValue::Property(property) => value_from_property(property),
        ResultValue::List(items) => Value::List(items.iter().map(result_to_value).collect()),
        ResultValue::Map(entries) => Value::Map(
            entries
                .iter()
                .map(|(key, value)| (format!("{key:?}"), result_to_value(value)))
                .collect(),
        ),
        ResultValue::Path(path) => {
            Value::Path(path.objects.iter().map(result_to_value).collect())
        }
        ResultValue::PropertyElement(element) => Value::Map(vec![
            ("key".into(), Value::Text(element.key.clone())),
            ("value".into(), result_to_value(&element.value)),
            ("element".into(), result_to_value(&element.element)),
        ]),
        ResultValue::Set(items) => Value::List(items.iter().map(result_to_value).collect()),
    }
}

// ---------------------------------------------------------------------------
// Expression evaluation
// ---------------------------------------------------------------------------

pub fn eval(
    ctx: &EvalCtx<'_>,
    expr: &BExpr,
    row: &Row,
    group: Option<&[Row]>,
) -> Result<Value> {
    Ok(match expr {
        BExpr::Const(value) => value.clone(),
        BExpr::Var(name) => {
            if name == "*" {
                return Err(Error::Execute("`*` cannot be evaluated as a value".into()));
            }
            row_get(row, name).cloned().unwrap_or(Value::Null)
        }
        BExpr::Param(name) => {
            return Err(Error::Unsupported(format!(
                "query parameter `${name}` is not bound in the selected profile"
            )))
        }
        BExpr::IdOf(inner) => match eval(ctx, inner, row, group)? {
            Value::Node(id) => Value::Int(id.0 as i64),
            Value::Rel(id) => Value::Int(id.0 as i64),
            _ => Value::Null,
        },
        BExpr::LabelsOf(inner) => match eval(ctx, inner, row, group)? {
            Value::Node(id) => match ctx.storage.get_vertex(id, ctx.txn)? {
                Some(record) => Value::List(
                    record
                        .labels
                        .iter()
                        .filter_map(|label| ctx.schema.label_name(*label))
                        .map(|name| Value::Text(name.to_string()))
                        .collect(),
                ),
                None => Value::Null,
            },
            _ => Value::Null,
        },
        BExpr::Prop(source, key) => {
            let source = eval(ctx, source, row, group)?;
            property_of(ctx, &source, *key)?
        }
        BExpr::Neg(inner) => match eval(ctx, inner, row, group)? {
            Value::Int(value) => Value::Int(-value),
            Value::Float(value) => Value::Float(-value),
            _ => Value::Null,
        },
        BExpr::Add(lhs, rhs) => add(eval(ctx, lhs, row, group)?, eval(ctx, rhs, row, group)?)?,
        BExpr::Sub(lhs, rhs) => {
            numeric_binary(eval(ctx, lhs, row, group)?, eval(ctx, rhs, row, group)?, |a, b| a - b, |a, b| a - b)
        }
        BExpr::Mul(lhs, rhs) => {
            numeric_binary(eval(ctx, lhs, row, group)?, eval(ctx, rhs, row, group)?, |a, b| a * b, |a, b| a * b)
        }
        BExpr::Div(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            match (&left, &right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        return Err(Error::Execute("division by zero".into()));
                    }
                    Value::Int(a / b)
                }
                (Value::Null, _) | (_, Value::Null) => Value::Null,
                _ => match (left.as_f64(), right.as_f64()) {
                    (Some(a), Some(b)) if b != 0.0 => Value::Float(a / b),
                    _ => Value::Null,
                },
            }
        }
        BExpr::Mod(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            match (&left, &right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        return Err(Error::Execute("modulo by zero".into()));
                    }
                    Value::Int(a % b)
                }
                (Value::Null, _) | (_, Value::Null) => Value::Null,
                _ => match (left.as_f64(), right.as_f64()) {
                    (Some(a), Some(b)) if b != 0.0 => Value::Float(a % b),
                    _ => Value::Null,
                },
            }
        }
        BExpr::Pow(lhs, rhs) => match (eval(ctx, lhs, row, group)?, eval(ctx, rhs, row, group)?) {
            (Value::Null, _) | (_, Value::Null) => Value::Null,
            (a, b) => match (a.as_f64(), b.as_f64()) {
                (Some(a), Some(b)) => Value::Float(a.powf(b)),
                _ => Value::Null,
            },
        },
        BExpr::Eq(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |ordering| ordering == std::cmp::Ordering::Equal)?,
        BExpr::Neq(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |ordering| ordering != std::cmp::Ordering::Equal)?,
        BExpr::Lt(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |o| o == std::cmp::Ordering::Less)?,
        BExpr::Lte(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |o| o != std::cmp::Ordering::Greater)?,
        BExpr::Gt(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |o| o == std::cmp::Ordering::Greater)?,
        BExpr::Gte(lhs, rhs) => compare(ctx, lhs, rhs, row, group, |o| o != std::cmp::Ordering::Less)?,
        BExpr::And(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            three_valued_and(&left, &right)
        }
        BExpr::Or(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            three_valued_or(&left, &right)
        }
        BExpr::Xor(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            match (truth(&left), truth(&right)) {
                (Some(a), Some(b)) => Value::Bool(a ^ b),
                _ => Value::Null,
            }
        }
        BExpr::Not(inner) => match truth(&eval(ctx, inner, row, group)?) {
            Some(value) => Value::Bool(!value),
            None => Value::Null,
        },
        BExpr::IsNull(inner) => Value::Bool(eval(ctx, inner, row, group)?.is_null()),
        BExpr::IsNotNull(inner) => Value::Bool(!eval(ctx, inner, row, group)?.is_null()),
        BExpr::In(lhs, rhs) => {
            let left = eval(ctx, lhs, row, group)?;
            let right = eval(ctx, rhs, row, group)?;
            if left.is_null() {
                Value::Null
            } else {
                match right {
                    Value::List(items) => {
                        Value::Bool(items.iter().any(|item| item.equals(&left)))
                    }
                    Value::Null => Value::Null,
                    other => {
                        return Err(Error::Execute(format!("IN expects a list, got {other:?}")))
                    }
                }
            }
        }
        BExpr::StartsWith(lhs, rhs) => string_predicate(ctx, lhs, rhs, row, group, |a, b| a.starts_with(b))?,
        BExpr::EndsWith(lhs, rhs) => string_predicate(ctx, lhs, rhs, row, group, |a, b| a.ends_with(b))?,
        BExpr::Contains(lhs, rhs) => string_predicate(ctx, lhs, rhs, row, group, |a, b| a.contains(b))?,
        BExpr::ListIndex(list, index) => {
            let list = eval(ctx, list, row, group)?;
            let index = eval(ctx, index, row, group)?;
            match (list, index.as_i64()) {
                (Value::List(items), Some(index)) => {
                    let len = items.len() as i64;
                    let normalized = if index < 0 { len + index } else { index };
                    if normalized < 0 || normalized >= len {
                        Value::Null
                    } else {
                        items[normalized as usize].clone()
                    }
                }
                _ => Value::Null,
            }
        }
        BExpr::ListSlice { list, start, end } => {
            let list = eval(ctx, list, row, group)?;
            let start = match start {
                Some(expr) => eval(ctx, expr, row, group)?.as_i64(),
                None => Some(0),
            };
            let end = match end {
                Some(expr) => eval(ctx, expr, row, group)?.as_i64(),
                None => None,
            };
            match list {
                Value::List(items) => {
                    let len = items.len() as i64;
                    let normalize = |index: i64| {
                        let value = if index < 0 { len + index } else { index };
                        value.clamp(0, len) as usize
                    };
                    let from = start.map(normalize).unwrap_or(0);
                    let to = end.map(normalize).unwrap_or(len as usize);
                    if from > to {
                        Value::List(Vec::new())
                    } else {
                        Value::List(items[from..to].to_vec())
                    }
                }
                _ => Value::Null,
            }
        }
        BExpr::List(items) => Value::List(
            items
                .iter()
                .map(|item| eval(ctx, item, row, group))
                .collect::<Result<Vec<_>>>()?,
        ),
        BExpr::Map(entries) => Value::Map(
            entries
                .iter()
                .map(|(key, value)| Ok((key.clone(), eval(ctx, value, row, group)?)))
                .collect::<Result<Vec<_>>>()?,
        ),
        BExpr::Function { name, args } => {
            let values = args
                .iter()
                .map(|arg| eval(ctx, arg, row, group))
                .collect::<Result<Vec<_>>>()?;
            call_function(ctx, name, &values)?
        }
        BExpr::Case {
            test,
            whens,
            else_branch,
        } => {
            let mut result = None;
            if let Some(test) = test {
                let test_value = eval(ctx, test, row, group)?;
                for (when, then) in whens {
                    let when_value = eval(ctx, when, row, group)?;
                    if when_value.equals(&test_value) {
                        result = Some(eval(ctx, then, row, group)?);
                        break;
                    }
                }
            } else {
                for (when, then) in whens {
                    if eval(ctx, when, row, group)?.is_true() {
                        result = Some(eval(ctx, then, row, group)?);
                        break;
                    }
                }
            }
            match result {
                Some(value) => value,
                None => match else_branch {
                    Some(expr) => eval(ctx, expr, row, group)?,
                    None => Value::Null,
                },
            }
        }
        BExpr::ListComprehension {
            var,
            list,
            filter,
            map,
        } => match eval(ctx, list, row, group)? {
            Value::List(items) => {
                let mut out = Vec::new();
                for item in items {
                    let mut scoped = row.clone();
                    row_put(&mut scoped, var, item.clone());
                    if let Some(filter) = filter {
                        if !eval(ctx, filter, &scoped, group)?.is_true() {
                            continue;
                        }
                    }
                    out.push(match map {
                        Some(map) => eval(ctx, map, &scoped, group)?,
                        None => item,
                    });
                }
                Value::List(out)
            }
            Value::Null => Value::Null,
            other => return Err(Error::Execute(format!("comprehension over {other:?}"))),
        },
        BExpr::Quantifier {
            kind,
            var,
            list,
            predicate,
        } => match eval(ctx, list, row, group)? {
            Value::List(items) => {
                let mut truths = Vec::new();
                for item in items {
                    let item_value = item.clone();
                    let mut scoped = row.clone();
                    row_put(&mut scoped, var, item);
                    let value = match predicate {
                        Some(predicate) => truth(&eval(ctx, predicate, &scoped, group)?),
                        None => truth(&item_value),
                    };
                    truths.push(value.unwrap_or(false));
                }
                Value::Bool(match kind {
                    QuantifierKind::All => truths.iter().all(|value| *value),
                    QuantifierKind::Any => truths.iter().any(|value| *value),
                    QuantifierKind::None => !truths.iter().any(|value| *value),
                    QuantifierKind::Single => truths.iter().filter(|value| **value).count() == 1,
                })
            }
            _ => Value::Null,
        },
        BExpr::LabelTest(inner, labels) => match eval(ctx, inner, row, group)? {
            Value::Node(id) => match ctx.storage.get_vertex(id, ctx.txn)? {
                Some(record) => {
                    Value::Bool(labels.iter().all(|label| record.labels.contains(label)))
                }
                None => Value::Bool(false),
            },
            _ => Value::Null,
        },
        BExpr::Aggregate { kind, distinct, arg } => {
            let group = group.ok_or_else(|| {
                Error::Execute("aggregate used outside of an aggregation context".into())
            })?;
            eval_aggregate(ctx, *kind, *distinct, arg.as_deref(), group)?
        }
    })
}

fn property_of(ctx: &EvalCtx<'_>, source: &Value, key: PropertyKeyId) -> Result<Value> {
    match source {
        Value::Node(id) => Ok(ctx
            .storage
            .get_vertex(*id, ctx.txn)?
            .and_then(|record| record.properties.get(&key).map(value_from_property))
            .unwrap_or(Value::Null)),
        Value::Rel(id) => Ok(ctx
            .storage
            .get_edge(*id, ctx.txn)?
            .and_then(|record| record.properties.get(&key).map(value_from_property))
            .unwrap_or(Value::Null)),
        Value::Date(year, month, day) => Ok(match ctx.schema.property_key_name(key) {
            Some("year") => Value::Int(*year as i64),
            Some("month") => Value::Int(*month as i64),
            Some("day") => Value::Int(*day as i64),
            _ => Value::Null,
        }),
        Value::Time(hour, minute, second) => Ok(match ctx.schema.property_key_name(key) {
            Some("hour") => Value::Int(*hour as i64),
            Some("minute") => Value::Int(*minute as i64),
            Some("second") => Value::Int(*second as i64),
            _ => Value::Null,
        }),
        Value::Map(entries) => Ok(ctx
            .schema
            .property_key_name(key)
            .and_then(|name| entries.iter().find(|(key, _)| key == name).map(|(_, value)| value.clone()))
            .unwrap_or(Value::Null)),
        Value::Null => Ok(Value::Null),
        _ => Ok(Value::Null),
    }
}

fn truth(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(value) => Some(*value),
        Value::Null => None,
        _ => None,
    }
}

fn three_valued_and(left: &Value, right: &Value) -> Value {
    match (truth(left), truth(right)) {
        (Some(false), _) | (_, Some(false)) => Value::Bool(false),
        (Some(true), Some(true)) => Value::Bool(true),
        _ => Value::Null,
    }
}

fn three_valued_or(left: &Value, right: &Value) -> Value {
    match (truth(left), truth(right)) {
        (Some(true), _) | (_, Some(true)) => Value::Bool(true),
        (Some(false), Some(false)) => Value::Bool(false),
        _ => Value::Null,
    }
}

fn compare<F>(
    ctx: &EvalCtx<'_>,
    lhs: &BExpr,
    rhs: &BExpr,
    row: &Row,
    group: Option<&[Row]>,
    predicate: F,
) -> Result<Value>
where
    F: Fn(std::cmp::Ordering) -> bool,
{
    let left = eval(ctx, lhs, row, group)?;
    let right = eval(ctx, rhs, row, group)?;
    if left.is_null() || right.is_null() {
        return Ok(Value::Null);
    }
    Ok(Value::Bool(predicate(left.total_cmp(&right))))
}

fn string_predicate<F>(
    ctx: &EvalCtx<'_>,
    lhs: &BExpr,
    rhs: &BExpr,
    row: &Row,
    group: Option<&[Row]>,
    predicate: F,
) -> Result<Value>
where
    F: Fn(&str, &str) -> bool,
{
    let left = eval(ctx, lhs, row, group)?;
    let right = eval(ctx, rhs, row, group)?;
    match (&left, &right) {
        (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
        (Value::Text(a), Value::Text(b)) => Ok(Value::Bool(predicate(a, b))),
        _ => Ok(Value::Null),
    }
}

fn add(left: Value, right: Value) -> Result<Value> {
    match (&left, &right) {
        (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
        (Value::Float(_), Value::Float(_)) | (Value::Int(_), Value::Float(_)) | (Value::Float(_), Value::Int(_)) => {
            Ok(Value::Float(left.as_f64().unwrap_or(0.0) + right.as_f64().unwrap_or(0.0)))
        }
        (Value::List(a), Value::List(b)) => {
            let mut items = a.clone();
            items.extend(b.clone());
            Ok(Value::List(items))
        }
        (Value::List(a), other) => {
            let mut items = a.clone();
            items.push(other.clone());
            Ok(Value::List(items))
        }
        (other, Value::List(b)) => {
            let mut items = vec![other.clone()];
            items.extend(b.clone());
            Ok(Value::List(items))
        }
        (Value::Text(_), _) | (_, Value::Text(_)) => {
            Ok(Value::Text(format!("{}{}", text_of(&left), text_of(&right))))
        }
        _ => Ok(Value::Null),
    }
}

fn text_of(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::Text(value) => value.clone(),
        other => format!("{other:?}"),
    }
}

fn numeric_binary(
    left: Value,
    right: Value,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> Value {
    match (&left, &right) {
        (Value::Null, _) | (_, Value::Null) => Value::Null,
        (Value::Int(a), Value::Int(b)) => Value::Int(int_op(*a, *b)),
        _ => match (left.as_f64(), right.as_f64()) {
            (Some(a), Some(b)) => Value::Float(float_op(a, b)),
            _ => Value::Null,
        },
    }
}

fn eval_aggregate(
    ctx: &EvalCtx<'_>,
    kind: AggregateKind,
    distinct: bool,
    arg: Option<&BExpr>,
    group: &[Row],
) -> Result<Value> {
    if kind == AggregateKind::CountStar {
        return Ok(Value::Int(group.len() as i64));
    }
    let mut values = Vec::new();
    for row in group {
        let value = match arg {
            Some(expr) => eval(ctx, expr, row, None)?,
            None => Value::Null,
        };
        values.push(value);
    }
    if distinct {
        let mut seen = std::collections::HashSet::new();
        values.retain(|value| seen.insert(value.canonical_key()));
    }
    match kind {
        AggregateKind::CountStar => unreachable!(),
        AggregateKind::Count => Ok(Value::Int(
            values.iter().filter(|value| !value.is_null()).count() as i64,
        )),
        AggregateKind::Sum => {
            let mut int_total = 0i64;
            let mut float_total = 0.0f64;
            let mut is_float = false;
            for value in &values {
                match value {
                    Value::Int(v) => {
                        int_total += v;
                        float_total += *v as f64;
                    }
                    Value::Float(v) => {
                        is_float = true;
                        float_total += v;
                    }
                    _ => {}
                }
            }
            if is_float {
                Ok(Value::Float(float_total))
            } else {
                Ok(Value::Int(int_total))
            }
        }
        AggregateKind::Avg => {
            let mut total = 0.0f64;
            let mut count = 0usize;
            for value in &values {
                if let Some(number) = value.as_f64() {
                    total += number;
                    count += 1;
                }
            }
            if count == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Float(total / count as f64))
            }
        }
        AggregateKind::Min => Ok(values
            .into_iter()
            .filter(|value| !value.is_null())
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(Value::Null)),
        AggregateKind::Max => Ok(values
            .into_iter()
            .filter(|value| !value.is_null())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(Value::Null)),
        AggregateKind::Collect => Ok(Value::List(
            values.into_iter().filter(|value| !value.is_null()).collect(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_valued_logic() {
        assert_eq!(three_valued_and(&Value::Bool(false), &Value::Null), Value::Bool(false));
        assert_eq!(three_valued_or(&Value::Bool(true), &Value::Null), Value::Bool(true));
        assert_eq!(three_valued_and(&Value::Bool(true), &Value::Null), Value::Null);
        assert_eq!(truth(&Value::Null), None);
    }

    #[test]
    fn add_overloads() {
        assert_eq!(add(Value::Int(1), Value::Int(2)).unwrap(), Value::Int(3));
        assert_eq!(
            add(Value::Text("A".into()), Value::Text("!".into())).unwrap(),
            Value::Text("A!".into())
        );
        assert_eq!(
            add(Value::List(vec![Value::Int(1)]), Value::Int(2)).unwrap(),
            Value::List(vec![Value::Int(1), Value::Int(2)])
        );
    }

    #[test]
    fn result_round_trip_keeps_vertex_refs() {
        assert_eq!(
            result_to_value(&ResultValue::VertexRef(VertexId(7))),
            Value::Node(VertexId(7))
        );
    }
}
