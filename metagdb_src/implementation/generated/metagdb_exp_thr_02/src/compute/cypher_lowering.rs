//! Lowering from the repository-generated Cypher ANTLR CST to the private AST.
//!
//! The generated parser decides whether input conforms to `Cypher.g4`; this
//! module only interprets the resulting contexts.  It never re-tokenises or
//! re-implements the grammar.  Because `antlr4rust` returns *every* terminal
//! child from plural terminal accessors (the upstream `children_of_type` type
//! filter is commented out), terminal presence is always tested through the
//! singular typed accessors, `get_tokens`, or an explicit ordered walk of the
//! context's children.

use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::tree::{ParseTree, TerminalNode, Tree};
use antlr4rust::TidExt;
use metagdb_cypher_antlr::cypherparser::*;
use metagdb_core::{Error, Result};

use super::cypher_ast::*;

type Node<'i> = TerminalNode<'i, CypherParserContextType>;

/// All direct terminal children of a generated context, in source order.
fn terminals<'i, C>(ctx: &C) -> Vec<String>
where
    C: ParserRuleContext<'i, TF = LocalTokenFactory<'i>, Ctx = CypherParserContextType>,
{
    ctx.get_children()
        .filter_map(|child| child.downcast_rc::<Node<'i>>().ok())
        .map(|node| node.get_text())
        .collect()
}

fn first_terminal_text<'i, C>(ctx: &C) -> Option<String>
where
    C: ParserRuleContext<'i, TF = LocalTokenFactory<'i>, Ctx = CypherParserContextType>,
{
    terminals(ctx).into_iter().next()
}

fn require<T>(value: Option<std::rc::Rc<T>>, what: &str) -> Result<std::rc::Rc<T>> {
    value.ok_or_else(|| Error::Parse(format!("{what} expected")))
}

/// Entry point: lower the top-level `oC_Cypher` rule.
pub fn lower_cypher(ctx: &OC_CypherContext) -> Result<CypherAst> {
    let statement = ctx
        .oC_Statement()
        .ok_or_else(|| Error::Parse("statement expected".into()))?;
    let query = statement
        .oC_Query()
        .ok_or_else(|| Error::Parse("query expected".into()))?;
    lower_query(query.as_ref())
}

fn lower_query(ctx: &OC_QueryContext) -> Result<CypherAst> {
    if let Some(call) = ctx.oC_StandaloneCall() {
        return lower_standalone_call(call.as_ref());
    }
    let regular = ctx
        .oC_RegularQuery()
        .ok_or_else(|| Error::Parse("query body expected".into()))?;
    let first = require(regular.oC_SingleQuery(), "query")?;
    let mut branches = vec![(lower_single_query(first.as_ref())?, false)];
    for union in regular.oC_Union_all() {
        let all = union.ALL().is_some();
        let query = require(union.oC_SingleQuery(), "UNION branch")?;
        branches.push((lower_single_query(query.as_ref())?, all));
    }
    Ok(CypherAst::Query(Query { branches }))
}

fn lower_single_query(ctx: &OC_SingleQueryContext) -> Result<Vec<Clause>> {
    if let Some(part) = ctx.oC_SinglePartQuery() {
        return lower_single_part_query(part.as_ref());
    }
    if let Some(multi) = ctx.oC_MultiPartQuery() {
        return lower_multi_part_query(multi.as_ref());
    }
    Err(Error::Parse("query body expected".into()))
}

fn lower_single_part_query(ctx: &OC_SinglePartQueryContext) -> Result<Vec<Clause>> {
    let mut clauses = Vec::new();
    for reading in ctx.oC_ReadingClause_all() {
        clauses.push(lower_reading_clause(reading.as_ref())?);
    }
    for updating in ctx.oC_UpdatingClause_all() {
        clauses.push(lower_updating_clause(updating.as_ref())?);
    }
    if let Some(ret) = ctx.oC_Return() {
        clauses.push(lower_return(ret.as_ref())?);
    }
    Ok(clauses)
}

fn lower_multi_part_query(ctx: &OC_MultiPartQueryContext) -> Result<Vec<Clause>> {
    // `(reading* updating* WITH)+ singlePartQuery`; the typed plural accessors
    // lose the interleaving, so walk the ordered children instead.
    let mut clauses = Vec::new();
    for child in ctx.get_children() {
        if let Ok(with_ctx) = child.clone().downcast_rc::<OC_WithContext>() {
            clauses.push(lower_with(with_ctx.as_ref())?);
        } else if let Ok(reading) = child.clone().downcast_rc::<OC_ReadingClauseContext>() {
            clauses.push(lower_reading_clause(reading.as_ref())?);
        } else if let Ok(updating) = child.clone().downcast_rc::<OC_UpdatingClauseContext>() {
            clauses.push(lower_updating_clause(updating.as_ref())?);
        }
    }
    let tail = require(ctx.oC_SinglePartQuery(), "final query part")?;
    clauses.extend(lower_single_part_query(tail.as_ref())?);
    Ok(clauses)
}

fn lower_reading_clause(ctx: &OC_ReadingClauseContext) -> Result<Clause> {
    if let Some(m) = ctx.oC_Match() {
        return lower_match(m.as_ref());
    }
    if let Some(u) = ctx.oC_Unwind() {
        return lower_unwind(u.as_ref());
    }
    if let Some(c) = ctx.oC_InQueryCall() {
        return lower_in_query_call(c.as_ref());
    }
    Err(Error::Parse("unsupported reading clause".into()))
}

fn lower_updating_clause(ctx: &OC_UpdatingClauseContext) -> Result<Clause> {
    if let Some(c) = ctx.oC_Create() {
        return Ok(Clause::Create {
            pattern: lower_pattern(c.oC_Pattern())?,
        });
    }
    if let Some(m) = ctx.oC_Merge() {
        return lower_merge(m.as_ref());
    }
    if let Some(d) = ctx.oC_Delete() {
        return lower_delete(d.as_ref());
    }
    if let Some(s) = ctx.oC_Set() {
        return Ok(Clause::Set {
            items: lower_set_items(s.as_ref())?,
        });
    }
    if let Some(r) = ctx.oC_Remove() {
        return Ok(Clause::Remove {
            items: lower_remove_items(r.as_ref())?,
        });
    }
    Err(Error::Parse("unsupported updating clause".into()))
}

fn lower_match(ctx: &OC_MatchContext) -> Result<Clause> {
    Ok(Clause::Match {
        optional: ctx.OPTIONAL().is_some(),
        pattern: lower_pattern(ctx.oC_Pattern())?,
        predicate: lower_optional_where(ctx.oC_Where())?,
    })
}

fn lower_unwind(ctx: &OC_UnwindContext) -> Result<Clause> {
    let expr = lower_expr(require(ctx.oC_Expression(), "UNWIND expression")?.as_ref())?;
    let var = ctx
        .oC_Variable()
        .map(|v| clean_name(&v.get_text()))
        .ok_or_else(|| Error::Parse("UNWIND variable expected".into()))?;
    Ok(Clause::Unwind { expr, var })
}

fn lower_merge(ctx: &OC_MergeContext) -> Result<Clause> {
    let part = require(ctx.oC_PatternPart(), "MERGE pattern")?;
    let pattern = Pattern {
        parts: vec![lower_pattern_part(part.as_ref())?],
    };
    let mut on_create = Vec::new();
    let mut on_match = Vec::new();
    for action in ctx.oC_MergeAction_all() {
        let items = action
            .oC_Set()
            .map(|set| lower_set_items(set.as_ref()))
            .transpose()?
            .unwrap_or_default();
        if action.MATCH().is_some() {
            on_match.extend(items);
        } else if action.CREATE().is_some() {
            on_create.extend(items);
        }
    }
    Ok(Clause::Merge {
        pattern,
        on_create,
        on_match,
    })
}

fn lower_delete(ctx: &OC_DeleteContext) -> Result<Clause> {
    let mut exprs = Vec::new();
    for expr in ctx.oC_Expression_all() {
        exprs.push(lower_expr(expr.as_ref())?);
    }
    if exprs.is_empty() {
        return Err(Error::Parse("DELETE requires an expression".into()));
    }
    Ok(Clause::Delete {
        detach: ctx.DETACH().is_some(),
        exprs,
    })
}

fn lower_set_items(ctx: &OC_SetContext) -> Result<Vec<SetItem>> {
    let mut items = Vec::new();
    for item in ctx.oC_SetItem_all() {
        if let Some(labels_ctx) = item.oC_NodeLabels() {
            let var = item
                .oC_Variable()
                .map(|v| clean_name(&v.get_text()))
                .ok_or_else(|| Error::Parse("SET label target expected".into()))?;
            items.push(SetItem::Labels {
                var,
                labels: lower_labels(labels_ctx.as_ref())?,
            });
            continue;
        }
        let target_ctx = require(item.oC_PropertyExpression(), "SET target")?;
        let (target, key) = lower_property_expression(target_ctx.as_ref())?;
        let value = lower_expr(require(item.oC_Expression(), "SET value")?.as_ref())?;
        items.push(SetItem::Property { target, key, value });
    }
    Ok(items)
}

fn lower_remove_items(ctx: &OC_RemoveContext) -> Result<Vec<RemoveItem>> {
    let mut items = Vec::new();
    for item in ctx.oC_RemoveItem_all() {
        if let Some(labels_ctx) = item.oC_NodeLabels() {
            let var = item
                .oC_Variable()
                .map(|v| clean_name(&v.get_text()))
                .ok_or_else(|| Error::Parse("REMOVE label target expected".into()))?;
            items.push(RemoveItem::Labels {
                var,
                labels: lower_labels(labels_ctx.as_ref())?,
            });
            continue;
        }
        let target_ctx = require(item.oC_PropertyExpression(), "REMOVE target")?;
        let (target, key) = lower_property_expression(target_ctx.as_ref())?;
        items.push(RemoveItem::Property { target, key });
    }
    Ok(items)
}

struct Projection {
    distinct: bool,
    items: Vec<ReturnItem>,
    order: Vec<SortItem>,
    skip: Option<Expr>,
    limit: Option<Expr>,
}

fn lower_projection_body(ctx: &OC_ProjectionBodyContext) -> Result<Projection> {
    let distinct = ctx.DISTINCT().is_some();
    let items_ctx = require(ctx.oC_ProjectionItems(), "projection items")?;
    let mut items = Vec::new();
    for item in items_ctx.oC_ProjectionItem_all() {
        let expr = lower_expr(require(item.oC_Expression(), "projection expression")?.as_ref())?;
        let alias = item.oC_Variable().map(|v| clean_name(&v.get_text()));
        items.push(ReturnItem { expr, alias });
    }
    let star = items.is_empty();
    let mut order = Vec::new();
    if let Some(order_ctx) = ctx.oC_Order() {
        for sort_item in order_ctx.oC_SortItem_all() {
            let expr =
                lower_expr(require(sort_item.oC_Expression(), "ORDER BY expression")?.as_ref())?;
            let descending = sort_item.DESC().is_some() || sort_item.DESCENDING().is_some();
            order.push(SortItem { expr, descending });
        }
    }
    let skip = ctx
        .oC_Skip()
        .map(|s| lower_expr(require(s.oC_Expression(), "SKIP expression")?.as_ref()))
        .transpose()?;
    let limit = ctx
        .oC_Limit()
        .map(|l| lower_expr(require(l.oC_Expression(), "LIMIT expression")?.as_ref()))
        .transpose()?;
    if star {
        items.push(ReturnItem {
            expr: Expr::Variable("*".into()),
            alias: None,
        });
    }
    Ok(Projection {
        distinct,
        items,
        order,
        skip,
        limit,
    })
}

fn lower_with(ctx: &OC_WithContext) -> Result<Clause> {
    let body = require(ctx.oC_ProjectionBody(), "WITH projection")?;
    let projection = lower_projection_body(body.as_ref())?;
    Ok(Clause::With {
        distinct: projection.distinct,
        items: projection.items,
        order: projection.order,
        skip: projection.skip,
        limit: projection.limit,
        predicate: lower_optional_where(ctx.oC_Where())?,
    })
}

fn lower_return(ctx: &OC_ReturnContext) -> Result<Clause> {
    let body = require(ctx.oC_ProjectionBody(), "RETURN projection")?;
    let projection = lower_projection_body(body.as_ref())?;
    Ok(Clause::Return {
        distinct: projection.distinct,
        items: projection.items,
        order: projection.order,
        skip: projection.skip,
        limit: projection.limit,
    })
}

// ---------------------------------------------------------------------------
// Procedure calls
// ---------------------------------------------------------------------------

fn lower_in_query_call(ctx: &OC_InQueryCallContext) -> Result<Clause> {
    let invocation = require(
        ctx.oC_ExplicitProcedureInvocation(),
        "CALL procedure name",
    )?;
    let (name, args) = lower_procedure_invocation(invocation.as_ref())?;
    Ok(Clause::Call {
        name,
        args,
        yields: lower_yields(ctx.oC_YieldItems())?,
    })
}

fn lower_standalone_call(ctx: &OC_StandaloneCallContext) -> Result<CypherAst> {
    let (name, args) = if let Some(invocation) = ctx.oC_ExplicitProcedureInvocation() {
        lower_procedure_invocation(invocation.as_ref())?
    } else if let Some(implicit) = ctx.oC_ImplicitProcedureInvocation() {
        (
            clean_name(
                &require(implicit.oC_ProcedureName(), "procedure name")?.get_text(),
            ),
            Vec::new(),
        )
    } else {
        return Err(Error::Parse("CALL requires a procedure name".into()));
    };
    Ok(CypherAst::Call {
        name,
        args,
        yields: lower_yields(ctx.oC_YieldItems())?,
    })
}

fn lower_procedure_invocation(
    ctx: &OC_ExplicitProcedureInvocationContext,
) -> Result<(String, Vec<ProcArg>)> {
    let name = clean_name(
        &require(ctx.oC_ProcedureName(), "procedure name")?.get_text(),
    );
    let mut args = Vec::new();
    for expr in ctx.oC_Expression_all() {
        match lower_expr(expr.as_ref())? {
            Expr::Binary(BinOp::Eq, left, right) => match *left {
                Expr::Variable(key) => args.push(ProcArg::Named(key, *right)),
                other => args.push(ProcArg::Named(render_key(&other), *right)),
            },
            other => args.push(ProcArg::Positional(other)),
        }
    }
    Ok((name, args))
}

fn lower_yields(ctx: Option<std::rc::Rc<OC_YieldItemsContextAll>>) -> Result<Option<YieldSpec>> {
    let Some(ctx) = ctx else {
        return Ok(None);
    };
    let star = terminals(&*ctx).iter().any(|text| text == "*");
    let mut items = Vec::new();
    for item in ctx.oC_YieldItem_all() {
        let variable = item
            .oC_Variable()
            .map(|v| clean_name(&v.get_text()))
            .ok_or_else(|| Error::Parse("YIELD variable expected".into()))?;
        let field = item
            .oC_ProcedureResultField()
            .map(|f| clean_name(&f.get_text()))
            .unwrap_or_else(|| variable.clone());
        items.push((field, variable));
    }
    Ok(Some(YieldSpec {
        star,
        items,
        predicate: lower_optional_where(ctx.oC_Where())?,
    }))
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

fn lower_pattern(ctx: Option<std::rc::Rc<OC_PatternContextAll>>) -> Result<Pattern> {
    let ctx = ctx.ok_or_else(|| Error::Parse("pattern expected".into()))?;
    let mut parts = Vec::new();
    for part in ctx.oC_PatternPart_all() {
        parts.push(lower_pattern_part(part.as_ref())?);
    }
    Ok(Pattern { parts })
}

fn lower_pattern_part(ctx: &OC_PatternPartContext) -> Result<PatternPart> {
    let path_var = ctx.oC_Variable().map(|v| clean_name(&v.get_text()));
    let element = ctx
        .oC_AnonymousPatternPart()
        .and_then(|part| part.oC_PatternElement())
        .ok_or_else(|| Error::Parse("pattern element expected".into()))?;
    Ok(PatternPart {
        path_var,
        element: lower_pattern_element(element.as_ref())?,
    })
}

fn lower_pattern_element(ctx: &OC_PatternElementContext) -> Result<PatternElement> {
    if let Some(inner) = ctx.oC_PatternElement() {
        return lower_pattern_element(inner.as_ref());
    }
    let node = lower_node_pattern(require(ctx.oC_NodePattern(), "node pattern")?.as_ref())?;
    let mut chains = Vec::new();
    for chain in ctx.oC_PatternElementChain_all() {
        let edge = lower_edge_pattern(
            require(chain.oC_RelationshipPattern(), "relationship pattern")?.as_ref(),
        )?;
        let target =
            lower_node_pattern(require(chain.oC_NodePattern(), "node pattern")?.as_ref())?;
        chains.push((edge, target));
    }
    Ok(PatternElement { node, chains })
}

fn lower_node_pattern(ctx: &OC_NodePatternContext) -> Result<NodePattern> {
    let labels = match ctx.oC_NodeLabels() {
        Some(labels) => lower_labels(labels.as_ref())?,
        None => Vec::new(),
    };
    Ok(NodePattern {
        var: ctx.oC_Variable().map(|v| clean_name(&v.get_text())),
        labels,
        properties: lower_properties(ctx.oC_Properties())?,
        id_hint: None,
    })
}

fn lower_edge_pattern(ctx: &OC_RelationshipPatternContext) -> Result<EdgePattern> {
    let left = ctx.oC_LeftArrowHead().is_some();
    let right = ctx.oC_RightArrowHead().is_some();
    let direction = match (left, right) {
        (true, false) => Direction::Incoming,
        (false, true) => Direction::Outgoing,
        (false, false) => Direction::Both,
        (true, true) => {
            return Err(Error::Unsupported(
                "a bidirectional arrow pattern is outside the core profile".into(),
            ))
        }
    };
    let Some(detail) = ctx.oC_RelationshipDetail() else {
        return Ok(EdgePattern {
            var: None,
            types: Vec::new(),
            direction,
            range: None,
            properties: None,
        });
    };
    let mut types = Vec::new();
    if let Some(types_ctx) = detail.oC_RelationshipTypes() {
        for name in types_ctx.oC_RelTypeName_all() {
            types.push(clean_name(&name.get_text()));
        }
    }
    Ok(EdgePattern {
        var: detail.oC_Variable().map(|v| clean_name(&v.get_text())),
        types,
        direction,
        range: detail
            .oC_RangeLiteral()
            .map(|r| lower_range(r.as_ref()))
            .transpose()?,
        properties: lower_properties(detail.oC_Properties())?,
    })
}

fn lower_range(ctx: &OC_RangeLiteralContext) -> Result<RangeLiteral> {
    let text = ctx.get_text().replace(' ', "");
    let body = text.trim_start_matches('*');
    if body.is_empty() {
        return Ok(RangeLiteral {
            start: None,
            end: None,
        });
    }
    if let Some((start, end)) = body.split_once("..") {
        let parse = |value: &str| -> Result<Option<u64>> {
            if value.is_empty() {
                Ok(None)
            } else {
                value
                    .parse::<u64>()
                    .map(Some)
                    .map_err(|_| Error::Parse(format!("invalid range bound: {value}")))
            }
        };
        Ok(RangeLiteral {
            start: parse(start)?,
            end: parse(end)?,
        })
    } else {
        let exact = body
            .parse::<u64>()
            .map_err(|_| Error::Parse(format!("invalid range length: {body}")))?;
        Ok(RangeLiteral {
            start: Some(exact),
            end: Some(exact),
        })
    }
}

fn lower_labels(ctx: &OC_NodeLabelsContext) -> Result<Vec<String>> {
    let mut labels = Vec::new();
    for label in ctx.oC_NodeLabel_all() {
        let name = require(label.oC_LabelName(), "label name")?;
        labels.push(clean_name(&name.get_text()));
    }
    Ok(labels)
}

fn lower_properties(ctx: Option<std::rc::Rc<OC_PropertiesContextAll>>) -> Result<Option<Expr>> {
    let Some(ctx) = ctx else {
        return Ok(None);
    };
    if let Some(map) = ctx.oC_MapLiteral() {
        return Ok(Some(lower_map_literal(map.as_ref())?));
    }
    if let Some(param) = ctx.oC_Parameter() {
        return Ok(Some(lower_parameter(param.as_ref())?));
    }
    Ok(None)
}

fn lower_map_literal(ctx: &OC_MapLiteralContext) -> Result<Expr> {
    let keys = ctx.oC_PropertyKeyName_all();
    let values = ctx.oC_Expression_all();
    let mut entries = Vec::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        entries.push((clean_name(&key.get_text()), lower_expr(value.as_ref())?));
    }
    Ok(Expr::Map(entries))
}

fn lower_parameter(ctx: &OC_ParameterContext) -> Result<Expr> {
    Ok(Expr::Parameter(
        ctx.get_text().trim_start_matches('$').to_string(),
    ))
}

/// Lower `var.key` used by SET/REMOVE, returning the base expression and key.
fn lower_property_expression(ctx: &OC_PropertyExpressionContext) -> Result<(Expr, String)> {
    let atom = require(ctx.oC_Atom(), "property target")?;
    let base = lower_atom(atom.as_ref())?;
    let lookups = ctx.oC_PropertyLookup_all();
    let first = lookups
        .first()
        .ok_or_else(|| Error::Parse("property key expected".into()))?;
    let key = clean_name(
        &require(first.oC_PropertyKeyName(), "property key")?.get_text(),
    );
    if lookups.len() > 1 {
        return Err(Error::Unsupported(
            "nested property assignment is outside the core profile".into(),
        ));
    }
    Ok((base, key))
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

fn lower_optional_where(ctx: Option<std::rc::Rc<OC_WhereContextAll>>) -> Result<Option<Expr>> {
    match ctx {
        Some(ctx) => {
            let expr = require(ctx.oC_Expression(), "WHERE expression")?;
            Ok(Some(lower_expr(expr.as_ref())?))
        }
        None => Ok(None),
    }
}

pub fn lower_expr(ctx: &OC_ExpressionContext) -> Result<Expr> {
    let or = require(ctx.oC_OrExpression(), "expression")?;
    lower_or(or.as_ref())
}

fn lower_or(ctx: &OC_OrExpressionContext) -> Result<Expr> {
    let mut iter = ctx.oC_XorExpression_all().into_iter();
    let first = iter
        .next()
        .ok_or_else(|| Error::Parse("expression expected".into()))?;
    let mut expr = lower_xor(first.as_ref())?;
    for operand in iter {
        expr = Expr::Binary(BinOp::Or, Box::new(expr), Box::new(lower_xor(operand.as_ref())?));
    }
    Ok(expr)
}

fn lower_xor(ctx: &OC_XorExpressionContext) -> Result<Expr> {
    let mut iter = ctx.oC_AndExpression_all().into_iter();
    let first = iter
        .next()
        .ok_or_else(|| Error::Parse("expression expected".into()))?;
    let mut expr = lower_and(first.as_ref())?;
    for operand in iter {
        expr = Expr::Binary(BinOp::Xor, Box::new(expr), Box::new(lower_and(operand.as_ref())?));
    }
    Ok(expr)
}

fn lower_and(ctx: &OC_AndExpressionContext) -> Result<Expr> {
    let mut iter = ctx.oC_NotExpression_all().into_iter();
    let first = iter
        .next()
        .ok_or_else(|| Error::Parse("expression expected".into()))?;
    let mut expr = lower_not(first.as_ref())?;
    for operand in iter {
        expr = Expr::Binary(BinOp::And, Box::new(expr), Box::new(lower_not(operand.as_ref())?));
    }
    Ok(expr)
}

fn lower_not(ctx: &OC_NotExpressionContext) -> Result<Expr> {
    let count = ctx.get_tokens(Cypher_NOT).len();
    let inner = require(ctx.oC_ComparisonExpression(), "comparison expression")?;
    let mut expr = lower_comparison(inner.as_ref())?;
    for _ in 0..count {
        expr = Expr::Not(Box::new(expr));
    }
    Ok(expr)
}

fn lower_comparison(ctx: &OC_ComparisonExpressionContext) -> Result<Expr> {
    let base = require(ctx.oC_AddOrSubtractExpression(), "expression")?;
    let mut expr = lower_add_subtract(base.as_ref())?;
    for partial in ctx.oC_PartialComparisonExpression_all() {
        let operator = first_terminal_text(partial.as_ref())
            .ok_or_else(|| Error::Parse("comparison operator expected".into()))?;
        let rhs = require(partial.oC_AddOrSubtractExpression(), "comparison operand")?;
        let op = match operator.as_str() {
            "=" => BinOp::Eq,
            "<>" => BinOp::Neq,
            "<" => BinOp::Lt,
            "<=" => BinOp::Lte,
            ">" => BinOp::Gt,
            ">=" => BinOp::Gte,
            other => {
                return Err(Error::Unsupported(format!(
                    "comparison operator {other} is outside the core profile"
                )))
            }
        };
        expr = Expr::Binary(op, Box::new(expr), Box::new(lower_add_subtract(rhs.as_ref())?));
    }
    Ok(expr)
}

fn lower_add_subtract(ctx: &OC_AddOrSubtractExpressionContext) -> Result<Expr> {
    combine_chain(
        ctx.oC_MultiplyDivideModuloExpression_all()
            .iter()
            .map(|operand| lower_multiply_divide(operand.as_ref()))
            .collect::<Result<Vec<_>>>()?,
        terminals(ctx),
        &[("+", BinOp::Add), ("-", BinOp::Sub)],
        "additive",
    )
}

fn lower_multiply_divide(ctx: &OC_MultiplyDivideModuloExpressionContext) -> Result<Expr> {
    combine_chain(
        ctx.oC_PowerOfExpression_all()
            .iter()
            .map(|operand| lower_power(operand.as_ref()))
            .collect::<Result<Vec<_>>>()?,
        terminals(ctx),
        &[("*", BinOp::Mul), ("/", BinOp::Div), ("%", BinOp::Mod)],
        "multiplicative",
    )
}

fn lower_power(ctx: &OC_PowerOfExpressionContext) -> Result<Expr> {
    combine_chain(
        ctx.oC_UnaryAddOrSubtractExpression_all()
            .iter()
            .map(|operand| lower_unary(operand.as_ref()))
            .collect::<Result<Vec<_>>>()?,
        terminals(ctx),
        &[("^", BinOp::Pow)],
        "power",
    )
}

fn combine_chain(
    operands: Vec<Expr>,
    terminals: Vec<String>,
    table: &[(&str, BinOp)],
    label: &str,
) -> Result<Expr> {
    // Only the operators of this precedence level are direct terminal children,
    // and they appear in source order, so zipping them with the operands keeps
    // `a - b - c` left-associative.
    let operators: Vec<&String> = terminals
        .iter()
        .filter(|text| table.iter().any(|(symbol, _)| symbol == &text.as_str()))
        .collect();
    let mut iter = operands.into_iter();
    let mut expr = iter
        .next()
        .ok_or_else(|| Error::Parse(format!("{label} expression expected")))?;
    for (index, operand) in iter.enumerate() {
        let operator = operators
            .get(index)
            .ok_or_else(|| Error::Parse(format!("{label} operator expected")))?;
        let op = table
            .iter()
            .find(|(text, _)| text == operator)
            .map(|(_, op)| *op)
            .ok_or_else(|| Error::Unsupported(format!("{label} operator {operator}")))?;
        expr = Expr::Binary(op, Box::new(expr), Box::new(operand));
    }
    Ok(expr)
}

fn lower_unary(ctx: &OC_UnaryAddOrSubtractExpressionContext) -> Result<Expr> {
    let operators: Vec<String> = terminals(ctx);
    let inner = require(ctx.oC_StringListNullOperatorExpression(), "unary operand")?;
    let mut expr = lower_string_list_null(inner.as_ref())?;
    for operator in operators.iter().rev() {
        if operator == "-" {
            expr = Expr::Negate(Box::new(expr));
        }
    }
    Ok(expr)
}

fn lower_string_list_null(ctx: &OC_StringListNullOperatorExpressionContext) -> Result<Expr> {
    let base = require(ctx.oC_PropertyOrLabelsExpression(), "expression operand")?;
    let mut expr = lower_property_or_labels(base.as_ref())?;
    for string_op in ctx.oC_StringOperatorExpression_all() {
        let rhs = lower_property_or_labels(
            require(string_op.oC_PropertyOrLabelsExpression(), "operand")?.as_ref(),
        )?;
        if string_op.STARTS().is_some() {
            expr = Expr::StartsWith(Box::new(expr), Box::new(rhs));
        } else if string_op.ENDS().is_some() {
            expr = Expr::EndsWith(Box::new(expr), Box::new(rhs));
        } else if string_op.CONTAINS().is_some() {
            expr = Expr::Contains(Box::new(expr), Box::new(rhs));
        } else {
            return Err(Error::Unsupported(
                "this string operator is outside the core profile".into(),
            ));
        }
    }
    for list_op in ctx.oC_ListOperatorExpression_all() {
        expr = lower_list_operator(expr, list_op.as_ref())?;
    }
    for null_op in ctx.oC_NullOperatorExpression_all() {
        expr = if null_op.NOT().is_some() {
            Expr::IsNotNull(Box::new(expr))
        } else {
            Expr::IsNull(Box::new(expr))
        };
    }
    Ok(expr)
}

fn lower_list_operator(base: Expr, ctx: &OC_ListOperatorExpressionContext) -> Result<Expr> {
    if ctx.IN().is_some() {
        let rhs = require(ctx.oC_PropertyOrLabelsExpression(), "IN operand")?;
        return Ok(Expr::In(
            Box::new(base),
            Box::new(lower_property_or_labels(rhs.as_ref())?),
        ));
    }
    let is_slice = terminals(ctx).iter().any(|text| text == "..");
    let expressions = ctx.oC_Expression_all();
    if is_slice {
        // `[a..b]`, `[..b]` and `[a..]` all parse to the same production, so the
        // `..` position decides whether an expression is the start or the end.
        let mut start: Option<Box<Expr>> = None;
        let mut end: Option<Box<Expr>> = None;
        let mut seen_dots = false;
        for child in ctx.get_children() {
            if let Ok(terminal) = child.clone().downcast_rc::<Node>() {
                if terminal.get_text() == ".." {
                    seen_dots = true;
                }
            } else if let Ok(expr) = child.clone().downcast_rc::<OC_ExpressionContext>() {
                let lowered = Box::new(lower_expr(expr.as_ref())?);
                if seen_dots {
                    end = Some(lowered);
                } else {
                    start = Some(lowered);
                }
            }
        }
        Ok(Expr::Slice {
            base: Box::new(base),
            start,
            end,
        })
    } else {
        let index = expressions
            .first()
            .ok_or_else(|| Error::Parse("list subscript expected".into()))?;
        Ok(Expr::Index(
            Box::new(base),
            Box::new(lower_expr(index.as_ref())?),
        ))
    }
}

fn lower_property_or_labels(ctx: &OC_PropertyOrLabelsExpressionContext) -> Result<Expr> {
    let atom = require(ctx.oC_Atom(), "expression atom")?;
    let mut expr = lower_atom(atom.as_ref())?;
    for lookup in ctx.oC_PropertyLookup_all() {
        let key = clean_name(
            &require(lookup.oC_PropertyKeyName(), "property key")?.get_text(),
        );
        expr = Expr::Property(Box::new(expr), key);
    }
    if let Some(labels) = ctx.oC_NodeLabels() {
        expr = Expr::HasLabels(Box::new(expr), lower_labels(labels.as_ref())?);
    }
    Ok(expr)
}

fn lower_atom(ctx: &OC_AtomContext) -> Result<Expr> {
    if let Some(literal) = ctx.oC_Literal() {
        return lower_literal(literal.as_ref());
    }
    if let Some(param) = ctx.oC_Parameter() {
        return lower_parameter(param.as_ref());
    }
    if let Some(case) = ctx.oC_CaseExpression() {
        return lower_case(case.as_ref());
    }
    if ctx.COUNT().is_some() {
        return Ok(Expr::CountAll);
    }
    if let Some(comprehension) = ctx.oC_ListComprehension() {
        return lower_list_comprehension(comprehension.as_ref());
    }
    if ctx.oC_PatternComprehension().is_some() {
        return Err(Error::Unsupported(
            "pattern comprehension is outside the core language profile".into(),
        ));
    }
    for (present, kind) in [
        (ctx.ALL().is_some(), Quantifier::All),
        (ctx.ANY().is_some(), Quantifier::Any),
        (ctx.NONE().is_some(), Quantifier::None),
        (ctx.SINGLE().is_some(), Quantifier::Single),
    ] {
        if present {
            let filter = require(ctx.oC_FilterExpression(), "quantifier body")?;
            return lower_quantifier(kind, filter.as_ref());
        }
    }
    if let Some(pattern) = ctx.oC_RelationshipsPattern() {
        return Ok(Expr::PatternExists(lower_relationships_pattern(
            pattern.as_ref(),
        )?));
    }
    if let Some(paren) = ctx.oC_ParenthesizedExpression() {
        let inner = require(paren.oC_Expression(), "parenthesised expression")?;
        return lower_expr(inner.as_ref());
    }
    if let Some(call) = ctx.oC_FunctionInvocation() {
        return lower_function(call.as_ref());
    }
    if let Some(variable) = ctx.oC_Variable() {
        return Ok(Expr::Variable(clean_name(&variable.get_text())));
    }
    Err(Error::Unsupported(
        "this expression form is outside the core language profile".into(),
    ))
}

fn lower_relationships_pattern(ctx: &OC_RelationshipsPatternContext) -> Result<Pattern> {
    let node = lower_node_pattern(require(ctx.oC_NodePattern(), "node pattern")?.as_ref())?;
    let mut chains = Vec::new();
    for chain in ctx.oC_PatternElementChain_all() {
        let edge = lower_edge_pattern(
            require(chain.oC_RelationshipPattern(), "relationship pattern")?.as_ref(),
        )?;
        let target =
            lower_node_pattern(require(chain.oC_NodePattern(), "node pattern")?.as_ref())?;
        chains.push((edge, target));
    }
    Ok(Pattern {
        parts: vec![PatternPart {
            path_var: None,
            element: PatternElement { node, chains },
        }],
    })
}

fn lower_filter_expression(
    ctx: &OC_FilterExpressionContext,
) -> Result<(String, Expr, Option<Expr>)> {
    let id_in_coll = require(ctx.oC_IdInColl(), "variable IN list")?;
    let var = id_in_coll
        .oC_Variable()
        .map(|v| clean_name(&v.get_text()))
        .ok_or_else(|| Error::Parse("loop variable expected".into()))?;
    let list = lower_expr(require(id_in_coll.oC_Expression(), "list expression")?.as_ref())?;
    let filter = match ctx.oC_Where() {
        Some(where_ctx) => Some(lower_expr(
            require(where_ctx.oC_Expression(), "filter predicate")?.as_ref(),
        )?),
        None => None,
    };
    Ok((var, list, filter))
}

fn lower_quantifier(kind: Quantifier, ctx: &OC_FilterExpressionContext) -> Result<Expr> {
    let (var, list, filter) = lower_filter_expression(ctx)?;
    Ok(Expr::Quantifier {
        kind,
        var,
        list: Box::new(list),
        filter: filter.map(Box::new),
    })
}

fn lower_list_comprehension(ctx: &OC_ListComprehensionContext) -> Result<Expr> {
    let filter = require(ctx.oC_FilterExpression(), "comprehension body")?;
    let (var, list, predicate) = lower_filter_expression(filter.as_ref())?;
    let map = ctx
        .oC_Expression()
        .map(|expr| lower_expr(expr.as_ref()).map(Box::new))
        .transpose()?;
    Ok(Expr::ListComprehension {
        var,
        list: Box::new(list),
        filter: predicate.map(Box::new),
        map,
    })
}

fn lower_case(ctx: &OC_CaseExpressionContext) -> Result<Expr> {
    // A subject expression appears before the first WHEN; the ELSE expression
    // appears after the last alternative.  Walking the ordered children keeps
    // the two apart even when the alternative count matches.
    let mut subject: Option<Box<Expr>> = None;
    let mut default: Option<Box<Expr>> = None;
    let mut alternatives = Vec::new();
    let mut seen_when = false;
    for child in ctx.get_children() {
        if let Ok(expr) = child.clone().downcast_rc::<OC_ExpressionContext>() {
            let lowered = lower_expr(expr.as_ref())?;
            if seen_when {
                default = Some(Box::new(lowered));
            } else {
                subject = Some(Box::new(lowered));
            }
        } else if let Ok(alternative) = child.clone().downcast_rc::<OC_CaseAlternativesContext>()
        {
            seen_when = true;
            let mut operands = alternative.oC_Expression_all().into_iter();
            let when = require(operands.next(), "WHEN expression")?;
            let then = require(operands.next(), "THEN expression")?;
            alternatives.push((
                lower_expr(when.as_ref())?,
                lower_expr(then.as_ref())?,
            ));
        }
    }
    Ok(Expr::Case {
        subject,
        alternatives,
        default,
    })
}

fn lower_function(ctx: &OC_FunctionInvocationContext) -> Result<Expr> {
    let name_ctx = require(ctx.oC_FunctionName(), "function name")?;
    let mut args = Vec::new();
    for expr in ctx.oC_Expression_all() {
        args.push(lower_expr(expr.as_ref())?);
    }
    Ok(Expr::Function {
        name: clean_name(&name_ctx.get_text()),
        distinct: ctx.DISTINCT().is_some(),
        args,
    })
}

fn lower_literal(ctx: &OC_LiteralContext) -> Result<Expr> {
    if let Some(boolean) = ctx.oC_BooleanLiteral() {
        return Ok(Expr::Bool(boolean.TRUE().is_some()));
    }
    if let Some(number) = ctx.oC_NumberLiteral() {
        let text = number.get_text();
        if number.oC_DoubleLiteral().is_some()
            || text.contains('.')
            || text.contains('e')
            || text.contains('E')
        {
            let value: f64 = text
                .parse()
                .map_err(|_| Error::Parse(format!("invalid numeric literal: {text}")))?;
            return Ok(Expr::Float(value));
        }
        let value: i64 = text
            .parse()
            .map_err(|_| Error::Parse(format!("invalid integer literal: {text}")))?;
        return Ok(Expr::Int(value));
    }
    if let Some(map) = ctx.oC_MapLiteral() {
        return lower_map_literal(map.as_ref());
    }
    if let Some(list) = ctx.oC_ListLiteral() {
        let mut items = Vec::new();
        for expr in list.oC_Expression_all() {
            items.push(lower_expr(expr.as_ref())?);
        }
        return Ok(Expr::List(items));
    }
    if ctx.NULL().is_some() {
        return Ok(Expr::Null);
    }
    Ok(Expr::Str(unescape_string(&ctx.get_text())))
}

// ---------------------------------------------------------------------------
// Text helpers
// ---------------------------------------------------------------------------

/// Strip backticks from an escaped symbolic name.
fn clean_name(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('`') && trimmed.ends_with('`') {
        trimmed[1..trimmed.len() - 1].replace("``", "`")
    } else {
        trimmed.to_string()
    }
}

/// Remove the surrounding quotes from a Cypher string literal.
fn unescape_string(text: &str) -> String {
    let trimmed = text.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() < 2 {
        return trimmed.to_string();
    }
    let quote = bytes[0];
    if (quote != b'\'' && quote != b'"') || bytes[bytes.len() - 1] != quote {
        return trimmed.to_string();
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('\\') => out.push('\\'),
            Some('\'') => out.push('\''),
            Some('"') => out.push('"'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                    if let Some(ch) = char::from_u32(code) {
                        out.push(ch);
                    }
                }
            }
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

/// Render a simple key for a positional/named procedure argument.
fn render_key(expr: &Expr) -> String {
    match expr {
        Expr::Variable(name) => name.clone(),
        Expr::Property(base, key) => format!("{}.{}", render_key(base), key),
        _ => String::new(),
    }
}
