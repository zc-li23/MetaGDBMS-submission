//! Runtime value model for expression evaluation.
//!
//! Rows carry [`Value`] rather than `ResultValue` because evaluation needs two
//! things the framework result model deliberately does not have: temporal
//! values (see [`super::temporal`]) and a three-valued comparison that can
//! answer "unknown" for `null`.  [`Value::into_result`] is the single conversion
//! back to the framework's `ResultValue` at the result-stream boundary.
//!
//! Comparison follows Cypher: two values of different types are not ordered
//! (`None`), and any comparison involving `null` is unknown.

use std::cmp::Ordering;

use metagdb_core::common::ids::{EdgeId, VertexId};
use metagdb_core::common::property::{PropertyValue, ResultPath, ResultValue};

use super::temporal::Temporal;

/// A traversed path: the vertices and relationships in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathValue {
    pub nodes: Vec<VertexId>,
    pub edges: Vec<EdgeId>,
}

impl PathValue {
    /// Number of relationships, which is what `length(p)` reports.
    pub fn length(&self) -> usize {
        self.edges.len()
    }
}

/// One execution row: values addressed by the schema beside them.
pub type EvalRow = Vec<Value>;

/// A value produced or consumed by expression evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Value>),
    /// Map with text keys, in insertion order.
    Map(Vec<(String, Value)>),
    Vertex(VertexId),
    Edge(EdgeId),
    Path(PathValue),
    Temporal(Temporal),
}

impl Value {
    pub fn from_property(value: &PropertyValue) -> Self {
        match value {
            PropertyValue::Null => Value::Null,
            PropertyValue::Bool(v) => Value::Bool(*v),
            PropertyValue::Int(v) => Value::Int(*v),
            PropertyValue::Float(v) => Value::Float(*v),
            PropertyValue::Text(v) => Value::Text(v.clone()),
        }
    }

    /// Convert to the framework's result value at the stream boundary.
    pub fn into_result(self) -> ResultValue {
        match self {
            Value::Null => ResultValue::Null,
            Value::Bool(v) => ResultValue::Bool(v),
            Value::Int(v) => ResultValue::Int(v),
            Value::Float(v) => ResultValue::Float(v),
            Value::Text(v) => ResultValue::Text(v),
            Value::List(items) => {
                ResultValue::List(items.into_iter().map(Value::into_result).collect())
            }
            Value::Map(entries) => ResultValue::Map(
                entries
                    .into_iter()
                    .map(|(key, value)| (ResultValue::Text(key), value.into_result()))
                    .collect(),
            ),
            Value::Vertex(id) => ResultValue::VertexRef(id),
            Value::Edge(id) => ResultValue::EdgeRef(id),
            Value::Path(path) => {
                let mut objects = Vec::with_capacity(path.nodes.len() + path.edges.len());
                for index in 0..path.nodes.len() {
                    objects.push(ResultValue::VertexRef(path.nodes[index]));
                    if let Some(edge) = path.edges.get(index) {
                        objects.push(ResultValue::EdgeRef(*edge));
                    }
                }
                ResultValue::Path(ResultPath::from_objects(objects))
            }
            // The framework result model has no temporal variant; the canonical
            // ISO-8601 text is the documented encoding.
            Value::Temporal(temporal) => ResultValue::Text(temporal.text),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Truth value of a filter predicate: only `true` passes.
    pub fn is_true(&self) -> bool {
        matches!(self, Value::Bool(true))
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(v) => Some(*v as f64),
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(items) => Some(items),
            _ => None,
        }
    }

    pub fn as_vertex(&self) -> Option<VertexId> {
        match self {
            Value::Vertex(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_edge(&self) -> Option<EdgeId> {
        match self {
            Value::Edge(id) => Some(*id),
            _ => None,
        }
    }

    /// Key used for grouping and set membership, so that two structurally equal
    /// values share a group regardless of their representation.
    pub fn group_key(&self) -> String {
        match self {
            Value::Null => "null".into(),
            Value::Bool(v) => format!("b{v}"),
            // An integral float shares a group with the equal integer, matching
            // Cypher's numeric equality.
            Value::Int(v) => format!("n{v}"),
            Value::Float(v) if is_integral(*v) => format!("n{}", *v as i64),
            Value::Float(v) => format!("f{v}"),
            Value::Text(v) => format!("t{v}"),
            Value::List(items) => {
                let mut out = String::from("l[");
                for item in items {
                    out.push_str(&item.group_key());
                    out.push(',');
                }
                out.push(']');
                out
            }
            Value::Map(entries) => {
                let mut out = String::from("m{");
                for (key, value) in entries {
                    out.push_str(key);
                    out.push(':');
                    out.push_str(&value.group_key());
                    out.push(',');
                }
                out.push('}');
                out
            }
            Value::Vertex(id) => format!("v{}", id.0),
            Value::Edge(id) => format!("e{}", id.0),
            Value::Path(path) => format!("p{path:?}"),
            Value::Temporal(temporal) => format!("d{:?}{}", temporal.kind, temporal.magnitude),
        }
    }

    /// Ordering used by `ORDER BY`, `min`, `max`, and list comparison.
    ///
    /// `None` means "not comparable", which callers treat as `null`.
    pub fn compare(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Null, _) | (_, Value::Null) => None,
            (Value::Int(left), Value::Int(right)) => Some(left.cmp(right)),
            (Value::Float(left), Value::Float(right)) => left.partial_cmp(right),
            (Value::Int(left), Value::Float(right)) => (*left as f64).partial_cmp(right),
            (Value::Float(left), Value::Int(right)) => left.partial_cmp(&(*right as f64)),
            (Value::Text(left), Value::Text(right)) => Some(left.cmp(right)),
            (Value::Bool(left), Value::Bool(right)) => Some(left.cmp(right)),
            (Value::Vertex(left), Value::Vertex(right)) => Some(left.0.cmp(&right.0)),
            (Value::Edge(left), Value::Edge(right)) => Some(left.0.cmp(&right.0)),
            (Value::Temporal(left), Value::Temporal(right)) if left.kind == right.kind => {
                Some(left.magnitude.cmp(&right.magnitude))
            }
            (Value::List(left), Value::List(right)) => {
                for (a, b) in left.iter().zip(right.iter()) {
                    match a.compare(b)? {
                        Ordering::Equal => continue,
                        other => return Some(other),
                    }
                }
                Some(left.len().cmp(&right.len()))
            }
            _ => None,
        }
    }

    /// Cypher equality: `None` means the result is `null`.
    pub fn equals(&self, other: &Value) -> Option<bool> {
        if self.is_null() || other.is_null() {
            return None;
        }
        match (self, other) {
            (Value::List(left), Value::List(right)) => {
                if left.len() != right.len() {
                    return Some(false);
                }
                let mut seen_unknown = false;
                for (a, b) in left.iter().zip(right.iter()) {
                    match a.equals(b) {
                        Some(true) => {}
                        Some(false) => return Some(false),
                        None => seen_unknown = true,
                    }
                }
                if seen_unknown {
                    None
                } else {
                    Some(true)
                }
            }
            (Value::Map(left), Value::Map(right)) => {
                if left.len() != right.len() {
                    return Some(false);
                }
                for (key, value) in left {
                    match right.iter().find(|(other, _)| other == key) {
                        Some((_, other)) => match value.equals(other) {
                            Some(true) => {}
                            Some(false) => return Some(false),
                            None => return None,
                        },
                        None => return Some(false),
                    }
                }
                Some(true)
            }
            // Values of different types are equal to nothing, not unknown.
            _ => Some(self.compare(other) == Some(Ordering::Equal)),
        }
    }
}

fn is_integral(value: f64) -> bool {
    value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_and_floats_compare_numerically() {
        assert_eq!(Value::Int(1).compare(&Value::Float(1.5)), Some(Ordering::Less));
        assert_eq!(Value::Int(2).equals(&Value::Float(2.0)), Some(true));
        assert_eq!(Value::Int(2).group_key(), Value::Float(2.0).group_key());
    }

    #[test]
    fn null_propagates_through_comparison_and_equality() {
        assert_eq!(Value::Null.compare(&Value::Int(1)), None);
        assert_eq!(Value::Null.equals(&Value::Null), None);
        assert_eq!(Value::Int(1).equals(&Value::Null), None);
        assert!(!Value::Null.is_true());
    }

    #[test]
    fn different_types_are_not_ordered() {
        assert_eq!(Value::Int(1).compare(&Value::Text("1".into())), None);
        assert_eq!(Value::Bool(true).equals(&Value::Int(1)), Some(false));
    }

    #[test]
    fn lists_compare_element_wise_then_by_length() {
        let short = Value::List(vec![Value::Int(1)]);
        let long = Value::List(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(short.compare(&long), Some(Ordering::Less));
        assert_eq!(short.equals(&long), Some(false));

        let with_null = Value::List(vec![Value::Null]);
        assert_eq!(with_null.equals(&Value::List(vec![Value::Null])), None);
    }

    #[test]
    fn maps_compare_independent_of_key_order() {
        let left = Value::Map(vec![
            ("a".into(), Value::Int(1)),
            ("b".into(), Value::Int(2)),
        ]);
        let right = Value::Map(vec![
            ("b".into(), Value::Int(2)),
            ("a".into(), Value::Int(1)),
        ]);
        assert_eq!(left.equals(&right), Some(true));
    }

    #[test]
    fn temporals_compare_within_their_family() {
        use super::super::temporal::TemporalKind;
        let day = Value::Temporal(Temporal::parse(TemporalKind::Duration, "P1D").unwrap());
        let half = Value::Temporal(Temporal::parse(TemporalKind::Duration, "PT12H").unwrap());
        assert_eq!(day.compare(&half), Some(Ordering::Greater));
    }

    #[test]
    fn paths_render_as_an_alternating_object_list() {
        let path = Value::Path(PathValue {
            nodes: vec![VertexId(1), VertexId(2)],
            edges: vec![EdgeId(7)],
        });
        match path.into_result() {
            ResultValue::Path(rendered) => {
                assert_eq!(rendered.objects.len(), 3);
                assert!(matches!(rendered.objects[0], ResultValue::VertexRef(VertexId(1))));
                assert!(matches!(rendered.objects[1], ResultValue::EdgeRef(EdgeId(7))));
            }
            other => panic!("expected a path, got {other:?}"),
        }
    }
}
