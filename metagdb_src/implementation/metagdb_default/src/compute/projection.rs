/**
 * @brief: CsrProjection: graph data projection implementation based on CSR (Compressed Sparse Row) format.
 *
 * Converts storage layer data into a compact memory layout to accelerate neighbor traversal performance for OLAP algorithms.
 */

use std::collections::HashMap;

use metagdb_core::common::ids::{LocalEid, LocalVid, PropertyKeyId, VertexId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l2_storage::{EdgeFilter, StorageEngine, VertexFilter};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::l4_compute::algebra::{ProjectionDirection, ProjectionSpec};
use metagdb_core::l4_compute::olap::GraphProjection;
use metagdb_core::Result;

/**
 * @brief: In-memory CSR graph projection container.
 */
pub struct CsrProjection {
    local_to_global: Vec<VertexId>,
    global_to_local: HashMap<VertexId, LocalVid>,
    row_ptr: Vec<usize>,
    col_idx: Vec<LocalVid>,
    vertex_props: Vec<HashMap<PropertyKeyId, PropertyValue>>,
}

impl CsrProjection {
    /**
     * @brief: Build CSR projection from underlying storage.
     * @param storage: Storage engine reference.
     * @param ctx: Current transaction context.
     * @param spec: Projection specification (contains which vertices/edges to select and direction).
     */
    pub fn build(
        storage: &dyn StorageEngine,
        ctx: &TxnContext,
        spec: &ProjectionSpec,
    ) -> Result<Self> {
        let vfilter = VertexFilter {
            labels: spec.vertex_labels.clone(),
        };
        let vertices: Vec<_> = storage.scan_vertices(vfilter, ctx)?.collect();

        let mut local_to_global = Vec::with_capacity(vertices.len());
        let mut global_to_local = HashMap::with_capacity(vertices.len());
        let mut vertex_props = Vec::with_capacity(vertices.len());

        for (i, v) in vertices.iter().enumerate() {
            local_to_global.push(v.id);
            global_to_local.insert(v.id, LocalVid(i as u32));
            let props: HashMap<PropertyKeyId, PropertyValue> = match &spec.vertex_properties {
                None => v.properties.iter().map(|(k, val)| (*k, val.clone())).collect(),
                Some(keys) => keys
                    .iter()
                    .filter_map(|k| v.properties.get(k).map(|val| (*k, val.clone())))
                    .collect(),
            };
            vertex_props.push(props);
        }

        let efilter = EdgeFilter {
            types: spec.edge_types.clone(),
        };

        let n = local_to_global.len();
        let mut row_ptr = vec![0usize; n + 1];
        let mut adjacency: Vec<Vec<LocalVid>> = (0..n).map(|_| Vec::new()).collect();

        for (i, vrec) in vertices.iter().enumerate() {
            let collect_to_local = |edge_endpoints: Vec<VertexId>| -> Vec<LocalVid> {
                edge_endpoints
                    .into_iter()
                    .filter_map(|t| global_to_local.get(&t).copied())
                    .collect()
            };

            match spec.direction {
                ProjectionDirection::Outgoing => {
                    let nbrs: Vec<VertexId> = storage
                        .out_edges(vrec.id, efilter.clone(), ctx)?
                        .map(|e| e.dst)
                        .collect();
                    adjacency[i] = collect_to_local(nbrs);
                }
                ProjectionDirection::Incoming => {
                    let nbrs: Vec<VertexId> = storage
                        .in_edges(vrec.id, efilter.clone(), ctx)?
                        .map(|e| e.src)
                        .collect();
                    adjacency[i] = collect_to_local(nbrs);
                }
                ProjectionDirection::Both => {
                    let mut nbrs: Vec<VertexId> = storage
                        .out_edges(vrec.id, efilter.clone(), ctx)?
                        .map(|e| e.dst)
                        .collect();
                    nbrs.extend(
                        storage
                            .in_edges(vrec.id, efilter.clone(), ctx)?
                            .map(|e| e.src),
                    );
                    adjacency[i] = collect_to_local(nbrs);
                }
            }
        }

        let mut col_idx: Vec<LocalVid> = Vec::new();
        for (i, nbrs) in adjacency.iter().enumerate() {
            row_ptr[i] = col_idx.len();
            col_idx.extend_from_slice(nbrs);
        }
        row_ptr[n] = col_idx.len();

        Ok(Self {
            local_to_global,
            global_to_local,
            row_ptr,
            col_idx,
            vertex_props,
        })
    }
}

impl GraphProjection for CsrProjection {
    /**
     * @brief: Get the number of vertices in the projection.
     */
    fn num_vertices(&self) -> usize {
        self.local_to_global.len()
    }

    /**
     * @brief: Get the total number of edges in the projection.
     */
    fn num_edges(&self) -> usize {
        self.col_idx.len()
    }

    /**
     * @brief: Get the neighbor list (local ID sequence) for a given local vertex ID.
     * @param v: Local vertex ID.
     */
    fn neighbors(&self, v: LocalVid) -> &[LocalVid] {
        let i = v.0 as usize;
        if i >= self.num_vertices() {
            return &[];
        }
        let start = self.row_ptr[i];
        let end = self.row_ptr[i + 1];
        &self.col_idx[start..end]
    }

    /**
     * @brief: Get the property value of a given vertex.
     * @param v: Local vertex ID.
     * @param key: Property key ID.
     */
    fn vertex_property(&self, v: LocalVid, key: PropertyKeyId) -> Option<&PropertyValue> {
        self.vertex_props.get(v.0 as usize)?.get(&key)
    }

    /**
     * @brief: Get the property value of a given edge. CsrProjection currently does not support edge property storage, always returns None.
     */
    fn edge_property(&self, _e: LocalEid, _key: PropertyKeyId) -> Option<&PropertyValue> {
        None
    }

    /**
     * @brief: Map a local ID back to the system's global vertex ID.
     */
    fn original_vertex_id(&self, v: LocalVid) -> Option<VertexId> {
        self.local_to_global.get(v.0 as usize).copied()
    }

    /**
     * @brief: Map a global vertex ID to a local ID. Returns None if the vertex is not included in this projection.
     */
    fn local_vid_of(&self, vid: VertexId) -> Option<LocalVid> {
        self.global_to_local.get(&vid).copied()
    }
}
