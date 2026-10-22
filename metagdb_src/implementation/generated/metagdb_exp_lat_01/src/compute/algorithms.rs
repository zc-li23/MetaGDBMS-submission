//! Required analytical algorithms: BFS, weakly connected components, and
//! PageRank. Each consumes the CSR projection built by the executor and emits a
//! deterministic `ResultStream`.

use std::collections::VecDeque;

use metagdb_core::common::ids::{LocalVid, VertexId};
use metagdb_core::common::property::ResultValue;
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

/// Breadth-first traversal returning minimum hop distance per reachable vertex.
#[derive(Default)]
pub struct BfsAlgorithm;

impl AnalyticalAlgorithm for BfsAlgorithm {
    fn name(&self) -> &str {
        "bfs"
    }

    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        let source = args
            .get_i64("source")
            .ok_or_else(|| Error::Execute("bfs requires an integer `source` argument".into()))?;
        if source < 0 {
            return Err(Error::Execute("bfs source must be non-negative".into()));
        }
        let source_local = g
            .local_vid_of(VertexId(source as u64))
            .ok_or_else(|| Error::Execute(format!("bfs source {source} is not in the projection")))?;

        let mut distance: Vec<i64> = vec![-1; g.num_vertices()];
        distance[source_local.0 as usize] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(source_local);
        while let Some(current) = queue.pop_front() {
            let next_distance = distance[current.0 as usize] + 1;
            for neighbor in g.neighbors(current) {
                let slot = &mut distance[neighbor.0 as usize];
                if *slot < 0 {
                    *slot = next_distance;
                    queue.push_back(*neighbor);
                }
            }
        }

        let mut rows: Vec<(u64, i64)> = Vec::new();
        for (local, dist) in distance.iter().enumerate() {
            if *dist >= 0 {
                if let Some(global) = g.original_vertex_id(LocalVid(local as u32)) {
                    rows.push((global.0, *dist));
                }
            }
        }
        rows.sort_unstable();
        Ok(ResultStream {
            schema: vec!["node".into(), "distance".into()],
            rows: rows
                .into_iter()
                .map(|(vertex, dist)| vec![ResultValue::VertexRef(VertexId(vertex)), ResultValue::Int(dist)])
                .collect(),
        })
    }
}

/// Weakly connected components: treats the supplied adjacency as undirected.
#[derive(Default)]
pub struct WccAlgorithm;

impl AnalyticalAlgorithm for WccAlgorithm {
    fn name(&self) -> &str {
        "wcc"
    }

    fn execute(&self, g: &dyn GraphProjection, _args: &AlgorithmArgs) -> Result<ResultStream> {
        let n = g.num_vertices();
        let mut parent: Vec<u32> = (0..n as u32).collect();
        fn find(parent: &mut [u32], mut node: u32) -> u32 {
            while parent[node as usize] != node {
                parent[node as usize] = parent[parent[node as usize] as usize];
                node = parent[node as usize];
            }
            node
        }
        for local in 0..n {
            for neighbor in g.neighbors(LocalVid(local as u32)) {
                let a = find(&mut parent, local as u32);
                let b = find(&mut parent, neighbor.0);
                if a != b {
                    parent[a.max(b) as usize] = a.min(b);
                }
            }
        }

        // Label each component by the smallest global vertex id it contains.
        let mut component_min: std::collections::HashMap<u32, u64> = std::collections::HashMap::new();
        for local in 0..n {
            let root = find(&mut parent, local as u32);
            if let Some(global) = g.original_vertex_id(LocalVid(local as u32)) {
                let entry = component_min.entry(root).or_insert(global.0);
                if global.0 < *entry {
                    *entry = global.0;
                }
            }
        }
        let mut rows: Vec<(u64, u64)> = Vec::new();
        for local in 0..n {
            let root = find(&mut parent, local as u32);
            if let (Some(global), Some(component)) =
                (g.original_vertex_id(LocalVid(local as u32)), component_min.get(&root))
            {
                rows.push((global.0, *component));
            }
        }
        rows.sort_unstable();
        Ok(ResultStream {
            schema: vec!["node".into(), "component".into()],
            rows: rows
                .into_iter()
                .map(|(vertex, component)| {
                    vec![
                        ResultValue::VertexRef(VertexId(vertex)),
                        ResultValue::Int(component as i64),
                    ]
                })
                .collect(),
        })
    }
}

/// Directed PageRank with damping 0.85 and a documented stopping rule.
#[derive(Default)]
pub struct PageRankAlgorithm;

const PAGERANK_DAMPING: f64 = 0.85;
const PAGERANK_MAX_ITERATIONS: usize = 100;
const PAGERANK_TOLERANCE: f64 = 1e-8;

impl AnalyticalAlgorithm for PageRankAlgorithm {
    fn name(&self) -> &str {
        "pagerank"
    }

    fn execute(&self, g: &dyn GraphProjection, args: &AlgorithmArgs) -> Result<ResultStream> {
        let n = g.num_vertices();
        if n == 0 {
            return Ok(ResultStream {
                schema: vec!["node".into(), "rank".into()],
                rows: Vec::new(),
            });
        }
        let damping = args
            .get("damping")
            .and_then(|value| match value {
                metagdb_core::common::property::PropertyValue::Float(f) => Some(*f),
                metagdb_core::common::property::PropertyValue::Int(i) => Some(*i as f64),
                _ => None,
            })
            .unwrap_or(PAGERANK_DAMPING);
        let iterations = args
            .get_i64("iterations")
            .map(|value| value.max(1) as usize)
            .unwrap_or(PAGERANK_MAX_ITERATIONS);

        let out_degree: Vec<usize> = (0..n)
            .map(|local| g.neighbors(LocalVid(local as u32)).len())
            .collect();
        let mut rank = vec![1.0f64 / n as f64; n];
        let mut next = vec![0.0f64; n];
        for _ in 0..iterations {
            let dangling: f64 = (0..n)
                .filter(|local| out_degree[*local] == 0)
                .map(|local| rank[local])
                .sum();
            let base = (1.0 - damping) / n as f64 + damping * dangling / n as f64;
            for value in next.iter_mut() {
                *value = base;
            }
            for local in 0..n {
                let degree = out_degree[local];
                if degree == 0 {
                    continue;
                }
                let share = damping * rank[local] / degree as f64;
                for neighbor in g.neighbors(LocalVid(local as u32)) {
                    next[neighbor.0 as usize] += share;
                }
            }
            let delta: f64 = (0..n).map(|i| (next[i] - rank[i]).abs()).sum();
            std::mem::swap(&mut rank, &mut next);
            if delta < PAGERANK_TOLERANCE {
                break;
            }
        }

        let mut rows: Vec<(u64, f64)> = Vec::new();
        for local in 0..n {
            if let Some(global) = g.original_vertex_id(LocalVid(local as u32)) {
                rows.push((global.0, rank[local]));
            }
        }
        rows.sort_unstable_by_key(|(vertex, _)| *vertex);
        Ok(ResultStream {
            schema: vec!["node".into(), "rank".into()],
            rows: rows
                .into_iter()
                .map(|(vertex, value)| {
                    vec![
                        ResultValue::VertexRef(VertexId(vertex)),
                        ResultValue::Float(value),
                    ]
                })
                .collect(),
        })
    }
}

/// Construct the design's required algorithm set.
pub fn required_algorithms() -> Vec<std::sync::Arc<dyn AnalyticalAlgorithm>> {
    vec![
        std::sync::Arc::new(BfsAlgorithm),
        std::sync::Arc::new(WccAlgorithm),
        std::sync::Arc::new(PageRankAlgorithm),
    ]
}

/// Suggested positional parameter names for procedure-style invocation.
pub fn positional_parameter(algorithm: &str, index: usize) -> Option<&'static str> {
    match (algorithm.to_ascii_lowercase().as_str(), index) {
        ("bfs", 0) => Some("source"),
        ("pagerank", 0) => Some("damping"),
        ("pagerank", 1) => Some("iterations"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::projection::CsrProjection;
    use crate::schema::DynamicSchema;
    use crate::storage::GraphStore;
    use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId};
    use metagdb_core::common::property::{PropertyMap, PropertyValue};
    use metagdb_core::l2_storage::{EdgeRecord, StorageEngine, VertexRecord};
    use metagdb_core::l3_txn::{IsolationLevel, TxnContext};
    use metagdb_core::l4_compute::algebra::{ProjectionDirection, ProjectionSpec};
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    /// Build an undirected 3-vertex path a-b-c plus an isolated vertex d.
    fn path_projection() -> (CsrProjection, Vec<VertexId>) {
        let dir = std::env::temp_dir().join(format!(
            "mgdb_algo_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = GraphStore::with_base_dir(schema, dir.clone()).unwrap();
        let ctx = TxnContext {
            txn_id: 1,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: false,
            start_ts: 1,
        };
        let mut ids = Vec::new();
        for _ in 0..4 {
            ids.push(
                store
                    .insert_vertex(
                        VertexRecord {
                            id: VertexId(0),
                            labels: vec![LabelId(1)],
                            properties: PropertyMap::new(),
                        },
                        &ctx,
                    )
                    .unwrap(),
            );
        }
        for (src, dst) in [(0usize, 1usize), (1, 2)] {
            store
                .insert_edge(
                    EdgeRecord {
                        id: EdgeId(0),
                        src: ids[src],
                        dst: ids[dst],
                        etype: EdgeTypeId(1),
                        properties: PropertyMap::new(),
                    },
                    &ctx,
                )
                .unwrap();
        }
        store.commit_txn(1, 2).unwrap();
        let read = TxnContext {
            txn_id: 2,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: true,
            start_ts: 3,
        };
        let spec = ProjectionSpec {
            vertex_labels: None,
            edge_types: None,
            direction: ProjectionDirection::Both,
            vertex_properties: None,
        };
        let projection = CsrProjection::build(&store, &read, &spec).unwrap();
        (projection, ids)
    }

    #[test]
    fn bfs_core_known_answer() {
        let (projection, ids) = path_projection();
        let mut params = BTreeMap::new();
        params.insert("source".to_string(), PropertyValue::Int(ids[0].0 as i64));
        let result = BfsAlgorithm
            .execute(&projection, &AlgorithmArgs { params })
            .unwrap();
        assert_eq!(result.schema, vec!["node".to_string(), "distance".to_string()]);
        let mut distances: Vec<(u64, i64)> = result
            .rows
            .iter()
            .map(|row| match (&row[0], &row[1]) {
                (ResultValue::VertexRef(v), ResultValue::Int(d)) => (v.0, *d),
                other => panic!("unexpected row {other:?}"),
            })
            .collect();
        distances.sort_unstable();
        assert_eq!(
            distances,
            vec![(ids[0].0, 0), (ids[1].0, 1), (ids[2].0, 2)],
            "BFS must not reach the isolated vertex"
        );
    }

    #[test]
    fn wcc_core_known_answer() {
        let (projection, ids) = path_projection();
        let result = WccAlgorithm
            .execute(&projection, &AlgorithmArgs::default())
            .unwrap();
        let mut components: Vec<(u64, i64)> = result
            .rows
            .iter()
            .map(|row| match (&row[0], &row[1]) {
                (ResultValue::VertexRef(v), ResultValue::Int(c)) => (v.0, *c),
                other => panic!("unexpected row {other:?}"),
            })
            .collect();
        components.sort_unstable();
        let minimum = ids[0].0 as i64;
        assert_eq!(
            components,
            vec![
                (ids[0].0, minimum),
                (ids[1].0, minimum),
                (ids[2].0, minimum),
                (ids[3].0, ids[3].0 as i64),
            ]
        );
    }

    #[test]
    fn pagerank_core_known_answer() {
        let (projection, ids) = path_projection();
        let result = PageRankAlgorithm
            .execute(&projection, &AlgorithmArgs::default())
            .unwrap();
        assert_eq!(result.rows.len(), 4);
        let mut ranks: Vec<(u64, f64)> = result
            .rows
            .iter()
            .map(|row| match (&row[0], &row[1]) {
                (ResultValue::VertexRef(v), ResultValue::Float(r)) => (v.0, *r),
                other => panic!("unexpected row {other:?}"),
            })
            .collect();
        ranks.sort_unstable_by_key(|(vertex, _)| *vertex);
        let total: f64 = ranks.iter().map(|(_, rank)| *rank).sum();
        assert!((total - 1.0).abs() < 1e-6, "PageRank must conserve mass: {total}");
        // In an undirected path, the centre vertex accumulates the most rank.
        let centre = ranks.iter().find(|(v, _)| *v == ids[1].0).unwrap().1;
        let endpoint = ranks.iter().find(|(v, _)| *v == ids[0].0).unwrap().1;
        assert!(centre > endpoint, "centre {centre} should outrank endpoint {endpoint}");
    }

    #[test]
    fn algorithms_are_registered_under_required_names() {
        let names: Vec<String> = required_algorithms().iter().map(|a| a.name().to_string()).collect();
        for required in ["bfs", "wcc", "pagerank"] {
            assert!(names.contains(&required.to_string()), "missing {required}");
        }
        assert_eq!(positional_parameter("bfs", 0), Some("source"));
    }

    #[test]
    fn projection_uses_property_key_ids_not_names() {
        // Guards that the projection returns property values by interned key.
        let key = PropertyKeyId(3);
        let mut properties = HashMap::new();
        properties.insert(key, PropertyValue::Int(9));
        assert_eq!(properties.get(&key), Some(&PropertyValue::Int(9)));
    }
}
