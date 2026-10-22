/**
 * @brief: Expression IR -- a unified scalar expression intermediate representation.
 *
 * Describes a computation tree that computes a scalar value from a single row of
 * data. It is contained by LogicalOp rather than forming an independent pipeline.
 * Each of the three languages parses into its own private AST, and its respective
 * Planner translates it into this IR.
 */

use crate::common::ids::PropertyKeyId;

/**
 * @brief: Scalar expression node, forming the expression computation tree.
 */
#[derive(Debug, Clone)]
pub enum Expr {
    // -- literals --
    /**
     * @brief: Signed 64-bit integer literal.
     */
    Int(i64),
    /**
     * @brief: IEEE 754 double-precision floating-point literal.
     */
    Float(f64),
    /**
     * @brief: UTF-8 string literal.
     */
    String(String),
    /**
     * @brief: Boolean literal.
     */
    Bool(bool),
    /**
     * @brief: NULL literal.
     */
    Null,

    // -- variable and property references --
    /**
     * @brief: Intra-row variable reference, located by name in the schema.
     */
    VarRef { name: String },
    /**
     * @brief: Property reference, reading a property value by PropertyKeyId from the
     *         vertex/edge of the source expression.
     */
    PropRef { source: Box<Expr>, key: PropertyKeyId },

    // -- arithmetic operations --
    /**
     * @brief: Unary negation.
     */
    Neg(Box<Expr>),
    /**
     * @brief: Addition.
     */
    Add(Box<Expr>, Box<Expr>),
    /**
     * @brief: Subtraction.
     */
    Sub(Box<Expr>, Box<Expr>),
    /**
     * @brief: Multiplication.
     */
    Mul(Box<Expr>, Box<Expr>),
    /**
     * @brief: Division.
     */
    Div(Box<Expr>, Box<Expr>),
    /**
     * @brief: Modulo.
     */
    Mod(Box<Expr>, Box<Expr>),

    // -- comparison operations --
    /**
     * @brief: Equality comparison. If either operand is Null, the result is Null.
     */
    Eq(Box<Expr>, Box<Expr>),
    /**
     * @brief: Not-equal comparison. If either operand is Null, the result is Null.
     */
    Neq(Box<Expr>, Box<Expr>),
    /**
     * @brief: Greater-than comparison. If either operand is Null, the result is Null.
     */
    Gt(Box<Expr>, Box<Expr>),
    /**
     * @brief: Greater-than-or-equal comparison. If either operand is Null, the
     *         result is Null.
     */
    Gte(Box<Expr>, Box<Expr>),
    /**
     * @brief: Less-than comparison. If either operand is Null, the result is Null.
     */
    Lt(Box<Expr>, Box<Expr>),
    /**
     * @brief: Less-than-or-equal comparison. If either operand is Null, the result
     *         is Null.
     */
    Lte(Box<Expr>, Box<Expr>),
    /**
     * @brief: Null check. Does not propagate Null; always returns Bool.
     */
    IsNull(Box<Expr>),
    /**
     * @brief: Non-null check. Does not propagate Null; always returns Bool.
     */
    IsNotNull(Box<Expr>),

    // -- logical operations --
    /**
     * @brief: Logical AND. Null AND false = false; otherwise Null propagates.
     */
    And(Box<Expr>, Box<Expr>),
    /**
     * @brief: Logical OR. Null OR true = true; otherwise Null propagates.
     */
    Or(Box<Expr>, Box<Expr>),
    /**
     * @brief: Logical NOT. NOT Null = Null.
     */
    Not(Box<Expr>),

    // -- string operations --
    /**
     * @brief: String prefix match.
     */
    StartsWith(Box<Expr>, Box<Expr>),
    /**
     * @brief: String suffix match.
     */
    EndsWith(Box<Expr>, Box<Expr>),
    /**
     * @brief: String containment match.
     */
    Contains(Box<Expr>, Box<Expr>),
    /**
     * @brief: String concatenation.
     */
    Concat(Vec<Expr>),

    // -- conditional expressions --
    /**
     * @brief: CASE WHEN ... THEN ... ELSE ... END multi-way branch expression.
     */
    Case {
        /**
         * @brief: List of (condition, result) branch pairs.
         */
        cases: Vec<(Expr, Expr)>,
        /**
         * @brief: Fallback expression when no branch matches.
         */
        else_branch: Option<Box<Expr>>,
    },
    /**
     * @brief: COALESCE: returns the first non-Null argument value.
     */
    Coalesce(Vec<Expr>),

    // -- list/map operations --
    /**
     * @brief: List literal, e.g., [1, 2, 3].
     */
    List(Vec<Expr>),
    /**
     * @brief: List subscript access, index is 0-based.
     */
    ListIndex {
        list: Box<Expr>,
        index: Box<Expr>,
    },
    /**
     * @brief: Get the length of a list or the number of characters in a string.
     */
    Size {
        value: Box<Expr>,
    },

    // -- function calls --
    /**
     * @brief: Named function call, covering built-in functions, UDFs, and
     *         language-specific functions.
     */
    FunctionCall {
        /**
         * @brief: Function registration name.
         */
        name: String,
        /**
         * @brief: Positional argument list.
         */
        args: Vec<Expr>,
    },
}

/**
 * @brief: Aggregate expression, appearing only in the aggs list of an Aggregate
 *         operator.
 */
#[derive(Debug, Clone)]
pub enum AggregateExpr {
    /**
     * @brief: COUNT(*): count all rows.
     */
    CountAll,
    /**
     * @brief: COUNT(expr): count the number of non-Null values.
     */
    Count(Expr),
    /**
     * @brief: COUNT(DISTINCT expr): count after deduplication.
     */
    CountDistinct(Expr),
    /**
     * @brief: SUM(expr): compute the sum, ignoring Nulls.
     */
    Sum(Expr),
    /**
     * @brief: SUM(DISTINCT expr): compute the sum after deduplication.
     */
    SumDistinct(Expr),
    /**
     * @brief: AVG(expr): compute the average, ignoring Nulls.
     */
    Avg(Expr),
    /**
     * @brief: MIN(expr): compute the minimum, ignoring Nulls.
     */
    Min(Expr),
    /**
     * @brief: MAX(expr): compute the maximum, ignoring Nulls.
     */
    Max(Expr),
    /**
     * @brief: COLLECT(expr): collect all values into a list.
     */
    Collect(Expr),
    /**
     * @brief: COLLECT(DISTINCT expr): collect after deduplication.
     */
    CollectDistinct(Expr),
}
