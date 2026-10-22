//! Breadth-first search: minimum hop distance from a source.
//!
//! Required behaviour from the design vector: "Return minimum hop distance from
//! a source for each reachable vertex".  Only reachable vertices are emitted;
//! unreachable vertices are absent rather than reported with an infinite
//! distance, so the result is exactly the traversal frontier.

use std::collections::VecDeque;

use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

use super::{local_source, reject_unknown_arguments, stream_from_pairs};

/// Breadth-first traversal over the projection's outgoing adjacency.
#[derive(Debug, Default, Clone, Copy)]
pub struct BfsAlgorithm;

impl AnalyticalAlgorithm for BfsAlgorithm {
    fn name(&self) -> &str {
        "bfs"
    }

    fn execute(&self, graph: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        reject_unknown_arguments(args, &["source"])?;
        let source = args.get_i64("source").ok_or_else(|| {
            Error::InvalidQuery("bfs requires an integer 'source' argument".into())
        })?;
        let start = local_source(graph, source, "bfs")?;

        let count = graph.num_vertices();
        let mut distance = vec![u32::MAX; count];
        distance[start.0 as usize] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(current) = queue.pop_front() {
            let next = distance[current.0 as usize] + 1;
            for neighbor in graph.neighbors(current) {
                let index = neighbor.0 as usize;
                if index < count && distance[index] == u32::MAX {
                    distance[index] = next;
                    queue.push_back(*neighbor);
                }
            }
        }

        let pairs: Vec<(u32, ResultValue)> = distance
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != u32::MAX)
            .map(|(index, value)| (index as u32, ResultValue::Int(*value as i64)))
            .collect();
        stream_from_pairs(graph, ["node", "distance"], pairs)
    }
}
