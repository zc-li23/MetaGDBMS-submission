/**
 * @brief: Cypher AST definitions shared by parser and planner.
 *
 * Defines the complete AST node types for the Cypher query language, including:
 * - Statement level: Query / StandaloneCall
 * - Clause level: Match / OptionalMatch / With / Return / Unwind / Union
 * - DML clauses: Create / Merge / Delete / Set / Remove
 * - Pattern structures: PatternElement / NodePattern / EdgePattern (supports anonymous nodes, undirected/bidirectional edges, variable-length paths)
 * - Expressions: arithmetic, comparison, logical, list, Map, function calls, parameters, NULL, etc.
 */

use std::any::Any;
use std::collections::BTreeMap;

use metagdb_core::common::property::PropertyValue;
use metagdb_core::l4_compute::engine::LanguageAst;
use metagdb_core::l5_interaction::QueryLanguage;

// ============================================================
// Top-level AST
// ============================================================

/**
 * @brief: Cypher AST top-level enum, distinguishing queries from standalone CALL statements.
 */
#[derive(Debug, Clone)]
pub enum CypherAst {
    /**
     * @brief: Regular query (may contain multiple clauses, including read/write/DML).
     */
    Query(Box<Query>),
    /**
     * @brief: Standalone CALL algorithm/procedure invocation (e.g., CALL bfs(source=1)).
     */
    CallAlgorithm {
        algo: String,
        args: BTreeMap<String, PropertyValue>,
    },
}

impl LanguageAst for CypherAst {
    fn language(&self) -> QueryLanguage {
        QueryLanguage::Cypher
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================
// Query structure
// ============================================================

/**
 * @brief: Query body, composed of an ordered clause chain and optional UNION.
 */
#[derive(Debug, Clone)]
pub struct Query {
    /**
     * @brief: Ordered list of clauses. In Cypher semantics, evaluated sequentially from front to back,
     *         variables are passed across clause boundaries following projection semantics.
     */
    pub clauses: Vec<Clause>,
    /**
     * @brief: If the query is composed of multiple SingleQuery parts connected by UNION [ALL],
     *         the branch Queries are recorded here.
     */
    pub unions: Option<UnionClause>,
}

// ============================================================
// Clause types
// ============================================================

/**
 * @brief: Query clause enum, covering read/write clauses and DML.
 */
#[derive(Debug, Clone)]
pub enum Clause {
    /**
     * @brief: MATCH clause (with WHERE).
     */
    Match(MatchClause),
    /**
     * @brief: OPTIONAL MATCH clause.
     */
    OptionalMatch(MatchClause),
    /**
     * @brief: WITH clause (intermediate projection).
     */
    With(WithClause),
    /**
     * @brief: RETURN clause (final projection).
     */
    Return(ReturnClause),
    /**
     * @brief: UNWIND clause, flattens a list into rows.
     */
    Unwind(UnwindClause),
    /**
     * @brief: CREATE clause.
     */
    Create(CreateClause),
    /**
     * @brief: MERGE clause (with ON MATCH / ON CREATE actions).
     */
    Merge(MergeClause),
    /**
     * @brief: DELETE / DETACH DELETE clause.
     */
    Delete(DeleteClause),
    /**
     * @brief: SET clause.
     */
    Set(SetClause),
    /**
     * @brief: REMOVE clause.
     */
    Remove(RemoveClause),
    /**
     * @brief: CALL subquery (CALL statement within a query).
     */
    CallSubquery(CallSubqueryClause),
}

// ============================================================
// Read clause structs
// ============================================================

/**
 * @brief: MATCH clause, containing graph pattern and optional WHERE filter.
 */
#[derive(Debug, Clone)]
pub struct MatchClause {
    pub pattern: MatchPattern,
    pub where_clause: Option<AstExpr>,
}

/**
 * @brief: WITH clause, defining intermediate projection and optional WHERE/ORDER BY/SKIP/LIMIT.
 */
#[derive(Debug, Clone)]
pub struct WithClause {
    pub distinct: bool,
    pub items: Vec<ReturnItem>,
    pub where_clause: Option<AstExpr>,
    pub order_by: Option<OrderBy>,
    pub skip: Option<AstExpr>,
    pub limit: Option<AstExpr>,
}

/**
 * @brief: RETURN clause, defining final projection and optional ORDER BY/SKIP/LIMIT.
 */
#[derive(Debug, Clone)]
pub struct ReturnClause {
    pub distinct: bool,
    pub items: Vec<ReturnItem>,
    pub order_by: Option<OrderBy>,
    pub skip: Option<AstExpr>,
    pub limit: Option<AstExpr>,
}

/**
 * @brief: UNWIND clause.
 */
#[derive(Debug, Clone)]
pub struct UnwindClause {
    /**
     * @brief: The list expression to be unwound.
     */
    pub expr: AstExpr,
    /**
     * @brief: Variable name bound to each element of the list.
     */
    pub var: String,
}

/**
 * @brief: A single UNION branch, carrying ALL/DISTINCT flag.
 */
#[derive(Debug, Clone)]
pub struct UnionBranch {
    /**
     * @brief: Whether this is UNION ALL.
     */
    pub all: bool,
    /**
     * @brief: Branch query body.
     */
    pub query: Query,
}

/**
 * @brief: UNION clause, connecting multiple query branches.
 */
#[derive(Debug, Clone)]
pub struct UnionClause {
    /**
     * @brief: List of UNION branches (each carrying its own ALL/DISTINCT flag).
     */
    pub branches: Vec<UnionBranch>,
}

// ============================================================
// DML clause structs
// ============================================================

/**
 * @brief: CREATE clause.
 */
#[derive(Debug, Clone)]
pub struct CreateClause {
    pub pattern: MatchPattern,
}

/**
 * @brief: MERGE clause.
 */
#[derive(Debug, Clone)]
pub struct MergeClause {
    pub pattern: MatchPattern,
    /**
     * @brief: List of ON CREATE / ON MATCH actions.
     */
    pub actions: Vec<MergeAction>,
}

/**
 * @brief: Actions attached to MERGE.
 */
#[derive(Debug, Clone)]
pub enum MergeAction {
    /**
     * @brief: ON MATCH ... SET ...
     */
    OnMatch(Vec<SetItem>),
    /**
     * @brief: ON CREATE ... SET ...
     */
    OnCreate(Vec<SetItem>),
}

/**
 * @brief: DELETE clause (plain delete or DETACH DELETE).
 */
#[derive(Debug, Clone)]
pub enum DeleteClause {
    /**
     * @brief: DELETE expression list.
     */
    Delete(Vec<AstExpr>),
    /**
     * @brief: DETACH DELETE expression list (cascading edge deletion).
     */
    DetachDelete(Vec<AstExpr>),
}

/**
 * @brief: SET clause.
 */
#[derive(Debug, Clone)]
pub struct SetClause {
    pub items: Vec<SetItem>,
}

/**
 * @brief: A single assignment within a SET clause.
 */
#[derive(Debug, Clone)]
pub enum SetItem {
    /**
     * @brief: Property assignment, e.g., n.prop = value.
     */
    Property {
        var: String,
        key: String,
        value: AstExpr,
    },
    /**
     * @brief: Label assignment, e.g., n:Label.
     */
    Labels {
        var: String,
        labels: Vec<String>,
    },
}

/**
 * @brief: REMOVE clause.
 */
#[derive(Debug, Clone)]
pub struct RemoveClause {
    pub items: Vec<RemoveItem>,
}

/**
 * @brief: A single removal item within a REMOVE clause.
 */
#[derive(Debug, Clone)]
pub enum RemoveItem {
    /**
     * @brief: Property removal, e.g., n.prop.
     */
    Property {
        var: String,
        key: String,
    },
    /**
     * @brief: Label removal, e.g., n:Label.
     */
    Labels {
        var: String,
        labels: Vec<String>,
    },
}

/**
 * @brief: CALL subquery (CALL statement within a query).
 */
#[derive(Debug, Clone)]
pub struct CallSubqueryClause {
    pub algo: String,
    pub args: BTreeMap<String, PropertyValue>,
}

// ============================================================
// Pattern structures
// ============================================================

/**
 * @brief: Graph match pattern, composed of a set of PatternElements.
 *
 * MATCH (a)-[r]->(b), (b)-[s]->(c) produces two PatternElements.
 */
#[derive(Debug, Clone)]
pub struct MatchPattern {
    /**
     * @brief: Comma-separated list of pattern elements.
     */
    pub elements: Vec<PatternElement>,
}

/**
 * @brief: A single path pattern element, e.g., (a:Person)-[:KNOWS]->(b).
 */
#[derive(Debug, Clone)]
pub struct PatternElement {
    /**
     * @brief: Starting node of the path.
     */
    pub node: NodePattern,
    /**
     * @brief: Edge chain in the path (may be empty, indicating single-node match only).
     */
    pub edges: Vec<PatternEdge>,
}

/**
 * @brief: Vertex pattern, e.g., (a:Person {name: "Alice"}).
 */
#[derive(Debug, Clone)]
pub struct NodePattern {
    /**
     * @brief: Bound variable name (e.g., a). None indicates an anonymous node.
     */
    pub var: Option<String>,
    /**
     * @brief: Label name. None indicates no label constraint.
     */
    pub label: Option<String>,
    /**
     * @brief: Vertex property constraints (only used for inline properties in MERGE/MATCH).
     */
    pub properties: Option<BTreeMap<String, AstExpr>>,
}

/**
 * @brief: Edge pattern, e.g., -[:KNOWS*1..3 {since: 2020}]->.
 */
#[derive(Debug, Clone)]
pub struct PatternEdge {
    /**
     * @brief: Bound edge variable name. None indicates anonymous.
     */
    pub edge_var: Option<String>,
    /**
     * @brief: List of edge type names. None indicates no type constraint.
     */
    pub edge_types: Option<Vec<String>>,
    /**
     * @brief: Direction semantics.
     */
    pub direction: EdgeDirection,
    /**
     * @brief: Variable-length path range (None indicates fixed-length single hop).
     */
    pub range: Option<RangeLiteral>,
    /**
     * @brief: Edge property constraints.
     */
    pub properties: Option<BTreeMap<String, AstExpr>>,
    /**
     * @brief: Target vertex pattern the edge points to.
     */
    pub target: NodePattern,
}

/**
 * @brief: Edge direction enum.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    /**
     * @brief: Outgoing edge: -[:TYPES]->
     */
    Outgoing,
    /**
     * @brief: Incoming edge: <-[:TYPES]-
     */
    Incoming,
    /**
     * @brief: Undirected/bidirectional: -[:TYPES]-
     */
    Both,
}

/**
 * @brief: Variable-length path range literal.
 *
 * Example: *1..3 means start=Some(1), end=Some(3); *..5 means start=None, end=Some(5).
 */
#[derive(Debug, Clone)]
pub struct RangeLiteral {
    /**
     * @brief: Minimum hop count (min in *min..), None means 1.
     */
    pub start: Option<i64>,
    /**
     * @brief: Maximum hop count (max in *..max), None means no upper limit.
     */
    pub end: Option<i64>,
}

// ============================================================
// Return / With item
// ============================================================

/**
 * @brief: A single projection item in RETURN / WITH.
 */
#[derive(Debug, Clone)]
pub enum ReturnItem {
    /**
     * @brief: Simple variable reference, e.g., RETURN v.
     */
    Var(String),
    /**
     * @brief: Property reference, e.g., RETURN v.name.
     */
    Property { var: String, key: String },
    /**
     * @brief: Aliased expression, e.g., RETURN count(*) AS cnt.
     */
    Alias { expr: Box<AstExpr>, alias: String },
    /**
     * @brief: General expression (no alias), e.g., RETURN 1 + 2.
     */
    Expression(AstExpr),
}

// ============================================================
// Ordering
// ============================================================

/**
 * @brief: ORDER BY clause.
 */
#[derive(Debug, Clone)]
pub struct OrderBy {
    pub items: Vec<SortItem>,
}

/**
 * @brief: A single item in ORDER BY.
 */
#[derive(Debug, Clone)]
pub struct SortItem {
    pub expr: AstExpr,
    /**
     * @brief: Sort direction. None means default ascending.
     */
    pub order: Option<SortOrder>,
}

/**
 * @brief: Sort direction.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

// ============================================================
// Expression AST
// ============================================================

/**
 * @brief: Cypher expression AST node.
 *
 * Covers literals, variable/property references, arithmetic, comparison, logical, string, list,
 * Map, function calls, CASE, parameters, and null testing, etc.
 */
#[derive(Debug, Clone)]
pub enum AstExpr {
    // ── Literals ──
    /**
     * @brief: Signed integer.
     */
    Int(i64),
    /**
     * @brief: Floating-point number.
     */
    Float(f64),
    /**
     * @brief: UTF-8 string.
     */
    Str(String),
    /**
     * @brief: Boolean literal.
     */
    Bool(bool),
    /**
     * @brief: NULL literal.
     */
    Null,

    // ── Parameters ──
    /**
     * @brief: Named parameter reference, e.g., $param.
     */
    Parameter(String),

    // ── Variable and property references ──
    /**
     * @brief: Inline variable reference.
     */
    Var(String),
    /**
     * @brief: Property reference var.key.
     */
    Prop { var: String, key: String },

    // ── Arithmetic operations ──
    /**
     * @brief: Unary negation.
     */
    Neg(Box<AstExpr>),
    /**
     * @brief: Addition.
     */
    Add(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Subtraction.
     */
    Sub(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Multiplication.
     */
    Mul(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Division.
     */
    Div(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Modulo.
     */
    Mod(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Exponentiation.
     */
    Pow(Box<AstExpr>, Box<AstExpr>),

    // ── Comparison operations ──
    /**
     * @brief: Equality comparison.
     */
    Eq(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Inequality comparison.
     */
    Neq(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Greater-than comparison.
     */
    Gt(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Greater-than-or-equal comparison.
     */
    Gte(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Less-than comparison.
     */
    Lt(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Less-than-or-equal comparison.
     */
    Lte(Box<AstExpr>, Box<AstExpr>),

    // ── Null testing ──
    /**
     * @brief: IS NULL test.
     */
    IsNull(Box<AstExpr>),
    /**
     * @brief: IS NOT NULL test.
     */
    IsNotNull(Box<AstExpr>),

    // ── Logical operations ──
    /**
     * @brief: Logical AND.
     */
    And(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Logical OR.
     */
    Or(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Logical XOR.
     */
    Xor(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Logical NOT.
     */
    Not(Box<AstExpr>),

    // ── Containment / membership tests ──
    /**
     * @brief: IN test, e.g., a IN [1,2,3].
     */
    In(Box<AstExpr>, Box<AstExpr>),

    // ── String operations ──
    /**
     * @brief: String prefix match.
     */
    StartsWith(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: String suffix match.
     */
    EndsWith(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: String containment match.
     */
    Contains(Box<AstExpr>, Box<AstExpr>),
    /**
     * @brief: Regular expression match.
     */
    RegexMatch(Box<AstExpr>, Box<AstExpr>),

    // ── String concatenation ──
    /**
     * @brief: String concatenation (obtained by lowering the + operator).
     */
    Concat(Box<AstExpr>, Box<AstExpr>),

    // ── List ──
    /**
     * @brief: List literal.
     */
    List(Vec<AstExpr>),
    /**
     * @brief: 0-based list subscript access.
     */
    ListIndex {
        list: Box<AstExpr>,
        index: Box<AstExpr>,
    },
    /**
     * @brief: List slice list[start..end].
     */
    ListSlice {
        list: Box<AstExpr>,
        start: Option<Box<AstExpr>>,
        end: Option<Box<AstExpr>>,
    },

    // ── Map ──
    /**
     * @brief: Map literal {key: val, ...}.
     */
    Map(Vec<(String, AstExpr)>),
    /**
     * @brief: Map property access map.key.
     */
    MapAccess {
        map: Box<AstExpr>,
        key: String,
    },

    // ── Function calls ──
    /**
     * @brief: Named function call.
     */
    FunctionCall {
        name: String,
        distinct: bool,
        args: Vec<AstExpr>,
    },

    // ── CASE expression ──
    /**
     * @brief: CASE WHEN ... THEN ... ELSE ... END.
     */
    Case {
        /**
         * @brief: Optional test expression (Simple CASE).
         */
        test: Option<Box<AstExpr>>,
        /**
         * @brief: List of (WHEN, THEN) branches.
         */
        cases: Vec<(AstExpr, AstExpr)>,
        /**
         * @brief: ELSE branch.
         */
        else_branch: Option<Box<AstExpr>>,
    },

    // ── Existential subquery ──
    /**
     * @brief: EXISTS { pattern } expression.
     */
    ExistsPattern(MatchPattern),
}
