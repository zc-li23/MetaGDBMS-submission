//! Volcano-style executor for the candidate physical operators.
//!
//! Rows are column-addressed frames; pattern matching, projections, aggregation
//! and mutations all operate on them.  Aggregates are evaluated by substituting
//! their computed value back into the expression tree, so an expression such as
//! `size(collect(n.name))` or `sum(n.age) + count(n)` needs no special casing.

use std::collections::{BTreeMap, HashMap, HashSet};

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue, ResultPath, ResultValue};
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::{EdgeFilter, EdgeRecord, StorageEngine, VertexFilter, VertexRecord};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{AlgorithmArgs, PhysicalPlan, ProjectionSpec};
use metagdb_core::l4_compute::engine::{ExecContext, Executor, ResultStream};
use metagdb_core::l4_compute::olap::{AlgorithmRegistry, GraphProjection};
use metagdb_core::{Error, Result};

use super::cypher_ast::is_aggregate_name;
use super::cypher_ast::*;
use super::physical_ops::*;
use super::projection::CsrProjection;

/// Runtime value produced by the Cypher evaluator.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Value>),
    Map(Vec<(String, Value)>),
    Vertex(VertexRecord),
    Edge(EdgeRecord),
    /// A traversal path: visited nodes and the relationships between them.
    Path(Vec<Value>, Vec<Value>),
    /// Days since the Unix epoch.
    Date(i64),
    /// Seconds since the Unix epoch (UTC).
    DateTime(i64),
    /// Seconds since midnight.
    Time(i64),
    /// Duration in seconds.
    Duration(i64),
}

impl Value {
    fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Total order used by ORDER BY, min/max and DISTINCT.
    fn compare(&self, other: &Value) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match (self, other) {
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
            (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Text(a), Value::Text(b)) => Some(a.cmp(b)),
            (Value::Date(a), Value::Date(b)) => Some(a.cmp(b)),
            (Value::DateTime(a), Value::DateTime(b)) => Some(a.cmp(b)),
            (Value::Time(a), Value::Time(b)) => Some(a.cmp(b)),
            (Value::Duration(a), Value::Duration(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }

    /// Structural equality with numeric coercion, used by `=`, `IN` and DISTINCT.
    fn equals(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Null, _) | (_, Value::Null) => None,
            (Value::Bool(a), Value::Bool(b)) => Some(a == b),
            (Value::Int(a), Value::Int(b)) => Some(a == b),
            (Value::Int(a), Value::Float(b)) => Some(*a as f64 == *b),
            (Value::Float(a), Value::Int(b)) => Some(*a == *b as f64),
            (Value::Float(a), Value::Float(b)) => Some(a == b),
            (Value::Text(a), Value::Text(b)) => Some(a == b),
            (Value::Date(a), Value::Date(b)) => Some(a == b),
            (Value::DateTime(a), Value::DateTime(b)) => Some(a == b),
            (Value::Time(a), Value::Time(b)) => Some(a == b),
            (Value::Duration(a), Value::Duration(b)) => Some(a == b),
            (Value::Vertex(a), Value::Vertex(b)) => Some(a.id == b.id),
            (Value::Edge(a), Value::Edge(b)) => Some(a.id == b.id),
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return Some(false);
                }
                let mut equal = true;
                for (left, right) in a.iter().zip(b.iter()) {
                    match left.equals(right) {
                        Some(true) => {}
                        _ => {
                            equal = false;
                            break;
                        }
                    }
                }
                Some(equal)
            }
            (Value::Map(a), Value::Map(b)) => {
                if a.len() != b.len() {
                    return Some(false);
                }
                let mut equal = true;
                for (key, left) in a {
                    match b.iter().find(|(other, _)| other == key) {
                        Some((_, right)) => match left.equals(right) {
                            Some(true) => {}
                            _ => {
                                equal = false;
                                break;
                            }
                        },
                        None => {
                            equal = false;
                            break;
                        }
                    }
                }
                Some(equal)
            }
            _ => Some(false),
        }
    }

    /// Canonical text used as a grouping/deduplication key.
    fn key(&self) -> String {
        match self {
            Value::Null => "∅".into(),
            Value::Bool(v) => format!("b{v}"),
            Value::Int(v) => format!("i{v}"),
            Value::Float(v) => format!("f{v}"),
            Value::Text(v) => format!("t{v}"),
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(Value::key).collect();
                format!("l[{}]", inner.join(","))
            }
            Value::Map(entries) => {
                let mut parts: Vec<String> = entries
                    .iter()
                    .map(|(key, value)| format!("{key}:{}", value.key()))
                    .collect();
                parts.sort();
                format!("m{{{}}}", parts.join(","))
            }
            Value::Vertex(v) => format!("v{}", v.id.0),
            Value::Edge(e) => format!("e{}", e.id.0),
            Value::Path(nodes, edges) => {
                let nodes: Vec<String> = nodes.iter().map(Value::key).collect();
                let edges: Vec<String> = edges.iter().map(Value::key).collect();
                format!("p[{}|{}]", nodes.join(","), edges.join(","))
            }
            Value::Date(v) => format!("d{v}"),
            Value::DateTime(v) => format!("dt{v}"),
            Value::Time(v) => format!("tm{v}"),
            Value::Duration(v) => format!("du{v}"),
        }
    }

    /// Truthiness for a predicate: only `true` passes (NULL and non-booleans fail).
    fn is_true(&self) -> bool {
        matches!(self, Value::Bool(true))
    }

    fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(value) => Some(value),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(value) => Some(*value),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(value) => Some(*value),
            Value::Int(value) => Some(*value as f64),
            _ => None,
        }
    }

    /// Convert one line of a Cypher map literal into a value.
    fn result_value(&self) -> Option<ResultValue> {
        Some(match self {
            Value::Null => ResultValue::Null,
            Value::Bool(v) => ResultValue::Bool(*v),
            Value::Int(v) => ResultValue::Int(*v),
            Value::Float(v) => ResultValue::Float(*v),
            Value::Text(v) => ResultValue::Text(v.clone()),
            Value::List(items) => ResultValue::List(
                items
                    .iter()
                    .map(Value::result_value)
                    .collect::<Option<Vec<_>>>()?,
            ),
            Value::Map(entries) => ResultValue::Map(
                entries
                    .iter()
                    .map(|(key, value)| {
                        Some((ResultValue::Text(key.clone()), value.result_value()?))
                    })
                    .collect::<Option<Vec<_>>>()?,
            ),
            Value::Vertex(vertex) => ResultValue::VertexRef(vertex.id),
            Value::Edge(edge) => ResultValue::EdgeRef(edge.id),
            Value::Path(nodes, edges) => {
                let mut objects = Vec::new();
                for (index, node) in nodes.iter().enumerate() {
                    objects.push(node.result_value()?);
                    if let Some(edge) = edges.get(index) {
                        objects.push(edge.result_value()?);
                    }
                }
                ResultValue::Path(ResultPath::from_objects(objects))
            }
            Value::Date(days) => ResultValue::Text(format_date(*days)),
            Value::DateTime(seconds) => ResultValue::Text(format!("{seconds}")),
            Value::Time(seconds) => ResultValue::Text(format!("{seconds}")),
            Value::Duration(seconds) => ResultValue::Text(format!("{seconds}")),
        })
    }
}

fn format_date(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Howard Hinnant's `civil_from_days`, accurate for the proleptic Gregorian
/// calendar and used by the temporal accessors.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let mp = if month > 2 { month - 3 } else { month + 9 } as u64;
    let doy = (153 * mp + 2) / 5 + day as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe as i64 - 719_468
}

/// Column-addressed row frame flowing between operators.
#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub cols: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

impl Frame {
    fn single_row() -> Self {
        Frame {
            cols: Vec::new(),
            rows: vec![Vec::new()],
        }
    }

    fn empty() -> Self {
        Frame {
            cols: Vec::new(),
            rows: Vec::new(),
        }
    }

    fn index(&self, name: &str) -> Option<usize> {
        self.cols.iter().position(|col| col == name)
    }

}

/// One row under construction while matching a pattern.
#[derive(Debug, Clone, Default)]
struct RowBuilder {
    cols: Vec<String>,
    values: Vec<Value>,
}

impl RowBuilder {
    fn from_row(cols: &[String], values: &[Value]) -> Self {
        let mut values = values.to_vec();
        values.resize(cols.len(), Value::Null);
        RowBuilder {
            cols: cols.to_vec(),
            values,
        }
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.cols
            .iter()
            .position(|col| col == name)
            .and_then(|index| self.values.get(index))
    }

    fn set(&mut self, name: &str, value: Value) {
        match self.cols.iter().position(|col| col == name) {
            Some(index) => self.values[index] = value,
            None => {
                self.cols.push(name.to_string());
                self.values.push(value);
            }
        }
    }
}

/// Normalise builders into one frame with a shared column layout.
fn builders_to_frame(builders: Vec<RowBuilder>) -> Frame {
    builders_to_frame_with(builders, Vec::new())
}

/// Like `builders_to_frame`, but keeps `fallback` columns when no builder
/// survived so that an empty intermediate result still has a schema.
fn builders_to_frame_with(builders: Vec<RowBuilder>, fallback: Vec<String>) -> Frame {
    let mut cols: Vec<String> = fallback;
    let mut seen: HashSet<String> = cols.iter().cloned().collect();
    for builder in &builders {
        for col in &builder.cols {
            if seen.insert(col.clone()) {
                cols.push(col.clone());
            }
        }
    }
    let index_of: HashMap<&str, usize> = cols
        .iter()
        .enumerate()
        .map(|(index, col)| (col.as_str(), index))
        .collect();
    let rows = builders
        .into_iter()
        .map(|builder| {
            let mut row = vec![Value::Null; cols.len()];
            for (position, col) in builder.cols.iter().enumerate() {
                if let Some(index) = index_of.get(col.as_str()) {
                    row[*index] = builder.values[position].clone();
                }
            }
            row
        })
        .collect();
    Frame { cols, rows }
}

type Row = Vec<Value>;

/// Framework executor for the candidate physical operators.
#[derive(Default)]
pub struct MetagdbExecutor;

impl Executor for MetagdbExecutor {
    fn execute(&self, pp: &PhysicalPlan, ctx: &ExecContext) -> Result<ResultStream> {
        let root = pp
            .downcast_ref::<PhysicalOp>()
            .ok_or_else(|| Error::Execute("physical plan has an unexpected type".into()))?;
        let runtime = Runtime { ctx };
        let frame = runtime.eval_plan(root)?;
        let rows = frame
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| {
                        value.result_value().ok_or_else(|| {
                            Error::Execute(format!(
                                "value {} cannot be represented in a result stream",
                                value.key()
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ResultStream {
            schema: frame.cols,
            rows,
        })
    }
}

/// Execution helper bundling the framework runtime context.
struct Runtime<'a> {
    ctx: &'a ExecContext,
}

impl<'a> Runtime<'a> {
    fn storage(&self) -> &dyn StorageEngine {
        &*self.ctx.storage
    }

    fn txn(&self) -> &TxnContext {
        &self.ctx.txn
    }

    fn schema(&self) -> &dyn GraphSchema {
        &*self.ctx.schema
    }

    fn algorithms(&self) -> &AlgorithmRegistry {
        &self.ctx.algorithms
    }

    // ---------------------------------------------------------------
    // Plan evaluation
    // ---------------------------------------------------------------

    fn eval_plan(&self, op: &PhysicalOp) -> Result<Frame> {
        match op {
            PhysicalOp::SingleRow => Ok(Frame::single_row()),
            PhysicalOp::Empty => Ok(Frame::empty()),
            PhysicalOp::Match {
                optional,
                pattern,
                predicate,
                input,
            } => {
                let frame = self.eval_plan(input)?;
                self.eval_match(&frame, *optional, pattern, predicate.as_ref())
            }
            PhysicalOp::Unwind { expr, var, input } => {
                let frame = self.eval_plan(input)?;
                self.eval_unwind(&frame, expr, var)
            }
            PhysicalOp::Project {
                is_return,
                distinct,
                items,
                order,
                skip,
                limit,
                predicate,
                input,
            } => {
                let frame = self.eval_plan(input)?;
                self.eval_project(
                    &frame,
                    *is_return,
                    *distinct,
                    items,
                    order,
                    skip.as_ref(),
                    limit.as_ref(),
                    predicate.as_ref(),
                )
            }
            PhysicalOp::Create { pattern, input } => {
                let frame = self.eval_plan(input)?;
                self.eval_create(&frame, pattern)
            }
            PhysicalOp::Merge {
                pattern,
                on_create,
                on_match,
                input,
            } => {
                let frame = self.eval_plan(input)?;
                self.eval_merge(&frame, pattern, on_create, on_match)
            }
            PhysicalOp::Set { items, input } => {
                let frame = self.eval_plan(input)?;
                self.eval_set(&frame, items)
            }
            PhysicalOp::Remove { items, input } => {
                let frame = self.eval_plan(input)?;
                self.eval_remove(&frame, items)
            }
            PhysicalOp::Delete {
                detach,
                exprs,
                input,
            } => {
                let frame = self.eval_plan(input)?;
                self.eval_delete(&frame, *detach, exprs)
            }
            PhysicalOp::Union { all, inputs } => self.eval_union(*all, inputs),
            PhysicalOp::BuildProjection { spec } => self.eval_build_projection(spec),
            PhysicalOp::RunAlgorithm { name, params, .. } => {
                let rows = self.run_algorithm(name, params)?;
                Ok(rows)
            }
            PhysicalOp::Yield { spec, input } => {
                let frame = self.eval_plan(input)?;
                self.eval_yield(&frame, spec)
            }
            PhysicalOp::Call {
                name,
                args,
                yields,
                input,
            } => {
                let frame = self.eval_plan(input)?;
                self.eval_call(&frame, name, args, yields.as_ref())
            }
        }
    }

    fn eval_union(&self, all: bool, inputs: &[PhysicalOp]) -> Result<Frame> {
        let mut frames = Vec::new();
        for input in inputs {
            frames.push(self.eval_plan(input)?);
        }
        let mut cols: Vec<String> = Vec::new();
        for frame in &frames {
            for col in &frame.cols {
                if !cols.contains(col) {
                    cols.push(col.clone());
                }
            }
        }
        let mut rows: Vec<Row> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        for frame in frames {
            for row in frame.rows {
                let mut normalised = vec![Value::Null; cols.len()];
                for (index, col) in frame.cols.iter().enumerate() {
                    if let Some(target) = cols.iter().position(|c| c == col) {
                        normalised[target] = row[index].clone();
                    }
                }
                if all {
                    rows.push(normalised);
                } else {
                    let key = frame_key(&normalised);
                    if seen.insert(key) {
                        rows.push(normalised);
                    }
                }
            }
        }
        Ok(Frame { cols, rows })
    }

    fn eval_build_projection(&self, spec: &ProjectionSpec) -> Result<Frame> {
        // Only used as the input of an algorithm call.
        let projection = CsrProjection::build(self.storage(), self.txn(), spec)?;
        let json = format!("projection:{}", projection.num_vertices());
        Ok(Frame {
            cols: vec!["__projection".into()],
            rows: vec![vec![Value::Text(json)]],
        })
    }

    fn run_algorithm(
        &self,
        name: &str,
        params: &BTreeMap<String, PropertyValue>,
    ) -> Result<Frame> {
        let algorithm = self.algorithms().algorithms.get(name).ok_or_else(|| {
            Error::Execute(format!("algorithm `{name}` is not registered"))
        })?;
        let spec = ProjectionSpec {
            vertex_labels: None,
            edge_types: None,
            direction: metagdb_core::l4_compute::algebra::ProjectionDirection::Outgoing,
            vertex_properties: None,
        };
        let projection = CsrProjection::build(self.storage(), self.txn(), &spec)?;
        let args = AlgorithmArgs {
            params: params.clone(),
        };
        let stream = algorithm.execute(&projection, &args)?;
        self.stream_to_frame(stream)
    }

    fn stream_to_frame(&self, stream: ResultStream) -> Result<Frame> {
        let rows = stream
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| self.result_to_value(value))
                    .collect::<Result<Row>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Frame {
            cols: stream.schema,
            rows,
        })
    }

    fn result_to_value(&self, value: &ResultValue) -> Result<Value> {
        Ok(match value {
            ResultValue::Null => Value::Null,
            ResultValue::Bool(v) => Value::Bool(*v),
            ResultValue::Int(v) => Value::Int(*v),
            ResultValue::Float(v) => Value::Float(*v),
            ResultValue::Text(v) => Value::Text(v.clone()),
            ResultValue::VertexRef(id) => match self.storage().get_vertex(*id, self.txn())? {
                Some(vertex) => Value::Vertex(vertex),
                None => Value::Null,
            },
            ResultValue::EdgeRef(id) => match self.storage().get_edge(*id, self.txn())? {
                Some(edge) => Value::Edge(edge),
                None => Value::Null,
            },
            ResultValue::Property(property) => match property {
                PropertyValue::Null => Value::Null,
                PropertyValue::Bool(v) => Value::Bool(*v),
                PropertyValue::Int(v) => Value::Int(*v),
                PropertyValue::Float(v) => Value::Float(*v),
                PropertyValue::Text(v) => Value::Text(v.clone()),
            },
            ResultValue::List(items) => Value::List(
                items
                    .iter()
                    .map(|item| self.result_to_value(item))
                    .collect::<Result<Vec<_>>>()?,
            ),
            other => {
                return Err(Error::Execute(format!(
                    "algorithm result value {other:?} is outside the supported result model"
                )))
            }
        })
    }

    fn eval_yield(&self, frame: &Frame, spec: &YieldSpec) -> Result<Frame> {
        let mut cols = Vec::new();
        let mut mapping: Vec<(usize, String)> = Vec::new();
        if spec.star {
            cols = frame.cols.clone();
            mapping = frame
                .cols
                .iter()
                .enumerate()
                .map(|(index, col)| (index, col.clone()))
                .collect();
        } else {
            for (position, (field, alias)) in spec.items.iter().enumerate() {
                let source = frame.index(field).or_else(|| {
                    // A procedure may expose a field under a different name; fall
                    // back to the declared position so `YIELD node` binds the
                    // first output column.
                    if position < frame.cols.len() {
                        Some(position)
                    } else {
                        None
                    }
                });
                let source = source.ok_or_else(|| {
                    Error::Execute(format!(
                        "YIELD field `{field}` is not produced by the called procedure"
                    ))
                })?;
                mapping.push((source, alias.clone()));
                cols.push(alias.clone());
            }
        }
        let mut rows = Vec::with_capacity(frame.rows.len());
        for row in &frame.rows {
            let mut produced = vec![Value::Null; cols.len()];
            for (index, (source, _)) in mapping.iter().enumerate() {
                produced[index] = row.get(*source).cloned().unwrap_or(Value::Null);
            }
            rows.push(produced);
        }
        let mut frame = Frame { cols, rows };
        if let Some(predicate) = &spec.predicate {
            frame = self.filter_frame(&frame, predicate)?;
        }
        Ok(frame)
    }

    fn eval_call(
        &self,
        frame: &Frame,
        name: &str,
        args: &[ProcArg],
        yields: Option<&YieldSpec>,
    ) -> Result<Frame> {
        let params = call_params(self.schema(), name, args, &frame.cols, frame.rows.first())?;
        let mut produced = Vec::new();
        for row in &frame.rows {
            let output = self.run_algorithm(name, &params)?;
            let output = match yields {
                Some(spec) => self.eval_yield(&output, spec)?,
                None => output,
            };
            for result_row in output.rows {
                let mut combined = row.clone();
                let mut cols = frame.cols.clone();
                for (index, col) in output.cols.iter().enumerate() {
                    match cols.iter().position(|existing| existing == col) {
                        Some(target) => combined[target] = result_row[index].clone(),
                        None => {
                            cols.push(col.clone());
                            combined.push(result_row[index].clone());
                        }
                    }
                }
                produced.push(RowBuilder {
                    cols,
                    values: combined,
                });
            }
        }
        Ok(builders_to_frame(produced))
    }

    // ---------------------------------------------------------------
    // Pattern matching
    // ---------------------------------------------------------------

    fn eval_match(
        &self,
        frame: &Frame,
        optional: bool,
        pattern: &Pattern,
        predicate: Option<&Expr>,
    ) -> Result<Frame> {
        let mut output: Vec<RowBuilder> = Vec::new();
        for row in &frame.rows {
            let mut builders = vec![RowBuilder::from_row(&frame.cols, row)];
            for part in &pattern.parts {
                let mut next = Vec::new();
                for builder in builders {
                    next.extend(self.match_part(&builder, part)?);
                }
                builders = next;
            }
            let mut matched = Vec::new();
            for builder in builders {
                if let Some(predicate) = predicate {
                    let value = self.eval_expr(predicate, &builder.cols, &builder.values)?;
                    if !value.is_true() {
                        continue;
                    }
                }
                matched.push(builder);
            }
            if matched.is_empty() && optional {
                let mut builder = RowBuilder::from_row(&frame.cols, row);
                for variable in pattern_variables(pattern) {
                    builder.set(&variable, Value::Null);
                }
                matched.push(builder);
            }
            output.extend(matched);
        }
        let mut fallback = frame.cols.clone();
        for variable in pattern_variables(pattern) {
            if !fallback.contains(&variable) {
                fallback.push(variable);
            }
        }
        Ok(builders_to_frame_with(output, fallback))
    }

    fn match_part(&self, builder: &RowBuilder, part: &PatternPart) -> Result<Vec<RowBuilder>> {
        let mut results = Vec::new();
        for (start, node) in self.node_candidates(builder, &part.element.node)? {
            let mut path_nodes = vec![node.clone()];
            let mut path_edges = Vec::new();
            let expansions = self.expand_chain(
                &start,
                &part.element.chains,
                0,
                &node,
                &mut path_nodes,
                &mut path_edges,
            )?;
            for (mut result, nodes, edges) in expansions {
                if let Some(path_var) = &part.path_var {
                    result.set(path_var, Value::Path(nodes, edges));
                }
                results.push(result);
            }
        }
        Ok(results)
    }

    #[allow(clippy::too_many_arguments)]
    fn expand_chain(
        &self,
        builder: &RowBuilder,
        chains: &[(EdgePattern, NodePattern)],
        index: usize,
        current: &Value,
        path_nodes: &mut Vec<Value>,
        path_edges: &mut Vec<Value>,
    ) -> Result<Vec<(RowBuilder, Vec<Value>, Vec<Value>)>> {
        if index == chains.len() {
            return Ok(vec![(
                builder.clone(),
                path_nodes.clone(),
                path_edges.clone(),
            )]);
        }
        let (edge_pattern, node_pattern) = &chains[index];
        let current_vertex = match current {
            Value::Vertex(vertex) => vertex.clone(),
            _ => return Ok(Vec::new()),
        };
        let mut results = Vec::new();
        for (edges, targets) in
            self.hop_paths(&current_vertex, edge_pattern, &builder.cols, &builder.values)?
        {
            let Some(target) = targets.last().cloned() else {
                continue;
            };
            let mut candidate = builder.clone();
            if let Some(edge_var) = &edge_pattern.var {
                if edge_pattern.range.is_some() {
                    candidate.set(
                        edge_var,
                        Value::List(edges.iter().cloned().map(Value::Edge).collect()),
                    );
                } else if let Some(first) = edges.first() {
                    candidate.set(edge_var, Value::Edge(first.clone()));
                }
            }
            let target_value = Value::Vertex(target.clone());
            let mut bound = candidate.clone();
            if let Some(var) = &node_pattern.var {
                match candidate.get(var) {
                    // A variable already bound earlier in the pattern must
                    // denote exactly this vertex.
                    Some(Value::Null) => continue,
                    Some(Value::Vertex(existing)) => {
                        if existing.id != target.id {
                            continue;
                        }
                    }
                    Some(_) => continue,
                    None => {
                        if !self.node_matches(&target, node_pattern, &candidate)? {
                            continue;
                        }
                        bound.set(var, target_value.clone());
                    }
                }
            } else if !self.node_matches(&target, node_pattern, &candidate)? {
                continue;
            }
            let mut next_nodes = path_nodes.clone();
            let mut next_edges = path_edges.clone();
            for (offset, edge) in edges.iter().enumerate() {
                next_edges.push(Value::Edge(edge.clone()));
                if let Some(node) = targets.get(offset) {
                    next_nodes.push(Value::Vertex(node.clone()));
                }
            }
            // Restore the caller's path accumulators for the next candidate.
            results.extend(self.expand_chain(
                &bound,
                chains,
                index + 1,
                &target_value,
                &mut next_nodes,
                &mut next_edges,
            )?);
        }
        Ok(results)
    }

    /// Enumerate relationship paths for one chain step.
    fn hop_paths(
        &self,
        start: &VertexRecord,
        pattern: &EdgePattern,
        cols: &[String],
        values: &[Value],
    ) -> Result<Vec<(Vec<EdgeRecord>, Vec<VertexRecord>)>> {
        let types = self.resolve_edge_types(&pattern.types);
        let (low, high) = match &pattern.range {
            None => (1usize, 1usize),
            Some(range) => {
                let low = range.start.unwrap_or(1) as usize;
                let high = range.end.map(|value| value as usize).unwrap_or(low + 8).max(low);
                (low, high.min(low.max(1) + 16))
            }
        };
        let filter = EdgeFilter {
            types: types.clone(),
        };
        let mut results = Vec::new();
        if low == 0 {
            // A zero-length walk reaches the start vertex itself.
            results.push((Vec::new(), vec![start.clone()]));
        }
        let mut current_path = Vec::new();
        let mut used = HashSet::new();
        self.walk(
            start,
            pattern,
            &filter,
            cols,
            values,
            low,
            high,
            &mut current_path,
            &mut used,
            &mut results,
        )?;
        Ok(results)
    }

    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        node: &VertexRecord,
        pattern: &EdgePattern,
        filter: &EdgeFilter,
        cols: &[String],
        values: &[Value],
        low: usize,
        high: usize,
        current: &mut Vec<(EdgeRecord, VertexRecord)>,
        used: &mut HashSet<u64>,
        out: &mut Vec<(Vec<EdgeRecord>, Vec<VertexRecord>)>,
    ) -> Result<()> {
        if current.len() >= high {
            return Ok(());
        }
        for edge in self.incident_edges(node.id, pattern.direction, filter)? {
            if used.contains(&edge.id.0) {
                continue;
            }
            // Inline relationship properties constrain the edge, not the
            // node it points at.
            if !self.properties_match(&edge.properties, &pattern.properties, cols, values)? {
                continue;
            }
            let other = if edge.src == node.id { edge.dst } else { edge.src };
            let Some(neighbor) = self.storage().get_vertex(other, self.txn())? else {
                continue;
            };
            used.insert(edge.id.0);
            current.push((edge.clone(), neighbor.clone()));
            if current.len() >= low {
                out.push((
                    current.iter().map(|(edge, _)| edge.clone()).collect(),
                    current.iter().map(|(_, node)| node.clone()).collect(),
                ));
            }
            self.walk(&neighbor, pattern, filter, cols, values, low, high, current, used, out)?;
            current.pop();
            used.remove(&edge.id.0);
        }
        Ok(())
    }

    fn incident_edges(
        &self,
        vertex: VertexId,
        direction: Direction,
        filter: &EdgeFilter,
    ) -> Result<Vec<EdgeRecord>> {
        let mut edges = Vec::new();
        if matches!(direction, Direction::Outgoing | Direction::Both) {
            edges.extend(self.storage().out_edges(vertex, filter.clone(), self.txn())?);
        }
        if matches!(direction, Direction::Incoming | Direction::Both) {
            edges.extend(self.storage().in_edges(vertex, filter.clone(), self.txn())?);
        }
        Ok(edges)
    }

    fn node_candidates(
        &self,
        builder: &RowBuilder,
        pattern: &NodePattern,
    ) -> Result<Vec<(RowBuilder, Value)>> {
        if let Some(var) = &pattern.var {
            if let Some(existing) = builder.get(var) {
                if existing.is_null() {
                    return Ok(Vec::new());
                }
                let vertex = match existing {
                    Value::Vertex(vertex) => vertex.clone(),
                    _ => return Ok(Vec::new()),
                };
                if !self.node_matches(&vertex, pattern, builder)? {
                    return Ok(Vec::new());
                }
                return Ok(vec![(builder.clone(), Value::Vertex(vertex))]);
            }
        }
        let mut candidates: Vec<VertexRecord> = Vec::new();
        if let Some(hint) = pattern.id_hint {
            if let Some(vertex) = self.storage().get_vertex(VertexId(hint), self.txn())? {
                candidates.push(vertex);
            }
        } else {
            let filter = VertexFilter {
                labels: self.resolve_labels(&pattern.labels),
            };
            candidates.extend(self.storage().scan_vertices(filter, self.txn())?);
        }
        let mut out = Vec::new();
        for vertex in candidates {
            if !self.node_matches(&vertex, pattern, builder)? {
                continue;
            }
            let mut bound = builder.clone();
            if let Some(var) = &pattern.var {
                bound.set(var, Value::Vertex(vertex.clone()));
            }
            out.push((bound, Value::Vertex(vertex)));
        }
        Ok(out)
    }

    fn node_matches(
        &self,
        vertex: &VertexRecord,
        pattern: &NodePattern,
        builder: &RowBuilder,
    ) -> Result<bool> {
        if let Some(hint) = pattern.id_hint {
            if vertex.id.0 != hint {
                return Ok(false);
            }
        }
        for label in &pattern.labels {
            let Some(id) = self.schema().label_by_name(label) else {
                return Ok(false);
            };
            if !vertex.labels.contains(&id) {
                return Ok(false);
            }
        }
        self.properties_match(
            &vertex.properties,
            &pattern.properties,
            &builder.cols,
            &builder.values,
        )
    }

    fn properties_match(
        &self,
        properties: &PropertyMap,
        pattern: &Option<Expr>,
        cols: &[String],
        values: &[Value],
    ) -> Result<bool> {
        let Some(Expr::Map(entries)) = pattern else {
            return Ok(true);
        };
        for (key, expr) in entries {
            let Some(property_key) = self.schema().property_key_by_name(key) else {
                return Ok(false);
            };
            let expected = self.eval_expr(expr, cols, values)?;
            let actual = properties.get(&property_key).cloned().unwrap_or(PropertyValue::Null);
            let actual = match actual {
                PropertyValue::Null => Value::Null,
                PropertyValue::Bool(v) => Value::Bool(v),
                PropertyValue::Int(v) => Value::Int(v),
                PropertyValue::Float(v) => Value::Float(v),
                PropertyValue::Text(v) => Value::Text(v),
            };
            if expected.equals(&actual) != Some(true) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn resolve_labels(&self, labels: &[String]) -> Option<Vec<LabelId>> {
        if labels.is_empty() {
            return None;
        }
        let mut ids = Vec::new();
        for label in labels {
            if let Some(id) = self.schema().label_by_name(label) {
                ids.push(id);
            }
        }
        Some(ids)
    }

    fn resolve_edge_types(&self, types: &[String]) -> Option<Vec<EdgeTypeId>> {
        if types.is_empty() {
            return None;
        }
        let mut ids = Vec::new();
        for name in types {
            if let Some(id) = self.schema().edge_type_by_name(name) {
                ids.push(id);
            }
        }
        Some(ids)
    }

    // ---------------------------------------------------------------
    // Projection, aggregation, ordering
    // ---------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    fn eval_project(
        &self,
        frame: &Frame,
        _is_return: bool,
        distinct: bool,
        items: &[ReturnItem],
        order: &[SortItem],
        skip: Option<&Expr>,
        limit: Option<&Expr>,
        predicate: Option<&Expr>,
    ) -> Result<Frame> {
        let effective = expand_star(items, frame);
        let output_names: Vec<String> = effective
            .iter()
            .map(|item| item_name(item, &frame.cols))
            .collect();
        let aggregated = effective.iter().any(|item| item.expr.is_aggregate());

        // (output values, combined row used for ORDER BY / WHERE)
        let mut produced: Vec<(Row, RowBuilder)> = Vec::new();
        if aggregated {
            let groups = group_rows(frame, &effective, self)?;
            for (representative, members) in groups {
                let mut values = Vec::with_capacity(effective.len());
                for item in &effective {
                    if item.expr.is_aggregate() {
                        let substituted = self.substitute_aggregates(&item.expr, &members)?;
                        values.push(self.eval_expr(&substituted, &frame.cols, &representative)?);
                    } else {
                        values.push(self.eval_expr(&item.expr, &frame.cols, &representative)?);
                    }
                }
                let combined =
                    combine(&frame.cols, &representative, &output_names, &values);
                produced.push((values, combined));
            }
        } else {
            for row in &frame.rows {
                let mut values = Vec::with_capacity(effective.len());
                for item in &effective {
                    values.push(self.eval_expr(&item.expr, &frame.cols, row)?);
                }
                let combined = combine(&frame.cols, row, &output_names, &values);
                produced.push((values, combined));
            }
        }

        if distinct {
            let mut seen = HashSet::new();
            produced.retain(|(values, _)| seen.insert(frame_key(values)));
        }

        if let Some(predicate) = predicate {
            let mut kept = Vec::new();
            for (values, combined) in produced {
                let result = self.eval_expr(predicate, &combined.cols, &combined.values)?;
                if result.is_true() {
                    kept.push((values, combined));
                }
            }
            produced = kept;
        }

        if !order.is_empty() {
            let mut decorated: Vec<(Vec<Value>, (Row, RowBuilder))> = Vec::new();
            for entry in produced {
                let mut keys = Vec::with_capacity(order.len());
                for sort in order {
                    let value = self.eval_expr(&sort.expr, &entry.1.cols, &entry.1.values)?;
                    keys.push(if sort.descending {
                        negate_for_order(value)
                    } else {
                        value
                    });
                }
                decorated.push((keys, entry));
            }
            decorated.sort_by(|a, b| compare_keys(&a.0, &b.0));
            produced = decorated.into_iter().map(|(_, entry)| entry).collect();
        }

        let skip_count = match skip {
            Some(expr) => self.eval_expr(expr, &frame.cols, &[]).ok().and_then(|v| v.as_int()).unwrap_or(0).max(0) as usize,
            None => 0,
        };
        let limit_count = match limit {
            Some(expr) => self
                .eval_expr(expr, &frame.cols, &[])
                .ok()
                .and_then(|v| v.as_int())
                .map(|value| value.max(0) as usize),
            None => None,
        };

        let rows: Vec<Row> = produced
            .into_iter()
            .skip(skip_count)
            .take(limit_count.unwrap_or(usize::MAX))
            .map(|(values, _)| values)
            .collect();
        Ok(Frame {
            cols: output_names,
            rows,
        })
    }

    /// Replace every aggregate call with its computed literal value.
    fn substitute_aggregates(
        &self,
        expr: &Expr,
        group: &[(Vec<String>, Row)],
    ) -> Result<Expr> {
        match expr {
            Expr::CountAll => Ok(Expr::Int(group.len() as i64)),
            Expr::Function {
                name,
                distinct,
                args,
            } if is_aggregate_name(name) => {
                let value = self.compute_aggregate(name, *distinct, args, group)?;
                value_to_literal(&value).ok_or_else(|| {
                    Error::Execute(format!(
                        "aggregate `{name}` produced a value that cannot be re-evaluated"
                    ))
                })
            }
            Expr::Property(base, key) => Ok(Expr::Property(
                Box::new(self.substitute_aggregates(base, group)?),
                key.clone(),
            )),
            Expr::Index(a, b) => Ok(Expr::Index(
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::Slice { base, start, end } => Ok(Expr::Slice {
                base: Box::new(self.substitute_aggregates(base, group)?),
                start: start
                    .as_ref()
                    .map(|value| self.substitute_aggregates(value, group).map(Box::new))
                    .transpose()?,
                end: end
                    .as_ref()
                    .map(|value| self.substitute_aggregates(value, group).map(Box::new))
                    .transpose()?,
            }),
            Expr::Negate(inner) => Ok(Expr::Negate(Box::new(
                self.substitute_aggregates(inner, group)?,
            ))),
            Expr::Not(inner) => Ok(Expr::Not(Box::new(
                self.substitute_aggregates(inner, group)?,
            ))),
            Expr::IsNull(inner) => Ok(Expr::IsNull(Box::new(
                self.substitute_aggregates(inner, group)?,
            ))),
            Expr::IsNotNull(inner) => Ok(Expr::IsNotNull(Box::new(
                self.substitute_aggregates(inner, group)?,
            ))),
            Expr::HasLabels(inner, labels) => Ok(Expr::HasLabels(
                Box::new(self.substitute_aggregates(inner, group)?),
                labels.clone(),
            )),
            Expr::Binary(op, a, b) => Ok(Expr::Binary(
                *op,
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::In(a, b) => Ok(Expr::In(
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::StartsWith(a, b) => Ok(Expr::StartsWith(
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::EndsWith(a, b) => Ok(Expr::EndsWith(
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::Contains(a, b) => Ok(Expr::Contains(
                Box::new(self.substitute_aggregates(a, group)?),
                Box::new(self.substitute_aggregates(b, group)?),
            )),
            Expr::List(items) => Ok(Expr::List(
                items
                    .iter()
                    .map(|item| self.substitute_aggregates(item, group))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Map(entries) => Ok(Expr::Map(
                entries
                    .iter()
                    .map(|(key, value)| {
                        Ok((key.clone(), self.substitute_aggregates(value, group)?))
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Case {
                subject,
                alternatives,
                default,
            } => Ok(Expr::Case {
                subject: subject
                    .as_ref()
                    .map(|value| self.substitute_aggregates(value, group).map(Box::new))
                    .transpose()?,
                alternatives: alternatives
                    .iter()
                    .map(|(when, then)| {
                        Ok((
                            self.substitute_aggregates(when, group)?,
                            self.substitute_aggregates(then, group)?,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
                default: default
                    .as_ref()
                    .map(|value| self.substitute_aggregates(value, group).map(Box::new))
                    .transpose()?,
            }),
            Expr::Function {
                name,
                distinct,
                args,
            } => Ok(Expr::Function {
                name: name.clone(),
                distinct: *distinct,
                args: args
                    .iter()
                    .map(|arg| self.substitute_aggregates(arg, group))
                    .collect::<Result<Vec<_>>>()?,
            }),
            other => Ok(other.clone()),
        }
    }

    fn compute_aggregate(
        &self,
        name: &str,
        distinct: bool,
        args: &[Expr],
        group: &[(Vec<String>, Row)],
    ) -> Result<Value> {
        let mut values = Vec::new();
        let mut seen = HashSet::new();
        for (cols, row) in group {
            if args.is_empty() {
                values.push(Value::Int(1));
                continue;
            }
            let value = self.eval_expr(&args[0], cols, row)?;
            if value.is_null() {
                continue;
            }
            if distinct && !seen.insert(value.key()) {
                continue;
            }
            values.push(value);
        }
        Ok(match name.to_ascii_lowercase().as_str() {
            "count" => Value::Int(values.len() as i64),
            "sum" => {
                let mut int_total = 0i64;
                let mut float_total = 0f64;
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
                        _ => {
                            return Err(Error::Execute(
                                "sum requires numeric values".into(),
                            ))
                        }
                    }
                }
                if is_float {
                    Value::Float(float_total)
                } else {
                    Value::Int(int_total)
                }
            }
            "avg" => {
                if values.is_empty() {
                    Value::Null
                } else {
                    let total: f64 = values
                        .iter()
                        .map(|value| {
                            value.as_float().ok_or_else(|| {
                                Error::Execute("avg requires numeric values".into())
                            })
                        })
                        .collect::<Result<Vec<_>>>()?
                        .iter()
                        .sum();
                    Value::Float(total / values.len() as f64)
                }
            }
            "min" => values
                .into_iter()
                .reduce(|a, b| match a.compare(&b) {
                    Some(std::cmp::Ordering::Greater) => b,
                    _ => a,
                })
                .unwrap_or(Value::Null),
            "max" => values
                .into_iter()
                .reduce(|a, b| match a.compare(&b) {
                    Some(std::cmp::Ordering::Less) => b,
                    _ => a,
                })
                .unwrap_or(Value::Null),
            "collect" => Value::List(values),
            other => {
                return Err(Error::Unsupported(format!(
                    "aggregate function `{other}` is outside the core language profile"
                )))
            }
        })
    }

    fn eval_unwind(&self, frame: &Frame, expr: &Expr, var: &str) -> Result<Frame> {
        let mut builders = Vec::new();
        for row in &frame.rows {
            let value = self.eval_expr(expr, &frame.cols, row)?;
            match value {
                Value::List(items) => {
                    for item in items {
                        let mut builder = RowBuilder::from_row(&frame.cols, row);
                        builder.set(var, item);
                        builders.push(builder);
                    }
                }
                Value::Null => {}
                other => {
                    let mut builder = RowBuilder::from_row(&frame.cols, row);
                    builder.set(var, other);
                    builders.push(builder);
                }
            }
        }
        let mut fallback = frame.cols.clone();
        if !fallback.iter().any(|col| col == var) {
            fallback.push(var.to_string());
        }
        Ok(builders_to_frame_with(builders, fallback))
    }

    fn filter_frame(&self, frame: &Frame, predicate: &Expr) -> Result<Frame> {
        let mut rows = Vec::new();
        for row in &frame.rows {
            let value = self.eval_expr(predicate, &frame.cols, row)?;
            if value.is_true() {
                rows.push(row.clone());
            }
        }
        Ok(Frame {
            cols: frame.cols.clone(),
            rows,
        })
    }

    // ---------------------------------------------------------------
    // Mutations
    // ---------------------------------------------------------------

    fn eval_create(&self, frame: &Frame, pattern: &Pattern) -> Result<Frame> {
        let mut builders = Vec::new();
        for row in &frame.rows {
            let mut builder = RowBuilder::from_row(&frame.cols, row);
            for part in &pattern.parts {
                let mut current = self.create_node(&mut builder, &part.element.node)?;
                for (edge_pattern, node_pattern) in &part.element.chains {
                    let target = self.create_node(&mut builder, node_pattern)?;
                    let edge = self.build_edge(&builder, edge_pattern, &current, &target)?;
                    let id = self.storage().insert_edge(edge, self.txn())?;
                    if let Some(var) = &edge_pattern.var {
                        let record = self
                            .storage()
                            .get_edge(id, self.txn())?
                            .ok_or_else(|| Error::Execute("created edge disappeared".into()))?;
                        builder.set(var, Value::Edge(record));
                    }
                    current = target;
                }
            }
            self.refresh_row(&mut builder)?;
            builders.push(builder);
        }
        Ok(builders_to_frame(builders))
    }

    fn create_node(&self, builder: &mut RowBuilder, pattern: &NodePattern) -> Result<VertexRecord> {
        if let Some(var) = &pattern.var {
            if let Some(existing) = builder.get(var).cloned() {
                if let Value::Vertex(vertex) = existing {
                    return Ok(vertex);
                }
                if !existing.is_null() {
                    return Err(Error::Execute(format!(
                        "CREATE cannot reuse `{var}` because it is not a vertex"
                    )));
                }
            }
        }
        let mut labels = Vec::new();
        for label in &pattern.labels {
            labels.push(
                self.schema()
                    .label_by_name(label)
                    .ok_or_else(|| Error::Schema(format!("unknown label `{label}`")))?,
            );
        }
        let properties = self.evaluate_property_map(
            &pattern.properties,
            &builder.cols,
            &builder.values,
        )?;
        let id = self.storage().insert_vertex(
            VertexRecord {
                id: VertexId(0),
                labels,
                properties,
            },
            self.txn(),
        )?;
        let record = self
            .storage()
            .get_vertex(id, self.txn())?
            .ok_or_else(|| Error::Execute("created vertex disappeared".into()))?;
        if let Some(var) = &pattern.var {
            builder.set(var, Value::Vertex(record.clone()));
        }
        Ok(record)
    }

    fn build_edge(
        &self,
        builder: &RowBuilder,
        pattern: &EdgePattern,
        src: &VertexRecord,
        dst: &VertexRecord,
    ) -> Result<EdgeRecord> {
        if pattern.range.is_some() {
            return Err(Error::Unsupported(
                "CREATE with a variable-length relationship is not supported".into(),
            ));
        }
        let etype = match pattern.types.first() {
            Some(name) => self
                .schema()
                .edge_type_by_name(name)
                .ok_or_else(|| Error::Schema(format!("unknown edge type `{name}`")))?,
            None => EdgeTypeId(0),
        };
        let (src, dst) = match pattern.direction {
            Direction::Incoming => (dst.id, src.id),
            _ => (src.id, dst.id),
        };
        Ok(EdgeRecord {
            id: EdgeId(0),
            src,
            dst,
            etype,
            properties: self.evaluate_property_map(
                &pattern.properties,
                &builder.cols,
                &builder.values,
            )?,
        })
    }

    fn evaluate_property_map(
        &self,
        pattern: &Option<Expr>,
        cols: &[String],
        values: &[Value],
    ) -> Result<PropertyMap> {
        let mut map = PropertyMap::new();
        let Some(Expr::Map(entries)) = pattern else {
            return Ok(map);
        };
        for (key, expr) in entries {
            let key = self
                .schema()
                .property_key_by_name(key)
                .ok_or_else(|| Error::Schema(format!("unknown property key `{key}`")))?;
            let value = self.eval_expr(expr, cols, values)?;
            map.insert(key, value_to_property(&value)?);
        }
        Ok(map)
    }

    fn eval_merge(
        &self,
        frame: &Frame,
        pattern: &Pattern,
        on_create: &[SetItem],
        on_match: &[SetItem],
    ) -> Result<Frame> {
        let mut builders = Vec::new();
        for row in &frame.rows {
            let builder = RowBuilder::from_row(&frame.cols, row);
            let mut matches = vec![builder.clone()];
            for part in &pattern.parts {
                let mut next = Vec::new();
                for candidate in matches {
                    next.extend(self.match_part(&candidate, part)?);
                }
                matches = next;
            }
            let mut chosen = if let Some(first) = matches.into_iter().next() {
                let mut updated = first;
                self.apply_set(&mut updated, on_match)?;
                updated
            } else {
                let mut created = vec![builder];
                for part in &pattern.parts {
                    let mut next = Vec::new();
                    for mut candidate in created {
                        let mut current = self.create_node(&mut candidate, &part.element.node)?;
                        for (edge_pattern, node_pattern) in &part.element.chains {
                            let target = self.create_node(&mut candidate, node_pattern)?;
                            let edge =
                                self.build_edge(&candidate, edge_pattern, &current, &target)?;
                            let id = self.storage().insert_edge(edge, self.txn())?;
                            if let Some(var) = &edge_pattern.var {
                                let record = self.storage().get_edge(id, self.txn())?.ok_or_else(
                                    || Error::Execute("created edge disappeared".into()),
                                )?;
                                candidate.set(var, Value::Edge(record));
                            }
                            current = target;
                        }
                        next.push(candidate);
                    }
                    created = next;
                }
                let mut created = created.into_iter().next().unwrap_or_default();
                self.apply_set(&mut created, on_create)?;
                created
            };
            self.refresh_row(&mut chosen)?;
            builders.push(chosen);
        }
        Ok(builders_to_frame(builders))
    }

    fn eval_set(&self, frame: &Frame, items: &[SetItem]) -> Result<Frame> {
        let mut builders = Vec::new();
        for row in &frame.rows {
            let mut builder = RowBuilder::from_row(&frame.cols, row);
            self.apply_set(&mut builder, items)?;
            self.refresh_row(&mut builder)?;
            builders.push(builder);
        }
        Ok(builders_to_frame(builders))
    }

    fn apply_set(&self, builder: &mut RowBuilder, items: &[SetItem]) -> Result<()> {
        for item in items {
            match item {
                SetItem::Property { target, key, value } => {
                    let value = self.eval_expr(value, &builder.cols, &builder.values)?;
                    let property_key = self
                        .schema()
                        .property_key_by_name(key)
                        .ok_or_else(|| Error::Schema(format!("unknown property key `{key}`")))?;
                    let property = value_to_property(&value)?;
                    match self.eval_expr(target, &builder.cols, &builder.values)? {
                        Value::Vertex(mut vertex) => {
                            if property == PropertyValue::Null {
                                vertex.properties.remove(&property_key);
                            } else {
                                vertex.properties.insert(property_key, property);
                            }
                            self.storage().update_vertex(vertex, self.txn())?;
                        }
                        Value::Edge(mut edge) => {
                            if property == PropertyValue::Null {
                                edge.properties.remove(&property_key);
                            } else {
                                edge.properties.insert(property_key, property);
                            }
                            self.storage().update_edge(edge, self.txn())?;
                        }
                        other => {
                            return Err(Error::Execute(format!(
                                "SET target must be a vertex or an edge, found {}",
                                other.key()
                            )))
                        }
                    }
                }
                SetItem::Labels { var, labels } => {
                    let Value::Vertex(mut vertex) = self
                        .eval_expr(&Expr::Variable(var.clone()), &builder.cols, &builder.values)?
                    else {
                        return Err(Error::Execute(format!(
                            "SET label target `{var}` is not a vertex"
                        )));
                    };
                    for label in labels {
                        let id = self
                            .schema()
                            .label_by_name(label)
                            .ok_or_else(|| Error::Schema(format!("unknown label `{label}`")))?;
                        if !vertex.labels.contains(&id) {
                            vertex.labels.push(id);
                        }
                    }
                    self.storage().update_vertex(vertex, self.txn())?;
                }
            }
            // Re-read the mutated element so a following item in the same SET
            // clause builds on the value the previous item just wrote.
            self.refresh_row(builder)?;
        }
        Ok(())
    }

    fn eval_remove(&self, frame: &Frame, items: &[RemoveItem]) -> Result<Frame> {
        let mut builders = Vec::new();
        for row in &frame.rows {
            let mut builder = RowBuilder::from_row(&frame.cols, row);
            for item in items {
                match item {
                    RemoveItem::Property { target, key } => {
                        let property_key = self.schema().property_key_by_name(key).ok_or_else(
                            || Error::Schema(format!("unknown property key `{key}`")),
                        )?;
                        match self.eval_expr(target, &builder.cols, &builder.values)? {
                            Value::Vertex(mut vertex) => {
                                vertex.properties.remove(&property_key);
                                self.storage().update_vertex(vertex, self.txn())?;
                            }
                            Value::Edge(mut edge) => {
                                edge.properties.remove(&property_key);
                                self.storage().update_edge(edge, self.txn())?;
                            }
                            other => {
                                return Err(Error::Execute(format!(
                                    "REMOVE target must be a vertex or edge, found {}",
                                    other.key()
                                )))
                            }
                        }
                    }
                    RemoveItem::Labels { var, labels } => {
                        let Value::Vertex(mut vertex) = self.eval_expr(
                            &Expr::Variable(var.clone()),
                            &builder.cols,
                            &builder.values,
                        )? else {
                            return Err(Error::Execute(format!(
                                "REMOVE label target `{var}` is not a vertex"
                            )));
                        };
                        for label in labels {
                            if let Some(id) = self.schema().label_by_name(label) {
                                vertex.labels.retain(|existing| *existing != id);
                            }
                        }
                        self.storage().update_vertex(vertex, self.txn())?;
                    }
                }
            }
            self.refresh_row(&mut builder)?;
            builders.push(builder);
        }
        Ok(builders_to_frame(builders))
    }

    fn eval_delete(&self, frame: &Frame, detach: bool, exprs: &[Expr]) -> Result<Frame> {
        for row in &frame.rows {
            for expr in exprs {
                match self.eval_expr(expr, &frame.cols, row)? {
                    Value::Vertex(vertex) => {
                        if detach {
                            let edges: Vec<EdgeRecord> = self
                                .storage()
                                .out_edges(vertex.id, EdgeFilter::default(), self.txn())?
                                .chain(
                                    self.storage().in_edges(
                                        vertex.id,
                                        EdgeFilter::default(),
                                        self.txn(),
                                    )?,
                                )
                                .collect();
                            for edge in edges {
                                if self.storage().get_edge(edge.id, self.txn())?.is_some() {
                                    self.storage().delete_edge(edge.id, self.txn())?;
                                }
                            }
                        }
                        self.storage().delete_vertex(vertex.id, self.txn())?;
                    }
                    Value::Edge(edge) => {
                        self.storage().delete_edge(edge.id, self.txn())?;
                    }
                    Value::Null => {}
                    other => {
                        return Err(Error::Execute(format!(
                            "DELETE expects a vertex or an edge, found {}",
                            other.key()
                        )))
                    }
                }
            }
        }
        Ok(frame.clone())
    }

    /// Re-read the vertices and edges bound in a row after a mutation.
    fn refresh_row(&self, builder: &mut RowBuilder) -> Result<()> {
        for index in 0..builder.values.len() {
            let refreshed = match &builder.values[index] {
                Value::Vertex(vertex) => self
                    .storage()
                    .get_vertex(vertex.id, self.txn())?
                    .map(Value::Vertex),
                Value::Edge(edge) => self.storage().get_edge(edge.id, self.txn())?.map(Value::Edge),
                _ => None,
            };
            if let Some(value) = refreshed {
                builder.values[index] = value;
            }
        }
        Ok(())
    }

    // ---------------------------------------------------------------
    // Expression evaluation
    // ---------------------------------------------------------------

    fn eval_expr(&self, expr: &Expr, cols: &[String], values: &[Value]) -> Result<Value> {
        match expr {
            Expr::Null => Ok(Value::Null),
            Expr::Bool(value) => Ok(Value::Bool(*value)),
            Expr::Int(value) => Ok(Value::Int(*value)),
            Expr::Float(value) => Ok(Value::Float(*value)),
            Expr::Str(value) => Ok(Value::Text(value.clone())),
            Expr::Parameter(name) => Err(Error::Unsupported(format!(
                "query parameters are not bound in this runtime: ${name}"
            ))),
            Expr::Variable(name) => {
                if name == "*" {
                    return Ok(Value::Null);
                }
                let index = cols.iter().position(|col| col == name).ok_or_else(|| {
                    Error::Execute(format!("variable `{name}` is not in scope"))
                })?;
                Ok(values.get(index).cloned().unwrap_or(Value::Null))
            }
            Expr::List(items) => Ok(Value::List(
                items
                    .iter()
                    .map(|item| self.eval_expr(item, cols, values))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Map(entries) => Ok(Value::Map(
                entries
                    .iter()
                    .map(|(key, value)| {
                        Ok((key.clone(), self.eval_expr(value, cols, values)?))
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Property(base, key) => {
                let base = self.eval_expr(base, cols, values)?;
                self.property_of(&base, key)
            }
            Expr::Index(base, index) => {
                let base = self.eval_expr(base, cols, values)?;
                let index = self.eval_expr(index, cols, values)?;
                self.index_of(&base, &index)
            }
            Expr::Slice { base, start, end } => {
                let base = self.eval_expr(base, cols, values)?;
                let start = match start {
                    Some(expr) => self.eval_expr(expr, cols, values)?.as_int(),
                    None => None,
                };
                let end = match end {
                    Some(expr) => self.eval_expr(expr, cols, values)?.as_int(),
                    None => None,
                };
                self.slice_of(&base, start, end)
            }
            Expr::Negate(inner) => match self.eval_expr(inner, cols, values)? {
                Value::Null => Ok(Value::Null),
                Value::Int(value) => Ok(Value::Int(-value)),
                Value::Float(value) => Ok(Value::Float(-value)),
                other => Err(Error::Execute(format!(
                    "cannot negate {}",
                    other.key()
                ))),
            },
            Expr::Not(inner) => match self.eval_expr(inner, cols, values)? {
                Value::Null => Ok(Value::Null),
                Value::Bool(value) => Ok(Value::Bool(!value)),
                other => Err(Error::Execute(format!(
                    "NOT expects a boolean, found {}",
                    other.key()
                ))),
            },
            Expr::IsNull(inner) => Ok(Value::Bool(
                self.eval_expr(inner, cols, values)?.is_null(),
            )),
            Expr::IsNotNull(inner) => Ok(Value::Bool(
                !self.eval_expr(inner, cols, values)?.is_null(),
            )),
            Expr::Binary(op, a, b) => {
                let left = self.eval_expr(a, cols, values)?;
                let right = self.eval_expr(b, cols, values)?;
                self.binary(*op, &left, &right)
            }
            Expr::In(a, b) => {
                let needle = self.eval_expr(a, cols, values)?;
                let haystack = self.eval_expr(b, cols, values)?;
                match haystack {
                    Value::List(items) => {
                        let mut saw_null = false;
                        for item in &items {
                            match needle.equals(item) {
                                Some(true) => return Ok(Value::Bool(true)),
                                Some(false) => {}
                                None => saw_null = true,
                            }
                        }
                        if saw_null && needle.is_null() {
                            Ok(Value::Null)
                        } else {
                            Ok(Value::Bool(false))
                        }
                    }
                    other => Err(Error::Execute(format!(
                        "IN expects a list, found {}",
                        other.key()
                    ))),
                }
            }
            Expr::StartsWith(a, b)
            | Expr::EndsWith(a, b)
            | Expr::Contains(a, b) => {
                let left = self.eval_expr(a, cols, values)?;
                let right = self.eval_expr(b, cols, values)?;
                if left.is_null() || right.is_null() {
                    return Ok(Value::Null);
                }
                let (Some(left), Some(right)) = (left.as_text(), right.as_text()) else {
                    return Err(Error::Execute(
                        "string operators require text operands".into(),
                    ));
                };
                Ok(Value::Bool(match expr {
                    Expr::StartsWith(..) => left.starts_with(right),
                    Expr::EndsWith(..) => left.ends_with(right),
                    _ => left.contains(right),
                }))
            }
            Expr::HasLabels(base, labels) => {
                let base = self.eval_expr(base, cols, values)?;
                let Value::Vertex(vertex) = base else {
                    return Ok(Value::Bool(false));
                };
                for label in labels {
                    let Some(id) = self.schema().label_by_name(label) else {
                        return Ok(Value::Bool(false));
                    };
                    if !vertex.labels.contains(&id) {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(true))
            }
            Expr::PatternExists(pattern) => {
                let builder = RowBuilder::from_row(cols, values);
                let mut matches = vec![builder];
                for part in &pattern.parts {
                    let mut next = Vec::new();
                    for candidate in matches {
                        next.extend(self.match_part(&candidate, part)?);
                    }
                    matches = next;
                }
                Ok(Value::Bool(!matches.is_empty()))
            }
            Expr::ListComprehension {
                var,
                list,
                filter,
                map,
            } => {
                let list = self.eval_expr(list, cols, values)?;
                let items = match list {
                    Value::List(items) => items,
                    Value::Null => return Ok(Value::Null),
                    other => {
                        return Err(Error::Execute(format!(
                            "list comprehension expects a list, found {}",
                            other.key()
                        )))
                    }
                };
                let mut out = Vec::new();
                for item in items {
                    let mut inner_cols = cols.to_vec();
                    let mut inner_values = values.to_vec();
                    match inner_cols.iter().position(|col| col == var) {
                        Some(index) => inner_values[index] = item.clone(),
                        None => {
                            inner_cols.push(var.clone());
                            inner_values.push(item.clone());
                        }
                    }
                    if let Some(filter) = filter {
                        if !self.eval_expr(filter, &inner_cols, &inner_values)?.is_true() {
                            continue;
                        }
                    }
                    out.push(match map {
                        Some(map) => self.eval_expr(map, &inner_cols, &inner_values)?,
                        None => item,
                    });
                }
                Ok(Value::List(out))
            }
            Expr::Quantifier {
                kind,
                var,
                list,
                filter,
            } => {
                let list = self.eval_expr(list, cols, values)?;
                let items = match list {
                    Value::List(items) => items,
                    Value::Null => return Ok(Value::Null),
                    other => {
                        return Err(Error::Execute(format!(
                            "quantifier expects a list, found {}",
                            other.key()
                        )))
                    }
                };
                let total = items.len();
                let mut matches = 0usize;
                for item in items {
                    let mut inner_cols = cols.to_vec();
                    let mut inner_values = values.to_vec();
                    match inner_cols.iter().position(|col| col == var) {
                        Some(index) => inner_values[index] = item,
                        None => {
                            inner_cols.push(var.clone());
                            inner_values.push(item);
                        }
                    }
                    let satisfied = match filter {
                        Some(filter) => self.eval_expr(filter, &inner_cols, &inner_values)?.is_true(),
                        None => true,
                    };
                    if satisfied {
                        matches += 1;
                    }
                }
                Ok(Value::Bool(match kind {
                    Quantifier::All => matches == total,
                    Quantifier::Any => matches > 0,
                    Quantifier::None => matches == 0,
                    Quantifier::Single => matches == 1,
                }))
            }
            Expr::Case {
                subject,
                alternatives,
                default,
            } => {
                let subject = match subject {
                    Some(subject) => Some(self.eval_expr(subject, cols, values)?),
                    None => None,
                };
                for (when, then) in alternatives {
                    let matched = match &subject {
                        Some(subject) => {
                            let when = self.eval_expr(when, cols, values)?;
                            subject.equals(&when) == Some(true)
                        }
                        None => self.eval_expr(when, cols, values)?.is_true(),
                    };
                    if matched {
                        return self.eval_expr(then, cols, values);
                    }
                }
                match default {
                    Some(default) => self.eval_expr(default, cols, values),
                    None => Ok(Value::Null),
                }
            }
            Expr::CountAll => Err(Error::Execute(
                "COUNT(*) may only appear inside an aggregation".into(),
            )),
            Expr::Function {
                name,
                distinct,
                args,
            } => {
                if is_aggregate_name(name) {
                    return Err(Error::Execute(format!(
                        "aggregate function `{name}` may only appear in RETURN or WITH"
                    )));
                }
                let _ = distinct;
                self.call_function(name, args, cols, values)
            }
        }
    }

    fn binary(&self, op: BinOp, left: &Value, right: &Value) -> Result<Value> {
        use BinOp::*;
        match op {
            Add | Sub | Mul | Div | Mod | Pow => self.arithmetic(op, left, right),
            Eq | Neq | Lt | Lte | Gt | Gte => {
                if left.is_null() || right.is_null() {
                    return Ok(Value::Null);
                }
                let ordering = left.compare(right);
                Ok(match op {
                    Eq => Value::Bool(left.equals(right) == Some(true)),
                    Neq => Value::Bool(left.equals(right) == Some(false)),
                    Lt => Value::Bool(ordering == Some(std::cmp::Ordering::Less)),
                    Lte => Value::Bool(matches!(
                        ordering,
                        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                    )),
                    Gt => Value::Bool(ordering == Some(std::cmp::Ordering::Greater)),
                    Gte => Value::Bool(matches!(
                        ordering,
                        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
                    )),
                    _ => unreachable!(),
                })
            }
            And => Ok(and_values(left, right)),
            Or => Ok(or_values(left, right)),
            Xor => match (left, right) {
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a ^ b)),
                _ => Err(Error::Execute("XOR expects booleans".into())),
            },
        }
    }

    fn arithmetic(&self, op: BinOp, left: &Value, right: &Value) -> Result<Value> {
        if left.is_null() || right.is_null() {
            return Ok(Value::Null);
        }
        match (left, right) {
            (Value::List(items), other) => {
                let mut items = items.clone();
                match op {
                    BinOp::Add => {
                        match other {
                            Value::List(more) => items.extend(more.clone()),
                            value => items.push(value.clone()),
                        }
                        Ok(Value::List(items))
                    }
                    _ => Err(Error::Execute("unsupported list arithmetic".into())),
                }
            }
            (Value::Text(a), Value::Text(b)) if op == BinOp::Add => {
                Ok(Value::Text(format!("{a}{b}")))
            }
            (Value::Int(a), Value::Int(b)) => Ok(match op {
                BinOp::Add => Value::Int(a.wrapping_add(*b)),
                BinOp::Sub => Value::Int(a.wrapping_sub(*b)),
                BinOp::Mul => Value::Int(a.wrapping_mul(*b)),
                BinOp::Div => {
                    if *b == 0 {
                        Value::Null
                    } else {
                        Value::Int(a / b)
                    }
                }
                BinOp::Mod => {
                    if *b == 0 {
                        Value::Null
                    } else {
                        Value::Int(a % b)
                    }
                }
                BinOp::Pow => Value::Float((*a as f64).powf(*b as f64)),
                _ => unreachable!(),
            }),
            _ => {
                let a = left.as_float().ok_or_else(|| {
                    Error::Execute(format!("cannot use {} in arithmetic", left.key()))
                })?;
                let b = right.as_float().ok_or_else(|| {
                    Error::Execute(format!("cannot use {} in arithmetic", right.key()))
                })?;
                Ok(match op {
                    BinOp::Add => Value::Float(a + b),
                    BinOp::Sub => Value::Float(a - b),
                    BinOp::Mul => Value::Float(a * b),
                    BinOp::Div => Value::Float(a / b),
                    BinOp::Mod => Value::Float(a % b),
                    BinOp::Pow => Value::Float(a.powf(b)),
                    _ => unreachable!(),
                })
            }
        }
    }

    fn property_of(&self, base: &Value, key: &str) -> Result<Value> {
        match base {
            Value::Null => Ok(Value::Null),
            Value::Vertex(vertex) => Ok(property_lookup(&vertex.properties, key, self.schema())),
            Value::Edge(edge) => Ok(property_lookup(&edge.properties, key, self.schema())),
            Value::Map(entries) => Ok(entries
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value.clone())
                .unwrap_or(Value::Null)),
            Value::Date(days) => {
                let (year, month, day) = civil_from_days(*days);
                Ok(match key {
                    "year" => Value::Int(year),
                    "month" => Value::Int(month as i64),
                    "day" => Value::Int(day as i64),
                    other => {
                        return Err(Error::Execute(format!(
                            "date has no field `{other}`"
                        )))
                    }
                })
            }
            Value::DateTime(seconds) => {
                let (year, month, day) = civil_from_days(seconds.div_euclid(86_400));
                let rest = seconds.rem_euclid(86_400);
                Ok(match key {
                    "year" => Value::Int(year),
                    "month" => Value::Int(month as i64),
                    "day" => Value::Int(day as i64),
                    "hour" => Value::Int(rest / 3600),
                    "minute" => Value::Int((rest % 3600) / 60),
                    "second" => Value::Int(rest % 60),
                    other => {
                        return Err(Error::Execute(format!(
                            "datetime has no field `{other}`"
                        )))
                    }
                })
            }
            other => Err(Error::Execute(format!(
                "cannot read property `{key}` of {}",
                other.key()
            ))),
        }
    }

    fn index_of(&self, base: &Value, index: &Value) -> Result<Value> {
        let Some(index) = index.as_int() else {
            return Err(Error::Execute("list index must be an integer".into()));
        };
        match base {
            Value::List(items) => {
                let len = items.len() as i64;
                let position = if index < 0 { len + index } else { index };
                Ok(items
                    .get(position as usize)
                    .cloned()
                    .unwrap_or(Value::Null))
            }
            Value::Null => Ok(Value::Null),
            other => Err(Error::Execute(format!(
                "cannot index {}",
                other.key()
            ))),
        }
    }

    fn slice_of(&self, base: &Value, start: Option<i64>, end: Option<i64>) -> Result<Value> {
        match base {
            Value::List(items) => {
                let len = items.len() as i64;
                let normalise = |value: i64| {
                    let value = if value < 0 { len + value } else { value };
                    value.clamp(0, len) as usize
                };
                let from = start.map(normalise).unwrap_or(0);
                let to = end.map(normalise).unwrap_or(len as usize);
                Ok(Value::List(if from <= to {
                    items[from..to].to_vec()
                } else {
                    Vec::new()
                }))
            }
            Value::Null => Ok(Value::Null),
            other => Err(Error::Execute(format!("cannot slice {}", other.key()))),
        }
    }

    fn call_function(
        &self,
        name: &str,
        args: &[Expr],
        cols: &[String],
        values: &[Value],
    ) -> Result<Value> {
        let mut evaluated = Vec::with_capacity(args.len());
        for arg in args {
            evaluated.push(self.eval_expr(arg, cols, values)?);
        }
        evaluate_function(name, &evaluated, self.schema())
    }
}

// ---------------------------------------------------------------------------
// Free helpers
// ---------------------------------------------------------------------------

fn property_lookup(properties: &PropertyMap, key: &str, schema: &dyn GraphSchema) -> Value {
    let Some(id) = schema.property_key_by_name(key) else {
        return Value::Null;
    };
    match properties.get(&id) {
        Some(PropertyValue::Bool(v)) => Value::Bool(*v),
        Some(PropertyValue::Int(v)) => Value::Int(*v),
        Some(PropertyValue::Float(v)) => Value::Float(*v),
        Some(PropertyValue::Text(v)) => Value::Text(v.clone()),
        _ => Value::Null,
    }
}

fn and_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Bool(false), _) | (_, Value::Bool(false)) => Value::Bool(false),
        (Value::Bool(true), Value::Bool(true)) => Value::Bool(true),
        _ => Value::Null,
    }
}

fn or_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Bool(true), _) | (_, Value::Bool(true)) => Value::Bool(true),
        (Value::Bool(false), Value::Bool(false)) => Value::Bool(false),
        _ => Value::Null,
    }
}

fn negate_for_order(value: Value) -> Value {
    match value {
        Value::Int(v) => Value::Int(-v),
        Value::Float(v) => Value::Float(-v),
        Value::Text(v) => Value::Text(reverse_ordering_key(&v)),
        other => other,
    }
}

/// A descending key for text that reverses the ascending comparison.
fn reverse_ordering_key(value: &str) -> String {
    value
        .chars()
        .map(|ch| char::from_u32(0x10FFFF - ch as u32).unwrap_or(ch))
        .collect()
}

fn compare_keys(left: &[Value], right: &[Value]) -> std::cmp::Ordering {
    for (a, b) in left.iter().zip(right.iter()) {
        match a.compare(b) {
            Some(std::cmp::Ordering::Equal) | None => {}
            Some(order) => return order,
        }
    }
    std::cmp::Ordering::Equal
}

fn frame_key(values: &[Value]) -> String {
    values
        .iter()
        .map(Value::key)
        .collect::<Vec<_>>()
        .join("\u{1}")
}

fn value_to_property(value: &Value) -> Result<PropertyValue> {
    Ok(match value {
        Value::Null => PropertyValue::Null,
        Value::Bool(v) => PropertyValue::Bool(*v),
        Value::Int(v) => PropertyValue::Int(*v),
        Value::Float(v) => PropertyValue::Float(*v),
        Value::Text(v) => PropertyValue::Text(v.clone()),
        other => {
            return Err(Error::Execute(format!(
                "{} cannot be stored as a property value",
                other.key()
            )))
        }
    })
}

fn value_to_literal(value: &Value) -> Option<Expr> {
    Some(match value {
        Value::Null => Expr::Null,
        Value::Bool(v) => Expr::Bool(*v),
        Value::Int(v) => Expr::Int(*v),
        Value::Float(v) => Expr::Float(*v),
        Value::Text(v) => Expr::Str(v.clone()),
        Value::List(items) => Expr::List(
            items
                .iter()
                .map(value_to_literal)
                .collect::<Option<Vec<_>>>()?,
        ),
        Value::Map(entries) => Expr::Map(
            entries
                .iter()
                .map(|(key, value)| Some((key.clone(), value_to_literal(value)?)))
                .collect::<Option<Vec<_>>>()?,
        ),
        _ => return None,
    })
}

/// Variables a pattern introduces, used to null-fill an OPTIONAL MATCH miss.
fn pattern_variables(pattern: &Pattern) -> Vec<String> {
    let mut vars = Vec::new();
    for part in &pattern.parts {
        if let Some(path_var) = &part.path_var {
            vars.push(path_var.clone());
        }
        collect_node_vars(&part.element.node, &mut vars);
        for (edge, node) in &part.element.chains {
            if let Some(var) = &edge.var {
                vars.push(var.clone());
            }
            collect_node_vars(node, &mut vars);
        }
    }
    vars
}

fn collect_node_vars(node: &NodePattern, vars: &mut Vec<String>) {
    if let Some(var) = &node.var {
        vars.push(var.clone());
    }
}

/// Expand `RETURN *` into one item per visible column.
fn expand_star(items: &[ReturnItem], frame: &Frame) -> Vec<ReturnItem> {
    if items.len() == 1 && items[0].expr.as_variable() == Some("*") {
        return frame
            .cols
            .iter()
            .map(|col| ReturnItem {
                expr: Expr::Variable(col.clone()),
                alias: None,
            })
            .collect();
    }
    items.to_vec()
}

fn item_name(item: &ReturnItem, _cols: &[String]) -> String {
    if let Some(alias) = &item.alias {
        return alias.clone();
    }
    match &item.expr {
        Expr::Variable(name) => name.clone(),
        Expr::Property(base, key) => match base.as_variable() {
            Some(var) => format!("{var}.{key}"),
            None => key.clone(),
        },
        other => render_expr(other),
    }
}

fn render_expr(expr: &Expr) -> String {
    match expr {
        Expr::Null => "null".into(),
        Expr::Bool(v) => v.to_string(),
        Expr::Int(v) => v.to_string(),
        Expr::Float(v) => v.to_string(),
        Expr::Str(v) => format!("'{v}'"),
        Expr::Variable(name) => name.clone(),
        Expr::Property(base, key) => format!("{}.{}", render_expr(base), key),
        Expr::Function { name, args, .. } => format!(
            "{}({})",
            name,
            args.iter().map(render_expr).collect::<Vec<_>>().join(", ")
        ),
        Expr::CountAll => "count(*)".into(),
        Expr::Binary(op, a, b) => format!("{} {:?} {}", render_expr(a), op, render_expr(b)),
        other => format!("{other:?}"),
    }
}

/// Combine a source row with the projected output columns for ORDER BY / WHERE.
fn combine(
    source_cols: &[String],
    source_values: &[Value],
    output_names: &[String],
    output_values: &[Value],
) -> RowBuilder {
    let mut builder = RowBuilder::from_row(source_cols, source_values);
    for (index, name) in output_names.iter().enumerate() {
        builder.set(name, output_values[index].clone());
    }
    builder
}

/// Group the frame for an aggregation, preserving first-seen group order.
fn group_rows(
    frame: &Frame,
    items: &[ReturnItem],
    runtime: &Runtime,
) -> Result<Vec<(Row, Vec<(Vec<String>, Row)>)>> {
    let keys: Vec<&Expr> = items
        .iter()
        .filter(|item| !item.expr.is_aggregate())
        .map(|item| &item.expr)
        .collect();
    if keys.is_empty() {
        // One group containing every row; an empty input still yields one
        // group so that `count(x)` over no rows returns 0.  The representative
        // row keeps the frame's column layout so ORDER BY and a following
        // RETURN can still resolve variables.
        let representative = frame
            .rows
            .first()
            .cloned()
            .unwrap_or_else(|| vec![Value::Null; frame.cols.len()]);
        let members = frame
            .rows
            .iter()
            .map(|row| (frame.cols.clone(), row.clone()))
            .collect();
        return Ok(vec![(representative, members)]);
    }
    let mut order: Vec<String> = Vec::new();
    let mut groups: HashMap<String, (Row, Vec<(Vec<String>, Row)>)> = HashMap::new();
    for row in &frame.rows {
        let mut key_values = Vec::with_capacity(keys.len());
        for key in &keys {
            key_values.push(runtime.eval_expr(key, &frame.cols, row)?);
        }
        let key = frame_key(&key_values);
        let entry = groups.entry(key.clone()).or_insert_with(|| {
            order.push(key);
            (row.clone(), Vec::new())
        });
        entry.1.push((frame.cols.clone(), row.clone()));
    }
    Ok(order
        .into_iter()
        .filter_map(|key| groups.remove(&key))
        .collect())
}

/// Convert procedure arguments into the scalar parameter set.
fn call_params(
    schema: &dyn GraphSchema,
    name: &str,
    args: &[ProcArg],
    _cols: &[String],
    _row: Option<&Row>,
) -> Result<BTreeMap<String, PropertyValue>> {
    let mut params = BTreeMap::new();
    let mut position = 0usize;
    for arg in args {
        let (key, expr) = match arg {
            ProcArg::Named(key, expr) => (key.clone(), expr),
            ProcArg::Positional(expr) => {
                let key = super::algorithms::parameter_name(name, position).ok_or_else(|| {
                    Error::Execute(format!(
                        "procedure `{name}` has no parameter at position {position}"
                    ))
                })?;
                position += 1;
                (key.to_string(), expr)
            }
        };
        let _ = schema;
        params.insert(key, literal_property(expr)?);
    }
    Ok(params)
}

fn literal_property(expr: &Expr) -> Result<PropertyValue> {
    Ok(match expr {
        Expr::Null => PropertyValue::Null,
        Expr::Bool(v) => PropertyValue::Bool(*v),
        Expr::Int(v) => PropertyValue::Int(*v),
        Expr::Float(v) => PropertyValue::Float(*v),
        Expr::Str(v) => PropertyValue::Text(v.clone()),
        other => {
            return Err(Error::Unsupported(format!(
                "algorithm arguments must be scalar literals, found {other:?}"
            )))
        }
    })
}

/// Evaluate a built-in scalar function.
fn evaluate_function(name: &str, args: &[Value], schema: &dyn GraphSchema) -> Result<Value> {
    let lowered = name.to_ascii_lowercase();
    let arg = |index: usize| -> Result<&Value> {
        args.get(index).ok_or_else(|| {
            Error::Execute(format!("function `{name}` is missing argument {index}"))
        })
    };
    match lowered.as_str() {
        "id" => Ok(match arg(0)? {
            Value::Vertex(vertex) => Value::Int(vertex.id.0 as i64),
            Value::Edge(edge) => Value::Int(edge.id.0 as i64),
            other => {
                return Err(Error::Execute(format!(
                    "id() expects a vertex or an edge, found {}",
                    other.key()
                )))
            }
        }),
        "labels" => Ok(match arg(0)? {
            Value::Vertex(vertex) => Value::List(
                vertex
                    .labels
                    .iter()
                    .map(|label| {
                        Value::Text(
                            schema
                                .label_name(*label)
                                .map(str::to_string)
                                .unwrap_or_else(|| format!("Label{}", label.0)),
                        )
                    })
                    .collect(),
            ),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "labels() expects a vertex, found {}",
                    other.key()
                )))
            }
        }),
        "type" => Ok(match arg(0)? {
            Value::Edge(edge) => Value::Text(
                schema
                    .edge_type_name(edge.etype)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("Type{}", edge.etype.0)),
            ),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "type() expects an edge, found {}",
                    other.key()
                )))
            }
        }),
        "keys" => Ok(match arg(0)? {
            Value::Vertex(vertex) => Value::List(key_names(&vertex.properties, schema)),
            Value::Edge(edge) => Value::List(key_names(&edge.properties, schema)),
            Value::Map(entries) => Value::List(
                entries
                    .iter()
                    .map(|(key, _)| Value::Text(key.clone()))
                    .collect(),
            ),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "keys() expects a vertex, edge or map, found {}",
                    other.key()
                )))
            }
        }),
        "properties" => Ok(match arg(0)? {
            Value::Vertex(vertex) => Value::Map(property_entries(&vertex.properties, schema)),
            Value::Edge(edge) => Value::Map(property_entries(&edge.properties, schema)),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "properties() expects a vertex or edge, found {}",
                    other.key()
                )))
            }
        }),
        "exists" => Ok(Value::Bool(!arg(0)?.is_null())),
        "coalesce" => Ok(args
            .iter()
            .find(|value| !value.is_null())
            .cloned()
            .unwrap_or(Value::Null)),
        "size" => Ok(match arg(0)? {
            Value::List(items) => Value::Int(items.len() as i64),
            Value::Text(text) => Value::Int(text.chars().count() as i64),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "size() expects a list or string, found {}",
                    other.key()
                )))
            }
        }),
        // `length(path)` counts relationships, matching Cypher semantics; the
        // profile also asserts `size(relationships(p))` and `size(nodes(p))`.
        "length" => Ok(match arg(0)? {
            Value::Path(_, edges) => Value::Int(edges.len() as i64),
            Value::List(items) => Value::Int(items.len() as i64),
            Value::Text(text) => Value::Int(text.chars().count() as i64),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "length() expects a path, list or string, found {}",
                    other.key()
                )))
            }
        }),
        "nodes" => Ok(match arg(0)? {
            Value::Path(nodes, _) => Value::List(nodes.clone()),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "nodes() expects a path, found {}",
                    other.key()
                )))
            }
        }),
        "relationships" => Ok(match arg(0)? {
            Value::Path(_, edges) => Value::List(edges.clone()),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "relationships() expects a path, found {}",
                    other.key()
                )))
            }
        }),
        "range" => {
            let start = require_int(arg(0)?, "range")?;
            let end = require_int(arg(1)?, "range")?;
            let step = match args.get(2) {
                Some(value) => require_int(value, "range")?,
                None => 1,
            };
            if step == 0 {
                return Err(Error::Execute("range() step must not be zero".into()));
            }
            let mut items = Vec::new();
            let mut current = start;
            if step > 0 {
                while current <= end {
                    items.push(Value::Int(current));
                    current += step;
                }
            } else {
                while current >= end {
                    items.push(Value::Int(current));
                    current += step;
                }
            }
            Ok(Value::List(items))
        }
        "head" => Ok(match arg(0)? {
            Value::List(items) => items.first().cloned().unwrap_or(Value::Null),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "head() expects a list, found {}",
                    other.key()
                )))
            }
        }),
        "last" => Ok(match arg(0)? {
            Value::List(items) => items.last().cloned().unwrap_or(Value::Null),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "last() expects a list, found {}",
                    other.key()
                )))
            }
        }),
        "tail" => Ok(match arg(0)? {
            Value::List(items) => Value::List(items.iter().skip(1).cloned().collect()),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "tail() expects a list, found {}",
                    other.key()
                )))
            }
        }),
        "reverse" => Ok(match arg(0)? {
            Value::List(items) => Value::List(items.iter().rev().cloned().collect()),
            Value::Text(text) => Value::Text(text.chars().rev().collect()),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "reverse() expects a list or string, found {}",
                    other.key()
                )))
            }
        }),
        "toupper" => Ok(text_map(arg(0)?, |text| text.to_uppercase())),
        "tolower" => Ok(text_map(arg(0)?, |text| text.to_lowercase())),
        "trim" => Ok(text_map(arg(0)?, |text| text.trim().to_string())),
        "substring" => {
            let text = require_text(arg(0)?, "substring")?;
            let start = require_int(arg(1)?, "substring")?.max(0) as usize;
            let chars: Vec<char> = text.chars().collect();
            let end = match args.get(2) {
                Some(value) => (start + require_int(value, "substring")?.max(0) as usize)
                    .min(chars.len()),
                None => chars.len(),
            };
            let start = start.min(chars.len());
            Ok(Value::Text(chars[start..end.max(start)].iter().collect()))
        }
        "left" => {
            let text = require_text(arg(0)?, "left")?;
            let count = require_int(arg(1)?, "left")?.max(0) as usize;
            Ok(Value::Text(text.chars().take(count).collect()))
        }
        "right" => {
            let text = require_text(arg(0)?, "right")?;
            let count = require_int(arg(1)?, "right")?.max(0) as usize;
            let chars: Vec<char> = text.chars().collect();
            let start = chars.len().saturating_sub(count);
            Ok(Value::Text(chars[start..].iter().collect()))
        }
        "replace" => {
            let text = require_text(arg(0)?, "replace")?;
            let from = require_text(arg(1)?, "replace")?;
            let to = require_text(arg(2)?, "replace")?;
            Ok(Value::Text(text.replace(from, to)))
        }
        "split" => {
            let text = require_text(arg(0)?, "split")?;
            let separator = require_text(arg(1)?, "split")?;
            Ok(Value::List(
                text.split(separator)
                    .map(|part| Value::Text(part.to_string()))
                    .collect(),
            ))
        }
        "tostring" => Ok(Value::Text(match arg(0)? {
            Value::Null => return Ok(Value::Null),
            Value::Text(text) => text.clone(),
            Value::Bool(value) => value.to_string(),
            Value::Int(value) => value.to_string(),
            Value::Float(value) => format_float(*value),
            other => other.key(),
        })),
        "tointeger" => Ok(match arg(0)? {
            Value::Int(value) => Value::Int(*value),
            Value::Float(value) => Value::Int(*value as i64),
            Value::Bool(value) => Value::Int(i64::from(*value)),
            Value::Text(text) => match text.trim().parse::<f64>() {
                Ok(value) => Value::Int(value as i64),
                Err(_) => Value::Null,
            },
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "toInteger() cannot convert {}",
                    other.key()
                )))
            }
        }),
        "tofloat" => Ok(match arg(0)? {
            Value::Float(value) => Value::Float(*value),
            Value::Int(value) => Value::Float(*value as f64),
            Value::Text(text) => match text.trim().parse::<f64>() {
                Ok(value) => Value::Float(value),
                Err(_) => Value::Null,
            },
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "toFloat() cannot convert {}",
                    other.key()
                )))
            }
        }),
        "toboolean" => Ok(match arg(0)? {
            Value::Bool(value) => Value::Bool(*value),
            Value::Text(text) => match text.trim().to_ascii_lowercase().as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => Value::Null,
            },
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "toBoolean() cannot convert {}",
                    other.key()
                )))
            }
        }),
        "abs" => Ok(numeric_map(arg(0)?, "abs", f64::abs, i64::abs)?),
        "ceil" => Ok(float_map(arg(0)?, f64::ceil)),
        "floor" => Ok(float_map(arg(0)?, f64::floor)),
        "round" => Ok(float_map(arg(0)?, f64::round)),
        "sqrt" => Ok(float_map(arg(0)?, f64::sqrt)),
        "sign" => Ok(match arg(0)? {
            Value::Int(value) => Value::Int(value.signum()),
            Value::Float(value) => Value::Int(if *value == 0.0 {
                0
            } else if *value > 0.0 {
                1
            } else {
                -1
            }),
            Value::Null => Value::Null,
            other => {
                return Err(Error::Execute(format!(
                    "sign() expects a number, found {}",
                    other.key()
                )))
            }
        }),
        "date" => Ok(Value::Date(parse_date(&require_text(arg(0)?, "date")?)?)),
        "datetime" => Ok(Value::DateTime(parse_datetime(&require_text(
            arg(0)?,
            "datetime",
        )?)?)),
        "time" => Ok(Value::Time(parse_time(&require_text(arg(0)?, "time")?)?)),
        "duration" => Ok(Value::Duration(parse_duration(&require_text(
            arg(0)?,
            "duration",
        )?)?)),
        other => Err(Error::Unsupported(format!(
            "function `{other}` is outside the core language profile"
        ))),
    }
}

fn format_float(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn key_names(properties: &PropertyMap, schema: &dyn GraphSchema) -> Vec<Value> {
    let mut names: Vec<String> = properties
        .keys()
        .map(|key| {
            schema
                .property_key_name(*key)
                .map(str::to_string)
                .unwrap_or_else(|| format!("key{}", key.0))
        })
        .collect();
    names.sort();
    names.into_iter().map(Value::Text).collect()
}

fn property_entries(properties: &PropertyMap, schema: &dyn GraphSchema) -> Vec<(String, Value)> {
    properties
        .iter()
        .map(|(key, value)| {
            let name = schema
                .property_key_name(*key)
                .map(str::to_string)
                .unwrap_or_else(|| format!("key{}", key.0));
            let value = match value {
                PropertyValue::Null => Value::Null,
                PropertyValue::Bool(v) => Value::Bool(*v),
                PropertyValue::Int(v) => Value::Int(*v),
                PropertyValue::Float(v) => Value::Float(*v),
                PropertyValue::Text(v) => Value::Text(v.clone()),
            };
            (name, value)
        })
        .collect()
}

fn require_int(value: &Value, function: &str) -> Result<i64> {
    value.as_int().ok_or_else(|| {
        Error::Execute(format!(
            "{function}() expects an integer, found {}",
            value.key()
        ))
    })
}

fn require_text<'a>(value: &'a Value, function: &str) -> Result<&'a str> {
    value.as_text().ok_or_else(|| {
        Error::Execute(format!(
            "{function}() expects a string, found {}",
            value.key()
        ))
    })
}

fn text_map(value: &Value, f: impl Fn(&str) -> String) -> Value {
    match value {
        Value::Null => Value::Null,
        Value::Text(text) => Value::Text(f(text)),
        other => Value::Text(f(&format!("{}", other.key()))),
    }
}

fn numeric_map(
    value: &Value,
    function: &str,
    float: impl Fn(f64) -> f64,
    int: impl Fn(i64) -> i64,
) -> Result<Value> {
    Ok(match value {
        Value::Int(v) => Value::Int(int(*v)),
        Value::Float(v) => Value::Float(float(*v)),
        Value::Null => Value::Null,
        other => {
            return Err(Error::Execute(format!(
                "{function}() expects a number, found {}",
                other.key()
            )))
        }
    })
}

fn float_map(value: &Value, f: impl Fn(f64) -> f64) -> Value {
    match value {
        Value::Int(v) => Value::Float(f(*v as f64)),
        Value::Float(v) => Value::Float(f(*v)),
        _ => Value::Null,
    }
}

// ---------------------------------------------------------------------------
// Temporal parsing
// ---------------------------------------------------------------------------

fn parse_date(text: &str) -> Result<i64> {
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() != 3 {
        return Err(Error::Execute(format!("invalid date literal `{text}`")));
    }
    let year: i64 = parts[0]
        .parse()
        .map_err(|_| Error::Execute(format!("invalid date year in `{text}`")))?;
    let month: u32 = parts[1]
        .parse()
        .map_err(|_| Error::Execute(format!("invalid date month in `{text}`")))?;
    let day: u32 = parts[2]
        .parse()
        .map_err(|_| Error::Execute(format!("invalid date day in `{text}`")))?;
    Ok(days_from_civil(year, month, day))
}

fn parse_datetime(text: &str) -> Result<i64> {
    let (date, rest) = text
        .split_once('T')
        .or_else(|| text.split_once('t'))
        .ok_or_else(|| Error::Execute(format!("invalid datetime literal `{text}`")))?;
    let days = parse_date(date)?;
    let clock = rest.trim_end_matches('Z').trim_end_matches('z');
    let seconds = parse_time(clock)?;
    Ok(days * 86_400 + seconds)
}

fn parse_time(text: &str) -> Result<i64> {
    let parts: Vec<&str> = text.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return Err(Error::Execute(format!("invalid time literal `{text}`")));
    }
    let hours: i64 = parts[0]
        .parse()
        .map_err(|_| Error::Execute(format!("invalid hour in `{text}`")))?;
    let minutes: i64 = parts[1]
        .parse()
        .map_err(|_| Error::Execute(format!("invalid minute in `{text}`")))?;
    let seconds: i64 = match parts.get(2) {
        Some(value) => value
            .parse()
            .map_err(|_| Error::Execute(format!("invalid second in `{text}`")))?,
        None => 0,
    };
    Ok(hours * 3600 + minutes * 60 + seconds)
}

/// Parse the ISO-8601 duration forms used by the profile (`P1D`, `PT12H`, ...).
fn parse_duration(text: &str) -> Result<i64> {
    let body = text
        .strip_prefix('P')
        .or_else(|| text.strip_prefix('p'))
        .ok_or_else(|| Error::Execute(format!("invalid duration literal `{text}`")))?;
    let mut total = 0i64;
    let mut number = String::new();
    let mut in_time = false;
    for ch in body.chars() {
        match ch {
            '0'..='9' | '-' | '+' => number.push(ch),
            'T' | 't' => {
                in_time = true;
                number.clear();
            }
            unit => {
                let value: i64 = number
                    .parse()
                    .map_err(|_| Error::Execute(format!("invalid duration literal `{text}`")))?;
                number.clear();
                let seconds = match unit {
                    'Y' | 'y' => value * 365 * 86_400,
                    'W' | 'w' => value * 7 * 86_400,
                    'D' | 'd' => value * 86_400,
                    'H' | 'h' => value * 3600,
                    'M' | 'm' if in_time => value * 60,
                    'M' | 'm' => value * 30 * 86_400,
                    'S' | 's' => value,
                    other => {
                        return Err(Error::Execute(format!(
                            "unsupported duration unit `{other}` in `{text}`"
                        )))
                    }
                };
                total += seconds;
            }
        }
    }
    Ok(total)
}
