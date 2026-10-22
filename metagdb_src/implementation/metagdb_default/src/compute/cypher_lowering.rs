#![allow(dead_code)]
#![allow(unused)]
/**
 * @brief: CST -> AST Lowering function module.
 *
 * Translates the ANTLR-generated Cypher CST to CypherAst. Lowering functions are split by clause type,
 * each function is responsible for extracting semantic information from the corresponding ANTLR Context and constructing AST nodes.
 */

use std::collections::BTreeMap;

use antlr4rust::tree::ParseTree;
use metagdb_core::common::property::PropertyValue;
use metagdb_core::{Error, Result};

use metagdb_cypher_antlr::cypherparser::*;

use super::cypher_ast::{
    AstExpr, CallSubqueryClause, Clause, CreateClause, CypherAst, DeleteClause, EdgeDirection,
    MatchClause, MatchPattern, MergeAction, MergeClause, NodePattern, OrderBy, PatternEdge,
    PatternElement, Query, RangeLiteral, RemoveClause, RemoveItem, ReturnClause, ReturnItem,
    SetClause, SetItem, SortItem, SortOrder, UnionBranch, UnionClause, UnwindClause, WithClause,
};

// ============================================================
// Top-level lowering
// ============================================================

/**
 * @brief: Lower from oC_Cypher context to get CypherAst.
 * @param ctx: ANTLR-parsed oC_Cypher context.
 * @return: Returns CypherAst on success.
 */
pub fn lower_cypher(ctx: &OC_CypherContext) -> Result<CypherAst> {
    let stmt = ctx
        .oC_Statement()
        .ok_or_else(|| Error::Parse("No valid statement in query".into()))?;
    lower_statement(&stmt)
}

/**
 * @brief: Lower from oC_Statement context to get CypherAst.
 *
 * Routes to Query or StandaloneCall branch.
 */
pub fn lower_statement(ctx: &OC_StatementContext) -> Result<CypherAst> {
    if let Some(query) = ctx.oC_Query() {
        lower_query(query.as_ref())
    } else {
        Err(Error::Parse("Statement type not supported".into()))
    }
}

/**
 * @brief: Lower from oC_Query context, routing to RegularQuery or single statement.
 */
pub fn lower_query(ctx: &OC_QueryContext) -> Result<CypherAst> {
    if let Some(rq) = ctx.oC_RegularQuery() {
        lower_regular_query(rq.as_ref())
    } else if let Some(call) = ctx.oC_StandaloneCall() {
        lower_standalone_call(call.as_ref())
    } else {
        Err(Error::Parse("Only RegularQuery or StandaloneCall supported".into()))
    }
}

/**
 * @brief: Lower from oC_RegularQuery context.
 *
 * RegularQuery = SingleQuery (UNION [ALL] SingleQuery)*
 */
pub fn lower_regular_query(ctx: &OC_RegularQueryContext) -> Result<CypherAst> {
    let sq = ctx
        .oC_SingleQuery()
        .ok_or_else(|| Error::Parse("RegularQuery is missing SingleQuery".into()))?;
    let main_query = lower_single_query(&sq)?;

    let unions = ctx.oC_Union_all();
    if unions.is_empty() {
        return Ok(CypherAst::Query(Box::new(main_query)));
    }

    // Record the ALL/DISTINCT flag for each UNION branch
    let mut branches = Vec::with_capacity(unions.len());
    for union_ctx in &unions {
        let all = union_ctx.ALL().is_some();
        let uq = union_ctx
            .oC_SingleQuery()
            .ok_or_else(|| Error::Parse("UNION is missing SingleQuery".into()))?;
        let branch_query = lower_single_query(&uq)?;
        branches.push(UnionBranch {
            all,
            query: branch_query,
        });
    }

    let mut q = main_query;
    q.unions = Some(UnionClause { branches });

    Ok(CypherAst::Query(Box::new(q)))
}

/**
 * @brief: Lower from oC_SingleQuery context, routing to SinglePartQuery or MultiPartQuery.
 */
pub fn lower_single_query(ctx: &OC_SingleQueryContext) -> Result<Query> {
    if let Some(spq) = ctx.oC_SinglePartQuery() {
        lower_single_part_query(&spq)
    } else if let Some(mpq) = ctx.oC_MultiPartQuery() {
        lower_multi_part_query(&mpq)
    } else {
        Err(Error::Parse("SingleQuery is missing a valid query body".into()))
    }
}

// ============================================================
// SinglePartQuery lowering
// ============================================================

/**
 * @brief: Lower from oC_SinglePartQuery context.
 *
 * SinglePartQuery = ReadingClause* (UpdatingClause)* Return?
 */
pub fn lower_single_part_query(ctx: &OC_SinglePartQueryContext) -> Result<Query> {
    let mut clauses: Vec<Clause> = Vec::new();

    // ReadingClauses
    for rc_ctx in &ctx.oC_ReadingClause_all() {
        if let Some(match_ctx) = rc_ctx.oC_Match() {
            let mc = lower_match(&match_ctx, false)?;
            clauses.push(Clause::Match(mc));
        } else if let Some(unwind_ctx) = rc_ctx.oC_Unwind() {
            let uc = lower_unwind(&unwind_ctx)?;
            clauses.push(Clause::Unwind(uc));
        } else if let Some(call_ctx) = rc_ctx.oC_InQueryCall() {
            let cc = lower_in_query_call(call_ctx.as_ref())?;
            clauses.push(Clause::CallSubquery(cc));
        }
    }

    // UpdatingClauses
    for uc_ctx in &ctx.oC_UpdatingClause_all() {
        if let Some(clause) = lower_updating_clause(uc_ctx)? {
            clauses.push(clause);
        }
    }

    // RETURN
    if let Some(ret_ctx) = ctx.oC_Return() {
        let rc = lower_return(&ret_ctx)?;
        clauses.push(Clause::Return(rc));
    }

    if clauses.is_empty() {
        return Err(Error::Parse("No valid clauses in the query".into()));
    }

    Ok(Query {
        clauses,
        unions: None,
    })
}

// ============================================================
// MultiPartQuery lowering
// ============================================================

/**
 * @brief: Lower from oC_MultiPartQuery context.
 *
 * MultiPartQuery = ReadingClause* (UpdatingClause)* With ... (ReadingClause* (UpdatingClause)* Return)?
 */
pub fn lower_multi_part_query(ctx: &OC_MultiPartQueryContext) -> Result<Query> {
    let mut clauses: Vec<Clause> = Vec::new();

    // Read ReadingClauses from the first code block
    let reading_clauses = ctx.oC_ReadingClause_all();
    for rc_ctx in &reading_clauses {
        if let Some(match_ctx) = rc_ctx.oC_Match() {
            let mc = lower_match(&match_ctx, false)?;
            clauses.push(Clause::Match(mc));
        } else if let Some(unwind_ctx) = rc_ctx.oC_Unwind() {
            let uc = lower_unwind(&unwind_ctx)?;
            clauses.push(Clause::Unwind(uc));
        } else if let Some(call_ctx) = rc_ctx.oC_InQueryCall() {
            let cc = lower_in_query_call(call_ctx.as_ref())?;
            clauses.push(Clause::CallSubquery(cc));
        }
    }

    let updating_clauses = ctx.oC_UpdatingClause_all();
    for uc_ctx in &updating_clauses {
        if let Some(clause) = lower_updating_clause(uc_ctx)? {
            clauses.push(clause);
        }
    }

    // Process WITH (at least one WITH in MultiPartQuery)
    let with_ctxs = ctx.oC_With_all();
    for (i, with_ctx) in with_ctxs.iter().enumerate() {
        let wc = lower_with(with_ctx)?;
        clauses.push(Clause::With(wc));

        // Each WITH may be followed by ReadingClause* + UpdatingClause*
        // In the ANTLR parse tree these follow within the same MultiPartQuery,
        // but here we simplify to a flat clause list.
        // Subsequent ReadingClauses and UpdatingClauses have been collected in the first iteration,
        // here only using with as a separator.
        if i == with_ctxs.len() - 1 {
            // After the last with: if there is a SinglePartQuery, extract RETURN from it
            if let Some(spq) = ctx.oC_SinglePartQuery() {
                if let Some(ret_ctx) = spq.oC_Return() {
                    let rc = lower_return(&ret_ctx)?;
                    clauses.push(Clause::Return(rc));
                }
            }
        }
    }

    Ok(Query {
        clauses,
        unions: None,
    })
}

// ============================================================
// UpdatingClause dispatch
// ============================================================

/**
 * @brief: Lower from oC_UpdatingClause context.
 */
fn lower_updating_clause(ctx: &OC_UpdatingClauseContext) -> Result<Option<Clause>> {
    if let Some(create_ctx) = ctx.oC_Create() {
        let cc = lower_create(&create_ctx)?;
        Ok(Some(Clause::Create(cc)))
    } else if let Some(merge_ctx) = ctx.oC_Merge() {
        let mc = lower_merge(&merge_ctx)?;
        Ok(Some(Clause::Merge(mc)))
    } else if let Some(delete_ctx) = ctx.oC_Delete() {
        let dc = lower_delete(&delete_ctx)?;
        Ok(Some(Clause::Delete(dc)))
    } else if let Some(set_ctx) = ctx.oC_Set() {
        let sc = lower_set(&set_ctx)?;
        Ok(Some(Clause::Set(sc)))
    } else if let Some(remove_ctx) = ctx.oC_Remove() {
        let rc = lower_remove(&remove_ctx)?;
        Ok(Some(Clause::Remove(rc)))
    } else {
        Ok(None)
    }
}

// ============================================================
// MATCH lowering
// ============================================================

/**
 * @brief: Lower from oC_Match context.
 * @param ctx: ANTLR Match context.
 * @param optional: Whether it is OPTIONAL MATCH.
 */
pub fn lower_match(ctx: &OC_MatchContext, _optional: bool) -> Result<MatchClause> {
    let pattern_ctx = ctx
        .oC_Pattern()
        .ok_or_else(|| Error::Parse("MATCH is missing pattern".into()))?;
    let pattern = lower_pattern(&pattern_ctx)?;

    let where_clause = if let Some(where_ctx) = ctx.oC_Where() {
        let expr_ctx = where_ctx
            .oC_Expression()
            .ok_or_else(|| Error::Parse("WHERE is missing expression".into()))?;
        Some(lower_expression(&expr_ctx)?)
    } else {
        None
    };

    Ok(MatchClause {
        pattern,
        where_clause,
    })
}

// ============================================================
// Pattern lowering
// ============================================================

/**
 * @brief: Lower from oC_Pattern context, extract all PatternParts to construct MatchPattern.
 */
pub fn lower_pattern(ctx: &OC_PatternContext) -> Result<MatchPattern> {
    let parts = ctx.oC_PatternPart_all();
    if parts.is_empty() {
        return Err(Error::Parse("Pattern is missing path elements".into()));
    }

    let mut elements = Vec::with_capacity(parts.len());
    for part in &parts {
        let pe = lower_pattern_part(part)?;
        elements.push(pe);
    }

    Ok(MatchPattern { elements })
}

/**
 * @brief: Lower from oC_PatternPart context.
 *
 * PatternPart = (Variable '=')? AnonymousPatternPart
 */
pub fn lower_pattern_part(ctx: &OC_PatternPartContext) -> Result<PatternElement> {
    let anon = ctx
        .oC_AnonymousPatternPart()
        .ok_or_else(|| Error::Parse("Only anonymous patterns supported (path = (...) syntax not supported)".into()))?;
    let elem_ctx = anon
        .oC_PatternElement()
        .ok_or_else(|| Error::Parse("AnonymousPatternPart is missing PatternElement".into()))?;

    lower_pattern_element(&elem_ctx)
}

/**
 * @brief: Lower from oC_PatternElement context.
 *
 * PatternElement = NodePattern PatternElementChain*
 */
pub fn lower_pattern_element(ctx: &OC_PatternElementContext) -> Result<PatternElement> {
    let node_ctx = ctx
        .oC_NodePattern()
        .ok_or_else(|| Error::Parse("Pattern is missing vertex pattern".into()))?;
    let node = lower_node_pattern(&node_ctx)?;

    let chains = ctx.oC_PatternElementChain_all();
    let mut edges = Vec::with_capacity(chains.len());

    for chain in &chains {
        let edge = lower_pattern_element_chain(chain)?;
        edges.push(edge);
    }

    Ok(PatternElement { node, edges })
}

/**
 * @brief: Lower vertex pattern from oC_NodePattern context.
 */
pub fn lower_node_pattern(ctx: &OC_NodePatternContext) -> Result<NodePattern> {
    let var = if let Some(v_ctx) = ctx.oC_Variable() {
        Some(extract_variable_name(&v_ctx)?)
    } else {
        None
    };

    let label = if let Some(labels_ctx) = ctx.oC_NodeLabels() {
        let node_labels = labels_ctx.oC_NodeLabel_all();
        match node_labels.first() {
            Some(first) => Some(extract_label_name(first)?),
            None => None,
        }
    } else {
        None
    };

    let properties = if let Some(props_ctx) = ctx.oC_Properties() {
        Some(lower_properties(&props_ctx)?)
    } else {
        None
    };

    Ok(NodePattern {
        var,
        label,
        properties,
    })
}

/**
 * @brief: Lower edge + target vertex from oC_PatternElementChain context.
 */
pub fn lower_pattern_element_chain(ctx: &OC_PatternElementChainContext) -> Result<PatternEdge> {
    let rel_ctx = ctx
        .oC_RelationshipPattern()
        .ok_or_else(|| Error::Parse("Edge chain is missing relationship pattern".into()))?;
    let (edge_var, edge_types, direction, range, properties) =
        lower_relationship_pattern(&rel_ctx)?;

    let node_ctx = ctx
        .oC_NodePattern()
        .ok_or_else(|| Error::Parse("Edge chain is missing target vertex".into()))?;
    let target = lower_node_pattern(&node_ctx)?;

    Ok(PatternEdge {
        edge_var,
        edge_types,
        direction,
        range,
        properties,
        target,
    })
}

/**
 * @brief: Lower from oC_RelationshipPattern context.
 */
pub fn lower_relationship_pattern(
    ctx: &OC_RelationshipPatternContext,
) -> Result<(
    Option<String>,
    Option<Vec<String>>,
    EdgeDirection,
    Option<RangeLiteral>,
    Option<BTreeMap<String, AstExpr>>,
)> {
    let has_left = ctx.oC_LeftArrowHead().is_some();
    let has_right = ctx.oC_RightArrowHead().is_some();

    let direction = match (has_left, has_right) {
        (true, false) => EdgeDirection::Incoming,
        (false, true) => EdgeDirection::Outgoing,
        (true, true) => EdgeDirection::Both,
        (false, false) => EdgeDirection::Both, // No arrow, plain dash connection
    };

    if let Some(detail) = ctx.oC_RelationshipDetail() {
        let edge_var = if let Some(v_ctx) = detail.oC_Variable() {
            Some(extract_variable_name(&v_ctx)?)
        } else {
            None
        };

        let edge_types = if let Some(types_ctx) = detail.oC_RelationshipTypes() {
            let names = types_ctx.oC_RelTypeName_all();
            if names.is_empty() {
                None
            } else {
                let mut type_names = Vec::with_capacity(names.len());
                for name_ctx in &names {
                    type_names.push(extract_schema_name_text(name_ctx)?);
                }
                Some(type_names)
            }
        } else {
            None
        };

        let range = if let Some(rl_ctx) = detail.oC_RangeLiteral() {
            Some(lower_range_literal(&rl_ctx)?)
        } else {
            None
        };

        let properties = if let Some(props_ctx) = detail.oC_Properties() {
            Some(lower_properties(&props_ctx)?)
        } else {
            None
        };

        Ok((edge_var, edge_types, direction, range, properties))
    } else {
        Ok((None, None, direction, None, None))
    }
}

/**
 * @brief: Lower variable-length path range from oC_RangeLiteral context.
 */
fn lower_range_literal(ctx: &OC_RangeLiteralContext) -> Result<RangeLiteral> {
    let text = ctx.get_text();
    // Form like "*1..3", "*..3", "*1..", "*"
    let inner = if text.starts_with('*') { &text[1..] } else { &text };

    if inner.is_empty() {
        return Ok(RangeLiteral {
            start: None,
            end: None,
        });
    }

    let parts: Vec<&str> = inner.split("..").collect();
    let start = if parts.is_empty() || parts[0].is_empty() {
        None
    } else {
        parts[0].parse::<i64>().ok()
    };

    let end = if parts.len() > 1 && !parts[1].is_empty() {
        parts[1].parse::<i64>().ok()
    } else if parts.len() == 1 && !parts[0].is_empty() {
        // Exact hop count "*3"
        parts[0].parse::<i64>().ok()
    } else {
        None
    };

    Ok(RangeLiteral { start, end })
}

// ============================================================
// Properties (For inline properties on nodes/edges)
// ============================================================

/**
 * @brief: Lower property map from oC_Properties context.
 *
 * Properties = MapLiteral | Parameter
 * Returns property key-value pairs, or None for unsupported types.
 */
fn lower_properties(ctx: &OC_PropertiesContext) -> Result<BTreeMap<String, AstExpr>> {
    if let Some(map_literal) = ctx.oC_MapLiteral() {
        return match lower_map_literal(map_literal.as_ref())? {
            AstExpr::Map(entries) => Ok(entries.into_iter().collect()),
            _ => Ok(BTreeMap::new()),
        };
    }
    if let Some(param) = ctx.oC_Parameter() {
        let text = param.get_text();
        let name = if text.starts_with('$') {
            text[1..].to_string()
        } else {
            text
        };
        // Properties from parameter reference: marked with special expression
        let mut map = BTreeMap::new();
        map.insert("$param".into(), AstExpr::Parameter(name));
        return Ok(map);
    }
    Ok(BTreeMap::new())
}

// ============================================================
// WITH lowering
// ============================================================

/**
 * @brief: Lower from oC_With context.
 */
pub fn lower_with(ctx: &OC_WithContext) -> Result<WithClause> {
    let body_ctx = ctx
        .oC_ProjectionBody()
        .ok_or_else(|| Error::Parse("WITH is missing projection body".into()))?;
    let (distinct, items, order_by, skip, limit) = lower_projection_body(&body_ctx)?;

    let where_clause = if let Some(where_ctx) = ctx.oC_Where() {
        let expr_ctx = where_ctx
            .oC_Expression()
            .ok_or_else(|| Error::Parse("WITH ... WHERE is missing expression".into()))?;
        Some(lower_expression(&expr_ctx)?)
    } else {
        None
    };

    Ok(WithClause {
        distinct,
        items,
        where_clause,
        order_by,
        skip,
        limit,
    })
}

// ============================================================
// RETURN lowering
// ============================================================

/**
 * @brief: Lower from oC_Return context.
 */
pub fn lower_return(ctx: &OC_ReturnContext) -> Result<ReturnClause> {
    let body_ctx = ctx
        .oC_ProjectionBody()
        .ok_or_else(|| Error::Parse("RETURN is missing projection body".into()))?;
    let (distinct, items, order_by, skip, limit) = lower_projection_body(&body_ctx)?;

    Ok(ReturnClause {
        distinct,
        items,
        order_by,
        skip,
        limit,
    })
}

/**
 * @brief: Lower from oC_ProjectionBody context, extracting projection items, ordering, and pagination.
 */
fn lower_projection_body(
    ctx: &OC_ProjectionBodyContext,
) -> Result<(
    bool,
    Vec<ReturnItem>,
    Option<OrderBy>,
    Option<AstExpr>,
    Option<AstExpr>,
)> {
    let distinct = ctx.DISTINCT().is_some();
    let items = if let Some(items_ctx) = ctx.oC_ProjectionItems() {
        lower_projection_items(&items_ctx)?
    } else {
        Vec::new()
    };

    let order_by = if let Some(order_ctx) = ctx.oC_Order() {
        Some(lower_order(&order_ctx)?)
    } else {
        None
    };

    let skip = if let Some(skip_ctx) = ctx.oC_Skip() {
        Some(lower_skip(&skip_ctx)?)
    } else {
        None
    };

    let limit = if let Some(limit_ctx) = ctx.oC_Limit() {
        Some(lower_limit(&limit_ctx)?)
    } else {
        None
    };

    Ok((distinct, items, order_by, skip, limit))
}

/**
 * @brief: Lower projection item list from oC_ProjectionItems context.
 */
fn lower_projection_items(ctx: &OC_ProjectionItemsContext) -> Result<Vec<ReturnItem>> {
    let proj_items = ctx.oC_ProjectionItem_all();
    if proj_items.is_empty() {
        return Err(Error::Parse("Projection item list is empty".into()));
    }

    let mut ret = Vec::with_capacity(proj_items.len());
    for item_ctx in &proj_items {
        let item = lower_projection_item(item_ctx)?;
        ret.push(item);
    }

    Ok(ret)
}

/**
 * @brief: Lower a single projection item from oC_ProjectionItem context.
 *
 * ProjectionItem = Expression (AS variable)?
 */
fn lower_projection_item(ctx: &OC_ProjectionItemContext) -> Result<ReturnItem> {
    let expr_ctx = ctx
        .oC_Expression()
        .ok_or_else(|| Error::Parse("Projection item is missing expression".into()))?;
    let expr = lower_expression(&expr_ctx)?;

    // Check if there is an AS alias
    if let Some(var_ctx) = ctx.oC_Variable() {
        let alias = extract_variable_name(&var_ctx)?;

        // If the expression itself is a simple variable reference with the same alias, deduplicate
        if let AstExpr::Var(ref v) = expr {
            if v == &alias {
                return Ok(ReturnItem::Var(alias));
            }
        }
        // If the expression is a property reference var.key, substitute with alias
        if let AstExpr::Prop { ref var, ref key } = expr {
            let inferred = format!("{var}.{key}");
            if inferred == alias {
                return Ok(ReturnItem::Property {
                    var: var.clone(),
                    key: key.clone(),
                });
            }
        }

        return Ok(ReturnItem::Alias {
            expr: Box::new(expr),
            alias,
        });
    }

    // No alias: infer from expression form
    match &expr {
        AstExpr::Var(name) => Ok(ReturnItem::Var(name.clone())),
        AstExpr::Prop { var, key } => Ok(ReturnItem::Property {
            var: var.clone(),
            key: key.clone(),
        }),
        _ => Ok(ReturnItem::Expression(expr)),
    }
}

/**
 * @brief: Lower ORDER BY from oC_Order context.
 */
fn lower_order(ctx: &OC_OrderContext) -> Result<OrderBy> {
    let sort_items = ctx.oC_SortItem_all();
    let mut items = Vec::with_capacity(sort_items.len());
    for si_ctx in &sort_items {
        let expr_ctx = si_ctx
            .oC_Expression()
            .ok_or_else(|| Error::Parse("ORDER BY item is missing expression".into()))?;
        let expr = lower_expression(&expr_ctx)?;

        let order = if si_ctx.DESC().is_some() || si_ctx.DESCENDING().is_some() {
            Some(SortOrder::Desc)
        } else {
            None // Default ascending
        };

        items.push(SortItem { expr, order });
    }
    Ok(OrderBy { items })
}

/**
 * @brief: Lower SKIP from oC_Skip context.
 */
fn lower_skip(ctx: &OC_SkipContext) -> Result<AstExpr> {
    let expr_ctx = ctx
        .oC_Expression()
        .ok_or_else(|| Error::Parse("SKIP is missing expression".into()))?;
    lower_expression(&expr_ctx)
}

/**
 * @brief: Lower LIMIT from oC_Limit context.
 */
fn lower_limit(ctx: &OC_LimitContext) -> Result<AstExpr> {
    let expr_ctx = ctx
        .oC_Expression()
        .ok_or_else(|| Error::Parse("LIMIT is missing expression".into()))?;
    lower_expression(&expr_ctx)
}

// ============================================================
// UNWIND lowering
// ============================================================

/**
 * @brief: Lower from oC_Unwind context.
 */
fn lower_unwind(ctx: &OC_UnwindContext) -> Result<UnwindClause> {
    let expr_ctx = ctx
        .oC_Expression()
        .ok_or_else(|| Error::Parse("UNWIND is missing expression".into()))?;
    let expr = lower_expression(&expr_ctx)?;

    let var_ctx = ctx
        .oC_Variable()
        .ok_or_else(|| Error::Parse("UNWIND is missing variable binding".into()))?;
    let var = extract_variable_name(&var_ctx)?;

    Ok(UnwindClause { expr, var })
}

// ============================================================
// CREATE lowering
// ============================================================

/**
 * @brief: Lower from oC_Create context.
 */
fn lower_create(ctx: &OC_CreateContext) -> Result<CreateClause> {
    let pattern_ctx = ctx
        .oC_Pattern()
        .ok_or_else(|| Error::Parse("CREATE is missing pattern".into()))?;
    let pattern = lower_pattern(&pattern_ctx)?;
    Ok(CreateClause { pattern })
}

// ============================================================
// MERGE lowering
// ============================================================

/**
 * @brief: Lower from oC_Merge context.
 */
fn lower_merge(ctx: &OC_MergeContext) -> Result<MergeClause> {
    let part_ctx = ctx
        .oC_PatternPart()
        .ok_or_else(|| Error::Parse("MERGE is missing pattern".into()))?;
    let pe = lower_pattern_part(&part_ctx)?;
    let pattern = MatchPattern {
        elements: vec![pe],
    };

    let mut actions = Vec::new();
    for action_ctx in &ctx.oC_MergeAction_all() {
        let is_on_create = action_ctx.CREATE().is_some();
        let set_ctx = action_ctx
            .oC_Set()
            .ok_or_else(|| Error::Parse("MERGE action is missing SET".into()))?;
        let set_items = lower_set_items(&set_ctx)?;
        if is_on_create {
            actions.push(MergeAction::OnCreate(set_items));
        } else {
            actions.push(MergeAction::OnMatch(set_items));
        }
    }

    Ok(MergeClause { pattern, actions })
}

// ============================================================
// DELETE lowering
// ============================================================

/**
 * @brief: Lower from oC_Delete context.
 */
fn lower_delete(ctx: &OC_DeleteContext) -> Result<DeleteClause> {
    let detach = ctx.DETACH().is_some();
    let exprs = ctx
        .oC_Expression_all()
        .iter()
        .map(|e| lower_expression(e))
        .collect::<Result<Vec<_>>>()?;

    if detach {
        Ok(DeleteClause::DetachDelete(exprs))
    } else {
        Ok(DeleteClause::Delete(exprs))
    }
}

// ============================================================
// SET lowering
// ============================================================

/**
 * @brief: Lower from oC_Set context.
 */
fn lower_set(ctx: &OC_SetContext) -> Result<SetClause> {
    let items = lower_set_items(ctx)?;
    Ok(SetClause { items })
}

/**
 * @brief: Extract SetItem list from oC_Set context.
 */
fn lower_set_items(ctx: &OC_SetContext) -> Result<Vec<SetItem>> {
    let mut items = Vec::new();
    for item_ctx in &ctx.oC_SetItem_all() {
        let item = lower_set_item(item_ctx)?;
        items.push(item);
    }
    Ok(items)
}

/**
 * @brief: Lower from oC_SetItem context.
 *
 * SetItem = PropertyExpression '=' Expression | Variable NodeLabels
 */
fn lower_set_item(ctx: &OC_SetItemContext) -> Result<SetItem> {
    // Try property assignment pattern: PropertyExpression = Expression
    if let Some(prop_expr) = ctx.oC_PropertyExpression() {
        // PropertyExpression = Atom PropertyLookup*
        let atom = prop_expr
            .oC_Atom()
            .ok_or_else(|| Error::Parse("SET property expression is missing atom".into()))?;
        let var = atom
            .oC_Variable()
            .ok_or_else(|| Error::Parse("SET target must be a variable".into()))?;
        let var_name = extract_variable_name(&var)?;

        let lookups = prop_expr.oC_PropertyLookup_all();
        if lookups.len() != 1 {
            return Err(Error::Parse("SET target must be a single-level property reference".into()));
        }
        let key = extract_property_key_name(&lookups[0])?;

        if let Some(right_expr) = ctx.oC_Expression() {
            let value = lower_expression(&right_expr)?;
            return Ok(SetItem::Property {
                var: var_name,
                key,
                value,
            });
        }
        return Err(Error::Parse("SET is missing assignment expression".into()));
    }

    // Try label setting pattern: Variable NodeLabels
    if let Some(var_ctx) = ctx.oC_Variable() {
        let var = extract_variable_name(&var_ctx)?;
        if let Some(labels_ctx) = ctx.oC_NodeLabels() {
            let mut labels = Vec::new();
            for nl in &labels_ctx.oC_NodeLabel_all() {
                labels.push(extract_label_name(nl)?);
            }
            return Ok(SetItem::Labels { var, labels });
        }
    }

    Err(Error::Parse("Unrecognized SET item format".into()))
}

// ============================================================
// REMOVE lowering
// ============================================================

/**
 * @brief: Lower from oC_Remove context.
 */
fn lower_remove(ctx: &OC_RemoveContext) -> Result<RemoveClause> {
    let mut items = Vec::new();
    for item_ctx in &ctx.oC_RemoveItem_all() {
        let item = lower_remove_item(item_ctx)?;
        items.push(item);
    }
    Ok(RemoveClause { items })
}

/**
 * @brief: Lower from oC_RemoveItem context.
 */
fn lower_remove_item(ctx: &OC_RemoveItemContext) -> Result<RemoveItem> {
    // Try label removal: Variable NodeLabels
    if let Some(var_ctx) = ctx.oC_Variable() {
        let var = extract_variable_name(&var_ctx)?;
        if let Some(labels_ctx) = ctx.oC_NodeLabels() {
            let mut labels = Vec::new();
            for nl in &labels_ctx.oC_NodeLabel_all() {
                labels.push(extract_label_name(nl)?);
            }
            return Ok(RemoveItem::Labels { var, labels });
        }
    }

    // Try property removal: PropertyExpression
    if let Some(prop_expr) = ctx.oC_PropertyExpression() {
        let atom = prop_expr
            .oC_Atom()
            .ok_or_else(|| Error::Parse("REMOVE property expression is missing atom".into()))?;
        let var = atom
            .oC_Variable()
            .ok_or_else(|| Error::Parse("REMOVE target must be a variable".into()))?;
        let var_name = extract_variable_name(&var)?;

        let lookups = prop_expr.oC_PropertyLookup_all();
        if lookups.len() != 1 {
            return Err(Error::Parse("REMOVE target must be a single-level property reference".into()));
        }
        let key = extract_property_key_name(&lookups[0])?;
        return Ok(RemoveItem::Property {
            var: var_name,
            key,
        });
    }

    Err(Error::Parse("Unrecognized REMOVE item format".into()))
}

// ============================================================
// CALL subquery lowering
// ============================================================

/**
 * @brief: Lower CALL subquery from oC_InQueryCall context.
 */
fn lower_in_query_call(ctx: &OC_InQueryCallContext) -> Result<CallSubqueryClause> {
    if let Some(invoc) = ctx.oC_ExplicitProcedureInvocation() {
        let proc_name = invoc
            .as_ref()
            .oC_ProcedureName()
            .ok_or_else(|| Error::Parse("CALL is missing procedure name".into()))?;
        let algo = extract_procedure_name(proc_name.as_ref())?;

        let mut args: BTreeMap<String, PropertyValue> = BTreeMap::new();
        let exprs = invoc.oC_Expression_all();
        for expr_ctx in &exprs {
            let (key, value) = lower_call_argument(expr_ctx.as_ref())?;
            args.insert(key, value);
        }
        Ok(CallSubqueryClause { algo, args })
    } else {
        Err(Error::Parse("CALL is missing procedure invocation".into()))
    }
}

// ============================================================
// Standalone CALL lowering
// ============================================================

/**
 * @brief: Lower Standalone CALL statement from oC_StandaloneCall context.
 */
pub fn lower_standalone_call(ctx: &OC_StandaloneCallContext) -> Result<CypherAst> {
    if let Some(invoc) = ctx.oC_ExplicitProcedureInvocation() {
        let proc_name = invoc
            .as_ref()
            .oC_ProcedureName()
            .ok_or_else(|| Error::Parse("CALL is missing procedure name".into()))?;
        let algo = extract_procedure_name(proc_name.as_ref())?;

        let mut args: BTreeMap<String, PropertyValue> = BTreeMap::new();
        let exprs = invoc.oC_Expression_all();
        for expr_ctx in &exprs {
            let (key, value) = lower_call_argument(expr_ctx.as_ref())?;
            args.insert(key, value);
        }
        Ok(CypherAst::CallAlgorithm { algo, args })
    } else if let Some(invoc) = ctx.oC_ImplicitProcedureInvocation() {
        let proc_name = invoc
            .as_ref()
            .oC_ProcedureName()
            .ok_or_else(|| Error::Parse("CALL is missing procedure name".into()))?;
        let algo = extract_procedure_name(proc_name.as_ref())?;
        Ok(CypherAst::CallAlgorithm {
            algo,
            args: BTreeMap::new(),
        })
    } else {
        Err(Error::Parse("CALL is missing procedure invocation".into()))
    }
}

/**
 * @brief: Extract key=value pairs from CALL argument expressions.
 */
pub fn lower_call_argument(ctx: &OC_ExpressionContext) -> Result<(String, PropertyValue)> {
    let or_expr = ctx
        .oC_OrExpression()
        .ok_or_else(|| Error::Parse("CALL argument expression is empty".into()))?;
    let xor_exprs = or_expr.oC_XorExpression_all();
    if xor_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter format error".into()));
    }
    let and_exprs = xor_exprs[0].oC_AndExpression_all();
    if and_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter format error".into()));
    }
    let not_exprs = and_exprs[0].oC_NotExpression_all();
    if not_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter format error".into()));
    }
    let comp = not_exprs[0]
        .oC_ComparisonExpression()
        .ok_or_else(|| Error::Parse("CALL parameter format error".into()))?;

    let add_sub = comp
        .oC_AddOrSubtractExpression()
        .ok_or_else(|| Error::Parse("CALL parameter format error".into()))?;
    let key = extract_var_from_add_sub(&add_sub)?;

    let partials = comp.oC_PartialComparisonExpression_all();
    if partials.len() != 1 {
        return Err(Error::Parse("CALL parameter must be in key = value form".into()));
    }
    let op_text = partials[0].get_text().trim().to_string();
    if !op_text.starts_with('=') {
        return Err(Error::Parse(format!(
            "CALL parameter must be in key = value form, actual operator: {op_text}"
        )));
    }

    let val_add_sub = partials[0]
        .oC_AddOrSubtractExpression()
        .ok_or_else(|| Error::Parse("CALL argument value missing".into()))?;
    let val = extract_literal_from_add_sub(&val_add_sub)?;

    Ok((key, val))
}

// ============================================================
// Expression lowering (recursive descent, following ANTLR hierarchy)
// ============================================================

/**
 * @brief: Lower expression from oC_Expression context.
 */
pub fn lower_expression(ctx: &OC_ExpressionContext) -> Result<AstExpr> {
    let or_expr = ctx
        .oC_OrExpression()
        .ok_or_else(|| Error::Parse("Expression is empty".into()))?;
    lower_or_expression(&or_expr)
}

/**
 * @brief: Lower OR expression layer.
 */
pub fn lower_or_expression(ctx: &OC_OrExpressionContext) -> Result<AstExpr> {
    let xor_exprs = ctx.oC_XorExpression_all();
    let mut left = lower_xor_expression(&xor_exprs[0])?;
    for i in 1..xor_exprs.len() {
        let right = lower_xor_expression(&xor_exprs[i])?;
        left = AstExpr::Or(Box::new(left), Box::new(right));
    }
    Ok(left)
}

/**
 * @brief: Lower XOR expression layer.
 */
pub fn lower_xor_expression(ctx: &OC_XorExpressionContext) -> Result<AstExpr> {
    let and_exprs = ctx.oC_AndExpression_all();
    let mut left = lower_and_expression(&and_exprs[0])?;
    for i in 1..and_exprs.len() {
        let right = lower_and_expression(&and_exprs[i])?;
        left = AstExpr::Xor(Box::new(left), Box::new(right));
    }
    Ok(left)
}

/**
 * @brief: Lower AND expression layer.
 */
pub fn lower_and_expression(ctx: &OC_AndExpressionContext) -> Result<AstExpr> {
    let not_exprs = ctx.oC_NotExpression_all();
    let mut left = lower_not_expression(&not_exprs[0])?;
    for i in 1..not_exprs.len() {
        let right = lower_not_expression(&not_exprs[i])?;
        left = AstExpr::And(Box::new(left), Box::new(right));
    }
    Ok(left)
}

/**
 * @brief: Lower NOT expression layer.
 */
pub fn lower_not_expression(ctx: &OC_NotExpressionContext) -> Result<AstExpr> {
    let not_count = ctx.NOT_all().len();
    let comp = ctx
        .oC_ComparisonExpression()
        .ok_or_else(|| Error::Parse("NOT expression is missing comparison expression".into()))?;
    let mut inner = lower_comparison_expression(&comp)?;
    for _ in 0..not_count {
        inner = AstExpr::Not(Box::new(inner));
    }
    Ok(inner)
}

/**
 * @brief: Lower comparison expression layer.
 */
pub fn lower_comparison_expression(ctx: &OC_ComparisonExpressionContext) -> Result<AstExpr> {
    let add_sub = ctx
        .oC_AddOrSubtractExpression()
        .ok_or_else(|| Error::Parse("Comparison expression is missing left operand".into()))?;
    let mut left = lower_add_or_subtract_expression(&add_sub)?;

    let partials = ctx.oC_PartialComparisonExpression_all();
    for partial in &partials {
        let right_add_sub = partial
            .oC_AddOrSubtractExpression()
            .ok_or_else(|| Error::Parse("Partial comparison expression is missing right operand".into()))?;
        let right = lower_add_or_subtract_expression(&right_add_sub)?;

        let op_text = partial.get_text().trim().to_string();
        left = if op_text.starts_with("<>") {
            AstExpr::Neq(Box::new(left), Box::new(right))
        } else if op_text.starts_with("<=") {
            AstExpr::Lte(Box::new(left), Box::new(right))
        } else if op_text.starts_with(">=") {
            AstExpr::Gte(Box::new(left), Box::new(right))
        } else if op_text.starts_with('=') {
            AstExpr::Eq(Box::new(left), Box::new(right))
        } else if op_text.starts_with('<') {
            AstExpr::Lt(Box::new(left), Box::new(right))
        } else if op_text.starts_with('>') {
            AstExpr::Gt(Box::new(left), Box::new(right))
        } else {
            return Err(Error::Parse(format!(
                "Unrecognized comparison operator: {op_text}"
            )));
        };
    }

    Ok(left)
}

/**
 * @brief: Lower addition/subtraction expression layer.
 *
 * Simplified strategy: if there is only one multiplication sub-expression, pass it through;
 * if there are two, determine the operator character via get_text().
 */
pub fn lower_add_or_subtract_expression(
    ctx: &OC_AddOrSubtractExpressionContext,
) -> Result<AstExpr> {
    let mul_exprs = ctx.oC_MultiplyDivideModuloExpression_all();
    match mul_exprs.len() {
        0 => Err(Error::Parse("Addition/subtraction expression is missing operand".into())),
        1 => lower_multiply_expression(&mul_exprs[0]),
        2 => {
            let left = lower_multiply_expression(&mul_exprs[0])?;
            let right = lower_multiply_expression(&mul_exprs[1])?;
            let text = ctx.get_text();
            if text.contains('+') {
                Ok(AstExpr::Add(Box::new(left), Box::new(right)))
            } else if text.contains('-') {
                Ok(AstExpr::Sub(Box::new(left), Box::new(right)))
            } else {
                Ok(left)
            }
        }
        _ => {
            let mut left = lower_multiply_expression(&mul_exprs[0])?;
            for mul_expr in mul_exprs.iter().skip(1) {
                let right = lower_multiply_expression(mul_expr)?;
                left = AstExpr::Add(Box::new(left), Box::new(right));
            }
            Ok(left)
        }
    }
}

/**
 * @brief: Lower multiplication/division/modulo expression layer.
 *
 * Simplified strategy: similar to add/sub layer, pass through transparently or detect operator by text.
 */
pub fn lower_multiply_expression(
    ctx: &OC_MultiplyDivideModuloExpressionContext,
) -> Result<AstExpr> {
    let pow_exprs = ctx.oC_PowerOfExpression_all();
    match pow_exprs.len() {
        0 => Err(Error::Parse("Multiplication/division expression is missing operand".into())),
        1 => lower_power_expression(&pow_exprs[0]),
        2 => {
            let left = lower_power_expression(&pow_exprs[0])?;
            let right = lower_power_expression(&pow_exprs[1])?;
            let text = ctx.get_text();
            if text.contains('/') {
                Ok(AstExpr::Div(Box::new(left), Box::new(right)))
            } else if text.contains('%') {
                Ok(AstExpr::Mod(Box::new(left), Box::new(right)))
            } else if text.contains('*') {
                Ok(AstExpr::Mul(Box::new(left), Box::new(right)))
            } else {
                Ok(left)
            }
        }
        _ => {
            let mut left = lower_power_expression(&pow_exprs[0])?;
            for pow_expr in pow_exprs.iter().skip(1) {
                let right = lower_power_expression(pow_expr)?;
                left = AstExpr::Mul(Box::new(left), Box::new(right));
            }
            Ok(left)
        }
    }
}

/**
 * @brief: Lower power expression layer.
 */
pub fn lower_power_expression(ctx: &OC_PowerOfExpressionContext) -> Result<AstExpr> {
    let unary_exprs = ctx.oC_UnaryAddOrSubtractExpression_all();
    let mut left = lower_unary_expression(&unary_exprs[0])?;

    if unary_exprs.len() > 1 {
        // Exponentiation: right-to-left associative
        let right = lower_unary_expression(&unary_exprs[1])?;
        left = AstExpr::Pow(Box::new(left), Box::new(right));
    }

    Ok(left)
}

/**
 * @brief: Lower unary addition/subtraction expression layer.
 */
pub fn lower_unary_expression(
    ctx: &OC_UnaryAddOrSubtractExpressionContext,
) -> Result<AstExpr> {
    let str_expr = ctx
        .oC_StringListNullOperatorExpression()
        .ok_or_else(|| Error::Parse("Unary expression structure is abnormal".into()))?;

    let text = ctx.get_text();
    let inner = lower_string_list_null_expression(str_expr.as_ref())?;

    if text.starts_with('-') {
        Ok(AstExpr::Neg(Box::new(inner)))
    } else {
        Ok(inner)
    }
}

/**
 * @brief: Lower string/list/null operator expression layer.
 *
 * StringListNullOperatorExpression = PropertyOrLabelsExpression
 *   (StringOperatorExpression | ListOperatorExpression | NullOperatorExpression)*
 */
pub fn lower_string_list_null_expression(
    ctx: &OC_StringListNullOperatorExpressionContext,
) -> Result<AstExpr> {
    let prop = ctx
        .oC_PropertyOrLabelsExpression()
        .ok_or_else(|| Error::Parse("Expression structure is abnormal".into()))?;
    let mut expr = lower_property_or_labels_expression(prop.as_ref())?;

    // Process postfix operators in ANTLR parse tree order

    // String operator expressions: STARTS WITH, ENDS WITH, CONTAINS, =~
    for so_ctx in &ctx.oC_StringOperatorExpression_all() {
        let right_prop = so_ctx
            .oC_PropertyOrLabelsExpression()
            .ok_or_else(|| Error::Parse("String operator is missing right operand".into()))?;
        let right = lower_property_or_labels_expression(right_prop.as_ref())?;

        if so_ctx.STARTS().is_some() {
            expr = AstExpr::StartsWith(Box::new(expr), Box::new(right));
        } else if so_ctx.ENDS().is_some() {
            expr = AstExpr::EndsWith(Box::new(expr), Box::new(right));
        } else if so_ctx.CONTAINS().is_some() {
            expr = AstExpr::Contains(Box::new(expr), Box::new(right));
        } else {
            // Regex match =~
            expr = AstExpr::RegexMatch(Box::new(expr), Box::new(right));
        }
    }

    // List operator expressions: IN
    for lo_ctx in &ctx.oC_ListOperatorExpression_all() {
        let right_prop = lo_ctx
            .oC_PropertyOrLabelsExpression()
            .ok_or_else(|| Error::Parse("IN operator is missing right operand".into()))?;
        let right = lower_property_or_labels_expression(right_prop.as_ref())?;
        expr = AstExpr::In(Box::new(expr), Box::new(right));
    }

    // Null operator expressions: IS NULL, IS NOT NULL
    for no_ctx in &ctx.oC_NullOperatorExpression_all() {
        if no_ctx.IS().is_some() && no_ctx.NOT().is_some() {
            expr = AstExpr::IsNotNull(Box::new(expr));
        } else if no_ctx.IS().is_some() && no_ctx.NULL().is_some() {
            expr = AstExpr::IsNull(Box::new(expr));
        }
    }

    Ok(expr)
}

/**
 * @brief: lowering PropertyOrLabelsExpression -> AstExpr.
 */
pub fn lower_property_or_labels_expression(
    ctx: &OC_PropertyOrLabelsExpressionContext,
) -> Result<AstExpr> {
    let atom = ctx
        .oC_Atom()
        .ok_or_else(|| Error::Parse("Property expression is missing atom".into()))?;
    let prop_lookups = ctx.oC_PropertyLookup_all();

    if prop_lookups.is_empty() {
        lower_atom(&atom)
    } else {
        let var = atom
            .oC_Variable()
            .ok_or_else(|| Error::Parse("The left side of property access must be a variable".into()))?;
        let var_name = extract_variable_name(&var)?;

        let mut expr: AstExpr = AstExpr::Var(var_name.clone());

        // Process property lookup chain: may have multiple levels
        for lookup in &prop_lookups {
            let key = extract_property_key_name(lookup)?;
            expr = AstExpr::Prop {
                var: var_name.clone(),
                key,
            };
        }

        if prop_lookups.len() == 1 {
            let key = extract_property_key_name(&prop_lookups[0])?;
            Ok(AstExpr::Prop {
                var: var_name,
                key,
            })
        } else {
            Err(Error::Parse("Nested property access is not supported".into()))
        }
    }
}

/**
 * @brief: lowering Atom -> AstExpr.
 */
pub fn lower_atom(ctx: &OC_AtomContext) -> Result<AstExpr> {
    if let Some(literal) = ctx.oC_Literal() {
        lower_literal(&literal)
    } else if let Some(var) = ctx.oC_Variable() {
        let name = extract_variable_name(&var)?;
        Ok(AstExpr::Var(name))
    } else if let Some(paren) = ctx.oC_ParenthesizedExpression() {
        let expr = paren
            .oC_Expression()
            .ok_or_else(|| Error::Parse("Parenthesized expression is empty".into()))?;
        lower_expression(&expr)
    } else if let Some(func) = ctx.oC_FunctionInvocation() {
        lower_function_invocation(func.as_ref())
    } else if let Some(case) = ctx.oC_CaseExpression() {
        lower_case_expression(case.as_ref())
    } else if let Some(list) = ctx.oC_ListComprehension() {
        lower_list_comprehension_from_atom(list.as_ref())
    } else if let Some(pattern) = ctx.oC_PatternComprehension() {
        lower_pattern_comprehension(pattern.as_ref())
    } else if let Some(param) = ctx.oC_Parameter() {
        let text = param.get_text();
        let name = if text.starts_with('$') {
            text[1..].to_string()
        } else {
            text
        };
        Ok(AstExpr::Parameter(name))
    } else if ctx.COUNT().is_some() {
        // COUNT(*) is specially handled in the grammar
        Ok(AstExpr::FunctionCall {
            name: "count".into(),
            distinct: false,
            args: vec![],
        })
    } else if let Some(filter) = ctx.oC_FilterExpression() {
        // ALL, ANY, NONE, SINGLE filter expressions
        lower_filter_expression(filter.as_ref())
    } else if ctx.oC_RelationshipsPattern().is_some() {
        // RelationshipsPattern not yet supported for lowering to expression
        Err(Error::Parse("RelationshipsPattern not yet supported".into()))
    } else {
        Err(Error::Parse("Unsupported expression atom type".into()))
    }
}

/**
 * @brief: lowering FilterExpression (ALL, ANY, NONE, SINGLE).
 *
 * FilterExpression = IdInColl (WHERE Expression)?
 */
fn lower_filter_expression(ctx: &OC_FilterExpressionContext) -> Result<AstExpr> {
    if let Some(where_ctx) = ctx.oC_Where() {
        let expr = where_ctx
            .oC_Expression()
            .ok_or_else(|| Error::Parse("FilterExpression WHERE is missing expression".into()))?;
        return lower_expression(&expr);
    }
    // When unable to lower, return a placeholder variable
    Err(Error::Parse("FilterExpression format not supported".into()))
}

/**
 * @brief: lowering Literal -> AstExpr.
 */
pub fn lower_literal(ctx: &OC_LiteralContext) -> Result<AstExpr> {
    if let Some(_num) = ctx.oC_NumberLiteral() {
        let text = ctx.get_text();
        if let Ok(i) = text.parse::<i64>() {
            Ok(AstExpr::Int(i))
        } else if let Ok(f) = text.parse::<f64>() {
            Ok(AstExpr::Float(f))
        } else {
            Err(Error::Parse(format!("Unable to parse numeric literal: {text}")))
        }
    } else if let Some(s) = ctx.StringLiteral() {
        let raw = s.get_text();
        let inner = if raw.len() >= 2 {
            &raw[1..raw.len() - 1]
        } else {
            &raw
        };
        Ok(AstExpr::Str(inner.to_string()))
    } else if let Some(_bool) = ctx.oC_BooleanLiteral() {
        let text = ctx.get_text().to_lowercase();
        if text == "true" {
            Ok(AstExpr::Bool(true))
        } else if text == "false" {
            Ok(AstExpr::Bool(false))
        } else {
            Err(Error::Parse(format!("Unable to parse boolean literal: {text}")))
        }
    } else if ctx.NULL().is_some() {
        Ok(AstExpr::Null)
    } else if let Some(list) = ctx.oC_ListLiteral() {
        lower_list_literal_from_literal(list.as_ref())
    } else if let Some(map) = ctx.oC_MapLiteral() {
        lower_map_literal(map.as_ref())
    } else {
        Err(Error::Parse("Unsupported literal type".into()))
    }
}

/**
 * @brief: Lower function call expression.
 */
pub fn lower_function_invocation(ctx: &OC_FunctionInvocationContext) -> Result<AstExpr> {
    let name_ctx = ctx
        .oC_FunctionName()
        .ok_or_else(|| Error::Parse("Function call is missing function name".into()))?;
    let name = extract_symbolic_name_text(name_ctx.as_ref())?;

    let distinct = ctx.DISTINCT().is_some();

    let args: Vec<AstExpr> = ctx
        .oC_Expression_all()
        .iter()
        .map(|e| lower_expression(e))
        .collect::<Result<Vec<_>>>()?;

    Ok(AstExpr::FunctionCall {
        name,
        distinct,
        args,
    })
}

/**
 * @brief: Lower CASE expression.
 *
 * CASE (Expression)? (WHEN Expression THEN Expression)+ (ELSE Expression)? END
 * In the ANTLR grammar, expressions after CASE and in WHEN/THEN are accessed sequentially via oC_Expression_all().
 */
pub fn lower_case_expression(ctx: &OC_CaseExpressionContext) -> Result<AstExpr> {
    let exprs = ctx.oC_Expression_all();
    if exprs.is_empty() {
        return Err(Error::Parse("CASE expression is missing branches".into()));
    }

    let mut cases = Vec::new();
    let mut else_branch = None;
    let mut test = None;
    let _idx = 0;

    // Check if there is a CASE test_expr form (Simple CASE)
    // Determine by position between CASE and WHEN: if there is no WHEN after CASE, the first expression is test
    // Simplified strategy: the role of the first expression is determined by whether there is a WHEN after CASE.
    // ANTLR grammar CaseExpression = CASE (Expression)? (WHEN Expression THEN Expression)+ (ELSE Expression)? END
    // Expression between CASE..WHEN is zero or one, between WHEN..THEN appears in pairs,
    // CASE alternative uses oC_CaseAlternatives

    // Simple implementation: iterate all Expressions, extract WHEN/THEN in pairs
    let mut i = 0;
    let n = exprs.len();

    // Try to identify: if there is an ELSE token, the last expression is the ELSE branch
    let has_else = ctx.ELSE().is_some();
    let main_count = if has_else { n - 1 } else { n };

    // Determine whether it is Simple CASE or Generic CASE
    // Simple CASE: CASE test_expr WHEN ... (test_expr is the first, rest in pairs)
    // Generic CASE: CASE WHEN cond THEN result ... (all in pairs)
    // If the number of expressions is odd and there is an ELSE, it is Simple CASE
    let is_simple = main_count % 2 == 1;

    if is_simple {
        // Simple CASE: first expr is test, rest are WHEN val / THEN result pairs
        test = Some(Box::new(lower_expression(&exprs[0])?));
        i = 1;
    }

    while i + 1 < main_count {
        let when_val = lower_expression(&exprs[i])?;
        let then_val = lower_expression(&exprs[i + 1])?;

        if let Some(ref t) = test {
            // Simple CASE: condition is test = when_val
            let cond = AstExpr::Eq(t.clone(), Box::new(when_val));
            cases.push((cond, then_val));
        } else {
            // Generic CASE: when_val IS the condition
            cases.push((when_val, then_val));
        }
        i += 2;
    }

    if has_else {
        else_branch = Some(Box::new(lower_expression(exprs.last().unwrap())?));
    }

    Ok(AstExpr::Case {
        test,
        cases,
        else_branch,
    })
}

/**
 * @brief: lowering ListComprehension.
 */
fn lower_list_comprehension_from_atom(
    _ctx: &OC_ListComprehensionContext,
) -> Result<AstExpr> {
    Err(Error::Parse("ListComprehension not yet supported".into()))
}

/**
 * @brief: lowering PatternComprehension.
 */
fn lower_pattern_comprehension(
    _ctx: &OC_PatternComprehensionContext,
) -> Result<AstExpr> {
    Err(Error::Parse("PatternComprehension not yet supported".into()))
}


/**
 * @brief: Lower Map literal.
 *
 * MapLiteral = '{' (PropertyKeyName ':' Expression (',' PropertyKeyName ':' Expression)*)? '}'
 */
pub fn lower_map_literal(ctx: &OC_MapLiteralContext) -> Result<AstExpr> {
    let keys = ctx.oC_PropertyKeyName_all();
    let vals = ctx.oC_Expression_all();

    // PropertyKeyName and Expression should match in count
    let n = keys.len().min(vals.len());
    let mut entries = Vec::with_capacity(n);
    for i in 0..n {
        let key = extract_property_key_name_from_ctx(&keys[i])?;
        let val = lower_expression(&vals[i])?;
        entries.push((key, val));
    }
    Ok(AstExpr::Map(entries))
}

/**
 * @brief: Extract key name from oC_PropertyKeyName context.
 */
fn extract_property_key_name_from_ctx(ctx: &OC_PropertyKeyNameContext) -> Result<String> {
    let schema = ctx
        .oC_SchemaName()
        .ok_or_else(|| Error::Parse("Property key name is missing schema name".into()))?;
    let sym = schema
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Property key name is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}


/**
 * @brief: Lower List literal (from Literal).
 */
fn lower_list_literal_from_literal(ctx: &OC_ListLiteralContext) -> Result<AstExpr> {
    let exprs = ctx.oC_Expression_all();
    let items: Vec<AstExpr> = exprs
        .iter()
        .map(|e| lower_expression(e))
        .collect::<Result<Vec<_>>>()?;
    Ok(AstExpr::List(items))
}

// ============================================================
// PropertyValue lowering
// ============================================================

/**
 * @brief: Extract PropertyValue from oC_Literal.
 */
pub fn lower_property_value(ctx: &OC_LiteralContext) -> Result<PropertyValue> {
    if let Some(_num) = ctx.oC_NumberLiteral() {
        let text = ctx.get_text();
        if let Ok(i) = text.parse::<i64>() {
            Ok(PropertyValue::Int(i))
        } else if let Ok(f) = text.parse::<f64>() {
            Ok(PropertyValue::Float(f))
        } else {
            Err(Error::Parse(format!("Unable to parse numeric literal: {text}")))
        }
    } else if let Some(s) = ctx.StringLiteral() {
        let raw = s.get_text();
        let inner = if raw.len() >= 2 {
            &raw[1..raw.len() - 1]
        } else {
            &raw
        };
        Ok(PropertyValue::Text(inner.to_string()))
    } else if let Some(_bool) = ctx.oC_BooleanLiteral() {
        let text = ctx.get_text().to_lowercase();
        if text == "true" {
            Ok(PropertyValue::Bool(true))
        } else if text == "false" {
            Ok(PropertyValue::Bool(false))
        } else {
            Err(Error::Parse(format!("Unable to parse boolean literal: {text}")))
        }
    } else if ctx.NULL().is_some() {
        Ok(PropertyValue::Null)
    } else {
        Err(Error::Parse("Unsupported literal type".into()))
    }
}

// ============================================================
// Name extraction helper functions
// ============================================================

/**
 * @brief: Extract variable name text from oC_Variable context.
 */
pub fn extract_variable_name(ctx: &OC_VariableContext) -> Result<String> {
    let sym = ctx
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Variable is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}

/**
 * @brief: Extract name text from oC_SymbolicName context.
 */
pub fn extract_symbolic_name(ctx: &OC_SymbolicNameContext) -> Result<String> {
    let text = ctx.get_text();

    if let Some(t) = ctx.UnescapedSymbolicName() {
        Ok(t.get_text())
    } else if let Some(t) = ctx.EscapedSymbolicName() {
        let raw = t.get_text();
        let inner = if raw.len() >= 2 {
            &raw[1..raw.len() - 1]
        } else {
            &raw
        };
        Ok(inner.to_string())
    } else if !text.is_empty() {
        Ok(text)
    } else {
        Err(Error::Parse("Unrecognized symbolic name".into()))
    }
}

/**
 * @brief: Extract function name text from oC_FunctionNameContext.
 */
pub fn extract_symbolic_name_text(ctx: &OC_FunctionNameContext) -> Result<String> {
    let sym = ctx
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Function name is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}

/**
 * @brief: Extract label name text from oC_NodeLabel context.
 */
pub fn extract_label_name(ctx: &OC_NodeLabelContext) -> Result<String> {
    let label_ctx = ctx
        .oC_LabelName()
        .ok_or_else(|| Error::Parse("Label is missing name".into()))?;
    let schema = label_ctx
        .oC_SchemaName()
        .ok_or_else(|| Error::Parse("Label is missing schema name".into()))?;
    extract_schema_name_text_inner(schema.as_ref())
}

/**
 * @brief: Extract edge type name text from oC_RelTypeName context.
 */
pub fn extract_schema_name_text(ctx: &OC_RelTypeNameContext) -> Result<String> {
    let schema = ctx
        .oC_SchemaName()
        .ok_or_else(|| Error::Parse("Relationship type name is missing schema name".into()))?;
    extract_schema_name_text_inner(schema.as_ref())
}

/**
 * @brief: Extract name text from oC_SchemaName context.
 */
pub fn extract_schema_name_text_inner(ctx: &OC_SchemaNameContext) -> Result<String> {
    let sym = ctx
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Schema name is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}

/**
 * @brief: Extract property key name from oC_PropertyLookup context.
 */
pub fn extract_property_key_name(ctx: &OC_PropertyLookupContext) -> Result<String> {
    let key = ctx
        .oC_PropertyKeyName()
        .ok_or_else(|| Error::Parse("Property lookup is missing key name".into()))?;
    let schema = key
        .oC_SchemaName()
        .ok_or_else(|| Error::Parse("Property key name is missing schema name".into()))?;
    let sym = schema
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Property key name is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}

/**
 * @brief: Extract procedure name text from oC_ProcedureName context.
 */
pub fn extract_procedure_name(ctx: &OC_ProcedureNameContext) -> Result<String> {
    let sym = ctx
        .oC_SymbolicName()
        .ok_or_else(|| Error::Parse("Procedure name is missing symbolic name".into()))?;
    extract_symbolic_name(&sym)
}

// ============================================================
// CALL parameter extraction helper functions
// ============================================================

/**
 * @brief: Extract simple variable name from AddOrSubtractExpression.
 */
pub fn extract_var_from_add_sub(ctx: &OC_AddOrSubtractExpressionContext) -> Result<String> {
    let mul_exprs = ctx.oC_MultiplyDivideModuloExpression_all();
    if mul_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter key must be a simple variable name".into()));
    }
    let pow_exprs = mul_exprs[0].oC_PowerOfExpression_all();
    if pow_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter key must be a simple variable name".into()));
    }
    let unary_exprs = pow_exprs[0].oC_UnaryAddOrSubtractExpression_all();
    if unary_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter key must be a simple variable name".into()));
    }
    let str_expr = unary_exprs[0]
        .oC_StringListNullOperatorExpression()
        .ok_or_else(|| Error::Parse("CALL parameter key must be a simple variable name".into()))?;
    let prop = str_expr
        .as_ref()
        .oC_PropertyOrLabelsExpression()
        .ok_or_else(|| Error::Parse("CALL parameter format is abnormal".into()))?;
    let atom = prop
        .as_ref()
        .oC_Atom()
        .ok_or_else(|| Error::Parse("CALL parameter format is abnormal".into()))?;
    let var = atom
        .as_ref()
        .oC_Variable()
        .ok_or_else(|| Error::Parse("CALL parameter key must be a variable name".into()))?;
    extract_variable_name(var.as_ref())
}

/**
 * @brief: Extract literal PropertyValue from AddOrSubtractExpression.
 */
pub fn extract_literal_from_add_sub(
    ctx: &OC_AddOrSubtractExpressionContext,
) -> Result<PropertyValue> {
    let mul_exprs = ctx.oC_MultiplyDivideModuloExpression_all();
    if mul_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter value must be a simple literal".into()));
    }
    let pow_exprs = mul_exprs[0].oC_PowerOfExpression_all();
    if pow_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter value must be a simple literal".into()));
    }
    let unary_exprs = pow_exprs[0].oC_UnaryAddOrSubtractExpression_all();
    if unary_exprs.len() != 1 {
        return Err(Error::Parse("CALL parameter value must be a simple literal".into()));
    }
    let str_expr = unary_exprs[0]
        .oC_StringListNullOperatorExpression()
        .ok_or_else(|| Error::Parse("CALL parameter value must be a simple literal".into()))?;
    let prop = str_expr
        .as_ref()
        .oC_PropertyOrLabelsExpression()
        .ok_or_else(|| Error::Parse("CALL parameter format is abnormal".into()))?;
    let atom = prop
        .as_ref()
        .oC_Atom()
        .ok_or_else(|| Error::Parse("CALL parameter format is abnormal".into()))?;
    let literal = atom
        .as_ref()
        .oC_Literal()
        .ok_or_else(|| Error::Parse("CALL parameter value must be a literal".into()))?;

    lower_property_value(literal.as_ref())
}
