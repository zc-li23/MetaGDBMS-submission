/**
 * @brief: L4 compute layer intermediate representation (IR) definitions, including
 *         logical plans and physical plans.
 */

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use crate::common::property::PropertyValue;
use crate::l4_compute::expression::Expr;

// ============================================================
// LogicalOp -- logical operators
// ============================================================

/**
 * @brief: Logical operator, defining language-agnostic algebraic computation semantics.
 */
#[derive(Debug, Clone)]
pub enum LogicalOp {
    // -- data source --
    SingleRow,
    VertexScan {
        var: String,
        label: Option<LabelId>,
    },
    EdgeScan {
        var: String,
        edge_type: Option<EdgeTypeId>,
    },

    // -- graph traversal --
    Expand {
        from: String,
        to: String,
        edge: Option<String>,
        direction: ExpandDirection,
        edge_types: Option<Vec<EdgeTypeId>>,
        input: Box<LogicalOp>,
    },

    // -- relational algebra --
    Filter {
        predicate: Expr,
        input: Box<LogicalOp>,
    },
    Project {
        items: Vec<(Expr, String)>,
        input: Box<LogicalOp>,
    },
    Aggregate {
        group_by: Vec<Expr>,
        aggs: Vec<(crate::l4_compute::expression::AggregateExpr, String)>,
        input: Box<LogicalOp>,
    },
    Sort {
        keys: Vec<(Expr, SortOrder)>,
        input: Box<LogicalOp>,
    },
    Limit {
        limit: usize,
        offset: usize,
        input: Box<LogicalOp>,
    },
    Join {
        kind: JoinKind,
        on: Vec<(Expr, Expr)>,
        left: Box<LogicalOp>,
        right: Box<LogicalOp>,
    },
    Union {
        all: bool,
        inputs: Vec<LogicalOp>,
    },

    // -- OLAP invocation path --
    GraphProjection { spec: ProjectionSpec },
    CallAlgorithm {
        name: String,
        args: AlgorithmArgs,
        input: Box<LogicalOp>,
    },

    // -- extension point --
    ExtensionOp {
        name: String,
        payload: OpaquePayload,
        input: Box<LogicalOp>,
    },
}

/**
 * @brief: Logical query plan, encapsulating the logical operator tree.
 */
#[derive(Debug, Clone)]
pub struct LogicalPlan {
    pub root: LogicalOp,
}

// ============================================================
// PhysicalPlan -- opaque executable tree
// ============================================================

/**
 * @brief: Physical execution plan, internally holding the concrete physical operator
 *         tree defined by the implementation side.
 *
 * core does not define the specific form of physical operators; it only provides
 * an opaque wrapper. The implementation side is responsible for construction,
 * lowering, downcasting, and execution.
 */
pub struct PhysicalPlan {
    inner: Box<dyn Any + Send + Sync>,
}

impl PhysicalPlan {
    /**
     * @brief: Construct a PhysicalPlan from an implementation-side physical operator
     *         root node.
     * @param root: The implementation-side physical operator root node; must satisfy
     *              Any + Send + Sync.
     */
    pub fn new<T: Any + Send + Sync>(root: T) -> Self {
        Self {
            inner: Box::new(root),
        }
    }

    /**
     * @brief: Retrieve an immutable reference to the inner physical operator root
     *         and downcast it to the target type.
     * @return: Some(&T) if the type matches, otherwise None.
     */
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
}

impl std::fmt::Debug for PhysicalPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysicalPlan")
            .field("type_id", &self.inner.type_id())
            .finish()
    }
}

// ============================================================
// Auxiliary types
// ============================================================

/**
 * @brief: Opaque payload for extension operators, interpreted by the implementation
 *         side.
 */
#[derive(Debug, Clone)]
pub struct OpaquePayload {
    inner: Arc<dyn Any + Send + Sync>,
}

impl OpaquePayload {
    pub fn new<T: Any + Send + Sync>(payload: T) -> Self {
        Self {
            inner: Arc::new(payload),
        }
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
}

/**
 * @brief: Graph traversal direction semantics.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpandDirection {
    Outgoing,
    Incoming,
    Both,
}

/**
 * @brief: Sort direction.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/**
 * @brief: Join type.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinKind {
    Inner,
    Left,
    Semi,
    Anti,
    Cross,
}

/**
 * @brief: Project field descriptor (deprecated, kept for backward compatibility;
 *         new code should use (Expr, alias) tuples).
 */
#[derive(Debug, Clone)]
pub enum ProjectField {
    Var(String),
    Property { var: String, key: PropertyKeyId },
}

/**
 * @brief: In-memory graph projection specification, determining which vertices,
 *         edges, and properties are loaded into memory.
 */
#[derive(Debug, Clone, Default)]
pub struct ProjectionSpec {
    pub vertex_labels: Option<Vec<LabelId>>,
    pub edge_types: Option<Vec<EdgeTypeId>>,
    pub direction: ProjectionDirection,
    pub vertex_properties: Option<Vec<PropertyKeyId>>,
}

/**
 * @brief: Projection direction semantics.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectionDirection {
    #[default]
    Outgoing,
    Incoming,
    Both,
}

/**
 * @brief: Algorithm invocation parameter set.
 */
#[derive(Debug, Clone, Default)]
pub struct AlgorithmArgs {
    pub params: BTreeMap<String, PropertyValue>,
}

impl AlgorithmArgs {
    /**
     * @brief: Get a parameter value by key name.
     * @param key: The parameter name.
     * @return: A reference to the property value if it exists.
     */
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.params.get(key)
    }

    /**
     * @brief: Get a parameter value by key name and attempt to convert it to i64.
     * @param key: The parameter name.
     * @return: The i64 value if conversion is possible.
     */
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.params.get(key).and_then(|v| v.as_i64())
    }
}
