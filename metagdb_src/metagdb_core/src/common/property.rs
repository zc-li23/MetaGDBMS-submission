/**
 * @brief: Property value and query result value definitions, covering the graph
 *         data property system and query output types.
 */

use std::collections::BTreeMap;

use super::ids::{EdgeId, PropertyKeyId, VertexId};

/**
 * @brief: Property value types defined by the property graph model.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

impl PropertyValue {
    /**
     * @brief: Attempt to convert the property value to i64.
     * @return: Some(i64) if the value is an integer type, otherwise None.
     */
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PropertyValue::Int(v) => Some(*v),
            _ => None,
        }
    }
}

/**
 * @brief: Property map on a vertex or edge.
 */
pub type PropertyMap = BTreeMap<PropertyKeyId, PropertyValue>;

/**
 * @brief: Query result cell value, additionally supporting vertex/edge references
 *         and wrapped properties beyond PropertyValue.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum ResultValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    VertexRef(VertexId),
    EdgeRef(EdgeId),
    Property(PropertyValue),
    /**
     * @brief: Runtime list value.
     */
    List(Vec<ResultValue>),
    /**
     * @brief: Runtime map value, as an ordered list of key/value pairs.
     */
    Map(Vec<(ResultValue, ResultValue)>),
    /**
     * @brief: Runtime path value: the sequence of visited objects together with
     *         the step labels attached at each position.
     */
    Path(ResultPath),
    /**
     * @brief: A property element: a key/value pair bound to its owning element.
     */
    PropertyElement(Box<PropertyElement>),
    /**
     * @brief: A set of distinct values.
     */
    Set(Vec<ResultValue>),
}

/**
 * @brief: A property element, as returned by `properties()`-style operations:
 *         the property key, its value, and a reference to the element that owns
 *         it.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyElement {
    pub key: String,
    pub value: ResultValue,
    pub element: ResultValue,
}

/**
 * @brief: A traversal path. `objects` holds the visited objects in order and
 *         `labels` holds, for each position, the step labels (`as("…")`) bound
 *         at that position.
 */
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResultPath {
    pub objects: Vec<ResultValue>,
    pub labels: Vec<Vec<String>>,
}

impl ResultPath {
    /**
     * @brief: Construct an empty path.
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * @brief: Construct a path from objects with no per-position labels.
     */
    pub fn from_objects(objects: Vec<ResultValue>) -> Self {
        let labels = vec![Vec::new(); objects.len()];
        Self { objects, labels }
    }
}

impl From<PropertyValue> for ResultValue {
    /**
     * @brief: Convert a property value to a wrapped result value type.
     * @param v: The source property value.
     * @return: The wrapped ResultValue.
     */
    fn from(v: PropertyValue) -> Self {
        ResultValue::Property(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ids::{EdgeId, VertexId};

    #[test]
    fn result_path_holds_objects_and_labels() {
        let mut p = ResultPath::from_objects(vec![
            ResultValue::VertexRef(VertexId(1)),
            ResultValue::Text("x".into()),
        ]);
        p.labels[0].push("a".into());
        assert_eq!(p.objects.len(), 2);
        assert_eq!(p.labels[0], vec!["a".to_string()]);
        assert_eq!(p.labels[1], Vec::<String>::new());
    }

    #[test]
    fn structured_collections_keep_nested_values() {
        let map = ResultValue::Map(vec![(
            ResultValue::Text("k".into()),
            ResultValue::List(vec![ResultValue::Int(1), ResultValue::Int(2)]),
        )]);
        let set = ResultValue::Set(vec![ResultValue::Int(1), ResultValue::Int(2)]);

        assert!(matches!(map, ResultValue::Map(entries) if entries.len() == 1));
        assert!(matches!(set, ResultValue::Set(values) if values.len() == 2));
    }

    #[test]
    fn property_element_keeps_its_owner() {
        let property = PropertyElement {
            key: "weight".into(),
            value: ResultValue::Float(0.5),
            element: ResultValue::EdgeRef(EdgeId(7)),
        };

        assert!(matches!(
            property.element,
            ResultValue::EdgeRef(EdgeId(7))
        ));
    }
}
