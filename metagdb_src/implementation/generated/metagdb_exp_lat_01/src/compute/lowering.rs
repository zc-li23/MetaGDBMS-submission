//! Lowering from the generated ANTLR Cypher CST into the private AST.
//!
//! Only the repository-generated lexer/parser is used as the syntax frontend:
//! `MetagdbParser` invokes `oC_Cypher()` and hands the resulting CST here. Each
//! function mirrors one grammar rule and reads the CST through the generated
//! accessors.

#![allow(non_upper_case_globals)]

use std::rc::Rc;

use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::token::Token;
use antlr4rust::tree::{ParseTree, Tree};
use metagdb_cypher_antlr::cypherparser::*;
use metagdb_core::{Error, Result};

use super::ast::*;

type LResult<T> = Result<T>;

fn parse_err(message: impl Into<String>) -> Error {
    Error::Parse(message.into())
}

fn unsupported(message: impl Into<String>) -> Error {
    Error::Unsupported(message.into())
}

// ---------------------------------------------------------------------------
// Text helpers
// ---------------------------------------------------------------------------

fn text<'input, C: ParseTree<'input>>(ctx: &C) -> String {
    ctx.get_text()
}

fn symbolic_name(ctx: &OC_SymbolicNameContext<'_>) -> String {
    unescape(text(ctx).trim())
}

fn unescape(raw: &str) -> String {
    let raw = raw.trim();
    if raw.starts_with('`') && raw.ends_with('`') && raw.len() >= 2 {
        raw[1..raw.len() - 1].replace("``", "`")
    } else {
        raw.to_string()
    }
}

fn schema_name(ctx: &OC_SchemaNameContext<'_>) -> String {
    unescape(text(ctx).trim())
}

fn integer_literal(ctx: &OC_IntegerLiteralContext<'_>) -> Result<i64> {
    let raw = text(ctx);
    let raw = raw.trim();
    if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).map_err(|e| parse_err(format!("bad hex integer: {e}")))
    } else if raw.len() > 1 && raw.starts_with('0') && raw[1..].chars().all(|c| ('0'..='7').contains(&c)) {
        i64::from_str_radix(&raw[1..], 8).map_err(|e| parse_err(format!("bad octal integer: {e}")))
    } else {
        raw.parse::<i64>()
            .map_err(|e| parse_err(format!("bad integer `{raw}`: {e}")))
    }
}

fn string_literal(ctx: &antlr4rust::tree::TerminalNode<'_, CypherParserContextType>) -> String {
    let raw = text(ctx);
    let trimmed = raw.trim();
    let inner = if trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{8}'),
            Some('f') => out.push('\u{c}'),
            Some('\\') => out.push('\\'),
            Some('\'') => out.push('\''),
            Some('"') => out.push('"'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                    if let Some(c) = char::from_u32(code) {
                        out.push(c);
                    }
                }
            }
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Query / clause lowering
// ---------------------------------------------------------------------------

pub fn lower_cypher(ctx: &OC_CypherContext<'_>) -> Result<CypherAst> {
    let statement = ctx
        .oC_Statement()
        .ok_or_else(|| parse_err("missing statement"))?;
    let query_ctx = statement
        .oC_Query()
        .ok_or_else(|| parse_err("missing query"))?;
    Ok(CypherAst {
        query: lower_query(&query_ctx)?,
    })
}

fn lower_query(ctx: &OC_QueryContext<'_>) -> LResult<Query> {
    if let Some(regular) = ctx.oC_RegularQuery() {
        return lower_regular_query(&regular);
    }
    if let Some(call) = ctx.oC_StandaloneCall() {
        return Ok(Query {
            head: vec![Clause::Call(lower_standalone_call(&call)?)],
            unions: Vec::new(),
        });
    }
    Err(unsupported("query form is not supported"))
}

fn lower_regular_query(ctx: &OC_RegularQueryContext<'_>) -> LResult<Query> {
    let first = ctx
        .oC_SingleQuery()
        .ok_or_else(|| parse_err("missing single query"))?;
    let head = lower_single_query(&first)?;
    let mut unions = Vec::new();
    for union in ctx.oC_Union_all() {
        let all = union.ALL().is_some();
        let single = union
            .oC_SingleQuery()
            .ok_or_else(|| parse_err("UNION without query"))?;
        unions.push(Union {
            all,
            clauses: lower_single_query(&single)?,
        });
    }
    Ok(Query { head, unions })
}

fn lower_single_query(ctx: &OC_SingleQueryContext<'_>) -> LResult<Vec<Clause>> {
    if let Some(single) = ctx.oC_SinglePartQuery() {
        return lower_single_part_query(&single);
    }
    if let Some(multi) = ctx.oC_MultiPartQuery() {
        return lower_multi_part_query(&multi);
    }
    Err(unsupported("single query form is not supported"))
}

fn lower_single_part_query(ctx: &OC_SinglePartQueryContext<'_>) -> LResult<Vec<Clause>> {
    let mut clauses = Vec::new();
    for reading in ctx.oC_ReadingClause_all() {
        clauses.push(lower_reading_clause(&reading)?);
    }
    for updating in ctx.oC_UpdatingClause_all() {
        clauses.push(lower_updating_clause(&updating)?);
    }
    if let Some(ret) = ctx.oC_Return() {
        clauses.push(Clause::Return(lower_return(&ret)?));
    }
    Ok(clauses)
}

fn lower_multi_part_query(ctx: &OC_MultiPartQueryContext<'_>) -> LResult<Vec<Clause>> {
    // The reading/updating accessors return per-type lists; the interleaving is
    // recovered from the ordered child rule indices so `MATCH ... WITH ... UNWIND`
    // and similar sequences keep their order.
    let readings: Vec<_> = ctx.oC_ReadingClause_all();
    let updatings: Vec<_> = ctx.oC_UpdatingClause_all();
    let withs: Vec<_> = ctx.oC_With_all();
    let mut reading_index = 0usize;
    let mut updating_index = 0usize;
    let mut with_index = 0usize;
    let mut clauses = Vec::new();
    for child in ctx.get_children() {
        match child.get_rule_index() {
            RULE_oC_ReadingClause => {
                clauses.push(lower_reading_clause(&readings[reading_index])?);
                reading_index += 1;
            }
            RULE_oC_UpdatingClause => {
                clauses.push(lower_updating_clause(&updatings[updating_index])?);
                updating_index += 1;
            }
            RULE_oC_With => {
                clauses.push(Clause::With(lower_with(&withs[with_index])?));
                with_index += 1;
            }
            _ => {}
        }
    }
    let single = ctx
        .oC_SinglePartQuery()
        .ok_or_else(|| parse_err("multi-part query without tail"))?;
    clauses.extend(lower_single_part_query(&single)?);
    Ok(clauses)
}

fn lower_reading_clause(ctx: &OC_ReadingClauseContext<'_>) -> LResult<Clause> {
    if let Some(m) = ctx.oC_Match() {
        return Ok(Clause::Match(lower_match(&m)?));
    }
    if let Some(u) = ctx.oC_Unwind() {
        let expr = lower_expression(
            u.oC_Expression()
                .ok_or_else(|| parse_err("UNWIND without expression"))?
                .as_ref(),
        )?;
        let var = lower_variable(
            u.oC_Variable()
                .ok_or_else(|| parse_err("UNWIND without variable"))?
                .as_ref(),
        );
        return Ok(Clause::Unwind { expr, var });
    }
    if let Some(call) = ctx.oC_InQueryCall() {
        return Ok(Clause::Call(lower_in_query_call(&call)?));
    }
    Err(unsupported("reading clause is not supported"))
}

fn lower_updating_clause(ctx: &OC_UpdatingClauseContext<'_>) -> LResult<Clause> {
    if let Some(create) = ctx.oC_Create() {
        let pattern = create
            .oC_Pattern()
            .ok_or_else(|| parse_err("CREATE without pattern"))?;
        return Ok(Clause::Create(lower_pattern(&pattern)?));
    }
    if let Some(merge) = ctx.oC_Merge() {
        return Ok(Clause::Merge(lower_merge(&merge)?));
    }
    if let Some(delete) = ctx.oC_Delete() {
        let mut exprs = Vec::new();
        for expr in delete.oC_Expression_all() {
            exprs.push(lower_expression(&expr)?);
        }
        return Ok(Clause::Delete {
            detach: delete.DETACH().is_some(),
            exprs,
        });
    }
    if let Some(set) = ctx.oC_Set() {
        return Ok(Clause::Set(lower_set(&set)?));
    }
    if let Some(remove) = ctx.oC_Remove() {
        return Ok(Clause::Remove(lower_remove(&remove)?));
    }
    Err(unsupported("updating clause is not supported"))
}

fn lower_match(ctx: &OC_MatchContext<'_>) -> LResult<MatchClause> {
    let pattern = ctx
        .oC_Pattern()
        .ok_or_else(|| parse_err("MATCH without pattern"))?;
    let where_clause = match ctx.oC_Where() {
        Some(w) => Some(lower_where(&w)?),
        None => None,
    };
    Ok(MatchClause {
        optional: ctx.OPTIONAL().is_some(),
        parts: lower_pattern(&pattern)?,
        where_clause,
    })
}

fn lower_where(ctx: &OC_WhereContext<'_>) -> LResult<Expr> {
    let expr = ctx
        .oC_Expression()
        .ok_or_else(|| parse_err("WHERE without expression"))?;
    lower_expression(&expr)
}

fn lower_with(ctx: &OC_WithContext<'_>) -> LResult<ProjectionBody> {
    let body = ctx
        .oC_ProjectionBody()
        .ok_or_else(|| parse_err("WITH without projection"))?;
    let mut projection = lower_projection_body(&body)?;
    if let Some(w) = ctx.oC_Where() {
        projection.where_clause = Some(lower_where(&w)?);
    }
    Ok(projection)
}

fn lower_return(ctx: &OC_ReturnContext<'_>) -> LResult<ProjectionBody> {
    let body = ctx
        .oC_ProjectionBody()
        .ok_or_else(|| parse_err("RETURN without projection"))?;
    lower_projection_body(&body)
}

fn lower_projection_body(ctx: &OC_ProjectionBodyContext<'_>) -> LResult<ProjectionBody> {
    let items_ctx = ctx
        .oC_ProjectionItems()
        .ok_or_else(|| parse_err("projection without items"))?;
    let mut items = Vec::new();
    if !items_ctx.get_tokens(Cypher_T__4).is_empty() {
        items.push(ProjectionItem {
            expr: Expr::Var("*".into()),
            alias: None,
        });
    }
    for item in items_ctx.oC_ProjectionItem_all() {
        let expr = lower_expression(
            item.oC_Expression()
                .ok_or_else(|| parse_err("projection item without expression"))?
                .as_ref(),
        )?;
        let alias = item.oC_Variable().map(|v| lower_variable(&v));
        items.push(ProjectionItem { expr, alias });
    }
    let mut order_by = Vec::new();
    if let Some(order) = ctx.oC_Order() {
        for sort in order.oC_SortItem_all() {
            let expr = lower_expression(
                sort.oC_Expression()
                    .ok_or_else(|| parse_err("ORDER BY without expression"))?
                    .as_ref(),
            )?;
            let desc = sort.DESC().is_some() || sort.DESCENDING().is_some();
            order_by.push((expr, desc));
        }
    }
    let skip = match ctx.oC_Skip() {
        Some(s) => Some(lower_expression(
            s.oC_Expression()
                .ok_or_else(|| parse_err("SKIP without expression"))?
                .as_ref(),
        )?),
        None => None,
    };
    let limit = match ctx.oC_Limit() {
        Some(l) => Some(lower_expression(
            l.oC_Expression()
                .ok_or_else(|| parse_err("LIMIT without expression"))?
                .as_ref(),
        )?),
        None => None,
    };
    Ok(ProjectionBody {
        distinct: ctx.DISTINCT().is_some(),
        items,
        order_by,
        skip,
        limit,
        where_clause: None,
    })
}

fn lower_merge(ctx: &OC_MergeContext<'_>) -> LResult<MergeClause> {
    let part_ctx = ctx
        .oC_PatternPart()
        .ok_or_else(|| parse_err("MERGE without pattern"))?;
    let part = lower_pattern_part(&part_ctx)?;
    let mut on_create = Vec::new();
    let mut on_match = Vec::new();
    for action in ctx.oC_MergeAction_all() {
        let set = action
            .oC_Set()
            .ok_or_else(|| parse_err("MERGE action without SET"))?;
        let items = lower_set(&set)?;
        if action.MATCH().is_some() {
            on_match.extend(items);
        } else {
            on_create.extend(items);
        }
    }
    Ok(MergeClause {
        part,
        on_create,
        on_match,
    })
}

fn lower_set(ctx: &OC_SetContext<'_>) -> LResult<Vec<SetItem>> {
    let mut items = Vec::new();
    for item in ctx.oC_SetItem_all() {
        let value = item.oC_Expression();
        if let Some(prop) = item.oC_PropertyExpression() {
            let target = lower_property_expression(&prop)?;
            let value = value.ok_or_else(|| parse_err("SET without value"))?;
            items.push(SetItem::Property {
                target,
                value: lower_expression(&value)?,
            });
            continue;
        }
        let var = item
            .oC_Variable()
            .ok_or_else(|| parse_err("SET item without target"))?;
        let name = lower_variable(&var);
        if let Some(labels) = item.oC_NodeLabels() {
            items.push(SetItem::Labels {
                name,
                labels: lower_node_labels(&labels),
            });
            continue;
        }
        let value = value.ok_or_else(|| parse_err("SET without value"))?;
        let merge = !item.get_tokens(Cypher_T__3).is_empty();
        let value = lower_expression(&value)?;
        if merge {
            items.push(SetItem::Merge { name, value });
        } else {
            items.push(SetItem::Variable { name, value });
        }
    }
    Ok(items)
}

fn lower_remove(ctx: &OC_RemoveContext<'_>) -> LResult<Vec<RemoveItem>> {
    let mut items = Vec::new();
    for item in ctx.oC_RemoveItem_all() {
        if let Some(prop) = item.oC_PropertyExpression() {
            items.push(RemoveItem::Property(lower_property_expression(&prop)?));
        } else if let Some(var) = item.oC_Variable() {
            let labels = item
                .oC_NodeLabels()
                .map(|labels| lower_node_labels(&labels))
                .unwrap_or_default();
            items.push(RemoveItem::Labels {
                name: lower_variable(&var),
                labels,
            });
        }
    }
    Ok(items)
}

// ---------------------------------------------------------------------------
// Procedure calls
// ---------------------------------------------------------------------------

fn procedure_name(ctx: &OC_ProcedureNameContext<'_>) -> String {
    let namespace = ctx
        .oC_Namespace()
        .map(|namespace| {
            let parts: Vec<String> = namespace
                .oC_SymbolicName_all()
                .iter()
                .map(|name| symbolic_name(name))
                .collect();
            if parts.is_empty() {
                String::new()
            } else {
                format!("{}.", parts.join("."))
            }
        })
        .unwrap_or_default();
    let name = ctx
        .oC_SymbolicName()
        .map(|name| symbolic_name(&name))
        .unwrap_or_default();
    format!("{namespace}{name}")
}

fn lower_call_arguments(
    exprs: &[Rc<OC_ExpressionContext<'_>>],
) -> LResult<Vec<(Option<String>, Expr)>> {
    let mut args = Vec::new();
    for expr in exprs {
        match lower_expression(expr)? {
            Expr::Binary(BinOp::Eq, lhs, rhs) => match *lhs {
                Expr::Var(name) => args.push((Some(name), *rhs)),
                other => args.push((None, Expr::Binary(BinOp::Eq, Box::new(other), rhs))),
            },
            other => args.push((None, other)),
        }
    }
    Ok(args)
}

fn lower_yields(ctx: &OC_YieldItemsContext<'_>) -> (Vec<(String, String)>, bool) {
    let mut yields = Vec::new();
    let star = !ctx.get_tokens(Cypher_T__4).is_empty();
    for item in ctx.oC_YieldItem_all() {
        let var = item
            .oC_Variable()
            .map(|v| lower_variable(&v))
            .unwrap_or_default();
        let field = item
            .oC_ProcedureResultField()
            .map(|f| {
                f.oC_SymbolicName()
                    .map(|n| symbolic_name(&n))
                    .unwrap_or_default()
            })
            .unwrap_or_else(|| var.clone());
        yields.push((field, var));
    }
    (yields, star)
}

fn lower_in_query_call(ctx: &OC_InQueryCallContext<'_>) -> LResult<CallClause> {
    let invocation = ctx
        .oC_ExplicitProcedureInvocation()
        .ok_or_else(|| unsupported("only explicit procedure invocations are supported"))?;
    let name = invocation
        .oC_ProcedureName()
        .map(|n| procedure_name(&n))
        .ok_or_else(|| parse_err("CALL without procedure name"))?;
    let args = lower_call_arguments(&invocation.oC_Expression_all())?;
    let (yields, star) = ctx
        .oC_YieldItems()
        .map(|items| lower_yields(&items))
        .unwrap_or_default();
    Ok(CallClause {
        name,
        args,
        yields,
        yield_star: star,
    })
}

fn lower_standalone_call(ctx: &OC_StandaloneCallContext<'_>) -> LResult<CallClause> {
    if let Some(invocation) = ctx.oC_ExplicitProcedureInvocation() {
        let name = invocation
            .oC_ProcedureName()
            .map(|n| procedure_name(&n))
            .ok_or_else(|| parse_err("CALL without procedure name"))?;
        let args = lower_call_arguments(&invocation.oC_Expression_all())?;
        let (yields, star) = ctx
            .oC_YieldItems()
            .map(|items| lower_yields(&items))
            .unwrap_or_default();
        return Ok(CallClause {
            name,
            args,
            yields,
            yield_star: star,
        });
    }
    if let Some(implicit) = ctx.oC_ImplicitProcedureInvocation() {
        let name = implicit
            .oC_ProcedureName()
            .map(|n| procedure_name(&n))
            .ok_or_else(|| parse_err("CALL without procedure name"))?;
        let (yields, star) = ctx
            .oC_YieldItems()
            .map(|items| lower_yields(&items))
            .unwrap_or_default();
        return Ok(CallClause {
            name,
            args: Vec::new(),
            yields,
            yield_star: star,
        });
    }
    Err(unsupported("procedure call form is not supported"))
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

fn lower_pattern(ctx: &OC_PatternContext<'_>) -> LResult<Vec<PatternPart>> {
    let mut parts = Vec::new();
    for part in ctx.oC_PatternPart_all() {
        parts.push(lower_pattern_part(&part)?);
    }
    Ok(parts)
}

fn lower_pattern_part(ctx: &OC_PatternPartContext<'_>) -> LResult<PatternPart> {
    let path_var = ctx.oC_Variable().map(|v| lower_variable(&v));
    let anonymous = ctx
        .oC_AnonymousPatternPart()
        .ok_or_else(|| parse_err("pattern part without element"))?;
    let element_ctx = anonymous
        .oC_PatternElement()
        .ok_or_else(|| parse_err("pattern part without element"))?;
    let element = lower_pattern_element(&element_ctx)?;
    Ok(PatternPart { path_var, element })
}

fn lower_pattern_element(ctx: &OC_PatternElementContext<'_>) -> LResult<PatternElement> {
    if let Some(nested) = ctx.oC_PatternElement() {
        return lower_pattern_element(&nested);
    }
    let node = ctx
        .oC_NodePattern()
        .ok_or_else(|| parse_err("pattern without node"))?;
    let anchor = lower_node_pattern(&node)?;
    let mut chains = Vec::new();
    for chain in ctx.oC_PatternElementChain_all() {
        let rel_ctx = chain
            .oC_RelationshipPattern()
            .ok_or_else(|| parse_err("pattern chain without relationship"))?;
        let rel = lower_relationship_pattern(&rel_ctx)?;
        let target_ctx = chain
            .oC_NodePattern()
            .ok_or_else(|| parse_err("pattern chain without target"))?;
        chains.push((rel, lower_node_pattern(&target_ctx)?));
    }
    Ok(PatternElement {
        node: anchor,
        chains,
    })
}

fn lower_node_pattern(ctx: &OC_NodePatternContext<'_>) -> LResult<NodePattern> {
    let var = ctx.oC_Variable().map(|v| lower_variable(&v));
    let labels = ctx
        .oC_NodeLabels()
        .map(|labels| lower_node_labels(&labels))
        .unwrap_or_default();
    let properties = ctx
        .oC_Properties()
        .map(|props| lower_properties(&props))
        .transpose()?
        .unwrap_or_default();
    Ok(NodePattern {
        var,
        labels,
        properties,
    })
}

fn lower_node_labels(ctx: &OC_NodeLabelsContext<'_>) -> Vec<String> {
    ctx.oC_NodeLabel_all()
        .iter()
        .filter_map(|label| label.oC_LabelName())
        .filter_map(|name| name.oC_SchemaName())
        .map(|name| schema_name(&name))
        .collect()
}

fn lower_relationship_pattern(ctx: &OC_RelationshipPatternContext<'_>) -> LResult<RelPattern> {
    let left = ctx.oC_LeftArrowHead().is_some();
    let right = ctx.oC_RightArrowHead().is_some();
    let direction = match (left, right) {
        (true, false) => Direction::Incoming,
        (false, true) => Direction::Outgoing,
        _ => Direction::Both,
    };
    let mut pattern = RelPattern {
        var: None,
        types: Vec::new(),
        direction,
        min_hops: 1,
        max_hops: Some(1),
        properties: Vec::new(),
    };
    if let Some(detail) = ctx.oC_RelationshipDetail() {
        pattern.var = detail.oC_Variable().map(|v| lower_variable(&v));
        if let Some(types) = detail.oC_RelationshipTypes() {
            pattern.types = types
                .oC_RelTypeName_all()
                .iter()
                .map(|name| {
                    name.oC_SchemaName()
                        .map(|n| schema_name(&n))
                        .unwrap_or_default()
                })
                .collect();
        }
        if let Some(range) = detail.oC_RangeLiteral() {
            let (min, max) = lower_range(&range);
            pattern.min_hops = min;
            pattern.max_hops = max;
        }
        if let Some(props) = detail.oC_Properties() {
            pattern.properties = lower_properties(&props)?;
        }
    }
    Ok(pattern)
}

fn lower_range(ctx: &OC_RangeLiteralContext<'_>) -> (i64, Option<i64>) {
    let raw = text(ctx);
    let raw = raw.trim().trim_start_matches('*').trim();
    if raw.is_empty() {
        return (1, None);
    }
    if let Some((left, right)) = raw.split_once("..") {
        let left = left.trim();
        let right = right.trim();
        let min = if left.is_empty() {
            1
        } else {
            left.parse::<i64>().unwrap_or(1)
        };
        let max = if right.is_empty() {
            None
        } else {
            right.parse::<i64>().ok()
        };
        return (min, max);
    }
    match raw.parse::<i64>() {
        Ok(value) => (value, Some(value)),
        Err(_) => (1, None),
    }
}

fn lower_properties(ctx: &OC_PropertiesContext<'_>) -> LResult<Vec<(String, Expr)>> {
    if let Some(map) = ctx.oC_MapLiteral() {
        return lower_map_literal(&map);
    }
    Err(unsupported(
        "parameterised patterns/properties are not part of the selected profile",
    ))
}

fn lower_map_literal(ctx: &OC_MapLiteralContext<'_>) -> LResult<Vec<(String, Expr)>> {
    let keys: Vec<String> = ctx
        .oC_PropertyKeyName_all()
        .iter()
        .map(|key| {
            key.oC_SchemaName()
                .map(|name| schema_name(&name))
                .unwrap_or_default()
        })
        .collect();
    let values: Vec<Rc<OC_ExpressionContext<'_>>> = ctx.oC_Expression_all();
    let mut map = Vec::new();
    for (key, value) in keys.into_iter().zip(values.iter()) {
        map.push((key, lower_expression(value)?));
    }
    Ok(map)
}

fn lower_variable(ctx: &OC_VariableContext<'_>) -> String {
    ctx.oC_SymbolicName()
        .map(|name| symbolic_name(&name))
        .unwrap_or_default()
}

fn lower_property_expression(ctx: &OC_PropertyExpressionContext<'_>) -> LResult<Expr> {
    let atom = ctx
        .oC_Atom()
        .ok_or_else(|| parse_err("property expression without atom"))?;
    let mut expr = lower_atom(&atom)?;
    for lookup in ctx.oC_PropertyLookup_all() {
        let key = lookup
            .oC_PropertyKeyName()
            .and_then(|name| name.oC_SchemaName())
            .map(|name| schema_name(&name))
            .ok_or_else(|| parse_err("property lookup without key"))?;
        expr = Expr::Prop(Box::new(expr), key);
    }
    Ok(expr)
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

pub fn lower_expression(ctx: &OC_ExpressionContext<'_>) -> LResult<Expr> {
    let or_expr = ctx
        .oC_OrExpression()
        .ok_or_else(|| parse_err("empty expression"))?;
    lower_or_expression(&or_expr)
}

fn lower_or_expression(ctx: &OC_OrExpressionContext<'_>) -> LResult<Expr> {
    fold_binary(
        ctx.oC_XorExpression_all()
            .iter()
            .map(|c| lower_xor_expression(c))
            .collect::<LResult<Vec<_>>>()?,
        ctx.OR_all().len(),
        BinOp::Or,
    )
}

fn lower_xor_expression(ctx: &OC_XorExpressionContext<'_>) -> LResult<Expr> {
    fold_binary(
        ctx.oC_AndExpression_all()
            .iter()
            .map(|c| lower_and_expression(c))
            .collect::<LResult<Vec<_>>>()?,
        ctx.XOR_all().len(),
        BinOp::Xor,
    )
}

fn lower_and_expression(ctx: &OC_AndExpressionContext<'_>) -> LResult<Expr> {
    fold_binary(
        ctx.oC_NotExpression_all()
            .iter()
            .map(|c| lower_not_expression(c))
            .collect::<LResult<Vec<_>>>()?,
        ctx.AND_all().len(),
        BinOp::And,
    )
}

/// Left-fold a homogeneous operator chain; the operator token count must be one
/// less than the operand count.
fn fold_binary(mut operands: Vec<Expr>, operators: usize, op: BinOp) -> LResult<Expr> {
    if operands.is_empty() {
        return Err(parse_err("empty binary expression"));
    }
    let expected = operands.len().saturating_sub(1);
    if operators != expected {
        // Defensive: some grammars allow a different grouping; fold what exists.
    }
    let mut acc = operands.remove(0);
    for operand in operands {
        acc = Expr::Binary(op.clone(), Box::new(acc), Box::new(operand));
    }
    Ok(acc)
}

fn lower_not_expression(ctx: &OC_NotExpressionContext<'_>) -> LResult<Expr> {
    // NOTE: the generated `NOT_all()` returns *every* terminal child (the
    // generated `*_all()` token accessors are not token-type filtered), so count
    // through `get_tokens`, which does filter.
    let count = ctx.get_tokens(Cypher_NOT).len();
    let inner = ctx
        .oC_ComparisonExpression()
        .ok_or_else(|| parse_err("NOT without expression"))?;
    let mut expr = lower_comparison_expression(&inner)?;
    for _ in 0..count {
        expr = Expr::Not(Box::new(expr));
    }
    Ok(expr)
}

fn lower_comparison_expression(ctx: &OC_ComparisonExpressionContext<'_>) -> LResult<Expr> {
    let mut acc = lower_add_or_subtract(
        ctx.oC_AddOrSubtractExpression()
            .ok_or_else(|| parse_err("comparison without left operand"))?
            .as_ref(),
    )?;
    for partial in ctx.oC_PartialComparisonExpression_all() {
        let op = comparison_operator(&partial)?;
        let rhs = lower_add_or_subtract(
            partial
                .oC_AddOrSubtractExpression()
                .ok_or_else(|| parse_err("comparison without right operand"))?
                .as_ref(),
        )?;
        acc = Expr::Binary(op, Box::new(acc), Box::new(rhs));
    }
    Ok(acc)
}

fn comparison_operator(ctx: &OC_PartialComparisonExpressionContext<'_>) -> LResult<BinOp> {
    let raw = text(ctx);
    let trimmed = raw.trim_start();
    for (prefix, op) in [
        ("<>", BinOp::Neq),
        ("<=", BinOp::Lte),
        (">=", BinOp::Gte),
        ("=", BinOp::Eq),
        ("<", BinOp::Lt),
        (">", BinOp::Gt),
    ] {
        if trimmed.starts_with(prefix) {
            return Ok(op);
        }
    }
    Err(parse_err("unknown comparison operator"))
}

/// Collect the operator token types of the given columns in document order.
macro_rules! ordered_operator_types {
    ($ctx:expr, $types:expr) => {{
        let mut tokens: Vec<(isize, i32)> = Vec::new();
        for token_type in $types {
            for node in $ctx.get_tokens(*token_type) {
                tokens.push((node.symbol.get_token_index(), *token_type));
            }
        }
        tokens.sort_by_key(|(index, _)| *index);
        tokens.into_iter().map(|(_, token_type)| token_type).collect::<Vec<i32>>()
    }};
}

fn lower_add_or_subtract(ctx: &OC_AddOrSubtractExpressionContext<'_>) -> LResult<Expr> {
    let operands: Vec<Expr> = ctx
        .oC_MultiplyDivideModuloExpression_all()
        .iter()
        .map(|c| lower_multiply_divide_modulo(c))
        .collect::<LResult<Vec<_>>>()?;
    let operators = ordered_operator_types!(ctx, &[Cypher_T__12, Cypher_T__13]);
    let mut acc = operands
        .first()
        .cloned()
        .ok_or_else(|| parse_err("empty additive expression"))?;
    for (index, operand) in operands.into_iter().skip(1).enumerate() {
        let op = if operators.get(index) == Some(&Cypher_T__13) {
            BinOp::Sub
        } else {
            BinOp::Add
        };
        acc = Expr::Binary(op, Box::new(acc), Box::new(operand));
    }
    Ok(acc)
}

fn lower_multiply_divide_modulo(
    ctx: &OC_MultiplyDivideModuloExpressionContext<'_>,
) -> LResult<Expr> {
    let operands: Vec<Expr> = ctx
        .oC_PowerOfExpression_all()
        .iter()
        .map(|c| lower_power_of(c))
        .collect::<LResult<Vec<_>>>()?;
    let operators = ordered_operator_types!(ctx, &[Cypher_T__4, Cypher_T__14, Cypher_T__15]);
    let mut acc = operands
        .first()
        .cloned()
        .ok_or_else(|| parse_err("empty multiplicative expression"))?;
    for (index, operand) in operands.into_iter().skip(1).enumerate() {
        let op = match operators.get(index) {
            Some(&Cypher_T__14) => BinOp::Div,
            Some(&Cypher_T__15) => BinOp::Mod,
            _ => BinOp::Mul,
        };
        acc = Expr::Binary(op, Box::new(acc), Box::new(operand));
    }
    Ok(acc)
}

fn lower_power_of(ctx: &OC_PowerOfExpressionContext<'_>) -> LResult<Expr> {
    let operands: Vec<Expr> = ctx
        .oC_UnaryAddOrSubtractExpression_all()
        .iter()
        .map(|c| lower_unary(&c))
        .collect::<LResult<Vec<_>>>()?;
    let mut acc = operands
        .first()
        .cloned()
        .ok_or_else(|| parse_err("empty power expression"))?;
    for operand in operands.into_iter().skip(1) {
        acc = Expr::Binary(BinOp::Pow, Box::new(acc), Box::new(operand));
    }
    Ok(acc)
}

fn lower_unary(ctx: &OC_UnaryAddOrSubtractExpressionContext<'_>) -> LResult<Expr> {
    let negatives = ctx.get_tokens(Cypher_T__13).len();
    let inner = ctx
        .oC_StringListNullOperatorExpression()
        .ok_or_else(|| parse_err("unary without operand"))?;
    let mut expr = lower_string_list_null(&inner)?;
    if negatives % 2 == 1 {
        expr = Expr::Neg(Box::new(expr));
    }
    Ok(expr)
}

fn lower_string_list_null(ctx: &OC_StringListNullOperatorExpressionContext<'_>) -> LResult<Expr> {
    let base = ctx
        .oC_PropertyOrLabelsExpression()
        .ok_or_else(|| parse_err("operator expression without base"))?;
    let mut acc = lower_property_or_labels(&base)?;

    // The grammar's three postfix groups interleave; reconstruct their order via
    // the child rule indices.
    let string_ops: Vec<_> = ctx.oC_StringOperatorExpression_all();
    let list_ops: Vec<_> = ctx.oC_ListOperatorExpression_all();
    let null_ops: Vec<_> = ctx.oC_NullOperatorExpression_all();
    let mut string_index = 0;
    let mut list_index = 0;
    let mut null_index = 0;
    for child in ctx.get_children() {
        match child.get_rule_index() {
            RULE_oC_StringOperatorExpression => {
                let op = &string_ops[string_index];
                string_index += 1;
                let rhs = op
                    .oC_PropertyOrLabelsExpression()
                    .ok_or_else(|| parse_err("string operator without operand"))?;
                let rhs = lower_property_or_labels(&rhs)?;
                let kind = if op.STARTS().is_some() {
                    BinOp::StartsWith
                } else if op.ENDS().is_some() {
                    BinOp::EndsWith
                } else {
                    BinOp::Contains
                };
                acc = Expr::Binary(kind, Box::new(acc), Box::new(rhs));
            }
            RULE_oC_ListOperatorExpression => {
                let op = &list_ops[list_index];
                list_index += 1;
                acc = lower_list_operator(&op, acc)?;
            }
            RULE_oC_NullOperatorExpression => {
                let op = &null_ops[null_index];
                null_index += 1;
                if op.NOT().is_some() {
                    acc = Expr::IsNotNull(Box::new(acc));
                } else {
                    acc = Expr::IsNull(Box::new(acc));
                }
            }
            _ => {}
        }
    }
    Ok(acc)
}

fn lower_list_operator(ctx: &OC_ListOperatorExpressionContext<'_>, base: Expr) -> LResult<Expr> {
    if let Some(rhs) = ctx.oC_PropertyOrLabelsExpression() {
        let rhs = lower_property_or_labels(&rhs)?;
        return Ok(Expr::Binary(BinOp::In, Box::new(base), Box::new(rhs)));
    }
    let exprs = ctx.oC_Expression_all();
    let raw = text(ctx);
    let has_dots = raw.contains("..");
    if !has_dots {
        let index = exprs
            .first()
            .ok_or_else(|| parse_err("list index without expression"))?;
        return Ok(Expr::ListIndex(
            Box::new(base),
            Box::new(lower_expression(index)?),
        ));
    }
    let after_open = raw.trim_start().trim_start_matches('[').to_string();
    let starts_with_dots = after_open.trim_start().starts_with("..");
    let before_close = after_open.trim_end().trim_end_matches(']').to_string();
    let ends_with_dots = before_close.trim_end().ends_with("..");
    let start = if starts_with_dots { None } else { exprs.first() };
    let end = if ends_with_dots {
        None
    } else if starts_with_dots {
        exprs.first()
    } else {
        exprs.get(1)
    };
    let start = match start {
        Some(expr) => Some(Box::new(lower_expression(expr)?)),
        None => None,
    };
    let end = match end {
        Some(expr) => Some(Box::new(lower_expression(expr)?)),
        None => None,
    };
    Ok(Expr::ListSlice {
        list: Box::new(base),
        start,
        end,
    })
}

fn lower_property_or_labels(ctx: &OC_PropertyOrLabelsExpressionContext<'_>) -> LResult<Expr> {
    let atom = ctx
        .oC_Atom()
        .ok_or_else(|| parse_err("expression without atom"))?;
    let mut expr = lower_atom(&atom)?;
    for lookup in ctx.oC_PropertyLookup_all() {
        let key = lookup
            .oC_PropertyKeyName()
            .and_then(|name| name.oC_SchemaName())
            .map(|name| schema_name(&name))
            .ok_or_else(|| parse_err("property lookup without key"))?;
        expr = Expr::Prop(Box::new(expr), key);
    }
    if let Some(labels) = ctx.oC_NodeLabels() {
        let names = lower_node_labels(&labels);
        if !names.is_empty() {
            expr = Expr::LabelTest(Box::new(expr), names);
        }
    }
    Ok(expr)
}

fn lower_atom(ctx: &OC_AtomContext<'_>) -> LResult<Expr> {
    if let Some(literal) = ctx.oC_Literal() {
        return lower_literal(&literal);
    }
    if let Some(param) = ctx.oC_Parameter() {
        return lower_parameter(&param);
    }
    if let Some(case) = ctx.oC_CaseExpression() {
        return lower_case(&case);
    }
    if ctx.COUNT().is_some() {
        return Ok(Expr::CountStar);
    }
    if let Some(comprehension) = ctx.oC_ListComprehension() {
        return lower_list_comprehension(&comprehension);
    }
    if ctx.ALL().is_some() || ctx.ANY().is_some() || ctx.NONE().is_some() || ctx.SINGLE().is_some() {
        let kind = if ctx.ALL().is_some() {
            QuantKind::All
        } else if ctx.ANY().is_some() {
            QuantKind::Any
        } else if ctx.NONE().is_some() {
            QuantKind::None
        } else {
            QuantKind::Single
        };
        let filter = ctx
            .oC_FilterExpression()
            .ok_or_else(|| parse_err("quantifier without filter"))?;
        return lower_quantifier(kind, &filter);
    }
    if let Some(paren) = ctx.oC_ParenthesizedExpression() {
        let inner = paren
            .oC_Expression()
            .ok_or_else(|| parse_err("empty parentheses"))?;
        return lower_expression(&inner);
    }
    if let Some(function) = ctx.oC_FunctionInvocation() {
        return lower_function(&function);
    }
    if let Some(variable) = ctx.oC_Variable() {
        return Ok(Expr::Var(lower_variable(&variable)));
    }
    if ctx.oC_RelationshipsPattern().is_some() || ctx.oC_PatternComprehension().is_some() {
        return Err(unsupported(
            "pattern-valued expressions are outside the selected profile",
        ));
    }
    Err(unsupported("expression atom is not supported"))
}

fn lower_literal(ctx: &OC_LiteralContext<'_>) -> LResult<Expr> {
    if let Some(number) = ctx.oC_NumberLiteral() {
        if let Some(double) = number.oC_DoubleLiteral() {
            let raw = text(double.as_ref());
            return raw
                .trim()
                .parse::<f64>()
                .map(Expr::Float)
                .map_err(|e| parse_err(format!("bad float `{raw}`: {e}")));
        }
        if let Some(integer) = number.oC_IntegerLiteral() {
            return Ok(Expr::Int(integer_literal(&integer)?));
        }
        return Err(parse_err("number literal without value"));
    }
    if let Some(string) = ctx.StringLiteral() {
        return Ok(Expr::Str(string_literal(&string)));
    }
    if let Some(boolean) = ctx.oC_BooleanLiteral() {
        return Ok(Expr::Bool(boolean.TRUE().is_some()));
    }
    if ctx.NULL().is_some() {
        return Ok(Expr::Null);
    }
    if let Some(map) = ctx.oC_MapLiteral() {
        return Ok(Expr::Map(lower_map_literal(&map)?));
    }
    if let Some(list) = ctx.oC_ListLiteral() {
        let items: Vec<Expr> = list
            .oC_Expression_all()
            .iter()
            .map(|expr| lower_expression(expr))
            .collect::<LResult<Vec<_>>>()?;
        return Ok(Expr::List(items));
    }
    Err(parse_err("literal form is not supported"))
}

fn lower_parameter(ctx: &OC_ParameterContext<'_>) -> LResult<Expr> {
    if let Some(name) = ctx.oC_SymbolicName() {
        return Ok(Expr::Parameter(symbolic_name(&name)));
    }
    if let Some(digit) = ctx.DecimalInteger() {
        return Ok(Expr::Parameter(text(digit.as_ref()).trim().to_string()));
    }
    Err(parse_err("parameter without name"))
}

fn lower_case(ctx: &OC_CaseExpressionContext<'_>) -> LResult<Expr> {
    // The WHEN/THEN expressions live inside oC_CaseAlternatives children; only
    // the optional test expression and the optional ELSE expression are direct
    // children of the CASE rule.
    let alternatives = ctx.oC_CaseAlternatives_all();
    let direct: Vec<Rc<OC_ExpressionContext<'_>>> = ctx.oC_Expression_all();
    let has_else = ctx.ELSE().is_some();
    let (test, else_branch) = if has_else {
        let else_expr = direct.last().cloned();
        let test = if direct.len() >= 2 {
            direct.first().cloned()
        } else {
            None
        };
        (test, else_expr)
    } else {
        (direct.first().cloned(), None)
    };
    let test = match test {
        Some(expr) => Some(Box::new(lower_expression(&expr)?)),
        None => None,
    };
    let mut whens = Vec::new();
    for alternative in &alternatives {
        let expressions = alternative.oC_Expression_all();
        if expressions.len() < 2 {
            return Err(parse_err("CASE alternative without WHEN and THEN"));
        }
        let when = lower_expression(&expressions[0])?;
        let then = lower_expression(&expressions[1])?;
        whens.push((when, then));
    }
    let else_branch = match else_branch {
        Some(expr) => Some(Box::new(lower_expression(&expr)?)),
        None => None,
    };
    Ok(Expr::Case {
        test,
        whens,
        else_branch,
    })
}

fn lower_list_comprehension(ctx: &OC_ListComprehensionContext<'_>) -> LResult<Expr> {
    let filter = ctx
        .oC_FilterExpression()
        .ok_or_else(|| parse_err("comprehension without filter"))?;
    let id_in_coll = filter
        .oC_IdInColl()
        .ok_or_else(|| parse_err("comprehension without variable"))?;
    let var = id_in_coll
        .oC_Variable()
        .map(|v| lower_variable(&v))
        .unwrap_or_default();
    let list = lower_expression(
        id_in_coll
            .oC_Expression()
            .ok_or_else(|| parse_err("comprehension without list"))?
            .as_ref(),
    )?;
    let predicate = match filter.oC_Where() {
        Some(w) => Some(Box::new(lower_where(&w)?)),
        None => None,
    };
    let map = match ctx.oC_Expression() {
        Some(expr) => Some(Box::new(lower_expression(&expr)?)),
        None => None,
    };
    Ok(Expr::ListComprehension {
        var,
        list: Box::new(list),
        filter: predicate,
        map,
    })
}

fn lower_quantifier(kind: QuantKind, filter: &OC_FilterExpressionContext<'_>) -> LResult<Expr> {
    let id_in_coll = filter
        .oC_IdInColl()
        .ok_or_else(|| parse_err("quantifier without variable"))?;
    let var = id_in_coll
        .oC_Variable()
        .map(|v| lower_variable(&v))
        .unwrap_or_default();
    let list = lower_expression(
        id_in_coll
            .oC_Expression()
            .ok_or_else(|| parse_err("quantifier without list"))?
            .as_ref(),
    )?;
    let predicate = match filter.oC_Where() {
        Some(w) => Some(Box::new(lower_where(&w)?)),
        None => None,
    };
    Ok(Expr::Quantifier {
        kind,
        var,
        list: Box::new(list),
        predicate,
    })
}

fn lower_function(ctx: &OC_FunctionInvocationContext<'_>) -> LResult<Expr> {
    let name_ctx = ctx
        .oC_FunctionName()
        .ok_or_else(|| parse_err("function invocation without name"))?;
    let name = if name_ctx.EXISTS().is_some() {
        "exists".to_string()
    } else {
        let namespace = name_ctx
            .oC_Namespace()
            .map(|namespace| {
                let parts: Vec<String> = namespace
                    .oC_SymbolicName_all()
                    .iter()
                    .map(|name| symbolic_name(name))
                    .collect();
                if parts.is_empty() {
                    String::new()
                } else {
                    format!("{}.", parts.join("."))
                }
            })
            .unwrap_or_default();
        let base = name_ctx
            .oC_SymbolicName()
            .map(|name| symbolic_name(&name))
            .unwrap_or_default();
        format!("{namespace}{base}")
    };
    let args: Vec<Expr> = ctx
        .oC_Expression_all()
        .iter()
        .map(|expr| lower_expression(expr))
        .collect::<LResult<Vec<_>>>()?;
    Ok(Expr::Function {
        name: name.to_ascii_lowercase(),
        distinct: ctx.DISTINCT().is_some(),
        args,
    })
}
