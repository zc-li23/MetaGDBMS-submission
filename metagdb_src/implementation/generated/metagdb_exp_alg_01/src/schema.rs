//! Graph metadata registry for the schemaless Cypher store.
//!
//! The frozen design vector declares `schema.mode = "schemaless"` with empty
//! `vertex_labels`, `edge_types`, and `property_keys`.  A schemaless store must
//! nevertheless resolve the label, edge-type, and property names it sees in
//! `CREATE`/`MATCH` patterns into stable IDs, and it must be able to convert an
//! ID back to the name it was resolved from (`labels(n)`, `type(r)`,
//! `keys(n)`).
//!
//! The framework hands the planner and the executor a `&dyn GraphSchema`
//! view only, so the registry itself has to be reachable without threading a
//! concrete handle through `OptContext`/`ExecContext`.  A single process-wide
//! registry is the smallest design that satisfies both directions of the
//! name/ID mapping: a name is registered on first use and the ID then maps back
//! to an interned name for the lifetime of the process.
//!
//! IDs are assigned densely from 1 in registration order, which keeps
//! `LabelId(1)`, `EdgeTypeId(1)`, and `PropertyKeyId(1)` valid for the fixed
//! system benchmark's fallback metadata.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::l1_model::{
    EdgeTypeDescriptor, GraphSchema, LabelDescriptor, PropertyKeyDescriptor, SchemaMode,
};

#[derive(Default)]
struct Registry {
    labels: Vec<LabelDescriptor>,
    label_by_name: HashMap<String, LabelId>,
    label_names: HashMap<LabelId, &'static str>,
    edge_types: Vec<EdgeTypeDescriptor>,
    edge_type_by_name: HashMap<String, EdgeTypeId>,
    edge_type_names: HashMap<EdgeTypeId, &'static str>,
    property_keys: Vec<PropertyKeyDescriptor>,
    property_key_by_name: HashMap<String, PropertyKeyId>,
    property_key_names: HashMap<PropertyKeyId, &'static str>,
}

impl Registry {
    fn intern(name: &str) -> &'static str {
        Box::leak(name.to_string().into_boxed_str())
    }

    fn label(&mut self, name: &str) -> LabelId {
        if let Some(id) = self.label_by_name.get(name) {
            return *id;
        }
        let id = LabelId(self.labels.len() as u32 + 1);
        let interned = Self::intern(name);
        self.labels.push(LabelDescriptor {
            id,
            name: name.to_string(),
        });
        self.label_by_name.insert(name.to_string(), id);
        self.label_names.insert(id, interned);
        id
    }

    fn edge_type(&mut self, name: &str) -> EdgeTypeId {
        if let Some(id) = self.edge_type_by_name.get(name) {
            return *id;
        }
        let id = EdgeTypeId(self.edge_types.len() as u32 + 1);
        let interned = Self::intern(name);
        self.edge_types.push(EdgeTypeDescriptor {
            id,
            name: name.to_string(),
        });
        self.edge_type_by_name.insert(name.to_string(), id);
        self.edge_type_names.insert(id, interned);
        id
    }

    fn property_key(&mut self, name: &str) -> PropertyKeyId {
        if let Some(id) = self.property_key_by_name.get(name) {
            return *id;
        }
        let id = PropertyKeyId(self.property_keys.len() as u32 + 1);
        let interned = Self::intern(name);
        self.property_keys.push(PropertyKeyDescriptor {
            id,
            name: name.to_string(),
        });
        self.property_key_by_name.insert(name.to_string(), id);
        self.property_key_names.insert(id, interned);
        id
    }
}

fn registry() -> &'static RwLock<Registry> {
    static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(Registry::default()))
}

fn read() -> std::sync::RwLockReadGuard<'static, Registry> {
    registry().read().unwrap_or_else(|error| error.into_inner())
}

fn write() -> std::sync::RwLockWriteGuard<'static, Registry> {
    registry().write().unwrap_or_else(|error| error.into_inner())
}

/// Schemaless metadata view over the process-wide name/ID registry.
#[derive(Debug, Default, Clone, Copy)]
pub struct SchemaRegistry;

impl SchemaRegistry {
    pub fn new() -> Self {
        Self
    }

    /// Resolve a vertex label name, registering it on first use.
    pub fn resolve_vertex_label(&self, name: &str) -> LabelId {
        write().label(name)
    }

    /// Resolve an edge type name, registering it on first use.
    pub fn resolve_edge_type(&self, name: &str) -> EdgeTypeId {
        write().edge_type(name)
    }

    /// Resolve a property key name, registering it on first use.
    pub fn resolve_property_key(&self, name: &str) -> PropertyKeyId {
        write().property_key(name)
    }

    /// Resolve an optional label, treating `None` as "no label restriction".
    pub fn resolve_optional_label(&self, name: Option<&str>) -> Option<LabelId> {
        name.map(|value| self.resolve_vertex_label(value))
    }
}

impl GraphSchema for SchemaRegistry {
    fn schema_mode(&self) -> SchemaMode {
        SchemaMode::Schemaless
    }

    fn vertex_labels(&self) -> Vec<LabelDescriptor> {
        read().labels.clone()
    }

    fn edge_types(&self) -> Vec<EdgeTypeDescriptor> {
        read().edge_types.clone()
    }

    fn property_keys(&self) -> Vec<PropertyKeyDescriptor> {
        read().property_keys.clone()
    }

    fn label_by_name(&self, name: &str) -> Option<LabelId> {
        read().label_by_name.get(name).copied()
    }

    fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId> {
        read().edge_type_by_name.get(name).copied()
    }

    fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId> {
        read().property_key_by_name.get(name).copied()
    }

    fn label_name(&self, id: LabelId) -> Option<&str> {
        read().label_names.get(&id).copied()
    }

    fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str> {
        read().edge_type_names.get(&id).copied()
    }

    fn property_key_name(&self, id: PropertyKeyId) -> Option<&str> {
        read().property_key_names.get(&id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schemaless_mode_is_reported() {
        assert_eq!(SchemaRegistry::new().schema_mode(), SchemaMode::Schemaless);
    }

    #[test]
    fn names_resolve_to_stable_ids_and_back() {
        let schema = SchemaRegistry::new();
        let first = schema.resolve_vertex_label("Person");
        let again = schema.resolve_vertex_label("Person");
        assert_eq!(first, again);
        assert_eq!(schema.label_name(first), Some("Person"));
        assert_eq!(schema.label_by_name("Person"), Some(first));

        let other = schema.resolve_vertex_label("City");
        assert_ne!(first, other);
        assert_eq!(schema.label_name(other), Some("City"));
    }

    #[test]
    fn edge_types_and_property_keys_round_trip() {
        let schema = SchemaRegistry::new();
        let knows = schema.resolve_edge_type("KNOWS");
        assert_eq!(schema.edge_type_name(knows), Some("KNOWS"));
        assert_eq!(schema.edge_type_by_name("KNOWS"), Some(knows));

        let age = schema.resolve_property_key("age");
        assert_eq!(schema.property_key_name(age), Some("age"));
        assert_eq!(schema.property_key_by_name("age"), Some(age));
        assert!(schema.property_key_name(PropertyKeyId(u32::MAX)).is_none());
    }

    #[test]
    fn metadata_lists_round_trip_every_registered_entry() {
        let schema = SchemaRegistry::new();
        schema.resolve_vertex_label("ProjectRegistryLabel");
        schema.resolve_edge_type("PROJECT_REGISTRY_EDGE");
        schema.resolve_property_key("project_registry_key");

        for item in schema.vertex_labels() {
            assert_eq!(schema.label_by_name(&item.name), Some(item.id));
            assert_eq!(schema.label_name(item.id), Some(item.name.as_str()));
        }
        for item in schema.edge_types() {
            assert_eq!(schema.edge_type_by_name(&item.name), Some(item.id));
            assert_eq!(schema.edge_type_name(item.id), Some(item.name.as_str()));
        }
        for item in schema.property_keys() {
            assert_eq!(schema.property_key_by_name(&item.name), Some(item.id));
            assert_eq!(schema.property_key_name(item.id), Some(item.name.as_str()));
        }
    }
}
