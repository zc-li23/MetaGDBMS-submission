//! Push-based physical plan execution.
//!
//! The design vector selects `execution.model = "push_based"`: [`run`] walks the
//! physical tree and every operator pushes rows into a [`Sink`].  Operators that
//! need the whole input — aggregation, duplicate elimination, sorting, cross
//! joins, `MERGE`, and DML — buffer or cache internally and then push their
//! results downstream, which is the usual shape of a push engine.
//!
//! Rows are `Vec<Value>` addressed by a column-name schema; a variable reference
//! resolves through that schema, so `WITH`/`RETURN` genuinely rebind the scope
//! rather than carrying hidden state.
//!
//! ## Result encoding
//!
//! `ResultValue` is the framework's output type, so [`Value::into_result`] is
//! the only conversion at the boundary.  Temporal values become their ISO-8601
//! text because the result model has no date/time variant; every other value
//! keeps its type.

use std::collections::HashMap;

use metagdb_core::common::ids::{EdgeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeRecord, StorageEngine, VertexFilter, VertexRecord,
};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{ExpandDirection, PhysicalPlan, SortOrder};
use metagdb_core::l4_compute::engine::{ExecContext, Executor, ResultStream};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};
use metagdb_core::l4_compute::olap::AlgorithmRegistry;
use metagdb_core::{Error, Result};

use super::physical_ops::{group_key_name, PhysicalOp, WriteOp};
use super::projection::CsrProjection;
use super::temporal::{Temporal, TemporalKind};
use super::value::{EvalRow, PathValue, Value};

/// Executor stage: runs the candidate's physical operator tree.
#[derive(Debug, Default, Clone, Copy)]
pub struct MetagdbExecutor;

impl Executor for MetagdbExecutor {
    fn execute(&self, pp: &PhysicalPlan, ctx: &ExecContext) -> Result<ResultStream> {
        let root = pp.downcast_ref::<PhysicalOp>().ok_or_else(|| {
            Error::Execute("the physical plan is not a candidate operator tree".into())
        })?;
        let eval = EvalContext {
            storage: ctx.storage.as_ref(),
            schema: ctx.schema.as_ref(),
            algorithms: ctx.algorithms.as_ref(),
            txn: ctx.txn.clone(),
            parameters: HashMap::new(),
        };
        let schema = schema_of(root, &eval)?;
        let mut sink = Collector {
            schema: schema.clone(),
            rows: Vec::new(),
        };
        run(root, &eval, &mut sink)?;
        Ok(ResultStream {
            schema,
            rows: sink
                .rows
                .into_iter()
                .map(|row| row.into_iter().map(Value::into_result).collect())
                .collect(),
        })
    }
}

/// Everything an operator needs besides its own payload.
pub struct EvalContext<'a> {
    pub storage: &'a dyn StorageEngine,
    pub schema: &'a dyn GraphSchema,
    pub algorithms: &'a AlgorithmRegistry,
    pub txn: TxnContext,
    pub parameters: HashMap<String, Value>,
}

impl EvalContext<'_> {
    fn key_name(&self, key: PropertyKeyId) -> Option<String> {
        self.schema.property_key_name(key).map(str::to_string)
    }

    fn full_scan(&self) -> Result<Vec<VertexRecord>> {
        Ok(self
            .storage
            .scan_vertices(VertexFilter::default(), &self.txn)?
            .collect())
    }
}

/// Consumer of pushed rows.
pub trait Sink {
    fn push(&mut self, row: EvalRow) -> Result<()>;
}

/// Terminal sink that materializes the result stream.
struct Collector {
    schema: Vec<String>,
    rows: Vec<EvalRow>,
}

impl Sink for Collector {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        debug_assert_eq!(row.len(), self.schema.len(), "row/column mismatch");
        self.rows.push(row);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Schemas
// ---------------------------------------------------------------------------

/// Output columns of an operator, computed before any row is produced.
pub fn schema_of(op: &PhysicalOp, ctx: &EvalContext<'_>) -> Result<Vec<String>> {
    Ok(match op {
        PhysicalOp::SingleRow | PhysicalOp::GraphProjection { .. } => Vec::new(),
        PhysicalOp::VertexScan { var, .. }
        | PhysicalOp::EdgeScan { var, .. }
        | PhysicalOp::VertexById { var, .. } => vec![var.clone()],
        PhysicalOp::Expand { input, to, edge, .. } => {
            let mut schema = schema_of(input, ctx)?;
            schema.push(to.clone());
            if let Some(edge) = edge {
                schema.push(edge.clone());
            }
            schema
        }
        PhysicalOp::VarExpand {
            input,
            to,
            edge,
            path,
            ..
        } => {
            let mut schema = schema_of(input, ctx)?;
            schema.push(to.clone());
            if let Some(edge) = edge {
                schema.push(edge.clone());
            }
            if let Some(path) = path {
                schema.push(path.clone());
            }
            schema
        }
        PhysicalOp::Filter { input, .. }
        | PhysicalOp::Distinct { input }
        | PhysicalOp::Sort { input, .. }
        | PhysicalOp::Limit { input, .. } => schema_of(input, ctx)?,
        PhysicalOp::Project { items, .. } => items.iter().map(|(_, alias)| alias.clone()).collect(),
        PhysicalOp::Aggregate {
            group_by, aggs, ..
        } => {
            let mut schema: Vec<String> = (0..group_by.len()).map(group_key_name).collect();
            schema.extend(aggs.iter().map(|(_, alias)| alias.clone()));
            schema
        }
        PhysicalOp::Unwind { input, alias, .. } => {
            let mut schema = schema_of(input, ctx)?;
            schema.push(alias.clone());
            schema
        }
        PhysicalOp::Union { inputs, .. } => {
            let first = inputs
                .first()
                .ok_or_else(|| Error::Execute("UNION has no branches".into()))?;
            schema_of(first, ctx)?
        }
        PhysicalOp::Join { left, right, .. } => {
            let mut schema = schema_of(left, ctx)?;
            schema.extend(schema_of(right, ctx)?);
            schema
        }
        PhysicalOp::Mutate {
            input,
            writes,
            projection,
        } => {
            let mut schema = schema_of(input, ctx)?;
            schema.extend(write_bindings(writes));
            if projection.is_empty() {
                schema
            } else {
                projection.iter().map(|(_, alias)| alias.clone()).collect()
            }
        }
        PhysicalOp::Merge { input, probe, .. } => {
            let probe_schema = schema_of(probe, ctx)?;
            if probe_schema.is_empty() {
                schema_of(input, ctx)?
            } else {
                probe_schema
            }
        }
        PhysicalOp::CallAlgorithm { name, .. } => algorithm_columns(name)?,
        PhysicalOp::Unsupported { reason } => {
            return Err(Error::Unsupported(reason.clone()));
        }
    })
}

/// Columns created by a write list, in execution order.
fn write_bindings(writes: &[WriteOp]) -> Vec<String> {
    let mut out = Vec::new();
    for write in writes {
        match write {
            WriteOp::CreateVertex { var, .. } => out.push(var.clone()),
            WriteOp::CreateEdge { var: Some(var), .. } => out.push(var.clone()),
            _ => {}
        }
    }
    out
}

/// Result columns of a registered algorithm, taken from the same signature the
/// planner types YIELD against.
pub fn algorithm_columns(name: &str) -> Result<Vec<String>> {
    crate::compute::planner::algorithm_signature(name)
        .map(|signature| signature.columns.iter().map(|s| s.to_string()).collect())
        .ok_or_else(|| Error::Execute(format!("algorithm '{name}' has no column signature")))
}

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

/// Run one operator, pushing its rows into `sink`.
pub fn run(op: &PhysicalOp, ctx: &EvalContext<'_>, sink: &mut dyn Sink) -> Result<()> {
    match op {
        PhysicalOp::SingleRow => sink.push(Vec::new()),
        PhysicalOp::GraphProjection { .. } => sink.push(Vec::new()),
        PhysicalOp::VertexById { id, .. } => {
            if let Some(record) = ctx.storage.get_vertex(VertexId(*id), &ctx.txn)? {
                sink.push(vec![Value::Vertex(record.id)])?;
            }
            Ok(())
        }
        PhysicalOp::VertexScan { var, label } => {
            let filter = VertexFilter {
                labels: label.map(|id| vec![id]),
            };
            for record in ctx.storage.scan_vertices(filter, &ctx.txn)? {
                sink.push(vec![Value::Vertex(record.id)])?;
            }
            let _ = var;
            Ok(())
        }
        PhysicalOp::EdgeScan { etype, .. } => {
            let filter = EdgeFilter {
                types: etype.map(|id| vec![id]),
            };
            for record in ctx.full_scan()? {
                for edge in ctx.storage.out_edges(record.id, filter.clone(), &ctx.txn)? {
                    sink.push(vec![Value::Edge(edge.id)])?;
                }
            }
            Ok(())
        }
        PhysicalOp::Expand {
            input,
            from,
            to,
            edge,
            direction,
            edge_types,
        } => {
            let input_schema = schema_of(input, ctx)?;
            let from_slot = slot(&input_schema, from)?;
            let filter = EdgeFilter {
                types: edge_types.clone(),
            };
            let mut expand = ExpandSink {
                sink,
                from_slot,
                filter,
                direction: *direction,
                to: to.clone(),
                edge: edge.clone(),
                ctx,
            };
            run(input, ctx, &mut expand)
        }
        PhysicalOp::VarExpand {
            input,
            from,
            to: _,
            edge,
            path,
            direction,
            edge_types,
            min,
            max,
        } => {
            let input_schema = schema_of(input, ctx)?;
            let from_slot = slot(&input_schema, from)?;
            let filter = EdgeFilter {
                types: edge_types.clone(),
            };
            let mut expand = VarExpandSink {
                sink,
                from_slot,
                filter,
                direction: *direction,
                edge: edge.clone(),
                path: path.clone(),
                min: *min,
                max: *max,
                ctx,
            };
            run(input, ctx, &mut expand)
        }
        PhysicalOp::Filter { predicate, input } => {
            let input_schema = schema_of(input, ctx)?;
            let mut filter = FilterSink {
                sink,
                schema: input_schema,
                predicate,
                ctx,
            };
            run(input, ctx, &mut filter)
        }
        PhysicalOp::Project { items, input } => {
            let input_schema = schema_of(input, ctx)?;
            let mut project = ProjectSink {
                sink,
                schema: input_schema,
                items,
                ctx,
            };
            run(input, ctx, &mut project)
        }
        PhysicalOp::Aggregate {
            group_by,
            aggs,
            input,
        } => {
            let input_schema = schema_of(input, ctx)?;
            let mut aggregate = AggregateSink::new(sink, input_schema, group_by, aggs, ctx);
            run(input, ctx, &mut aggregate)?;
            aggregate.finish()
        }
        PhysicalOp::Distinct { input } => {
            let mut distinct = DistinctSink {
                sink,
                seen: Vec::new(),
            };
            run(input, ctx, &mut distinct)
        }
        PhysicalOp::Sort { keys, input } => {
            let input_schema = schema_of(input, ctx)?;
            let mut sort = SortSink {
                sink,
                schema: input_schema,
                keys,
                rows: Vec::new(),
                ctx,
            };
            run(input, ctx, &mut sort)?;
            sort.finish()
        }
        PhysicalOp::Limit {
            limit,
            offset,
            input,
        } => {
            let mut limit_sink = LimitSink {
                sink,
                seen: 0,
                limit: *limit,
                offset: *offset,
            };
            run(input, ctx, &mut limit_sink)
        }
        PhysicalOp::Unwind { list, alias, input } => {
            let input_schema = schema_of(input, ctx)?;
            let _ = alias;
            let mut unwind = UnwindSink {
                sink,
                schema: input_schema,
                list,
                ctx,
            };
            run(input, ctx, &mut unwind)
        }
        PhysicalOp::Union { all, inputs } => {
            let mut dedup: Vec<String> = Vec::new();
            for input in inputs {
                let mut branch = UnionSink {
                    sink,
                    all: *all,
                    seen: &mut dedup,
                };
                run(input, ctx, &mut branch)?;
            }
            Ok(())
        }
        PhysicalOp::Join {
            kind,
            on,
            left,
            right,
        } => {
            let left_schema = schema_of(left, ctx)?;
            let right_schema = schema_of(right, ctx)?;
            // Materialize the right side once; it is typically a constant.
            let right_rows = {
                let mut collect = Collector {
                    schema: right_schema.clone(),
                    rows: Vec::new(),
                };
                run(right, ctx, &mut collect)?;
                collect.rows
            };
            match kind {
                metagdb_core::l4_compute::algebra::JoinKind::Cross if on.is_empty() => {
                    let _ = (&left_schema, &right_schema);
                    let mut join = JoinSink {
                        sink,
                        right_rows,
                        on,
                    };
                    run(left, ctx, &mut join)
                }
                other => Err(Error::Unsupported(format!(
                    "{other:?} joins are outside the requested scope"
                ))),
            }
        }
        PhysicalOp::Mutate {
            input,
            writes,
            projection,
        } => {
            let input_schema = schema_of(input, ctx)?;
            let mut mutate = MutateSink {
                sink,
                schema: input_schema,
                writes,
                projection,
                ctx,
            };
            run(input, ctx, &mut mutate)
        }
        PhysicalOp::Merge {
            input,
            probe,
            create,
            actions,
        } => {
            if !matches!(input.as_ref(), PhysicalOp::SingleRow) {
                return Err(Error::Unsupported(
                    "MERGE over a pattern that reads rows from an earlier clause is outside \
                     the requested scope"
                        .into(),
                ));
            }
            let probe_schema = schema_of(probe, ctx)?;
            let input_schema = schema_of(input, ctx)?;
            let bindings = write_bindings(create);
            let mut merge = MergeSink {
                sink,
                probe,
                probe_schema,
                input_schema,
                create,
                actions,
                bindings,
                ctx,
            };
            run(input, ctx, &mut merge)
        }
        PhysicalOp::CallAlgorithm {
            name, args, spec, ..
        } => {
            let projection = CsrProjection::build(ctx.storage, &ctx.txn, spec)?;
            let algorithm = ctx
                .algorithms
                .algorithms
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, algorithm)| algorithm)
                .ok_or_else(|| Error::Execute(format!("algorithm '{name}' is not registered")))?;
            let stream = algorithm.execute(&projection, args)?;
            for row in stream.rows {
                sink.push(row.iter().map(Value::from_result).collect())?;
            }
            Ok(())
        }
        PhysicalOp::Unsupported { reason } => Err(Error::Unsupported(reason.clone())),
    }
}

// ---------------------------------------------------------------------------
// Operator sinks
// ---------------------------------------------------------------------------

struct ExpandSink<'a, 's> {
    sink: &'s mut dyn Sink,
    from_slot: usize,
    filter: EdgeFilter,
    direction: ExpandDirection,
    to: String,
    edge: Option<String>,
    ctx: &'a EvalContext<'a>,
}

impl Sink for ExpandSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let Some(source) = row[self.from_slot].as_vertex() else {
            return Err(Error::Execute(
                "a traversal source must be a vertex; null or non-vertex values cannot expand"
                    .into(),
            ));
        };
        let outgoing = matches!(
            self.direction,
            ExpandDirection::Outgoing | ExpandDirection::Both
        );
        let incoming = matches!(
            self.direction,
            ExpandDirection::Incoming | ExpandDirection::Both
        );
        let mut edges: Vec<EdgeRecord> = Vec::new();
        if outgoing {
            edges.extend(self.ctx.storage.out_edges(source, self.filter.clone(), &self.ctx.txn)?);
        }
        if incoming {
            edges.extend(self.ctx.storage.in_edges(source, self.filter.clone(), &self.ctx.txn)?);
        }
        for edge in edges {
            let target = if outgoing && edge.src == source {
                edge.dst
            } else {
                edge.src
            };
            let mut next = row.clone();
            next.push(Value::Vertex(target));
            if self.edge.is_some() {
                next.push(Value::Edge(edge.id));
            }
            self.sink.push(next)?;
        }
        let _ = (&self.to, &self.edge);
        Ok(())
    }
}

struct VarExpandSink<'a, 's> {
    sink: &'s mut dyn Sink,
    from_slot: usize,
    filter: EdgeFilter,
    direction: ExpandDirection,
    edge: Option<String>,
    path: Option<String>,
    min: u32,
    max: Option<u32>,
    ctx: &'a EvalContext<'a>,
}

impl Sink for VarExpandSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let Some(source) = row[self.from_slot].as_vertex() else {
            return Err(Error::Execute(
                "a traversal source must be a vertex; null or non-vertex values cannot expand"
                    .into(),
            ));
        };
        let mut frontier: Vec<PathValue> = vec![PathValue {
            nodes: vec![source],
            edges: Vec::new(),
        }];
        let ceiling = self.max.unwrap_or(u32::MAX);
        let mut depth = 0u32;
        loop {
            for path in &frontier {
                if depth >= self.min {
                    let mut next = row.clone();
                    let last = *path.nodes.last().expect("a path always has a start");
                    next.push(Value::Vertex(last));
                    if self.edge.is_some() {
                        next.push(Value::List(
                            path.edges.iter().map(|id| Value::Edge(*id)).collect(),
                        ));
                    }
                    if self.path.is_some() {
                        next.push(Value::Path(path.clone()));
                    }
                    self.sink.push(next)?;
                }
            }
            if depth >= ceiling {
                return Ok(());
            }
            let mut next_frontier = Vec::new();
            for path in &frontier {
                let last = *path.nodes.last().expect("a path always has a start");
                let outgoing = matches!(
                    self.direction,
                    ExpandDirection::Outgoing | ExpandDirection::Both
                );
                let incoming = matches!(
                    self.direction,
                    ExpandDirection::Incoming | ExpandDirection::Both
                );
                let mut edges: Vec<EdgeRecord> = Vec::new();
                if outgoing {
                    edges.extend(
                        self.ctx
                            .storage
                            .out_edges(last, self.filter.clone(), &self.ctx.txn)?,
                    );
                }
                if incoming {
                    edges.extend(
                        self.ctx
                            .storage
                            .in_edges(last, self.filter.clone(), &self.ctx.txn)?,
                    );
                }
                for edge in edges {
                    // Cypher paths do not repeat a relationship.
                    if path.edges.contains(&edge.id) {
                        continue;
                    }
                    let target = if outgoing && edge.src == last {
                        edge.dst
                    } else {
                        edge.src
                    };
                    let mut extended = path.clone();
                    extended.nodes.push(target);
                    extended.edges.push(edge.id);
                    next_frontier.push(extended);
                }
            }
            if next_frontier.is_empty() {
                return Ok(());
            }
            frontier = next_frontier;
            depth += 1;
        }
    }
}

struct FilterSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    predicate: &'a Expr,
    ctx: &'a EvalContext<'a>,
}

impl Sink for FilterSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let keep = eval(self.predicate, &row, &self.schema, self.ctx)?;
        if keep.is_true() {
            self.sink.push(row)?;
        }
        Ok(())
    }
}

struct ProjectSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    items: &'a [(Expr, String)],
    ctx: &'a EvalContext<'a>,
}

impl Sink for ProjectSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let mut out = Vec::with_capacity(self.items.len());
        for (expr, _) in self.items {
            out.push(eval(expr, &row, &self.schema, self.ctx)?);
        }
        self.sink.push(out)
    }
}

struct AggregateSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    group_by: &'a [Expr],
    aggs: &'a [(AggregateExpr, String)],
    ctx: &'a EvalContext<'a>,
    groups: Vec<(String, Vec<Value>, Vec<Accumulator>)>,
    index: HashMap<String, usize>,
}

impl<'a, 's> AggregateSink<'a, 's> {
    fn new(
        sink: &'s mut dyn Sink,
        schema: Vec<String>,
        group_by: &'a [Expr],
        aggs: &'a [(AggregateExpr, String)],
        ctx: &'a EvalContext<'a>,
    ) -> Self {
        Self {
            sink,
            schema,
            group_by,
            aggs,
            ctx,
            groups: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn finish(&mut self) -> Result<()> {
        if self.groups.is_empty() && self.group_by.is_empty() {
            // An aggregation with no grouping keys always produces one row.
            let values = self
                .aggs
                .iter()
                .map(|(aggregate, _)| Accumulator::new(aggregate)?.value())
                .collect::<Result<Vec<_>>>()?;
            return self.sink.push(values);
        }
        for (_, keys, accumulators) in std::mem::take(&mut self.groups) {
            let mut row = keys;
            for accumulator in accumulators {
                row.push(accumulator.value()?);
            }
            self.sink.push(row)?;
        }
        Ok(())
    }
}

impl Sink for AggregateSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let mut keys = Vec::with_capacity(self.group_by.len());
        for expr in self.group_by {
            keys.push(eval(expr, &row, &self.schema, self.ctx)?);
        }
        let key = keys
            .iter()
            .map(Value::group_key)
            .collect::<Vec<_>>()
            .join("|");
        let index = match self.index.get(&key) {
            Some(index) => *index,
            None => {
                let accumulators = self
                    .aggs
                    .iter()
                    .map(|(aggregate, _)| Accumulator::new(aggregate))
                    .collect::<Result<Vec<_>>>()?;
                self.groups.push((key.clone(), keys, accumulators));
                let index = self.groups.len() - 1;
                self.index.insert(key, index);
                index
            }
        };
        let (_, _, accumulators) = &mut self.groups[index];
        for (slot, (aggregate, _)) in accumulators.iter_mut().zip(self.aggs.iter()) {
            slot.accumulate(aggregate, &row, &self.schema, self.ctx)?;
        }
        Ok(())
    }
}

struct DistinctSink<'s> {
    sink: &'s mut dyn Sink,
    seen: Vec<String>,
}

impl Sink for DistinctSink<'_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let key = row.iter().map(Value::group_key).collect::<Vec<_>>().join("|");
        if !self.seen.contains(&key) {
            self.seen.push(key);
            self.sink.push(row)?;
        }
        Ok(())
    }
}

struct UnionSink<'s, 'a> {
    sink: &'s mut dyn Sink,
    all: bool,
    seen: &'a mut Vec<String>,
}

impl Sink for UnionSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        if self.all {
            return self.sink.push(row);
        }
        let key = row.iter().map(Value::group_key).collect::<Vec<_>>().join("|");
        if !self.seen.contains(&key) {
            self.seen.push(key);
            self.sink.push(row)?;
        }
        Ok(())
    }
}

struct SortSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    keys: &'a [(Expr, SortOrder)],
    rows: Vec<EvalRow>,
    ctx: &'a EvalContext<'a>,
}

impl<'a> SortSink<'a, '_> {
    fn finish(&mut self) -> Result<()> {
        let mut keyed = Vec::with_capacity(self.rows.len());
        for row in std::mem::take(&mut self.rows) {
            let mut sort_keys = Vec::with_capacity(self.keys.len());
            for (expr, _) in self.keys {
                sort_keys.push(eval(expr, &row, &self.schema, self.ctx)?);
            }
            keyed.push((sort_keys, row));
        }
        // A stable sort keeps the input order for equal keys, which is what
        // makes `ORDER BY a.name` deterministic on equal ages.
        keyed.sort_by(|left, right| compare_sort_keys(&left.0, &right.0, self.keys));
        for (_, row) in keyed {
            self.sink.push(row)?;
        }
        Ok(())
    }
}

impl Sink for SortSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        self.rows.push(row);
        Ok(())
    }
}

/// Nulls sort last in both directions, so an ordering never depends on whether
/// a property happens to be missing.
fn compare_sort_keys(
    left: &[Value],
    right: &[Value],
    keys: &[(Expr, SortOrder)],
) -> std::cmp::Ordering {
    for (index, (_, order)) in keys.iter().enumerate() {
        let ordering = match (&left[index], &right[index]) {
            (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
            (Value::Null, _) => std::cmp::Ordering::Greater,
            (_, Value::Null) => std::cmp::Ordering::Less,
            (a, b) => match a.compare(b) {
                Some(ordering) => ordering,
                // Not comparable: keep the input order.
                None => std::cmp::Ordering::Equal,
            },
        };
        let ordering = match order {
            SortOrder::Asc => ordering,
            SortOrder::Desc => ordering.reverse(),
        };
        // Nulls stay last regardless of the requested direction.
        let ordering = match (&left[index], &right[index]) {
            (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
            (Value::Null, _) => std::cmp::Ordering::Greater,
            (_, Value::Null) => std::cmp::Ordering::Less,
            _ => ordering,
        };
        if ordering != std::cmp::Ordering::Equal {
            return ordering;
        }
    }
    std::cmp::Ordering::Equal
}

struct LimitSink<'s> {
    sink: &'s mut dyn Sink,
    seen: usize,
    limit: usize,
    offset: usize,
}

impl Sink for LimitSink<'_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        if self.seen >= self.offset && self.seen < self.offset.saturating_add(self.limit) {
            self.sink.push(row)?;
        }
        self.seen += 1;
        Ok(())
    }
}

struct UnwindSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    list: &'a Expr,
    ctx: &'a EvalContext<'a>,
}

impl Sink for UnwindSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        match eval(self.list, &row, &self.schema, self.ctx)? {
            Value::Null => Ok(()),
            Value::List(items) => {
                for item in items {
                    let mut next = row.clone();
                    next.push(item);
                    self.sink.push(next)?;
                }
                Ok(())
            }
            other => Err(Error::Execute(format!(
                "UNWIND requires a list, got {}",
                describe(&other)
            ))),
        }
    }
}

struct JoinSink<'a, 's> {
    sink: &'s mut dyn Sink,
    right_rows: Vec<EvalRow>,
    on: &'a [(Expr, Expr)],
}

impl Sink for JoinSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        for right in &self.right_rows {
            let mut combined = row.clone();
            combined.extend(right.iter().cloned());
            self.sink.push(combined)?;
        }
        let _ = self.on;
        Ok(())
    }
}

struct MutateSink<'a, 's> {
    sink: &'s mut dyn Sink,
    schema: Vec<String>,
    writes: &'a [WriteOp],
    projection: &'a [(Expr, String)],
    ctx: &'a EvalContext<'a>,
}

impl Sink for MutateSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let mut extended = row.clone();
        apply_writes(self.writes, &mut extended, &self.schema, self.ctx)?;
        if self.projection.is_empty() {
            return self.sink.push(extended);
        }
        let mut bindings = self.schema.clone();
        bindings.extend(write_bindings(self.writes));
        let mut out = Vec::with_capacity(self.projection.len());
        for (expr, _) in self.projection {
            out.push(eval(expr, &extended, &bindings, self.ctx)?);
        }
        self.sink.push(out)
    }
}

struct MergeSink<'a, 's> {
    sink: &'s mut dyn Sink,
    probe: &'a PhysicalOp,
    probe_schema: Vec<String>,
    input_schema: Vec<String>,
    create: &'a [WriteOp],
    actions: &'a [(bool, Vec<WriteOp>)],
    bindings: Vec<String>,
    ctx: &'a EvalContext<'a>,
}

impl Sink for MergeSink<'_, '_> {
    fn push(&mut self, row: EvalRow) -> Result<()> {
        let probed = {
            let mut collect = Collector {
                schema: self.probe_schema.clone(),
                rows: Vec::new(),
            };
            run(self.probe, self.ctx, &mut collect)?;
            collect.rows
        };
        if let Some(mut current) = probed.into_iter().next() {
            for (on_match, writes) in self.actions {
                if *on_match {
                    apply_writes(writes, &mut current, &self.probe_schema, self.ctx)?;
                }
            }
            return self.sink.push(current);
        }

        // Nothing matched: create the pattern, then report the row in the same
        // column order the probe would have produced.
        let mut created = row.clone();
        apply_writes(self.create, &mut created, &self.input_schema, self.ctx)?;
        let mut extended_schema = self.input_schema.clone();
        extended_schema.extend(self.bindings.clone());
        for (on_match, writes) in self.actions {
            if !*on_match {
                apply_writes(writes, &mut created, &extended_schema, self.ctx)?;
            }
        }
        let mut aligned = Vec::with_capacity(self.probe_schema.len());
        for column in &self.probe_schema {
            let index = extended_schema
                .iter()
                .position(|name| name == column)
                .ok_or_else(|| {
                    Error::Execute(format!("MERGE did not bind the pattern variable '{column}'"))
                })?;
            aligned.push(created.get(index).cloned().unwrap_or(Value::Null));
        }
        self.sink.push(aligned)
    }
}

// ---------------------------------------------------------------------------
// Writes
// ---------------------------------------------------------------------------

/// Apply a write list to one row, extending the row with newly bound variables.
fn apply_writes(
    writes: &[WriteOp],
    row: &mut EvalRow,
    schema: &[String],
    ctx: &EvalContext<'_>,
) -> Result<()> {
    // Each write may bind a variable the next write reads (`CREATE
    // (a)-[:T]->(:Person {…})` reads the anonymous target when it creates the
    // edge), so the schema grows alongside the row.
    let mut current: Vec<String> = schema.to_vec();
    for write in writes {
        match write {
            WriteOp::CreateVertex {
                var,
                labels,
                properties,
            } => {
                let mut map = PropertyMap::new();
                for (key, expr) in properties {
                    let value = eval(expr, row, &current, ctx)?;
                    map.insert(*key, to_property(&value)?);
                }
                let record = VertexRecord {
                    id: VertexId(0),
                    labels: labels.clone(),
                    properties: map,
                };
                let id = ctx.storage.insert_vertex(record, &ctx.txn)?;
                row.push(Value::Vertex(id));
                current.push(var.clone());
            }
            WriteOp::CreateEdge {
                from,
                to,
                etype,
                properties,
                var,
                reversed,
            } => {
                let (src, dst) = if *reversed {
                    (to.as_str(), from.as_str())
                } else {
                    (from.as_str(), to.as_str())
                };
                let src = vertex_of(row, &current, src)?;
                let dst = vertex_of(row, &current, dst)?;
                let mut map = PropertyMap::new();
                for (key, expr) in properties {
                    let value = eval(expr, row, &current, ctx)?;
                    map.insert(*key, to_property(&value)?);
                }
                let record = EdgeRecord {
                    id: EdgeId(0),
                    src,
                    dst,
                    etype: *etype,
                    properties: map,
                };
                let id = ctx.storage.insert_edge(record, &ctx.txn)?;
                if let Some(var) = var {
                    row.push(Value::Edge(id));
                    current.push(var.clone());
                }
            }
            WriteOp::SetProperty { target, key, value } => {
                let element = eval(target, row, &current, ctx)?;
                let value = eval(value, row, &current, ctx)?;
                let name = ctx.key_name(*key);
                set_property(&element, name.as_deref(), *key, &value, ctx)?;
            }
            WriteOp::SetLabels { target, labels } => {
                let element = eval(target, row, &current, ctx)?;
                let Some(id) = element.as_vertex() else {
                    return Err(Error::Execute("labels can only be set on a vertex".into()));
                };
                let mut record = ctx
                    .storage
                    .get_vertex(id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                for label in labels {
                    if !record.labels.contains(label) {
                        record.labels.push(*label);
                    }
                }
                ctx.storage.update_vertex(record, &ctx.txn)?;
            }
            WriteOp::RemoveProperty { target, key } => {
                let element = eval(target, row, &current, ctx)?;
                if let Some(edge) = element.as_edge() {
                    let mut record = ctx
                        .storage
                        .get_edge(edge, &ctx.txn)?
                        .ok_or_else(|| Error::NotFound(format!("edge {}", edge.0)))?;
                    record.properties.remove(key);
                    ctx.storage.update_edge(record, &ctx.txn)?;
                    continue;
                }
                let Some(id) = element.as_vertex() else {
                    return Err(Error::Execute(
                        "properties can only be removed from a vertex or an edge".into(),
                    ));
                };
                let mut record = ctx
                    .storage
                    .get_vertex(id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                record.properties.remove(key);
                ctx.storage.update_vertex(record, &ctx.txn)?;
            }
            WriteOp::RemoveLabels { target, labels } => {
                let element = eval(target, row, &current, ctx)?;
                let Some(id) = element.as_vertex() else {
                    return Err(Error::Execute("labels can only be removed from a vertex".into()));
                };
                let mut record = ctx
                    .storage
                    .get_vertex(id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                record.labels.retain(|label| !labels.contains(label));
                ctx.storage.update_vertex(record, &ctx.txn)?;
            }
            WriteOp::Delete { target, detach } => {
                let element = eval(target, row, &current, ctx)?;
                if let Some(id) = element.as_vertex() {
                    if *detach {
                        let mut incident: Vec<EdgeId> = Vec::new();
                        for edge in ctx
                            .storage
                            .out_edges(id, EdgeFilter::default(), &ctx.txn)?
                        {
                            incident.push(edge.id);
                        }
                        for edge in ctx.storage.in_edges(id, EdgeFilter::default(), &ctx.txn)? {
                            incident.push(edge.id);
                        }
                        incident.sort_unstable_by_key(|edge| edge.0);
                        incident.dedup();
                        for edge in incident {
                            ctx.storage.delete_edge(edge, &ctx.txn)?;
                        }
                    }
                    ctx.storage.delete_vertex(id, &ctx.txn)?;
                    continue;
                }
                let Some(edge) = element.as_edge() else {
                    return Err(Error::Execute(
                        "DELETE requires a vertex or a relationship; a null value deletes nothing"
                            .into(),
                    ));
                };
                ctx.storage.delete_edge(edge, &ctx.txn)?;
            }
        }
    }
    Ok(())
}

/// The two pseudo-keys the planner uses for whole-map assignment.
const KEY_REPLACE_ALL: &str = "*";
const KEY_MERGE: &str = "+";

fn set_property(
    element: &Value,
    name: Option<&str>,
    key: PropertyKeyId,
    value: &Value,
    ctx: &EvalContext<'_>,
) -> Result<()> {
    if name == Some(KEY_REPLACE_ALL) || name == Some(KEY_MERGE) {
        let Value::Map(entries) = value else {
            return Err(Error::Execute(
                "replacing or merging properties requires a map value".into(),
            ));
        };
        let mut map = PropertyMap::new();
        for (entry, entry_value) in entries {
            let key_id = ctx
                .schema
                .property_key_by_name(entry)
                .ok_or_else(|| Error::Execute(format!("unknown property key '{entry}'")))?;
            map.insert(key_id, to_property(entry_value)?);
        }
        if let Some(id) = element.as_vertex() {
            let mut record = ctx
                .storage
                .get_vertex(id, &ctx.txn)?
                .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
            if name == Some(KEY_MERGE) {
                for (key, value) in map {
                    record.properties.insert(key, value);
                }
            } else {
                record.properties = map;
            }
            return ctx.storage.update_vertex(record, &ctx.txn);
        }
        let Some(edge) = element.as_edge() else {
            return Err(Error::Execute(
                "properties can only be assigned on a vertex or an edge".into(),
            ));
        };
        let mut record = ctx
            .storage
            .get_edge(edge, &ctx.txn)?
            .ok_or_else(|| Error::NotFound(format!("edge {}", edge.0)))?;
        if name == Some(KEY_MERGE) {
            for (key, value) in map {
                record.properties.insert(key, value);
            }
        } else {
            record.properties = map;
        }
        return ctx.storage.update_edge(record, &ctx.txn);
    }

    let property = to_property(value)?;
    if let Some(id) = element.as_vertex() {
        let mut record = ctx
            .storage
            .get_vertex(id, &ctx.txn)?
            .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
        record.properties.insert(key, property);
        return ctx.storage.update_vertex(record, &ctx.txn);
    }
    let Some(edge) = element.as_edge() else {
        return Err(Error::Execute(
            "properties can only be assigned on a vertex or an edge".into(),
        ));
    };
    let mut record = ctx
        .storage
        .get_edge(edge, &ctx.txn)?
        .ok_or_else(|| Error::NotFound(format!("edge {}", edge.0)))?;
    record.properties.insert(key, property);
    ctx.storage.update_edge(record, &ctx.txn)
}

fn vertex_of(row: &EvalRow, schema: &[String], name: &str) -> Result<VertexId> {
    let index = slot(schema, name)?;
    row[index]
        .as_vertex()
        .ok_or_else(|| Error::Execute(format!("'{name}' is not a vertex")))
}

/// Convert an evaluated value into a stored property.
///
/// Lists and maps are not part of the design vector's property set, so they are
/// refused rather than flattened into a debug string.
fn to_property(value: &Value) -> Result<PropertyValue> {
    Ok(match value {
        Value::Null => PropertyValue::Null,
        Value::Bool(v) => PropertyValue::Bool(*v),
        Value::Int(v) => PropertyValue::Int(*v),
        Value::Float(v) => PropertyValue::Float(*v),
        Value::Text(v) => PropertyValue::Text(v.clone()),
        Value::Temporal(temporal) => PropertyValue::Text(temporal.text.clone()),
        other => {
            return Err(Error::Unsupported(format!(
                "{} values cannot be stored as properties",
                describe(other)
            )))
        }
    })
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

fn slot(schema: &[String], name: &str) -> Result<usize> {
    schema
        .iter()
        .position(|column| column == name)
        .ok_or_else(|| Error::Execute(format!("variable '{name}' is not bound in this row")))
}

fn describe(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Int(_) => "integer",
        Value::Float(_) => "float",
        Value::Text(_) => "string",
        Value::List(_) => "list",
        Value::Map(_) => "map",
        Value::Vertex(_) => "vertex",
        Value::Edge(_) => "relationship",
        Value::Path(_) => "path",
        Value::Temporal(_) => "temporal",
    }
}

impl Value {
    /// Convert a framework result cell back into an evaluation value.
    pub fn from_result(value: &metagdb_core::common::property::ResultValue) -> Self {
        use metagdb_core::common::property::ResultValue;
        match value {
            ResultValue::Null => Value::Null,
            ResultValue::Bool(v) => Value::Bool(*v),
            ResultValue::Int(v) => Value::Int(*v),
            ResultValue::Float(v) => Value::Float(*v),
            ResultValue::Text(v) => Value::Text(v.clone()),
            ResultValue::VertexRef(v) => Value::Vertex(*v),
            ResultValue::EdgeRef(v) => Value::Edge(*v),
            ResultValue::Property(v) => Value::from_property(v),
            ResultValue::List(items) => {
                Value::List(items.iter().map(Value::from_result).collect())
            }
            ResultValue::Set(items) => {
                Value::List(items.iter().map(Value::from_result).collect())
            }
            ResultValue::Map(entries) => Value::Map(
                entries
                    .iter()
                    .map(|(key, value)| (render_cell_key(key), Value::from_result(value)))
                    .collect(),
            ),
            ResultValue::Path(path) => {
                let mut nodes = Vec::new();
                let mut edges = Vec::new();
                for object in &path.objects {
                    match object {
                        ResultValue::VertexRef(id) => nodes.push(*id),
                        ResultValue::EdgeRef(id) => edges.push(*id),
                        _ => {}
                    }
                }
                Value::Path(PathValue { nodes, edges })
            }
            ResultValue::PropertyElement(element) => Value::Map(vec![(
                element.key.clone(),
                Value::from_result(&element.value),
            )]),
        }
    }
}

fn render_cell_key(value: &metagdb_core::common::property::ResultValue) -> String {
    match value {
        metagdb_core::common::property::ResultValue::Text(text) => text.clone(),
        other => format!("{other:?}"),
    }
}

/// Evaluate one expression against a row.
pub fn eval(expr: &Expr, row: &EvalRow, schema: &[String], ctx: &EvalContext<'_>) -> Result<Value> {
    Ok(match expr {
        Expr::Int(v) => Value::Int(*v),
        Expr::Float(v) => Value::Float(*v),
        Expr::String(v) => Value::Text(v.clone()),
        Expr::Bool(v) => Value::Bool(*v),
        Expr::Null => Value::Null,
        Expr::VarRef { name } => row[slot(schema, name)?].clone(),
        Expr::PropRef { source, key } => {
            let subject = eval(source, row, schema, ctx)?;
            property_of(&subject, *key, ctx)?
        }
        Expr::Neg(inner) => match eval(inner, row, schema, ctx)? {
            Value::Null => Value::Null,
            Value::Int(v) => Value::Int(-v),
            Value::Float(v) => Value::Float(-v),
            other => {
                return Err(Error::Execute(format!(
                    "cannot negate a {}",
                    describe(&other)
                )))
            }
        },
        Expr::Add(left, right) => arithmetic(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '+')?,
        Expr::Sub(left, right) => arithmetic(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '-')?,
        Expr::Mul(left, right) => arithmetic(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '*')?,
        Expr::Div(left, right) => arithmetic(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '/')?,
        Expr::Mod(left, right) => arithmetic(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '%')?,
        Expr::Eq(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '=')?,
        Expr::Neq(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, 'n')?,
        Expr::Lt(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '<')?,
        Expr::Gt(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, '>')?,
        Expr::Lte(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, 'l')?,
        Expr::Gte(left, right) => compare(eval(left, row, schema, ctx)?, eval(right, row, schema, ctx)?, 'g')?,
        Expr::IsNull(inner) => Value::Bool(eval(inner, row, schema, ctx)?.is_null()),
        Expr::IsNotNull(inner) => Value::Bool(!eval(inner, row, schema, ctx)?.is_null()),
        Expr::And(left, right) => {
            let left = eval(left, row, schema, ctx)?;
            if left == Value::Bool(false) {
                Value::Bool(false)
            } else {
                let right = eval(right, row, schema, ctx)?;
                match (&left, &right) {
                    (Value::Bool(true), Value::Bool(v)) => Value::Bool(*v),
                    (Value::Bool(true), Value::Null) => Value::Null,
                    (Value::Null, Value::Bool(false)) => Value::Bool(false),
                    (_, Value::Null) => Value::Null,
                    (_, Value::Bool(v)) => Value::Bool(*v),
                    _ => {
                        return Err(Error::Execute(
                            "AND requires boolean operands".into(),
                        ))
                    }
                }
            }
        }
        Expr::Or(left, right) => {
            let left = eval(left, row, schema, ctx)?;
            if left == Value::Bool(true) {
                Value::Bool(true)
            } else {
                let right = eval(right, row, schema, ctx)?;
                match (&left, &right) {
                    (Value::Bool(false), Value::Bool(v)) => Value::Bool(*v),
                    (Value::Bool(false), Value::Null) => Value::Null,
                    (Value::Null, Value::Bool(true)) => Value::Bool(true),
                    (_, Value::Null) => Value::Null,
                    (_, Value::Bool(v)) => Value::Bool(*v),
                    _ => return Err(Error::Execute("OR requires boolean operands".into())),
                }
            }
        }
        Expr::Not(inner) => match eval(inner, row, schema, ctx)? {
            Value::Bool(v) => Value::Bool(!v),
            Value::Null => Value::Null,
            _ => return Err(Error::Execute("NOT requires a boolean operand".into())),
        },
        Expr::StartsWith(left, right) => string_predicate(
            eval(left, row, schema, ctx)?,
            eval(right, row, schema, ctx)?,
            |text, prefix| text.starts_with(prefix),
        )?,
        Expr::EndsWith(left, right) => string_predicate(
            eval(left, row, schema, ctx)?,
            eval(right, row, schema, ctx)?,
            |text, suffix| text.ends_with(suffix),
        )?,
        Expr::Contains(left, right) => string_predicate(
            eval(left, row, schema, ctx)?,
            eval(right, row, schema, ctx)?,
            |text, needle| text.contains(needle),
        )?,
        Expr::Concat(items) => {
            let mut out = String::new();
            for item in items {
                match eval(item, row, schema, ctx)? {
                    Value::Null => return Ok(Value::Null),
                    Value::Text(text) => out.push_str(&text),
                    other => out.push_str(&render(&other)),
                }
            }
            Value::Text(out)
        }
        Expr::Case {
            cases,
            else_branch,
        } => {
            let mut result = None;
            for (condition, then) in cases {
                if eval(condition, row, schema, ctx)?.is_true() {
                    result = Some(eval(then, row, schema, ctx)?);
                    break;
                }
            }
            match result {
                Some(value) => value,
                None => match else_branch {
                    Some(else_branch) => eval(else_branch, row, schema, ctx)?,
                    None => Value::Null,
                },
            }
        }
        Expr::Coalesce(items) => {
            let mut result = Value::Null;
            for item in items {
                let value = eval(item, row, schema, ctx)?;
                if !value.is_null() {
                    result = value;
                    break;
                }
            }
            result
        }
        Expr::List(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(eval(item, row, schema, ctx)?);
            }
            Value::List(out)
        }
        Expr::ListIndex { list, index } => {
            let list = eval(list, row, schema, ctx)?;
            let index = eval(index, row, schema, ctx)?;
            index_list(&list, &index)?
        }
        Expr::Size { value } => match eval(value, row, schema, ctx)? {
            Value::Null => Value::Null,
            Value::Text(text) => Value::Int(text.chars().count() as i64),
            Value::List(items) => Value::Int(items.len() as i64),
            Value::Map(entries) => Value::Int(entries.len() as i64),
            Value::Path(path) => Value::Int(path.length() as i64),
            other => {
                return Err(Error::Execute(format!(
                    "size() is not defined for a {}",
                    describe(&other)
                )))
            }
        },
        Expr::FunctionCall { name, args } => evaluate_function(name, args, row, schema, ctx)?,
    })
}

fn property_of(subject: &Value, key: PropertyKeyId, ctx: &EvalContext<'_>) -> Result<Value> {
    Ok(match subject {
        Value::Null => Value::Null,
        Value::Vertex(id) => match ctx.storage.get_vertex(*id, &ctx.txn)? {
            Some(record) => record
                .properties
                .get(&key)
                .map(Value::from_property)
                .unwrap_or(Value::Null),
            None => Value::Null,
        },
        Value::Edge(id) => match ctx.storage.get_edge(*id, &ctx.txn)? {
            Some(record) => record
                .properties
                .get(&key)
                .map(Value::from_property)
                .unwrap_or(Value::Null),
            None => Value::Null,
        },
        Value::Map(entries) => {
            let name = ctx
                .key_name(key)
                .ok_or_else(|| Error::Execute("unknown property key".into()))?;
            entries
                .iter()
                .find(|(entry, _)| entry == &name)
                .map(|(_, value)| value.clone())
                .unwrap_or(Value::Null)
        }
        Value::Temporal(temporal) => {
            let name = ctx
                .key_name(key)
                .ok_or_else(|| Error::Execute("unknown property key".into()))?;
            temporal
                .accessor(&name)?
                .map(Value::Int)
                .unwrap_or(Value::Null)
        }
        other => {
            return Err(Error::Execute(format!(
                "a {} has no properties",
                describe(other)
            )))
        }
    })
}

fn arithmetic(left: Value, right: Value, operator: char) -> Result<Value> {
    if left.is_null() || right.is_null() {
        return Ok(Value::Null);
    }
    if operator == '+' {
        match (&left, &right) {
            (Value::Text(a), Value::Text(b)) => return Ok(Value::Text(format!("{a}{b}"))),
            (Value::Text(a), other) => return Ok(Value::Text(format!("{a}{}", render(other)))),
            (other, Value::Text(b)) => return Ok(Value::Text(format!("{}{b}", render(other)))),
            // Two lists concatenate; a list plus a scalar appends.
            (Value::List(left), Value::List(right)) => {
                let mut out = left.clone();
                out.extend(right.iter().cloned());
                return Ok(Value::List(out));
            }
            (Value::List(items), other) => {
                let mut out = items.clone();
                out.push(other.clone());
                return Ok(Value::List(out));
            }
            (other, Value::List(items)) => {
                let mut out = vec![other.clone()];
                out.extend(items.iter().cloned());
                return Ok(Value::List(out));
            }
            _ => {}
        }
    }
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(match operator {
            '+' => Value::Int(a.wrapping_add(*b)),
            '-' => Value::Int(a.wrapping_sub(*b)),
            '*' => Value::Int(a.wrapping_mul(*b)),
            '/' => {
                if *b == 0 {
                    return Ok(Value::Null);
                }
                Value::Int(a.div_euclid(*b))
            }
            '%' => {
                if *b == 0 {
                    return Ok(Value::Null);
                }
                Value::Int(a.rem_euclid(*b))
            }
            _ => unreachable!("operator is one of the five arithmetic symbols"),
        }),
        _ => {
            let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) else {
                return Err(Error::Execute(format!(
                    "cannot apply '{operator}' to a {} and a {}",
                    describe(&left),
                    describe(&right)
                )));
            };
            Ok(match operator {
                '+' => Value::Float(a + b),
                '-' => Value::Float(a - b),
                '*' => Value::Float(a * b),
                '/' => {
                    if b == 0.0 {
                        Value::Null
                    } else {
                        Value::Float(a / b)
                    }
                }
                '%' => {
                    if b == 0.0 {
                        Value::Null
                    } else {
                        Value::Float(a % b)
                    }
                }
                _ => unreachable!("operator is one of the five arithmetic symbols"),
            })
        }
    }
}

fn compare(left: Value, right: Value, operator: char) -> Result<Value> {
    if left.is_null() || right.is_null() {
        return Ok(Value::Null);
    }
    Ok(match operator {
        '=' => match left.equals(&right) {
            Some(value) => Value::Bool(value),
            None => Value::Null,
        },
        'n' => match left.equals(&right) {
            Some(value) => Value::Bool(!value),
            None => Value::Null,
        },
        _ => match left.compare(&right) {
            None => Value::Null,
            Some(ordering) => Value::Bool(match operator {
                '<' => ordering == std::cmp::Ordering::Less,
                '>' => ordering == std::cmp::Ordering::Greater,
                'l' => ordering != std::cmp::Ordering::Greater,
                'g' => ordering != std::cmp::Ordering::Less,
                _ => unreachable!("operator is one of =, n, <, >, l, g"),
            }),
        },
    })
}

fn string_predicate<F>(left: Value, right: Value, predicate: F) -> Result<Value>
where
    F: Fn(&str, &str) -> bool,
{
    match (&left, &right) {
        (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
        (Value::Text(text), Value::Text(other)) => Ok(Value::Bool(predicate(text, other))),
        _ => Err(Error::Execute(
            "string predicates require string operands".into(),
        )),
    }
}

fn index_list(list: &Value, index: &Value) -> Result<Value> {
    let Value::List(items) = list else {
        return Err(Error::Execute("subscript requires a list".into()));
    };
    let Some(index) = index.as_i64() else {
        return Err(Error::Execute(
            "a list subscript must be an integer".into(),
        ));
    };
    let length = items.len() as i64;
    let position = if index < 0 { length + index } else { index };
    if position < 0 || position >= length {
        return Ok(Value::Null);
    }
    Ok(items[position as usize].clone())
}

/// Readable rendering used for string concatenation and error messages.
fn render(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(v) => v.to_string(),
        Value::Int(v) => v.to_string(),
        Value::Float(v) => render_float(*v),
        Value::Text(v) => v.clone(),
        other => format!("{}", describe(other)),
    }
}

fn render_float(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{:.1}", value)
    } else {
        format!("{value}")
    }
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

fn evaluate_function(
    name: &str,
    args: &[Expr],
    row: &EvalRow,
    schema: &[String],
    ctx: &EvalContext<'_>,
) -> Result<Value> {
    let lower = name.to_ascii_lowercase();
    // Comprehension and quantifier bodies bind their own variable, so they are
    // evaluated inside the loop that introduces it rather than eagerly here.
    if matches!(
        lower.as_str(),
        "list_comprehension" | "all" | "any" | "none" | "single"
    ) {
        return evaluate_scoped_function(&lower, args, row, schema, ctx);
    }
    let mut evaluated = Vec::with_capacity(args.len());
    for arg in args {
        evaluated.push(eval(arg, row, schema, ctx)?);
    }
    match lower.as_str() {
        // -- introspection --------------------------------------------------
        "id" => match evaluated.first() {
            Some(Value::Vertex(id)) => Ok(Value::Int(id.0 as i64)),
            Some(Value::Edge(id)) => Ok(Value::Int(id.0 as i64)),
            Some(Value::Null) => Ok(Value::Null),
            _ => Err(Error::Execute("id() requires a vertex or relationship".into())),
        },
        "labels" => match evaluated.first() {
            Some(Value::Vertex(id)) => {
                let record = ctx
                    .storage
                    .get_vertex(*id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                let mut labels = Vec::with_capacity(record.labels.len());
                for label in &record.labels {
                    labels.push(Value::Text(
                        ctx.schema
                            .label_name(*label)
                            .unwrap_or("<unknown>")
                            .to_string(),
                    ));
                }
                Ok(Value::List(labels))
            }
            Some(Value::Null) => Ok(Value::Null),
            _ => Err(Error::Execute("labels() requires a vertex".into())),
        },
        "type" => match evaluated.first() {
            Some(Value::Edge(id)) => {
                let record = ctx
                    .storage
                    .get_edge(*id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("edge {}", id.0)))?;
                Ok(Value::Text(
                    ctx.schema
                        .edge_type_name(record.etype)
                        .unwrap_or("<unknown>")
                        .to_string(),
                ))
            }
            Some(Value::Null) => Ok(Value::Null),
            _ => Err(Error::Execute("type() requires a relationship".into())),
        },
        "properties" => match evaluated.first() {
            Some(Value::Map(entries)) => Ok(Value::Map(entries.clone())),
            Some(Value::Vertex(id)) => {
                let record = ctx
                    .storage
                    .get_vertex(*id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                Ok(Value::Map(properties_to_map(&record.properties, ctx)))
            }
            Some(Value::Edge(id)) => {
                let record = ctx
                    .storage
                    .get_edge(*id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("edge {}", id.0)))?;
                Ok(Value::Map(properties_to_map(&record.properties, ctx)))
            }
            Some(Value::Null) => Ok(Value::Null),
            _ => Err(Error::Execute(
                "properties() requires a vertex, relationship, or map".into(),
            )),
        },
        "keys" => match evaluated.first() {
            Some(Value::Map(entries)) => Ok(Value::List(
                entries.iter().map(|(key, _)| Value::Text(key.clone())).collect(),
            )),
            Some(Value::Vertex(id)) => {
                let record = ctx
                    .storage
                    .get_vertex(*id, &ctx.txn)?
                    .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
                Ok(Value::List(
                    record
                        .properties
                        .keys()
                        .map(|key| {
                            Value::Text(
                                ctx.key_name(*key).unwrap_or_else(|| "<unknown>".into()),
                            )
                        })
                        .collect(),
                ))
            }
            Some(Value::Null) => Ok(Value::Null),
            _ => Err(Error::Execute("keys() requires a vertex or map".into())),
        },
        "exists" => Ok(Value::Bool(!evaluated
            .first()
            .cloned()
            .unwrap_or(Value::Null)
            .is_null())),
        "coalesce" => Ok(evaluated
            .into_iter()
            .find(|value| !value.is_null())
            .unwrap_or(Value::Null)),
        "has_label" => {
            let Some(Value::Vertex(id)) = evaluated.first().cloned() else {
                return Ok(Value::Bool(false));
            };
            let record = ctx
                .storage
                .get_vertex(id, &ctx.txn)?
                .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
            for label in evaluated.iter().skip(1) {
                let Some(code) = label.as_i64() else { continue };
                if record.labels.contains(&LabelId(code as u32)) {
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        "has_properties" => {
            let Some(Value::Vertex(id)) = evaluated.first().cloned() else {
                return Ok(Value::Bool(false));
            };
            let Some(Value::Text(parameter)) = evaluated.get(1).cloned() else {
                return Ok(Value::Bool(false));
            };
            let record = ctx
                .storage
                .get_vertex(id, &ctx.txn)?
                .ok_or_else(|| Error::NotFound(format!("vertex {}", id.0)))?;
            let Some(Value::Map(entries)) = ctx.parameters.get(&parameter).cloned() else {
                return Err(Error::Execute(format!(
                    "query parameter '{parameter}' is not bound"
                )));
            };
            Ok(Value::Bool(entries.iter().all(|(key, value)| {
                ctx.schema
                    .property_key_by_name(key)
                    .and_then(|key| record.properties.get(&key))
                    .map(|stored| Value::from_property(stored).equals(value) == Some(true))
                    .unwrap_or(false)
            })))
        }
        "parameter" => {
            let Some(Value::Text(name)) = evaluated.first().cloned() else {
                return Err(Error::Execute("a parameter needs a name".into()));
            };
            ctx.parameters
                .get(&name)
                .cloned()
                .ok_or_else(|| Error::Execute(format!("query parameter '{name}' is not bound")))
        }
        "map" => {
            if evaluated.len() % 2 != 0 {
                return Err(Error::Execute("a map literal needs key/value pairs".into()));
            }
            let mut entries = Vec::with_capacity(evaluated.len() / 2);
            let mut iter = evaluated.into_iter();
            while let (Some(key), Some(value)) = (iter.next(), iter.next()) {
                entries.push((render(&key), value));
            }
            Ok(Value::Map(entries))
        }
        "slice" => {
            let list = evaluated.first().cloned().unwrap_or(Value::Null);
            let from = evaluated.get(1).cloned().unwrap_or(Value::Null);
            let to = evaluated.get(2).cloned().unwrap_or(Value::Null);
            let Value::List(items) = list else {
                return if list.is_null() {
                    Ok(Value::Null)
                } else {
                    Err(Error::Execute("slice() requires a list".into()))
                };
            };
            let length = items.len() as i64;
            let normalise = |value: &Value, default: i64| -> i64 {
                match value.as_i64() {
                    Some(index) if index < 0 => (length + index).max(0),
                    Some(index) => index.min(length),
                    None => default,
                }
            };
            let start = normalise(&from, 0);
            let end = normalise(&to, length);
            let start = start.clamp(0, length) as usize;
            let end = end.clamp(0, length) as usize;
            Ok(Value::List(if start >= end {
                Vec::new()
            } else {
                items[start..end].to_vec()
            }))
        }
        "in" => {
            let item = evaluated.first().cloned().unwrap_or(Value::Null);
            let list = evaluated.get(1).cloned().unwrap_or(Value::Null);
            if item.is_null() || list.is_null() {
                return Ok(Value::Null);
            }
            let Value::List(items) = list else {
                return Err(Error::Execute("IN requires a list on its right".into()));
            };
            let mut unknown = false;
            for candidate in &items {
                match item.equals(candidate) {
                    Some(true) => return Ok(Value::Bool(true)),
                    Some(false) => {}
                    None => unknown = true,
                }
            }
            Ok(if unknown { Value::Null } else { Value::Bool(false) })
        }
        "xor" => {
            let left = evaluated.first().cloned().unwrap_or(Value::Null);
            let right = evaluated.get(1).cloned().unwrap_or(Value::Null);
            match (left, right) {
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
                (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                _ => Err(Error::Execute("XOR requires boolean operands".into())),
            }
        }
        "power" => {
            let left = evaluated.first().cloned().unwrap_or(Value::Null);
            let right = evaluated.get(1).cloned().unwrap_or(Value::Null);
            match (left.as_f64(), right.as_f64()) {
                (Some(a), Some(b)) => Ok(Value::Float(a.powf(b))),
                _ => Ok(Value::Null),
            }
        }
        // -- collection functions -------------------------------------------
        "size" => match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Ok(Value::Null),
            Value::Text(text) => Ok(Value::Int(text.chars().count() as i64)),
            Value::List(items) => Ok(Value::Int(items.len() as i64)),
            Value::Map(entries) => Ok(Value::Int(entries.len() as i64)),
            Value::Path(path) => Ok(Value::Int(path.length() as i64)),
            other => Err(Error::Execute(format!(
                "size() is not defined for a {}",
                describe(&other)
            ))),
        },
        "length" => match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Ok(Value::Null),
            Value::Path(path) => Ok(Value::Int(path.length() as i64)),
            Value::Text(text) => Ok(Value::Int(text.chars().count() as i64)),
            Value::List(items) => Ok(Value::Int(items.len() as i64)),
            other => Err(Error::Execute(format!(
                "length() is not defined for a {}",
                describe(&other)
            ))),
        },
        "nodes" => match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Ok(Value::Null),
            Value::Path(path) => Ok(Value::List(
                path.nodes.into_iter().map(Value::Vertex).collect(),
            )),
            other => Err(Error::Execute(format!(
                "nodes() requires a path, got a {}",
                describe(&other)
            ))),
        },
        "relationships" => match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Ok(Value::Null),
            Value::Path(path) => Ok(Value::List(
                path.edges.into_iter().map(Value::Edge).collect(),
            )),
            other => Err(Error::Execute(format!(
                "relationships() requires a path, got a {}",
                describe(&other)
            ))),
        },
        "head" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::List(items) => items.first().cloned().unwrap_or(Value::Null),
            other => {
                return Err(Error::Execute(format!(
                    "head() requires a list, got a {}",
                    describe(&other)
                )))
            }
        }),
        "last" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::List(items) => items.last().cloned().unwrap_or(Value::Null),
            other => {
                return Err(Error::Execute(format!(
                    "last() requires a list, got a {}",
                    describe(&other)
                )))
            }
        }),
        "tail" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::List(items) => Value::List(items.into_iter().skip(1).collect()),
            other => {
                return Err(Error::Execute(format!(
                    "tail() requires a list, got a {}",
                    describe(&other)
                )))
            }
        }),
        "reverse" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::List(mut items) => {
                items.reverse();
                Value::List(items)
            }
            Value::Text(text) => Value::Text(text.chars().rev().collect()),
            other => {
                return Err(Error::Execute(format!(
                    "reverse() requires a list or a string, got a {}",
                    describe(&other)
                )))
            }
        }),
        "range" => {
            let start = evaluated.first().and_then(Value::as_i64).ok_or_else(|| {
                Error::Execute("range() requires an integer start".into())
            })?;
            let end = evaluated
                .get(1)
                .and_then(Value::as_i64)
                .ok_or_else(|| Error::Execute("range() requires an integer end".into()))?;
            let step = match evaluated.get(2) {
                Some(value) => value.as_i64().ok_or_else(|| {
                    Error::Execute("range() requires an integer step".into())
                })?,
                None => 1,
            };
            if step == 0 {
                return Err(Error::Execute("range() step must not be zero".into()));
            }
            let mut out = Vec::new();
            let mut current = start;
            if step > 0 {
                while current <= end {
                    out.push(Value::Int(current));
                    current += step;
                }
            } else {
                while current >= end {
                    out.push(Value::Int(current));
                    current += step;
                }
            }
            Ok(Value::List(out))
        }
        // -- string functions ------------------------------------------------
        "toupper" => map_text(&evaluated, |text| text.to_uppercase()),
        "tolower" => map_text(&evaluated, |text| text.to_lowercase()),
        "trim" => map_text(&evaluated, |text| text.trim().to_string()),
        "substring" => {
            let Value::Text(text) = evaluated.first().cloned().unwrap_or(Value::Null) else {
                return Ok(Value::Null);
            };
            let characters: Vec<char> = text.chars().collect();
            let start = evaluated
                .get(1)
                .and_then(Value::as_i64)
                .ok_or_else(|| Error::Execute("substring() needs an integer start".into()))?;
            let start = start.clamp(0, characters.len() as i64) as usize;
            let end = match evaluated.get(2) {
                Some(value) => {
                    let length = value.as_i64().ok_or_else(|| {
                        Error::Execute("substring() needs an integer length".into())
                    })?;
                    (start as i64 + length).clamp(0, characters.len() as i64) as usize
                }
                None => characters.len(),
            };
            Ok(Value::Text(characters[start..end.max(start)].iter().collect()))
        }
        "left" => take_side(&evaluated, true),
        "right" => take_side(&evaluated, false),
        "replace" => {
            let Value::Text(text) = evaluated.first().cloned().unwrap_or(Value::Null) else {
                return Ok(Value::Null);
            };
            let search = evaluated.get(1).and_then(Value::as_text).ok_or_else(|| {
                Error::Execute("replace() needs a string to replace".into())
            })?;
            let replacement = evaluated
                .get(2)
                .and_then(Value::as_text)
                .ok_or_else(|| Error::Execute("replace() needs a replacement".into()))?;
            Ok(Value::Text(text.replace(search, replacement)))
        }
        "split" => {
            let Value::Text(text) = evaluated.first().cloned().unwrap_or(Value::Null) else {
                return Ok(Value::Null);
            };
            let separator = evaluated
                .get(1)
                .and_then(Value::as_text)
                .ok_or_else(|| Error::Execute("split() needs a separator".into()))?;
            let parts = if separator.is_empty() {
                text.chars().map(|c| c.to_string()).collect::<Vec<_>>()
            } else {
                text.split(separator).map(str::to_string).collect()
            };
            Ok(Value::List(parts.into_iter().map(Value::Text).collect()))
        }
        // -- numeric and conversion functions --------------------------------
        "abs" => numeric(&evaluated, |a| a.abs(), |a| a.abs()),
        "ceil" => float_function(&evaluated, f64::ceil),
        "floor" => float_function(&evaluated, f64::floor),
        "round" => float_function(&evaluated, f64::round),
        "sqrt" => float_function(&evaluated, f64::sqrt),
        "sign" => numeric(
            &evaluated,
            |a| a.signum(),
            |a| if a > 0.0 { 1.0 } else if a < 0.0 { -1.0 } else { 0.0 },
        ),
        "tostring" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::Text(text) => Value::Text(text),
            Value::Bool(value) => Value::Text(value.to_string()),
            Value::Int(value) => Value::Text(value.to_string()),
            Value::Float(value) => Value::Text(render_float(value)),
            Value::Temporal(temporal) => Value::Text(temporal.text),
            other => {
                return Err(Error::Execute(format!(
                    "toString() is not defined for a {}",
                    describe(&other)
                )))
            }
        }),
        "tointeger" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::Int(value) => Value::Int(value),
            Value::Float(value) => Value::Int(value.trunc() as i64),
            Value::Text(text) => match text.trim().parse::<i64>() {
                Ok(value) => Value::Int(value),
                Err(_) => match text.trim().parse::<f64>() {
                    Ok(value) => Value::Int(value.trunc() as i64),
                    Err(_) => Value::Null,
                },
            },
            other => {
                return Err(Error::Execute(format!(
                    "toInteger() is not defined for a {}",
                    describe(&other)
                )))
            }
        }),
        "tofloat" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::Int(value) => Value::Float(value as f64),
            Value::Float(value) => Value::Float(value),
            Value::Text(text) => match text.trim().parse::<f64>() {
                Ok(value) => Value::Float(value),
                Err(_) => Value::Null,
            },
            other => {
                return Err(Error::Execute(format!(
                    "toFloat() is not defined for a {}",
                    describe(&other)
                )))
            }
        }),
        "toboolean" => Ok(match evaluated.first().cloned().unwrap_or(Value::Null) {
            Value::Null => Value::Null,
            Value::Bool(value) => Value::Bool(value),
            Value::Text(text) => Value::Bool(text.trim().eq_ignore_ascii_case("true")),
            other => {
                return Err(Error::Execute(format!(
                    "toBoolean() is not defined for a {}",
                    describe(&other)
                )))
            }
        }),
        "path_append" => {
            let (Some(Value::Path(left)), Some(Value::Path(right))) =
                (evaluated.first().cloned(), evaluated.get(1).cloned())
            else {
                return Err(Error::Execute(
                    "path_append() requires two paths".into(),
                ));
            };
            let mut nodes = left.nodes;
            // The two pieces share their junction vertex.
            nodes.extend(right.nodes.into_iter().skip(1));
            let mut edges = left.edges;
            edges.extend(right.edges);
            Ok(Value::Path(PathValue { nodes, edges }))
        }
        // -- temporal functions ----------------------------------------------
        "date" => temporal_function(&evaluated, TemporalKind::Date),
        "time" => temporal_function(&evaluated, TemporalKind::Time),
        "localtime" => temporal_function(&evaluated, TemporalKind::LocalTime),
        "datetime" => temporal_function(&evaluated, TemporalKind::DateTime),
        "localdatetime" => temporal_function(&evaluated, TemporalKind::LocalDateTime),
        "duration" => temporal_function(&evaluated, TemporalKind::Duration),
        other => Err(Error::Unsupported(format!(
            "the function '{other}' is outside the requested scope"
        ))),
    }
}

/// Evaluate a list comprehension or a list quantifier, which bind their own
/// variable for the filter and projection bodies.
fn evaluate_scoped_function(
    name: &str,
    args: &[Expr],
    row: &EvalRow,
    schema: &[String],
    ctx: &EvalContext<'_>,
) -> Result<Value> {
    let variable = match args.first() {
        Some(Expr::String(variable)) => variable.clone(),
        _ => {
            return Err(Error::Execute(format!(
                "{name}() needs its bound variable as a literal name"
            )))
        }
    };
    let list = match args.get(1) {
        Some(list) => eval(list, row, schema, ctx)?,
        None => Value::Null,
    };
    let Value::List(items) = list else {
        // An empty or null list satisfies `all` and fails the others.
        return Ok(Value::Bool(matches!(name, "all")));
    };
    let filter = args.get(2);
    let mut matched = 0usize;
    let mut projected = Vec::new();
    for item in items {
        let (extended, extended_schema) = extend_row(row, schema, &variable, item.clone());
        let keep = match filter {
            Some(filter) => eval(filter, &extended, &extended_schema, ctx)?.is_true(),
            None => true,
        };
        if !keep {
            continue;
        }
        matched += 1;
        if name == "list_comprehension" {
            let value = match args.get(3) {
                Some(projection) => eval(projection, &extended, &extended_schema, ctx)?,
                None => item,
            };
            projected.push(value);
        }
    }
    let count = match filter {
        Some(_) => {
            // `matched` counts survivors; the quantifier's denominator is the
            // number of items the predicate was evaluated over.
            let total = match eval(&args[1], row, schema, ctx)? {
                Value::List(items) => items.len(),
                _ => 0,
            };
            total
        }
        None => matched,
    };
    Ok(match name {
        "list_comprehension" => Value::List(projected),
        "all" => Value::Bool(matched == count),
        "any" => Value::Bool(matched > 0),
        "none" => Value::Bool(matched == 0),
        _ => Value::Bool(matched == 1),
    })
}

fn extend_row(row: &EvalRow, schema: &[String], name: &str, value: Value) -> (EvalRow, Vec<String>) {
    let mut extended = row.clone();
    extended.push(value);
    let mut extended_schema = schema.to_vec();
    extended_schema.push(name.to_string());
    (extended, extended_schema)
}

fn properties_to_map(properties: &PropertyMap, ctx: &EvalContext<'_>) -> Vec<(String, Value)> {
    properties
        .iter()
        .map(|(key, value)| {
            (
                ctx.key_name(*key).unwrap_or_else(|| "<unknown>".into()),
                Value::from_property(value),
            )
        })
        .collect()
}

fn map_text(values: &[Value], transform: impl Fn(&str) -> String) -> Result<Value> {
    match values.first().cloned().unwrap_or(Value::Null) {
        Value::Null => Ok(Value::Null),
        Value::Text(text) => Ok(Value::Text(transform(&text))),
        other => Err(Error::Execute(format!(
            "a string function requires a string, got a {}",
            describe(&other)
        ))),
    }
}

fn take_side(values: &[Value], from_left: bool) -> Result<Value> {
    let Value::Text(text) = values.first().cloned().unwrap_or(Value::Null) else {
        return Ok(Value::Null);
    };
    let count = values
        .get(1)
        .and_then(Value::as_i64)
        .ok_or_else(|| Error::Execute("left()/right() need an integer length".into()))?;
    let characters: Vec<char> = text.chars().collect();
    let count = count.clamp(0, characters.len() as i64) as usize;
    let taken: String = if from_left {
        characters[..count].iter().collect()
    } else {
        characters[characters.len() - count..].iter().collect()
    };
    Ok(Value::Text(taken))
}

fn numeric(
    values: &[Value],
    integer: impl Fn(i64) -> i64,
    float: impl Fn(f64) -> f64,
) -> Result<Value> {
    Ok(match values.first().cloned().unwrap_or(Value::Null) {
        Value::Null => Value::Null,
        Value::Int(value) => Value::Int(integer(value)),
        Value::Float(value) => Value::Float(float(value)),
        other => {
            return Err(Error::Execute(format!(
                "a numeric function requires a number, got a {}",
                describe(&other)
            )))
        }
    })
}

fn float_function(values: &[Value], transform: impl Fn(f64) -> f64) -> Result<Value> {
    Ok(match values.first().cloned().unwrap_or(Value::Null) {
        Value::Null => Value::Null,
        Value::Int(value) => Value::Float(transform(value as f64)),
        Value::Float(value) => Value::Float(transform(value)),
        other => {
            return Err(Error::Execute(format!(
                "a numeric function requires a number, got a {}",
                describe(&other)
            )))
        }
    })
}

fn temporal_function(values: &[Value], kind: TemporalKind) -> Result<Value> {
    match values.first().cloned().unwrap_or(Value::Null) {
        Value::Null => Ok(Value::Null),
        Value::Text(text) => Ok(Value::Temporal(Temporal::parse(kind, &text)?)),
        other => Err(Error::Execute(format!(
            "a temporal function requires a string, got a {}",
            describe(&other)
        ))),
    }
}

// ---------------------------------------------------------------------------
// Aggregation
// ---------------------------------------------------------------------------

enum Accumulator {
    CountAll(i64),
    Count(i64),
    CountDistinct(Vec<String>),
    Sum(f64, bool),
    SumDistinct(Vec<String>, f64),
    Avg(f64, i64),
    Min(Option<Value>),
    Max(Option<Value>),
    Collect(Vec<Value>),
    CollectDistinct(Vec<String>, Vec<Value>),
}

impl Accumulator {
    fn new(aggregate: &AggregateExpr) -> Result<Self> {
        Ok(match aggregate {
            AggregateExpr::CountAll => Accumulator::CountAll(0),
            AggregateExpr::Count(_) => Accumulator::Count(0),
            AggregateExpr::CountDistinct(_) => Accumulator::CountDistinct(Vec::new()),
            AggregateExpr::Sum(_) => Accumulator::Sum(0.0, false),
            AggregateExpr::SumDistinct(_) => Accumulator::SumDistinct(Vec::new(), 0.0),
            AggregateExpr::Avg(_) => Accumulator::Avg(0.0, 0),
            AggregateExpr::Min(_) => Accumulator::Min(None),
            AggregateExpr::Max(_) => Accumulator::Max(None),
            AggregateExpr::Collect(_) => Accumulator::Collect(Vec::new()),
            AggregateExpr::CollectDistinct(_) => {
                Accumulator::CollectDistinct(Vec::new(), Vec::new())
            }
        })
    }

    fn accumulate(
        &mut self,
        aggregate: &AggregateExpr,
        row: &EvalRow,
        schema: &[String],
        ctx: &EvalContext<'_>,
    ) -> Result<()> {
        let inner = |expr: &Expr| eval(expr, row, schema, ctx);
        match (self, aggregate) {
            (Accumulator::CountAll(count), AggregateExpr::CountAll) => *count += 1,
            (Accumulator::Count(count), AggregateExpr::Count(expr)) => {
                if !inner(expr)?.is_null() {
                    *count += 1;
                }
            }
            (Accumulator::CountDistinct(seen), AggregateExpr::CountDistinct(expr)) => {
                let value = inner(expr)?;
                if !value.is_null() {
                    let key = value.group_key();
                    if !seen.contains(&key) {
                        seen.push(key);
                    }
                }
            }
            (Accumulator::Sum(total, seen_any), AggregateExpr::Sum(expr)) => {
                let value = inner(expr)?;
                if let Some(number) = value.as_f64() {
                    *total += number;
                    *seen_any = true;
                }
            }
            (Accumulator::SumDistinct(seen, total), AggregateExpr::SumDistinct(expr)) => {
                let value = inner(expr)?;
                if let Some(number) = value.as_f64() {
                    let key = value.group_key();
                    if !seen.contains(&key) {
                        seen.push(key);
                        *total += number;
                    }
                }
            }
            (Accumulator::Avg(total, count), AggregateExpr::Avg(expr)) => {
                let value = inner(expr)?;
                if let Some(number) = value.as_f64() {
                    *total += number;
                    *count += 1;
                }
            }
            (Accumulator::Min(current), AggregateExpr::Min(expr)) => {
                let value = inner(expr)?;
                if !value.is_null()
                    && current
                        .as_ref()
                        .map(|existing| value.compare(existing) == Some(std::cmp::Ordering::Less))
                        .unwrap_or(true)
                {
                    *current = Some(value);
                }
            }
            (Accumulator::Max(current), AggregateExpr::Max(expr)) => {
                let value = inner(expr)?;
                if !value.is_null()
                    && current
                        .as_ref()
                        .map(|existing| value.compare(existing) == Some(std::cmp::Ordering::Greater))
                        .unwrap_or(true)
                {
                    *current = Some(value);
                }
            }
            (Accumulator::Collect(items), AggregateExpr::Collect(expr)) => {
                let value = inner(expr)?;
                if !value.is_null() {
                    items.push(value);
                }
            }
            (Accumulator::CollectDistinct(seen, items), AggregateExpr::CollectDistinct(expr)) => {
                let value = inner(expr)?;
                if !value.is_null() {
                    let key = value.group_key();
                    if !seen.contains(&key) {
                        seen.push(key);
                        items.push(value);
                    }
                }
            }
            _ => {
                return Err(Error::Execute(
                    "an aggregate accumulated into the wrong state".into(),
                ))
            }
        }
        Ok(())
    }

    fn value(self) -> Result<Value> {
        Ok(match self {
            Accumulator::CountAll(count) | Accumulator::Count(count) => Value::Int(count),
            Accumulator::CountDistinct(seen) => Value::Int(seen.len() as i64),
            // Cypher's sum of no values is `0`.
            Accumulator::Sum(total, _) => Value::Float(total),
            Accumulator::SumDistinct(_, total) => Value::Float(total),
            Accumulator::Avg(total, count) => {
                if count == 0 {
                    Value::Null
                } else {
                    Value::Float(total / count as f64)
                }
            }
            Accumulator::Min(current) | Accumulator::Max(current) => {
                current.unwrap_or(Value::Null)
            }
            Accumulator::Collect(items) => Value::List(items),
            Accumulator::CollectDistinct(_, items) => Value::List(items),
        })
    }
}
