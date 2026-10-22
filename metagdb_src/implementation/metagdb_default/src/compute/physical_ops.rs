/**
 * @brief: metagdb_default implementation-side physical operator tree definitions.
 *
 * These types are constructed by the Optimizer, downcast and executed by the Executor.
 * Wrapped into opaque executable trees via core's PhysicalPlan::new().
 */

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, OpaquePayload, ProjectionSpec, SortOrder,
};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};

// ============================================================
// DML payload types (for safe OpaquePayload downcast)
// ============================================================

/**
 * @brief: Payload for CREATE (var:Label {props}).
 */
#[derive(Debug, Clone)]
pub struct CreateNodePayload {
    pub var: String,
    pub label_id: Option<LabelId>,
    pub properties: Vec<(PropertyKeyId, Expr)>,
}

/**
 * @brief: Payload for CREATE (...)-[:TYPE {props}]->(...).
 */
#[derive(Debug, Clone)]
pub struct CreateEdgePayload {
    pub from_var: String,
    pub to_var: String,
    pub edge_var: Option<String>,
    pub edge_type_ids: Option<Vec<EdgeTypeId>>,
    pub properties: Vec<(PropertyKeyId, Expr)>,
}

/**
 * @brief: Payload for SET var.key = value.
 */
#[derive(Debug, Clone)]
pub struct SetPropertyPayload {
    pub var: String,
    pub key_id: PropertyKeyId,
    pub value: Expr,
}

/**
 * @brief: Payload for SET var:Label1:Label2.
 */
#[derive(Debug, Clone)]
pub struct SetLabelsPayload {
    pub var: String,
    pub label_ids: Vec<LabelId>,
}

/**
 * @brief: Payload for REMOVE var.key.
 */
#[derive(Debug, Clone)]
pub struct RemovePropertyPayload {
    pub var: String,
    pub key_id: PropertyKeyId,
}

/**
 * @brief: Payload for REMOVE var:Label.
 */
#[derive(Debug, Clone)]
pub struct RemoveLabelsPayload {
    pub var: String,
    pub label_ids: Vec<LabelId>,
}

/**
 * @brief: Payload for DELETE / DETACH DELETE.
 */
#[derive(Debug, Clone)]
pub struct DeletePayload {
    pub detach: bool,
    pub target_exprs: Vec<Expr>,
}

/**
 * @brief: Payload for UNWIND expr AS var.
 */
#[derive(Debug, Clone)]
pub struct UnwindPayload {
    pub expr: Expr,
    pub var: String,
}

// ============================================================
// MERGE payload types
// ============================================================

/**
 * @brief: Target type for MERGE to match/create.
 */
#[derive(Debug, Clone)]
pub enum MergeTarget {
    /**
     * @brief: Match/create a node.
     */
    Node {
        var: String,
        label_id: Option<LabelId>,
        properties: Vec<(PropertyKeyId, Expr)>,
    },
    /**
     * @brief: Match/create an edge.
     */
    Edge {
        from_var: String,
        to_var: String,
        edge_var: String,
        edge_type_ids: Vec<EdgeTypeId>,
        properties: Vec<(PropertyKeyId, Expr)>,
    },
}

/**
 * @brief: MERGE payload: match targets + ON CREATE / ON MATCH SET actions.
 */
#[derive(Debug, Clone)]
pub struct MergePayload {
    pub targets: Vec<MergeTarget>,
    /**
     * @brief: ON CREATE SET actions: (variable name, property key ID, value expression).
     */
    pub on_create: Vec<(String, PropertyKeyId, Expr)>,
    /**
     * @brief: ON MATCH SET actions.
     */
    pub on_match: Vec<(String, PropertyKeyId, Expr)>,
}

/**
 * @brief: Payload for variable-length path expansion.
 */
#[derive(Debug, Clone)]
pub struct VarLengthExpandPayload {
    pub from: String,
    pub to: String,
    pub edge: Option<String>,
    pub direction: ExpandDirection,
    pub edge_types: Option<Vec<EdgeTypeId>>,
    /// Minimum hop count (default 1)
    pub min_hops: usize,
    /// Maximum hop count (None means no upper limit, execution defaults to 10)
    pub max_hops: Option<usize>,
}

/**
 * @brief: Physical operator enum for the default implementation, serving as the target for Optimizer lowering and the entry point for Executor execution.
 */
#[derive(Debug, Clone)]
pub enum DefaultPhysicalOp {
    // ── OLTP data sources ──
    /**
     * @brief: Single-row sentinel operator.
     */
    SingleRow,
    /**
     * @brief: Vertex scan, filtered by label, outputs a single column [var].
     */
    VertexScan {
        var: String,
        label: Option<LabelId>,
    },

    // ── Graph traversal ──
    /**
     * @brief: Graph neighbor expansion, expanding from the from column to the to column.
     */
    Expand {
        from: String,
        to: String,
        edge: Option<String>,
        direction: ExpandDirection,
        edge_types: Option<Vec<EdgeTypeId>>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Variable-length path expansion (BFS multi-hop traversal).
     */
    VarLengthExpand {
        from: String,
        to: String,
        edge: Option<String>,
        direction: ExpandDirection,
        edge_types: Option<Vec<EdgeTypeId>>,
        min_hops: usize,
        max_hops: Option<usize>,
        input: Box<DefaultPhysicalOp>,
    },

    // ── Relational algebra ──
    /**
     * @brief: Filter row stream by predicate expression.
     */
    Filter {
        predicate: Expr,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Project input rows by (Expr, alias) list.
     */
    Project {
        items: Vec<(Expr, String)>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Sort operator.
     */
    Sort {
        keys: Vec<(Expr, SortOrder)>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Pagination operator (SKIP/LIMIT).
     */
    Limit {
        limit: usize,
        offset: usize,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Aggregate operator (GROUP BY + aggregate functions).
     */
    Aggregate {
        group_by: Vec<(Expr, String)>,
        aggs: Vec<(AggregateExpr, String)>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: UNION [ALL] operator.
     */
    Union {
        all: bool,
        inputs: Vec<DefaultPhysicalOp>,
    },
    /**
     * @brief: Cartesian product operator (CrossJoin, NestedLoop implementation).
     */
    CrossJoin {
        left: Box<DefaultPhysicalOp>,
        right: Box<DefaultPhysicalOp>,
    },

    // ── OLAP ──
    /**
     * @brief: Build in-memory graph projection from the storage engine.
     */
    BuildProjection {
        spec: ProjectionSpec,
    },
    /**
     * @brief: Execute an algorithm on the in-memory graph projection.
     */
    RunAlgorithm {
        name: String,
        args: AlgorithmArgs,
        input: Box<DefaultPhysicalOp>,
    },

    // ── DML operators ──
    /**
     * @brief: Create a vertex.
     */
    CreateNode {
        var: String,
        label_id: Option<LabelId>,
        properties: Vec<(PropertyKeyId, Expr)>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Create an edge.
     */
    CreateEdge {
        from_var: String,
        to_var: String,
        edge_var: Option<String>,
        edge_type_ids: Option<Vec<EdgeTypeId>>,
        properties: Vec<(PropertyKeyId, Expr)>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Set a property value.
     */
    SetProperty {
        var: String,
        key_id: PropertyKeyId,
        value: Expr,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Set labels.
     */
    SetLabels {
        var: String,
        label_ids: Vec<LabelId>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Remove a property.
     */
    RemoveProperty {
        var: String,
        key_id: PropertyKeyId,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Remove labels.
     */
    RemoveLabels {
        var: String,
        label_ids: Vec<LabelId>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: Delete vertices/edges (supports DETACH).
     */
    Delete {
        detach: bool,
        targets: Vec<Expr>,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: UNWIND list expansion operator.
     */
    Unwind {
        expr: Expr,
        var: String,
        input: Box<DefaultPhysicalOp>,
    },
    /**
     * @brief: MERGE operator (match-or-create semantics, supports nodes and edges).
     */
    Merge {
        payload: MergePayload,
        input: Box<DefaultPhysicalOp>,
    },

    // ── Extension operators (for operations like Aggregate whose semantics are not yet mature) ──
    /**
     * @brief: Opaque extension operator, carrying operations like Merge / Aggregate whose semantics are not yet mature.
     */
    ExtensionOp {
        name: String,
        payload: OpaquePayload,
        input: Box<DefaultPhysicalOp>,
    },
}
