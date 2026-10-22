//! User-requested analytical algorithms: `bfs`, `wcc` and `pagerank`.
//!
//! Each algorithm is an independent `AnalyticalAlgorithm` over the shared CSR
//! projection.  They never read the query language and never read storage
//! directly, so the same kernel serves the core entry point and the public
//! Cypher `CALL` path.

use std::collections::{HashMap, VecDeque};

use metagdb_core::common::ids::{LocalVid, VertexId};
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

/// Breadth-first traversal returning the minimum hop distance of every
/// reachable vertex from `source`.
pub struct BfsAlgorithm;

impl BfsAlgorithm {
    pub const NAME: &'static str = "bfs";
}

impl AnalyticalAlgorithm for BfsAlgorithm {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        let source = args.get_i64("source").ok_or_else(|| {
            Error::InvalidQuery("bfs requires an integer `source` argument".into())
        })?;
        let source = u64::try_from(source).map_err(|_| {
            Error::InvalidQuery(format!("bfs source {source} is not a valid vertex id"))
        })?;
        // The argument is the stable *global* vertex id, the same identity the
        // storage and the public query language use.
        let source = g.local_vid_of(VertexId(source)).ok_or_else(|| {
            Error::NotFound(format!(
                "bfs source vertex {source} is not present in the projection"
            ))
        })?;

        let mut distance = vec![u64::MAX; g.num_vertices()];
        distance[source.0 as usize] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(source);
        while let Some(current) = queue.pop_front() {
            let next = distance[current.0 as usize] + 1;
            for neighbor in g.neighbors(current) {
                let slot = &mut distance[neighbor.0 as usize];
                if *slot == u64::MAX {
                    *slot = next;
                    queue.push_back(*neighbor);
                }
            }
        }

        let mut reached: Vec<(LocalVid, u64)> = distance
            .iter()
            .enumerate()
            .filter(|(_, hops)| **hops != u64::MAX)
            .map(|(index, hops)| (LocalVid(index as u32), *hops))
            .collect();
        reached.sort_by_key(|(vertex, hops)| (*hops, vertex.0));

        let rows = reached
            .into_iter()
            .filter_map(|(vertex, hops)| {
                g.original_vertex_id(vertex)
                    .map(|global| vec![ResultValue::VertexRef(global), ResultValue::Int(hops as i64)])
            })
            .collect();

        Ok(ResultStream {
            schema: vec!["vertex".into(), "distance".into()],
            rows,
        })
    }
}

/// Weakly connected components: vertices are in one component when a path
/// exists after ignoring edge direction.
pub struct WccAlgorithm;

impl WccAlgorithm {
    pub const NAME: &'static str = "wcc";
}

impl AnalyticalAlgorithm for WccAlgorithm {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn execute(&self, g: &dyn GraphProjection, _args: &AlgorithmArgs) -> Result<ResultStream> {
        let count = g.num_vertices();
        let mut parent: Vec<u32> = (0..count as u32).collect();

        fn find(parent: &mut [u32], mut node: u32) -> u32 {
            while parent[node as usize] != node {
                let grandparent = parent[parent[node as usize] as usize];
                parent[node as usize] = grandparent;
                node = grandparent;
            }
            node
        }

        for index in 0..count as u32 {
            for neighbor in g.neighbors(LocalVid(index)) {
                let a = find(&mut parent, index);
                let b = find(&mut parent, neighbor.0);
                if a != b {
                    // Union by larger root keeps the smallest local id as the
                    // representative, which makes the component label stable.
                    let (keep, drop) = if a < b { (a, b) } else { (b, a) };
                    parent[drop as usize] = keep;
                }
            }
        }

        // Component labels are the smallest *global* vertex id in the
        // component, so a label does not depend on projection order.
        let mut component_min: HashMap<u32, u64> = HashMap::new();
        for index in 0..count as u32 {
            let root = find(&mut parent, index);
            if let Some(global) = g.original_vertex_id(LocalVid(index)) {
                let entry = component_min.entry(root).or_insert(global.0);
                if global.0 < *entry {
                    *entry = global.0;
                }
            }
        }
        let mut label = vec![u64::MAX; count];
        for index in 0..count as u32 {
            let root = find(&mut parent, index);
            if let Some(minimum) = component_min.get(&root) {
                label[index as usize] = *minimum;
            }
        }

        let rows = (0..count as u32)
            .filter_map(|index| {
                g.original_vertex_id(LocalVid(index)).map(|global| {
                    vec![
                        ResultValue::VertexRef(global),
                        ResultValue::Int(label[index as usize] as i64),
                    ]
                })
            })
            .collect();

        Ok(ResultStream {
            schema: vec!["vertex".into(), "component".into()],
            rows,
        })
    }
}

/// PageRank with configurable damping, iteration cap and convergence tolerance.
pub struct PageRankAlgorithm;

impl PageRankAlgorithm {
    pub const NAME: &'static str = "pagerank";
    /// Conventional damping factor used when the caller does not override it.
    pub const DEFAULT_DAMPING: f64 = 0.85;
    /// Iteration cap used when the caller does not override it.
    pub const DEFAULT_ITERATIONS: usize = 100;
    /// L1 convergence tolerance used when the caller does not override it.
    pub const DEFAULT_TOLERANCE: f64 = 1.0e-9;
}

impl AnalyticalAlgorithm for PageRankAlgorithm {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        let damping = match args.get("damping") {
            Some(metagdb_core::common::property::PropertyValue::Float(value)) => *value,
            Some(metagdb_core::common::property::PropertyValue::Int(value)) => *value as f64,
            Some(_) => {
                return Err(Error::InvalidQuery(
                    "pagerank damping must be numeric".into(),
                ))
            }
            None => Self::DEFAULT_DAMPING,
        };
        if !(0.0..=1.0).contains(&damping) {
            return Err(Error::InvalidQuery(format!(
                "pagerank damping {damping} must be within [0, 1]"
            )));
        }
        let iterations = args
            .get_i64("iterations")
            .map(|value| value.max(0) as usize)
            .unwrap_or(Self::DEFAULT_ITERATIONS);
        let tolerance = match args.get("tolerance") {
            Some(metagdb_core::common::property::PropertyValue::Float(value)) => *value,
            Some(metagdb_core::common::property::PropertyValue::Int(value)) => *value as f64,
            Some(_) => {
                return Err(Error::InvalidQuery(
                    "pagerank tolerance must be numeric".into(),
                ))
            }
            None => Self::DEFAULT_TOLERANCE,
        };

        let count = g.num_vertices();
        if count == 0 {
            return Ok(ResultStream {
                schema: vec!["vertex".into(), "score".into()],
                rows: Vec::new(),
            });
        }
        let initial = 1.0 / count as f64;
        let mut rank = vec![initial; count];
        let mut next = vec![0.0f64; count];

        for _ in 0..iterations {
            let dangling: f64 = (0..count)
                .filter(|index| g.neighbors(LocalVid(*index as u32)).is_empty())
                .map(|index| rank[index])
                .sum();
            let base = (1.0 - damping) / count as f64 + damping * dangling / count as f64;
            for value in next.iter_mut() {
                *value = base;
            }
            for index in 0..count {
                let neighbors = g.neighbors(LocalVid(index as u32));
                if neighbors.is_empty() {
                    continue;
                }
                let share = damping * rank[index] / neighbors.len() as f64;
                for neighbor in neighbors {
                    next[neighbor.0 as usize] += share;
                }
            }
            let delta: f64 = rank
                .iter()
                .zip(next.iter())
                .map(|(old, new)| (old - new).abs())
                .sum();
            std::mem::swap(&mut rank, &mut next);
            if delta < tolerance {
                break;
            }
        }

        let rows = (0..count as u32)
            .filter_map(|index| {
                g.original_vertex_id(LocalVid(index)).map(|global| {
                    vec![ResultValue::VertexRef(global), ResultValue::Float(rank[index as usize])]
                })
            })
            .collect();

        Ok(ResultStream {
            schema: vec!["vertex".into(), "score".into()],
            rows,
        })
    }
}

/// Declared scalar parameter order for each registered algorithm.
///
/// A procedure's parameters are part of its signature, so a positional call
/// such as `CALL bfs(1)` binds position 0 to `source`.
pub fn parameter_name(algorithm: &str, position: usize) -> Option<&'static str> {
    let order: &[&str] = match algorithm.to_ascii_lowercase().as_str() {
        "bfs" => &["source"],
        "wcc" => &[],
        "pagerank" => &["damping", "iterations", "tolerance"],
        _ => return None,
    };
    order.get(position).copied()
}

/// All algorithms required by the design vector, in declaration order.
pub fn required_algorithms() -> Vec<std::sync::Arc<dyn AnalyticalAlgorithm>> {
    vec![
        std::sync::Arc::new(BfsAlgorithm),
        std::sync::Arc::new(WccAlgorithm),
        std::sync::Arc::new(PageRankAlgorithm),
    ]
}
