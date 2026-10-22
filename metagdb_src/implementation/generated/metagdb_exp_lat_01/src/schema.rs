//! Dynamic, schemaless graph metadata.
//!
//! The frozen design vector selects `schema.mode = schemaless` with no declared
//! labels, edge types, or property keys. Names are therefore interned on first
//! use: the planner and executor resolve a name through the read-only
//! `GraphSchema` trait, and this implementation assigns a dense ID on demand.
//!
//! Interned names are leaked once so that `label_name`/`edge_type_name`/
//! `property_key_name` can return `&str` borrowed from `&self` (the trait
//! signature forbids returning a borrow of an internal lock guard). The number
//! of distinct names is bounded by the experiment's dictionaries.

use std::collections::HashMap;
use std::sync::RwLock;

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::l1_model::{
    EdgeTypeDescriptor, GraphSchema, LabelDescriptor, PropertyKeyDescriptor, SchemaMode,
};

#[derive(Default)]
struct Namespace {
    by_name: HashMap<String, u32>,
    by_id: HashMap<u32, &'static str>,
    ordered: Vec<(u32, &'static str)>,
    next: u32,
}

impl Namespace {
    fn intern(&mut self, name: &str) -> u32 {
        if let Some(id) = self.by_name.get(name) {
            return *id;
        }
        let id = self.next;
        self.next += 1;
        let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
        self.by_name.insert(name.to_string(), id);
        self.by_id.insert(id, leaked);
        self.ordered.push((id, leaked));
        id
    }

    /// Re-register a persisted name with its original ID.
    fn restore(&mut self, name: &str, id: u32) {
        if let Some(existing) = self.by_name.get(name).copied() {
            if existing == id {
                return;
            }
        }
        let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
        self.by_name.insert(name.to_string(), id);
        self.by_id.insert(id, leaked);
        if !self.ordered.iter().any(|(existing_id, _)| *existing_id == id) {
            self.ordered.push((id, leaked));
        }
        if id >= self.next {
            self.next = id + 1;
        }
    }

    fn get(&self, name: &str) -> Option<u32> {
        self.by_name.get(name).copied()
    }

    fn name(&self, id: u32) -> Option<&'static str> {
        self.by_id.get(&id).copied()
    }
}

/// Schemaless metadata container with on-demand name interning.
pub struct DynamicSchema {
    labels: RwLock<Namespace>,
    edge_types: RwLock<Namespace>,
    property_keys: RwLock<Namespace>,
}

impl DynamicSchema {
    /// Create an empty schemaless namespace set.
    pub fn schemaless() -> Self {
        Self {
            labels: RwLock::new(Namespace::default()),
            edge_types: RwLock::new(Namespace::default()),
            property_keys: RwLock::new(Namespace::default()),
        }
    }

    /// Resolve a label name, interning it if it is new.
    pub fn intern_label(&self, name: &str) -> LabelId {
        LabelId(self.labels.write().unwrap().intern(name))
    }

    /// Resolve an edge type name, interning it if it is new.
    pub fn intern_edge_type(&self, name: &str) -> EdgeTypeId {
        EdgeTypeId(self.edge_types.write().unwrap().intern(name))
    }

    /// Resolve a property key name, interning it if it is new.
    pub fn intern_property_key(&self, name: &str) -> PropertyKeyId {
        PropertyKeyId(self.property_keys.write().unwrap().intern(name))
    }

    /// Look up an already known label without creating a new ID.
    pub fn existing_label(&self, name: &str) -> Option<LabelId> {
        self.labels.read().unwrap().get(name).map(LabelId)
    }

    /// Look up an already known edge type without creating a new ID.
    pub fn existing_edge_type(&self, name: &str) -> Option<EdgeTypeId> {
        self.edge_types.read().unwrap().get(name).map(EdgeTypeId)
    }

    /// Look up an already known property key without creating a new ID.
    pub fn existing_property_key(&self, name: &str) -> Option<PropertyKeyId> {
        self.property_keys.read().unwrap().get(name).map(PropertyKeyId)
    }

    /// Restore persisted label metadata (reopen path).
    pub fn restore_label(&self, id: u32, name: &str) {
        self.labels.write().unwrap().restore(name, id);
    }

    /// Restore persisted edge-type metadata (reopen path).
    pub fn restore_edge_type(&self, id: u32, name: &str) {
        self.edge_types.write().unwrap().restore(name, id);
    }

    /// Restore persisted property-key metadata (reopen path).
    pub fn restore_property_key(&self, id: u32, name: &str) {
        self.property_keys.write().unwrap().restore(name, id);
    }

    /// Snapshot of the currently interned label table.
    pub fn label_table(&self) -> Vec<(u32, String)> {
        self.labels
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| (*id, name.to_string()))
            .collect()
    }

    /// Snapshot of the currently interned edge-type table.
    pub fn edge_type_table(&self) -> Vec<(u32, String)> {
        self.edge_types
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| (*id, name.to_string()))
            .collect()
    }

    /// Snapshot of the currently interned property-key table.
    pub fn property_key_table(&self) -> Vec<(u32, String)> {
        self.property_keys
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| (*id, name.to_string()))
            .collect()
    }
}

impl GraphSchema for DynamicSchema {
    fn schema_mode(&self) -> SchemaMode {
        SchemaMode::Schemaless
    }

    fn vertex_labels(&self) -> Vec<LabelDescriptor> {
        self.labels
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| LabelDescriptor {
                id: LabelId(*id),
                name: name.to_string(),
            })
            .collect()
    }

    fn edge_types(&self) -> Vec<EdgeTypeDescriptor> {
        self.edge_types
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| EdgeTypeDescriptor {
                id: EdgeTypeId(*id),
                name: name.to_string(),
            })
            .collect()
    }

    fn property_keys(&self) -> Vec<PropertyKeyDescriptor> {
        self.property_keys
            .read()
            .unwrap()
            .ordered
            .iter()
            .map(|(id, name)| PropertyKeyDescriptor {
                id: PropertyKeyId(*id),
                name: name.to_string(),
            })
            .collect()
    }

    fn label_by_name(&self, name: &str) -> Option<LabelId> {
        Some(self.intern_label(name))
    }

    fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId> {
        Some(self.intern_edge_type(name))
    }

    fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId> {
        Some(self.intern_property_key(name))
    }

    fn label_name(&self, id: LabelId) -> Option<&str> {
        self.labels.read().unwrap().name(id.0)
    }

    fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str> {
        self.edge_types.read().unwrap().name(id.0)
    }

    fn property_key_name(&self, id: PropertyKeyId) -> Option<&str> {
        self.property_keys.read().unwrap().name(id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interning_is_stable_and_dense() {
        let schema = DynamicSchema::schemaless();
        let person = schema.intern_label("Person");
        let city = schema.intern_label("City");
        assert_ne!(person, city);
        assert_eq!(schema.intern_label("Person"), person);
        assert_eq!(schema.label_by_name("Person"), Some(person));
        assert_eq!(schema.label_name(person), Some("Person"));

        let knows = schema.intern_edge_type("KNOWS");
        assert_eq!(schema.edge_type_name(knows), Some("KNOWS"));
        let name = schema.intern_property_key("name");
        assert_eq!(schema.property_key_name(name), Some("name"));
    }

    #[test]
    fn mode_is_schemaless_and_tables_round_trip() {
        let schema = DynamicSchema::schemaless();
        assert_eq!(schema.schema_mode(), SchemaMode::Schemaless);
        let id = schema.intern_label("Company");
        for descriptor in schema.vertex_labels() {
            assert_eq!(schema.label_by_name(&descriptor.name), Some(descriptor.id));
            assert_eq!(schema.label_name(descriptor.id), Some(descriptor.name.as_str()));
        }
        assert!(schema.vertex_labels().iter().any(|d| d.id == id));
    }

    #[test]
    fn restore_keeps_ids() {
        let schema = DynamicSchema::schemaless();
        schema.restore_label(7, "Person");
        assert_eq!(schema.existing_label("Person"), Some(LabelId(7)));
        assert_eq!(schema.intern_label("City"), LabelId(8));
        assert_eq!(schema.label_name(LabelId(7)), Some("Person"));
    }
}
