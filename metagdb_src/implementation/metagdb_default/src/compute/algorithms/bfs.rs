/**
 * @brief: BfsAlgorithm: breadth-first search algorithm implementation on GraphProjection.
 *
 * Computes the shortest path distances from a given source node to all reachable nodes in the projection graph.
 */

use std::collections::VecDeque;

use metagdb_core::common::ids::{LocalVid, VertexId};
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

/**
 * @brief: BFS algorithm engine.
 */
#[derive(Default)]
pub struct BfsAlgorithm;

impl AnalyticalAlgorithm for BfsAlgorithm {
    /**
     * @brief: Returns algorithm registration name "bfs".
     */
    fn name(&self) -> &str {
        "bfs"
    }

    /**
     * @brief: Execute BFS algorithm.
     * @param g: Graph projection reference.
     * @param args: Algorithm parameters, must contain "source" (source node ID).
     * @return: Returns result stream in (vertex, distance) form.
     */
    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        let source_global = args
            .get_i64("source")
            .ok_or_else(|| Error::Execute("bfs: missing parameter `source`".into()))?;
        let source = VertexId(source_global as u64);

        let source_local = g.local_vid_of(source).ok_or_else(|| {
            Error::Execute(format!(
                "bfs: source vertex {:?} is not in the current projection",
                source
            ))
        })?;

        let n = g.num_vertices();
        let mut dist: Vec<i64> = vec![-1; n];
        dist[source_local.0 as usize] = 0;

        let mut q: VecDeque<LocalVid> = VecDeque::new();
        q.push_back(source_local);

        while let Some(u) = q.pop_front() {
            let du = dist[u.0 as usize];
            for &v in g.neighbors(u) {
                if dist[v.0 as usize] == -1 {
                    dist[v.0 as usize] = du + 1;
                    q.push_back(v);
                }
            }
        }

        let mut rows: Vec<Vec<ResultValue>> = Vec::new();
        for (i, d) in dist.iter().enumerate().take(n) {
            if *d < 0 {
                continue;
            }
            let local = LocalVid(i as u32);
            let vid = g.original_vertex_id(local).ok_or_else(|| {
                Error::Execute(format!(
                    "bfs: local={i} in projection has no corresponding global VertexId"
                ))
            })?;
            rows.push(vec![ResultValue::VertexRef(vid), ResultValue::Int(*d)]);
        }

        rows.sort_by_key(|row| {
            let d = match &row[1] {
                ResultValue::Int(d) => *d,
                _ => i64::MAX,
            };
            let v = match &row[0] {
                ResultValue::VertexRef(v) => v.0,
                _ => u64::MAX,
            };
            (d, v)
        });

        Ok(ResultStream {
            schema: vec!["vertex".into(), "distance".into()],
            rows,
        })
    }
}
