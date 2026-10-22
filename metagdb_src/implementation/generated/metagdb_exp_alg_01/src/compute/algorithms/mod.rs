//! The three user-requested analytical algorithms.
//!
//! `algorithms.required` in the frozen design vector names `bfs`, `wcc`, and
//! `pagerank`. They are open user requirements rather than framework catalog
//! entries, so each one is implemented here against the [`GraphProjection`]
//! contract, registers its exact stable name, validates its own arguments, and
//! returns typed `ResultStream` values:
//!
//! | name | columns | rows |
//! |---|---|---|
//! | `bfs` | `node`, `distance` | one row per vertex reachable from the source |
//! | `wcc` | `node`, `component` | one row per vertex, component = minimum global ID |
//! | `pagerank` | `node`, `rank` | one row per vertex |
//!
//! Every algorithm reads `GraphProjection::neighbors` as the *outgoing* edge
//! relation. `bfs` and `pagerank` are directed by definition; `wcc` ignores
//! orientation, so its caller must supply a projection built with
//! `ProjectionDirection::Both` (the language path selects that projection).

use std::sync::Arc;

use metagdb_core::common::ids::VertexId;
use metagdb_core::common::property::{PropertyValue, ResultValue};
use metagdb_core::l4_compute::algebra::AlgorithmArgs;
use metagdb_core::l4_compute::engine::ResultStream;
use metagdb_core::l4_compute::olap::{AnalyticalAlgorithm, GraphProjection};
use metagdb_core::{Error, Result};

mod bfs;
mod pagerank;
mod wcc;

pub use bfs::BfsAlgorithm;
pub use pagerank::PageRankAlgorithm;
pub use wcc::WccAlgorithm;

/// Every algorithm the design vector requires, in registration order.
pub fn default_algorithms() -> Vec<Arc<dyn AnalyticalAlgorithm>> {
    vec![
        Arc::new(BfsAlgorithm),
        Arc::new(WccAlgorithm),
        Arc::new(PageRankAlgorithm),
    ]
}

/// Reject arguments the algorithm does not define.
///
/// An unknown parameter is a caller mistake; accepting it silently would make a
/// typo look like a supported option.
pub(crate) fn reject_unknown_arguments(args: &AlgorithmArgs, allowed: &[&str]) -> Result<()> {
    for key in args.params.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(Error::InvalidQuery(format!(
                "unsupported algorithm argument '{key}'; supported: {}",
                allowed.join(", ")
            )));
        }
    }
    Ok(())
}

/// Read an optional numeric argument, accepting an integer where a float is
/// expected so `pagerank({damping: 1})` is not rejected on a technicality.
pub(crate) fn optional_number(args: &AlgorithmArgs, key: &str) -> Result<Option<f64>> {
    match args.get(key) {
        None | Some(PropertyValue::Null) => Ok(None),
        Some(PropertyValue::Int(value)) => Ok(Some(*value as f64)),
        Some(PropertyValue::Float(value)) => Ok(Some(*value)),
        Some(other) => Err(Error::InvalidQuery(format!(
            "algorithm argument '{key}' must be numeric, got {other:?}"
        ))),
    }
}

/// Resolve a global vertex ID to a projection-local ID, or fail precisely.
pub(crate) fn local_source(
    graph: &dyn GraphProjection,
    source: i64,
    algorithm: &str,
) -> Result<metagdb_core::common::ids::LocalVid> {
    if source <= 0 {
        return Err(Error::InvalidQuery(format!(
            "{algorithm} requires a positive vertex id, got {source}"
        )));
    }
    graph
        .local_vid_of(VertexId(source as u64))
        .ok_or_else(|| {
            Error::NotFound(format!(
                "{algorithm} source vertex {source} is not in the graph projection"
            ))
        })
}

/// Build a `ResultStream` from `(local vertex, value)` pairs in global-ID order.
pub(crate) fn stream_from_pairs(
    graph: &dyn GraphProjection,
    schema: [&str; 2],
    mut pairs: Vec<(u32, ResultValue)>,
) -> Result<ResultStream> {
    pairs.sort_unstable_by_key(|(local, _)| *local);
    let mut rows = Vec::with_capacity(pairs.len());
    for (local, value) in pairs {
        let global = graph
            .original_vertex_id(metagdb_core::common::ids::LocalVid(local))
            .ok_or_else(|| {
                Error::Execute(format!("projection lost local vertex {local}"))
            })?;
        rows.push(vec![ResultValue::VertexRef(global), value]);
    }
    Ok(ResultStream {
        schema: vec![schema[0].to_string(), schema[1].to_string()],
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::projection::CsrProjection;
    use crate::compute::test_support::MemoryStorage;
    use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
    use metagdb_core::l4_compute::algebra::ProjectionDirection;
    use metagdb_core::l4_compute::algebra::ProjectionSpec;

    /// Seven vertices, two components: {1,2,3,4,5} and {6,7}.
    ///
    /// ```text
    /// 1 -> 2      6 -> 7
    /// 1 -> 3
    /// 2 -> 4
    /// 3 -> 4
    /// 3 -> 5
    /// ```
    pub(crate) fn sample() -> MemoryStorage {
        MemoryStorage::new()
            .bare_vertex(1)
            .bare_vertex(2)
            .bare_vertex(3)
            .bare_vertex(4)
            .bare_vertex(5)
            .bare_vertex(6)
            .bare_vertex(7)
            .edge(1, 1, 2, 1)
            .edge(2, 1, 3, 1)
            .edge(3, 2, 4, 1)
            .edge(4, 3, 4, 1)
            .edge(5, 3, 5, 1)
            .edge(6, 6, 7, 1)
    }

    fn project(storage: &MemoryStorage, direction: ProjectionDirection) -> CsrProjection {
        let spec = ProjectionSpec {
            direction,
            ..ProjectionSpec::default()
        };
        CsrProjection::build(storage, &MemoryStorage::context(), &spec)
            .expect("the sample graph projects")
    }

    /// `(global vertex id, integer column)` from a two-column stream.
    fn pairs(stream: &ResultStream) -> Vec<(u64, i64)> {
        stream
            .rows
            .iter()
            .map(|row| {
                let node = match row[0] {
                    ResultValue::VertexRef(id) => id.0,
                    ref other => panic!("unexpected node cell {other:?}"),
                };
                let value = match row[1] {
                    ResultValue::Int(value) => value,
                    ref other => panic!("unexpected value cell {other:?}"),
                };
                (node, value)
            })
            .collect()
    }

    #[test]
    fn each_algorithm_produces_exactly_the_columns_its_signature_declares() {
        // The planner types YIELD columns from this signature and the executor
        // names CallAlgorithm's output from it, so the two must agree with what
        // the algorithm actually returns.
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let bfs_args = AlgorithmArgs {
            params: [("source".to_string(), PropertyValue::Int(1))]
                .into_iter()
                .collect(),
        };
        assert_eq!(
            BfsAlgorithm.execute(&graph, &bfs_args).unwrap().schema,
            crate::compute::planner::algorithm_signature("bfs")
                .unwrap()
                .columns
        );
        assert_eq!(
            WccAlgorithm
                .execute(&graph, &AlgorithmArgs::default())
                .unwrap()
                .schema,
            crate::compute::planner::algorithm_signature("wcc")
                .unwrap()
                .columns
        );
        assert_eq!(
            PageRankAlgorithm
                .execute(&graph, &AlgorithmArgs::default())
                .unwrap()
                .schema,
            crate::compute::planner::algorithm_signature("pagerank")
                .unwrap()
                .columns
        );
    }

    #[test]
    fn registry_registers_every_required_name_exactly_once() {
        let names: Vec<String> = default_algorithms()
            .iter()
            .map(|algorithm| algorithm.name().to_string())
            .collect();
        assert_eq!(names, vec!["bfs", "wcc", "pagerank"]);
    }

    #[test]
    fn bfs_distances_on_the_sample_graph() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let args = AlgorithmArgs {
            params: [("source".to_string(), PropertyValue::Int(1))].into_iter().collect(),
        };
        let stream = BfsAlgorithm.execute(&graph, &args).expect("bfs runs");
        assert_eq!(stream.schema, vec!["node", "distance"]);
        // 1 is the source, 2 and 3 are one hop away, 4 and 5 are two hops away,
        // and 6 and 7 are unreachable from 1.
        assert_eq!(
            pairs(&stream),
            vec![(1, 0), (2, 1), (3, 1), (4, 2), (5, 2)]
        );
    }

    #[test]
    fn bfs_from_the_second_component_reaches_only_that_component() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let args = AlgorithmArgs {
            params: [("source".to_string(), PropertyValue::Int(6))].into_iter().collect(),
        };
        let stream = BfsAlgorithm.execute(&graph, &args).expect("bfs runs");
        assert_eq!(pairs(&stream), vec![(6, 0), (7, 1)]);
    }

    #[test]
    fn bfs_rejects_a_missing_source_and_a_bad_argument_set() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let missing = AlgorithmArgs::default();
        assert!(BfsAlgorithm.execute(&graph, &missing).is_err());

        let absent = AlgorithmArgs {
            params: [("source".to_string(), PropertyValue::Int(9999))]
                .into_iter()
                .collect(),
        };
        assert!(BfsAlgorithm.execute(&graph, &absent).is_err());

        let unknown = AlgorithmArgs {
            params: [
                ("source".to_string(), PropertyValue::Int(1)),
                ("depth".to_string(), PropertyValue::Int(2)),
            ]
            .into_iter()
            .collect(),
        };
        assert!(BfsAlgorithm.execute(&graph, &unknown).is_err());
    }

    #[test]
    fn wcc_labels_both_components_by_their_minimum_global_id() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Both);
        let stream = WccAlgorithm.execute(&graph, &AlgorithmArgs::default()).expect("wcc runs");
        assert_eq!(stream.schema, vec!["node", "component"]);
        assert_eq!(
            pairs(&stream),
            vec![
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (5, 1),
                (6, 6),
                (7, 6),
            ]
        );
    }

    #[test]
    fn wcc_is_direction_agnostic_on_either_projection() {
        // 1 <- 2 is one weakly connected component even though nothing is
        // reachable from 1. A projection lists every edge in at least one
        // adjacency list, so symmetrizing the adjacency it is given recovers
        // exactly the undirected edge set; the answer must not depend on the
        // projection's direction.
        let storage = MemoryStorage::new()
            .bare_vertex(1)
            .bare_vertex(2)
            .edge(1, 2, 1, 1);
        for direction in [
            ProjectionDirection::Outgoing,
            ProjectionDirection::Incoming,
            ProjectionDirection::Both,
        ] {
            let graph = project(&storage, direction);
            let stream = WccAlgorithm.execute(&graph, &AlgorithmArgs::default()).unwrap();
            assert_eq!(
                pairs(&stream),
                vec![(1, 1), (2, 1)],
                "component labelling differed for {direction:?}"
            );
        }
    }

    #[test]
    fn wcc_on_an_empty_graph_is_an_empty_stream() {
        let storage = MemoryStorage::new();
        let graph = project(&storage, ProjectionDirection::Both);
        let stream = WccAlgorithm.execute(&graph, &AlgorithmArgs::default()).unwrap();
        assert!(stream.rows.is_empty());
        assert_eq!(stream.schema, vec!["node", "component"]);
    }

    #[test]
    fn pagerank_matches_the_closed_form_on_two_vertices() {
        // 1 -> 2 with the dangling node 2 redistributing evenly has the exact
        // fixed point rank(1) = 20/57 and rank(2) = 37/57 at damping 0.85.
        let storage = MemoryStorage::new()
            .bare_vertex(1)
            .bare_vertex(2)
            .edge(1, 1, 2, 1);
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let stream = PageRankAlgorithm
            .execute(&graph, &AlgorithmArgs::default())
            .expect("pagerank runs");
        assert_eq!(stream.schema, vec!["node", "rank"]);
        let ranks: Vec<f64> = stream
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Float(value) => value,
                ref other => panic!("unexpected rank cell {other:?}"),
            })
            .collect();
        assert_eq!(ranks.len(), 2);
        assert!(
            (ranks[0] - 20.0 / 57.0).abs() < 1e-8,
            "rank of vertex 1 was {}",
            ranks[0]
        );
        assert!(
            (ranks[1] - 37.0 / 57.0).abs() < 1e-8,
            "rank of vertex 2 was {}",
            ranks[1]
        );
    }

    #[test]
    fn pagerank_keeps_its_invariants_on_the_sample_graph() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        let stream = PageRankAlgorithm
            .execute(&graph, &AlgorithmArgs::default())
            .expect("pagerank runs");
        let values: Vec<f64> = stream
            .rows
            .iter()
            .map(|row| match row[1] {
                ResultValue::Float(value) => value,
                ref other => panic!("unexpected rank cell {other:?}"),
            })
            .collect();
        assert_eq!(values.len(), 7);
        assert!(values.iter().all(|value| *value > 0.0));
        let total: f64 = values.iter().sum();
        assert!((total - 1.0).abs() < 1e-9, "ranks summed to {total}");

        // Vertex 4 receives both of the sample graph's in-edges, so it must
        // outrank every source-only vertex.
        let rank_of = |id: u64| {
            let index = stream
                .rows
                .iter()
                .position(|row| match row[0] {
                    ResultValue::VertexRef(v) => v.0 == id,
                    _ => false,
                })
                .expect("vertex present");
            values[index]
        };
        assert!(rank_of(4) > rank_of(1));
        assert!(rank_of(4) > rank_of(6));
    }

    #[test]
    fn pagerank_rejects_out_of_range_and_unknown_arguments() {
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        for damping in [1.0, -0.5, 2.0] {
            let args = AlgorithmArgs {
                params: [("damping".to_string(), PropertyValue::Float(damping))]
                    .into_iter()
                    .collect(),
            };
            assert!(
                PageRankAlgorithm.execute(&graph, &args).is_err(),
                "damping {damping} must be rejected"
            );
        }
        let unknown = AlgorithmArgs {
            params: [("epsilon".to_string(), PropertyValue::Float(0.1))]
                .into_iter()
                .collect(),
        };
        assert!(PageRankAlgorithm.execute(&graph, &unknown).is_err());
    }

    #[test]
    fn algorithms_use_the_storage_backed_projection_shape() {
        // The projection factory path used by the fixed benchmark must produce
        // graphs the algorithms can consume unchanged.
        let storage = sample();
        let graph = project(&storage, ProjectionDirection::Outgoing);
        assert_eq!(graph.num_vertices(), 7);
        assert_eq!(graph.num_edges(), 6);
        assert_eq!(
            graph.local_vid_of(VertexId(1)),
            Some(metagdb_core::common::ids::LocalVid(0))
        );
        assert_eq!(graph.original_vertex_id(metagdb_core::common::ids::LocalVid(0)), Some(VertexId(1)));
        // Labels and edge types are metadata only; the projection carries the
        // graph structure the algorithms need.
        let _ = (LabelId(1), EdgeTypeId(1), PropertyKeyId(1));
    }
}
