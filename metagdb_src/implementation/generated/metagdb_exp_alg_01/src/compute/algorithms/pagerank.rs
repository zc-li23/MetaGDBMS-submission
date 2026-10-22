//! PageRank with a documented damping factor, stopping rule, and tolerance.
//!
//! Required behaviour from the design vector: "Compute PageRank with documented
//! damping, stopping rule, and floating-point tolerance".
//!
//! The documented definition this implementation follows is
//!
//! ```text
//! rank[v] = (1 - d) / N + d * ( sum over u -> v of rank[u] / outdeg(u)
//!                               + dangling / N )
//! ```
//!
//! where `d` is the damping factor (`damping`, default `0.85`), `N` is the
//! number of projected vertices, `outdeg(u)` is the projected outgoing degree,
//! and `dangling` is the total rank mass sitting on vertices with no outgoing
//! edge — that mass is redistributed uniformly instead of vanishing, so the
//! ranks keep summing to 1.
//!
//! **Stopping rule.** The iteration starts from the uniform vector `1/N` and
//! stops after the first pass whose L1 change `sum |rank_new - rank_old|` is
//! strictly below `tolerance` (default `1e-9`), or after `max_iterations`
//! passes (default `100`), whichever comes first.  Ranks are therefore accurate
//! to the requested tolerance for graphs whose spectral gap allows convergence
//! inside the iteration cap; when the cap is hit the result is the value after
//! the last pass and is not silently reported as converged.
//!
//! Propagation runs along `GraphProjection::neighbors`, i.e. the projection's
//! outgoing adjacency, which is the direction mass flows in the definition above.

use metagdb_core::common::ids::LocalVid;
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

use super::{optional_number, reject_unknown_arguments, stream_from_pairs};

const DEFAULT_DAMPING: f64 = 0.85;
const DEFAULT_TOLERANCE: f64 = 1e-9;
const DEFAULT_MAX_ITERATIONS: i64 = 100;

/// Power-iteration PageRank over the projection's outgoing adjacency.
#[derive(Debug, Default, Clone, Copy)]
pub struct PageRankAlgorithm;

impl AnalyticalAlgorithm for PageRankAlgorithm {
    fn name(&self) -> &str {
        "pagerank"
    }

    fn execute(&self, graph: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        reject_unknown_arguments(args, &["damping", "tolerance", "max_iterations"])?;
        let damping = optional_number(args, "damping")?.unwrap_or(DEFAULT_DAMPING);
        if !(0.0..1.0).contains(&damping) {
            return Err(Error::InvalidQuery(format!(
                "pagerank damping must be in [0, 1), got {damping}"
            )));
        }
        let tolerance = optional_number(args, "tolerance")?.unwrap_or(DEFAULT_TOLERANCE);
        if tolerance <= 0.0 {
            return Err(Error::InvalidQuery(format!(
                "pagerank tolerance must be positive, got {tolerance}"
            )));
        }
        let max_iterations = match args.get("max_iterations") {
            None => DEFAULT_MAX_ITERATIONS,
            Some(value) => value.as_i64().ok_or_else(|| {
                Error::InvalidQuery("pagerank max_iterations must be an integer".into())
            })?,
        };
        if max_iterations < 1 {
            return Err(Error::InvalidQuery(format!(
                "pagerank max_iterations must be at least 1, got {max_iterations}"
            )));
        }

        let count = graph.num_vertices();
        if count == 0 {
            return Ok(ResultStream {
                schema: vec!["node".to_string(), "rank".to_string()],
                rows: Vec::new(),
            });
        }

        let uniform = 1.0 / count as f64;
        let mut rank = vec![uniform; count];
        let mut scratch = vec![0.0f64; count];

        for _ in 0..max_iterations {
            let base = (1.0 - damping) / count as f64;
            scratch.iter_mut().for_each(|value| *value = base);
            let mut dangling = 0.0;
            for vertex in 0..count {
                let neighbors = graph.neighbors(LocalVid(vertex as u32));
                if neighbors.is_empty() {
                    dangling += rank[vertex];
                    continue;
                }
                let share = rank[vertex] / neighbors.len() as f64;
                for neighbor in neighbors {
                    let index = neighbor.0 as usize;
                    if index < count {
                        scratch[index] += damping * share;
                    }
                }
            }
            let distributed = damping * dangling / count as f64;
            scratch.iter_mut().for_each(|value| *value += distributed);

            let delta: f64 = rank
                .iter()
                .zip(scratch.iter())
                .map(|(old, new)| (old - new).abs())
                .sum();
            std::mem::swap(&mut rank, &mut scratch);
            if delta < tolerance {
                break;
            }
        }

        let pairs: Vec<(u32, ResultValue)> = rank
            .iter()
            .enumerate()
            .map(|(index, value)| (index as u32, ResultValue::Float(*value)))
            .collect();
        stream_from_pairs(graph, ["node", "rank"], pairs)
    }
}
