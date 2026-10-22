//! Weakly connected components: one component label per vertex.
//!
//! Required behaviour from the design vector: "Assign the same component to
//! vertices connected when edge direction is ignored".
//!
//! The projection lists every edge in at least one adjacency list, so taking
//! the undirected closure of the adjacency it exposes recovers exactly the
//! undirected edge set — the answer does not depend on the projection's
//! direction.  A component is labelled by the smallest *global* vertex ID it
//! contains, which is stable across projections and easy to check by hand.

use metagdb_core::common::ids::{LocalVid, VertexId};
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

use super::{reject_unknown_arguments, stream_from_pairs};

/// Union-find over a symmetrized adjacency list.
#[derive(Debug, Default, Clone, Copy)]
pub struct WccAlgorithm;

impl AnalyticalAlgorithm for WccAlgorithm {
    fn name(&self) -> &str {
        "wcc"
    }

    fn execute(&self, graph: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        reject_unknown_arguments(args, &[])?;
        let count = graph.num_vertices();
        if count == 0 {
            return Ok(ResultStream {
                schema: vec!["node".to_string(), "component".to_string()],
                rows: Vec::new(),
            });
        }

        let mut parent: Vec<u32> = (0..count as u32).collect();
        for vertex in 0..count as u32 {
            for neighbor in graph.neighbors(LocalVid(vertex)) {
                if (neighbor.0 as usize) < count {
                    union(&mut parent, vertex, neighbor.0);
                }
            }
        }

        // Label every root with the smallest global vertex ID it contains.
        let mut label: Vec<Option<VertexId>> = vec![None; count];
        for vertex in 0..count as u32 {
            let global = graph.original_vertex_id(LocalVid(vertex)).ok_or_else(|| {
                Error::Execute(format!("projection lost local vertex {vertex}"))
            })?;
            let root = find(&mut parent, vertex) as usize;
            match label[root] {
                Some(current) if current.0 <= global.0 => {}
                _ => label[root] = Some(global),
            }
        }

        let mut pairs = Vec::with_capacity(count);
        for vertex in 0..count as u32 {
            let root = find(&mut parent, vertex) as usize;
            let component = label[root]
                .ok_or_else(|| Error::Execute(format!("component {root} has no label")))?;
            pairs.push((vertex, ResultValue::Int(component.0 as i64)));
        }
        stream_from_pairs(graph, ["node", "component"], pairs)
    }
}

fn find(parent: &mut [u32], mut vertex: u32) -> u32 {
    let mut root = vertex;
    while parent[root as usize] != root {
        root = parent[root as usize];
    }
    // Path compression keeps repeated lookups effectively constant time.
    while parent[vertex as usize] != root {
        let next = parent[vertex as usize];
        parent[vertex as usize] = root;
        vertex = next;
    }
    root
}

fn union(parent: &mut [u32], left: u32, right: u32) {
    let left_root = find(parent, left);
    let right_root = find(parent, right);
    if left_root == right_root {
        return;
    }
    // Attach the larger root under the smaller one so component scans are cheap.
    let (low, high) = if left_root < right_root {
        (left_root, right_root)
    } else {
        (right_root, left_root)
    };
    parent[high as usize] = low;
}
