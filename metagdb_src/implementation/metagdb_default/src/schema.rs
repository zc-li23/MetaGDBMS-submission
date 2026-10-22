/**
 * @brief: Default implementation of GraphSchema, supporting statically preconfigured demo metadata.
 */

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::l1_model::{
    EdgeTypeDescriptor, GraphSchema, LabelDescriptor, PropertyKeyDescriptor, SchemaMode,
};

/**
 * @brief: Default Schema container, storing all label and type definitions via vector tables.
 */
pub struct DefaultSchema {
    mode: SchemaMode,
    labels: Vec<LabelDescriptor>,
    edge_types: Vec<EdgeTypeDescriptor>,
    property_keys: Vec<PropertyKeyDescriptor>,
}

impl DefaultSchema {
    /**
     * @brief: Construct a demo Schema, preconfigured with common Person, City, KNOWS metadata.
     */
    pub fn demo_schema() -> Self {
        Self {
            mode: SchemaMode::Hybrid,
            labels: vec![
                LabelDescriptor {
                    id: LabelId(1),
                    name: "Person".into(),
                },
                LabelDescriptor {
                    id: LabelId(2),
                    name: "City".into(),
                },
            ],
            edge_types: vec![
                EdgeTypeDescriptor {
                    id: EdgeTypeId(1),
                    name: "KNOWS".into(),
                },
                EdgeTypeDescriptor {
                    id: EdgeTypeId(2),
                    name: "LIVES_IN".into(),
                },
            ],
            property_keys: vec![
                PropertyKeyDescriptor {
                    id: PropertyKeyId(1),
                    name: "name".into(),
                },
                PropertyKeyDescriptor {
                    id: PropertyKeyId(2),
                    name: "age".into(),
                },
            ],
        }
    }

    /**
     * @brief: Construct a generic Schema, suitable for .graph file bulk import scenarios.
     *
     * Defines a single generic vertex label "Node", a single edge type "EDGE",
     * and two property keys "label" (mapping to v_label) and "community" (mapping to v_com).
     */
    pub fn generic_schema() -> Self {
        Self {
            mode: SchemaMode::Hybrid,
            labels: vec![LabelDescriptor {
                id: LabelId(0),
                name: "Node".into(),
            }],
            edge_types: vec![EdgeTypeDescriptor {
                id: EdgeTypeId(0),
                name: "EDGE".into(),
            }],
            property_keys: vec![
                PropertyKeyDescriptor {
                    id: PropertyKeyId(1),
                    name: "label".into(),
                },
                PropertyKeyDescriptor {
                    id: PropertyKeyId(2),
                    name: "community".into(),
                },
            ],
        }
    }
}

impl GraphSchema for DefaultSchema {
    /**
     * @brief: Get the Schema strictness mode.
     */
    fn schema_mode(&self) -> SchemaMode {
        self.mode
    }

    /**
     * @brief: Get all registered vertex label descriptors.
     */
    fn vertex_labels(&self) -> Vec<LabelDescriptor> {
        self.labels.clone()
    }

    /**
     * @brief: Get all registered edge type descriptors.
     */
    fn edge_types(&self) -> Vec<EdgeTypeDescriptor> {
        self.edge_types.clone()
    }

    /**
     * @brief: Get all registered property key descriptors.
     */
    fn property_keys(&self) -> Vec<PropertyKeyDescriptor> {
        self.property_keys.clone()
    }

    /**
     * @brief: Look up a label ID by name.
     */
    fn label_by_name(&self, name: &str) -> Option<LabelId> {
        self.labels.iter().find(|d| d.name == name).map(|d| d.id)
    }

    /**
     * @brief: Look up an edge type ID by name.
     */
    fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId> {
        self.edge_types
            .iter()
            .find(|d| d.name == name)
            .map(|d| d.id)
    }

    /**
     * @brief: Look up a property key ID by name.
     */
    fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId> {
        self.property_keys
            .iter()
            .find(|d| d.name == name)
            .map(|d| d.id)
    }

    /**
     * @brief: Get the name for a given label ID.
     */
    fn label_name(&self, id: LabelId) -> Option<&str> {
        self.labels
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.as_str())
    }

    /**
     * @brief: Get the name for a given property key ID.
     */
    fn property_key_name(&self, id: PropertyKeyId) -> Option<&str> {
        self.property_keys
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.as_str())
    }

    /**
     * @brief: Get the name for a given edge type ID.
     */
    fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str> {
        self.edge_types
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.as_str())
    }
}
