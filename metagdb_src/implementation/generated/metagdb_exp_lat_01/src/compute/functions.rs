//! Scalar Cypher function library.
//!
//! Functions that need graph access (labels/properties/type) receive an
//! [`EvalCtx`]. Temporal constructors return dedicated value variants so the
//! comparison operators remain well-defined without a date/time storage type.

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::{Error, Result};

use super::ir::{value_from_property, Value};

/// Evaluation context shared by expressions and functions.
pub struct EvalCtx<'a> {
    pub storage: &'a dyn StorageEngine,
    pub txn: &'a TxnContext,
    pub schema: &'a dyn GraphSchema,
}

pub fn call(ctx: &EvalCtx<'_>, name: &str, args: &[Value]) -> Result<Value> {
    match name {
        "size" | "length" => match args.first() {
            Some(Value::Null) | None => Ok(Value::Null),
            Some(Value::Text(text)) => Ok(Value::Int(text.chars().count() as i64)),
            Some(Value::List(items)) => Ok(Value::Int(items.len() as i64)),
            Some(Value::Map(entries)) => Ok(Value::Int(entries.len() as i64)),
            Some(Value::Path(items)) => Ok(Value::Int(if name == "length" {
                items.iter().filter(|item| matches!(item, Value::Rel(_))).count() as i64
            } else {
                items.len() as i64
            })),
            Some(Value::Node(_)) | Some(Value::Rel(_)) => Ok(Value::Int(1)),
            Some(other) => Err(fn_error(name, other)),
        },
        "head" => match args.first() {
            Some(Value::List(items)) => Ok(items.first().cloned().unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "last" => match args.first() {
            Some(Value::List(items)) => Ok(items.last().cloned().unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "tail" => match args.first() {
            Some(Value::List(items)) => Ok(Value::List(items.iter().skip(1).cloned().collect())),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "reverse" => match args.first() {
            Some(Value::List(items)) => {
                Ok(Value::List(items.iter().rev().cloned().collect()))
            }
            Some(Value::Text(text)) => Ok(Value::Text(text.chars().rev().collect())),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "range" => range(args),
        "coalesce" => Ok(args
            .iter()
            .find(|value| !value.is_null())
            .cloned()
            .unwrap_or(Value::Null)),
        "abs" => numeric_unary(args, |value| value.abs(), |value| value.abs()),
        "ceil" => float_unary(args, f64::ceil),
        "floor" => float_unary(args, f64::floor),
        "round" => float_unary(args, f64::round),
        "sqrt" => float_unary(args, f64::sqrt),
        "sign" => match args.first() {
            Some(Value::Int(value)) => Ok(Value::Int(value.signum())),
            Some(Value::Float(value)) => Ok(Value::Int(if *value == 0.0 {
                0
            } else if *value > 0.0 {
                1
            } else {
                -1
            })),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "tostring" => Ok(match args.first() {
            Some(Value::Null) | None => Value::Null,
            Some(Value::Int(value)) => Value::Text(value.to_string()),
            Some(Value::Float(value)) => Value::Text(format_float(*value)),
            Some(Value::Text(value)) => Value::Text(value.clone()),
            Some(Value::Bool(value)) => Value::Text(value.to_string()),
            Some(other) => return Err(fn_error(name, other)),
        }),
        "tointeger" => Ok(match args.first() {
            Some(Value::Int(value)) => Value::Int(*value),
            Some(Value::Float(value)) => Value::Int(*value as i64),
            Some(Value::Text(value)) => match value.trim().parse::<i64>() {
                Ok(parsed) => Value::Int(parsed),
                Err(_) => match value.trim().parse::<f64>() {
                    Ok(parsed) => Value::Int(parsed as i64),
                    Err(_) => Value::Null,
                },
            },
            Some(Value::Bool(value)) => Value::Int(i64::from(*value)),
            _ => Value::Null,
        }),
        "tofloat" => Ok(match args.first() {
            Some(Value::Int(value)) => Value::Float(*value as f64),
            Some(Value::Float(value)) => Value::Float(*value),
            Some(Value::Text(value)) => match value.trim().parse::<f64>() {
                Ok(parsed) => Value::Float(parsed),
                Err(_) => Value::Null,
            },
            _ => Value::Null,
        }),
        "toboolean" => Ok(match args.first() {
            Some(Value::Bool(value)) => Value::Bool(*value),
            Some(Value::Text(value)) => match value.trim().to_ascii_lowercase().as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => Value::Null,
            },
            _ => Value::Null,
        }),
        "toupper" => text_unary(args, |text| text.to_uppercase()),
        "tolower" => text_unary(args, |text| text.to_lowercase()),
        "trim" => text_unary(args, |text| text.trim().to_string()),
        "substring" => substring(args),
        "left" => text_slice(args, true),
        "right" => text_slice(args, false),
        "replace" => replace(args),
        "split" => split(args),
        "exists" => Ok(Value::Bool(matches!(args.first(), Some(value) if !value.is_null()))),
        "keys" => with_node(ctx, args, |record| {
            let mut keys: Vec<Value> = record
                .properties
                .keys()
                .filter_map(|key| ctx.schema.property_key_name(*key))
                .map(|name| Value::Text(name.to_string()))
                .collect();
            keys.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
            Value::List(keys)
        }),
        "properties" => with_node(ctx, args, |record| {
            let mut entries: Vec<(String, Value)> = record
                .properties
                .iter()
                .filter_map(|(key, value)| {
                    ctx.schema
                        .property_key_name(*key)
                        .map(|name| (name.to_string(), value_from_property(value)))
                })
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Map(entries)
        }),
        "labels" => with_node(ctx, args, |record| {
            Value::List(
                record
                    .labels
                    .iter()
                    .filter_map(|label| ctx.schema.label_name(*label))
                    .map(|name| Value::Text(name.to_string()))
                    .collect(),
            )
        }),
        "type" => match args.first() {
            Some(Value::Rel(id)) => {
                let edge = ctx
                    .storage
                    .get_edge(*id, ctx.txn)?
                    .ok_or_else(|| Error::Execute(format!("edge {} no longer exists", id.0)))?;
                Ok(match ctx.schema.edge_type_name(edge.etype) {
                    Some(name) => Value::Text(name.to_string()),
                    None => Value::Null,
                })
            }
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "nodes" => match args.first() {
            Some(Value::Path(items)) => Ok(Value::List(
                items
                    .iter()
                    .filter(|item| matches!(item, Value::Node(_)))
                    .cloned()
                    .collect(),
            )),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "relationships" => match args.first() {
            Some(Value::Path(items)) => Ok(Value::List(
                items
                    .iter()
                    .filter(|item| matches!(item, Value::Rel(_)))
                    .cloned()
                    .collect(),
            )),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "date" => match args.first() {
            Some(Value::Text(text)) => parse_date(text).map(|date| date.unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "datetime" => match args.first() {
            Some(Value::Text(text)) => parse_datetime(text).map(|value| value.unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "time" => match args.first() {
            Some(Value::Text(text)) => parse_time(text).map(|value| value.unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        "duration" => match args.first() {
            Some(Value::Text(text)) => parse_duration(text).map(|value| value.unwrap_or(Value::Null)),
            Some(Value::Null) | None => Ok(Value::Null),
            Some(other) => Err(fn_error(name, other)),
        },
        _ => Err(Error::Unsupported(format!(
            "function `{name}` is outside the selected Cypher profile"
        ))),
    }
}

fn fn_error(name: &str, value: &Value) -> Error {
    Error::Execute(format!("function `{name}` received an unsupported argument: {value:?}"))
}

fn with_node<F>(ctx: &EvalCtx<'_>, args: &[Value], body: F) -> Result<Value>
where
    F: FnOnce(&metagdb_core::l2_storage::VertexRecord) -> Value,
{
    match args.first() {
        Some(Value::Node(id)) => match ctx.storage.get_vertex(*id, ctx.txn)? {
            Some(record) => Ok(body(&record)),
            None => Ok(Value::Null),
        },
        Some(Value::Null) | None => Ok(Value::Null),
        Some(other) => Err(Error::Execute(format!(
            "function received a non-node argument: {other:?}"
        ))),
    }
}

fn numeric_unary(args: &[Value], int_op: fn(i64) -> i64, float_op: fn(f64) -> f64) -> Result<Value> {
    match args.first() {
        Some(Value::Int(value)) => Ok(Value::Int(int_op(*value))),
        Some(Value::Float(value)) => Ok(Value::Float(float_op(*value))),
        Some(Value::Null) | None => Ok(Value::Null),
        Some(other) => Err(Error::Execute(format!("numeric function got {other:?}"))),
    }
}

fn float_unary(args: &[Value], op: fn(f64) -> f64) -> Result<Value> {
    match args.first() {
        Some(Value::Int(value)) => Ok(Value::Float(op(*value as f64))),
        Some(Value::Float(value)) => Ok(Value::Float(op(*value))),
        Some(Value::Null) | None => Ok(Value::Null),
        Some(other) => Err(Error::Execute(format!("numeric function got {other:?}"))),
    }
}

fn text_unary(args: &[Value], op: fn(&str) -> String) -> Result<Value> {
    match args.first() {
        Some(Value::Text(value)) => Ok(Value::Text(op(value))),
        Some(Value::Null) | None => Ok(Value::Null),
        Some(other) => Err(Error::Execute(format!("string function got {other:?}"))),
    }
}

fn range(args: &[Value]) -> Result<Value> {
    let start = match args.first() {
        Some(Value::Int(value)) => *value,
        _ => return Ok(Value::Null),
    };
    let end = match args.get(1) {
        Some(Value::Int(value)) => *value,
        _ => return Ok(Value::Null),
    };
    let step = match args.get(2) {
        Some(Value::Int(value)) => *value,
        None => 1,
        _ => return Ok(Value::Null),
    };
    if step == 0 {
        return Err(Error::Execute("range step must not be zero".into()));
    }
    let mut values = Vec::new();
    let mut current = start;
    if step > 0 {
        while current <= end {
            values.push(Value::Int(current));
            current += step;
        }
    } else {
        while current >= end {
            values.push(Value::Int(current));
            current += step;
        }
    }
    Ok(Value::List(values))
}

fn substring(args: &[Value]) -> Result<Value> {
    let text = match args.first() {
        Some(Value::Text(value)) => value,
        Some(Value::Null) | None => return Ok(Value::Null),
        Some(other) => return Err(Error::Execute(format!("substring got {other:?}"))),
    };
    let start = match args.get(1) {
        Some(Value::Int(value)) => (*value).max(0) as usize,
        _ => return Ok(Value::Null),
    };
    let chars: Vec<char> = text.chars().collect();
    let start = start.min(chars.len());
    let length = match args.get(2) {
        Some(Value::Int(value)) => (*value).max(0) as usize,
        None => chars.len() - start,
        _ => return Ok(Value::Null),
    };
    let end = (start + length).min(chars.len());
    Ok(Value::Text(chars[start..end].iter().collect()))
}

fn text_slice(args: &[Value], from_left: bool) -> Result<Value> {
    let text = match args.first() {
        Some(Value::Text(value)) => value,
        Some(Value::Null) | None => return Ok(Value::Null),
        Some(other) => return Err(Error::Execute(format!("string function got {other:?}"))),
    };
    let count = match args.get(1) {
        Some(Value::Int(value)) => (*value).max(0) as usize,
        _ => return Ok(Value::Null),
    };
    let chars: Vec<char> = text.chars().collect();
    let count = count.min(chars.len());
    let slice = if from_left {
        &chars[..count]
    } else {
        &chars[chars.len() - count..]
    };
    Ok(Value::Text(slice.iter().collect()))
}

fn replace(args: &[Value]) -> Result<Value> {
    match (args.first(), args.get(1), args.get(2)) {
        (Some(Value::Text(text)), Some(Value::Text(search)), Some(Value::Text(replacement))) => {
            Ok(Value::Text(text.replace(search.as_str(), replacement)))
        }
        (Some(Value::Null), _, _) | (_, Some(Value::Null), _) | (_, _, Some(Value::Null)) => {
            Ok(Value::Null)
        }
        _ => Ok(Value::Null),
    }
}

fn split(args: &[Value]) -> Result<Value> {
    match (args.first(), args.get(1)) {
        (Some(Value::Text(text)), Some(Value::Text(separator))) => Ok(Value::List(
            text.split(separator.as_str())
                .map(|part| Value::Text(part.to_string()))
                .collect(),
        )),
        _ => Ok(Value::Null),
    }
}

fn format_float(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 9e15 {
        format!("{value:.1}")
    } else {
        value.to_string()
    }
}

fn parse_date(text: &str) -> Result<Option<Value>> {
    let parts: Vec<&str> = text.trim().split('-').collect();
    if parts.len() != 3 {
        return Ok(None);
    }
    let year = match parts[0].parse::<i32>() {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let month = match parts[1].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let day = match parts[2].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    Ok(Some(Value::Date(year, month, day)))
}

fn parse_time(text: &str) -> Result<Option<Value>> {
    let parts: Vec<&str> = text.trim().split(':').collect();
    if parts.len() < 2 {
        return Ok(None);
    }
    let hour = match parts[0].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let minute = match parts[1].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let second = parts
        .get(2)
        .and_then(|value| value.trim_end_matches('Z').parse::<u32>().ok())
        .unwrap_or(0);
    Ok(Some(Value::Time(hour, minute, second)))
}

fn parse_datetime(text: &str) -> Result<Option<Value>> {
    let (date_part, time_part) = match text.trim().split_once(['T', ' ']) {
        Some((date, time)) => (date, time),
        None => (text.trim(), "00:00:00"),
    };
    let date = match parse_date(date_part)? {
        Some(Value::Date(y, m, d)) => (y as i64, m as i64, d as i64),
        _ => return Ok(None),
    };
    let time = match parse_time(time_part)? {
        Some(Value::Time(h, m, s)) => (h as i64, m as i64, s as i64),
        _ => (0, 0, 0),
    };
    let encoded = ((((date.0 * 100 + date.1) * 100 + date.2) * 100 + time.0) * 100 + time.1) * 100
        + time.2;
    Ok(Some(Value::DateTime(encoded)))
}

fn parse_duration(text: &str) -> Result<Option<Value>> {
    let mut chars = text.trim().chars().peekable();
    if chars.next() != Some('P') {
        return Ok(None);
    }
    let mut seconds: i64 = 0;
    let mut number = String::new();
    let mut in_time = false;
    while let Some(ch) = chars.next() {
        match ch {
            'T' => in_time = true,
            '0'..='9' | '.' => number.push(ch),
            unit => {
                let value: f64 = number.parse().unwrap_or(0.0);
                number.clear();
                let factor = match (unit, in_time) {
                    ('D', false) => 86_400.0,
                    ('H', true) => 3_600.0,
                    ('M', true) => 60.0,
                    ('S', true) => 1.0,
                    ('W', false) => 604_800.0,
                    _ => 0.0,
                };
                seconds += (value * factor) as i64;
            }
        }
    }
    Ok(Some(Value::Duration(seconds)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> (crate::schema::DynamicSchema,) {
        (crate::schema::DynamicSchema::schemaless(),)
    }

    #[test]
    fn range_and_string_functions() {
        let (schema,) = ctx();
        let storage = crate::storage::GraphStore::with_base_dir(
            std::sync::Arc::new(crate::schema::DynamicSchema::schemaless()),
            std::env::temp_dir().join(format!(
                "mgdb_fn_{}_{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            )),
        )
        .unwrap();
        let txn = metagdb_core::l3_txn::TxnContext {
            txn_id: 1,
            isolation: metagdb_core::l3_txn::IsolationLevel::SnapshotIsolation,
            read_only: true,
            start_ts: 1,
        };
        let context = EvalCtx {
            storage: &storage,
            txn: &txn,
            schema: &schema,
        };
        assert_eq!(
            call(&context, "range", &[Value::Int(0), Value::Int(5), Value::Int(2)]).unwrap(),
            Value::List(vec![Value::Int(0), Value::Int(2), Value::Int(4)])
        );
        assert_eq!(
            call(&context, "split", &[Value::Text("NYC".into()), Value::Text("Y".into())]).unwrap(),
            Value::List(vec![Value::Text("N".into()), Value::Text("C".into())])
        );
        assert_eq!(
            call(&context, "substring", &[Value::Text("NYC".into()), Value::Int(0), Value::Int(2)]).unwrap(),
            Value::Text("NY".into())
        );
    }

    #[test]
    fn temporal_comparisons_are_ordered() {
        let value = parse_date("2020-01-01").unwrap().unwrap();
        let later = parse_date("2020-01-02").unwrap().unwrap();
        assert_eq!(value.total_cmp(&later), std::cmp::Ordering::Less);
        match parse_duration("P1D").unwrap().unwrap() {
            Value::Duration(seconds) => assert_eq!(seconds, 86_400),
            other => panic!("unexpected {other:?}"),
        }
    }
}
