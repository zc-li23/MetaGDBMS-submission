//! Cypher AST to framework logical algebra.
//!
//! `MetagdbPlanner` binds every label, relationship type, and property key
//! through `GraphSchema` (the schemaless registry resolves unknown names on
//! first use, so an unknown relationship type matches nothing rather than
//! failing) and lowers the clause sequence into a left-deep `LogicalOp` tree.
//!
//! Framework operators carry the relational core: `VertexScan`, `Expand`,
//! `Filter`, `Project`, `Aggregate`, `Sort`, `Limit`, `Join`, `Union`,
//! `GraphProjection`, and `CallAlgorithm`.  Constructs the IR cannot express
//! travel as the named extension payloads in [`super::logical_ext`].
//!
//! Names are resolved, never guessed from query text: the planner never looks
//! at raw query strings, and no plan shape depends on the spelling of a
//! particular benchmark case.
//!
//! ## Algorithm calls
//!
//! `CALL <name>(<args>)` becomes `GraphProjection -> CallAlgorithm` using the
//! registered name verbatim.  Because YIELD columns must be typed at plan time,
//! the planner carries the candidate's own algorithm signatures (parameter
//! order and result columns); the optimizer additionally verifies the name is
//! actually present in the runtime's algorithm registry.

use std::collections::BTreeSet;

use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
use metagdb_core::common::property::PropertyValue;
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l4_compute::algebra::{
    AlgorithmArgs, ExpandDirection, JoinKind, LogicalOp, LogicalPlan, ProjectionDirection,
    ProjectionSpec, SortOrder,
};
use metagdb_core::l4_compute::engine::{LanguageAst, Planner};
use metagdb_core::l4_compute::expression::{AggregateExpr, Expr};
use metagdb_core::l5_interaction::QueryLanguage;
use metagdb_core::{Error, Result};

use crate::language::ast::*;
use crate::language::CypherAst;

use super::logical_ext::{
    MergeSpec, MutateSpec, UnwindSpec, VarExpandSpec, EXT_DISTINCT, EXT_MERGE, EXT_MUTATE,
    EXT_UNWIND, EXT_VAR_EXPAND,
};
use super::physical_ops::{
    group_key_name, WriteOp,
};

/// Planner stage: Cypher AST to logical algebra.
#[derive(Debug, Default, Clone, Copy)]
pub struct MetagdbPlanner;

impl Planner for MetagdbPlanner {
    fn plan(&self, ast: Box<dyn LanguageAst>, schema: &dyn GraphSchema) -> Result<LogicalPlan> {
        if ast.language() != QueryLanguage::Cypher {
            return Err(Error::Plan(format!(
                "MetagdbPlanner handles Cypher only, got {:?}",
                ast.language()
            )));
        }
        let ast = ast
            .as_any()
            .downcast_ref::<CypherAst>()
            .ok_or_else(|| Error::Plan("the AST is not a CypherAst".into()))?;
        let mut binder = Binder::new(schema);
        binder.plan_ast(ast)
    }
}

/// Bound variable names available in the current clause sequence.
#[derive(Debug, Default, Clone)]
struct Scope {
    names: BTreeSet<String>,
}

impl Scope {
    fn contains(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    fn bind(&mut self, name: &str) {
        self.names.insert(name.to_string());
    }

    fn replace(&mut self, names: Vec<String>) {
        self.names = names.into_iter().collect();
    }
}

struct Binder<'a> {
    schema: &'a dyn GraphSchema,
    anonymous: usize,
    /// When set, `lower_expr` records aggregate calls here and yields a
    /// reference to their result, which lets an aggregate appear anywhere in a
    /// projected expression (`size(collect(n.name))`, `collect(n.age)[0]`).
    aggregates: Option<Vec<(AggregateExpr, String)>>,
}

impl<'a> Binder<'a> {
    fn new(schema: &'a dyn GraphSchema) -> Self {
        Self {
            schema,
            anonymous: 0,
            aggregates: None,
        }
    }

    fn fresh(&mut self) -> String {
        self.anonymous += 1;
        format!("  anon{}", self.anonymous)
    }

    /// Resolve a label through the bound schema, registering it when the
    /// schemaless registry has not seen the name before.
    fn label(&self, name: &str) -> LabelId {
        self.schema
            .label_by_name(name)
            .unwrap_or_else(|| self.registry().resolve_vertex_label(name))
    }

    fn edge_type(&self, name: &str) -> EdgeTypeId {
        self.schema
            .edge_type_by_name(name)
            .unwrap_or_else(|| self.registry().resolve_edge_type(name))
    }

    fn property_key(&self, name: &str) -> PropertyKeyId {
        self.schema
            .property_key_by_name(name)
            .unwrap_or_else(|| self.registry().resolve_property_key(name))
    }

    fn registry(&self) -> crate::schema::SchemaRegistry {
        crate::schema::SchemaRegistry::new()
    }

    fn plan_ast(&mut self, ast: &CypherAst) -> Result<LogicalPlan> {
        match &ast.body {
            AstBody::Union { all, branches } => {
                let mut ops = Vec::with_capacity(branches.len());
                for branch in branches {
                    let (op, _scope) = self.plan_clauses(branch)?;
                    ops.push(op);
                }
                if ops.len() == 1 {
                    return Ok(LogicalPlan {
                        root: ops.remove(0),
                    });
                }
                Ok(LogicalPlan {
                    root: LogicalOp::Union {
                        all: *all,
                        inputs: ops,
                    },
                })
            }
            AstBody::Call(call) => {
                let mut scope = Scope::default();
                let root = self.plan_call(LogicalOp::SingleRow, &mut scope, call)?;
                Ok(LogicalPlan { root })
            }
        }
    }

    /// Lower a clause sequence, returning the final operator and its scope.
    fn plan_clauses(&mut self, clauses: &[Clause]) -> Result<(LogicalOp, Scope)> {
        let mut op = LogicalOp::SingleRow;
        let mut scope = Scope::default();
        let mut has_projection = false;
        for clause in clauses {
            op = self.plan_clause(op, &mut scope, clause, &mut has_projection)?;
        }
        if !has_projection {
            // Cypher permits a statement without RETURN; the result is a
            // mutation summary, which this candidate reports as zero columns.
            op = LogicalOp::Project {
                items: Vec::new(),
                input: Box::new(op),
            };
        }
        Ok((op, scope))
    }

    fn plan_clause(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        clause: &Clause,
        has_projection: &mut bool,
    ) -> Result<LogicalOp> {
        match clause {
            Clause::Match {
                optional: _,
                pattern,
                predicate,
            } => {
                // The predicate is split into conjuncts and each one is applied
                // as soon as the variables it names are bound, so
                // `MATCH (a)-[e]->(b) WHERE id(a) = 1` filters before it
                // expands rather than after.
                let mut pending: Vec<AstExpr> = predicate
                    .as_ref()
                    .map(|predicate| conjuncts(predicate))
                    .unwrap_or_default();
                let mut op = self.plan_pattern(op, scope, pattern, &mut pending)?;
                for leftover in pending {
                    op = LogicalOp::Filter {
                        predicate: self.lower_expr(scope, &leftover)?,
                        input: Box::new(op),
                    };
                }
                Ok(op)
            }
            Clause::Unwind { expr, alias } => {
                let list = self.lower_expr(scope, expr)?;
                scope.bind(alias);
                Ok(LogicalOp::ExtensionOp {
                    name: EXT_UNWIND.into(),
                    payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(UnwindSpec {
                        list,
                        alias: alias.clone(),
                    }),
                    input: Box::new(op),
                })
            }
            Clause::With { body, predicate } => {
                let op = self.plan_projection(op, scope, body, predicate.as_ref(), true)?;
                *has_projection = true;
                Ok(op)
            }
            Clause::Return(body) => {
                let op = self.plan_projection(op, scope, body, None, false)?;
                *has_projection = true;
                Ok(op)
            }
            Clause::Create(pattern) => self.plan_create(op, scope, pattern),
            Clause::Merge { pattern, actions } => self.plan_merge(op, scope, pattern, actions),
            Clause::Set(items) => self.plan_set(op, scope, items),
            Clause::Remove(items) => self.plan_remove(op, scope, items),
            Clause::Delete { detach, targets } => self.plan_delete(op, scope, *detach, targets),
            Clause::Call(call) => self.plan_call(op, scope, call),
        }
    }

    // -- patterns ---------------------------------------------------------

    fn plan_pattern(
        &mut self,
        mut op: LogicalOp,
        scope: &mut Scope,
        pattern: &Pattern,
        pending: &mut Vec<AstExpr>,
    ) -> Result<LogicalOp> {
        for part in &pattern.parts {
            op = self.plan_pattern_part(op, scope, part, pending)?;
        }
        Ok(op)
    }

    fn plan_pattern_part(
        &mut self,
        mut op: LogicalOp,
        scope: &mut Scope,
        part: &PatternPart,
        pending: &mut Vec<AstExpr>,
    ) -> Result<LogicalOp> {
        let element = &part.element;
        let mut filters: Vec<Expr> = Vec::new();

        let start_var = match &element.start.variable {
            Some(name) => name.clone(),
            None => self.fresh(),
        };
        let scanned = !scope.contains(&start_var);
        if scanned {
            op = self.scan_start(op, &start_var, &element.start)?;
            scope.bind(&start_var);
        }
        self.constrain_node(scope, &start_var, &element.start, &mut filters, scanned)?;
        self.take_bound_predicates(scope, pending, &mut filters)?;
        op = apply_filters(&mut filters, op);

        // A named path needs the traversed relationship list, which only the
        // variable-length traversal carries. One chain binds the path directly;
        // several chains each bind a piece, and the clause finally assembles
        // them into the path the query names.
        let mut path_pieces: Vec<String> = Vec::new();
        if let Some(path_variable) = &part.variable {
            let mut current = start_var.clone();
            let single_chain = element.chains.len() == 1;
            for (relationship, node) in &element.chains {
                // A single chain binds the query's own path variable; several
                // chains bind pieces that are assembled afterwards.
                let piece = if single_chain {
                    path_variable.clone()
                } else {
                    self.fresh()
                };
                let (expanded, target) = self.plan_variable_expand(
                    op,
                    scope,
                    &current,
                    relationship,
                    node,
                    Some(piece.clone()),
                    &mut filters,
                )?;
                op = expanded;
                path_pieces.push(piece);
                current = target;
                self.take_bound_predicates(scope, pending, &mut filters)?;
                op = apply_filters(&mut filters, op);
            }
            if path_pieces.is_empty() {
                return Err(Error::Unsupported(
                    "a named path needs at least one relationship".into(),
                ));
            }
            scope.bind(path_variable);
        } else {
            // Each chain extends the row bound by the previous step, so the
            // source variable advances along the element.
            let mut current = start_var;
            for (relationship, node) in &element.chains {
                if relationship.range.is_some() {
                    let (expanded, target) = self.plan_variable_expand(
                        op,
                        scope,
                        &current,
                        relationship,
                        node,
                        None,
                        &mut filters,
                    )?;
                    op = expanded;
                    current = target;
                    self.take_bound_predicates(scope, pending, &mut filters)?;
                    op = apply_filters(&mut filters, op);
                    continue;
                }
                let target = match &node.variable {
                    Some(name) => name.clone(),
                    None => self.fresh(),
                };
                if let Some(edge) = &relationship.variable {
                    if scope.contains(edge) {
                        return Err(Error::Unsupported(format!(
                            "relationship variable '{edge}' is already bound"
                        )));
                    }
                }
                op = LogicalOp::Expand {
                    from: current.clone(),
                    to: target.clone(),
                    edge: relationship.variable.clone(),
                    direction: expand_direction(relationship.direction),
                    edge_types: self.edge_types(&relationship.types),
                    input: Box::new(op),
                };
                scope.bind(&target);
                if let Some(edge) = &relationship.variable {
                    scope.bind(edge);
                }
                self.constrain_node(scope, &target, node, &mut filters, false)?;
                self.constrain_relationship(scope, relationship, &mut filters)?;
                self.take_bound_predicates(scope, pending, &mut filters)?;
                op = apply_filters(&mut filters, op);
                current = target;
            }
        }

        op = apply_filters(&mut filters, op);

        if path_pieces.len() > 1 {
            let path_variable = part
                .variable
                .clone()
                .ok_or_else(|| Error::Plan("a multi-chain path needs its variable".into()))?;
            let mut combined = Expr::VarRef {
                name: path_pieces[0].clone(),
            };
            for piece in &path_pieces[1..] {
                combined = Expr::FunctionCall {
                    name: "path_append".into(),
                    args: vec![
                        combined,
                        Expr::VarRef {
                            name: piece.clone(),
                        },
                    ],
                };
            }
            // Keep every bound name and add the assembled path.
            let mut items: Vec<(Expr, String)> = scope
                .names
                .iter()
                .filter(|name| **name != path_variable)
                .map(|name| (Expr::VarRef { name: name.clone() }, name.clone()))
                .collect();
            items.push((combined, path_variable));
            op = LogicalOp::Project {
                items,
                input: Box::new(op),
            };
        }
        Ok(op)
    }

    /// Move every predicate whose variables are all bound onto the pipeline.
    fn take_bound_predicates(
        &mut self,
        scope: &Scope,
        pending: &mut Vec<AstExpr>,
        filters: &mut Vec<Expr>,
    ) -> Result<()> {
        let mut still_pending = Vec::with_capacity(pending.len());
        for predicate in pending.drain(..) {
            if variables_bound(scope, &predicate) {
                filters.push(self.lower_expr(scope, &predicate)?);
            } else {
                still_pending.push(predicate);
            }
        }
        *pending = still_pending;
        Ok(())
    }

    fn scan_start(
        &mut self,
        op: LogicalOp,
        var: &str,
        node: &NodePattern,
    ) -> Result<LogicalOp> {
        let label = node.labels.first().map(|name| self.label(name));
        let scan = LogicalOp::VertexScan {
            var: var.to_string(),
            label,
        };
        Ok(match op {
            LogicalOp::SingleRow => scan,
            other => LogicalOp::Join {
                kind: JoinKind::Cross,
                on: Vec::new(),
                left: Box::new(other),
                right: Box::new(scan),
            },
        })
    }

    fn plan_variable_expand(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        from: &str,
        relationship: &RelationshipPattern,
        node: &NodePattern,
        path: Option<String>,
        filters: &mut Vec<Expr>,
    ) -> Result<(LogicalOp, String)> {
        let to = match &node.variable {
            Some(name) => name.clone(),
            None => self.fresh(),
        };
        if scope.contains(&to) {
            return Err(Error::Unsupported(format!(
                "variable-length traversal cannot re-bind '{to}'"
            )));
        }
        let (min, max) = relationship
            .range
            .as_ref()
            .map(Range::bounds)
            .unwrap_or((1, Some(1)));
        let spec = VarExpandSpec {
            from: from.to_string(),
            to: to.clone(),
            edge: relationship.variable.clone(),
            path: path.clone(),
            direction: expand_direction(relationship.direction),
            edge_types: self.edge_types(&relationship.types),
            min,
            max,
        };
        let result = LogicalOp::ExtensionOp {
            name: EXT_VAR_EXPAND.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(spec),
            input: Box::new(op),
        };
        scope.bind(&to);
        if let Some(edge) = &relationship.variable {
            scope.bind(edge);
        }
        if let Some(path) = &path {
            scope.bind(path);
        }
        self.constrain_node(scope, &to, node, filters, false)?;
        self.constrain_relationship(scope, relationship, filters)?;
        Ok((result, to))
    }

    fn constrain_node(
        &mut self,
        scope: &Scope,
        var: &str,
        node: &NodePattern,
        filters: &mut Vec<Expr>,
        skip_first_label: bool,
    ) -> Result<()> {
        // A scanned node already had its first label applied by `VertexScan`;
        // any further label is a predicate, which also covers a bound node.
        for (index, name) in node.labels.iter().enumerate() {
            if skip_first_label && index == 0 {
                continue;
            }
            let id = self.label(name);
            filters.push(Expr::FunctionCall {
                name: "has_label".into(),
                args: vec![
                    Expr::VarRef { name: var.into() },
                    Expr::Int(id.0 as i64),
                ],
            });
        }
        if let Some(properties) = &node.properties {
            self.constrain_properties(scope, var, properties, filters)?;
        }
        Ok(())
    }

    fn constrain_relationship(
        &mut self,
        scope: &Scope,
        relationship: &RelationshipPattern,
        filters: &mut Vec<Expr>,
    ) -> Result<()> {
        if let Some(properties) = &relationship.properties {
            match &relationship.variable {
                Some(edge) => self.constrain_properties(scope, edge, properties, filters)?,
                None => {
                    return Err(Error::Unsupported(
                        "relationship properties require a relationship variable".into(),
                    ))
                }
            }
        }
        Ok(())
    }

    fn constrain_properties(
        &mut self,
        scope: &Scope,
        var: &str,
        properties: &AstExpr,
        filters: &mut Vec<Expr>,
    ) -> Result<()> {
        match properties {
            AstExpr::Map(entries) => {
                for (key, value) in entries {
                    let key_id = self.property_key(key);
                    let subject = Expr::PropRef {
                        source: Box::new(Expr::VarRef { name: var.into() }),
                        key: key_id,
                    };
                    let value = self.lower_expr(scope, value)?;
                    filters.push(Expr::Eq(Box::new(subject), Box::new(value)));
                }
                Ok(())
            }
            // A parameter supplied as properties is compared key by key.
            AstExpr::Parameter(name) => {
                let subject = Expr::VarRef { name: var.into() };
                filters.push(Expr::FunctionCall {
                    name: "has_properties".into(),
                    args: vec![subject, Expr::String(name.clone())],
                });
                Ok(())
            }
            other => Err(Error::Unsupported(format!(
                "a pattern may only carry a map literal or a parameter, got {other:?}"
            ))),
        }
    }

    fn edge_types(&self, names: &[String]) -> Option<Vec<EdgeTypeId>> {
        if names.is_empty() {
            return None;
        }
        Some(names.iter().map(|name| self.edge_type(name)).collect())
    }

    // -- projections ------------------------------------------------------

    fn plan_projection(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        body: &ProjectionBody,
        predicate: Option<&AstExpr>,
        scoped: bool,
    ) -> Result<LogicalOp> {
        let mut group_by: Vec<Expr> = Vec::new();
        let mut aggs: Vec<(AggregateExpr, String)> = Vec::new();
        // Non-aggregate items are ordinary expressions until an aggregate forces
        // grouping; then they become the grouping keys and are addressed by the
        // aggregate's positional column names.
        let mut plan: Vec<(Expr, String, bool)> = Vec::new();
        for item in &body.items {
            let alias = item
                .alias
                .clone()
                .unwrap_or_else(|| expr_name(&item.expr));
            if contains_aggregate(&item.expr) {
                let (rewritten, collected) = self.lower_with_aggregates(scope, &item.expr)?;
                for (aggregate, name) in collected {
                    if !aggs.iter().any(|(_, existing)| existing == &name) {
                        aggs.push((aggregate, name));
                    }
                }
                plan.push((rewritten, alias, true));
            } else {
                let lowered = self.lower_expr(scope, &item.expr)?;
                group_by.push(lowered.clone());
                plan.push((lowered, alias, false));
            }
        }

        let mut items: Vec<(Expr, String)> = Vec::with_capacity(plan.len());
        let mut key_index = 0usize;
        for (expr, alias, is_aggregate) in plan {
            if !aggs.is_empty() && !is_aggregate {
                items.push((Expr::VarRef {
                    name: group_key_name(key_index),
                }, alias));
                key_index += 1;
            } else {
                items.push((expr, alias));
            }
        }

        // `ORDER BY` may name either an input variable (`RETURN n.name ORDER BY
        // n.age`) or a projected alias (`RETURN n.name AS nm ORDER BY nm`).
        // When every key is resolvable against the input row, the sort runs
        // before the projection, which keeps both spellings working.
        let sort_on_input = !body.order.is_empty()
            && body
                .order
                .iter()
                .all(|key| variables_bound(scope, &key.expr));
        let mut result = op;
        if sort_on_input {
            result = LogicalOp::Sort {
                keys: self.lower_sort_keys(scope, &body.order)?,
                input: Box::new(result),
            };
        }
        if !aggs.is_empty() {
            result = LogicalOp::Aggregate {
                group_by,
                aggs,
                input: Box::new(result),
            };
        }
        result = LogicalOp::Project {
            items: items.clone(),
            input: Box::new(result),
        };
        if body.distinct {
            result = LogicalOp::ExtensionOp {
                name: EXT_DISTINCT.into(),
                payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(()),
                input: Box::new(result),
            };
        }
        // After the projection the only names in scope are its aliases; a WITH
        // predicate and an ORDER BY key written against them must resolve there.
        let aliases = Scope {
            names: items.iter().map(|(_, alias)| alias.clone()).collect(),
        };
        if let Some(predicate) = predicate {
            result = LogicalOp::Filter {
                predicate: self.lower_expr(&aliases, predicate)?,
                input: Box::new(result),
            };
        }
        if !sort_on_input && !body.order.is_empty() {
            result = LogicalOp::Sort {
                keys: self.lower_sort_keys(&aliases, &body.order)?,
                input: Box::new(result),
            };
        }
        if body.skip.is_some() || body.limit.is_some() {
            let limit = match &body.limit {
                Some(expr) => self.constant_usize(expr, "LIMIT")?,
                None => usize::MAX,
            };
            let offset = match &body.skip {
                Some(expr) => self.constant_usize(expr, "SKIP")?,
                None => 0,
            };
            result = LogicalOp::Limit {
                limit,
                offset,
                input: Box::new(result),
            };
        }

        if scoped {
            scope.replace(items.into_iter().map(|(_, alias)| alias).collect());
        }
        Ok(result)
    }

    fn lower_sort_keys(
        &mut self,
        scope: &Scope,
        order: &[SortItem],
    ) -> Result<Vec<(Expr, SortOrder)>> {
        let mut keys = Vec::with_capacity(order.len());
        for key in order {
            // A key may name an alias (`ORDER BY degree`) or repeat an input
            // expression that the projection already carries under its derived
            // name (`ORDER BY a.name` beside `RETURN a.name`).
            let lowered = match self.lower_expr(scope, &key.expr) {
                Ok(expr) => expr,
                Err(error) => {
                    let derived = expr_name(&key.expr);
                    if scope.contains(&derived) {
                        Expr::VarRef { name: derived }
                    } else {
                        return Err(error);
                    }
                }
            };
            keys.push((
                lowered,
                if key.descending {
                    SortOrder::Desc
                } else {
                    SortOrder::Asc
                },
            ));
        }
        Ok(keys)
    }

    fn constant_usize(&self, expr: &AstExpr, what: &str) -> Result<usize> {
        match expr {
            AstExpr::Int(value) if *value >= 0 => Ok(*value as usize),
            other => Err(Error::Unsupported(format!(
                "{what} must be a non-negative integer literal, got {other:?}"
            ))),
        }
    }

    /// Record an aggregate call and return a reference to its result.
    fn collect_aggregate(
        &mut self,
        scope: &Scope,
        name: &str,
        distinct: bool,
        args: &[AstExpr],
    ) -> Result<Expr> {
        if self.aggregates.is_none() {
            return Err(Error::Plan(format!(
                "aggregate {name}() is only allowed in a projected expression"
            )));
        }
        let inner = match args.len() {
            0 => return Err(Error::Plan(format!("{name}() needs an argument"))),
            1 => self.lower_expr(scope, &args[0])?,
            _ => {
                return Err(Error::Unsupported(format!(
                    "{name}() with {} arguments is outside the requested scope",
                    args.len()
                )))
            }
        };
        let aggregate = match (name.to_ascii_lowercase().as_str(), distinct) {
            ("count", false) => AggregateExpr::Count(inner),
            ("count", true) => AggregateExpr::CountDistinct(inner),
            ("sum", false) => AggregateExpr::Sum(inner),
            ("sum", true) => AggregateExpr::SumDistinct(inner),
            ("avg", false) => AggregateExpr::Avg(inner),
            ("min", false) => AggregateExpr::Min(inner),
            ("max", false) => AggregateExpr::Max(inner),
            ("collect", false) => AggregateExpr::Collect(inner),
            ("collect", true) => AggregateExpr::CollectDistinct(inner),
            (_, true) => {
                return Err(Error::Unsupported(format!(
                    "DISTINCT inside {name}() is outside the requested scope"
                )))
            }
            (other, false) => return Err(Error::Plan(format!("{other}() is not an aggregate"))),
        };
        let rendered = expr_name(&AstExpr::Function {
            name: name.to_string(),
            distinct,
            args: args.to_vec(),
        });
        let sink = self.aggregates.as_mut().expect("checked above");
        if !sink.iter().any(|(_, existing)| existing == &rendered) {
            sink.push((aggregate, rendered.clone()));
        }
        Ok(Expr::VarRef { name: rendered })
    }

    /// Lower a projected expression, collecting the aggregates it contains.
    fn lower_with_aggregates(
        &mut self,
        scope: &Scope,
        expr: &AstExpr,
    ) -> Result<(Expr, Vec<(AggregateExpr, String)>)> {
        self.aggregates = Some(Vec::new());
        let lowered = self.lower_expr(scope, expr);
        let collected = self.aggregates.take().unwrap_or_default();
        Ok((lowered?, collected))
    }

    // -- DML --------------------------------------------------------------

    fn plan_create(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        pattern: &Pattern,
    ) -> Result<LogicalOp> {
        let mut writes = Vec::new();
        for part in &pattern.parts {
            self.write_element(scope, &part.element, &mut writes)?;
        }
        Ok(LogicalOp::ExtensionOp {
            name: EXT_MUTATE.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(MutateSpec {
                writes,
                projection: Vec::new(),
            }),
            input: Box::new(op),
        })
    }

    fn write_element(
        &mut self,
        scope: &mut Scope,
        element: &PatternElement,
        writes: &mut Vec<WriteOp>,
    ) -> Result<()> {
        // A pattern position whose variable is already bound refers to an
        // existing element: only unbound positions are created.
        // A fresh name is only bound once the position is known to be new, so
        // an anonymous pattern position is created rather than mistaken for an
        // already-bound variable.
        let start_var = match &element.start.variable {
            Some(name) => name.clone(),
            None => self.fresh(),
        };
        if !scope.contains(&start_var) {
            scope.bind(&start_var);
            writes.push(WriteOp::CreateVertex {
                var: start_var.clone(),
                labels: element
                    .start
                    .labels
                    .iter()
                    .map(|name| self.label(name))
                    .collect(),
                properties: self.write_properties(scope, &element.start.properties)?,
            });
        } else if !element.start.labels.is_empty() || element.start.properties.is_some() {
            return Err(Error::Unsupported(
                "a CREATE or MERGE pattern cannot add labels or properties to a bound vertex".into(),
            ));
        }

        let mut previous = start_var;
        for (relationship, node) in &element.chains {
            let target = match &node.variable {
                Some(name) => name.clone(),
                None => self.fresh(),
            };
            if !scope.contains(&target) {
                scope.bind(&target);
                writes.push(WriteOp::CreateVertex {
                    var: target.clone(),
                    labels: node.labels.iter().map(|name| self.label(name)).collect(),
                    properties: self.write_properties(scope, &node.properties)?,
                });
            } else if !node.labels.is_empty() || node.properties.is_some() {
                return Err(Error::Unsupported(
                    "a CREATE or MERGE pattern cannot add labels or properties to a bound vertex"
                        .into(),
                ));
            }
            let (from, to, reversed) = match relationship.direction {
                Direction::Outgoing => (previous.clone(), target.clone(), false),
                Direction::Incoming => (target.clone(), previous.clone(), true),
                Direction::Undirected => {
                    return Err(Error::Unsupported(
                        "CREATE and MERGE require a directed relationship".into(),
                    ))
                }
            };
            let etype = relationship
                .types
                .first()
                .map(|name| self.edge_type(name))
                .ok_or_else(|| Error::Plan("CREATE requires a relationship type".into()))?;
            writes.push(WriteOp::CreateEdge {
                from,
                to,
                etype,
                properties: self.write_properties(scope, &relationship.properties)?,
                var: relationship.variable.clone(),
                reversed,
            });
            previous = target;
        }
        Ok(())
    }

    fn write_properties(
        &mut self,
        scope: &Scope,
        properties: &Option<Box<AstExpr>>,
    ) -> Result<Vec<(PropertyKeyId, Expr)>> {
        let mut out = Vec::new();
        if let Some(properties) = properties {
            match properties.as_ref() {
                AstExpr::Map(entries) => {
                    for (key, value) in entries {
                        out.push((self.property_key(key), self.lower_expr(scope, value)?));
                    }
                }
                other => {
                    return Err(Error::Unsupported(format!(
                        "CREATE requires a map literal for properties, got {other:?}"
                    )))
                }
            }
        }
        Ok(out)
    }

    fn plan_merge(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        pattern: &PatternPart,
        actions: &[MergeAction],
    ) -> Result<LogicalOp> {
        let start = pattern
            .element
            .start
            .variable
            .clone()
            .ok_or_else(|| Error::Unsupported("MERGE needs a variable on its start node".into()))?;

        // The probe is the same pattern matched against the current row.
        let mut probe = op.clone();
        let mut probe_scope = scope.clone();
        probe = self.plan_pattern_part(
            probe,
            &mut probe_scope,
            &PatternPart {
                variable: None,
                element: pattern.element.clone(),
            },
            // A MERGE probe carries no WHERE predicate of its own.
            &mut Vec::new(),
        )?;

        let mut create = Vec::new();
        let mut create_scope = scope.clone();
        self.write_element(&mut create_scope, &pattern.element, &mut create)?;

        let mut lowered_actions = Vec::new();
        for action in actions {
            let mut writes = Vec::new();
            for item in &action.items {
                writes.push(self.lower_set_item(scope, item)?);
            }
            lowered_actions.push((action.on_match, writes));
        }
        scope.bind(&start);
        Ok(LogicalOp::ExtensionOp {
            name: EXT_MERGE.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(MergeSpec {
                probe: Box::new(probe),
                create,
                actions: lowered_actions,
            }),
            input: Box::new(op),
        })
    }

    fn plan_set(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        items: &[SetItem],
    ) -> Result<LogicalOp> {
        let mut writes = Vec::new();
        for item in items {
            writes.push(self.lower_set_item(scope, item)?);
        }
        Ok(LogicalOp::ExtensionOp {
            name: EXT_MUTATE.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(MutateSpec {
                writes,
                projection: Vec::new(),
            }),
            input: Box::new(op),
        })
    }

    fn lower_set_item(&mut self, scope: &Scope, item: &SetItem) -> Result<WriteOp> {
        match item {
            SetItem::Property { target, value } => {
                let (var, key) = split_property(target)?;
                Ok(WriteOp::SetProperty {
                    target: Expr::VarRef { name: var },
                    key: self.property_key(&key),
                    value: self.lower_expr(scope, value)?,
                })
            }
            SetItem::Replace { variable, value } => Ok(WriteOp::SetProperty {
                target: Expr::VarRef {
                    name: variable.clone(),
                },
                key: self.property_key("*"),
                value: self.lower_expr(scope, value)?,
            }),
            SetItem::Merge { variable, value } => Ok(WriteOp::SetProperty {
                target: Expr::VarRef {
                    name: variable.clone(),
                },
                key: self.property_key("+"),
                value: self.lower_expr(scope, value)?,
            }),
            SetItem::Labels { variable, labels } => Ok(WriteOp::SetLabels {
                target: Expr::VarRef {
                    name: variable.clone(),
                },
                labels: labels.iter().map(|name| self.label(name)).collect(),
            }),
        }
    }

    fn plan_remove(
        &mut self,
        op: LogicalOp,
        _scope: &mut Scope,
        items: &[RemoveItem],
    ) -> Result<LogicalOp> {
        let mut writes = Vec::new();
        for item in items {
            writes.push(match item {
                RemoveItem::Property(target) => {
                    let (var, key) = split_property(target)?;
                    WriteOp::RemoveProperty {
                        target: Expr::VarRef { name: var },
                        key: self.property_key(&key),
                    }
                }
                RemoveItem::Labels { variable, labels } => WriteOp::RemoveLabels {
                    target: Expr::VarRef {
                        name: variable.clone(),
                    },
                    labels: labels.iter().map(|name| self.label(name)).collect(),
                },
            });
        }
        Ok(LogicalOp::ExtensionOp {
            name: EXT_MUTATE.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(MutateSpec {
                writes,
                projection: Vec::new(),
            }),
            input: Box::new(op),
        })
    }

    fn plan_delete(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        detach: bool,
        targets: &[AstExpr],
    ) -> Result<LogicalOp> {
        let mut writes = Vec::new();
        for target in targets {
            writes.push(WriteOp::Delete {
                target: self.lower_expr(scope, target)?,
                detach,
            });
        }
        Ok(LogicalOp::ExtensionOp {
            name: EXT_MUTATE.into(),
            payload: metagdb_core::l4_compute::algebra::OpaquePayload::new(MutateSpec {
                writes,
                projection: Vec::new(),
            }),
            input: Box::new(op),
        })
    }

    // -- calls ------------------------------------------------------------

    fn plan_call(
        &mut self,
        op: LogicalOp,
        scope: &mut Scope,
        call: &ProcedureCall,
    ) -> Result<LogicalOp> {
        let signature = algorithm_signature(&call.name).ok_or_else(|| {
            Error::Unsupported(format!(
                "procedure '{}' is not implemented by this candidate",
                call.name
            ))
        })?;
        if call.args.len() > signature.parameters.len() {
            return Err(Error::Plan(format!(
                "{}() takes at most {} arguments, got {}",
                call.name,
                signature.parameters.len(),
                call.args.len()
            )));
        }
        let mut args = AlgorithmArgs::default();
        for (index, arg) in call.args.iter().enumerate() {
            let value = match arg {
                AstExpr::Int(value) => PropertyValue::Int(*value),
                AstExpr::Float(value) => PropertyValue::Float(*value),
                AstExpr::Bool(value) => PropertyValue::Bool(*value),
                AstExpr::String(value) => PropertyValue::Text(value.clone()),
                AstExpr::Null => PropertyValue::Null,
                other => {
                    return Err(Error::Unsupported(format!(
                        "{}() arguments must be scalars, got {other:?}",
                        call.name
                    )))
                }
            };
            args.params
                .insert(signature.parameters[index].to_string(), value);
        }

        let projection = LogicalOp::GraphProjection {
            spec: ProjectionSpec {
                vertex_labels: None,
                edge_types: None,
                direction: ProjectionDirection::Outgoing,
                vertex_properties: None,
            },
        };
        let call_op = LogicalOp::CallAlgorithm {
            name: call.name.clone(),
            args,
            input: Box::new(projection),
        };

        // Without YIELD the columns keep the algorithm's own names; with YIELD
        // the requested columns are renamed, and `YIELD *` keeps them all.
        let mut items = Vec::new();
        match &call.yields {
            None => {
                for column in signature.columns {
                    scope.bind(column);
                }
                return Ok(LogicalOp::Join {
                    kind: JoinKind::Cross,
                    on: Vec::new(),
                    left: Box::new(op),
                    right: Box::new(call_op),
                });
            }
            Some(_) => {
                let yields = match &call.yields {
                    Some(yields) => yields,
                    None => unreachable!("the None arm returned above"),
                };
                if yields.all {
                    for column in signature.columns {
                        items.push((
                            Expr::VarRef {
                                name: (*column).into(),
                            },
                            (*column).to_string(),
                        ));
                    }
                } else {
                    for item in &yields.items {
                        let field = item.field.clone().unwrap_or_else(|| item.alias.clone());
                        if !signature.columns.contains(&field.as_str()) {
                            return Err(Error::Plan(format!(
                                "{}() does not yield a column named '{field}'",
                                call.name
                            )));
                        }
                        items.push((Expr::VarRef { name: field }, item.alias.clone()));
                    }
                }
            }
        }
        let mut result = LogicalOp::Project {
            items: items.clone(),
            input: Box::new(call_op),
        };
        result = LogicalOp::Join {
            kind: JoinKind::Cross,
            on: Vec::new(),
            left: Box::new(op),
            right: Box::new(result),
        };
        for (_, alias) in &items {
            scope.bind(alias);
        }
        if let Some(predicate) = call.yields.as_ref().and_then(|yields| yields.predicate.as_ref()) {
            let names = Scope {
                names: items.iter().map(|(_, alias)| alias.clone()).collect(),
            };
            result = LogicalOp::Filter {
                predicate: self.lower_expr(&names, predicate)?,
                input: Box::new(result),
            };
        }
        Ok(result)
    }

    // -- expressions ------------------------------------------------------

    fn lower_expr(&mut self, scope: &Scope, expr: &AstExpr) -> Result<Expr> {
        Ok(match expr {
            AstExpr::Int(value) => Expr::Int(*value),
            AstExpr::Float(value) => Expr::Float(*value),
            AstExpr::String(value) => Expr::String(value.clone()),
            AstExpr::Bool(value) => Expr::Bool(*value),
            AstExpr::Null => Expr::Null,
            AstExpr::List(items) => {
                let mut lowered = Vec::with_capacity(items.len());
                for item in items {
                    lowered.push(self.lower_expr(scope, item)?);
                }
                Expr::List(lowered)
            }
            AstExpr::Map(entries) => {
                let mut args = Vec::with_capacity(entries.len() * 2);
                for (key, value) in entries {
                    args.push(Expr::String(key.clone()));
                    args.push(self.lower_expr(scope, value)?);
                }
                Expr::FunctionCall {
                    name: "map".into(),
                    args,
                }
            }
            AstExpr::Variable(name) => {
                if !scope.contains(name) {
                    return Err(Error::Plan(format!("variable '{name}' is not bound here")));
                }
                Expr::VarRef { name: name.clone() }
            }
            AstExpr::Parameter(name) => Expr::FunctionCall {
                name: "parameter".into(),
                args: vec![Expr::String(name.clone())],
            },
            AstExpr::Property { base, key } => {
                let source = self.lower_expr(scope, base)?;
                Expr::PropRef {
                    source: Box::new(source),
                    key: self.property_key(key),
                }
            }
            AstExpr::Labels { base, labels } => {
                let source = self.lower_expr(scope, base)?;
                let mut args = vec![source];
                for label in labels {
                    args.push(Expr::Int(self.label(label).0 as i64));
                }
                Expr::FunctionCall {
                    name: "has_label".into(),
                    args,
                }
            }
            AstExpr::Unary { op, operand } => {
                let operand = self.lower_expr(scope, operand)?;
                match op {
                    UnaryOp::Not => Expr::Not(Box::new(operand)),
                    UnaryOp::Negate => Expr::Neg(Box::new(operand)),
                    UnaryOp::Plus => operand,
                }
            }
            AstExpr::Binary { op, left, right } => binary_expr(
                *op,
                self.lower_expr(scope, left)?,
                self.lower_expr(scope, right)?,
            ),
            AstExpr::IsNull { operand, negated } => {
                let operand = Box::new(self.lower_expr(scope, operand)?);
                if *negated {
                    Expr::IsNotNull(operand)
                } else {
                    Expr::IsNull(operand)
                }
            }
            AstExpr::ListIndex { list, index } => Expr::ListIndex {
                list: Box::new(self.lower_expr(scope, list)?),
                index: Box::new(self.lower_expr(scope, index)?),
            },
            AstExpr::ListSlice { list, from, to } => {
                let mut args = vec![self.lower_expr(scope, list)?];
                args.push(match from {
                    Some(expr) => self.lower_expr(scope, expr)?,
                    None => Expr::Null,
                });
                args.push(match to {
                    Some(expr) => self.lower_expr(scope, expr)?,
                    None => Expr::Null,
                });
                Expr::FunctionCall {
                    name: "slice".into(),
                    args,
                }
            }
            AstExpr::In { item, list } => Expr::FunctionCall {
                name: "in".into(),
                args: vec![self.lower_expr(scope, item)?, self.lower_expr(scope, list)?],
            },
            AstExpr::StringOp { op, left, right } => {
                let left = Box::new(self.lower_expr(scope, left)?);
                let right = Box::new(self.lower_expr(scope, right)?);
                match op {
                    StringOp::StartsWith => Expr::StartsWith(left, right),
                    StringOp::EndsWith => Expr::EndsWith(left, right),
                    StringOp::Contains => Expr::Contains(left, right),
                }
            }
            AstExpr::Case {
                operand,
                alternatives,
                fallback,
            } => {
                let mut lowered = Vec::with_capacity(alternatives.len());
                for (when, then) in alternatives {
                    let condition = match operand {
                        Some(operand) => Expr::Eq(
                            Box::new(self.lower_expr(scope, operand)?),
                            Box::new(self.lower_expr(scope, when)?),
                        ),
                        None => self.lower_expr(scope, when)?,
                    };
                    lowered.push((condition, self.lower_expr(scope, then)?));
                }
                Expr::Case {
                    cases: lowered,
                    else_branch: match fallback {
                        Some(expr) => Some(Box::new(self.lower_expr(scope, expr)?)),
                        None => None,
                    },
                }
            }
            AstExpr::Function {
                name,
                distinct,
                args,
            } => {
                if is_aggregate_function(name) {
                    return self.collect_aggregate(scope, name, *distinct, args);
                }
                if *distinct {
                    return Err(Error::Unsupported(format!(
                        "DISTINCT inside {name}() is outside the requested scope"
                    )));
                }
                let mut lowered = Vec::with_capacity(args.len());
                for arg in args {
                    lowered.push(self.lower_expr(scope, arg)?);
                }
                Expr::FunctionCall {
                    name: name.to_ascii_lowercase(),
                    args: lowered,
                }
            }
            AstExpr::CountAll => {
                let Some(sink) = self.aggregates.as_mut() else {
                    return Err(Error::Plan(
                        "count(*) is only allowed in a projected expression".into(),
                    ));
                };
                let name = "count(*)".to_string();
                if !sink.iter().any(|(_, existing)| existing == &name) {
                    sink.push((AggregateExpr::CountAll, name.clone()));
                }
                return Ok(Expr::VarRef { name });
            }
            AstExpr::ListComprehension {
                variable,
                list,
                filter,
                projection,
            } => {
                let mut args = vec![
                    Expr::String(variable.clone()),
                    self.lower_expr(scope, list)?,
                ];
                let mut inner = scope.clone();
                inner.bind(variable);
                args.push(match filter {
                    Some(expr) => self.lower_expr(&inner, expr)?,
                    None => Expr::Bool(true),
                });
                args.push(match projection {
                    Some(expr) => self.lower_expr(&inner, expr)?,
                    None => Expr::VarRef {
                        name: variable.clone(),
                    },
                });
                Expr::FunctionCall {
                    name: "list_comprehension".into(),
                    args,
                }
            }
            AstExpr::Quantifier {
                kind,
                variable,
                list,
                filter,
            } => {
                let mut inner = scope.clone();
                inner.bind(variable);
                Expr::FunctionCall {
                    name: match kind {
                        QuantifierKind::All => "all",
                        QuantifierKind::Any => "any",
                        QuantifierKind::None => "none",
                        QuantifierKind::Single => "single",
                    }
                    .into(),
                    args: vec![
                        Expr::String(variable.clone()),
                        self.lower_expr(scope, list)?,
                        self.lower_expr(&inner, filter)?,
                    ],
                }
            }
            AstExpr::PatternComprehension { .. } | AstExpr::Path(_) => {
                return Err(Error::Unsupported(
                    "pattern expressions are only supported in EXISTS-style predicates".into(),
                ))
            }
        })
    }
}

fn expand_direction(direction: Direction) -> ExpandDirection {
    match direction {
        Direction::Outgoing => ExpandDirection::Outgoing,
        Direction::Incoming => ExpandDirection::Incoming,
        Direction::Undirected => ExpandDirection::Both,
    }
}

fn split_property(expr: &AstExpr) -> Result<(String, String)> {
    match expr {
        AstExpr::Property { base, key } => match base.as_ref() {
            AstExpr::Variable(name) => Ok((name.clone(), key.clone())),
            other => Err(Error::Unsupported(format!(
                "a property target must be a plain variable, got {other:?}"
            ))),
        },
        other => Err(Error::Plan(format!(
            "expected a property reference, got {other:?}"
        ))),
    }
}

/// Framework expression for one Cypher binary operator.
fn binary_expr(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    let left = Box::new(left);
    let right = Box::new(right);
    match op {
        BinaryOp::Or => Expr::Or(left, right),
        BinaryOp::Xor => Expr::FunctionCall {
            name: "xor".into(),
            args: vec![*left, *right],
        },
        BinaryOp::And => Expr::And(left, right),
        BinaryOp::Eq => Expr::Eq(left, right),
        BinaryOp::NotEq => Expr::Neq(left, right),
        BinaryOp::Less => Expr::Lt(left, right),
        BinaryOp::Greater => Expr::Gt(left, right),
        BinaryOp::LessOrEqual => Expr::Lte(left, right),
        BinaryOp::GreaterOrEqual => Expr::Gte(left, right),
        BinaryOp::Add => Expr::Add(left, right),
        BinaryOp::Subtract => Expr::Sub(left, right),
        BinaryOp::Multiply => Expr::Mul(left, right),
        BinaryOp::Divide => Expr::Div(left, right),
        BinaryOp::Modulo => Expr::Mod(left, right),
        BinaryOp::Power => Expr::FunctionCall {
            name: "power".into(),
            args: vec![*left, *right],
        },
    }
}

/// Wrap `op` in one filter per predicate, innermost first.
fn apply_filters(filters: &mut Vec<Expr>, mut op: LogicalOp) -> LogicalOp {
    for predicate in filters.drain(..) {
        op = LogicalOp::Filter {
            predicate,
            input: Box::new(op),
        };
    }
    op
}

/// Split a predicate at its top-level `AND`s, which is what makes each part
/// eligible to be applied as soon as its own variables are bound.
fn conjuncts(expr: &AstExpr) -> Vec<AstExpr> {
    match expr {
        AstExpr::Binary {
            op: BinaryOp::And,
            left,
            right,
        } => {
            let mut parts = conjuncts(left);
            parts.extend(conjuncts(right));
            parts
        }
        other => vec![other.clone()],
    }
}

/// True when every variable the expression names is bound in `scope`.
fn variables_bound(scope: &Scope, expr: &AstExpr) -> bool {
    match expr {
        AstExpr::Variable(name) => scope.contains(name),
        AstExpr::Property { base, .. }
        | AstExpr::Labels { base, .. }
        | AstExpr::Unary { operand: base, .. }
        | AstExpr::IsNull { operand: base, .. } => variables_bound(scope, base),
        AstExpr::Binary { left, right, .. } => {
            variables_bound(scope, left) && variables_bound(scope, right)
        }
        AstExpr::ListIndex { list, index } => {
            variables_bound(scope, list) && variables_bound(scope, index)
        }
        AstExpr::ListSlice { list, from, to } => {
            variables_bound(scope, list)
                && from.as_deref().map(|e| variables_bound(scope, e)).unwrap_or(true)
                && to.as_deref().map(|e| variables_bound(scope, e)).unwrap_or(true)
        }
        AstExpr::In { item, list } => variables_bound(scope, item) && variables_bound(scope, list),
        AstExpr::StringOp { left, right, .. } => {
            variables_bound(scope, left) && variables_bound(scope, right)
        }
        AstExpr::Case {
            operand,
            alternatives,
            fallback,
        } => {
            operand.as_deref().map(|e| variables_bound(scope, e)).unwrap_or(true)
                && alternatives
                    .iter()
                    .all(|(when, then)| variables_bound(scope, when) && variables_bound(scope, then))
                && fallback.as_deref().map(|e| variables_bound(scope, e)).unwrap_or(true)
        }
        AstExpr::Function { args, .. } => args.iter().all(|arg| variables_bound(scope, arg)),
        AstExpr::List(items) => items.iter().all(|item| variables_bound(scope, item)),
        AstExpr::Map(entries) => entries.iter().all(|(_, value)| variables_bound(scope, value)),
        // A comprehension or quantifier binds its own variable, so its body is
        // not required to resolve in the outer scope.
        AstExpr::ListComprehension { list, .. } => variables_bound(scope, list),
        AstExpr::Quantifier { list, .. } => variables_bound(scope, list),
        AstExpr::PatternComprehension { .. } | AstExpr::Path(_) => true,
        AstExpr::Int(_)
        | AstExpr::Float(_)
        | AstExpr::String(_)
        | AstExpr::Bool(_)
        | AstExpr::Null
        | AstExpr::Parameter(_)
        | AstExpr::CountAll => true,
    }
}

fn is_aggregate_function(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "count" | "sum" | "avg" | "min" | "max" | "collect"
    )
}

/// True when the expression contains an aggregate call anywhere.
fn contains_aggregate(expr: &AstExpr) -> bool {
    match expr {
        AstExpr::CountAll => true,
        AstExpr::Function { name, args, .. } => {
            is_aggregate_function(name) || args.iter().any(contains_aggregate)
        }
        AstExpr::Binary { left, right, .. } => {
            contains_aggregate(left) || contains_aggregate(right)
        }
        AstExpr::Unary { operand, .. }
        | AstExpr::IsNull { operand, .. }
        | AstExpr::Property { base: operand, .. }
        | AstExpr::Labels { base: operand, .. } => contains_aggregate(operand),
        AstExpr::ListIndex { list, index } => {
            contains_aggregate(list) || contains_aggregate(index)
        }
        AstExpr::ListSlice { list, from, to } => {
            contains_aggregate(list)
                || from.as_deref().map(contains_aggregate).unwrap_or(false)
                || to.as_deref().map(contains_aggregate).unwrap_or(false)
        }
        AstExpr::In { item, list } => contains_aggregate(item) || contains_aggregate(list),
        AstExpr::StringOp { left, right, .. } => {
            contains_aggregate(left) || contains_aggregate(right)
        }
        AstExpr::Case {
            operand,
            alternatives,
            fallback,
        } => {
            operand.as_deref().map(contains_aggregate).unwrap_or(false)
                || alternatives
                    .iter()
                    .any(|(when, then)| contains_aggregate(when) || contains_aggregate(then))
                || fallback.as_deref().map(contains_aggregate).unwrap_or(false)
        }
        AstExpr::List(items) => items.iter().any(contains_aggregate),
        AstExpr::Map(entries) => entries.iter().any(|(_, value)| contains_aggregate(value)),
        AstExpr::ListComprehension { list, filter, projection, .. } => {
            contains_aggregate(list)
                || filter.as_deref().map(contains_aggregate).unwrap_or(false)
                || projection.as_deref().map(contains_aggregate).unwrap_or(false)
        }
        AstExpr::Quantifier { list, filter, .. } => {
            contains_aggregate(list) || contains_aggregate(filter)
        }
        AstExpr::PatternComprehension { filter, projection, .. } => {
            filter.as_deref().map(contains_aggregate).unwrap_or(false)
                || contains_aggregate(projection)
        }
        AstExpr::Int(_)
        | AstExpr::Float(_)
        | AstExpr::String(_)
        | AstExpr::Bool(_)
        | AstExpr::Null
        | AstExpr::Variable(_)
        | AstExpr::Parameter(_)
        | AstExpr::Path(_) => false,
    }
}

/// Canonical column name for an un-aliased projection item.
fn expr_name(expr: &AstExpr) -> String {
    match expr {
        AstExpr::Int(value) => value.to_string(),
        AstExpr::Float(value) => value.to_string(),
        AstExpr::String(value) => format!("\"{value}\""),
        AstExpr::Bool(value) => value.to_string(),
        AstExpr::Null => "null".into(),
        AstExpr::Variable(name) => name.clone(),
        AstExpr::Parameter(name) => format!("${name}"),
        AstExpr::Property { base, key } => format!("{}.{}", expr_name(base), key),
        AstExpr::Labels { base, labels } => {
            format!("{}:{}", expr_name(base), labels.join(":"))
        }
        AstExpr::Unary { op, operand } => {
            let operator = match op {
                UnaryOp::Not => "NOT ",
                UnaryOp::Negate => "-",
                UnaryOp::Plus => "+",
            };
            format!("{operator}{}", expr_name(operand))
        }
        AstExpr::Binary { op, left, right } => format!(
            "{} {} {}",
            expr_name(left),
            match op {
                BinaryOp::Or => "OR",
                BinaryOp::Xor => "XOR",
                BinaryOp::And => "AND",
                BinaryOp::Eq => "=",
                BinaryOp::NotEq => "<>",
                BinaryOp::Less => "<",
                BinaryOp::Greater => ">",
                BinaryOp::LessOrEqual => "<=",
                BinaryOp::GreaterOrEqual => ">=",
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Modulo => "%",
                BinaryOp::Power => "^",
            },
            expr_name(right)
        ),
        AstExpr::IsNull { operand, negated } => format!(
            "{} IS {}NULL",
            expr_name(operand),
            if *negated { "NOT " } else { "" }
        ),
        AstExpr::ListIndex { list, index } => {
            format!("{}[{}]", expr_name(list), expr_name(index))
        }
        AstExpr::ListSlice { list, from, to } => format!(
            "{}[{}..{}]",
            expr_name(list),
            from.as_deref().map(expr_name).unwrap_or_default(),
            to.as_deref().map(expr_name).unwrap_or_default()
        ),
        AstExpr::In { item, list } => format!("{} IN {}", expr_name(item), expr_name(list)),
        AstExpr::StringOp { op, left, right } => format!(
            "{} {} {}",
            expr_name(left),
            match op {
                StringOp::StartsWith => "STARTS WITH",
                StringOp::EndsWith => "ENDS WITH",
                StringOp::Contains => "CONTAINS",
            },
            expr_name(right)
        ),
        AstExpr::Case { .. } => "CASE".into(),
        AstExpr::Function { name, args, .. } => format!(
            "{}({})",
            name,
            args.iter().map(expr_name).collect::<Vec<_>>().join(", ")
        ),
        AstExpr::CountAll => "count(*)".into(),
        AstExpr::List(items) => format!(
            "[{}]",
            items.iter().map(expr_name).collect::<Vec<_>>().join(", ")
        ),
        AstExpr::Map(_) => "{...}".into(),
        AstExpr::ListComprehension { variable, .. } => format!("[{variable} IN ...]"),
        AstExpr::PatternComprehension { .. } => "[...]".into(),
        AstExpr::Quantifier { kind, variable, .. } => format!(
            "{}({variable} IN ...)",
            match kind {
                QuantifierKind::All => "all",
                QuantifierKind::Any => "any",
                QuantifierKind::None => "none",
                QuantifierKind::Single => "single",
            }
        ),
        AstExpr::Path(_) => "(...)".into(),
    }
}

/// Positional parameters and result columns of the candidate's algorithms.
///
/// YIELD columns have to be typed at plan time, so the planner carries the
/// signatures of the algorithms this candidate implements. The optimizer
/// additionally checks the name is present in the runtime registry, so a
/// signature here never *creates* an algorithm.
pub struct AlgorithmSignature {
    pub parameters: &'static [&'static str],
    pub columns: &'static [&'static str],
}

pub fn algorithm_signature(name: &str) -> Option<AlgorithmSignature> {
    match name.to_ascii_lowercase().as_str() {
        "bfs" => Some(AlgorithmSignature {
            parameters: &["source"],
            columns: &["node", "distance"],
        }),
        "wcc" => Some(AlgorithmSignature {
            parameters: &[],
            columns: &["node", "component"],
        }),
        "pagerank" => Some(AlgorithmSignature {
            parameters: &["damping", "tolerance", "max_iterations"],
            columns: &["node", "rank"],
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::SchemaRegistry;
    use metagdb_core::l4_compute::engine::Parser;
    use metagdb_core::l4_compute::algebra::OpaquePayload;

    fn plan(source: &str) -> Result<LogicalPlan> {
        let ast = crate::language::MetagdbParser.parse(source)?;
        MetagdbPlanner.plan(ast, &SchemaRegistry::new())
    }

    /// Operator names from the root downwards, for shape assertions.
    fn shape(op: &LogicalOp, out: &mut Vec<String>) {
        match op {
            LogicalOp::SingleRow => out.push("single_row".into()),
            LogicalOp::VertexScan { .. } => out.push("vertex_scan".into()),
            LogicalOp::EdgeScan { .. } => out.push("edge_scan".into()),
            LogicalOp::Expand { input, .. } => {
                out.push("expand".into());
                shape(input, out);
            }
            LogicalOp::Filter { input, .. } => {
                out.push("filter".into());
                shape(input, out);
            }
            LogicalOp::Project { input, .. } => {
                out.push("project".into());
                shape(input, out);
            }
            LogicalOp::Aggregate { input, .. } => {
                out.push("aggregate".into());
                shape(input, out);
            }
            LogicalOp::Sort { input, .. } => {
                out.push("sort".into());
                shape(input, out);
            }
            LogicalOp::Limit { input, .. } => {
                out.push("limit".into());
                shape(input, out);
            }
            LogicalOp::Join { left, right, .. } => {
                out.push("join".into());
                shape(left, out);
                shape(right, out);
            }
            LogicalOp::Union { inputs, .. } => {
                out.push("union".into());
                for input in inputs {
                    shape(input, out);
                }
            }
            LogicalOp::GraphProjection { .. } => out.push("projection".into()),
            LogicalOp::CallAlgorithm { input, .. } => {
                out.push("call".into());
                shape(input, out);
            }
            LogicalOp::ExtensionOp { name, input, .. } => {
                out.push(name.clone());
                shape(input, out);
            }
        }
    }

    fn names(source: &str) -> Vec<String> {
        let plan = plan(source).expect("plans");
        let mut out = Vec::new();
        shape(&plan.root, &mut out);
        out
    }

    fn payload<'a, T: 'static>(op: &'a LogicalOp, expected: &str) -> &'a T {
        match op {
            LogicalOp::ExtensionOp { name, payload, .. } => {
                assert_eq!(name, expected);
                payload
                    .downcast_ref::<T>()
                    .expect("the payload has the declared type")
            }
            other => panic!("expected {expected}, got {other:?}"),
        }
    }

    #[test]
    fn a_plain_match_scans_and_projects() {
        assert_eq!(names("MATCH (n) RETURN n"), vec!["project", "vertex_scan"]);
    }

    #[test]
    fn a_label_is_bound_through_the_schema() {
        let plan = plan("MATCH (n:Person) RETURN n").expect("plans");
        let LogicalOp::Project { input, .. } = &plan.root else {
            panic!("expected a projection");
        };
        match input.as_ref() {
            LogicalOp::VertexScan { var, label } => {
                assert_eq!(var, "n");
                assert_eq!(*label, Some(SchemaRegistry::new().label_by_name("Person").unwrap()));
            }
            other => panic!("expected a vertex scan, got {other:?}"),
        }
    }

    #[test]
    fn relationship_types_and_properties_bind_to_ids() {
        let plan = plan("MATCH (a)-[r:KNOWS {since: 2015}]->(b) RETURN b").expect("plans");
        fn find_expand(op: &LogicalOp) -> Option<Option<Vec<EdgeTypeId>>> {
            match op {
                LogicalOp::Expand { input, edge_types, .. } => {
                    find_expand(input);
                    Some(edge_types.clone())
                }
                LogicalOp::Filter { input, .. }
                | LogicalOp::Project { input, .. }
                | LogicalOp::Aggregate { input, .. }
                | LogicalOp::Sort { input, .. }
                | LogicalOp::Limit { input, .. }
                | LogicalOp::ExtensionOp { input, .. } => find_expand(input),
                _ => None,
            }
        }
        let edge_types = find_expand(&plan.root).expect("an expand is planned");
        let expected = SchemaRegistry::new().edge_type_by_name("KNOWS").unwrap();
        assert_eq!(edge_types, Some(vec![expected]));
        // The relationship property predicate binds through the schema.
        let since = SchemaRegistry::new().property_key_by_name("since").unwrap();
        let rendered = format!("{:?}", plan.root);
        assert!(
            rendered.contains(&format!("key: PropertyKeyId({})", since.0)),
            "the relationship property must be bound to its key id: {rendered}"
        );
    }

    #[test]
    fn unknown_names_are_registered_because_the_design_is_schemaless() {
        // `d.schema.mode` is schemaless, so an unknown relationship type is a
        // valid new name rather than an error, and it matches nothing at run time.
        let plan = plan("MATCH (a)-[:LivesIn]->(b) RETURN count(b)").expect("plans");
        let mut out = Vec::new();
        shape(&plan.root, &mut out);
        assert!(out.contains(&"expand".to_string()));
        assert!(SchemaRegistry::new().edge_type_by_name("LivesIn").is_some());
    }

    #[test]
    fn filters_sorting_and_pagination_lower_to_their_operators() {
        assert_eq!(
            // The ORDER BY key resolves against the input row, so the sort runs
            // before the projection and survives it.
            names("MATCH (n:Person) WHERE n.age > 29 RETURN n.name ORDER BY n.name SKIP 1 LIMIT 2"),
            vec!["limit", "project", "sort", "filter", "vertex_scan"]
        );
    }

    #[test]
    fn aggregation_emits_an_aggregate_below_the_projection() {
        let names = names("MATCH (n:Person) RETURN n.age, count(n)");
        assert_eq!(names, vec!["project", "aggregate", "vertex_scan"]);

        let plan = plan("MATCH (n:Person) RETURN n.age, count(n)").expect("plans");
        let LogicalOp::Project { input, items } = &plan.root else {
            panic!("expected a projection");
        };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0].0, Expr::VarRef { .. }));
        match input.as_ref() {
            LogicalOp::Aggregate { group_by, aggs, .. } => {
                assert_eq!(group_by.len(), 1);
                assert_eq!(aggs.len(), 1);
                assert_eq!(aggs[0].1, "count(n)");
            }
            other => panic!("expected an aggregate, got {other:?}"),
        }
    }

    #[test]
    fn distinct_unwind_and_union_lower() {
        assert!(names("MATCH (n:Person) RETURN DISTINCT n.age").contains(&"cypher.distinct".to_string()));
        assert_eq!(names("UNWIND [1, 2] AS x RETURN x"), vec!["project", "cypher.unwind", "single_row"]);
        assert_eq!(
            names("RETURN 1 UNION ALL RETURN 2"),
            vec!["union", "project", "single_row", "project", "single_row"]
        );
    }

    #[test]
    fn a_variable_length_pattern_uses_the_named_extension() {
        let plan = plan("MATCH (a)-[:KNOWS*1..2]->(b) RETURN b").expect("plans");
        let mut out = Vec::new();
        shape(&plan.root, &mut out);
        assert_eq!(out, vec!["project", EXT_VAR_EXPAND, "vertex_scan"]);

        let LogicalOp::Project { input, .. } = &plan.root else {
            panic!("expected a projection");
        };
        let spec: &VarExpandSpec = payload(input, EXT_VAR_EXPAND);
        assert_eq!(spec.from, "a");
        assert_eq!(spec.to, "b");
        assert_eq!(spec.min, 1);
        assert_eq!(spec.max, Some(2));
    }

    #[test]
    fn a_named_path_binds_the_path_variable() {
        let plan = plan("MATCH p = (a {name: 'A'})-[:KNOWS*2]->(b) RETURN length(p)").expect("plans");
        let LogicalOp::Project { input, .. } = &plan.root else {
            panic!("expected a projection");
        };
        let inner = match input.as_ref() {
            LogicalOp::Filter { input, .. } => input.as_ref(),
            other => other,
        };
        let spec: &VarExpandSpec = payload(inner, EXT_VAR_EXPAND);
        assert_eq!(spec.path.as_deref(), Some("p"));
        assert_eq!(spec.min, 2);
        assert_eq!(spec.max, Some(2));
    }

    #[test]
    fn dml_clauses_produce_mutation_payloads() {
        let deleted = plan("MATCH (n:City) DETACH DELETE n RETURN count(n)").expect("plans");
        let mut out = Vec::new();
        shape(&deleted.root, &mut out);
        assert_eq!(out, vec!["project", "aggregate", EXT_MUTATE, "vertex_scan"]);

        let create = plan("CREATE (n:Person {name: 'F', age: 40}) RETURN n.name").expect("plans");
        let LogicalOp::Project { input, .. } = &create.root else {
            panic!("expected a projection");
        };
        let spec: &MutateSpec = payload(input.as_ref(), EXT_MUTATE);
        assert_eq!(spec.writes.len(), 1);
        match &spec.writes[0] {
            WriteOp::CreateVertex {
                var,
                labels,
                properties,
            } => {
                assert_eq!(var, "n");
                assert_eq!(labels.len(), 1);
                assert_eq!(properties.len(), 2);
            }
            other => panic!("expected a vertex write, got {other:?}"),
        }
    }

    #[test]
    fn merge_plans_a_probe_and_a_create_branch() {
        let plan = plan("MERGE (n:Person {name: 'F'}) ON CREATE SET n.age = 40 RETURN n.age")
            .expect("plans");
        fn find_merge(op: &LogicalOp) -> Option<&MergeSpec> {
            match op {
                LogicalOp::ExtensionOp { name, payload, input } => {
                    if name == EXT_MERGE {
                        return payload.downcast_ref::<MergeSpec>();
                    }
                    find_merge(input)
                }
                LogicalOp::Project { input, .. } | LogicalOp::Filter { input, .. } => find_merge(input),
                _ => None,
            }
        }
        let spec = find_merge(&plan.root).expect("a merge payload");
        assert_eq!(spec.create.len(), 1);
        assert_eq!(spec.actions.len(), 1);
        assert!(!spec.actions[0].0, "the action is ON CREATE");
    }

    #[test]
    fn algorithm_calls_plan_a_projection_feed_and_the_registered_name() {
        let called = plan("CALL bfs(1) YIELD node RETURN count(*)").expect("plans");
        fn find_call(op: &LogicalOp) -> Option<(&str, &metagdb_core::l4_compute::algebra::AlgorithmArgs, bool)> {
            match op {
                LogicalOp::CallAlgorithm { name, args, input } => {
                    let has_projection = matches!(input.as_ref(), LogicalOp::GraphProjection { .. });
                    Some((name.as_str(), args, has_projection))
                }
                LogicalOp::Project { input, .. }
                | LogicalOp::Filter { input, .. }
                | LogicalOp::Aggregate { input, .. }
                | LogicalOp::ExtensionOp { input, .. } => find_call(input),
                LogicalOp::Join { left, right, .. } => find_call(left).or_else(|| find_call(right)),
                _ => None,
            }
        }
        let (name, args, has_projection) = find_call(&called.root).expect("a call");
        assert_eq!(name, "bfs");
        assert!(has_projection, "the algorithm input must be a GraphProjection");
        assert_eq!(
            args.get("source"),
            Some(&PropertyValue::Int(1)),
            "positional arguments map to the algorithm's documented parameter order"
        );

        // Round-tripping a name the candidate does not implement is refused.
        let error = plan("CALL db.labels() YIELD label RETURN label").expect_err("refused");
        assert!(matches!(error, Error::Unsupported(_)), "got {error:?}");

        // A YIELD column the algorithm cannot produce is a planning error.
        let error = plan("CALL bfs(1) YIELD depth RETURN depth").expect_err("refused");
        assert!(matches!(error, Error::Plan(_)), "got {error:?}");
    }

    #[test]
    fn unbound_variables_are_refused_at_plan_time() {
        let error = plan("RETURN missing").expect_err("refused");
        assert!(matches!(error, Error::Plan(_)), "got {error:?}");
    }

    #[test]
    fn a_non_cypher_ast_is_refused() {
        struct Foreign;
        impl LanguageAst for Foreign {
            fn language(&self) -> QueryLanguage {
                QueryLanguage::Gremlin
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        let error = MetagdbPlanner
            .plan(Box::new(Foreign), &SchemaRegistry::new())
            .expect_err("refused");
        assert!(matches!(error, Error::Plan(_)), "got {error:?}");
    }

    #[test]
    fn plan_shapes_never_depend_on_query_text_spelling() {
        // Two spellings of the same statement produce the same operator shapes.
        let first = names("MATCH (n:Person) WHERE n.age > 29 RETURN n.name");
        assert_eq!(first, vec!["project", "filter", "vertex_scan"]);

        let alternate = names("MATCH (n:Person) WHERE 29 < n.age RETURN n.name");
        assert_eq!(alternate, first);
        // No operator carries the raw query text.
        let plan = plan("MATCH (n:Person) WHERE n.age > 29 RETURN n.name").expect("plans");
        let rendered = format!("{:?}", plan.root);
        assert!(!rendered.contains("MATCH"));
        assert!(!rendered.contains("WHERE"));
        let _ = OpaquePayload::new(());
    }
}
