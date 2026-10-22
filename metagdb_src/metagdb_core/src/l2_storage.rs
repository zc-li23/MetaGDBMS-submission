/**
 * @brief: L2 storage engine layer contract, defining the read, scan, and access
 *         model for graph data.
 */

use crate::common::ids::{EdgeId, EdgeTypeId, LabelId, VertexId};
use crate::common::property::PropertyMap;
use crate::l3_txn::TxnContext;
use crate::Result;

/**
 * @brief: Vertex record, containing a unique ID, a set of labels, and a property map.
 */
#[derive(Debug, Clone)]
pub struct VertexRecord {
    pub id: VertexId,
    pub labels: Vec<LabelId>,
    pub properties: PropertyMap,
}

/**
 * @brief: Edge record, containing a unique ID, endpoint IDs, type, and a property map.
 */
#[derive(Debug, Clone)]
pub struct EdgeRecord {
    pub id: EdgeId,
    pub src: VertexId,
    pub dst: VertexId,
    pub etype: EdgeTypeId,
    pub properties: PropertyMap,
}

/**
 * @brief: Vertex scan filter, used to constrain the label range during scanning.
 */
#[derive(Debug, Clone, Default)]
pub struct VertexFilter {
    pub labels: Option<Vec<LabelId>>,
}

/**
 * @brief: Edge scan filter, used to constrain the edge type range during scanning.
 */
#[derive(Debug, Clone, Default)]
pub struct EdgeFilter {
    pub types: Option<Vec<EdgeTypeId>>,
}

pub type VertexIter = Box<dyn Iterator<Item = VertexRecord> + Send>;
pub type EdgeIter = Box<dyn Iterator<Item = EdgeRecord> + Send>;

/**
 * @brief: Property index handle, used to identify a specific index structure in
 *         the storage engine.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexHandle(pub u32);

pub type IndexKey = crate::common::property::PropertyValue;

/**
 * @brief: L2.b storage management access interface, providing the core access
 *         primitives for graph data.
 */
pub trait StorageEngine: Send + Sync {
    /**
     * @brief: Get a single vertex record by vertex ID.
     * @param vid: The target vertex ID.
     * @param ctx: Transaction context (controls visibility).
     * @return: Some(VertexRecord) if it exists and is visible, otherwise None.
     */
    fn get_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>>;

    /**
     * @brief: Scan all visible vertices in the database, with optional label filtering.
     * @param filter: Scan filter conditions.
     * @param ctx: Transaction context.
     * @return: A vertex record iterator.
     */
    fn scan_vertices(&self, filter: VertexFilter, ctx: &TxnContext) -> Result<VertexIter>;

    /**
     * @brief: Get a single edge record by edge ID.
     * @param eid: The target edge ID.
     * @param ctx: Transaction context.
     * @return: Some(EdgeRecord) if it exists and is visible, otherwise None.
     */
    fn get_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>>;

    /**
     * @brief: Find all visible outgoing edges from the specified vertex.
     * @param vid: The source vertex ID.
     * @param filter: Edge type filter conditions.
     * @param ctx: Transaction context.
     * @return: An edge record iterator.
     */
    fn out_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter>;

    /**
     * @brief: Find all visible incoming edges to the specified vertex.
     * @param vid: The target vertex ID.
     * @param filter: Edge type filter conditions.
     * @param ctx: Transaction context.
     * @return: An edge record iterator.
     */
    fn in_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter>;

    /**
     * @brief: Use a specific index to look up vertices matching the given key.
     * @param idx: The index handle.
     * @param key: The index key value.
     * @param ctx: Transaction context.
     * @return: An iterator of matching vertex records.
     */
    fn index_lookup(
        &self,
        idx: IndexHandle,
        key: &IndexKey,
        ctx: &TxnContext,
    ) -> Result<VertexIter>;

    /**
     * @brief: Insert a new vertex under transaction protection.
     * @param record: The vertex record to insert.
     * @param ctx: Transaction context.
     * @return: The assigned vertex ID on success.
     */
    fn insert_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<VertexId>;

    /**
     * @brief: Insert a new edge under transaction protection.
     * @param record: The edge record to insert.
     * @param ctx: Transaction context.
     * @return: The assigned edge ID on success.
     */
    fn insert_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<EdgeId>;

    /**
     * @brief: Update an existing vertex under transaction protection.
     * @param record: The new vertex record (id must refer to an existing vertex).
     * @param ctx: Transaction context.
     */
    fn update_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<()>;

    /**
     * @brief: Delete a vertex under transaction protection.
     * @param vid: The vertex ID to delete.
     * @param ctx: Transaction context.
     */
    fn delete_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<()>;

    /**
     * @brief: Update an existing edge under transaction protection (src/dst/etype
     *         may not be modified).
     * @param record: The new edge record (id must refer to an existing edge, and
     *                src/dst/etype must match the original record).
     * @param ctx: Transaction context.
     */
    fn update_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<()>;

    /**
     * @brief: Delete an edge under transaction protection.
     * @param eid: The edge ID to delete.
     * @param ctx: Transaction context.
     */
    fn delete_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<()>;
}
