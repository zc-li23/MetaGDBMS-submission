#![allow(nonstandard_style)]
// Generated from data/language/GQL.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::gqlparser::*;

pub trait GQLListener<'input> : ParseTreeListener<'input,GQLParserContextType>{
/**
 * Enter a parse tree produced by {@link GQLParser#gqlRequest}.
 * @param ctx the parse tree
 */
fn enter_gqlRequest(&mut self, _ctx: &GqlRequestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#gqlRequest}.
 * @param ctx the parse tree
 */
fn exit_gqlRequest(&mut self, _ctx: &GqlRequestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#gqlProgram}.
 * @param ctx the parse tree
 */
fn enter_gqlProgram(&mut self, _ctx: &GqlProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#gqlProgram}.
 * @param ctx the parse tree
 */
fn exit_gqlProgram(&mut self, _ctx: &GqlProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#programActivity}.
 * @param ctx the parse tree
 */
fn enter_programActivity(&mut self, _ctx: &ProgramActivityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#programActivity}.
 * @param ctx the parse tree
 */
fn exit_programActivity(&mut self, _ctx: &ProgramActivityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionActivity}.
 * @param ctx the parse tree
 */
fn enter_sessionActivity(&mut self, _ctx: &SessionActivityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionActivity}.
 * @param ctx the parse tree
 */
fn exit_sessionActivity(&mut self, _ctx: &SessionActivityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionActivityCommand}.
 * @param ctx the parse tree
 */
fn enter_sessionActivityCommand(&mut self, _ctx: &SessionActivityCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionActivityCommand}.
 * @param ctx the parse tree
 */
fn exit_sessionActivityCommand(&mut self, _ctx: &SessionActivityCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#transactionActivity}.
 * @param ctx the parse tree
 */
fn enter_transactionActivity(&mut self, _ctx: &TransactionActivityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#transactionActivity}.
 * @param ctx the parse tree
 */
fn exit_transactionActivity(&mut self, _ctx: &TransactionActivityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endTransactionCommand}.
 * @param ctx the parse tree
 */
fn enter_endTransactionCommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endTransactionCommand}.
 * @param ctx the parse tree
 */
fn exit_endTransactionCommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetCommand}.
 * @param ctx the parse tree
 */
fn enter_sessionSetCommand(&mut self, _ctx: &SessionSetCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetCommand}.
 * @param ctx the parse tree
 */
fn exit_sessionSetCommand(&mut self, _ctx: &SessionSetCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetSchemaClause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetSchemaClause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetGraphClause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetGraphClause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetTimeZoneClause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetTimeZoneClause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setTimeZoneValue}.
 * @param ctx the parse tree
 */
fn enter_setTimeZoneValue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setTimeZoneValue}.
 * @param ctx the parse tree
 */
fn exit_setTimeZoneValue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetParameterClause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetParameterClause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetGraphParameterClause(&mut self, _ctx: &SessionSetGraphParameterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetGraphParameterClause(&mut self, _ctx: &SessionSetGraphParameterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetBindingTableParameterClause(&mut self, _ctx: &SessionSetBindingTableParameterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetBindingTableParameterClause(&mut self, _ctx: &SessionSetBindingTableParameterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
 * @param ctx the parse tree
 */
fn enter_sessionSetValueParameterClause(&mut self, _ctx: &SessionSetValueParameterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
 * @param ctx the parse tree
 */
fn exit_sessionSetValueParameterClause(&mut self, _ctx: &SessionSetValueParameterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionSetParameterName}.
 * @param ctx the parse tree
 */
fn enter_sessionSetParameterName(&mut self, _ctx: &SessionSetParameterNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionSetParameterName}.
 * @param ctx the parse tree
 */
fn exit_sessionSetParameterName(&mut self, _ctx: &SessionSetParameterNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionResetCommand}.
 * @param ctx the parse tree
 */
fn enter_sessionResetCommand(&mut self, _ctx: &SessionResetCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionResetCommand}.
 * @param ctx the parse tree
 */
fn exit_sessionResetCommand(&mut self, _ctx: &SessionResetCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionResetArguments}.
 * @param ctx the parse tree
 */
fn enter_sessionResetArguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionResetArguments}.
 * @param ctx the parse tree
 */
fn exit_sessionResetArguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sessionCloseCommand}.
 * @param ctx the parse tree
 */
fn enter_sessionCloseCommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sessionCloseCommand}.
 * @param ctx the parse tree
 */
fn exit_sessionCloseCommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#startTransactionCommand}.
 * @param ctx the parse tree
 */
fn enter_startTransactionCommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#startTransactionCommand}.
 * @param ctx the parse tree
 */
fn exit_startTransactionCommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#transactionCharacteristics}.
 * @param ctx the parse tree
 */
fn enter_transactionCharacteristics(&mut self, _ctx: &TransactionCharacteristicsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#transactionCharacteristics}.
 * @param ctx the parse tree
 */
fn exit_transactionCharacteristics(&mut self, _ctx: &TransactionCharacteristicsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#transactionMode}.
 * @param ctx the parse tree
 */
fn enter_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#transactionMode}.
 * @param ctx the parse tree
 */
fn exit_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#transactionAccessMode}.
 * @param ctx the parse tree
 */
fn enter_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#transactionAccessMode}.
 * @param ctx the parse tree
 */
fn exit_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#implementationDefinedAccessMode}.
 * @param ctx the parse tree
 */
fn enter_implementationDefinedAccessMode(&mut self, _ctx: &ImplementationDefinedAccessModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#implementationDefinedAccessMode}.
 * @param ctx the parse tree
 */
fn exit_implementationDefinedAccessMode(&mut self, _ctx: &ImplementationDefinedAccessModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#rollbackCommand}.
 * @param ctx the parse tree
 */
fn enter_rollbackCommand(&mut self, _ctx: &RollbackCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#rollbackCommand}.
 * @param ctx the parse tree
 */
fn exit_rollbackCommand(&mut self, _ctx: &RollbackCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#commitCommand}.
 * @param ctx the parse tree
 */
fn enter_commitCommand(&mut self, _ctx: &CommitCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#commitCommand}.
 * @param ctx the parse tree
 */
fn exit_commitCommand(&mut self, _ctx: &CommitCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedProcedureSpecification(&mut self, _ctx: &NestedProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedProcedureSpecification(&mut self, _ctx: &NestedProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_procedureSpecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_procedureSpecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_catalogModifyingProcedureSpecification(&mut self, _ctx: &CatalogModifyingProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_catalogModifyingProcedureSpecification(&mut self, _ctx: &CatalogModifyingProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedDataModifyingProcedureSpecification(&mut self, _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedDataModifyingProcedureSpecification(&mut self, _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_dataModifyingProcedureSpecification(&mut self, _ctx: &DataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_dataModifyingProcedureSpecification(&mut self, _ctx: &DataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedQuerySpecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedQuerySpecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#querySpecification}.
 * @param ctx the parse tree
 */
fn enter_querySpecification(&mut self, _ctx: &QuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#querySpecification}.
 * @param ctx the parse tree
 */
fn exit_querySpecification(&mut self, _ctx: &QuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_unsignedNumericLiteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_unsignedNumericLiteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#integerLiteral}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#integerLiteral}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unbrokenSingleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_unbrokenSingleQuotedCharacterSequence(&mut self, _ctx: &UnbrokenSingleQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unbrokenSingleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_unbrokenSingleQuotedCharacterSequence(&mut self, _ctx: &UnbrokenSingleQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unbrokenDoubleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_unbrokenDoubleQuotedCharacterSequence(&mut self, _ctx: &UnbrokenDoubleQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unbrokenDoubleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_unbrokenDoubleQuotedCharacterSequence(&mut self, _ctx: &UnbrokenDoubleQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unbrokenAccentQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_unbrokenAccentQuotedCharacterSequence(&mut self, _ctx: &UnbrokenAccentQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unbrokenAccentQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_unbrokenAccentQuotedCharacterSequence(&mut self, _ctx: &UnbrokenAccentQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#singleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_singleQuotedCharacterSequence(&mut self, _ctx: &SingleQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#singleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_singleQuotedCharacterSequence(&mut self, _ctx: &SingleQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#doubleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_doubleQuotedCharacterSequence(&mut self, _ctx: &DoubleQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#doubleQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_doubleQuotedCharacterSequence(&mut self, _ctx: &DoubleQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#accentQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn enter_accentQuotedCharacterSequence(&mut self, _ctx: &AccentQuotedCharacterSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#accentQuotedCharacterSequence}.
 * @param ctx the parse tree
 */
fn exit_accentQuotedCharacterSequence(&mut self, _ctx: &AccentQuotedCharacterSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unbrokenCharacterStringLiteral}.
 * @param ctx the parse tree
 */
fn enter_unbrokenCharacterStringLiteral(&mut self, _ctx: &UnbrokenCharacterStringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unbrokenCharacterStringLiteral}.
 * @param ctx the parse tree
 */
fn exit_unbrokenCharacterStringLiteral(&mut self, _ctx: &UnbrokenCharacterStringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#temporalLiteral}.
 * @param ctx the parse tree
 */
fn enter_temporalLiteral(&mut self, _ctx: &TemporalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#temporalLiteral}.
 * @param ctx the parse tree
 */
fn exit_temporalLiteral(&mut self, _ctx: &TemporalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sqlDatetimeLiteral}.
 * @param ctx the parse tree
 */
fn enter_sqlDatetimeLiteral(&mut self, _ctx: &SqlDatetimeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sqlDatetimeLiteral}.
 * @param ctx the parse tree
 */
fn exit_sqlDatetimeLiteral(&mut self, _ctx: &SqlDatetimeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dateLiteral}.
 * @param ctx the parse tree
 */
fn enter_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dateLiteral}.
 * @param ctx the parse tree
 */
fn exit_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#timeLiteral}.
 * @param ctx the parse tree
 */
fn enter_timeLiteral(&mut self, _ctx: &TimeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#timeLiteral}.
 * @param ctx the parse tree
 */
fn exit_timeLiteral(&mut self, _ctx: &TimeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeLiteral}.
 * @param ctx the parse tree
 */
fn enter_datetimeLiteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeLiteral}.
 * @param ctx the parse tree
 */
fn exit_datetimeLiteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#durationLiteral}.
 * @param ctx the parse tree
 */
fn enter_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#durationLiteral}.
 * @param ctx the parse tree
 */
fn exit_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sqlIntervalLiteral}.
 * @param ctx the parse tree
 */
fn enter_sqlIntervalLiteral(&mut self, _ctx: &SqlIntervalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sqlIntervalLiteral}.
 * @param ctx the parse tree
 */
fn exit_sqlIntervalLiteral(&mut self, _ctx: &SqlIntervalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sqlIntervalType}.
 * @param ctx the parse tree
 */
fn enter_sqlIntervalType(&mut self, _ctx: &SqlIntervalTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sqlIntervalType}.
 * @param ctx the parse tree
 */
fn exit_sqlIntervalType(&mut self, _ctx: &SqlIntervalTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#delimitedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_delimitedIdentifier(&mut self, _ctx: &DelimitedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#delimitedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_delimitedIdentifier(&mut self, _ctx: &DelimitedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#objectName}.
 * @param ctx the parse tree
 */
fn enter_objectName(&mut self, _ctx: &ObjectNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#objectName}.
 * @param ctx the parse tree
 */
fn exit_objectName(&mut self, _ctx: &ObjectNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
 * @param ctx the parse tree
 */
fn enter_objectNameOrBindingVariable(&mut self, _ctx: &ObjectNameOrBindingVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
 * @param ctx the parse tree
 */
fn exit_objectNameOrBindingVariable(&mut self, _ctx: &ObjectNameOrBindingVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#directoryName}.
 * @param ctx the parse tree
 */
fn enter_directoryName(&mut self, _ctx: &DirectoryNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#directoryName}.
 * @param ctx the parse tree
 */
fn exit_directoryName(&mut self, _ctx: &DirectoryNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#schemaName}.
 * @param ctx the parse tree
 */
fn enter_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#schemaName}.
 * @param ctx the parse tree
 */
fn exit_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphName}.
 * @param ctx the parse tree
 */
fn enter_graphName(&mut self, _ctx: &GraphNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphName}.
 * @param ctx the parse tree
 */
fn exit_graphName(&mut self, _ctx: &GraphNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#delimitedGraphName}.
 * @param ctx the parse tree
 */
fn enter_delimitedGraphName(&mut self, _ctx: &DelimitedGraphNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#delimitedGraphName}.
 * @param ctx the parse tree
 */
fn exit_delimitedGraphName(&mut self, _ctx: &DelimitedGraphNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphTypeName}.
 * @param ctx the parse tree
 */
fn enter_graphTypeName(&mut self, _ctx: &GraphTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphTypeName}.
 * @param ctx the parse tree
 */
fn exit_graphTypeName(&mut self, _ctx: &GraphTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementTypeName}.
 * @param ctx the parse tree
 */
fn enter_elementTypeName(&mut self, _ctx: &ElementTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementTypeName}.
 * @param ctx the parse tree
 */
fn exit_elementTypeName(&mut self, _ctx: &ElementTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableName}.
 * @param ctx the parse tree
 */
fn enter_bindingTableName(&mut self, _ctx: &BindingTableNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableName}.
 * @param ctx the parse tree
 */
fn exit_bindingTableName(&mut self, _ctx: &BindingTableNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
 * @param ctx the parse tree
 */
fn enter_delimitedBindingTableName(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
 * @param ctx the parse tree
 */
fn exit_delimitedBindingTableName(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureName}.
 * @param ctx the parse tree
 */
fn enter_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureName}.
 * @param ctx the parse tree
 */
fn exit_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelName}.
 * @param ctx the parse tree
 */
fn enter_labelName(&mut self, _ctx: &LabelNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelName}.
 * @param ctx the parse tree
 */
fn exit_labelName(&mut self, _ctx: &LabelNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyName}.
 * @param ctx the parse tree
 */
fn enter_propertyName(&mut self, _ctx: &PropertyNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyName}.
 * @param ctx the parse tree
 */
fn exit_propertyName(&mut self, _ctx: &PropertyNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fieldName}.
 * @param ctx the parse tree
 */
fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fieldName}.
 * @param ctx the parse tree
 */
fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parameterName}.
 * @param ctx the parse tree
 */
fn enter_parameterName(&mut self, _ctx: &ParameterNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parameterName}.
 * @param ctx the parse tree
 */
fn exit_parameterName(&mut self, _ctx: &ParameterNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphVariable}.
 * @param ctx the parse tree
 */
fn enter_graphVariable(&mut self, _ctx: &GraphVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphVariable}.
 * @param ctx the parse tree
 */
fn exit_graphVariable(&mut self, _ctx: &GraphVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternVariable}.
 * @param ctx the parse tree
 */
fn enter_graphPatternVariable(&mut self, _ctx: &GraphPatternVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternVariable}.
 * @param ctx the parse tree
 */
fn exit_graphPatternVariable(&mut self, _ctx: &GraphPatternVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathOrSubpathVariable}.
 * @param ctx the parse tree
 */
fn enter_pathOrSubpathVariable(&mut self, _ctx: &PathOrSubpathVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathOrSubpathVariable}.
 * @param ctx the parse tree
 */
fn exit_pathOrSubpathVariable(&mut self, _ctx: &PathOrSubpathVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementVariable}.
 * @param ctx the parse tree
 */
fn enter_elementVariable(&mut self, _ctx: &ElementVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementVariable}.
 * @param ctx the parse tree
 */
fn exit_elementVariable(&mut self, _ctx: &ElementVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathVariable}.
 * @param ctx the parse tree
 */
fn enter_pathVariable(&mut self, _ctx: &PathVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathVariable}.
 * @param ctx the parse tree
 */
fn exit_pathVariable(&mut self, _ctx: &PathVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#subpathVariable}.
 * @param ctx the parse tree
 */
fn enter_subpathVariable(&mut self, _ctx: &SubpathVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#subpathVariable}.
 * @param ctx the parse tree
 */
fn exit_subpathVariable(&mut self, _ctx: &SubpathVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableVariable}.
 * @param ctx the parse tree
 */
fn enter_bindingTableVariable(&mut self, _ctx: &BindingTableVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableVariable}.
 * @param ctx the parse tree
 */
fn exit_bindingTableVariable(&mut self, _ctx: &BindingTableVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#valueVariable}.
 * @param ctx the parse tree
 */
fn enter_valueVariable(&mut self, _ctx: &ValueVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#valueVariable}.
 * @param ctx the parse tree
 */
fn exit_valueVariable(&mut self, _ctx: &ValueVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingVariable}.
 * @param ctx the parse tree
 */
fn enter_bindingVariable(&mut self, _ctx: &BindingVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingVariable}.
 * @param ctx the parse tree
 */
fn exit_bindingVariable(&mut self, _ctx: &BindingVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#predefinedTypeLiteral}.
 * @param ctx the parse tree
 */
fn enter_predefinedTypeLiteral(&mut self, _ctx: &PredefinedTypeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#predefinedTypeLiteral}.
 * @param ctx the parse tree
 */
fn exit_predefinedTypeLiteral(&mut self, _ctx: &PredefinedTypeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#characterStringLiteral}.
 * @param ctx the parse tree
 */
fn enter_characterStringLiteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#characterStringLiteral}.
 * @param ctx the parse tree
 */
fn exit_characterStringLiteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#byteStringLiteral}.
 * @param ctx the parse tree
 */
fn enter_byteStringLiteral(&mut self, _ctx: &ByteStringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#byteStringLiteral}.
 * @param ctx the parse tree
 */
fn exit_byteStringLiteral(&mut self, _ctx: &ByteStringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureBody}.
 * @param ctx the parse tree
 */
fn enter_procedureBody(&mut self, _ctx: &ProcedureBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureBody}.
 * @param ctx the parse tree
 */
fn exit_procedureBody(&mut self, _ctx: &ProcedureBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
 * @param ctx the parse tree
 */
fn enter_bindingVariableDefinitionBlock(&mut self, _ctx: &BindingVariableDefinitionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
 * @param ctx the parse tree
 */
fn exit_bindingVariableDefinitionBlock(&mut self, _ctx: &BindingVariableDefinitionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
 * @param ctx the parse tree
 */
fn enter_bindingVariableDefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
 * @param ctx the parse tree
 */
fn exit_bindingVariableDefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#statementBlock}.
 * @param ctx the parse tree
 */
fn enter_statementBlock(&mut self, _ctx: &StatementBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#statementBlock}.
 * @param ctx the parse tree
 */
fn exit_statementBlock(&mut self, _ctx: &StatementBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nextStatement}.
 * @param ctx the parse tree
 */
fn enter_nextStatement(&mut self, _ctx: &NextStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nextStatement}.
 * @param ctx the parse tree
 */
fn exit_nextStatement(&mut self, _ctx: &NextStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphVariableDefinition}.
 * @param ctx the parse tree
 */
fn enter_graphVariableDefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphVariableDefinition}.
 * @param ctx the parse tree
 */
fn exit_graphVariableDefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
 * @param ctx the parse tree
 */
fn enter_optTypedGraphInitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
 * @param ctx the parse tree
 */
fn exit_optTypedGraphInitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphInitializer}.
 * @param ctx the parse tree
 */
fn enter_graphInitializer(&mut self, _ctx: &GraphInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphInitializer}.
 * @param ctx the parse tree
 */
fn exit_graphInitializer(&mut self, _ctx: &GraphInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
 * @param ctx the parse tree
 */
fn enter_bindingTableVariableDefinition(&mut self, _ctx: &BindingTableVariableDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
 * @param ctx the parse tree
 */
fn exit_bindingTableVariableDefinition(&mut self, _ctx: &BindingTableVariableDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
 * @param ctx the parse tree
 */
fn enter_optTypedBindingTableInitializer(&mut self, _ctx: &OptTypedBindingTableInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
 * @param ctx the parse tree
 */
fn exit_optTypedBindingTableInitializer(&mut self, _ctx: &OptTypedBindingTableInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableInitializer}.
 * @param ctx the parse tree
 */
fn enter_bindingTableInitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableInitializer}.
 * @param ctx the parse tree
 */
fn exit_bindingTableInitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#valueVariableDefinition}.
 * @param ctx the parse tree
 */
fn enter_valueVariableDefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#valueVariableDefinition}.
 * @param ctx the parse tree
 */
fn exit_valueVariableDefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
 * @param ctx the parse tree
 */
fn enter_optTypedValueInitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
 * @param ctx the parse tree
 */
fn exit_optTypedValueInitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphExpression}.
 * @param ctx the parse tree
 */
fn enter_graphExpression(&mut self, _ctx: &GraphExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphExpression}.
 * @param ctx the parse tree
 */
fn exit_graphExpression(&mut self, _ctx: &GraphExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#currentGraph}.
 * @param ctx the parse tree
 */
fn enter_currentGraph(&mut self, _ctx: &CurrentGraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#currentGraph}.
 * @param ctx the parse tree
 */
fn exit_currentGraph(&mut self, _ctx: &CurrentGraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedGraphQuerySpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedGraphQuerySpecification(&mut self, _ctx: &NestedGraphQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedGraphQuerySpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedGraphQuerySpecification(&mut self, _ctx: &NestedGraphQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableExpression}.
 * @param ctx the parse tree
 */
fn enter_bindingTableExpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableExpression}.
 * @param ctx the parse tree
 */
fn exit_bindingTableExpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedBindingTableQuerySpecification(&mut self, _ctx: &NestedBindingTableQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedBindingTableQuerySpecification(&mut self, _ctx: &NestedBindingTableQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
 * @param ctx the parse tree
 */
fn enter_objectExpressionPrimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
 * @param ctx the parse tree
 */
fn exit_objectExpressionPrimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_linearCatalogModifyingStatement(&mut self, _ctx: &LinearCatalogModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_linearCatalogModifyingStatement(&mut self, _ctx: &LinearCatalogModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleCatalogModifyingStatement(&mut self, _ctx: &SimpleCatalogModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleCatalogModifyingStatement(&mut self, _ctx: &SimpleCatalogModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_primitiveCatalogModifyingStatement(&mut self, _ctx: &PrimitiveCatalogModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_primitiveCatalogModifyingStatement(&mut self, _ctx: &PrimitiveCatalogModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#createSchemaStatement}.
 * @param ctx the parse tree
 */
fn enter_createSchemaStatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#createSchemaStatement}.
 * @param ctx the parse tree
 */
fn exit_createSchemaStatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dropSchemaStatement}.
 * @param ctx the parse tree
 */
fn enter_dropSchemaStatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dropSchemaStatement}.
 * @param ctx the parse tree
 */
fn exit_dropSchemaStatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#createGraphStatement}.
 * @param ctx the parse tree
 */
fn enter_createGraphStatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#createGraphStatement}.
 * @param ctx the parse tree
 */
fn exit_createGraphStatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#openGraphType}.
 * @param ctx the parse tree
 */
fn enter_openGraphType(&mut self, _ctx: &OpenGraphTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#openGraphType}.
 * @param ctx the parse tree
 */
fn exit_openGraphType(&mut self, _ctx: &OpenGraphTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#ofGraphType}.
 * @param ctx the parse tree
 */
fn enter_ofGraphType(&mut self, _ctx: &OfGraphTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#ofGraphType}.
 * @param ctx the parse tree
 */
fn exit_ofGraphType(&mut self, _ctx: &OfGraphTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
 * @param ctx the parse tree
 */
fn enter_graphTypeLikeGraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
 * @param ctx the parse tree
 */
fn exit_graphTypeLikeGraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphSource}.
 * @param ctx the parse tree
 */
fn enter_graphSource(&mut self, _ctx: &GraphSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphSource}.
 * @param ctx the parse tree
 */
fn exit_graphSource(&mut self, _ctx: &GraphSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dropGraphStatement}.
 * @param ctx the parse tree
 */
fn enter_dropGraphStatement(&mut self, _ctx: &DropGraphStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dropGraphStatement}.
 * @param ctx the parse tree
 */
fn exit_dropGraphStatement(&mut self, _ctx: &DropGraphStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
 * @param ctx the parse tree
 */
fn enter_createGraphTypeStatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
 * @param ctx the parse tree
 */
fn exit_createGraphTypeStatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphTypeSource}.
 * @param ctx the parse tree
 */
fn enter_graphTypeSource(&mut self, _ctx: &GraphTypeSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphTypeSource}.
 * @param ctx the parse tree
 */
fn exit_graphTypeSource(&mut self, _ctx: &GraphTypeSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#copyOfGraphType}.
 * @param ctx the parse tree
 */
fn enter_copyOfGraphType(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#copyOfGraphType}.
 * @param ctx the parse tree
 */
fn exit_copyOfGraphType(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
 * @param ctx the parse tree
 */
fn enter_dropGraphTypeStatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
 * @param ctx the parse tree
 */
fn exit_dropGraphTypeStatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
 * @param ctx the parse tree
 */
fn enter_callCatalogModifyingProcedureStatement(&mut self, _ctx: &CallCatalogModifyingProcedureStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
 * @param ctx the parse tree
 */
fn exit_callCatalogModifyingProcedureStatement(&mut self, _ctx: &CallCatalogModifyingProcedureStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_linearDataModifyingStatement(&mut self, _ctx: &LinearDataModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_linearDataModifyingStatement(&mut self, _ctx: &LinearDataModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_focusedLinearDataModifyingStatement(&mut self, _ctx: &FocusedLinearDataModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_focusedLinearDataModifyingStatement(&mut self, _ctx: &FocusedLinearDataModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
 * @param ctx the parse tree
 */
fn enter_focusedLinearDataModifyingStatementBody(&mut self, _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
 * @param ctx the parse tree
 */
fn exit_focusedLinearDataModifyingStatementBody(&mut self, _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn enter_focusedNestedDataModifyingProcedureSpecification(&mut self, _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
 * @param ctx the parse tree
 */
fn exit_focusedNestedDataModifyingProcedureSpecification(&mut self, _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_ambientLinearDataModifyingStatement(&mut self, _ctx: &AmbientLinearDataModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_ambientLinearDataModifyingStatement(&mut self, _ctx: &AmbientLinearDataModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
 * @param ctx the parse tree
 */
fn enter_ambientLinearDataModifyingStatementBody(&mut self, _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
 * @param ctx the parse tree
 */
fn exit_ambientLinearDataModifyingStatementBody(&mut self, _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleLinearDataAccessingStatement(&mut self, _ctx: &SimpleLinearDataAccessingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleLinearDataAccessingStatement(&mut self, _ctx: &SimpleLinearDataAccessingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleDataAccessingStatement(&mut self, _ctx: &SimpleDataAccessingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleDataAccessingStatement(&mut self, _ctx: &SimpleDataAccessingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleDataModifyingStatement(&mut self, _ctx: &SimpleDataModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleDataModifyingStatement(&mut self, _ctx: &SimpleDataModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn enter_primitiveDataModifyingStatement(&mut self, _ctx: &PrimitiveDataModifyingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
 * @param ctx the parse tree
 */
fn exit_primitiveDataModifyingStatement(&mut self, _ctx: &PrimitiveDataModifyingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertStatement}.
 * @param ctx the parse tree
 */
fn enter_insertStatement(&mut self, _ctx: &InsertStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertStatement}.
 * @param ctx the parse tree
 */
fn exit_insertStatement(&mut self, _ctx: &InsertStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setStatement}.
 * @param ctx the parse tree
 */
fn enter_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setStatement}.
 * @param ctx the parse tree
 */
fn exit_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setItemList}.
 * @param ctx the parse tree
 */
fn enter_setItemList(&mut self, _ctx: &SetItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setItemList}.
 * @param ctx the parse tree
 */
fn exit_setItemList(&mut self, _ctx: &SetItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setItem}.
 * @param ctx the parse tree
 */
fn enter_setItem(&mut self, _ctx: &SetItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setItem}.
 * @param ctx the parse tree
 */
fn exit_setItem(&mut self, _ctx: &SetItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setPropertyItem}.
 * @param ctx the parse tree
 */
fn enter_setPropertyItem(&mut self, _ctx: &SetPropertyItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setPropertyItem}.
 * @param ctx the parse tree
 */
fn exit_setPropertyItem(&mut self, _ctx: &SetPropertyItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
 * @param ctx the parse tree
 */
fn enter_setAllPropertiesItem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
 * @param ctx the parse tree
 */
fn exit_setAllPropertiesItem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setLabelItem}.
 * @param ctx the parse tree
 */
fn enter_setLabelItem(&mut self, _ctx: &SetLabelItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setLabelItem}.
 * @param ctx the parse tree
 */
fn exit_setLabelItem(&mut self, _ctx: &SetLabelItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelSetSpecification}.
 * @param ctx the parse tree
 */
fn enter_labelSetSpecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelSetSpecification}.
 * @param ctx the parse tree
 */
fn exit_labelSetSpecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#removeStatement}.
 * @param ctx the parse tree
 */
fn enter_removeStatement(&mut self, _ctx: &RemoveStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#removeStatement}.
 * @param ctx the parse tree
 */
fn exit_removeStatement(&mut self, _ctx: &RemoveStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#removeItemList}.
 * @param ctx the parse tree
 */
fn enter_removeItemList(&mut self, _ctx: &RemoveItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#removeItemList}.
 * @param ctx the parse tree
 */
fn exit_removeItemList(&mut self, _ctx: &RemoveItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#removeItem}.
 * @param ctx the parse tree
 */
fn enter_removeItem(&mut self, _ctx: &RemoveItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#removeItem}.
 * @param ctx the parse tree
 */
fn exit_removeItem(&mut self, _ctx: &RemoveItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#removePropertyItem}.
 * @param ctx the parse tree
 */
fn enter_removePropertyItem(&mut self, _ctx: &RemovePropertyItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#removePropertyItem}.
 * @param ctx the parse tree
 */
fn exit_removePropertyItem(&mut self, _ctx: &RemovePropertyItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#removeLabelItem}.
 * @param ctx the parse tree
 */
fn enter_removeLabelItem(&mut self, _ctx: &RemoveLabelItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#removeLabelItem}.
 * @param ctx the parse tree
 */
fn exit_removeLabelItem(&mut self, _ctx: &RemoveLabelItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#deleteStatement}.
 * @param ctx the parse tree
 */
fn enter_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#deleteStatement}.
 * @param ctx the parse tree
 */
fn exit_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#deleteItemList}.
 * @param ctx the parse tree
 */
fn enter_deleteItemList(&mut self, _ctx: &DeleteItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#deleteItemList}.
 * @param ctx the parse tree
 */
fn exit_deleteItemList(&mut self, _ctx: &DeleteItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#deleteItem}.
 * @param ctx the parse tree
 */
fn enter_deleteItem(&mut self, _ctx: &DeleteItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#deleteItem}.
 * @param ctx the parse tree
 */
fn exit_deleteItem(&mut self, _ctx: &DeleteItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
 * @param ctx the parse tree
 */
fn enter_callDataModifyingProcedureStatement(&mut self, _ctx: &CallDataModifyingProcedureStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
 * @param ctx the parse tree
 */
fn exit_callDataModifyingProcedureStatement(&mut self, _ctx: &CallDataModifyingProcedureStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#compositeQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_compositeQueryStatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#compositeQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_compositeQueryStatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#compositeQueryExpression}.
 * @param ctx the parse tree
 */
fn enter_compositeQueryExpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#compositeQueryExpression}.
 * @param ctx the parse tree
 */
fn exit_compositeQueryExpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#queryConjunction}.
 * @param ctx the parse tree
 */
fn enter_queryConjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#queryConjunction}.
 * @param ctx the parse tree
 */
fn exit_queryConjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setOperator}.
 * @param ctx the parse tree
 */
fn enter_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setOperator}.
 * @param ctx the parse tree
 */
fn exit_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
 * @param ctx the parse tree
 */
fn enter_compositeQueryPrimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
 * @param ctx the parse tree
 */
fn exit_compositeQueryPrimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#linearQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_linearQueryStatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#linearQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_linearQueryStatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_focusedLinearQueryStatement(&mut self, _ctx: &FocusedLinearQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_focusedLinearQueryStatement(&mut self, _ctx: &FocusedLinearQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
 * @param ctx the parse tree
 */
fn enter_focusedLinearQueryStatementPart(&mut self, _ctx: &FocusedLinearQueryStatementPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
 * @param ctx the parse tree
 */
fn exit_focusedLinearQueryStatementPart(&mut self, _ctx: &FocusedLinearQueryStatementPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
 * @param ctx the parse tree
 */
fn enter_focusedLinearQueryAndPrimitiveResultStatementPart(&mut self, _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
 * @param ctx the parse tree
 */
fn exit_focusedLinearQueryAndPrimitiveResultStatementPart(&mut self, _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
 * @param ctx the parse tree
 */
fn enter_focusedPrimitiveResultStatement(&mut self, _ctx: &FocusedPrimitiveResultStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
 * @param ctx the parse tree
 */
fn exit_focusedPrimitiveResultStatement(&mut self, _ctx: &FocusedPrimitiveResultStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
 * @param ctx the parse tree
 */
fn enter_focusedNestedQuerySpecification(&mut self, _ctx: &FocusedNestedQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
 * @param ctx the parse tree
 */
fn exit_focusedNestedQuerySpecification(&mut self, _ctx: &FocusedNestedQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_ambientLinearQueryStatement(&mut self, _ctx: &AmbientLinearQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_ambientLinearQueryStatement(&mut self, _ctx: &AmbientLinearQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleLinearQueryStatement(&mut self, _ctx: &SimpleLinearQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleLinearQueryStatement(&mut self, _ctx: &SimpleLinearQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleQueryStatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleQueryStatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_primitiveQueryStatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_primitiveQueryStatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#matchStatement}.
 * @param ctx the parse tree
 */
fn enter_matchStatement(&mut self, _ctx: &MatchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#matchStatement}.
 * @param ctx the parse tree
 */
fn exit_matchStatement(&mut self, _ctx: &MatchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleMatchStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleMatchStatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleMatchStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleMatchStatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#optionalMatchStatement}.
 * @param ctx the parse tree
 */
fn enter_optionalMatchStatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#optionalMatchStatement}.
 * @param ctx the parse tree
 */
fn exit_optionalMatchStatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#optionalOperand}.
 * @param ctx the parse tree
 */
fn enter_optionalOperand(&mut self, _ctx: &OptionalOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#optionalOperand}.
 * @param ctx the parse tree
 */
fn exit_optionalOperand(&mut self, _ctx: &OptionalOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#matchStatementBlock}.
 * @param ctx the parse tree
 */
fn enter_matchStatementBlock(&mut self, _ctx: &MatchStatementBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#matchStatementBlock}.
 * @param ctx the parse tree
 */
fn exit_matchStatementBlock(&mut self, _ctx: &MatchStatementBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#callQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_callQueryStatement(&mut self, _ctx: &CallQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#callQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_callQueryStatement(&mut self, _ctx: &CallQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#filterStatement}.
 * @param ctx the parse tree
 */
fn enter_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#filterStatement}.
 * @param ctx the parse tree
 */
fn exit_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#letStatement}.
 * @param ctx the parse tree
 */
fn enter_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#letStatement}.
 * @param ctx the parse tree
 */
fn exit_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
 * @param ctx the parse tree
 */
fn enter_letVariableDefinitionList(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
 * @param ctx the parse tree
 */
fn exit_letVariableDefinitionList(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#letVariableDefinition}.
 * @param ctx the parse tree
 */
fn enter_letVariableDefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#letVariableDefinition}.
 * @param ctx the parse tree
 */
fn exit_letVariableDefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#forStatement}.
 * @param ctx the parse tree
 */
fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#forStatement}.
 * @param ctx the parse tree
 */
fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#forItem}.
 * @param ctx the parse tree
 */
fn enter_forItem(&mut self, _ctx: &ForItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#forItem}.
 * @param ctx the parse tree
 */
fn exit_forItem(&mut self, _ctx: &ForItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#forItemAlias}.
 * @param ctx the parse tree
 */
fn enter_forItemAlias(&mut self, _ctx: &ForItemAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#forItemAlias}.
 * @param ctx the parse tree
 */
fn exit_forItemAlias(&mut self, _ctx: &ForItemAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
 * @param ctx the parse tree
 */
fn enter_forOrdinalityOrOffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
 * @param ctx the parse tree
 */
fn exit_forOrdinalityOrOffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
 * @param ctx the parse tree
 */
fn enter_orderByAndPageStatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
 * @param ctx the parse tree
 */
fn exit_orderByAndPageStatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#primitiveResultStatement}.
 * @param ctx the parse tree
 */
fn enter_primitiveResultStatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#primitiveResultStatement}.
 * @param ctx the parse tree
 */
fn exit_primitiveResultStatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#returnStatement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#returnStatement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#returnStatementBody}.
 * @param ctx the parse tree
 */
fn enter_returnStatementBody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#returnStatementBody}.
 * @param ctx the parse tree
 */
fn exit_returnStatementBody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#returnItemList}.
 * @param ctx the parse tree
 */
fn enter_returnItemList(&mut self, _ctx: &ReturnItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#returnItemList}.
 * @param ctx the parse tree
 */
fn exit_returnItemList(&mut self, _ctx: &ReturnItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#returnItem}.
 * @param ctx the parse tree
 */
fn enter_returnItem(&mut self, _ctx: &ReturnItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#returnItem}.
 * @param ctx the parse tree
 */
fn exit_returnItem(&mut self, _ctx: &ReturnItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#returnItemAlias}.
 * @param ctx the parse tree
 */
fn enter_returnItemAlias(&mut self, _ctx: &ReturnItemAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#returnItemAlias}.
 * @param ctx the parse tree
 */
fn exit_returnItemAlias(&mut self, _ctx: &ReturnItemAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectStatement}.
 * @param ctx the parse tree
 */
fn enter_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectStatement}.
 * @param ctx the parse tree
 */
fn exit_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectItemList}.
 * @param ctx the parse tree
 */
fn enter_selectItemList(&mut self, _ctx: &SelectItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectItemList}.
 * @param ctx the parse tree
 */
fn exit_selectItemList(&mut self, _ctx: &SelectItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectItem}.
 * @param ctx the parse tree
 */
fn enter_selectItem(&mut self, _ctx: &SelectItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectItem}.
 * @param ctx the parse tree
 */
fn exit_selectItem(&mut self, _ctx: &SelectItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectItemAlias}.
 * @param ctx the parse tree
 */
fn enter_selectItemAlias(&mut self, _ctx: &SelectItemAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectItemAlias}.
 * @param ctx the parse tree
 */
fn exit_selectItemAlias(&mut self, _ctx: &SelectItemAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#havingClause}.
 * @param ctx the parse tree
 */
fn enter_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#havingClause}.
 * @param ctx the parse tree
 */
fn exit_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectStatementBody}.
 * @param ctx the parse tree
 */
fn enter_selectStatementBody(&mut self, _ctx: &SelectStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectStatementBody}.
 * @param ctx the parse tree
 */
fn exit_selectStatementBody(&mut self, _ctx: &SelectStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectGraphMatchList}.
 * @param ctx the parse tree
 */
fn enter_selectGraphMatchList(&mut self, _ctx: &SelectGraphMatchListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectGraphMatchList}.
 * @param ctx the parse tree
 */
fn exit_selectGraphMatchList(&mut self, _ctx: &SelectGraphMatchListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectGraphMatch}.
 * @param ctx the parse tree
 */
fn enter_selectGraphMatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectGraphMatch}.
 * @param ctx the parse tree
 */
fn exit_selectGraphMatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#selectQuerySpecification}.
 * @param ctx the parse tree
 */
fn enter_selectQuerySpecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#selectQuerySpecification}.
 * @param ctx the parse tree
 */
fn exit_selectQuerySpecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#callProcedureStatement}.
 * @param ctx the parse tree
 */
fn enter_callProcedureStatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#callProcedureStatement}.
 * @param ctx the parse tree
 */
fn exit_callProcedureStatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureCall}.
 * @param ctx the parse tree
 */
fn enter_procedureCall(&mut self, _ctx: &ProcedureCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureCall}.
 * @param ctx the parse tree
 */
fn exit_procedureCall(&mut self, _ctx: &ProcedureCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#inlineProcedureCall}.
 * @param ctx the parse tree
 */
fn enter_inlineProcedureCall(&mut self, _ctx: &InlineProcedureCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#inlineProcedureCall}.
 * @param ctx the parse tree
 */
fn exit_inlineProcedureCall(&mut self, _ctx: &InlineProcedureCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#variableScopeClause}.
 * @param ctx the parse tree
 */
fn enter_variableScopeClause(&mut self, _ctx: &VariableScopeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#variableScopeClause}.
 * @param ctx the parse tree
 */
fn exit_variableScopeClause(&mut self, _ctx: &VariableScopeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
 * @param ctx the parse tree
 */
fn enter_bindingVariableReferenceList(&mut self, _ctx: &BindingVariableReferenceListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
 * @param ctx the parse tree
 */
fn exit_bindingVariableReferenceList(&mut self, _ctx: &BindingVariableReferenceListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#namedProcedureCall}.
 * @param ctx the parse tree
 */
fn enter_namedProcedureCall(&mut self, _ctx: &NamedProcedureCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#namedProcedureCall}.
 * @param ctx the parse tree
 */
fn exit_namedProcedureCall(&mut self, _ctx: &NamedProcedureCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureArgumentList}.
 * @param ctx the parse tree
 */
fn enter_procedureArgumentList(&mut self, _ctx: &ProcedureArgumentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureArgumentList}.
 * @param ctx the parse tree
 */
fn exit_procedureArgumentList(&mut self, _ctx: &ProcedureArgumentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureArgument}.
 * @param ctx the parse tree
 */
fn enter_procedureArgument(&mut self, _ctx: &ProcedureArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureArgument}.
 * @param ctx the parse tree
 */
fn exit_procedureArgument(&mut self, _ctx: &ProcedureArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#useGraphClause}.
 * @param ctx the parse tree
 */
fn enter_useGraphClause(&mut self, _ctx: &UseGraphClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#useGraphClause}.
 * @param ctx the parse tree
 */
fn exit_useGraphClause(&mut self, _ctx: &UseGraphClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#atSchemaClause}.
 * @param ctx the parse tree
 */
fn enter_atSchemaClause(&mut self, _ctx: &AtSchemaClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#atSchemaClause}.
 * @param ctx the parse tree
 */
fn exit_atSchemaClause(&mut self, _ctx: &AtSchemaClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingVariableReference}.
 * @param ctx the parse tree
 */
fn enter_bindingVariableReference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingVariableReference}.
 * @param ctx the parse tree
 */
fn exit_bindingVariableReference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementVariableReference}.
 * @param ctx the parse tree
 */
fn enter_elementVariableReference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementVariableReference}.
 * @param ctx the parse tree
 */
fn exit_elementVariableReference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathVariableReference}.
 * @param ctx the parse tree
 */
fn enter_pathVariableReference(&mut self, _ctx: &PathVariableReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathVariableReference}.
 * @param ctx the parse tree
 */
fn exit_pathVariableReference(&mut self, _ctx: &PathVariableReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
 * @param ctx the parse tree
 */
fn enter_graphPatternBindingTable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
 * @param ctx the parse tree
 */
fn exit_graphPatternBindingTable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
 * @param ctx the parse tree
 */
fn enter_graphPatternYieldClause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
 * @param ctx the parse tree
 */
fn exit_graphPatternYieldClause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
 * @param ctx the parse tree
 */
fn enter_graphPatternYieldItemList(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
 * @param ctx the parse tree
 */
fn exit_graphPatternYieldItemList(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
 * @param ctx the parse tree
 */
fn enter_graphPatternYieldItem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
 * @param ctx the parse tree
 */
fn exit_graphPatternYieldItem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPattern}.
 * @param ctx the parse tree
 */
fn enter_graphPattern(&mut self, _ctx: &GraphPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPattern}.
 * @param ctx the parse tree
 */
fn exit_graphPattern(&mut self, _ctx: &GraphPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#matchMode}.
 * @param ctx the parse tree
 */
fn enter_matchMode(&mut self, _ctx: &MatchModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#matchMode}.
 * @param ctx the parse tree
 */
fn exit_matchMode(&mut self, _ctx: &MatchModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
 * @param ctx the parse tree
 */
fn enter_repeatableElementsMatchMode(&mut self, _ctx: &RepeatableElementsMatchModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
 * @param ctx the parse tree
 */
fn exit_repeatableElementsMatchMode(&mut self, _ctx: &RepeatableElementsMatchModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
 * @param ctx the parse tree
 */
fn enter_differentEdgesMatchMode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
 * @param ctx the parse tree
 */
fn exit_differentEdgesMatchMode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
 * @param ctx the parse tree
 */
fn enter_elementBindingsOrElements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
 * @param ctx the parse tree
 */
fn exit_elementBindingsOrElements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
 * @param ctx the parse tree
 */
fn enter_edgeBindingsOrEdges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
 * @param ctx the parse tree
 */
fn exit_edgeBindingsOrEdges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPatternList}.
 * @param ctx the parse tree
 */
fn enter_pathPatternList(&mut self, _ctx: &PathPatternListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPatternList}.
 * @param ctx the parse tree
 */
fn exit_pathPatternList(&mut self, _ctx: &PathPatternListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPattern}.
 * @param ctx the parse tree
 */
fn enter_pathPattern(&mut self, _ctx: &PathPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPattern}.
 * @param ctx the parse tree
 */
fn exit_pathPattern(&mut self, _ctx: &PathPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_pathVariableDeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_pathVariableDeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#keepClause}.
 * @param ctx the parse tree
 */
fn enter_keepClause(&mut self, _ctx: &KeepClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#keepClause}.
 * @param ctx the parse tree
 */
fn exit_keepClause(&mut self, _ctx: &KeepClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
 * @param ctx the parse tree
 */
fn enter_graphPatternWhereClause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
 * @param ctx the parse tree
 */
fn exit_graphPatternWhereClause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPatternPrefix}.
 * @param ctx the parse tree
 */
fn enter_pathPatternPrefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPatternPrefix}.
 * @param ctx the parse tree
 */
fn exit_pathPatternPrefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathModePrefix}.
 * @param ctx the parse tree
 */
fn enter_pathModePrefix(&mut self, _ctx: &PathModePrefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathModePrefix}.
 * @param ctx the parse tree
 */
fn exit_pathModePrefix(&mut self, _ctx: &PathModePrefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathMode}.
 * @param ctx the parse tree
 */
fn enter_pathMode(&mut self, _ctx: &PathModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathMode}.
 * @param ctx the parse tree
 */
fn exit_pathMode(&mut self, _ctx: &PathModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathSearchPrefix}.
 * @param ctx the parse tree
 */
fn enter_pathSearchPrefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathSearchPrefix}.
 * @param ctx the parse tree
 */
fn exit_pathSearchPrefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#allPathSearch}.
 * @param ctx the parse tree
 */
fn enter_allPathSearch(&mut self, _ctx: &AllPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#allPathSearch}.
 * @param ctx the parse tree
 */
fn exit_allPathSearch(&mut self, _ctx: &AllPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathOrPaths}.
 * @param ctx the parse tree
 */
fn enter_pathOrPaths(&mut self, _ctx: &PathOrPathsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathOrPaths}.
 * @param ctx the parse tree
 */
fn exit_pathOrPaths(&mut self, _ctx: &PathOrPathsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#anyPathSearch}.
 * @param ctx the parse tree
 */
fn enter_anyPathSearch(&mut self, _ctx: &AnyPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#anyPathSearch}.
 * @param ctx the parse tree
 */
fn exit_anyPathSearch(&mut self, _ctx: &AnyPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#numberOfPaths}.
 * @param ctx the parse tree
 */
fn enter_numberOfPaths(&mut self, _ctx: &NumberOfPathsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#numberOfPaths}.
 * @param ctx the parse tree
 */
fn exit_numberOfPaths(&mut self, _ctx: &NumberOfPathsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#shortestPathSearch}.
 * @param ctx the parse tree
 */
fn enter_shortestPathSearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#shortestPathSearch}.
 * @param ctx the parse tree
 */
fn exit_shortestPathSearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#allShortestPathSearch}.
 * @param ctx the parse tree
 */
fn enter_allShortestPathSearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#allShortestPathSearch}.
 * @param ctx the parse tree
 */
fn exit_allShortestPathSearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
 * @param ctx the parse tree
 */
fn enter_anyShortestPathSearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
 * @param ctx the parse tree
 */
fn exit_anyShortestPathSearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
 * @param ctx the parse tree
 */
fn enter_countedShortestPathSearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
 * @param ctx the parse tree
 */
fn exit_countedShortestPathSearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
 * @param ctx the parse tree
 */
fn enter_countedShortestGroupSearch(&mut self, _ctx: &CountedShortestGroupSearchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
 * @param ctx the parse tree
 */
fn exit_countedShortestGroupSearch(&mut self, _ctx: &CountedShortestGroupSearchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#numberOfGroups}.
 * @param ctx the parse tree
 */
fn enter_numberOfGroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#numberOfGroups}.
 * @param ctx the parse tree
 */
fn exit_numberOfGroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPatternExpression}.
 * @param ctx the parse tree
 */
fn enter_pathPatternExpression(&mut self, _ctx: &PathPatternExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPatternExpression}.
 * @param ctx the parse tree
 */
fn exit_pathPatternExpression(&mut self, _ctx: &PathPatternExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathMultisetAlternation}.
 * @param ctx the parse tree
 */
fn enter_pathMultisetAlternation(&mut self, _ctx: &PathMultisetAlternationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathMultisetAlternation}.
 * @param ctx the parse tree
 */
fn exit_pathMultisetAlternation(&mut self, _ctx: &PathMultisetAlternationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPatternUnion}.
 * @param ctx the parse tree
 */
fn enter_pathPatternUnion(&mut self, _ctx: &PathPatternUnionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPatternUnion}.
 * @param ctx the parse tree
 */
fn exit_pathPatternUnion(&mut self, _ctx: &PathPatternUnionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathTerm}.
 * @param ctx the parse tree
 */
fn enter_pathTerm(&mut self, _ctx: &PathTermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathTerm}.
 * @param ctx the parse tree
 */
fn exit_pathTerm(&mut self, _ctx: &PathTermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathFactor}.
 * @param ctx the parse tree
 */
fn enter_pathFactor(&mut self, _ctx: &PathFactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathFactor}.
 * @param ctx the parse tree
 */
fn exit_pathFactor(&mut self, _ctx: &PathFactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#quantifiedPathPrimary}.
 * @param ctx the parse tree
 */
fn enter_quantifiedPathPrimary(&mut self, _ctx: &QuantifiedPathPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#quantifiedPathPrimary}.
 * @param ctx the parse tree
 */
fn exit_quantifiedPathPrimary(&mut self, _ctx: &QuantifiedPathPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#questionedPathPrimary}.
 * @param ctx the parse tree
 */
fn enter_questionedPathPrimary(&mut self, _ctx: &QuestionedPathPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#questionedPathPrimary}.
 * @param ctx the parse tree
 */
fn exit_questionedPathPrimary(&mut self, _ctx: &QuestionedPathPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathPrimary}.
 * @param ctx the parse tree
 */
fn enter_pathPrimary(&mut self, _ctx: &PathPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathPrimary}.
 * @param ctx the parse tree
 */
fn exit_pathPrimary(&mut self, _ctx: &PathPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementPattern}.
 * @param ctx the parse tree
 */
fn enter_elementPattern(&mut self, _ctx: &ElementPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementPattern}.
 * @param ctx the parse tree
 */
fn exit_elementPattern(&mut self, _ctx: &ElementPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodePattern}.
 * @param ctx the parse tree
 */
fn enter_nodePattern(&mut self, _ctx: &NodePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodePattern}.
 * @param ctx the parse tree
 */
fn exit_nodePattern(&mut self, _ctx: &NodePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementPatternFiller}.
 * @param ctx the parse tree
 */
fn enter_elementPatternFiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementPatternFiller}.
 * @param ctx the parse tree
 */
fn exit_elementPatternFiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_elementVariableDeclaration(&mut self, _ctx: &ElementVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_elementVariableDeclaration(&mut self, _ctx: &ElementVariableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#isLabelExpression}.
 * @param ctx the parse tree
 */
fn enter_isLabelExpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#isLabelExpression}.
 * @param ctx the parse tree
 */
fn exit_isLabelExpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#isOrColon}.
 * @param ctx the parse tree
 */
fn enter_isOrColon(&mut self, _ctx: &IsOrColonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#isOrColon}.
 * @param ctx the parse tree
 */
fn exit_isOrColon(&mut self, _ctx: &IsOrColonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementPatternPredicate}.
 * @param ctx the parse tree
 */
fn enter_elementPatternPredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementPatternPredicate}.
 * @param ctx the parse tree
 */
fn exit_elementPatternPredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
 * @param ctx the parse tree
 */
fn enter_elementPatternWhereClause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
 * @param ctx the parse tree
 */
fn exit_elementPatternWhereClause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementPropertySpecification}.
 * @param ctx the parse tree
 */
fn enter_elementPropertySpecification(&mut self, _ctx: &ElementPropertySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementPropertySpecification}.
 * @param ctx the parse tree
 */
fn exit_elementPropertySpecification(&mut self, _ctx: &ElementPropertySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
 * @param ctx the parse tree
 */
fn enter_propertyKeyValuePairList(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
 * @param ctx the parse tree
 */
fn exit_propertyKeyValuePairList(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
 * @param ctx the parse tree
 */
fn enter_propertyKeyValuePair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
 * @param ctx the parse tree
 */
fn exit_propertyKeyValuePair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgePattern}.
 * @param ctx the parse tree
 */
fn enter_edgePattern(&mut self, _ctx: &EdgePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgePattern}.
 * @param ctx the parse tree
 */
fn exit_edgePattern(&mut self, _ctx: &EdgePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_fullEdgePattern(&mut self, _ctx: &FullEdgePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_fullEdgePattern(&mut self, _ctx: &FullEdgePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
 * @param ctx the parse tree
 */
fn enter_fullEdgePointingLeft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
 * @param ctx the parse tree
 */
fn exit_fullEdgePointingLeft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeUndirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeUndirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
 * @param ctx the parse tree
 */
fn enter_fullEdgePointingRight(&mut self, _ctx: &FullEdgePointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
 * @param ctx the parse tree
 */
fn exit_fullEdgePointingRight(&mut self, _ctx: &FullEdgePointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeLeftOrUndirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeLeftOrUndirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeUndirectedOrRight(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeUndirectedOrRight(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeLeftOrRight(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeLeftOrRight(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeAnyDirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeAnyDirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgePointingLeft}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgePointingLeft(&mut self, _ctx: &AbbreviatedEdgePointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgePointingLeft}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgePointingLeft(&mut self, _ctx: &AbbreviatedEdgePointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgeUndirected}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeUndirected(&mut self, _ctx: &AbbreviatedEdgeUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgeUndirected}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeUndirected(&mut self, _ctx: &AbbreviatedEdgeUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgePointingRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgePointingRight(&mut self, _ctx: &AbbreviatedEdgePointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgePointingRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgePointingRight(&mut self, _ctx: &AbbreviatedEdgePointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgeLeftOrUndirected}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeLeftOrUndirected(&mut self, _ctx: &AbbreviatedEdgeLeftOrUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgeLeftOrUndirected}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeLeftOrUndirected(&mut self, _ctx: &AbbreviatedEdgeLeftOrUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgeUndirectedOrRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeUndirectedOrRight(&mut self, _ctx: &AbbreviatedEdgeUndirectedOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgeUndirectedOrRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeUndirectedOrRight(&mut self, _ctx: &AbbreviatedEdgeUndirectedOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgeLeftOrRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeLeftOrRight(&mut self, _ctx: &AbbreviatedEdgeLeftOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgeLeftOrRight}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeLeftOrRight(&mut self, _ctx: &AbbreviatedEdgeLeftOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code abbreviatedEdgeAnyDirection}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeAnyDirection(&mut self, _ctx: &AbbreviatedEdgeAnyDirectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abbreviatedEdgeAnyDirection}
 * labeled alternative in {@link GQLParser#abbreviatedEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeAnyDirection(&mut self, _ctx: &AbbreviatedEdgeAnyDirectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedPathPatternExpression(&mut self, _ctx: &ParenthesizedPathPatternExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedPathPatternExpression(&mut self, _ctx: &ParenthesizedPathPatternExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_subpathVariableDeclaration(&mut self, _ctx: &SubpathVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_subpathVariableDeclaration(&mut self, _ctx: &SubpathVariableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedPathPatternWhereClause(&mut self, _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedPathPatternWhereClause(&mut self, _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertGraphPattern}.
 * @param ctx the parse tree
 */
fn enter_insertGraphPattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertGraphPattern}.
 * @param ctx the parse tree
 */
fn exit_insertGraphPattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertPathPatternList}.
 * @param ctx the parse tree
 */
fn enter_insertPathPatternList(&mut self, _ctx: &InsertPathPatternListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertPathPatternList}.
 * @param ctx the parse tree
 */
fn exit_insertPathPatternList(&mut self, _ctx: &InsertPathPatternListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertPathPattern}.
 * @param ctx the parse tree
 */
fn enter_insertPathPattern(&mut self, _ctx: &InsertPathPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertPathPattern}.
 * @param ctx the parse tree
 */
fn exit_insertPathPattern(&mut self, _ctx: &InsertPathPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertNodePattern}.
 * @param ctx the parse tree
 */
fn enter_insertNodePattern(&mut self, _ctx: &InsertNodePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertNodePattern}.
 * @param ctx the parse tree
 */
fn exit_insertNodePattern(&mut self, _ctx: &InsertNodePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertEdgePattern}.
 * @param ctx the parse tree
 */
fn enter_insertEdgePattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertEdgePattern}.
 * @param ctx the parse tree
 */
fn exit_insertEdgePattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
 * @param ctx the parse tree
 */
fn enter_insertEdgePointingLeft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
 * @param ctx the parse tree
 */
fn exit_insertEdgePointingLeft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
 * @param ctx the parse tree
 */
fn enter_insertEdgePointingRight(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
 * @param ctx the parse tree
 */
fn exit_insertEdgePointingRight(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
 * @param ctx the parse tree
 */
fn enter_insertEdgeUndirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
 * @param ctx the parse tree
 */
fn exit_insertEdgeUndirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
 * @param ctx the parse tree
 */
fn enter_insertElementPatternFiller(&mut self, _ctx: &InsertElementPatternFillerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
 * @param ctx the parse tree
 */
fn exit_insertElementPatternFiller(&mut self, _ctx: &InsertElementPatternFillerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
 * @param ctx the parse tree
 */
fn enter_labelAndPropertySetSpecification(&mut self, _ctx: &LabelAndPropertySetSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
 * @param ctx the parse tree
 */
fn exit_labelAndPropertySetSpecification(&mut self, _ctx: &LabelAndPropertySetSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelExpression}.
 * @param ctx the parse tree
 */
fn enter_labelExpression(&mut self, _ctx: &LabelExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelExpression}.
 * @param ctx the parse tree
 */
fn exit_labelExpression(&mut self, _ctx: &LabelExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelTerm}.
 * @param ctx the parse tree
 */
fn enter_labelTerm(&mut self, _ctx: &LabelTermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelTerm}.
 * @param ctx the parse tree
 */
fn exit_labelTerm(&mut self, _ctx: &LabelTermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelFactor}.
 * @param ctx the parse tree
 */
fn enter_labelFactor(&mut self, _ctx: &LabelFactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelFactor}.
 * @param ctx the parse tree
 */
fn exit_labelFactor(&mut self, _ctx: &LabelFactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelPrimary}.
 * @param ctx the parse tree
 */
fn enter_labelPrimary(&mut self, _ctx: &LabelPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelPrimary}.
 * @param ctx the parse tree
 */
fn exit_labelPrimary(&mut self, _ctx: &LabelPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#wildcardLabel}.
 * @param ctx the parse tree
 */
fn enter_wildcardLabel(&mut self, _ctx: &WildcardLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#wildcardLabel}.
 * @param ctx the parse tree
 */
fn exit_wildcardLabel(&mut self, _ctx: &WildcardLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parenthesizedLabelExpression}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedLabelExpression(&mut self, _ctx: &ParenthesizedLabelExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parenthesizedLabelExpression}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedLabelExpression(&mut self, _ctx: &ParenthesizedLabelExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
 * @param ctx the parse tree
 */
fn enter_graphPatternQuantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
 * @param ctx the parse tree
 */
fn exit_graphPatternQuantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fixedQuantifier}.
 * @param ctx the parse tree
 */
fn enter_fixedQuantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fixedQuantifier}.
 * @param ctx the parse tree
 */
fn exit_fixedQuantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#generalQuantifier}.
 * @param ctx the parse tree
 */
fn enter_generalQuantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#generalQuantifier}.
 * @param ctx the parse tree
 */
fn exit_generalQuantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#lowerBound}.
 * @param ctx the parse tree
 */
fn enter_lowerBound(&mut self, _ctx: &LowerBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#lowerBound}.
 * @param ctx the parse tree
 */
fn exit_lowerBound(&mut self, _ctx: &LowerBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#upperBound}.
 * @param ctx the parse tree
 */
fn enter_upperBound(&mut self, _ctx: &UpperBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#upperBound}.
 * @param ctx the parse tree
 */
fn exit_upperBound(&mut self, _ctx: &UpperBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
 * @param ctx the parse tree
 */
fn enter_simplifiedPathPatternExpression(&mut self, _ctx: &SimplifiedPathPatternExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
 * @param ctx the parse tree
 */
fn exit_simplifiedPathPatternExpression(&mut self, _ctx: &SimplifiedPathPatternExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingLeft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingLeft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingUndirected(&mut self, _ctx: &SimplifiedDefaultingUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingUndirected(&mut self, _ctx: &SimplifiedDefaultingUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingRight(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingRight(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingLeftOrUndirected(&mut self, _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingLeftOrUndirected(&mut self, _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingUndirectedOrRight(&mut self, _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingUndirectedOrRight(&mut self, _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingLeftOrRight(&mut self, _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingLeftOrRight(&mut self, _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDefaultingAnyDirection(&mut self, _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDefaultingAnyDirection(&mut self, _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedContents}.
 * @param ctx the parse tree
 */
fn enter_simplifiedContents(&mut self, _ctx: &SimplifiedContentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedContents}.
 * @param ctx the parse tree
 */
fn exit_simplifiedContents(&mut self, _ctx: &SimplifiedContentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
 * @param ctx the parse tree
 */
fn enter_simplifiedPathUnion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
 * @param ctx the parse tree
 */
fn exit_simplifiedPathUnion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
 * @param ctx the parse tree
 */
fn enter_simplifiedMultisetAlternation(&mut self, _ctx: &SimplifiedMultisetAlternationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
 * @param ctx the parse tree
 */
fn exit_simplifiedMultisetAlternation(&mut self, _ctx: &SimplifiedMultisetAlternationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedTerm}.
 * @param ctx the parse tree
 */
fn enter_simplifiedTerm(&mut self, _ctx: &SimplifiedTermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedTerm}.
 * @param ctx the parse tree
 */
fn exit_simplifiedTerm(&mut self, _ctx: &SimplifiedTermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedFactorLow}.
 * @param ctx the parse tree
 */
fn enter_simplifiedFactorLow(&mut self, _ctx: &SimplifiedFactorLowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedFactorLow}.
 * @param ctx the parse tree
 */
fn exit_simplifiedFactorLow(&mut self, _ctx: &SimplifiedFactorLowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
 * @param ctx the parse tree
 */
fn enter_simplifiedFactorHigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
 * @param ctx the parse tree
 */
fn exit_simplifiedFactorHigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedQuantified}.
 * @param ctx the parse tree
 */
fn enter_simplifiedQuantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedQuantified}.
 * @param ctx the parse tree
 */
fn exit_simplifiedQuantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
 * @param ctx the parse tree
 */
fn enter_simplifiedQuestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
 * @param ctx the parse tree
 */
fn exit_simplifiedQuestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedTertiary}.
 * @param ctx the parse tree
 */
fn enter_simplifiedTertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedTertiary}.
 * @param ctx the parse tree
 */
fn exit_simplifiedTertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
 * @param ctx the parse tree
 */
fn enter_simplifiedDirectionOverride(&mut self, _ctx: &SimplifiedDirectionOverrideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
 * @param ctx the parse tree
 */
fn exit_simplifiedDirectionOverride(&mut self, _ctx: &SimplifiedDirectionOverrideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideLeft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideLeft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideUndirected(&mut self, _ctx: &SimplifiedOverrideUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideUndirected(&mut self, _ctx: &SimplifiedOverrideUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideRight(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideRight(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideLeftOrUndirected(&mut self, _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideLeftOrUndirected(&mut self, _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideUndirectedOrRight(&mut self, _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideUndirectedOrRight(&mut self, _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideLeftOrRight(&mut self, _ctx: &SimplifiedOverrideLeftOrRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideLeftOrRight(&mut self, _ctx: &SimplifiedOverrideLeftOrRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
 * @param ctx the parse tree
 */
fn enter_simplifiedOverrideAnyDirection(&mut self, _ctx: &SimplifiedOverrideAnyDirectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
 * @param ctx the parse tree
 */
fn exit_simplifiedOverrideAnyDirection(&mut self, _ctx: &SimplifiedOverrideAnyDirectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedSecondary}.
 * @param ctx the parse tree
 */
fn enter_simplifiedSecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedSecondary}.
 * @param ctx the parse tree
 */
fn exit_simplifiedSecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedNegation}.
 * @param ctx the parse tree
 */
fn enter_simplifiedNegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedNegation}.
 * @param ctx the parse tree
 */
fn exit_simplifiedNegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simplifiedPrimary}.
 * @param ctx the parse tree
 */
fn enter_simplifiedPrimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simplifiedPrimary}.
 * @param ctx the parse tree
 */
fn exit_simplifiedPrimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#whereClause}.
 * @param ctx the parse tree
 */
fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#whereClause}.
 * @param ctx the parse tree
 */
fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#yieldClause}.
 * @param ctx the parse tree
 */
fn enter_yieldClause(&mut self, _ctx: &YieldClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#yieldClause}.
 * @param ctx the parse tree
 */
fn exit_yieldClause(&mut self, _ctx: &YieldClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#yieldItemList}.
 * @param ctx the parse tree
 */
fn enter_yieldItemList(&mut self, _ctx: &YieldItemListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#yieldItemList}.
 * @param ctx the parse tree
 */
fn exit_yieldItemList(&mut self, _ctx: &YieldItemListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#yieldItem}.
 * @param ctx the parse tree
 */
fn enter_yieldItem(&mut self, _ctx: &YieldItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#yieldItem}.
 * @param ctx the parse tree
 */
fn exit_yieldItem(&mut self, _ctx: &YieldItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#yieldItemName}.
 * @param ctx the parse tree
 */
fn enter_yieldItemName(&mut self, _ctx: &YieldItemNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#yieldItemName}.
 * @param ctx the parse tree
 */
fn exit_yieldItemName(&mut self, _ctx: &YieldItemNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#yieldItemAlias}.
 * @param ctx the parse tree
 */
fn enter_yieldItemAlias(&mut self, _ctx: &YieldItemAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#yieldItemAlias}.
 * @param ctx the parse tree
 */
fn exit_yieldItemAlias(&mut self, _ctx: &YieldItemAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#groupByClause}.
 * @param ctx the parse tree
 */
fn enter_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#groupByClause}.
 * @param ctx the parse tree
 */
fn exit_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#groupingElementList}.
 * @param ctx the parse tree
 */
fn enter_groupingElementList(&mut self, _ctx: &GroupingElementListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#groupingElementList}.
 * @param ctx the parse tree
 */
fn exit_groupingElementList(&mut self, _ctx: &GroupingElementListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#groupingElement}.
 * @param ctx the parse tree
 */
fn enter_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#groupingElement}.
 * @param ctx the parse tree
 */
fn exit_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#emptyGroupingSet}.
 * @param ctx the parse tree
 */
fn enter_emptyGroupingSet(&mut self, _ctx: &EmptyGroupingSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#emptyGroupingSet}.
 * @param ctx the parse tree
 */
fn exit_emptyGroupingSet(&mut self, _ctx: &EmptyGroupingSetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#orderByClause}.
 * @param ctx the parse tree
 */
fn enter_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#orderByClause}.
 * @param ctx the parse tree
 */
fn exit_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlCountAllFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlCountAllFunction(&mut self, _ctx: &GqlCountAllFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlCountAllFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlCountAllFunction(&mut self, _ctx: &GqlCountAllFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlGeneralSetFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlGeneralSetFunction(&mut self, _ctx: &GqlGeneralSetFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlGeneralSetFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlGeneralSetFunction(&mut self, _ctx: &GqlGeneralSetFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlBinarySetFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlBinarySetFunction(&mut self, _ctx: &GqlBinarySetFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlBinarySetFunction}
 * labeled alternative in {@link GQLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlBinarySetFunction(&mut self, _ctx: &GqlBinarySetFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#generalSetFunctionType}.
 * @param ctx the parse tree
 */
fn enter_generalSetFunctionType(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#generalSetFunctionType}.
 * @param ctx the parse tree
 */
fn exit_generalSetFunctionType(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn enter_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn exit_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#binarySetFunctionType}.
 * @param ctx the parse tree
 */
fn enter_binarySetFunctionType(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#binarySetFunctionType}.
 * @param ctx the parse tree
 */
fn exit_binarySetFunctionType(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sortSpecificationList}.
 * @param ctx the parse tree
 */
fn enter_sortSpecificationList(&mut self, _ctx: &SortSpecificationListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sortSpecificationList}.
 * @param ctx the parse tree
 */
fn exit_sortSpecificationList(&mut self, _ctx: &SortSpecificationListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sortSpecification}.
 * @param ctx the parse tree
 */
fn enter_sortSpecification(&mut self, _ctx: &SortSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sortSpecification}.
 * @param ctx the parse tree
 */
fn exit_sortSpecification(&mut self, _ctx: &SortSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sortKey}.
 * @param ctx the parse tree
 */
fn enter_sortKey(&mut self, _ctx: &SortKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sortKey}.
 * @param ctx the parse tree
 */
fn exit_sortKey(&mut self, _ctx: &SortKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#orderingSpecification}.
 * @param ctx the parse tree
 */
fn enter_orderingSpecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#orderingSpecification}.
 * @param ctx the parse tree
 */
fn exit_orderingSpecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nullOrdering}.
 * @param ctx the parse tree
 */
fn enter_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nullOrdering}.
 * @param ctx the parse tree
 */
fn exit_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#limitClause}.
 * @param ctx the parse tree
 */
fn enter_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#limitClause}.
 * @param ctx the parse tree
 */
fn exit_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#offsetClause}.
 * @param ctx the parse tree
 */
fn enter_offsetClause(&mut self, _ctx: &OffsetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#offsetClause}.
 * @param ctx the parse tree
 */
fn exit_offsetClause(&mut self, _ctx: &OffsetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#offsetSynonym}.
 * @param ctx the parse tree
 */
fn enter_offsetSynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#offsetSynonym}.
 * @param ctx the parse tree
 */
fn exit_offsetSynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
 * @param ctx the parse tree
 */
fn enter_nestedGraphTypeSpecification(&mut self, _ctx: &NestedGraphTypeSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
 * @param ctx the parse tree
 */
fn exit_nestedGraphTypeSpecification(&mut self, _ctx: &NestedGraphTypeSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
 * @param ctx the parse tree
 */
fn enter_graphTypeSpecificationBody(&mut self, _ctx: &GraphTypeSpecificationBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
 * @param ctx the parse tree
 */
fn exit_graphTypeSpecificationBody(&mut self, _ctx: &GraphTypeSpecificationBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementTypeDefinitionList}.
 * @param ctx the parse tree
 */
fn enter_elementTypeDefinitionList(&mut self, _ctx: &ElementTypeDefinitionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementTypeDefinitionList}.
 * @param ctx the parse tree
 */
fn exit_elementTypeDefinitionList(&mut self, _ctx: &ElementTypeDefinitionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementTypeDefinition}.
 * @param ctx the parse tree
 */
fn enter_elementTypeDefinition(&mut self, _ctx: &ElementTypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementTypeDefinition}.
 * @param ctx the parse tree
 */
fn exit_elementTypeDefinition(&mut self, _ctx: &ElementTypeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypeDefinition}.
 * @param ctx the parse tree
 */
fn enter_nodeTypeDefinition(&mut self, _ctx: &NodeTypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypeDefinition}.
 * @param ctx the parse tree
 */
fn exit_nodeTypeDefinition(&mut self, _ctx: &NodeTypeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypePattern}.
 * @param ctx the parse tree
 */
fn enter_nodeTypePattern(&mut self, _ctx: &NodeTypePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypePattern}.
 * @param ctx the parse tree
 */
fn exit_nodeTypePattern(&mut self, _ctx: &NodeTypePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypePhrase}.
 * @param ctx the parse tree
 */
fn enter_nodeTypePhrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypePhrase}.
 * @param ctx the parse tree
 */
fn exit_nodeTypePhrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypeName}.
 * @param ctx the parse tree
 */
fn enter_nodeTypeName(&mut self, _ctx: &NodeTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypeName}.
 * @param ctx the parse tree
 */
fn exit_nodeTypeName(&mut self, _ctx: &NodeTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypeFiller}.
 * @param ctx the parse tree
 */
fn enter_nodeTypeFiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypeFiller}.
 * @param ctx the parse tree
 */
fn exit_nodeTypeFiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypeLabelSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_nodeTypeLabelSetDefinition(&mut self, _ctx: &NodeTypeLabelSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypeLabelSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_nodeTypeLabelSetDefinition(&mut self, _ctx: &NodeTypeLabelSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypePropertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_nodeTypePropertyTypeSetDefinition(&mut self, _ctx: &NodeTypePropertyTypeSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypePropertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_nodeTypePropertyTypeSetDefinition(&mut self, _ctx: &NodeTypePropertyTypeSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypeDefinition}.
 * @param ctx the parse tree
 */
fn enter_edgeTypeDefinition(&mut self, _ctx: &EdgeTypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypeDefinition}.
 * @param ctx the parse tree
 */
fn exit_edgeTypeDefinition(&mut self, _ctx: &EdgeTypeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypePattern}.
 * @param ctx the parse tree
 */
fn enter_edgeTypePattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypePattern}.
 * @param ctx the parse tree
 */
fn exit_edgeTypePattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypePhrase}.
 * @param ctx the parse tree
 */
fn enter_edgeTypePhrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypePhrase}.
 * @param ctx the parse tree
 */
fn exit_edgeTypePhrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypeName}.
 * @param ctx the parse tree
 */
fn enter_edgeTypeName(&mut self, _ctx: &EdgeTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypeName}.
 * @param ctx the parse tree
 */
fn exit_edgeTypeName(&mut self, _ctx: &EdgeTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypeFiller}.
 * @param ctx the parse tree
 */
fn enter_edgeTypeFiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypeFiller}.
 * @param ctx the parse tree
 */
fn exit_edgeTypeFiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypeLabelSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_edgeTypeLabelSetDefinition(&mut self, _ctx: &EdgeTypeLabelSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypeLabelSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_edgeTypeLabelSetDefinition(&mut self, _ctx: &EdgeTypeLabelSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeTypePropertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_edgeTypePropertyTypeSetDefinition(&mut self, _ctx: &EdgeTypePropertyTypeSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeTypePropertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_edgeTypePropertyTypeSetDefinition(&mut self, _ctx: &EdgeTypePropertyTypeSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeTypePattern}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeTypePattern(&mut self, _ctx: &FullEdgeTypePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeTypePattern}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeTypePattern(&mut self, _ctx: &FullEdgeTypePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeTypePatternPointingRight}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeTypePatternPointingRight(&mut self, _ctx: &FullEdgeTypePatternPointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeTypePatternPointingRight}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeTypePatternPointingRight(&mut self, _ctx: &FullEdgeTypePatternPointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeTypePatternPointingLeft}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeTypePatternPointingLeft(&mut self, _ctx: &FullEdgeTypePatternPointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeTypePatternPointingLeft}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeTypePatternPointingLeft(&mut self, _ctx: &FullEdgeTypePatternPointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fullEdgeTypePatternUndirected}.
 * @param ctx the parse tree
 */
fn enter_fullEdgeTypePatternUndirected(&mut self, _ctx: &FullEdgeTypePatternUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fullEdgeTypePatternUndirected}.
 * @param ctx the parse tree
 */
fn exit_fullEdgeTypePatternUndirected(&mut self, _ctx: &FullEdgeTypePatternUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#arcTypePointingRight}.
 * @param ctx the parse tree
 */
fn enter_arcTypePointingRight(&mut self, _ctx: &ArcTypePointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#arcTypePointingRight}.
 * @param ctx the parse tree
 */
fn exit_arcTypePointingRight(&mut self, _ctx: &ArcTypePointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
 * @param ctx the parse tree
 */
fn enter_arcTypePointingLeft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
 * @param ctx the parse tree
 */
fn exit_arcTypePointingLeft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#arcTypeUndirected}.
 * @param ctx the parse tree
 */
fn enter_arcTypeUndirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#arcTypeUndirected}.
 * @param ctx the parse tree
 */
fn exit_arcTypeUndirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#arcTypeFiller}.
 * @param ctx the parse tree
 */
fn enter_arcTypeFiller(&mut self, _ctx: &ArcTypeFillerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#arcTypeFiller}.
 * @param ctx the parse tree
 */
fn exit_arcTypeFiller(&mut self, _ctx: &ArcTypeFillerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePattern}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeTypePattern(&mut self, _ctx: &AbbreviatedEdgeTypePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePattern}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeTypePattern(&mut self, _ctx: &AbbreviatedEdgeTypePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternPointingRight}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeTypePatternPointingRight(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternPointingRight}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeTypePatternPointingRight(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternPointingLeft}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeTypePatternPointingLeft(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternPointingLeft}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeTypePatternPointingLeft(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternUndirected}.
 * @param ctx the parse tree
 */
fn enter_abbreviatedEdgeTypePatternUndirected(&mut self, _ctx: &AbbreviatedEdgeTypePatternUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#abbreviatedEdgeTypePatternUndirected}.
 * @param ctx the parse tree
 */
fn exit_abbreviatedEdgeTypePatternUndirected(&mut self, _ctx: &AbbreviatedEdgeTypePatternUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeTypeReference}.
 * @param ctx the parse tree
 */
fn enter_nodeTypeReference(&mut self, _ctx: &NodeTypeReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeTypeReference}.
 * @param ctx the parse tree
 */
fn exit_nodeTypeReference(&mut self, _ctx: &NodeTypeReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
 * @param ctx the parse tree
 */
fn enter_sourceNodeTypeReference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
 * @param ctx the parse tree
 */
fn exit_sourceNodeTypeReference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
 * @param ctx the parse tree
 */
fn enter_destinationNodeTypeReference(&mut self, _ctx: &DestinationNodeTypeReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
 * @param ctx the parse tree
 */
fn exit_destinationNodeTypeReference(&mut self, _ctx: &DestinationNodeTypeReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeKind}.
 * @param ctx the parse tree
 */
fn enter_edgeKind(&mut self, _ctx: &EdgeKindContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeKind}.
 * @param ctx the parse tree
 */
fn exit_edgeKind(&mut self, _ctx: &EdgeKindContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endpointDefinition}.
 * @param ctx the parse tree
 */
fn enter_endpointDefinition(&mut self, _ctx: &EndpointDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endpointDefinition}.
 * @param ctx the parse tree
 */
fn exit_endpointDefinition(&mut self, _ctx: &EndpointDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endpointPairDefinition}.
 * @param ctx the parse tree
 */
fn enter_endpointPairDefinition(&mut self, _ctx: &EndpointPairDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endpointPairDefinition}.
 * @param ctx the parse tree
 */
fn exit_endpointPairDefinition(&mut self, _ctx: &EndpointPairDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endpointPairDefinitionPointingRight}.
 * @param ctx the parse tree
 */
fn enter_endpointPairDefinitionPointingRight(&mut self, _ctx: &EndpointPairDefinitionPointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endpointPairDefinitionPointingRight}.
 * @param ctx the parse tree
 */
fn exit_endpointPairDefinitionPointingRight(&mut self, _ctx: &EndpointPairDefinitionPointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endpointPairDefinitionPointingLeft}.
 * @param ctx the parse tree
 */
fn enter_endpointPairDefinitionPointingLeft(&mut self, _ctx: &EndpointPairDefinitionPointingLeftContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endpointPairDefinitionPointingLeft}.
 * @param ctx the parse tree
 */
fn exit_endpointPairDefinitionPointingLeft(&mut self, _ctx: &EndpointPairDefinitionPointingLeftContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#endpointPairDefinitionUndirected}.
 * @param ctx the parse tree
 */
fn enter_endpointPairDefinitionUndirected(&mut self, _ctx: &EndpointPairDefinitionUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#endpointPairDefinitionUndirected}.
 * @param ctx the parse tree
 */
fn exit_endpointPairDefinitionUndirected(&mut self, _ctx: &EndpointPairDefinitionUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#connectorPointingRight}.
 * @param ctx the parse tree
 */
fn enter_connectorPointingRight(&mut self, _ctx: &ConnectorPointingRightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#connectorPointingRight}.
 * @param ctx the parse tree
 */
fn exit_connectorPointingRight(&mut self, _ctx: &ConnectorPointingRightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#connectorUndirected}.
 * @param ctx the parse tree
 */
fn enter_connectorUndirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#connectorUndirected}.
 * @param ctx the parse tree
 */
fn exit_connectorUndirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sourceNodeTypeName}.
 * @param ctx the parse tree
 */
fn enter_sourceNodeTypeName(&mut self, _ctx: &SourceNodeTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sourceNodeTypeName}.
 * @param ctx the parse tree
 */
fn exit_sourceNodeTypeName(&mut self, _ctx: &SourceNodeTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#destinationNodeTypeName}.
 * @param ctx the parse tree
 */
fn enter_destinationNodeTypeName(&mut self, _ctx: &DestinationNodeTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#destinationNodeTypeName}.
 * @param ctx the parse tree
 */
fn exit_destinationNodeTypeName(&mut self, _ctx: &DestinationNodeTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labelSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_labelSetDefinition(&mut self, _ctx: &LabelSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labelSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_labelSetDefinition(&mut self, _ctx: &LabelSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn enter_propertyTypeSetDefinition(&mut self, _ctx: &PropertyTypeSetDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyTypeSetDefinition}.
 * @param ctx the parse tree
 */
fn exit_propertyTypeSetDefinition(&mut self, _ctx: &PropertyTypeSetDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyTypeDefinitionList}.
 * @param ctx the parse tree
 */
fn enter_propertyTypeDefinitionList(&mut self, _ctx: &PropertyTypeDefinitionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyTypeDefinitionList}.
 * @param ctx the parse tree
 */
fn exit_propertyTypeDefinitionList(&mut self, _ctx: &PropertyTypeDefinitionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyTypeDefinition}.
 * @param ctx the parse tree
 */
fn enter_propertyTypeDefinition(&mut self, _ctx: &PropertyTypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyTypeDefinition}.
 * @param ctx the parse tree
 */
fn exit_propertyTypeDefinition(&mut self, _ctx: &PropertyTypeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyValueType}.
 * @param ctx the parse tree
 */
fn enter_propertyValueType(&mut self, _ctx: &PropertyValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyValueType}.
 * @param ctx the parse tree
 */
fn exit_propertyValueType(&mut self, _ctx: &PropertyValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableType}.
 * @param ctx the parse tree
 */
fn enter_bindingTableType(&mut self, _ctx: &BindingTableTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableType}.
 * @param ctx the parse tree
 */
fn exit_bindingTableType(&mut self, _ctx: &BindingTableTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dynamicPropertyValueType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_dynamicPropertyValueType(&mut self, _ctx: &DynamicPropertyValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dynamicPropertyValueType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_dynamicPropertyValueType(&mut self, _ctx: &DynamicPropertyValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code predefType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_predefType(&mut self, _ctx: &PredefTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code predefType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_predefType(&mut self, _ctx: &PredefTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code listType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_listType1(&mut self, _ctx: &ListType1Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code listType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_listType1(&mut self, _ctx: &ListType1Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code recordType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_recordType1(&mut self, _ctx: &RecordType1Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code recordType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_recordType1(&mut self, _ctx: &RecordType1Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code closedDynamicUnionType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_closedDynamicUnionType2(&mut self, _ctx: &ClosedDynamicUnionType2Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code closedDynamicUnionType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_closedDynamicUnionType2(&mut self, _ctx: &ClosedDynamicUnionType2Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code recordType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_recordType2(&mut self, _ctx: &RecordType2Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code recordType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_recordType2(&mut self, _ctx: &RecordType2Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code listType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_listType2(&mut self, _ctx: &ListType2Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code listType2}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_listType2(&mut self, _ctx: &ListType2Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code pathType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_pathType(&mut self, _ctx: &PathTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code pathType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_pathType(&mut self, _ctx: &PathTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code openDynamicUnionType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_openDynamicUnionType(&mut self, _ctx: &OpenDynamicUnionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code openDynamicUnionType}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_openDynamicUnionType(&mut self, _ctx: &OpenDynamicUnionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code closedDynamicUnionType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn enter_closedDynamicUnionType1(&mut self, _ctx: &ClosedDynamicUnionType1Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code closedDynamicUnionType1}
 * labeled alternative in {@link GQLParser#valueType}.
 * @param ctx the parse tree
 */
fn exit_closedDynamicUnionType1(&mut self, _ctx: &ClosedDynamicUnionType1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#typed}.
 * @param ctx the parse tree
 */
fn enter_typed(&mut self, _ctx: &TypedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#typed}.
 * @param ctx the parse tree
 */
fn exit_typed(&mut self, _ctx: &TypedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#predefinedType}.
 * @param ctx the parse tree
 */
fn enter_predefinedType(&mut self, _ctx: &PredefinedTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#predefinedType}.
 * @param ctx the parse tree
 */
fn exit_predefinedType(&mut self, _ctx: &PredefinedTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#booleanType}.
 * @param ctx the parse tree
 */
fn enter_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#booleanType}.
 * @param ctx the parse tree
 */
fn exit_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#characterStringType}.
 * @param ctx the parse tree
 */
fn enter_characterStringType(&mut self, _ctx: &CharacterStringTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#characterStringType}.
 * @param ctx the parse tree
 */
fn exit_characterStringType(&mut self, _ctx: &CharacterStringTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#byteStringType}.
 * @param ctx the parse tree
 */
fn enter_byteStringType(&mut self, _ctx: &ByteStringTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#byteStringType}.
 * @param ctx the parse tree
 */
fn exit_byteStringType(&mut self, _ctx: &ByteStringTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#minLength}.
 * @param ctx the parse tree
 */
fn enter_minLength(&mut self, _ctx: &MinLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#minLength}.
 * @param ctx the parse tree
 */
fn exit_minLength(&mut self, _ctx: &MinLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#maxLength}.
 * @param ctx the parse tree
 */
fn enter_maxLength(&mut self, _ctx: &MaxLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#maxLength}.
 * @param ctx the parse tree
 */
fn exit_maxLength(&mut self, _ctx: &MaxLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fixedLength}.
 * @param ctx the parse tree
 */
fn enter_fixedLength(&mut self, _ctx: &FixedLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fixedLength}.
 * @param ctx the parse tree
 */
fn exit_fixedLength(&mut self, _ctx: &FixedLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#numericType}.
 * @param ctx the parse tree
 */
fn enter_numericType(&mut self, _ctx: &NumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#numericType}.
 * @param ctx the parse tree
 */
fn exit_numericType(&mut self, _ctx: &NumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#exactNumericType}.
 * @param ctx the parse tree
 */
fn enter_exactNumericType(&mut self, _ctx: &ExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#exactNumericType}.
 * @param ctx the parse tree
 */
fn exit_exactNumericType(&mut self, _ctx: &ExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#binaryExactNumericType}.
 * @param ctx the parse tree
 */
fn enter_binaryExactNumericType(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#binaryExactNumericType}.
 * @param ctx the parse tree
 */
fn exit_binaryExactNumericType(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn enter_signedBinaryExactNumericType(&mut self, _ctx: &SignedBinaryExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn exit_signedBinaryExactNumericType(&mut self, _ctx: &SignedBinaryExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn enter_unsignedBinaryExactNumericType(&mut self, _ctx: &UnsignedBinaryExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn exit_unsignedBinaryExactNumericType(&mut self, _ctx: &UnsignedBinaryExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn enter_verboseBinaryExactNumericType(&mut self, _ctx: &VerboseBinaryExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
 * @param ctx the parse tree
 */
fn exit_verboseBinaryExactNumericType(&mut self, _ctx: &VerboseBinaryExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#decimalExactNumericType}.
 * @param ctx the parse tree
 */
fn enter_decimalExactNumericType(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#decimalExactNumericType}.
 * @param ctx the parse tree
 */
fn exit_decimalExactNumericType(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#precision}.
 * @param ctx the parse tree
 */
fn enter_precision(&mut self, _ctx: &PrecisionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#precision}.
 * @param ctx the parse tree
 */
fn exit_precision(&mut self, _ctx: &PrecisionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#scale}.
 * @param ctx the parse tree
 */
fn enter_scale(&mut self, _ctx: &ScaleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#scale}.
 * @param ctx the parse tree
 */
fn exit_scale(&mut self, _ctx: &ScaleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#approximateNumericType}.
 * @param ctx the parse tree
 */
fn enter_approximateNumericType(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#approximateNumericType}.
 * @param ctx the parse tree
 */
fn exit_approximateNumericType(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#temporalType}.
 * @param ctx the parse tree
 */
fn enter_temporalType(&mut self, _ctx: &TemporalTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#temporalType}.
 * @param ctx the parse tree
 */
fn exit_temporalType(&mut self, _ctx: &TemporalTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#temporalInstantType}.
 * @param ctx the parse tree
 */
fn enter_temporalInstantType(&mut self, _ctx: &TemporalInstantTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#temporalInstantType}.
 * @param ctx the parse tree
 */
fn exit_temporalInstantType(&mut self, _ctx: &TemporalInstantTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#temporalDurationType}.
 * @param ctx the parse tree
 */
fn enter_temporalDurationType(&mut self, _ctx: &TemporalDurationTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#temporalDurationType}.
 * @param ctx the parse tree
 */
fn exit_temporalDurationType(&mut self, _ctx: &TemporalDurationTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeType}.
 * @param ctx the parse tree
 */
fn enter_datetimeType(&mut self, _ctx: &DatetimeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeType}.
 * @param ctx the parse tree
 */
fn exit_datetimeType(&mut self, _ctx: &DatetimeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#localdatetimeType}.
 * @param ctx the parse tree
 */
fn enter_localdatetimeType(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#localdatetimeType}.
 * @param ctx the parse tree
 */
fn exit_localdatetimeType(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dateType}.
 * @param ctx the parse tree
 */
fn enter_dateType(&mut self, _ctx: &DateTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dateType}.
 * @param ctx the parse tree
 */
fn exit_dateType(&mut self, _ctx: &DateTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#timeType}.
 * @param ctx the parse tree
 */
fn enter_timeType(&mut self, _ctx: &TimeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#timeType}.
 * @param ctx the parse tree
 */
fn exit_timeType(&mut self, _ctx: &TimeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#localtimeType}.
 * @param ctx the parse tree
 */
fn enter_localtimeType(&mut self, _ctx: &LocaltimeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#localtimeType}.
 * @param ctx the parse tree
 */
fn exit_localtimeType(&mut self, _ctx: &LocaltimeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#durationType}.
 * @param ctx the parse tree
 */
fn enter_durationType(&mut self, _ctx: &DurationTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#durationType}.
 * @param ctx the parse tree
 */
fn exit_durationType(&mut self, _ctx: &DurationTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#referenceValueType}.
 * @param ctx the parse tree
 */
fn enter_referenceValueType(&mut self, _ctx: &ReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#referenceValueType}.
 * @param ctx the parse tree
 */
fn exit_referenceValueType(&mut self, _ctx: &ReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_graphReferenceValueType(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_graphReferenceValueType(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_closedGraphReferenceValueType(&mut self, _ctx: &ClosedGraphReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_closedGraphReferenceValueType(&mut self, _ctx: &ClosedGraphReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_openGraphReferenceValueType(&mut self, _ctx: &OpenGraphReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_openGraphReferenceValueType(&mut self, _ctx: &OpenGraphReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_bindingTableReferenceValueType(&mut self, _ctx: &BindingTableReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_bindingTableReferenceValueType(&mut self, _ctx: &BindingTableReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_nodeReferenceValueType(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_nodeReferenceValueType(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_closedNodeReferenceValueType(&mut self, _ctx: &ClosedNodeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_closedNodeReferenceValueType(&mut self, _ctx: &ClosedNodeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_openNodeReferenceValueType(&mut self, _ctx: &OpenNodeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_openNodeReferenceValueType(&mut self, _ctx: &OpenNodeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_edgeReferenceValueType(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_edgeReferenceValueType(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_closedEdgeReferenceValueType(&mut self, _ctx: &ClosedEdgeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_closedEdgeReferenceValueType(&mut self, _ctx: &ClosedEdgeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn enter_openEdgeReferenceValueType(&mut self, _ctx: &OpenEdgeReferenceValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
 * @param ctx the parse tree
 */
fn exit_openEdgeReferenceValueType(&mut self, _ctx: &OpenEdgeReferenceValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#listValueTypeName}.
 * @param ctx the parse tree
 */
fn enter_listValueTypeName(&mut self, _ctx: &ListValueTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#listValueTypeName}.
 * @param ctx the parse tree
 */
fn exit_listValueTypeName(&mut self, _ctx: &ListValueTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
 * @param ctx the parse tree
 */
fn enter_listValueTypeNameSynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
 * @param ctx the parse tree
 */
fn exit_listValueTypeNameSynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
 * @param ctx the parse tree
 */
fn enter_fieldTypesSpecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
 * @param ctx the parse tree
 */
fn exit_fieldTypesSpecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fieldTypeList}.
 * @param ctx the parse tree
 */
fn enter_fieldTypeList(&mut self, _ctx: &FieldTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fieldTypeList}.
 * @param ctx the parse tree
 */
fn exit_fieldTypeList(&mut self, _ctx: &FieldTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathValueType}.
 * @param ctx the parse tree
 */
fn enter_pathValueType(&mut self, _ctx: &PathValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathValueType}.
 * @param ctx the parse tree
 */
fn exit_pathValueType(&mut self, _ctx: &PathValueTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#notNull}.
 * @param ctx the parse tree
 */
fn enter_notNull(&mut self, _ctx: &NotNullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#notNull}.
 * @param ctx the parse tree
 */
fn exit_notNull(&mut self, _ctx: &NotNullContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#fieldType}.
 * @param ctx the parse tree
 */
fn enter_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#fieldType}.
 * @param ctx the parse tree
 */
fn exit_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#schemaReference}.
 * @param ctx the parse tree
 */
fn enter_schemaReference(&mut self, _ctx: &SchemaReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#schemaReference}.
 * @param ctx the parse tree
 */
fn exit_schemaReference(&mut self, _ctx: &SchemaReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
 * @param ctx the parse tree
 */
fn enter_absoluteCatalogSchemaReference(&mut self, _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
 * @param ctx the parse tree
 */
fn exit_absoluteCatalogSchemaReference(&mut self, _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
 * @param ctx the parse tree
 */
fn enter_catalogSchemaParentAndName(&mut self, _ctx: &CatalogSchemaParentAndNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
 * @param ctx the parse tree
 */
fn exit_catalogSchemaParentAndName(&mut self, _ctx: &CatalogSchemaParentAndNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
 * @param ctx the parse tree
 */
fn enter_relativeCatalogSchemaReference(&mut self, _ctx: &RelativeCatalogSchemaReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
 * @param ctx the parse tree
 */
fn exit_relativeCatalogSchemaReference(&mut self, _ctx: &RelativeCatalogSchemaReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
 * @param ctx the parse tree
 */
fn enter_predefinedSchemaReference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
 * @param ctx the parse tree
 */
fn exit_predefinedSchemaReference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
 * @param ctx the parse tree
 */
fn enter_absoluteDirectoryPath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
 * @param ctx the parse tree
 */
fn exit_absoluteDirectoryPath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
 * @param ctx the parse tree
 */
fn enter_relativeDirectoryPath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
 * @param ctx the parse tree
 */
fn exit_relativeDirectoryPath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
 * @param ctx the parse tree
 */
fn enter_simpleDirectoryPath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
 * @param ctx the parse tree
 */
fn exit_simpleDirectoryPath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphReference}.
 * @param ctx the parse tree
 */
fn enter_graphReference(&mut self, _ctx: &GraphReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphReference}.
 * @param ctx the parse tree
 */
fn exit_graphReference(&mut self, _ctx: &GraphReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
 * @param ctx the parse tree
 */
fn enter_catalogGraphParentAndName(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
 * @param ctx the parse tree
 */
fn exit_catalogGraphParentAndName(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#homeGraph}.
 * @param ctx the parse tree
 */
fn enter_homeGraph(&mut self, _ctx: &HomeGraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#homeGraph}.
 * @param ctx the parse tree
 */
fn exit_homeGraph(&mut self, _ctx: &HomeGraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#graphTypeReference}.
 * @param ctx the parse tree
 */
fn enter_graphTypeReference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#graphTypeReference}.
 * @param ctx the parse tree
 */
fn exit_graphTypeReference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
 * @param ctx the parse tree
 */
fn enter_catalogGraphTypeParentAndName(&mut self, _ctx: &CatalogGraphTypeParentAndNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
 * @param ctx the parse tree
 */
fn exit_catalogGraphTypeParentAndName(&mut self, _ctx: &CatalogGraphTypeParentAndNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#bindingTableReference}.
 * @param ctx the parse tree
 */
fn enter_bindingTableReference(&mut self, _ctx: &BindingTableReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#bindingTableReference}.
 * @param ctx the parse tree
 */
fn exit_bindingTableReference(&mut self, _ctx: &BindingTableReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogBindingTableParentAndName}.
 * @param ctx the parse tree
 */
fn enter_catalogBindingTableParentAndName(&mut self, _ctx: &CatalogBindingTableParentAndNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogBindingTableParentAndName}.
 * @param ctx the parse tree
 */
fn exit_catalogBindingTableParentAndName(&mut self, _ctx: &CatalogBindingTableParentAndNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#procedureReference}.
 * @param ctx the parse tree
 */
fn enter_procedureReference(&mut self, _ctx: &ProcedureReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#procedureReference}.
 * @param ctx the parse tree
 */
fn exit_procedureReference(&mut self, _ctx: &ProcedureReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
 * @param ctx the parse tree
 */
fn enter_catalogProcedureParentAndName(&mut self, _ctx: &CatalogProcedureParentAndNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
 * @param ctx the parse tree
 */
fn exit_catalogProcedureParentAndName(&mut self, _ctx: &CatalogProcedureParentAndNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
 * @param ctx the parse tree
 */
fn enter_catalogObjectParentReference(&mut self, _ctx: &CatalogObjectParentReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
 * @param ctx the parse tree
 */
fn exit_catalogObjectParentReference(&mut self, _ctx: &CatalogObjectParentReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#referenceParameter}.
 * @param ctx the parse tree
 */
fn enter_referenceParameter(&mut self, _ctx: &ReferenceParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#referenceParameter}.
 * @param ctx the parse tree
 */
fn exit_referenceParameter(&mut self, _ctx: &ReferenceParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#externalObjectReference}.
 * @param ctx the parse tree
 */
fn enter_externalObjectReference(&mut self, _ctx: &ExternalObjectReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#externalObjectReference}.
 * @param ctx the parse tree
 */
fn exit_externalObjectReference(&mut self, _ctx: &ExternalObjectReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#comparisonPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_comparisonPredicateCond(&mut self, _ctx: &ComparisonPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#comparisonPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_comparisonPredicateCond(&mut self, _ctx: &ComparisonPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#compOp}.
 * @param ctx the parse tree
 */
fn enter_compOp(&mut self, _ctx: &CompOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#compOp}.
 * @param ctx the parse tree
 */
fn exit_compOp(&mut self, _ctx: &CompOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#nullPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_nullPredicateCond(&mut self, _ctx: &NullPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#nullPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_nullPredicateCond(&mut self, _ctx: &NullPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#normalizedPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_normalizedPredicateCond(&mut self, _ctx: &NormalizedPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#normalizedPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_normalizedPredicateCond(&mut self, _ctx: &NormalizedPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#directedPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_directedPredicateCond(&mut self, _ctx: &DirectedPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#directedPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_directedPredicateCond(&mut self, _ctx: &DirectedPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#labeledPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_labeledPredicateCond(&mut self, _ctx: &LabeledPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#labeledPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_labeledPredicateCond(&mut self, _ctx: &LabeledPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#sourceDestinationPredicateCond}.
 * @param ctx the parse tree
 */
fn enter_sourceDestinationPredicateCond(&mut self, _ctx: &SourceDestinationPredicateCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#sourceDestinationPredicateCond}.
 * @param ctx the parse tree
 */
fn exit_sourceDestinationPredicateCond(&mut self, _ctx: &SourceDestinationPredicateCondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
 * @param ctx the parse tree
 */
fn enter_unsignedValueSpecification(&mut self, _ctx: &UnsignedValueSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
 * @param ctx the parse tree
 */
fn exit_unsignedValueSpecification(&mut self, _ctx: &UnsignedValueSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unsignedIntegerSpecification}.
 * @param ctx the parse tree
 */
fn enter_unsignedIntegerSpecification(&mut self, _ctx: &UnsignedIntegerSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unsignedIntegerSpecification}.
 * @param ctx the parse tree
 */
fn exit_unsignedIntegerSpecification(&mut self, _ctx: &UnsignedIntegerSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#parameterValueSpecification}.
 * @param ctx the parse tree
 */
fn enter_parameterValueSpecification(&mut self, _ctx: &ParameterValueSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#parameterValueSpecification}.
 * @param ctx the parse tree
 */
fn exit_parameterValueSpecification(&mut self, _ctx: &ParameterValueSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#predefinedParameter}.
 * @param ctx the parse tree
 */
fn enter_predefinedParameter(&mut self, _ctx: &PredefinedParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#predefinedParameter}.
 * @param ctx the parse tree
 */
fn exit_predefinedParameter(&mut self, _ctx: &PredefinedParameterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlPredicateExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_gqlPredicateExpression(&mut self, _ctx: &GqlPredicateExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlPredicateExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_gqlPredicateExpression(&mut self, _ctx: &GqlPredicateExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlNotExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_gqlNotExpression(&mut self, _ctx: &GqlNotExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlNotExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_gqlNotExpression(&mut self, _ctx: &GqlNotExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLogicalXorExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_gqlLogicalXorExpression(&mut self, _ctx: &GqlLogicalXorExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLogicalXorExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_gqlLogicalXorExpression(&mut self, _ctx: &GqlLogicalXorExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLogicalOrExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_gqlLogicalOrExpression(&mut self, _ctx: &GqlLogicalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLogicalOrExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_gqlLogicalOrExpression(&mut self, _ctx: &GqlLogicalOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLogicalAndExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_gqlLogicalAndExpression(&mut self, _ctx: &GqlLogicalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLogicalAndExpression}
 * labeled alternative in {@link GQLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_gqlLogicalAndExpression(&mut self, _ctx: &GqlLogicalAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlAtomExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlAtomExpression(&mut self, _ctx: &GqlAtomExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlAtomExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlAtomExpression(&mut self, _ctx: &GqlAtomExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlNormalizedExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlNormalizedExpression(&mut self, _ctx: &GqlNormalizedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlNormalizedExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlNormalizedExpression(&mut self, _ctx: &GqlNormalizedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSameExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlSameExpression(&mut self, _ctx: &GqlSameExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSameExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlSameExpression(&mut self, _ctx: &GqlSameExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlDirectedExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlDirectedExpression(&mut self, _ctx: &GqlDirectedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlDirectedExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlDirectedExpression(&mut self, _ctx: &GqlDirectedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlBooleanTestExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlBooleanTestExpression(&mut self, _ctx: &GqlBooleanTestExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlBooleanTestExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlBooleanTestExpression(&mut self, _ctx: &GqlBooleanTestExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlGraphRefValueExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlGraphRefValueExpression(&mut self, _ctx: &GqlGraphRefValueExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlGraphRefValueExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlGraphRefValueExpression(&mut self, _ctx: &GqlGraphRefValueExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlAllDifferentExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlAllDifferentExpression(&mut self, _ctx: &GqlAllDifferentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlAllDifferentExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlAllDifferentExpression(&mut self, _ctx: &GqlAllDifferentExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlExistsExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlExistsExpression(&mut self, _ctx: &GqlExistsExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlExistsExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlExistsExpression(&mut self, _ctx: &GqlExistsExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLetExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlLetExpression(&mut self, _ctx: &GqlLetExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLetExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlLetExpression(&mut self, _ctx: &GqlLetExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlComparisonExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlComparisonExpression(&mut self, _ctx: &GqlComparisonExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlComparisonExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlComparisonExpression(&mut self, _ctx: &GqlComparisonExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLabeledExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlLabeledExpression(&mut self, _ctx: &GqlLabeledExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLabeledExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlLabeledExpression(&mut self, _ctx: &GqlLabeledExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlPropertyExistExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlPropertyExistExpression(&mut self, _ctx: &GqlPropertyExistExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlPropertyExistExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlPropertyExistExpression(&mut self, _ctx: &GqlPropertyExistExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlNullExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlNullExpression(&mut self, _ctx: &GqlNullExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlNullExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlNullExpression(&mut self, _ctx: &GqlNullExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSourceDestinationExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlSourceDestinationExpression(&mut self, _ctx: &GqlSourceDestinationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSourceDestinationExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlSourceDestinationExpression(&mut self, _ctx: &GqlSourceDestinationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlBindingTableValueExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn enter_gqlBindingTableValueExpression(&mut self, _ctx: &GqlBindingTableValueExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlBindingTableValueExpression}
 * labeled alternative in {@link GQLParser#expressionPredicate}.
 * @param ctx the parse tree
 */
fn exit_gqlBindingTableValueExpression(&mut self, _ctx: &GqlBindingTableValueExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlParameterExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlParameterExpression(&mut self, _ctx: &GqlParameterExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlParameterExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlParameterExpression(&mut self, _ctx: &GqlParameterExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlUnaryExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlUnaryExpression(&mut self, _ctx: &GqlUnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlUnaryExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlUnaryExpression(&mut self, _ctx: &GqlUnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlPropertyReference}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlPropertyReference(&mut self, _ctx: &GqlPropertyReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlPropertyReference}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlPropertyReference(&mut self, _ctx: &GqlPropertyReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlHighArithmeticExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlHighArithmeticExpression(&mut self, _ctx: &GqlHighArithmeticExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlHighArithmeticExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlHighArithmeticExpression(&mut self, _ctx: &GqlHighArithmeticExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlValueQueryExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlValueQueryExpression(&mut self, _ctx: &GqlValueQueryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlValueQueryExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlValueQueryExpression(&mut self, _ctx: &GqlValueQueryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLiteralExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlLiteralExpression(&mut self, _ctx: &GqlLiteralExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLiteralExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlLiteralExpression(&mut self, _ctx: &GqlLiteralExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlCollectionExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlCollectionExpression(&mut self, _ctx: &GqlCollectionExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlCollectionExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlCollectionExpression(&mut self, _ctx: &GqlCollectionExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlConcatenationExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlConcatenationExpression(&mut self, _ctx: &GqlConcatenationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlConcatenationExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlConcatenationExpression(&mut self, _ctx: &GqlConcatenationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlLowArithmeticExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlLowArithmeticExpression(&mut self, _ctx: &GqlLowArithmeticExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlLowArithmeticExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlLowArithmeticExpression(&mut self, _ctx: &GqlLowArithmeticExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlParenthesizedExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlParenthesizedExpression(&mut self, _ctx: &GqlParenthesizedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlParenthesizedExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlParenthesizedExpression(&mut self, _ctx: &GqlParenthesizedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlFunctionExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlFunctionExpression(&mut self, _ctx: &GqlFunctionExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlFunctionExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlFunctionExpression(&mut self, _ctx: &GqlFunctionExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlVariableExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn enter_gqlVariableExpression(&mut self, _ctx: &GqlVariableExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlVariableExpression}
 * labeled alternative in {@link GQLParser#expressionAtom}.
 * @param ctx the parse tree
 */
fn exit_gqlVariableExpression(&mut self, _ctx: &GqlVariableExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#truthValue}.
 * @param ctx the parse tree
 */
fn enter_truthValue(&mut self, _ctx: &TruthValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#truthValue}.
 * @param ctx the parse tree
 */
fn exit_truthValue(&mut self, _ctx: &TruthValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unaryOperator}.
 * @param ctx the parse tree
 */
fn enter_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unaryOperator}.
 * @param ctx the parse tree
 */
fn exit_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlOneArgScalarFunction}
 * labeled alternative in {@link GQLParser#numericFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlOneArgScalarFunction(&mut self, _ctx: &GqlOneArgScalarFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlOneArgScalarFunction}
 * labeled alternative in {@link GQLParser#numericFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlOneArgScalarFunction(&mut self, _ctx: &GqlOneArgScalarFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlTwoArgScalarFunction}
 * labeled alternative in {@link GQLParser#numericFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlTwoArgScalarFunction(&mut self, _ctx: &GqlTwoArgScalarFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlTwoArgScalarFunction}
 * labeled alternative in {@link GQLParser#numericFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlTwoArgScalarFunction(&mut self, _ctx: &GqlTwoArgScalarFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#functionParameter}.
 * @param ctx the parse tree
 */
fn enter_functionParameter(&mut self, _ctx: &FunctionParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#functionParameter}.
 * @param ctx the parse tree
 */
fn exit_functionParameter(&mut self, _ctx: &FunctionParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#propertyReference}.
 * @param ctx the parse tree
 */
fn enter_propertyReference(&mut self, _ctx: &PropertyReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#propertyReference}.
 * @param ctx the parse tree
 */
fn exit_propertyReference(&mut self, _ctx: &PropertyReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#oneArgNumericFunctionName}.
 * @param ctx the parse tree
 */
fn enter_oneArgNumericFunctionName(&mut self, _ctx: &OneArgNumericFunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#oneArgNumericFunctionName}.
 * @param ctx the parse tree
 */
fn exit_oneArgNumericFunctionName(&mut self, _ctx: &OneArgNumericFunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#twoArgNumericFunctionName}.
 * @param ctx the parse tree
 */
fn enter_twoArgNumericFunctionName(&mut self, _ctx: &TwoArgNumericFunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#twoArgNumericFunctionName}.
 * @param ctx the parse tree
 */
fn exit_twoArgNumericFunctionName(&mut self, _ctx: &TwoArgNumericFunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSubstringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlSubstringFunction(&mut self, _ctx: &GqlSubstringFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSubstringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlSubstringFunction(&mut self, _ctx: &GqlSubstringFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlFoldStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlFoldStringFunction(&mut self, _ctx: &GqlFoldStringFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlFoldStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlFoldStringFunction(&mut self, _ctx: &GqlFoldStringFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSingleTrimStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlSingleTrimStringFunction(&mut self, _ctx: &GqlSingleTrimStringFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSingleTrimStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlSingleTrimStringFunction(&mut self, _ctx: &GqlSingleTrimStringFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlMultiTrimStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlMultiTrimStringFunction(&mut self, _ctx: &GqlMultiTrimStringFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlMultiTrimStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlMultiTrimStringFunction(&mut self, _ctx: &GqlMultiTrimStringFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlNormStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlNormStringFunction(&mut self, _ctx: &GqlNormStringFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlNormStringFunction}
 * labeled alternative in {@link GQLParser#stringFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlNormStringFunction(&mut self, _ctx: &GqlNormStringFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlListTrimFunction}
 * labeled alternative in {@link GQLParser#listFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlListTrimFunction(&mut self, _ctx: &GqlListTrimFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlListTrimFunction}
 * labeled alternative in {@link GQLParser#listFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlListTrimFunction(&mut self, _ctx: &GqlListTrimFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlElementsOfPathFunction}
 * labeled alternative in {@link GQLParser#listFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlElementsOfPathFunction(&mut self, _ctx: &GqlElementsOfPathFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlElementsOfPathFunction}
 * labeled alternative in {@link GQLParser#listFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlElementsOfPathFunction(&mut self, _ctx: &GqlElementsOfPathFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlNullIfCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlNullIfCaseFunction(&mut self, _ctx: &GqlNullIfCaseFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlNullIfCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlNullIfCaseFunction(&mut self, _ctx: &GqlNullIfCaseFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlCoalesceCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlCoalesceCaseFunction(&mut self, _ctx: &GqlCoalesceCaseFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlCoalesceCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlCoalesceCaseFunction(&mut self, _ctx: &GqlCoalesceCaseFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSimpleCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlSimpleCaseFunction(&mut self, _ctx: &GqlSimpleCaseFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSimpleCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlSimpleCaseFunction(&mut self, _ctx: &GqlSimpleCaseFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlSearchedCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlSearchedCaseFunction(&mut self, _ctx: &GqlSearchedCaseFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlSearchedCaseFunction}
 * labeled alternative in {@link GQLParser#caseFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlSearchedCaseFunction(&mut self, _ctx: &GqlSearchedCaseFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#simpleWhenClause}.
 * @param ctx the parse tree
 */
fn enter_simpleWhenClause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#simpleWhenClause}.
 * @param ctx the parse tree
 */
fn exit_simpleWhenClause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#searchedWhenClause}.
 * @param ctx the parse tree
 */
fn enter_searchedWhenClause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#searchedWhenClause}.
 * @param ctx the parse tree
 */
fn exit_searchedWhenClause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elseClause}.
 * @param ctx the parse tree
 */
fn enter_elseClause(&mut self, _ctx: &ElseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elseClause}.
 * @param ctx the parse tree
 */
fn exit_elseClause(&mut self, _ctx: &ElseClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#whenOperand}.
 * @param ctx the parse tree
 */
fn enter_whenOperand(&mut self, _ctx: &WhenOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#whenOperand}.
 * @param ctx the parse tree
 */
fn exit_whenOperand(&mut self, _ctx: &WhenOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#castFunction}.
 * @param ctx the parse tree
 */
fn enter_castFunction(&mut self, _ctx: &CastFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#castFunction}.
 * @param ctx the parse tree
 */
fn exit_castFunction(&mut self, _ctx: &CastFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#elementIdFunction}.
 * @param ctx the parse tree
 */
fn enter_elementIdFunction(&mut self, _ctx: &ElementIdFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#elementIdFunction}.
 * @param ctx the parse tree
 */
fn exit_elementIdFunction(&mut self, _ctx: &ElementIdFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeValueFunction}.
 * @param ctx the parse tree
 */
fn enter_datetimeValueFunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeValueFunction}.
 * @param ctx the parse tree
 */
fn exit_datetimeValueFunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dateFunction}.
 * @param ctx the parse tree
 */
fn enter_dateFunction(&mut self, _ctx: &DateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dateFunction}.
 * @param ctx the parse tree
 */
fn exit_dateFunction(&mut self, _ctx: &DateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#timeFunction}.
 * @param ctx the parse tree
 */
fn enter_timeFunction(&mut self, _ctx: &TimeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#timeFunction}.
 * @param ctx the parse tree
 */
fn exit_timeFunction(&mut self, _ctx: &TimeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#localTimeFunction}.
 * @param ctx the parse tree
 */
fn enter_localTimeFunction(&mut self, _ctx: &LocalTimeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#localTimeFunction}.
 * @param ctx the parse tree
 */
fn exit_localTimeFunction(&mut self, _ctx: &LocalTimeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeFunction}.
 * @param ctx the parse tree
 */
fn enter_datetimeFunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeFunction}.
 * @param ctx the parse tree
 */
fn exit_datetimeFunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#localDatetimeFunction}.
 * @param ctx the parse tree
 */
fn enter_localDatetimeFunction(&mut self, _ctx: &LocalDatetimeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#localDatetimeFunction}.
 * @param ctx the parse tree
 */
fn exit_localDatetimeFunction(&mut self, _ctx: &LocalDatetimeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dateFunctionParameters}.
 * @param ctx the parse tree
 */
fn enter_dateFunctionParameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dateFunctionParameters}.
 * @param ctx the parse tree
 */
fn exit_dateFunctionParameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#timeFunctionParameters}.
 * @param ctx the parse tree
 */
fn enter_timeFunctionParameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#timeFunctionParameters}.
 * @param ctx the parse tree
 */
fn exit_timeFunctionParameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
 * @param ctx the parse tree
 */
fn enter_datetimeFunctionParameters(&mut self, _ctx: &DatetimeFunctionParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
 * @param ctx the parse tree
 */
fn exit_datetimeFunctionParameters(&mut self, _ctx: &DatetimeFunctionParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#dateString}.
 * @param ctx the parse tree
 */
fn enter_dateString(&mut self, _ctx: &DateStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#dateString}.
 * @param ctx the parse tree
 */
fn exit_dateString(&mut self, _ctx: &DateStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#timeString}.
 * @param ctx the parse tree
 */
fn enter_timeString(&mut self, _ctx: &TimeStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#timeString}.
 * @param ctx the parse tree
 */
fn exit_timeString(&mut self, _ctx: &TimeStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#datetimeString}.
 * @param ctx the parse tree
 */
fn enter_datetimeString(&mut self, _ctx: &DatetimeStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#datetimeString}.
 * @param ctx the parse tree
 */
fn exit_datetimeString(&mut self, _ctx: &DatetimeStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlDatetimeSubtractionFunction}
 * labeled alternative in {@link GQLParser#durationFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlDatetimeSubtractionFunction(&mut self, _ctx: &GqlDatetimeSubtractionFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlDatetimeSubtractionFunction}
 * labeled alternative in {@link GQLParser#durationFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlDatetimeSubtractionFunction(&mut self, _ctx: &GqlDatetimeSubtractionFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gqlDurationFunction}
 * labeled alternative in {@link GQLParser#durationFunction}.
 * @param ctx the parse tree
 */
fn enter_gqlDurationFunction(&mut self, _ctx: &GqlDurationFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gqlDurationFunction}
 * labeled alternative in {@link GQLParser#durationFunction}.
 * @param ctx the parse tree
 */
fn exit_gqlDurationFunction(&mut self, _ctx: &GqlDurationFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#durationFunctionParameters}.
 * @param ctx the parse tree
 */
fn enter_durationFunctionParameters(&mut self, _ctx: &DurationFunctionParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#durationFunctionParameters}.
 * @param ctx the parse tree
 */
fn exit_durationFunctionParameters(&mut self, _ctx: &DurationFunctionParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#durationString}.
 * @param ctx the parse tree
 */
fn enter_durationString(&mut self, _ctx: &DurationStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#durationString}.
 * @param ctx the parse tree
 */
fn exit_durationString(&mut self, _ctx: &DurationStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#generalFunction}.
 * @param ctx the parse tree
 */
fn enter_generalFunction(&mut self, _ctx: &GeneralFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#generalFunction}.
 * @param ctx the parse tree
 */
fn exit_generalFunction(&mut self, _ctx: &GeneralFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#collectionValueConstructor}.
 * @param ctx the parse tree
 */
fn enter_collectionValueConstructor(&mut self, _ctx: &CollectionValueConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#collectionValueConstructor}.
 * @param ctx the parse tree
 */
fn exit_collectionValueConstructor(&mut self, _ctx: &CollectionValueConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#trimSpecification}.
 * @param ctx the parse tree
 */
fn enter_trimSpecification(&mut self, _ctx: &TrimSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#trimSpecification}.
 * @param ctx the parse tree
 */
fn exit_trimSpecification(&mut self, _ctx: &TrimSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#normalForm}.
 * @param ctx the parse tree
 */
fn enter_normalForm(&mut self, _ctx: &NormalFormContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#normalForm}.
 * @param ctx the parse tree
 */
fn exit_normalForm(&mut self, _ctx: &NormalFormContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#listValueConstructor}.
 * @param ctx the parse tree
 */
fn enter_listValueConstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#listValueConstructor}.
 * @param ctx the parse tree
 */
fn exit_listValueConstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#recordValueConstructor}.
 * @param ctx the parse tree
 */
fn enter_recordValueConstructor(&mut self, _ctx: &RecordValueConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#recordValueConstructor}.
 * @param ctx the parse tree
 */
fn exit_recordValueConstructor(&mut self, _ctx: &RecordValueConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#field}.
 * @param ctx the parse tree
 */
fn enter_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#field}.
 * @param ctx the parse tree
 */
fn exit_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#pathValueConstructor}.
 * @param ctx the parse tree
 */
fn enter_pathValueConstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#pathValueConstructor}.
 * @param ctx the parse tree
 */
fn exit_pathValueConstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#unsignedLiteral}.
 * @param ctx the parse tree
 */
fn enter_unsignedLiteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#unsignedLiteral}.
 * @param ctx the parse tree
 */
fn exit_unsignedLiteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#generalLiteral}.
 * @param ctx the parse tree
 */
fn enter_generalLiteral(&mut self, _ctx: &GeneralLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#generalLiteral}.
 * @param ctx the parse tree
 */
fn exit_generalLiteral(&mut self, _ctx: &GeneralLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn enter_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn exit_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#recordLiteral}.
 * @param ctx the parse tree
 */
fn enter_recordLiteral(&mut self, _ctx: &RecordLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#recordLiteral}.
 * @param ctx the parse tree
 */
fn exit_recordLiteral(&mut self, _ctx: &RecordLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link GQLParser#recordFieldLiteral}.
 * @param ctx the parse tree
 */
fn enter_recordFieldLiteral(&mut self, _ctx: &RecordFieldLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GQLParser#recordFieldLiteral}.
 * @param ctx the parse tree
 */
fn exit_recordFieldLiteral(&mut self, _ctx: &RecordFieldLiteralContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : GQLListener<'input> }


