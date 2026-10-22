//! Typed normalization of query results for Benchmark v2.
//!
//! Benchmark v2 states its expectations as *typed values* rather than as opaque
//! strings. This module owns the single authoritative conversion from the
//! engine's [`ResultValue`] stream into that typed model, and the canonical
//! rendering used for both comparison and human-readable reports.
//!
//! Normalization rules (documented in `benchmark/README.md`):
//!
//! * floats that hold an integral value render as integers, so `30` and `30.0`
//!   compare equal;
//! * maps compare independent of key insertion order;
//! * sets compare independent of member order and drop duplicates;
//! * lists and paths keep their element order even when rows are unordered —
//!   only the row order is relaxed by `ordered: false`.
//!
//! Equality between two values is defined as equality of their canonical
//! rendering, which is exactly the normalization the rules above describe.

use metagdb_core::common::property::{PropertyValue, ResultValue};

use crate::json::Json;

/// A normalized result value.
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Val>),
    /// Unordered collection; rendering sorts members.
    Set(Vec<Val>),
    /// Map with string keys; rendering sorts by key.
    Map(Vec<(String, Val)>),
    /// Vertex reference addressed by fixture-local insertion ordinal.
    Vertex(i64),
    /// Edge reference addressed by fixture-local insertion ordinal.
    Edge(i64),
    Path(Vec<Val>),
    Property {
        key: String,
        value: Box<Val>,
        element: Option<Box<Val>>,
    },
    /// A cell the profile refuses to normalize; never a strict pass.
    Unresolved(String),
}

/// True when the value tree contains a cell the profile cannot normalize.
pub fn is_unresolved(value: &Val) -> bool {
    match value {
        Val::Unresolved(_) => true,
        Val::List(items) | Val::Set(items) | Val::Path(items) => items.iter().any(is_unresolved),
        Val::Map(entries) => entries.iter().any(|(_, item)| is_unresolved(item)),
        Val::Property { value, element, .. } => {
            is_unresolved(value) || element.as_deref().is_some_and(is_unresolved)
        }
        _ => false,
    }
}

/// Rows that contain an unresolved cell anywhere.
pub fn rows_unresolved(rows: &[Vec<Val>]) -> bool {
    rows.iter().flatten().any(is_unresolved)
}

// ---------------------------------------------------------------------------
// Engine values -> normalized values
// ---------------------------------------------------------------------------

pub fn from_result_value(value: &ResultValue) -> Val {
    match value {
        ResultValue::Null => Val::Null,
        ResultValue::Bool(v) => Val::Bool(*v),
        ResultValue::Int(v) => Val::Int(*v),
        ResultValue::Float(v) => Val::Float(*v),
        ResultValue::Text(v) => Val::Text(v.clone()),
        ResultValue::VertexRef(v) => Val::Vertex(v.0 as i64),
        ResultValue::EdgeRef(v) => Val::Edge(v.0 as i64),
        ResultValue::Property(v) => from_property_value(v),
        ResultValue::List(items) => Val::List(items.iter().map(from_result_value).collect()),
        ResultValue::Set(items) => Val::Set(items.iter().map(from_result_value).collect()),
        ResultValue::Map(entries) => Val::Map(
            entries
                .iter()
                .map(|(key, item)| (render_key(&from_result_value(key)), from_result_value(item)))
                .collect(),
        ),
        ResultValue::Path(path) => Val::Path(path.objects.iter().map(from_result_value).collect()),
        ResultValue::PropertyElement(element) => Val::Property {
            key: element.key.clone(),
            value: Box::new(from_result_value(&element.value)),
            element: Some(Box::new(from_result_value(&element.element))),
        },
    }
}

fn from_property_value(value: &PropertyValue) -> Val {
    match value {
        PropertyValue::Null => Val::Null,
        PropertyValue::Bool(v) => Val::Bool(*v),
        PropertyValue::Int(v) => Val::Int(*v),
        PropertyValue::Float(v) => Val::Float(*v),
        PropertyValue::Text(v) => Val::Text(v.clone()),
    }
}

pub fn from_stream(stream: &metagdb_core::l4_compute::engine::ResultStream) -> Vec<Vec<Val>> {
    stream
        .rows
        .iter()
        .map(|row| row.iter().map(from_result_value).collect())
        .collect()
}

// ---------------------------------------------------------------------------
// JSON (case files) -> normalized values
// ---------------------------------------------------------------------------

/// Decode a Benchmark v2 JSON value into the typed model.
pub fn from_json(value: &Json) -> Result<Val, String> {
    Ok(match value {
        Json::Null => Val::Null,
        Json::Bool(v) => Val::Bool(*v),
        Json::Int(v) => Val::Int(*v),
        Json::Float(v) => Val::Float(*v),
        Json::Text(v) => Val::Text(v.clone()),
        Json::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(from_json(item)?);
            }
            Val::List(out)
        }
        Json::Object(entries) => {
            if entries.len() != 1 {
                return Err(
                    "plain JSON objects are ambiguous; wrap map values in {\"$map\": ...}".into(),
                );
            }
            let (tag, payload) = entries.iter().next().expect("length checked");
            match tag.as_str() {
                "$set" => Val::Set(decode_items(payload)?),
                "$path" => Val::Path(decode_items(payload)?),
                "$vertex" => Val::Vertex(payload.as_i64().ok_or("$vertex must be an integer")?),
                "$edge" => Val::Edge(payload.as_i64().ok_or("$edge must be an integer")?),
                "$unresolved" => Val::Unresolved(
                    payload
                        .as_str()
                        .ok_or("$unresolved must be a string")?
                        .to_string(),
                ),
                "$map" => {
                    let entries = payload
                        .as_object()
                        .ok_or("$map payload must be an object")?;
                    let mut out = Vec::with_capacity(entries.len());
                    for (key, item) in entries {
                        out.push((key.clone(), from_json(item)?));
                    }
                    Val::Map(out)
                }
                "$property" => {
                    let object = payload
                        .as_object()
                        .ok_or("$property payload must be an object")?;
                    let key = object
                        .get("key")
                        .and_then(Json::as_str)
                        .ok_or("$property requires a string `key`")?
                        .to_string();
                    let inner = object.get("value").ok_or("$property requires a `value`")?;
                    Val::Property {
                        key,
                        value: Box::new(from_json(inner)?),
                        element: match object.get("element") {
                            Some(element) => Some(Box::new(from_json(element)?)),
                            None => None,
                        },
                    }
                }
                other => return Err(format!("unknown value tag {other:?}")),
            }
        }
    })
}

/// Encode a normalized value for the private benchmark child-process boundary.
pub fn to_json(value: &Val) -> Json {
    match value {
        Val::Null => Json::Null,
        Val::Bool(value) => Json::Bool(*value),
        Val::Int(value) => Json::Int(*value),
        Val::Float(value) => Json::Float(*value),
        Val::Text(value) => Json::Text(value.clone()),
        Val::List(items) => Json::Array(items.iter().map(to_json).collect()),
        Val::Set(items) => tagged("$set", Json::Array(items.iter().map(to_json).collect())),
        Val::Map(entries) => tagged(
            "$map",
            Json::Object(
                entries
                    .iter()
                    .map(|(key, value)| (key.clone(), to_json(value)))
                    .collect(),
            ),
        ),
        Val::Vertex(id) => tagged("$vertex", Json::Int(*id)),
        Val::Edge(id) => tagged("$edge", Json::Int(*id)),
        Val::Path(items) => tagged("$path", Json::Array(items.iter().map(to_json).collect())),
        Val::Property {
            key,
            value,
            element,
        } => {
            let mut payload = std::collections::BTreeMap::from([
                ("key".to_string(), Json::Text(key.clone())),
                ("value".to_string(), to_json(value)),
            ]);
            if let Some(element) = element {
                payload.insert("element".to_string(), to_json(element));
            }
            tagged("$property", Json::Object(payload))
        }
        Val::Unresolved(value) => tagged("$unresolved", Json::Text(value.clone())),
    }
}

fn tagged(name: &str, payload: Json) -> Json {
    Json::Object(std::collections::BTreeMap::from([(
        name.to_string(),
        payload,
    )]))
}

fn decode_items(value: &Json) -> Result<Vec<Val>, String> {
    let items = value
        .as_array()
        .ok_or("tagged value payload must be an array")?;
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(from_json(item)?);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Canonical rendering and comparison
// ---------------------------------------------------------------------------

fn float_text(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
        (value as i64).to_string()
    } else {
        format!("{value}")
    }
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Render a map key.
///
/// Map keys are compared as their plain textual form, so a `Text` key renders
/// without quotes (`Person: 5`) while every other key kind falls back to the
/// ordinary canonical rendering. Case files spell map keys the same way.
pub fn render_key(value: &Val) -> String {
    match value {
        Val::Text(text) => text.clone(),
        other => render(other),
    }
}

pub fn render(value: &Val) -> String {
    match value {
        Val::Null => "null".into(),
        Val::Bool(true) => "true".into(),
        Val::Bool(false) => "false".into(),
        Val::Int(v) => v.to_string(),
        Val::Float(v) => float_text(*v),
        Val::Text(v) => format!("\"{}\"", escape(v)),
        Val::List(items) => format!("[{}]", render_all(items)),
        Val::Set(items) => {
            let mut rendered: Vec<String> = items.iter().map(render).collect();
            rendered.sort();
            rendered.dedup();
            format!("set{{{}}}", rendered.join(", "))
        }
        Val::Map(entries) => {
            let mut rendered: Vec<(String, String)> = entries
                .iter()
                .map(|(key, item)| (key.clone(), render(item)))
                .collect();
            rendered.sort();
            format!(
                "{{{}}}",
                rendered
                    .iter()
                    .map(|(key, item)| format!("{key}:{item}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Val::Vertex(id) => format!("v[{id}]"),
        Val::Edge(id) => format!("e[{id}]"),
        Val::Path(items) => format!("path[{}]", render_all(items)),
        Val::Property {
            key,
            value,
            element,
        } => match element {
            Some(element) => format!("p{{{key}={}@{}}}", render(value), render(element)),
            None => format!("p{{{key}={}}}", render(value)),
        },
        Val::Unresolved(text) => format!("unresolved({text})"),
    }
}

fn render_all(items: &[Val]) -> String {
    items.iter().map(render).collect::<Vec<_>>().join(", ")
}

pub fn render_rows(rows: &[Vec<Val>], ordered: bool) -> String {
    let mut rendered: Vec<String> = rows
        .iter()
        .map(|row| format!("[{}]", render_all(row)))
        .collect();
    if !ordered {
        rendered.sort();
    }
    format!("[{}]", rendered.join(", "))
}

/// Compare an actual result set with an expectation.
///
/// `ordered` relaxes only the *row* order. Element order inside a list or a
/// path cell is always significant, matching the documented rules.
pub fn rows_match(actual: &[Vec<Val>], expected: &[Vec<Val>], ordered: bool) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    let mut actual_keys: Vec<String> = actual.iter().map(|row| render_row(row)).collect();
    let mut expected_keys: Vec<String> = expected.iter().map(|row| render_row(row)).collect();
    if !ordered {
        actual_keys.sort();
        expected_keys.sort();
    }
    actual_keys == expected_keys
}

fn render_row(row: &[Val]) -> String {
    let mut out = String::from("[");
    for (index, cell) in row.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&render(cell));
    }
    out.push(']');
    out
}

/// Column-count of every row, used when a corpus only guarantees result shape.
pub fn shape_of(rows: &[Vec<Val>]) -> Vec<usize> {
    let mut widths: Vec<usize> = rows.iter().map(|row| row.len()).collect();
    widths.sort_unstable();
    widths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ints(values: &[i64]) -> Vec<Val> {
        values.iter().map(|v| Val::Int(*v)).collect()
    }

    #[test]
    fn unordered_relaxes_row_order_only() {
        let actual = vec![vec![Val::List(ints(&[2, 1]))], vec![Val::List(ints(&[3]))]];
        let expected = vec![
            vec![Val::List(vec![Val::Int(3)])],
            vec![Val::List(ints(&[2, 1]))],
        ];
        assert!(rows_match(&actual, &expected, false));
        assert!(!rows_match(&actual, &expected, true));
    }

    #[test]
    fn unordered_still_compares_list_element_order() {
        let actual = vec![vec![Val::List(ints(&[1, 2]))]];
        let expected = vec![vec![Val::List(ints(&[2, 1]))]];
        assert!(!rows_match(&actual, &expected, false));
    }

    #[test]
    fn sets_and_maps_normalize_order() {
        let actual = vec![vec![Val::Set(ints(&[3, 1, 2]))]];
        let expected = vec![vec![Val::Set(ints(&[1, 2, 3]))]];
        assert!(rows_match(&actual, &expected, false));

        let actual = vec![vec![Val::Map(vec![
            ("b".into(), Val::Int(2)),
            ("a".into(), Val::Int(1)),
        ])]];
        let expected = vec![vec![Val::Map(vec![
            ("a".into(), Val::Int(1)),
            ("b".into(), Val::Int(2)),
        ])]];
        assert!(rows_match(&actual, &expected, true));
    }

    #[test]
    fn integral_floats_compare_as_integers() {
        assert_eq!(render(&Val::Float(30.0)), "30");
        assert!(rows_match(
            &vec![vec![Val::Float(30.0)]],
            &vec![vec![Val::Int(30)]],
            true
        ));
        assert_eq!(render(&Val::Float(29.6)), "29.6");
    }

    #[test]
    fn empty_results_match_empty_expectations() {
        assert!(rows_match(&[], &[], false));
        assert!(!rows_match(&[vec![Val::Null]], &[], false));
    }

    #[test]
    fn property_payloads_round_trip_through_json() {
        let value = Val::Property {
            key: "name".into(),
            value: Box::new(Val::Text("A".into())),
            element: Some(Box::new(Val::Vertex(1))),
        };
        let json = crate::json::parse(
            "{\"$property\":{\"key\":\"name\",\"value\":\"A\",\"element\":{\"$vertex\":1}}}",
        )
        .unwrap();
        assert_eq!(from_json(&json).unwrap(), value);
        assert_eq!(render(&value), "p{name=\"A\"@v[1]}");
    }

    #[test]
    fn rejects_plain_objects() {
        let json = crate::json::parse("{\"a\":1}").unwrap();
        assert!(from_json(&json).is_err());
    }
}
