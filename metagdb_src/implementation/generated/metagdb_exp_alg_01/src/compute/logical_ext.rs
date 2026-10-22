//! Named extension payloads carried by `LogicalOp::ExtensionOp`.
//!
//! The framework's logical IR covers scans, expansion, filters, projection,
//! aggregation, ordering, pagination, unions, and algorithm invocation.  Four
//! Cypher constructs have no faithful framework form, so they travel as named
//! extension payloads whose types live here:
//!
//! | extension name | payload | why the framework IR is not enough |
//! |---|---|---|
//! | `cypher.unwind` | [`UnwindSpec`] | list-to-rows iteration is not a relational operator in the IR |
//! | `cypher.distinct` | `()` | `DISTINCT` is not `Aggregate`; it keeps the projected tuples |
//! | `cypher.var_expand` | [`VarExpandSpec`] | variable-length traversal enumerates *paths*, not neighbours |
//! | `cypher.mutate` | [`MutateSpec`] | multi-operation DML over the current row |
//! | `cypher.merge` | [`MergeSpec`] | `MERGE` probes before it writes, per row |
//!
//! Using `ExtensionOp` for these keeps the framework IR as the plan's backbone
//! instead of hiding the whole statement behind one opaque node.

use metagdb_core::common::ids::EdgeTypeId;
use metagdb_core::l4_compute::algebra::{ExpandDirection, LogicalOp};
use metagdb_core::l4_compute::expression::Expr;

use super::physical_ops::WriteOp;

/// Extension name for `UNWIND`.
pub const EXT_UNWIND: &str = "cypher.unwind";
/// Extension name for `DISTINCT`.
pub const EXT_DISTINCT: &str = "cypher.distinct";
/// Extension name for variable-length traversal.
pub const EXT_VAR_EXPAND: &str = "cypher.var_expand";
/// Extension name for the statement's mutations.
pub const EXT_MUTATE: &str = "cypher.mutate";
/// Extension name for `MERGE`.
pub const EXT_MERGE: &str = "cypher.merge";

/// `UNWIND <list> AS <alias>`
#[derive(Debug, Clone)]
pub struct UnwindSpec {
    pub list: Expr,
    pub alias: String,
}

/// `MATCH (from)-[edge:TYPE*min..max]->(to)`, optionally binding the path.
#[derive(Debug, Clone)]
pub struct VarExpandSpec {
    pub from: String,
    pub to: String,
    pub edge: Option<String>,
    /// Bound to the traversed path when the pattern part names one.
    pub path: Option<String>,
    pub direction: ExpandDirection,
    pub edge_types: Option<Vec<EdgeTypeId>>,
    /// Minimum number of hops; `0` contributes the start vertex itself.
    pub min: u32,
    /// Maximum number of hops; `None` means unbounded.
    pub max: Option<u32>,
}

/// The statement's mutations, applied per input row, then projected.
#[derive(Debug, Clone)]
pub struct MutateSpec {
    pub writes: Vec<WriteOp>,
    pub projection: Vec<(Expr, String)>,
}

/// `MERGE pattern [ON MATCH SET …] [ON CREATE SET …]`
#[derive(Debug, Clone)]
pub struct MergeSpec {
    /// Probes one input row for the pattern; empty means "not found".
    pub probe: Box<LogicalOp>,
    /// Writes performed when the probe found nothing.
    pub create: Vec<WriteOp>,
    /// `(on_match, writes)` actions applied after the merge resolved.
    pub actions: Vec<(bool, Vec<WriteOp>)>,
}
