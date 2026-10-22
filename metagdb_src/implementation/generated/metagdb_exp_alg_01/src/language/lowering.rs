//! Lowering from the generated ANTLR Cypher CST to the private AST.
//!
//! The generated crate `metagdb_cypher_antlr` is the only syntax frontend: this
//! module walks contexts produced by its parser, never a candidate tokenizer.
//!
//! Two access patterns are used deliberately:
//!
//! * **Child accessors** (`oC_Expression()`, `oC_PatternPart_all()`, …) wherever
//!   the summary exposes one, so the code is checked against the real generated
//!   API rather than against reconstructed grammar knowledge;
//! * **Ordered child walking** where the grammar's meaning depends on sibling
//!   order that the accessors do not expose — the `+`/`-` between addends, the
//!   `UNION` boundaries of a multi-part query, or the operator token inside a
//!   comparison. [`child_tokens`] and [`child_of`] read direct children, so a
//!   nested expression's own tokens are never mistaken for a sibling operator.
//!
//! Grammar-valid input whose *semantics* the candidate does not implement is
//! rejected with `Error::Unsupported` here or in planning; it is never silently
//! dropped.

use std::rc::Rc;

use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::tree::{ParseTree, TerminalNode, Tree};
use antlr4rust::TidExt;
use antlr4rust::errors::ANTLRError;
use antlr4rust::token::Token;

use metagdb_cypher_antlr::cypherparser::*;
use metagdb_core::{Error, Result};

use super::ast::*;

// ---------------------------------------------------------------------------
// CST helpers
// ---------------------------------------------------------------------------

fn parse_error(message: impl Into<String>) -> Error {
    Error::Parse(message.into())
}

fn required<T>(value: Option<T>, what: &str) -> Result<T> {
    value.ok_or_else(|| parse_error(format!("the parsed statement is missing {what}")))
}

/// Text of every direct terminal child, in source order.
///
/// Only terminals are returned, so a nested rule's tokens cannot leak in.
fn child_tokens<'input, T>(ctx: &T) -> Vec<String>
where
    T: ParserRuleContext<'input, Ctx = CypherParserContextType> + ?Sized,
{
    ctx.get_children()
        .filter_map(|child| {
            child
                .downcast_rc::<TerminalNode<'input, CypherParserContextType>>()
                .ok()
        })
        .map(|node| node.symbol.get_text().to_string())
        .collect()
}

/// Escape-free source text of a name-like context (no whitespace children).
fn context_text<'input, T>(ctx: &T) -> String
where
    T: ParseTree<'input> + ?Sized,
{
    ctx.get_text()
}

fn variable_name<'input>(ctx: &OC_VariableContext<'input>) -> Result<String> {
    let name = required(ctx.oC_SymbolicName(), "a variable name")?;
    Ok(context_text(&*name))
}

fn schema_name<'input>(ctx: &OC_SchemaNameContext<'input>) -> String {
    context_text(ctx)
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Lower the `oC_Cypher` entry rule's CST into the private AST.
pub fn lower_cypher<'input>(root: &OC_CypherContext<'input>) -> Result<CypherAst> {
    let statement = required(root.oC_Statement(), "a statement")?;
    let query = required(statement.oC_Query(), "a query")?;
    Ok(CypherAst {
        body: lower_query(&query)?,
    })
}

/// Convenience wrapper used by tests and by the parser wrapper.
pub fn lower_antlr_error(error: ANTLRError) -> Error {
    Error::Parse(format!("{error}"))
}

fn lower_query<'input>(ctx: &OC_QueryContext<'input>) -> Result<AstBody> {
    if let Some(regular) = ctx.oC_RegularQuery() {
        return lower_regular_query(&regular);
    }
    if let Some(call) = ctx.oC_StandaloneCall() {
        let invocation = match (
            call.oC_ExplicitProcedureInvocation(),
            call.oC_ImplicitProcedureInvocation(),
        ) {
            (Some(explicit), _) => lower_explicit_invocation(&explicit)?,
            (None, Some(implicit)) => ProcedureCall {
                name: lower_procedure_name(required(implicit.oC_ProcedureName(), "a procedure name")?.as_ref()),
                args: Vec::new(),
                yields: None,
            },
            (None, None) => return Err(parse_error("CALL is missing its procedure invocation")),
        };
        let yields = match call.oC_YieldItems() {
            Some(items) => Some(lower_yield_items(&items)?),
            None => None,
        };
        return Ok(AstBody::Call(ProcedureCall {
            yields,
            ..invocation
        }));
    }
    Err(Error::Unsupported(
        "only regular queries and standalone CALL statements are supported".into(),
    ))
}

fn lower_explicit_invocation<'input>(
    ctx: &OC_ExplicitProcedureInvocationContext<'input>,
) -> Result<ProcedureCall> {
    let name = lower_procedure_name(required(ctx.oC_ProcedureName(), "a procedure name")?.as_ref());
    let mut args = Vec::new();
    for item in ctx.oC_Expression_all() {
        args.push(lower_expression(&item)?);
    }
    Ok(ProcedureCall {
        name,
        args,
        yields: None,
    })
}

fn lower_procedure_name<'input>(ctx: &OC_ProcedureNameContext<'input>) -> String {
    let mut parts = Vec::new();
    if let Some(namespace) = ctx.oC_Namespace() {
        for name in namespace.oC_SymbolicName_all() {
            parts.push(context_text(&*name));
        }
    }
    if let Some(name) = ctx.oC_SymbolicName() {
        parts.push(context_text(&*name));
    }
    parts.join(".")
}

fn lower_yield_items<'input>(ctx: &OC_YieldItemsContext<'input>) -> Result<YieldItems> {
    let contexts = ctx.oC_YieldItem_all();
    let mut items = Vec::new();
    for item in &contexts {
        let alias = variable_name(required(item.oC_Variable(), "a YIELD variable")?.as_ref())?;
        let field = match item.oC_ProcedureResultField() {
            Some(result) => Some(context_text(&*required(result.oC_SymbolicName(), "a result field")?)),
            None => None,
        };
        items.push(YieldItem { field, alias });
    }
    let predicate = match ctx.oC_Where() {
        Some(where_) => Some(lower_expression(required(where_.oC_Expression(), "a WHERE predicate")?.as_ref())?),
        None => None,
    };
    Ok(YieldItems {
        // `YIELD *` is the only form without explicit items.
        all: contexts.is_empty(),
        items,
        predicate,
    })
}

// ---------------------------------------------------------------------------
// Queries and clauses
// ---------------------------------------------------------------------------

fn lower_regular_query<'input>(ctx: &OC_RegularQueryContext<'input>) -> Result<AstBody> {
    let mut branches = vec![lower_single_query(required(ctx.oC_SingleQuery(), "a query body")?.as_ref())?];
    let mut all_flags = Vec::new();
    for union in ctx.oC_Union_all() {
        all_flags.push(union.ALL().is_some());
        branches.push(lower_single_query(required(
            union.oC_SingleQuery(),
            "a UNION branch",
        )?.as_ref())?);
    }
    // A single-branch query has no UNION keyword at all; only a multi-branch
    // chain carries a combination mode.
    if !all_flags.is_empty() {
        let uniform = all_flags.iter().all(|flag| *flag == all_flags[0]);
        if !uniform {
            // The AST carries one combination mode per UNION chain; refusing a
            // mixed chain keeps the representation honest instead of picking
            // the first branch's mode and silently changing the others.
            return Err(Error::Unsupported(
                "a UNION chain that mixes UNION and UNION ALL is not supported".into(),
            ));
        }
    }
    Ok(AstBody::Union {
        all: all_flags.first().copied().unwrap_or(false),
        branches,
    })
}

fn lower_single_query<'input>(ctx: &OC_SingleQueryContext<'input>) -> Result<Vec<Clause>> {
    if let Some(multi) = ctx.oC_MultiPartQuery() {
        let mut clauses = Vec::new();
        for child in multi.get_children() {
            if let Ok(reading) = child.clone().downcast_rc::<OC_ReadingClauseContext<'input>>() {
                clauses.push(lower_reading_clause(&reading)?);
            } else if let Ok(updating) = child.clone().downcast_rc::<OC_UpdatingClauseContext<'input>>() {
                clauses.push(lower_updating_clause(&updating)?);
            } else if let Ok(with) = child.clone().downcast_rc::<OC_WithContext<'input>>() {
                clauses.push(lower_with(&with)?);
            } else if let Ok(single) = child.clone().downcast_rc::<OC_SinglePartQueryContext<'input>>() {
                clauses.extend(lower_single_part_query(&single)?);
            }
        }
        return Ok(clauses);
    }
    if let Some(single) = ctx.oC_SinglePartQuery() {
        return lower_single_part_query(&single);
    }
    Err(parse_error("the query body is neither single-part nor multi-part"))
}

fn lower_single_part_query<'input>(ctx: &OC_SinglePartQueryContext<'input>) -> Result<Vec<Clause>> {
    let mut clauses = Vec::new();
    for child in ctx.get_children() {
        if let Ok(reading) = child.clone().downcast_rc::<OC_ReadingClauseContext<'input>>() {
            clauses.push(lower_reading_clause(&reading)?);
        } else if let Ok(updating) = child.clone().downcast_rc::<OC_UpdatingClauseContext<'input>>() {
            clauses.push(lower_updating_clause(&updating)?);
        } else if let Ok(ret) = child.clone().downcast_rc::<OC_ReturnContext<'input>>() {
            clauses.push(Clause::Return(lower_projection_body(required(
                ret.oC_ProjectionBody(),
                "a RETURN projection",
            )?.as_ref())?));
        }
    }
    Ok(clauses)
}

fn lower_reading_clause<'input>(ctx: &OC_ReadingClauseContext<'input>) -> Result<Clause> {
    if let Some(match_) = ctx.oC_Match() {
        let pattern = required(match_.oC_Pattern(), "a pattern")?;
        return Ok(Clause::Match {
            optional: match_.OPTIONAL().is_some(),
            pattern: lower_pattern(&pattern)?,
            predicate: match lower_where(match_.oC_Where()) {
                Some(predicate) => Some(predicate?),
                None => None,
            },
        });
    }
    if let Some(unwind) = ctx.oC_Unwind() {
        return Ok(Clause::Unwind {
            expr: lower_expression(required(unwind.oC_Expression(), "an UNWIND expression")?.as_ref())?,
            alias: variable_name(required(unwind.oC_Variable(), "an UNWIND alias")?.as_ref())?,
        });
    }
    if let Some(call) = ctx.oC_InQueryCall() {
        let invocation = lower_explicit_invocation(required(
            call.oC_ExplicitProcedureInvocation(),
            "a procedure invocation",
        )?.as_ref())?;
        let yields = match call.oC_YieldItems() {
            Some(items) => Some(lower_yield_items(&items)?),
            None => None,
        };
        return Ok(Clause::Call(ProcedureCall {
            yields,
            ..invocation
        }));
    }
    Err(Error::Unsupported(
        "this reading clause is outside the requested scope".into(),
    ))
}

fn lower_updating_clause<'input>(ctx: &OC_UpdatingClauseContext<'input>) -> Result<Clause> {
    if let Some(create) = ctx.oC_Create() {
        return Ok(Clause::Create(lower_pattern(required(
            create.oC_Pattern(),
            "a pattern",
        )?.as_ref())?));
    }
    if let Some(merge) = ctx.oC_Merge() {
        return Ok(Clause::Merge {
            pattern: lower_pattern_part(required(merge.oC_PatternPart(), "a pattern part")?.as_ref())?,
            actions: lower_merge_actions(&merge)?,
        });
    }
    if let Some(delete) = ctx.oC_Delete() {
        let mut targets = Vec::new();
        for expr in delete.oC_Expression_all() {
            targets.push(lower_expression(&expr)?);
        }
        return Ok(Clause::Delete {
            detach: delete.DETACH().is_some(),
            targets,
        });
    }
    if let Some(set) = ctx.oC_Set() {
        let mut items = Vec::new();
        for item in set.oC_SetItem_all() {
            items.push(lower_set_item(&item)?);
        }
        return Ok(Clause::Set(items));
    }
    if let Some(remove) = ctx.oC_Remove() {
        let mut items = Vec::new();
        for item in remove.oC_RemoveItem_all() {
            items.push(lower_remove_item(&item)?);
        }
        return Ok(Clause::Remove(items));
    }
    Err(Error::Unsupported(
        "this updating clause is outside the requested scope".into(),
    ))
}

fn lower_with<'input>(ctx: &OC_WithContext<'input>) -> Result<Clause> {
    let body = lower_projection_body(required(ctx.oC_ProjectionBody(), "a WITH projection")?.as_ref())?;
    Ok(Clause::With {
        body,
        predicate: match lower_where(ctx.oC_Where()) {
            Some(predicate) => Some(predicate?),
            None => None,
        },
    })
}

fn lower_merge_actions<'input>(ctx: &OC_MergeContext<'input>) -> Result<Vec<MergeAction>> {
    let mut actions = Vec::new();
    for action in ctx.oC_MergeAction_all() {
        let set = required(action.oC_Set(), "a MERGE action")?;
        let mut items = Vec::new();
        for item in set.oC_SetItem_all() {
            items.push(lower_set_item(&item)?);
        }
        actions.push(MergeAction {
            on_match: action.MATCH().is_some(),
            items,
        });
    }
    Ok(actions)
}

fn lower_set_item<'input>(ctx: &OC_SetItemContext<'input>) -> Result<SetItem> {
    let labels = ctx.oC_NodeLabels();
    let variable = ctx.oC_Variable();
    match (ctx.oC_PropertyExpression(), variable, labels) {
        (Some(property), _, _) => Ok(SetItem::Property {
            target: lower_property_expression(&property)?,
            value: lower_expression(required(ctx.oC_Expression(), "a SET value")?.as_ref())?,
        }),
        (None, Some(variable), Some(labels)) => Ok(SetItem::Labels {
            variable: variable_name(&variable)?,
            labels: lower_node_labels(&labels)?,
        }),
        (None, Some(variable), None) => {
            let name = variable_name(&variable)?;
            let value = lower_expression(required(ctx.oC_Expression(), "a SET value")?.as_ref())?;
            // `+=` merges a map into the existing properties; `=` replaces them.
            let merge = child_tokens(ctx).iter().any(|token| token == "+=");
            Ok(if merge {
                SetItem::Merge {
                    variable: name,
                    value,
                }
            } else {
                SetItem::Replace {
                    variable: name,
                    value,
                }
            })
        }
        (None, None, _) => Err(parse_error("SET item has neither a variable nor a property")),
    }
}

fn lower_remove_item<'input>(ctx: &OC_RemoveItemContext<'input>) -> Result<RemoveItem> {
    if let Some(labels) = ctx.oC_NodeLabels() {
        let variable = required(ctx.oC_Variable(), "a REMOVE variable")?;
        return Ok(RemoveItem::Labels {
            variable: variable_name(&variable)?,
            labels: lower_node_labels(&labels)?,
        });
    }
    Ok(RemoveItem::Property(lower_property_expression(required(
        ctx.oC_PropertyExpression(),
        "a REMOVE property",
    )?.as_ref())?))
}

fn lower_property_expression<'input>(
    ctx: &OC_PropertyExpressionContext<'input>,
) -> Result<AstExpr> {
    let mut expr = lower_atom(required(ctx.oC_Atom(), "an operand")?.as_ref())?;
    for lookup in ctx.oC_PropertyLookup_all() {
        let key_ctx = required(lookup.oC_PropertyKeyName(), "a property key")?;
        let key = schema_name(required(key_ctx.oC_SchemaName(), "a property key")?.as_ref());
        expr = AstExpr::Property {
            base: Box::new(expr),
            key,
        };
    }
    Ok(expr)
}

fn lower_projection_body<'input>(ctx: &OC_ProjectionBodyContext<'input>) -> Result<ProjectionBody> {
    let items_ctx = required(ctx.oC_ProjectionItems(), "projection items")?;
    let mut items = Vec::new();
    for item in items_ctx.oC_ProjectionItem_all() {
        let expr = lower_expression(required(item.oC_Expression(), "a projected expression")?.as_ref())?;
        let alias = match item.oC_Variable() {
            Some(variable) => Some(variable_name(&variable)?),
            None => None,
        };
        items.push(ProjectionItem { expr, alias });
    }
    let mut order = Vec::new();
    if let Some(order_ctx) = ctx.oC_Order() {
        for item in order_ctx.oC_SortItem_all() {
            let descending = item.DESC().is_some() || item.DESCENDING().is_some();
            order.push(SortItem {
                expr: lower_expression(required(item.oC_Expression(), "an ORDER BY key")?.as_ref())?,
                descending,
            });
        }
    }
    let skip = match ctx.oC_Skip() {
        Some(skip) => Some(lower_expression(required(skip.oC_Expression(), "a SKIP value")?.as_ref())?),
        None => None,
    };
    let limit = match ctx.oC_Limit() {
        Some(limit) => Some(lower_expression(required(
            limit.oC_Expression(),
            "a LIMIT value",
        )?.as_ref())?),
        None => None,
    };
    Ok(ProjectionBody {
        distinct: ctx.DISTINCT().is_some(),
        items,
        order,
        skip,
        limit,
    })
}

fn lower_where<'input>(ctx: Option<Rc<OC_WhereContext<'input>>>) -> Option<Result<AstExpr>> {
    ctx.map(|where_| match where_.oC_Expression() {
        Some(expr) => lower_expression(&expr),
        None => Err(parse_error("WHERE is missing its predicate")),
    })
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

fn lower_pattern<'input>(ctx: &OC_PatternContext<'input>) -> Result<Pattern> {
    let mut parts = Vec::new();
    for part in ctx.oC_PatternPart_all() {
        parts.push(lower_pattern_part(&part)?);
    }
    Ok(Pattern { parts })
}

fn lower_pattern_part<'input>(ctx: &OC_PatternPartContext<'input>) -> Result<PatternPart> {
    let element = required(ctx.oC_AnonymousPatternPart(), "a pattern element")?;
    let element = required(element.oC_PatternElement(), "a pattern element")?;
    let variable = match ctx.oC_Variable() {
        Some(variable) => Some(variable_name(&variable)?),
        None => None,
    };
    Ok(PatternPart {
        variable,
        element: lower_pattern_element(&element)?,
    })
}

fn lower_pattern_element<'input>(ctx: &OC_PatternElementContext<'input>) -> Result<PatternElement> {
    // `'(' oC_PatternElement ')'` nests; unwrap it so patterns flatten.
    if ctx.oC_NodePattern().is_none() {
        if let Some(inner) = ctx.oC_PatternElement() {
            return lower_pattern_element(&inner);
        }
        return Err(parse_error("pattern element has no node pattern"));
    }
    let start = lower_node_pattern(required(ctx.oC_NodePattern(), "a node pattern")?.as_ref())?;
    let mut chains = Vec::new();
    for chain in ctx.oC_PatternElementChain_all() {
        let relationship =
            lower_relationship_pattern(required(chain.oC_RelationshipPattern(), "a relationship")?.as_ref())?;
        let node = lower_node_pattern(required(chain.oC_NodePattern(), "a node pattern")?.as_ref())?;
        chains.push((relationship, node));
    }
    Ok(PatternElement { start, chains })
}

fn lower_node_pattern<'input>(ctx: &OC_NodePatternContext<'input>) -> Result<NodePattern> {
    let variable = match ctx.oC_Variable() {
        Some(variable) => Some(variable_name(&variable)?),
        None => None,
    };
    let labels = match ctx.oC_NodeLabels() {
        Some(labels) => lower_node_labels(&labels)?,
        None => Vec::new(),
    };
    let properties = match ctx.oC_Properties() {
        Some(properties) => Some(Box::new(lower_properties(&properties)?)),
        None => None,
    };
    Ok(NodePattern {
        variable,
        labels,
        properties,
    })
}

fn lower_node_labels<'input>(ctx: &OC_NodeLabelsContext<'input>) -> Result<Vec<String>> {
    let mut labels = Vec::new();
    for label in ctx.oC_NodeLabel_all() {
        let name = required(label.oC_LabelName(), "a label name")?;
        labels.push(schema_name(required(name.oC_SchemaName(), "a label name")?.as_ref()));
    }
    Ok(labels)
}

fn lower_relationship_pattern<'input>(
    ctx: &OC_RelationshipPatternContext<'input>,
) -> Result<RelationshipPattern> {
    let incoming = ctx.oC_LeftArrowHead().is_some();
    let outgoing = ctx.oC_RightArrowHead().is_some();
    let direction = match (incoming, outgoing) {
        (false, true) => Direction::Outgoing,
        (true, false) => Direction::Incoming,
        _ => Direction::Undirected,
    };

    let mut variable = None;
    let mut types = Vec::new();
    let mut range = None;
    let mut properties = None;
    if let Some(inner) = ctx.oC_RelationshipDetail() {
        if let Some(name) = inner.oC_Variable() {
            variable = Some(variable_name(&name)?);
        }
        if let Some(types_ctx) = inner.oC_RelationshipTypes() {
            for type_name in types_ctx.oC_RelTypeName_all() {
                types.push(schema_name(required(
                    type_name.oC_SchemaName(),
                    "a relationship type",
                )?.as_ref()));
            }
        }
        if let Some(range_ctx) = inner.oC_RangeLiteral() {
            range = Some(lower_range_literal(&range_ctx)?);
        }
        if let Some(props) = inner.oC_Properties() {
            properties = Some(Box::new(lower_properties(&props)?));
        }
    }
    Ok(RelationshipPattern {
        variable,
        types,
        direction,
        range,
        properties,
    })
}

/// `*`, `*n`, `*..m`, `*n..`, `*n..m` in the order the grammar writes them.
enum RangePart {
    Bound(u32),
    Dots,
}

fn lower_range_literal<'input>(ctx: &OC_RangeLiteralContext<'input>) -> Result<Range> {
    // Reading the parts in source order is what distinguishes `*n..` from
    // `*..n`; the two have the same child count.
    let mut parts = Vec::new();
    for child in ctx.get_children() {
        if let Ok(integer) = child.clone().downcast_rc::<OC_IntegerLiteralContext<'input>>() {
            let value = lower_integer_literal(&integer)?;
            let value = u32::try_from(value)
                .map_err(|_| parse_error("a relationship bound must be a non-negative integer"))?;
            parts.push(RangePart::Bound(value));
        } else if let Ok(node) = child.downcast_rc::<TerminalNode<'input, CypherParserContextType>>() {
            if node.symbol.get_text() == ".." {
                parts.push(RangePart::Dots);
            }
        }
    }
    Ok(match parts.as_slice() {
        // `*` means "one or more".
        [] => Range {
            min: None,
            max: None,
        },
        // `*n` means exactly n hops.
        [RangePart::Bound(value)] => Range {
            min: Some(*value),
            max: Some(*value),
        },
        // `*..` means "one or more".
        [RangePart::Dots] => Range {
            min: None,
            max: None,
        },
        [RangePart::Bound(min), RangePart::Dots] => Range {
            min: Some(*min),
            max: None,
        },
        [RangePart::Dots, RangePart::Bound(max)] => Range {
            min: None,
            max: Some(*max),
        },
        [RangePart::Bound(min), RangePart::Dots, RangePart::Bound(max)] => Range {
            min: Some(*min),
            max: Some(*max),
        },
        _ => {
            return Err(parse_error(
                "a variable-length relationship has an unexpected bound list",
            ))
        }
    })
}

fn lower_properties<'input>(ctx: &OC_PropertiesContext<'input>) -> Result<AstExpr> {
    if let Some(map) = ctx.oC_MapLiteral() {
        return lower_map_literal(&map);
    }
    if let Some(parameter) = ctx.oC_Parameter() {
        return lower_parameter(&parameter);
    }
    Err(parse_error("properties must be a map literal or a parameter"))
}

fn lower_map_literal<'input>(ctx: &OC_MapLiteralContext<'input>) -> Result<AstExpr> {
    let keys = ctx.oC_PropertyKeyName_all();
    let values = ctx.oC_Expression_all();
    if keys.len() != values.len() {
        return Err(parse_error("a map literal has mismatched keys and values"));
    }
    let mut entries = Vec::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        let name = schema_name(required(key.oC_SchemaName(), "a map key")?.as_ref());
        entries.push((name, lower_expression(value)?));
    }
    Ok(AstExpr::Map(entries))
}

fn lower_parameter<'input>(ctx: &OC_ParameterContext<'input>) -> Result<AstExpr> {
    if let Some(name) = ctx.oC_SymbolicName() {
        return Ok(AstExpr::Parameter(context_text(&*name)));
    }
    if let Some(digits) = ctx.DecimalInteger() {
        return Ok(AstExpr::Parameter(format!(
            "{}",
            digits.symbol.get_text()
        )));
    }
    Err(parse_error("a parameter needs a name or a position"))
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

/// Lower an `oC_Expression` node.
pub fn lower_expression<'input>(ctx: &OC_ExpressionContext<'input>) -> Result<AstExpr> {
    let or = required(ctx.oC_OrExpression(), "an expression body")?;
    lower_or_expression(&or)
}

fn lower_or_expression<'input>(ctx: &OC_OrExpressionContext<'input>) -> Result<AstExpr> {
    let parts = ctx.oC_XorExpression_all();
    let mut iter = parts.iter();
    let first = iter
        .next()
        .ok_or_else(|| parse_error("OR expression has no operands"))?;
    let mut expr = lower_xor_expression(first)?;
    for part in iter {
        expr = AstExpr::Binary {
            op: BinaryOp::Or,
            left: Box::new(expr),
            right: Box::new(lower_xor_expression(part)?),
        };
    }
    Ok(expr)
}

fn lower_xor_expression<'input>(ctx: &OC_XorExpressionContext<'input>) -> Result<AstExpr> {
    let parts = ctx.oC_AndExpression_all();
    let mut iter = parts.iter();
    let first = iter
        .next()
        .ok_or_else(|| parse_error("XOR expression has no operands"))?;
    let mut expr = lower_and_expression(first)?;
    for part in iter {
        expr = AstExpr::Binary {
            op: BinaryOp::Xor,
            left: Box::new(expr),
            right: Box::new(lower_and_expression(part)?),
        };
    }
    Ok(expr)
}

fn lower_and_expression<'input>(ctx: &OC_AndExpressionContext<'input>) -> Result<AstExpr> {
    let parts = ctx.oC_NotExpression_all();
    let mut iter = parts.iter();
    let first = iter
        .next()
        .ok_or_else(|| parse_error("AND expression has no operands"))?;
    let mut expr = lower_not_expression(first)?;
    for part in iter {
        expr = AstExpr::Binary {
            op: BinaryOp::And,
            left: Box::new(expr),
            right: Box::new(lower_not_expression(part)?),
        };
    }
    Ok(expr)
}

fn lower_not_expression<'input>(ctx: &OC_NotExpressionContext<'input>) -> Result<AstExpr> {
    let mut expr = lower_comparison_expression(required(
        ctx.oC_ComparisonExpression(),
        "a comparison",
    )?.as_ref())?;
    // Count the NOT terminals by their source text rather than through the
    // generated accessor list, which reports each matched token once per
    // repetition of the grammar's `(NOT SP?)*` group.
    let negations = child_tokens(ctx)
        .iter()
        .filter(|token| token.eq_ignore_ascii_case("not"))
        .count();
    for _ in 0..negations {
        expr = AstExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(expr),
        };
    }
    Ok(expr)
}

fn lower_comparison_expression<'input>(ctx: &OC_ComparisonExpressionContext<'input>) -> Result<AstExpr> {
    let base = required(ctx.oC_AddOrSubtractExpression(), "a comparison operand")?;
    let mut expr = lower_add_or_subtract(&base)?;
    for partial in ctx.oC_PartialComparisonExpression_all() {
        let operator = child_tokens(&*partial)
            .into_iter()
            .next()
            .ok_or_else(|| parse_error("a comparison is missing its operator"))?;
        let rhs = lower_add_or_subtract(required(
            partial.oC_AddOrSubtractExpression(),
            "a comparison operand",
        )?.as_ref())?;
        let op = match operator.as_str() {
            "=" => BinaryOp::Eq,
            "<>" => BinaryOp::NotEq,
            "<" => BinaryOp::Less,
            ">" => BinaryOp::Greater,
            "<=" => BinaryOp::LessOrEqual,
            ">=" => BinaryOp::GreaterOrEqual,
            other => {
                return Err(Error::Unsupported(format!(
                    "comparison operator {other} is outside the requested scope"
                )))
            }
        };
        expr = AstExpr::Binary {
            op,
            left: Box::new(expr),
            right: Box::new(rhs),
        };
    }
    Ok(expr)
}

fn lower_add_or_subtract<'input>(ctx: &OC_AddOrSubtractExpressionContext<'input>) -> Result<AstExpr> {
    let parts = ctx.oC_MultiplyDivideModuloExpression_all();
    let operators: Vec<String> = child_tokens(ctx)
        .into_iter()
        .filter(|token| token == "+" || token == "-")
        .collect();
    if parts.len() != operators.len() + 1 {
        return Err(parse_error("an arithmetic expression has mismatched operands and operators"));
    }
    let mut expr = lower_multiply_divide_modulo(&parts[0])?;
    for (index, operator) in operators.iter().enumerate() {
        let op = if operator == "+" {
            BinaryOp::Add
        } else {
            BinaryOp::Subtract
        };
        expr = AstExpr::Binary {
            op,
            left: Box::new(expr),
            right: Box::new(lower_multiply_divide_modulo(&parts[index + 1])?),
        };
    }
    Ok(expr)
}

fn lower_multiply_divide_modulo<'input>(
    ctx: &OC_MultiplyDivideModuloExpressionContext<'input>,
) -> Result<AstExpr> {
    let parts = ctx.oC_PowerOfExpression_all();
    let operators: Vec<String> = child_tokens(ctx)
        .into_iter()
        .filter(|token| token == "*" || token == "/" || token == "%")
        .collect();
    if parts.len() != operators.len() + 1 {
        return Err(parse_error("an arithmetic expression has mismatched operands and operators"));
    }
    let mut expr = lower_power_of_expression(&parts[0])?;
    for (index, operator) in operators.iter().enumerate() {
        let op = match operator.as_str() {
            "*" => BinaryOp::Multiply,
            "/" => BinaryOp::Divide,
            _ => BinaryOp::Modulo,
        };
        expr = AstExpr::Binary {
            op,
            left: Box::new(expr),
            right: Box::new(lower_power_of_expression(&parts[index + 1])?),
        };
    }
    Ok(expr)
}

fn lower_power_of_expression<'input>(ctx: &OC_PowerOfExpressionContext<'input>) -> Result<AstExpr> {
    let parts = ctx.oC_UnaryAddOrSubtractExpression_all();
    let operators: Vec<String> = child_tokens(ctx)
        .into_iter()
        .filter(|token| token == "^")
        .collect();
    if parts.len() != operators.len() + 1 {
        return Err(parse_error("a power expression has mismatched operands and operators"));
    }
    let mut expr = lower_unary_add_or_subtract(&parts[0])?;
    for (index, _) in operators.iter().enumerate() {
        expr = AstExpr::Binary {
            op: BinaryOp::Power,
            left: Box::new(expr),
            right: Box::new(lower_unary_add_or_subtract(&parts[index + 1])?),
        };
    }
    Ok(expr)
}

fn lower_unary_add_or_subtract<'input>(
    ctx: &OC_UnaryAddOrSubtractExpressionContext<'input>,
) -> Result<AstExpr> {
    let mut expr = lower_string_list_null(required(
        ctx.oC_StringListNullOperatorExpression(),
        "an operand",
    )?.as_ref())?;
    // Prefix signs bind right to left, so apply them in reverse source order.
    let signs: Vec<String> = child_tokens(ctx)
        .into_iter()
        .filter(|token| token == "+" || token == "-")
        .collect();
    for sign in signs.iter().rev() {
        if sign == "-" {
            expr = AstExpr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(expr),
            };
        }
    }
    Ok(expr)
}

fn lower_string_list_null<'input>(
    ctx: &OC_StringListNullOperatorExpressionContext<'input>,
) -> Result<AstExpr> {
    let mut expr = lower_property_or_labels(required(
        ctx.oC_PropertyOrLabelsExpression(),
        "an operand",
    )?.as_ref())?;
    // The operators apply in source order, which the child list preserves.
    for child in ctx.get_children() {
        if let Ok(string_op) = child.clone().downcast_rc::<OC_StringOperatorExpressionContext<'input>>() {
            let right = lower_property_or_labels(required(
                string_op.oC_PropertyOrLabelsExpression(),
                "a string operand",
            )?.as_ref())?;
            let op = if string_op.STARTS().is_some() {
                StringOp::StartsWith
            } else if string_op.ENDS().is_some() {
                StringOp::EndsWith
            } else if string_op.CONTAINS().is_some() {
                StringOp::Contains
            } else {
                return Err(parse_error("unknown string operator"));
            };
            expr = AstExpr::StringOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        } else if let Ok(list_op) = child.clone().downcast_rc::<OC_ListOperatorExpressionContext<'input>>() {
            expr = lower_list_operator(&list_op, expr)?;
        } else if let Ok(null_op) = child.clone().downcast_rc::<OC_NullOperatorExpressionContext<'input>>() {
            expr = AstExpr::IsNull {
                operand: Box::new(expr),
                negated: null_op.NOT().is_some(),
            };
        }
    }
    Ok(expr)
}

fn lower_list_operator<'input>(
    ctx: &OC_ListOperatorExpressionContext<'input>,
    base: AstExpr,
) -> Result<AstExpr> {
    if ctx.IN().is_some() {
        let right = lower_property_or_labels(required(
            ctx.oC_PropertyOrLabelsExpression(),
            "an IN operand",
        )?.as_ref())?;
        return Ok(AstExpr::In {
            item: Box::new(base),
            list: Box::new(right),
        });
    }
    // The `..` position matters: `[..n]` has an upper bound and `[n..]` a lower
    // one, and both have the same number of child expressions. Reading the
    // children in source order is the only way to tell them apart.
    enum Part<'i> {
        Dots,
        Bound(Rc<OC_ExpressionContext<'i>>),
    }
    let mut parts = Vec::new();
    for child in ctx.get_children() {
        if let Ok(node) = child
            .clone()
            .downcast_rc::<TerminalNode<'input, CypherParserContextType>>()
        {
            if node.symbol.get_text() == ".." {
                parts.push(Part::Dots);
            }
        } else if let Ok(expression) = child.downcast_rc::<OC_ExpressionContext<'input>>() {
            parts.push(Part::Bound(expression));
        }
    }
    let dots = parts.iter().position(|part| matches!(part, Part::Dots));
    let Some(dots) = dots else {
        // No range: a single-expression subscript.
        let index = parts
            .into_iter()
            .find_map(|part| match part {
                Part::Bound(expression) => Some(expression),
                Part::Dots => None,
            })
            .ok_or_else(|| parse_error("list subscript is missing its index"))?;
        return Ok(AstExpr::ListIndex {
            list: Box::new(base),
            index: Box::new(lower_expression(&index)?),
        });
    };
    let mut lower_bound = None;
    let mut upper_bound = None;
    for (index, part) in parts.into_iter().enumerate() {
        let Part::Bound(expression) = part else { continue };
        let lowered = lower_expression(&expression)?;
        if index < dots {
            lower_bound = Some(Box::new(lowered));
        } else {
            upper_bound = Some(Box::new(lowered));
        }
    }
    Ok(AstExpr::ListSlice {
        list: Box::new(base),
        from: lower_bound,
        to: upper_bound,
    })
}

fn lower_property_or_labels<'input>(
    ctx: &OC_PropertyOrLabelsExpressionContext<'input>,
) -> Result<AstExpr> {
    let mut expr = lower_atom(required(ctx.oC_Atom(), "an operand")?.as_ref())?;
    for lookup in ctx.oC_PropertyLookup_all() {
        let key_ctx = required(lookup.oC_PropertyKeyName(), "a property key")?;
        let key = schema_name(required(key_ctx.oC_SchemaName(), "a property key")?.as_ref());
        expr = AstExpr::Property {
            base: Box::new(expr),
            key,
        };
    }
    if let Some(labels) = ctx.oC_NodeLabels() {
        expr = AstExpr::Labels {
            base: Box::new(expr),
            labels: lower_node_labels(&labels)?,
        };
    }
    Ok(expr)
}

fn lower_atom<'input>(ctx: &OC_AtomContext<'input>) -> Result<AstExpr> {
    if let Some(literal) = ctx.oC_Literal() {
        return lower_literal(&literal);
    }
    if let Some(parameter) = ctx.oC_Parameter() {
        return lower_parameter(&parameter);
    }
    if let Some(case) = ctx.oC_CaseExpression() {
        return lower_case_expression(&case);
    }
    if ctx.COUNT().is_some() {
        return Ok(AstExpr::CountAll);
    }
    if let Some(comprehension) = ctx.oC_ListComprehension() {
        return lower_list_comprehension(&comprehension);
    }
    if let Some(comprehension) = ctx.oC_PatternComprehension() {
        return lower_pattern_comprehension(&comprehension);
    }
    if let Some(filter) = ctx.oC_FilterExpression() {
        let kind = if ctx.ALL().is_some() {
            QuantifierKind::All
        } else if ctx.ANY().is_some() {
            QuantifierKind::Any
        } else if ctx.NONE().is_some() {
            QuantifierKind::None
        } else if ctx.SINGLE().is_some() {
            QuantifierKind::Single
        } else {
            return Err(parse_error("unknown list quantifier"));
        };
        return lower_quantifier(&filter, kind);
    }
    if let Some(pattern) = ctx.oC_RelationshipsPattern() {
        return Ok(AstExpr::Path(lower_relationships_pattern(&pattern)?));
    }
    if let Some(parenthesized) = ctx.oC_ParenthesizedExpression() {
        return lower_expression(required(
            parenthesized.oC_Expression(),
            "a parenthesized expression",
        )?.as_ref());
    }
    if let Some(invocation) = ctx.oC_FunctionInvocation() {
        return lower_function_invocation(&invocation);
    }
    if let Some(variable) = ctx.oC_Variable() {
        return Ok(AstExpr::Variable(variable_name(&variable)?));
    }
    Err(Error::Unsupported(
        "this expression form is outside the requested scope".into(),
    ))
}

fn lower_quantifier<'input>(
    ctx: &OC_FilterExpressionContext<'input>,
    kind: QuantifierKind,
) -> Result<AstExpr> {
    let id_in_coll = required(ctx.oC_IdInColl(), "a quantified list")?;
    let variable = variable_name(required(id_in_coll.oC_Variable(), "a quantifier variable")?.as_ref())?;
    let list = lower_expression(required(id_in_coll.oC_Expression(), "a quantified list")?.as_ref())?;
    let filter = match lower_where(ctx.oC_Where()) {
        Some(filter) => filter?,
        None => AstExpr::Bool(true),
    };
    Ok(AstExpr::Quantifier {
        kind,
        variable,
        list: Box::new(list),
        filter: Box::new(filter),
    })
}

fn lower_list_comprehension<'input>(
    ctx: &OC_ListComprehensionContext<'input>,
) -> Result<AstExpr> {
    let filter = required(ctx.oC_FilterExpression(), "a comprehension variable")?;
    let id_in_coll = required(filter.oC_IdInColl(), "a comprehension list")?;
    let variable = variable_name(required(id_in_coll.oC_Variable(), "a comprehension variable")?.as_ref())?;
    let list = lower_expression(required(id_in_coll.oC_Expression(), "a comprehension list")?.as_ref())?;
    let predicate = match lower_where(filter.oC_Where()) {
        Some(predicate) => Some(Box::new(predicate?)),
        None => None,
    };
    let projection = match ctx.oC_Expression() {
        Some(expr) => Some(Box::new(lower_expression(&expr)?)),
        None => None,
    };
    Ok(AstExpr::ListComprehension {
        variable,
        list: Box::new(list),
        filter: predicate,
        projection,
    })
}

fn lower_pattern_comprehension<'input>(
    ctx: &OC_PatternComprehensionContext<'input>,
) -> Result<AstExpr> {
    let pattern = lower_relationships_pattern(required(
        ctx.oC_RelationshipsPattern(),
        "a pattern comprehension body",
    )?.as_ref())?;
    let variable = match ctx.oC_Variable() {
        Some(variable) => Some(variable_name(&variable)?),
        None => None,
    };
    let mut expressions = ctx.oC_Expression_all().into_iter();
    let first = expressions
        .next()
        .ok_or_else(|| parse_error("a pattern comprehension needs a projection"))?;
    // `oC_PatternComprehension` puts an optional WHERE expression first and the
    // projection last.
    let (filter, projection) = if ctx.WHERE().is_some() {
        let projection = expressions
            .next()
            .ok_or_else(|| parse_error("a pattern comprehension needs a projection"))?;
        (Some(Box::new(lower_expression(&first)?)), lower_expression(&projection)?)
    } else {
        (None, lower_expression(&first)?)
    };
    Ok(AstExpr::PatternComprehension {
        variable,
        pattern,
        filter,
        projection: Box::new(projection),
    })
}

fn lower_relationships_pattern<'input>(ctx: &OC_RelationshipsPatternContext<'input>) -> Result<Pattern> {
    let start = lower_node_pattern(required(ctx.oC_NodePattern(), "a node pattern")?.as_ref())?;
    let mut chains = Vec::new();
    for chain in ctx.oC_PatternElementChain_all() {
        let relationship =
            lower_relationship_pattern(required(chain.oC_RelationshipPattern(), "a relationship")?.as_ref())?;
        let node = lower_node_pattern(required(chain.oC_NodePattern(), "a node pattern")?.as_ref())?;
        chains.push((relationship, node));
    }
    Ok(Pattern {
        parts: vec![PatternPart {
            variable: None,
            element: PatternElement { start, chains },
        }],
    })
}

fn lower_function_invocation<'input>(ctx: &OC_FunctionInvocationContext<'input>) -> Result<AstExpr> {
    let name_ctx = required(ctx.oC_FunctionName(), "a function name")?;
    let name = if name_ctx.EXISTS().is_some() {
        "exists".to_string()
    } else {
        let mut parts = Vec::new();
        if let Some(namespace) = name_ctx.oC_Namespace() {
            for part in namespace.oC_SymbolicName_all() {
                parts.push(context_text(&*part));
            }
        }
        parts.push(context_text(&*required(
            name_ctx.oC_SymbolicName(),
            "a function name",
        )?));
        parts.join(".")
    };
    let mut args = Vec::new();
    for arg in ctx.oC_Expression_all() {
        args.push(lower_expression(&arg)?);
    }
    Ok(AstExpr::Function {
        name,
        distinct: ctx.DISTINCT().is_some(),
        args,
    })
}

fn lower_case_expression<'input>(ctx: &OC_CaseExpressionContext<'input>) -> Result<AstExpr> {
    let expressions = ctx.oC_Expression_all();
    let alternatives_ctx = ctx.oC_CaseAlternatives_all();
    // The rule owns at most two expressions of its own: an optional operand
    // before the first WHEN, and an optional ELSE value after the alternatives.
    let has_else = ctx.ELSE().is_some();
    let head_count = expressions.len() - usize::from(has_else);
    let operand = if head_count == 1 {
        Some(Box::new(lower_expression(&expressions[0])?))
    } else {
        None
    };
    let mut alternatives = Vec::new();
    for alternative in alternatives_ctx.iter() {
        let parts = alternative.oC_Expression_all();
        if parts.len() != 2 {
            return Err(parse_error("a CASE alternative needs a condition and a result"));
        }
        alternatives.push((lower_expression(&parts[0])?, lower_expression(&parts[1])?));
    }
    // `ELSE <expr>` is the last expression of the rule, after the alternatives.
    let fallback = if has_else {
        let else_expr = expressions
            .last()
            .ok_or_else(|| parse_error("ELSE is missing its expression"))?;
        Some(Box::new(lower_expression(else_expr)?))
    } else {
        None
    };
    Ok(AstExpr::Case {
        operand,
        alternatives,
        fallback,
    })
}

fn lower_literal<'input>(ctx: &OC_LiteralContext<'input>) -> Result<AstExpr> {
    if let Some(number) = ctx.oC_NumberLiteral() {
        return lower_number_literal(&number);
    }
    if let Some(text) = ctx.StringLiteral() {
        return Ok(AstExpr::String(unescape_string(
            text.symbol.get_text(),
        )));
    }
    if let Some(boolean) = ctx.oC_BooleanLiteral() {
        return Ok(AstExpr::Bool(boolean.TRUE().is_some()));
    }
    if ctx.NULL().is_some() {
        return Ok(AstExpr::Null);
    }
    if let Some(map) = ctx.oC_MapLiteral() {
        return lower_map_literal(&map);
    }
    if let Some(list) = ctx.oC_ListLiteral() {
        let mut items = Vec::new();
        for expr in list.oC_Expression_all() {
            items.push(lower_expression(&expr)?);
        }
        return Ok(AstExpr::List(items));
    }
    Err(parse_error("unrecognized literal"))
}

fn lower_number_literal<'input>(ctx: &OC_NumberLiteralContext<'input>) -> Result<AstExpr> {
    if let Some(double) = ctx.oC_DoubleLiteral() {
        let text = if let Some(value) = double.ExponentDecimalReal() {
            value.symbol.get_text().to_string()
        } else if let Some(value) = double.RegularDecimalReal() {
            value.symbol.get_text().to_string()
        } else {
            return Err(parse_error("a double literal has no digits"));
        };
        return text
            .parse::<f64>()
            .map(AstExpr::Float)
            .map_err(|_| parse_error(format!("'{text}' is not a valid double literal")));
    }
    if let Some(integer) = ctx.oC_IntegerLiteral() {
        return Ok(AstExpr::Int(lower_integer_literal(&integer)?));
    }
    Err(parse_error("unrecognized number literal"))
}

fn lower_integer_literal<'input>(ctx: &OC_IntegerLiteralContext<'input>) -> Result<i64> {
    if let Some(hex) = ctx.HexInteger() {
        let text = hex.symbol.get_text().to_string();
        let digits = text.trim_start_matches("0x").trim_start_matches("0X");
        return i64::from_str_radix(digits, 16)
            .map_err(|_| parse_error(format!("'{text}' is not a valid hexadecimal integer")));
    }
    if let Some(octal) = ctx.OctalInteger() {
        let text = octal.symbol.get_text().to_string();
        let digits = text.trim_start_matches('0');
        let digits = if digits.is_empty() { "0" } else { digits };
        return i64::from_str_radix(digits, 8)
            .map_err(|_| parse_error(format!("'{text}' is not a valid octal integer")));
    }
    if let Some(decimal) = ctx.DecimalInteger() {
        let text = decimal.symbol.get_text().to_string();
        return text
            .parse::<i64>()
            .map_err(|_| parse_error(format!("'{text}' is not a valid integer")));
    }
    Err(parse_error("an integer literal has no digits"))
}

/// Remove the surrounding quotes and resolve escape sequences.
fn unescape_string(raw: &str) -> String {
    let trimmed = raw
        .strip_prefix('\'')
        .and_then(|rest| rest.strip_suffix('\''))
        .or_else(|| raw.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')))
        .unwrap_or(raw);
    let mut out = String::with_capacity(trimmed.len());
    let mut chars = trimmed.chars();
    while let Some(current) = chars.next() {
        if current != '\\' {
            out.push(current);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('b') => out.push('\u{8}'),
            Some('f') => out.push('\u{c}'),
            Some('u') => {
                let digits: String = chars.by_ref().take(4).collect();
                if let Ok(code) = u32::from_str_radix(&digits, 16) {
                    if let Some(character) = char::from_u32(code) {
                        out.push(character);
                    }
                }
            }
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Expose the lowering entry point to the parser wrapper without forcing it to
/// import the whole generated API.
pub fn lower_root<'input>(root: &OC_CypherContext<'input>) -> Result<CypherAst> {
    lower_cypher(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use antlr4rust::common_token_stream::CommonTokenStream;
    use antlr4rust::error_strategy::BailErrorStrategy;
    use antlr4rust::input_stream::InputStream;
    use metagdb_cypher_antlr::cypherlexer::CypherLexer;

    /// Parse with the repository-generated frontend, then lower its CST.
    fn lower_source(source: &str) -> Result<CypherAst> {
        let stream = InputStream::new(source);
        let lexer = CypherLexer::new(stream);
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = CypherParser::new(tokens);
        parser.set_error_strategy(Box::new(BailErrorStrategy::<CypherParserContextType>::new()));
        let root = parser.oC_Cypher().map_err(lower_antlr_error)?;
        lower_cypher(&root)
    }

    fn clauses(source: &str) -> Vec<Clause> {
        match lower_source(source).expect("source lowers").body {
            AstBody::Union { mut branches, .. } => branches.remove(0),
            other => panic!("expected a clause list, got {other:?}"),
        }
    }

    fn projection(source: &str) -> ProjectionBody {
        clauses(source)
            .pop()
            .and_then(|clause| match clause {
                Clause::Return(body) => Some(body),
                _ => None,
            })
            .expect("last clause is RETURN")
    }

    #[test]
    fn a_standalone_call_is_a_root_alternative() {
        let body = lower_source("CALL pagerank() YIELD node RETURN count(*)").expect("lowers");
        // `CALL … YIELD … RETURN …` is an in-query call, so it stays a clause list.
        match body.body {
            AstBody::Union { branches, .. } => match &branches[0][0] {
                Clause::Call(call) => {
                    assert_eq!(call.name, "pagerank");
                    let yields = call.yields.as_ref().expect("yield items");
                    assert_eq!(yields.items[0].alias, "node");
                }
                other => panic!("expected a call clause, got {other:?}"),
            },
            other => panic!("expected a union body, got {other:?}"),
        }

        let standalone = lower_source("CALL bfs(7)").expect("lowers");
        match standalone.body {
            AstBody::Call(call) => {
                assert_eq!(call.name, "bfs");
                assert_eq!(call.args, vec![AstExpr::Int(7)]);
                assert!(call.yields.is_none());
            }
            other => panic!("expected a standalone call, got {other:?}"),
        }
    }

    #[test]
    fn an_algorithm_call_lowers_its_scalar_arguments() {
        let body = lower_source("CALL pagerank(0.85) YIELD node, rank").expect("lowers");
        let AstBody::Call(call) = body.body else {
            panic!("a CALL without a following clause is a standalone call");
        };
        assert_eq!(call.name, "pagerank");
        assert_eq!(call.args, vec![AstExpr::Float(0.85)]);
        let yields = call.yields.as_ref().unwrap();
        assert!(!yields.all);
        assert_eq!(yields.items.len(), 2);

        let in_query = lower_source("CALL pagerank(0.5) YIELD node RETURN count(*)").expect("lowers");
        let AstBody::Union { branches, .. } = in_query.body else {
            panic!("a CALL with a following clause is an in-query clause");
        };
        let Clause::Call(call) = &branches[0][0] else {
            panic!("expected a call clause");
        };
        assert_eq!(call.args, vec![AstExpr::Float(0.5)]);
    }

    #[test]
    fn arithmetic_precedence_is_preserved() {
        let body = projection("RETURN 1 + 2 * 3");
        let expr = &body.items[0].expr;
        match expr {
            AstExpr::Binary {
                op: BinaryOp::Add,
                left,
                right,
            } => {
                assert_eq!(**left, AstExpr::Int(1));
                match &**right {
                    AstExpr::Binary {
                        op: BinaryOp::Multiply,
                        left,
                        right,
                    } => {
                        assert_eq!(**left, AstExpr::Int(2));
                        assert_eq!(**right, AstExpr::Int(3));
                    }
                    other => panic!("expected multiplication, got {other:?}"),
                }
            }
            other => panic!("expected addition at the root, got {other:?}"),
        }
    }

    #[test]
    fn comparison_binds_tighter_than_and_which_binds_tighter_than_or() {
        let body = projection("RETURN 1 < 2 AND 3 > 4 OR NOT true");
        match &body.items[0].expr {
            AstExpr::Binary {
                op: BinaryOp::Or,
                left,
                right,
            } => {
                match &**left {
                    AstExpr::Binary {
                        op: BinaryOp::And,
                        left,
                        right,
                    } => {
                        assert!(matches!(
                            **left,
                            AstExpr::Binary {
                                op: BinaryOp::Less,
                                ..
                            }
                        ));
                        assert!(matches!(
                            **right,
                            AstExpr::Binary {
                                op: BinaryOp::Greater,
                                ..
                            }
                        ));
                    }
                    other => panic!("expected AND, got {other:?}"),
                }
                assert!(matches!(
                    **right,
                    AstExpr::Unary {
                        op: UnaryOp::Not,
                        ..
                    }
                ));
            }
            other => panic!("expected OR at the root, got {other:?}"),
        }
    }

    #[test]
    fn unary_minus_and_parentheses_lower_as_written() {
        let body = projection("RETURN -(1 + 2)");
        match &body.items[0].expr {
            AstExpr::Unary {
                op: UnaryOp::Negate,
                operand,
            } => assert_eq!(**operand, AstExpr::Binary {
                op: BinaryOp::Add,
                left: Box::new(AstExpr::Int(1)),
                right: Box::new(AstExpr::Int(2)),
            }),
            other => panic!("expected negation, got {other:?}"),
        }
    }

    #[test]
    fn pattern_direction_types_and_ranges_lower_faithfully() {
        let clause = clauses("MATCH (a:Person)-[r:KNOWS|LIKES*1..3]->(b) RETURN b").remove(0);
        let Clause::Match { pattern, .. } = clause else {
            panic!("expected MATCH");
        };
        let part = &pattern.parts[0];
        assert_eq!(part.element.start.variable.as_deref(), Some("a"));
        assert_eq!(part.element.start.labels, vec!["Person".to_string()]);
        let (relationship, node) = &part.element.chains[0];
        assert_eq!(relationship.variable.as_deref(), Some("r"));
        assert_eq!(relationship.types, vec!["KNOWS".to_string(), "LIKES".to_string()]);
        assert_eq!(relationship.direction, Direction::Outgoing);
        assert_eq!(
            relationship.range,
            Some(Range {
                min: Some(1),
                max: Some(3)
            })
        );
        assert_eq!(node.variable.as_deref(), Some("b"));

        let incoming = clauses("MATCH (a)<-[:KNOWS]-(b) RETURN a").remove(0);
        let Clause::Match { pattern, .. } = incoming else {
            panic!("expected MATCH");
        };
        assert_eq!(pattern.parts[0].element.chains[0].0.direction, Direction::Incoming);
    }

    #[test]
    fn variable_length_bounds_distinguish_leading_and_trailing_integers() {
        for (source, expected) in [
            ("MATCH (a)-[:KNOWS*]->(b) RETURN b", Range { min: None, max: None }),
            ("MATCH (a)-[:KNOWS*2]->(b) RETURN b", Range { min: Some(2), max: Some(2) }),
            ("MATCH (a)-[:KNOWS*2..]->(b) RETURN b", Range { min: Some(2), max: None }),
            ("MATCH (a)-[:KNOWS*..3]->(b) RETURN b", Range { min: None, max: Some(3) }),
            ("MATCH (a)-[:KNOWS*1..2]->(b) RETURN b", Range { min: Some(1), max: Some(2) }),
        ] {
            let clause = clauses(source).remove(0);
            let Clause::Match { pattern, .. } = clause else {
                panic!("expected MATCH for {source}");
            };
            assert_eq!(
                pattern.parts[0].element.chains[0].0.range,
                Some(expected),
                "range mismatch for {source}"
            );
        }
    }

    #[test]
    fn optional_match_and_where_are_preserved() {
        let clause = clauses("OPTIONAL MATCH (n) WHERE n.age > 1 RETURN n").remove(0);
        match clause {
            Clause::Match {
                optional,
                predicate,
                pattern,
            } => {
                assert!(optional);
                assert!(predicate.is_some());
                assert_eq!(pattern.parts.len(), 1);
            }
            other => panic!("expected MATCH, got {other:?}"),
        }
    }

    #[test]
    fn dml_clauses_lower_with_their_modifiers() {
        let create = clauses("CREATE (n:Person {name: 'A'})").remove(0);
        let Clause::Create(pattern) = create else {
            panic!("expected CREATE");
        };
        assert_eq!(pattern.parts[0].element.start.labels, vec!["Person".to_string()]);
        assert!(pattern.parts[0].element.start.properties.is_some());

        let delete = clauses("MATCH (n) DETACH DELETE n").remove(1);
        match delete {
            Clause::Delete { detach, targets } => {
                assert!(detach);
                assert_eq!(targets, vec![AstExpr::Variable("n".into())]);
            }
            other => panic!("expected DELETE, got {other:?}"),
        }

        // The vendored grammar requires no space after a comma between SET items.
        let set = clauses("MATCH (n) SET n.age = 1,n:Admin").remove(1);
        match set {
            Clause::Set(items) => {
                assert_eq!(items.len(), 2);
                assert!(matches!(items[0], SetItem::Property { .. }));
                assert!(matches!(items[1], SetItem::Labels { .. }));
            }
            other => panic!("expected SET, got {other:?}"),
        }

        let remove = clauses("MATCH (n) REMOVE n.age, n:Person").remove(1);
        match remove {
            Clause::Remove(items) => {
                assert_eq!(items.len(), 2);
                assert!(matches!(items[0], RemoveItem::Property(_)));
                assert!(matches!(items[1], RemoveItem::Labels { .. }));
            }
            other => panic!("expected REMOVE, got {other:?}"),
        }

        let merge = clauses("MERGE (n:Person {name: 'F'}) ON CREATE SET n.age = 1").remove(0);
        match merge {
            Clause::Merge { pattern, actions } => {
                assert!(pattern.variable.is_none());
                assert_eq!(actions.len(), 1);
                assert!(!actions[0].on_match);
            }
            other => panic!("expected MERGE, got {other:?}"),
        }
    }

    #[test]
    fn union_chains_distinguish_all_from_distinct() {
        assert_eq!(
            lower_source("RETURN 1 UNION RETURN 2").expect("lowers").body,
            AstBody::Union {
                all: false,
                branches: vec![
                    vec![Clause::Return(ProjectionBody {
                        distinct: false,
                        items: vec![ProjectionItem {
                            expr: AstExpr::Int(1),
                            alias: None
                        }],
                        ..ProjectionBody::default()
                    })],
                    vec![Clause::Return(ProjectionBody {
                        distinct: false,
                        items: vec![ProjectionItem {
                            expr: AstExpr::Int(2),
                            alias: None
                        }],
                        ..ProjectionBody::default()
                    })],
                ]
            }
        );

        let all = lower_source("RETURN 1 UNION ALL RETURN 2").expect("lowers");
        match all.body {
            AstBody::Union { all, .. } => assert!(all),
            other => panic!("expected a union, got {other:?}"),
        }

        let mixed = lower_source("RETURN 1 UNION ALL RETURN 2 UNION RETURN 3");
        let error = mixed.expect_err("mixed union chains are refused");
        assert!(matches!(error, Error::Unsupported(_)), "got {error}");
    }

    #[test]
    fn multi_part_queries_keep_their_clause_order() {
        let clauses = clauses("MATCH (n:Person) WITH n.age AS age MATCH (m) RETURN age, m");
        let shapes: Vec<&str> = clauses
            .iter()
            .map(|clause| match clause {
                Clause::Match { .. } => "match",
                Clause::With { .. } => "with",
                Clause::Return(_) => "return",
                _ => "other",
            })
            .collect();
        assert_eq!(shapes, vec!["match", "with", "match", "return"]);
    }

    #[test]
    fn projection_modifiers_lower_together() {
        let body = projection("MATCH (n) RETURN DISTINCT n.name AS nm ORDER BY nm DESC SKIP 1 LIMIT 2");
        assert!(body.distinct);
        assert_eq!(body.items[0].alias.as_deref(), Some("nm"));
        assert_eq!(body.order.len(), 1);
        assert!(body.order[0].descending);
        assert_eq!(body.skip, Some(AstExpr::Int(1)));
        assert_eq!(body.limit, Some(AstExpr::Int(2)));
    }

    #[test]
    fn literals_lower_to_typed_values() {
        let body = projection("RETURN 30, 1.5, 'a', true, null, [1, 2], {k: 1}");
        let expected = [
            AstExpr::Int(30),
            AstExpr::Float(1.5),
            AstExpr::String("a".into()),
            AstExpr::Bool(true),
            AstExpr::Null,
            AstExpr::List(vec![AstExpr::Int(1), AstExpr::Int(2)]),
            AstExpr::Map(vec![("k".to_string(), AstExpr::Int(1))]),
        ];
        let actual: Vec<AstExpr> = body.items.iter().map(|item| item.expr.clone()).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn numeric_literal_bases_are_decoded() {
        let body = projection("RETURN 0x10, 017, 42");
        let values: Vec<i64> = body
            .items
            .iter()
            .map(|item| match item.expr {
                AstExpr::Int(value) => value,
                ref other => panic!("expected an integer, got {other:?}"),
            })
            .collect();
        assert_eq!(values, vec![16, 15, 42]);
    }

    #[test]
    fn string_escapes_are_resolved() {
        let body = projection("RETURN 'a\\'b\\n'");
        assert_eq!(body.items[0].expr, AstExpr::String("a'b\n".to_string()));
    }

    #[test]
    fn collection_and_quantifier_forms_lower() {
        let body = projection(
            "RETURN [x IN [1, 2] WHERE x > 1 | x * 2], head([1]), count(*), coalesce(null, 1), \
             [1, 2, 3][0], [1, 2, 3][1..2], 1 IN [1], 'a' STARTS WITH 'a'",
        );
        assert!(matches!(
            body.items[0].expr,
            AstExpr::ListComprehension { .. }
        ));
        assert!(matches!(body.items[1].expr, AstExpr::Function { .. }));
        assert_eq!(body.items[2].expr, AstExpr::CountAll);
        assert!(matches!(body.items[3].expr, AstExpr::Function { .. }));
        assert!(matches!(body.items[4].expr, AstExpr::ListIndex { .. }));
        assert!(matches!(body.items[5].expr, AstExpr::ListSlice { .. }));
        assert!(matches!(body.items[6].expr, AstExpr::In { .. }));
        assert!(matches!(
            body.items[7].expr,
            AstExpr::StringOp {
                op: StringOp::StartsWith,
                ..
            }
        ));

        let quantified = projection("RETURN all(x IN [1] WHERE x > 0)");
        match &quantified.items[0].expr {
            AstExpr::Quantifier { kind, variable, .. } => {
                assert_eq!(*kind, QuantifierKind::All);
                assert_eq!(variable, "x");
            }
            other => panic!("expected a quantifier, got {other:?}"),
        }
    }

    #[test]
    fn case_and_property_access_lower() {
        let body = projection("RETURN CASE WHEN true THEN 1 ELSE 2 END, n.age, n:Person");
        match &body.items[0].expr {
            AstExpr::Case {
                operand,
                alternatives,
                fallback,
            } => {
                assert!(operand.is_none());
                assert_eq!(alternatives.len(), 1);
                assert!(fallback.is_some());
            }
            other => panic!("expected CASE, got {other:?}"),
        }
        match &body.items[1].expr {
            AstExpr::Property { base, key } => {
                assert_eq!(**base, AstExpr::Variable("n".into()));
                assert_eq!(key, "age");
            }
            other => panic!("expected a property access, got {other:?}"),
        }
        match &body.items[2].expr {
            AstExpr::Labels { base, labels } => {
                assert_eq!(**base, AstExpr::Variable("n".into()));
                assert_eq!(labels, &vec!["Person".to_string()]);
            }
            other => panic!("expected a label predicate, got {other:?}"),
        }
    }

    #[test]
    fn a_pattern_expression_lowers_to_a_path() {
        let body = projection("RETURN (a {name: 'A'})-[:KNOWS]->(b)");
        let AstExpr::Path(pattern) = &body.items[0].expr else {
            panic!("expected a path expression");
        };
        assert_eq!(pattern.parts[0].element.chains.len(), 1);
    }

    #[test]
    fn unwind_and_call_clauses_lower() {
        let unwind = clauses("UNWIND [1, 2] AS x RETURN x").remove(0);
        match unwind {
            Clause::Unwind { expr, alias } => {
                assert_eq!(expr, AstExpr::List(vec![AstExpr::Int(1), AstExpr::Int(2)]));
                assert_eq!(alias, "x");
            }
            other => panic!("expected UNWIND, got {other:?}"),
        }

        let call = clauses("CALL bfs(1) YIELD node RETURN count(*)").remove(0);
        let Clause::Call(call) = call else {
            panic!("expected CALL");
        };
        assert_eq!(call.name, "bfs");
        assert_eq!(call.args, vec![AstExpr::Int(1)]);
    }

    #[test]
    fn invalid_source_fails_in_the_generated_grammar_not_in_lowering() {
        for source in [
            "MATCH (n RETURN n",
            "MATCH (n) RETURN",
            "RETURN 1 +",
            "g.V().count()",
            "MATCH (n) SET RETURN n",
        ] {
            let error = lower_source(source).expect_err("must not parse");
            assert!(
                matches!(error, Error::Parse(_)),
                "{source} produced {error:?} instead of a parse error"
            );
        }
    }
}
