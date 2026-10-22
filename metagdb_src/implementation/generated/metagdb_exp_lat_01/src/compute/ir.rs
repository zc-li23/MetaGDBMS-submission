//! Bound intermediate representation shared by the planner, optimizer, and
//! executor: runtime values, rows, and resolved expressions.
//!
//! Names (labels, edge types, property keys) have already been interned by the
//! planner, so a bound expression references IDs rather than strings.

use std::cmp::Ordering;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyValue, ResultValue};

/// A runtime value produced by expression evaluation.
///
/// `PartialEq` is the structural equality used by tests; query comparisons go
/// through `equals`, which treats integers and floats numerically.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Value>),
    Map(Vec<(String, Value)>),
    Node(VertexId),
    Rel(EdgeId),
    Path(Vec<Value>),
    Date(i32, u32, u32),
    Time(u32, u32, u32),
    DateTime(i64),
    Duration(i64),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Cypher truthiness: only `true` passes; `null` and `false` do not.
    pub fn is_true(&self) -> bool {
        matches!(self, Value::Bool(true))
    }

    pub fn as_node(&self) -> Option<VertexId> {
        match self {
            Value::Node(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_rel(&self) -> Option<EdgeId> {
        match self {
            Value::Rel(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) if f.fract() == 0.0 => Some(*f as i64),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(items) => Some(items),
            _ => None,
        }
    }

    /// Structural equality used by DISTINCT and IN.
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => (*a as f64) == *b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Node(a), Value::Node(b)) => a == b,
            (Value::Rel(a), Value::Rel(b)) => a == b,
            (Value::List(a), Value::List(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.equals(y))
            }
            (Value::Map(a), Value::Map(b)) => {
                let mut left: Vec<_> = a.iter().collect();
                let mut right: Vec<_> = b.iter().collect();
                left.sort_by(|x, y| x.0.cmp(&y.0));
                right.sort_by(|x, y| x.0.cmp(&y.0));
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|((ka, va), (kb, vb))| ka == kb && va.equals(vb))
            }
            (
                Value::Date(ay, am, ad),
                Value::Date(by, bm, bd),
            ) => ay == by && am == bm && ad == bd,
            (Value::Time(ah, am, asec), Value::Time(bh, bm, bsec)) => {
                (ah, am, asec) == (bh, bm, bsec)
            }
            (Value::DateTime(a), Value::DateTime(b)) => a == b,
            (Value::Duration(a), Value::Duration(b)) => a == b,
            (Value::Path(a), Value::Path(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.equals(y))
            }
            _ => false,
        }
    }

    /// Total ordering used by ORDER BY. Nulls sort last in ascending order.
    pub fn total_cmp(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::Null, Value::Null) => Ordering::Equal,
            (Value::Null, _) => Ordering::Greater,
            (_, Value::Null) => Ordering::Less,
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b).unwrap_or(Ordering::Equal),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)).unwrap_or(Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            (Value::Text(a), Value::Text(b)) => a.cmp(b),
            (Value::Date(ay, am, ad), Value::Date(by, bm, bd)) => (ay, am, ad).cmp(&(by, bm, bd)),
            (Value::Time(ah, am, asec), Value::Time(bh, bm, bsec)) => {
                (ah, am, asec).cmp(&(bh, bm, bsec))
            }
            (Value::DateTime(a), Value::DateTime(b)) => a.cmp(b),
            (Value::Duration(a), Value::Duration(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }

    /// Canonical key for deduplication.
    pub fn canonical_key(&self) -> String {
        match self {
            Value::Null => "n".into(),
            Value::Bool(b) => format!("b{b}"),
            Value::Int(i) => format!("i{i}"),
            Value::Float(f) => format!("f{}", canonical_float(*f)),
            Value::Text(t) => format!("s{t}"),
            Value::Node(id) => format!("v{}", id.0),
            Value::Rel(id) => format!("e{}", id.0),
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(|item| item.canonical_key()).collect();
                format!("[{}]", inner.join(","))
            }
            Value::Map(entries) => {
                let mut pairs: Vec<String> = entries
                    .iter()
                    .map(|(key, value)| format!("{key}:{}", value.canonical_key()))
                    .collect();
                pairs.sort();
                format!("{{{}}}", pairs.join(","))
            }
            Value::Path(items) => {
                let inner: Vec<String> = items.iter().map(|item| item.canonical_key()).collect();
                format!("p({})", inner.join(","))
            }
            Value::Date(y, m, d) => format!("d{y}-{m}-{d}"),
            Value::Time(h, m, s) => format!("t{h}:{m}:{s}"),
            Value::DateTime(v) => format!("dt{v}"),
            Value::Duration(v) => format!("du{v}"),
        }
    }
}

fn canonical_float(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 9e15 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}

/// Convert a stored property into a runtime value.
pub fn value_from_property(property: &PropertyValue) -> Value {
    match property {
        PropertyValue::Null => Value::Null,
        PropertyValue::Bool(b) => Value::Bool(*b),
        PropertyValue::Int(i) => Value::Int(*i),
        PropertyValue::Float(f) => Value::Float(*f),
        PropertyValue::Text(t) => Value::Text(t.clone()),
    }
}

/// Convert a runtime value into a stored property (null for non-scalars).
pub fn value_to_property(value: &Value) -> PropertyValue {
    match value {
        Value::Null => PropertyValue::Null,
        Value::Bool(b) => PropertyValue::Bool(*b),
        Value::Int(i) => PropertyValue::Int(*i),
        Value::Float(f) => PropertyValue::Float(*f),
        Value::Text(t) => PropertyValue::Text(t.clone()),
        _ => PropertyValue::Null,
    }
}

/// Convert a runtime value into the framework's result model.
pub fn value_to_result(value: &Value) -> ResultValue {
    match value {
        Value::Null => ResultValue::Null,
        Value::Bool(b) => ResultValue::Bool(*b),
        Value::Int(i) => ResultValue::Int(*i),
        Value::Float(f) => ResultValue::Float(*f),
        Value::Text(t) => ResultValue::Text(t.clone()),
        Value::Node(id) => ResultValue::VertexRef(*id),
        Value::Rel(id) => ResultValue::EdgeRef(*id),
        Value::List(items) => ResultValue::List(items.iter().map(value_to_result).collect()),
        Value::Map(entries) => ResultValue::Map(
            entries
                .iter()
                .map(|(key, value)| {
                    (
                        ResultValue::Text(key.clone()),
                        value_to_result(value),
                    )
                })
                .collect(),
        ),
        Value::Path(items) => ResultValue::Path(
            metagdb_core::common::property::ResultPath::from_objects(
                items.iter().map(value_to_result).collect(),
            ),
        ),
        Value::Date(y, m, d) => ResultValue::Text(format!("{y:04}-{m:02}-{d:02}")),
        Value::Time(h, m, s) => ResultValue::Text(format!("{h:02}:{m:02}:{s:02}")),
        Value::DateTime(v) => ResultValue::Int(*v),
        Value::Duration(v) => ResultValue::Int(*v),
    }
}

/// A row is an ordered set of variable bindings.
pub type Row = Vec<(String, Value)>;

pub fn row_get<'a>(row: &'a Row, name: &str) -> Option<&'a Value> {
    row.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

pub fn row_put(row: &mut Row, name: &str, value: Value) {
    if let Some(slot) = row.iter_mut().find(|(key, _)| key == name) {
        slot.1 = value;
    } else {
        row.push((name.to_string(), value));
    }
}

/// Aggregate function kinds supported by the projection stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AggregateKind {
    Count,
    CountStar,
    Sum,
    Avg,
    Min,
    Max,
    Collect,
}

/// Bound scalar expression with interned identifiers.
#[derive(Clone, Debug)]
pub enum BExpr {
    Const(Value),
    Var(String),
    Param(String),
    Prop(Box<BExpr>, PropertyKeyId),
    LabelsOf(Box<BExpr>),
    Neg(Box<BExpr>),
    Add(Box<BExpr>, Box<BExpr>),
    Sub(Box<BExpr>, Box<BExpr>),
    Mul(Box<BExpr>, Box<BExpr>),
    Div(Box<BExpr>, Box<BExpr>),
    Mod(Box<BExpr>, Box<BExpr>),
    Pow(Box<BExpr>, Box<BExpr>),
    Eq(Box<BExpr>, Box<BExpr>),
    Neq(Box<BExpr>, Box<BExpr>),
    Lt(Box<BExpr>, Box<BExpr>),
    Lte(Box<BExpr>, Box<BExpr>),
    Gt(Box<BExpr>, Box<BExpr>),
    Gte(Box<BExpr>, Box<BExpr>),
    And(Box<BExpr>, Box<BExpr>),
    Or(Box<BExpr>, Box<BExpr>),
    Xor(Box<BExpr>, Box<BExpr>),
    Not(Box<BExpr>),
    IsNull(Box<BExpr>),
    IsNotNull(Box<BExpr>),
    In(Box<BExpr>, Box<BExpr>),
    StartsWith(Box<BExpr>, Box<BExpr>),
    EndsWith(Box<BExpr>, Box<BExpr>),
    Contains(Box<BExpr>, Box<BExpr>),
    ListIndex(Box<BExpr>, Box<BExpr>),
    ListSlice {
        list: Box<BExpr>,
        start: Option<Box<BExpr>>,
        end: Option<Box<BExpr>>,
    },
    List(Vec<BExpr>),
    Map(Vec<(String, BExpr)>),
    Function {
        name: String,
        args: Vec<BExpr>,
    },
    Case {
        test: Option<Box<BExpr>>,
        whens: Vec<(BExpr, BExpr)>,
        else_branch: Option<Box<BExpr>>,
    },
    ListComprehension {
        var: String,
        list: Box<BExpr>,
        filter: Option<Box<BExpr>>,
        map: Option<Box<BExpr>>,
    },
    Quantifier {
        kind: QuantifierKind,
        var: String,
        list: Box<BExpr>,
        predicate: Option<Box<BExpr>>,
    },
    LabelTest(Box<BExpr>, Vec<LabelId>),
    /// `id(expr)` is kept distinct so the optimizer can push it into a scan.
    IdOf(Box<BExpr>),
    Aggregate {
        kind: AggregateKind,
        distinct: bool,
        arg: Option<Box<BExpr>>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuantifierKind {
    All,
    Any,
    None,
    Single,
}

impl BExpr {
    pub fn contains_aggregate(&self) -> bool {
        match self {
            BExpr::Aggregate { .. } => true,
            BExpr::Const(_) | BExpr::Var(_) | BExpr::Param(_) => false,
            BExpr::Prop(inner, _) | BExpr::LabelsOf(inner) | BExpr::Neg(inner)
            | BExpr::Not(inner) | BExpr::IsNull(inner) | BExpr::IsNotNull(inner)
            | BExpr::IdOf(inner) | BExpr::LabelTest(inner, _) => inner.contains_aggregate(),
            BExpr::Add(a, b)
            | BExpr::Sub(a, b)
            | BExpr::Mul(a, b)
            | BExpr::Div(a, b)
            | BExpr::Mod(a, b)
            | BExpr::Pow(a, b)
            | BExpr::Eq(a, b)
            | BExpr::Neq(a, b)
            | BExpr::Lt(a, b)
            | BExpr::Lte(a, b)
            | BExpr::Gt(a, b)
            | BExpr::Gte(a, b)
            | BExpr::And(a, b)
            | BExpr::Or(a, b)
            | BExpr::Xor(a, b)
            | BExpr::In(a, b)
            | BExpr::StartsWith(a, b)
            | BExpr::EndsWith(a, b)
            | BExpr::Contains(a, b)
            | BExpr::ListIndex(a, b) => a.contains_aggregate() || b.contains_aggregate(),
            BExpr::ListSlice { list, start, end } => {
                list.contains_aggregate()
                    || start.as_ref().map_or(false, |x| x.contains_aggregate())
                    || end.as_ref().map_or(false, |x| x.contains_aggregate())
            }
            BExpr::List(items) => items.iter().any(|item| item.contains_aggregate()),
            BExpr::Map(entries) => entries.iter().any(|(_, value)| value.contains_aggregate()),
            BExpr::Function { args, .. } => args.iter().any(|arg| arg.contains_aggregate()),
            BExpr::Case {
                test,
                whens,
                else_branch,
            } => {
                test.as_ref().map_or(false, |x| x.contains_aggregate())
                    || whens
                        .iter()
                        .any(|(a, b)| a.contains_aggregate() || b.contains_aggregate())
                    || else_branch.as_ref().map_or(false, |x| x.contains_aggregate())
            }
            BExpr::ListComprehension {
                list,
                filter,
                map,
                ..
            } => {
                list.contains_aggregate()
                    || filter.as_ref().map_or(false, |x| x.contains_aggregate())
                    || map.as_ref().map_or(false, |x| x.contains_aggregate())
            }
            BExpr::Quantifier {
                list, predicate, ..
            } => {
                list.contains_aggregate()
                    || predicate.as_ref().map_or(false, |x| x.contains_aggregate())
            }
        }
    }

    /// Textual name used as the default projection alias.
    pub fn default_alias(&self) -> String {
        match self {
            BExpr::Var(name) => name.clone(),
            BExpr::Prop(source, key) => format!("{}.{}", source.default_alias(), key.0),
            BExpr::IdOf(inner) => format!("id({})", inner.default_alias()),
            BExpr::Function { name, args } => {
                let inner: Vec<String> = args.iter().map(|arg| arg.default_alias()).collect();
                format!("{}({})", name, inner.join(", "))
            }
            BExpr::Aggregate { kind, .. } => match kind {
                AggregateKind::CountStar => "count(*)".into(),
                AggregateKind::Count => "count".into(),
                AggregateKind::Sum => "sum".into(),
                AggregateKind::Avg => "avg".into(),
                AggregateKind::Min => "min".into(),
                AggregateKind::Max => "max".into(),
                AggregateKind::Collect => "collect".into(),
            },
            _ => "expr".into(),
        }
    }
}

/// Labelled edge accessor used by the runtime to resolve labels/properties.
pub struct Endpoint {
    pub vertex: VertexId,
    pub edge: EdgeId,
    pub etype: EdgeTypeId,
    pub src: VertexId,
    pub dst: VertexId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_equality_across_types() {
        assert!(Value::Int(2).equals(&Value::Float(2.0)));
        assert!(!Value::Int(2).equals(&Value::Float(2.5)));
    }

    #[test]
    fn canonical_key_distinguishes_lists() {
        let a = Value::List(vec![Value::Int(1), Value::Int(2)]);
        let b = Value::List(vec![Value::Int(2), Value::Int(1)]);
        assert_ne!(a.canonical_key(), b.canonical_key());
    }

    #[test]
    fn nulls_sort_last() {
        assert_eq!(Value::Int(5).total_cmp(&Value::Null), std::cmp::Ordering::Less);
        assert_eq!(Value::Null.total_cmp(&Value::Null), std::cmp::Ordering::Equal);
    }
}
