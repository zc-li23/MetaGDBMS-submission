/**
 * @brief: Identifier (ID) type definitions, including globally persistent IDs and
 *         runtime local IDs.
 */

/**
 * @brief: Global vertex ID, unique over the entire database lifecycle.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VertexId(pub u64);

/**
 * @brief: Global edge ID.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(pub u64);

/**
 * @brief: Global ID for a vertex label.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LabelId(pub u32);

/**
 * @brief: Global ID for an edge type.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeTypeId(pub u32);

/**
 * @brief: Global ID for a property key.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyKeyId(pub u32);

/**
 * @brief: Projection-local vertex ID, commonly used by OLAP operators for local
 *         addressing within compact in-memory graph projections.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalVid(pub u32);

/**
 * @brief: Projection-local edge ID.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalEid(pub u32);
