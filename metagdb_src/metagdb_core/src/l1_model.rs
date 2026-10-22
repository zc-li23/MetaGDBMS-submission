/**
 * @brief: L1 data model layer contract, defining the property graph Schema
 *         description and metadata access interface.
 */

use crate::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};

/**
 * @brief: Schema mode enumeration, defining the strictness of schema constraints
 *         enforced by the database.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaMode {
    Strict,
    Hybrid,
    Schemaless,
}

/**
 * @brief: Vertex label descriptor, recording the label ID and name.
 */
#[derive(Debug, Clone)]
pub struct LabelDescriptor {
    pub id: LabelId,
    pub name: String,
}

/**
 * @brief: Edge type descriptor, recording the type ID and name.
 */
#[derive(Debug, Clone)]
pub struct EdgeTypeDescriptor {
    pub id: EdgeTypeId,
    pub name: String,
}

/**
 * @brief: Property key descriptor, recording the property key ID and name.
 */
#[derive(Debug, Clone)]
pub struct PropertyKeyDescriptor {
    pub id: PropertyKeyId,
    pub name: String,
}

/**
 * @brief: Graph metadata Schema access interface, providing a read-only view of
 *         labels, types, and property keys.
 */
pub trait GraphSchema: Send + Sync {
    /**
     * @brief: Get the Schema mode of the current database.
     * @return: A SchemaMode enum value.
     */
    fn schema_mode(&self) -> SchemaMode;

    /**
     * @brief: Get the list of all registered vertex label descriptors.
     * @return: An owned list of label descriptors.
     */
    fn vertex_labels(&self) -> Vec<LabelDescriptor>;

    /**
     * @brief: Get the list of all registered edge type descriptors.
     * @return: An owned list of edge type descriptors.
     */
    fn edge_types(&self) -> Vec<EdgeTypeDescriptor>;

    /**
     * @brief: Get the list of all registered property key descriptors.
     * @return: An owned list of property key descriptors.
     */
    fn property_keys(&self) -> Vec<PropertyKeyDescriptor>;

    /**
     * @brief: Look up a vertex label ID by name.
     * @param name: The label name.
     * @return: Some(LabelId) if found, otherwise None.
     */
    fn label_by_name(&self, name: &str) -> Option<LabelId>;

    /**
     * @brief: Look up an edge type ID by name.
     * @param name: The type name.
     * @return: Some(EdgeTypeId) if found, otherwise None.
     */
    fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId>;

    /**
     * @brief: Look up a property key ID by name.
     * @param name: The property key name.
     * @return: Some(PropertyKeyId) if found, otherwise None.
     */
    fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId>;

    /**
     * @brief: Get the label name corresponding to the specified ID.
     * @param id: The label ID.
     * @return: The name string if found, otherwise None.
     */
    fn label_name(&self, id: LabelId) -> Option<&str>;

    /**
     * @brief: Get the edge type name corresponding to the specified ID.
     * @param id: The edge type ID.
     * @return: The name string if found, otherwise None.
     */
    fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str>;

    /**
     * @brief: Get the property key name corresponding to the specified ID.
     * @param id: The property key ID.
     * @return: The name string if found, otherwise None.
     */
    fn property_key_name(&self, id: PropertyKeyId) -> Option<&str>;
}
