//! Schemaless graph metadata registry for `metagdb_exp_thr_02`.
//!
//! The frozen design vector selects `schema.mode = "schemaless"`.  A schemaless
//! system accepts any label, edge type and property key, so this registry names
//! them lazily: the first time a name is seen it receives a stable dense id.
//! The `GraphSchema` trait is read-only, therefore the registry keeps its
//! namespace behind interior mutability and interns on lookup.  That keeps the
//! planner/optimizer/executor unit structs stateless, as the assembly contract
//! requires, while still resolving labels through `GraphSchema`.
//!
//! Names are stored as leaked `&'static str` because `GraphSchema::label_name`
//! returns a borrow tied to `&self` while the backing store is a lock guard.  The
//! set of distinct metadata names is small and bounded by the schema vocabulary,
//! so the leak is a deliberate, bounded trade-off recorded in `decisions.md`.

use std::collections::HashMap;
use std::sync::RwLock;

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::l1_model::{
    EdgeTypeDescriptor, GraphSchema, LabelDescriptor, PropertyKeyDescriptor, SchemaMode,
};

/// Append-only namespace mapping dense ids to names and back.
#[derive(Default)]
struct Namespace {
    by_name: HashMap<String, u32>,
    /// `names[id]` is the name registered for that id; `None` means unused.
    names: Vec<Option<&'static str>>,
    next: u32,
}

impl Namespace {
    fn insert_at(&mut self, id: u32, name: &str) {
        if self.names.len() <= id as usize {
            self.names.resize(id as usize + 1, None);
        }
        if self.names[id as usize].is_none() {
            let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
            self.names[id as usize] = Some(leaked);
            self.by_name.insert(name.to_string(), id);
        }
        if id >= self.next {
            self.next = id + 1;
        }
    }

    fn intern(&mut self, name: &str) -> u32 {
        if let Some(id) = self.by_name.get(name) {
            return *id;
        }
        let id = self.next;
        self.next += 1;
        self.insert_at(id, name);
        id
    }

    fn lookup(&self, name: &str) -> Option<u32> {
        self.by_name.get(name).copied()
    }

    fn name_of(&self, id: u32) -> Option<&'static str> {
        self.names.get(id as usize).copied().flatten()
    }

    fn snapshot(&self) -> Vec<(u32, String)> {
        self.names
            .iter()
            .enumerate()
            .filter_map(|(index, name)| name.map(|name| (index as u32, name.to_string())))
            .collect()
    }
}

/// Schemaless metadata registry with lazy interning.
pub struct MetaSchema {
    labels: RwLock<Namespace>,
    edge_types: RwLock<Namespace>,
    property_keys: RwLock<Namespace>,
}

impl MetaSchema {
    fn empty() -> Self {
        Self {
            labels: RwLock::new(Namespace::default()),
            edge_types: RwLock::new(Namespace::default()),
            property_keys: RwLock::new(Namespace::default()),
        }
    }

    /// Metadata vocabulary used by the shared Cypher benchmark fixture and by
    /// interactive sessions.  Pre-registering keeps ids stable across reopen and
    /// makes reverse lookups (`labels()`, `type()`) available immediately.
    pub fn demo_schema() -> Self {
        let schema = Self::empty();
        for name in ["Person", "City", "Company"] {
            schema.intern_label(name);
        }
        for name in ["KNOWS", "LIVES_IN", "WORKS_AT"] {
            schema.intern_edge_type(name);
        }
        for name in [
            "name",
            "age",
            "joined",
            "population",
            "founded",
            "since",
            "weight",
        ] {
            schema.intern_property_key(name);
        }
        schema
    }

    /// Metadata vocabulary for `.graph` bulk import.  The loader stores every
    /// vertex under `LabelId(0)` and every edge under `EdgeTypeId(0)`, with the
    /// numeric label/community fields under `PropertyKeyId(1)` and
    /// `PropertyKeyId(2)`.
    pub fn generic_schema() -> Self {
        let schema = Self::empty();
        schema.labels.write().unwrap().insert_at(0, "Node");
        schema.edge_types.write().unwrap().insert_at(0, "EDGE");
        schema.property_keys.write().unwrap().insert_at(1, "label");
        schema.property_keys.write().unwrap().insert_at(2, "community");
        schema
    }

    /// Intern a vertex label name, returning its stable id.
    pub fn intern_label(&self, name: &str) -> LabelId {
        LabelId(self.labels.write().unwrap().intern(name))
    }

    /// Intern an edge type name, returning its stable id.
    pub fn intern_edge_type(&self, name: &str) -> EdgeTypeId {
        EdgeTypeId(self.edge_types.write().unwrap().intern(name))
    }

    /// Intern a property key name, returning its stable id.
    pub fn intern_property_key(&self, name: &str) -> PropertyKeyId {
        PropertyKeyId(self.property_keys.write().unwrap().intern(name))
    }

    /// Resolve a registered label id back to its ordinal position, used by the
    /// benchmark-visible `id()` semantics only for vertices.
    pub fn label_count(&self) -> usize {
        self.labels.read().unwrap().snapshot().len()
    }
}

impl GraphSchema for MetaSchema {
    fn schema_mode(&self) -> SchemaMode {
        SchemaMode::Schemaless
    }

    fn vertex_labels(&self) -> Vec<LabelDescriptor> {
        self.labels
            .read()
            .unwrap()
            .snapshot()
            .into_iter()
            .map(|(id, name)| LabelDescriptor {
                id: LabelId(id),
                name,
            })
            .collect()
    }

    fn edge_types(&self) -> Vec<EdgeTypeDescriptor> {
        self.edge_types
            .read()
            .unwrap()
            .snapshot()
            .into_iter()
            .map(|(id, name)| EdgeTypeDescriptor {
                id: EdgeTypeId(id),
                name,
            })
            .collect()
    }

    fn property_keys(&self) -> Vec<PropertyKeyDescriptor> {
        self.property_keys
            .read()
            .unwrap()
            .snapshot()
            .into_iter()
            .map(|(id, name)| PropertyKeyDescriptor {
                id: PropertyKeyId(id),
                name,
            })
            .collect()
    }

    /// In schemaless mode an unknown name is registered on first use, so this
    /// returns `Some` for every well-formed name rather than rejecting it.
    fn label_by_name(&self, name: &str) -> Option<LabelId> {
        if let Some(id) = self.labels.read().unwrap().lookup(name) {
            return Some(LabelId(id));
        }
        Some(self.intern_label(name))
    }

    fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId> {
        if let Some(id) = self.edge_types.read().unwrap().lookup(name) {
            return Some(EdgeTypeId(id));
        }
        Some(self.intern_edge_type(name))
    }

    fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId> {
        if let Some(id) = self.property_keys.read().unwrap().lookup(name) {
            return Some(PropertyKeyId(id));
        }
        Some(self.intern_property_key(name))
    }

    fn label_name(&self, id: LabelId) -> Option<&str> {
        self.labels.read().unwrap().name_of(id.0)
    }

    fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str> {
        self.edge_types.read().unwrap().name_of(id.0)
    }

    fn property_key_name(&self, id: PropertyKeyId) -> Option<&str> {
        self.property_keys.read().unwrap().name_of(id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interned_names_round_trip_and_are_stable() {
        let schema = MetaSchema::demo_schema();
        let a = schema.label_by_name("Person").unwrap();
        let b = schema.intern_label("Person");
        assert_eq!(a, b);
        assert_eq!(schema.label_name(a), Some("Person"));

        let city = schema.label_by_name("City").unwrap();
        assert_ne!(a, city);
        assert_eq!(schema.label_name(city), Some("City"));
    }

    #[test]
    fn schemaless_accepts_new_metadata() {
        let schema = MetaSchema::demo_schema();
        let label = schema.label_by_name("Widget").unwrap();
        assert_eq!(schema.label_name(label), Some("Widget"));
        let etype = schema.edge_type_by_name("FEEDS").unwrap();
        assert_eq!(schema.edge_type_name(etype), Some("FEEDS"));
        let key = schema.property_key_by_name("mass").unwrap();
        assert_eq!(schema.property_key_name(key), Some("mass"));
    }

    #[test]
    fn generic_schema_uses_zero_based_element_metadata() {
        let schema = MetaSchema::generic_schema();
        assert_eq!(schema.label_name(LabelId(0)), Some("Node"));
        assert_eq!(schema.edge_type_name(EdgeTypeId(0)), Some("EDGE"));
        assert_eq!(schema.property_key_name(PropertyKeyId(1)), Some("label"));
        assert_eq!(schema.property_key_name(PropertyKeyId(2)), Some("community"));
    }

    #[test]
    fn declared_metadata_round_trips_through_the_trait() {
        let schema = MetaSchema::demo_schema();
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

    #[test]
    fn schema_mode_is_schemaless() {
        assert_eq!(MetaSchema::demo_schema().schema_mode(), SchemaMode::Schemaless);
    }
}
