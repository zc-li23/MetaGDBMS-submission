/**
 * @brief: L4.a OLAP analytical operator contract, defining the standard interface for
 *         in-memory graph projections and analytical algorithms.
 */

use std::collections::HashMap;
use std::sync::Arc;

use crate::common::ids::{LocalEid, LocalVid, PropertyKeyId, VertexId};
use crate::common::property::PropertyValue;
use crate::l4_compute::algebra::AlgorithmArgs;
use crate::l4_compute::engine::ResultStream;
use crate::Result;

/**
 * @brief: In-memory graph projection interface, materialized by the concrete storage
 *         engine as a compact in-memory representation.
 */
pub trait GraphProjection: Send + Sync {
    /**
     * @brief: Get the total number of vertices in the projection.
     */
    fn num_vertices(&self) -> usize;

    /**
     * @brief: Get the total number of edges in the projection.
     */
    fn num_edges(&self) -> usize;

    /**
     * @brief: Get the neighbor list for the specified local vertex.
     * @param v: Local vertex ID.
     * @return: Slice of neighbor local IDs.
     */
    fn neighbors(&self, v: LocalVid) -> &[LocalVid];

    /**
     * @brief: Get the value of a specific property for the specified local vertex.
     * @param v: Local vertex ID.
     * @param key: Property key ID.
     * @return: A reference to the property value if it exists.
     */
    fn vertex_property(&self, v: LocalVid, key: PropertyKeyId) -> Option<&PropertyValue>;

    /**
     * @brief: Get the value of a specific property for the specified local edge.
     * @param e: Local edge ID.
     * @param key: Property key ID.
     * @return: A reference to the property value if it exists.
     */
    fn edge_property(&self, e: LocalEid, key: PropertyKeyId) -> Option<&PropertyValue>;

    /**
     * @brief: Convert a local vertex ID back to a global vertex ID.
     * @param v: Local vertex ID.
     * @return: The corresponding global VertexId.
     */
    fn original_vertex_id(&self, v: LocalVid) -> Option<VertexId>;

    /**
     * @brief: Look up a global vertex ID and convert it to a local vertex ID.
     * @param vid: Global vertex ID.
     * @return: Some(LocalVid) if the vertex exists in the projection, otherwise None.
     */
    fn local_vid_of(&self, vid: VertexId) -> Option<LocalVid>;
}

/**
 * @brief: OLAP analytical algorithm operator contract.
 */
pub trait AnalyticalAlgorithm: Send + Sync {
    /**
     * @brief: Get the registered name of the algorithm.
     */
    fn name(&self) -> &str;

    /**
     * @brief: Execute the algorithm logic on the given in-memory graph projection.
     * @param g: Reference to the in-memory graph projection.
     * @param args: Runtime algorithm arguments.
     * @return: A ResultStream containing the results on success.
     */
    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream>;
}

/**
 * @brief: Algorithm registry for managing and looking up available analytical
 *         algorithms at runtime.
 */
#[derive(Default)]
pub struct AlgorithmRegistry {
    pub algorithms: HashMap<String, Arc<dyn AnalyticalAlgorithm>>,
}
