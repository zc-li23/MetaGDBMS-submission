// Generated from data/language/GQL.g4 by ANTLR 4.13.2

use super::gqlparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by GQLParser.

pub trait GQLBaseListener<'input>:
    ParseTreeListener<'input, GQLParserContextType> {

    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlrequest(&mut self, _ctx: &GqlRequestContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlrequest(&mut self, _ctx: &GqlRequestContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlprogram(&mut self, _ctx: &GqlProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlprogram(&mut self, _ctx: &GqlProgramContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_programactivity(&mut self, _ctx: &ProgramActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_programactivity(&mut self, _ctx: &ProgramActivityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionactivity(&mut self, _ctx: &SessionActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionactivity(&mut self, _ctx: &SessionActivityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionactivitycommand(&mut self, _ctx: &SessionActivityCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionactivitycommand(&mut self, _ctx: &SessionActivityCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transactionactivity(&mut self, _ctx: &TransactionActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transactionactivity(&mut self, _ctx: &TransactionActivityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endtransactioncommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endtransactioncommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetcommand(&mut self, _ctx: &SessionSetCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetcommand(&mut self, _ctx: &SessionSetCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetschemaclause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetschemaclause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetgraphclause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetgraphclause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsettimezoneclause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsettimezoneclause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_settimezonevalue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_settimezonevalue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetparameterclause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetparameterclause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetgraphparameterclause(&mut self, _ctx: &SessionSetGraphParameterClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetgraphparameterclause(&mut self, _ctx: &SessionSetGraphParameterClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetbindingtableparameterclause(&mut self, _ctx: &SessionSetBindingTableParameterClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetbindingtableparameterclause(&mut self, _ctx: &SessionSetBindingTableParameterClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetvalueparameterclause(&mut self, _ctx: &SessionSetValueParameterClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetvalueparameterclause(&mut self, _ctx: &SessionSetValueParameterClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionsetparametername(&mut self, _ctx: &SessionSetParameterNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionsetparametername(&mut self, _ctx: &SessionSetParameterNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionresetcommand(&mut self, _ctx: &SessionResetCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionresetcommand(&mut self, _ctx: &SessionResetCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionresetarguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionresetarguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sessionclosecommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sessionclosecommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_starttransactioncommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_starttransactioncommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transactioncharacteristics(&mut self, _ctx: &TransactionCharacteristicsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transactioncharacteristics(&mut self, _ctx: &TransactionCharacteristicsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transactionmode(&mut self, _ctx: &TransactionModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transactionmode(&mut self, _ctx: &TransactionModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transactionaccessmode(&mut self, _ctx: &TransactionAccessModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transactionaccessmode(&mut self, _ctx: &TransactionAccessModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_implementationdefinedaccessmode(&mut self, _ctx: &ImplementationDefinedAccessModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_implementationdefinedaccessmode(&mut self, _ctx: &ImplementationDefinedAccessModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rollbackcommand(&mut self, _ctx: &RollbackCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rollbackcommand(&mut self, _ctx: &RollbackCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_commitcommand(&mut self, _ctx: &CommitCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_commitcommand(&mut self, _ctx: &CommitCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nestedprocedurespecification(&mut self, _ctx: &NestedProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nestedprocedurespecification(&mut self, _ctx: &NestedProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedurespecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedurespecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_catalogmodifyingprocedurespecification(&mut self, _ctx: &CatalogModifyingProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_catalogmodifyingprocedurespecification(&mut self, _ctx: &CatalogModifyingProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nesteddatamodifyingprocedurespecification(&mut self, _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nesteddatamodifyingprocedurespecification(&mut self, _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datamodifyingprocedurespecification(&mut self, _ctx: &DataModifyingProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datamodifyingprocedurespecification(&mut self, _ctx: &DataModifyingProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nestedqueryspecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nestedqueryspecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_queryspecification(&mut self, _ctx: &QuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_queryspecification(&mut self, _ctx: &QuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unsignednumericliteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unsignednumericliteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_integerliteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_integerliteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_floatliteral(&mut self, _ctx: &FloatLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_floatliteral(&mut self, _ctx: &FloatLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unbrokensinglequotedcharactersequence(&mut self, _ctx: &UnbrokenSingleQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unbrokensinglequotedcharactersequence(&mut self, _ctx: &UnbrokenSingleQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unbrokendoublequotedcharactersequence(&mut self, _ctx: &UnbrokenDoubleQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unbrokendoublequotedcharactersequence(&mut self, _ctx: &UnbrokenDoubleQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unbrokenaccentquotedcharactersequence(&mut self, _ctx: &UnbrokenAccentQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unbrokenaccentquotedcharactersequence(&mut self, _ctx: &UnbrokenAccentQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_singlequotedcharactersequence(&mut self, _ctx: &SingleQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_singlequotedcharactersequence(&mut self, _ctx: &SingleQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_doublequotedcharactersequence(&mut self, _ctx: &DoubleQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_doublequotedcharactersequence(&mut self, _ctx: &DoubleQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_accentquotedcharactersequence(&mut self, _ctx: &AccentQuotedCharacterSequenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_accentquotedcharactersequence(&mut self, _ctx: &AccentQuotedCharacterSequenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unbrokencharacterstringliteral(&mut self, _ctx: &UnbrokenCharacterStringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unbrokencharacterstringliteral(&mut self, _ctx: &UnbrokenCharacterStringLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nullliteral(&mut self, _ctx: &NullLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nullliteral(&mut self, _ctx: &NullLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_temporalliteral(&mut self, _ctx: &TemporalLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_temporalliteral(&mut self, _ctx: &TemporalLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sqldatetimeliteral(&mut self, _ctx: &SqlDatetimeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sqldatetimeliteral(&mut self, _ctx: &SqlDatetimeLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dateliteral(&mut self, _ctx: &DateLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dateliteral(&mut self, _ctx: &DateLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timeliteral(&mut self, _ctx: &TimeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timeliteral(&mut self, _ctx: &TimeLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimeliteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimeliteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_durationliteral(&mut self, _ctx: &DurationLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_durationliteral(&mut self, _ctx: &DurationLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sqlintervalliteral(&mut self, _ctx: &SqlIntervalLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sqlintervalliteral(&mut self, _ctx: &SqlIntervalLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sqlintervaltype(&mut self, _ctx: &SqlIntervalTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sqlintervaltype(&mut self, _ctx: &SqlIntervalTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_delimitedidentifier(&mut self, _ctx: &DelimitedIdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_delimitedidentifier(&mut self, _ctx: &DelimitedIdentifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objectname(&mut self, _ctx: &ObjectNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objectname(&mut self, _ctx: &ObjectNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objectnameorbindingvariable(&mut self, _ctx: &ObjectNameOrBindingVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objectnameorbindingvariable(&mut self, _ctx: &ObjectNameOrBindingVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_directoryname(&mut self, _ctx: &DirectoryNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_directoryname(&mut self, _ctx: &DirectoryNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_schemaname(&mut self, _ctx: &SchemaNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_schemaname(&mut self, _ctx: &SchemaNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphname(&mut self, _ctx: &GraphNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphname(&mut self, _ctx: &GraphNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_delimitedgraphname(&mut self, _ctx: &DelimitedGraphNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_delimitedgraphname(&mut self, _ctx: &DelimitedGraphNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphtypename(&mut self, _ctx: &GraphTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphtypename(&mut self, _ctx: &GraphTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementtypename(&mut self, _ctx: &ElementTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementtypename(&mut self, _ctx: &ElementTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtablename(&mut self, _ctx: &BindingTableNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtablename(&mut self, _ctx: &BindingTableNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_delimitedbindingtablename(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_delimitedbindingtablename(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedurename(&mut self, _ctx: &ProcedureNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedurename(&mut self, _ctx: &ProcedureNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelname(&mut self, _ctx: &LabelNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelname(&mut self, _ctx: &LabelNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functionname(&mut self, _ctx: &FunctionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functionname(&mut self, _ctx: &FunctionNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertyname(&mut self, _ctx: &PropertyNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertyname(&mut self, _ctx: &PropertyNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fieldname(&mut self, _ctx: &FieldNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fieldname(&mut self, _ctx: &FieldNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parametername(&mut self, _ctx: &ParameterNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parametername(&mut self, _ctx: &ParameterNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphvariable(&mut self, _ctx: &GraphVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphvariable(&mut self, _ctx: &GraphVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternvariable(&mut self, _ctx: &GraphPatternVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternvariable(&mut self, _ctx: &GraphPatternVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathorsubpathvariable(&mut self, _ctx: &PathOrSubpathVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathorsubpathvariable(&mut self, _ctx: &PathOrSubpathVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementvariable(&mut self, _ctx: &ElementVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementvariable(&mut self, _ctx: &ElementVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathvariable(&mut self, _ctx: &PathVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathvariable(&mut self, _ctx: &PathVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_subpathvariable(&mut self, _ctx: &SubpathVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_subpathvariable(&mut self, _ctx: &SubpathVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtablevariable(&mut self, _ctx: &BindingTableVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtablevariable(&mut self, _ctx: &BindingTableVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuevariable(&mut self, _ctx: &ValueVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuevariable(&mut self, _ctx: &ValueVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingvariable(&mut self, _ctx: &BindingVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingvariable(&mut self, _ctx: &BindingVariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_predefinedtypeliteral(&mut self, _ctx: &PredefinedTypeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predefinedtypeliteral(&mut self, _ctx: &PredefinedTypeLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_booleanliteral(&mut self, _ctx: &BooleanLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_booleanliteral(&mut self, _ctx: &BooleanLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_characterstringliteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_characterstringliteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bytestringliteral(&mut self, _ctx: &ByteStringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bytestringliteral(&mut self, _ctx: &ByteStringLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedurebody(&mut self, _ctx: &ProcedureBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedurebody(&mut self, _ctx: &ProcedureBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingvariabledefinitionblock(&mut self, _ctx: &BindingVariableDefinitionBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingvariabledefinitionblock(&mut self, _ctx: &BindingVariableDefinitionBlockContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingvariabledefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingvariabledefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_statementblock(&mut self, _ctx: &StatementBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_statementblock(&mut self, _ctx: &StatementBlockContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nextstatement(&mut self, _ctx: &NextStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nextstatement(&mut self, _ctx: &NextStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphvariabledefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphvariabledefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opttypedgraphinitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opttypedgraphinitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphinitializer(&mut self, _ctx: &GraphInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphinitializer(&mut self, _ctx: &GraphInitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtablevariabledefinition(&mut self, _ctx: &BindingTableVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtablevariabledefinition(&mut self, _ctx: &BindingTableVariableDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opttypedbindingtableinitializer(&mut self, _ctx: &OptTypedBindingTableInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opttypedbindingtableinitializer(&mut self, _ctx: &OptTypedBindingTableInitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtableinitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtableinitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuevariabledefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuevariabledefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opttypedvalueinitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opttypedvalueinitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphexpression(&mut self, _ctx: &GraphExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphexpression(&mut self, _ctx: &GraphExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_currentgraph(&mut self, _ctx: &CurrentGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_currentgraph(&mut self, _ctx: &CurrentGraphContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nestedgraphqueryspecification(&mut self, _ctx: &NestedGraphQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nestedgraphqueryspecification(&mut self, _ctx: &NestedGraphQuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtableexpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtableexpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nestedbindingtablequeryspecification(&mut self, _ctx: &NestedBindingTableQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nestedbindingtablequeryspecification(&mut self, _ctx: &NestedBindingTableQuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objectexpressionprimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objectexpressionprimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_linearcatalogmodifyingstatement(&mut self, _ctx: &LinearCatalogModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_linearcatalogmodifyingstatement(&mut self, _ctx: &LinearCatalogModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplecatalogmodifyingstatement(&mut self, _ctx: &SimpleCatalogModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplecatalogmodifyingstatement(&mut self, _ctx: &SimpleCatalogModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primitivecatalogmodifyingstatement(&mut self, _ctx: &PrimitiveCatalogModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primitivecatalogmodifyingstatement(&mut self, _ctx: &PrimitiveCatalogModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_createschemastatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_createschemastatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dropschemastatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dropschemastatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_creategraphstatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_creategraphstatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opengraphtype(&mut self, _ctx: &OpenGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opengraphtype(&mut self, _ctx: &OpenGraphTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ofgraphtype(&mut self, _ctx: &OfGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ofgraphtype(&mut self, _ctx: &OfGraphTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphtypelikegraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphtypelikegraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphsource(&mut self, _ctx: &GraphSourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphsource(&mut self, _ctx: &GraphSourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dropgraphstatement(&mut self, _ctx: &DropGraphStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dropgraphstatement(&mut self, _ctx: &DropGraphStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_creategraphtypestatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_creategraphtypestatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphtypesource(&mut self, _ctx: &GraphTypeSourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphtypesource(&mut self, _ctx: &GraphTypeSourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_copyofgraphtype(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_copyofgraphtype(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dropgraphtypestatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dropgraphtypestatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callcatalogmodifyingprocedurestatement(&mut self, _ctx: &CallCatalogModifyingProcedureStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callcatalogmodifyingprocedurestatement(&mut self, _ctx: &CallCatalogModifyingProcedureStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_lineardatamodifyingstatement(&mut self, _ctx: &LinearDataModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lineardatamodifyingstatement(&mut self, _ctx: &LinearDataModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedlineardatamodifyingstatement(&mut self, _ctx: &FocusedLinearDataModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedlineardatamodifyingstatement(&mut self, _ctx: &FocusedLinearDataModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedlineardatamodifyingstatementbody(&mut self, _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedlineardatamodifyingstatementbody(&mut self, _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusednesteddatamodifyingprocedurespecification(&mut self, _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusednesteddatamodifyingprocedurespecification(&mut self, _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ambientlineardatamodifyingstatement(&mut self, _ctx: &AmbientLinearDataModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ambientlineardatamodifyingstatement(&mut self, _ctx: &AmbientLinearDataModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ambientlineardatamodifyingstatementbody(&mut self, _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ambientlineardatamodifyingstatementbody(&mut self, _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplelineardataaccessingstatement(&mut self, _ctx: &SimpleLinearDataAccessingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplelineardataaccessingstatement(&mut self, _ctx: &SimpleLinearDataAccessingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simpledataaccessingstatement(&mut self, _ctx: &SimpleDataAccessingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simpledataaccessingstatement(&mut self, _ctx: &SimpleDataAccessingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simpledatamodifyingstatement(&mut self, _ctx: &SimpleDataModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simpledatamodifyingstatement(&mut self, _ctx: &SimpleDataModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primitivedatamodifyingstatement(&mut self, _ctx: &PrimitiveDataModifyingStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primitivedatamodifyingstatement(&mut self, _ctx: &PrimitiveDataModifyingStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertstatement(&mut self, _ctx: &InsertStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertstatement(&mut self, _ctx: &InsertStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setstatement(&mut self, _ctx: &SetStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setstatement(&mut self, _ctx: &SetStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setitemlist(&mut self, _ctx: &SetItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setitemlist(&mut self, _ctx: &SetItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setitem(&mut self, _ctx: &SetItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setitem(&mut self, _ctx: &SetItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setpropertyitem(&mut self, _ctx: &SetPropertyItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setpropertyitem(&mut self, _ctx: &SetPropertyItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setallpropertiesitem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setallpropertiesitem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setlabelitem(&mut self, _ctx: &SetLabelItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setlabelitem(&mut self, _ctx: &SetLabelItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelsetspecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelsetspecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_removestatement(&mut self, _ctx: &RemoveStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_removestatement(&mut self, _ctx: &RemoveStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_removeitemlist(&mut self, _ctx: &RemoveItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_removeitemlist(&mut self, _ctx: &RemoveItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_removeitem(&mut self, _ctx: &RemoveItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_removeitem(&mut self, _ctx: &RemoveItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_removepropertyitem(&mut self, _ctx: &RemovePropertyItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_removepropertyitem(&mut self, _ctx: &RemovePropertyItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_removelabelitem(&mut self, _ctx: &RemoveLabelItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_removelabelitem(&mut self, _ctx: &RemoveLabelItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_deletestatement(&mut self, _ctx: &DeleteStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_deletestatement(&mut self, _ctx: &DeleteStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_deleteitemlist(&mut self, _ctx: &DeleteItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_deleteitemlist(&mut self, _ctx: &DeleteItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_deleteitem(&mut self, _ctx: &DeleteItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_deleteitem(&mut self, _ctx: &DeleteItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_calldatamodifyingprocedurestatement(&mut self, _ctx: &CallDataModifyingProcedureStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_calldatamodifyingprocedurestatement(&mut self, _ctx: &CallDataModifyingProcedureStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compositequerystatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compositequerystatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compositequeryexpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compositequeryexpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_queryconjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_queryconjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setoperator(&mut self, _ctx: &SetOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setoperator(&mut self, _ctx: &SetOperatorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compositequeryprimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compositequeryprimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_linearquerystatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_linearquerystatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedlinearquerystatement(&mut self, _ctx: &FocusedLinearQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedlinearquerystatement(&mut self, _ctx: &FocusedLinearQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedlinearquerystatementpart(&mut self, _ctx: &FocusedLinearQueryStatementPartContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedlinearquerystatementpart(&mut self, _ctx: &FocusedLinearQueryStatementPartContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedlinearqueryandprimitiveresultstatementpart(&mut self, _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedlinearqueryandprimitiveresultstatementpart(&mut self, _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusedprimitiveresultstatement(&mut self, _ctx: &FocusedPrimitiveResultStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusedprimitiveresultstatement(&mut self, _ctx: &FocusedPrimitiveResultStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_focusednestedqueryspecification(&mut self, _ctx: &FocusedNestedQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_focusednestedqueryspecification(&mut self, _ctx: &FocusedNestedQuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ambientlinearquerystatement(&mut self, _ctx: &AmbientLinearQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ambientlinearquerystatement(&mut self, _ctx: &AmbientLinearQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplelinearquerystatement(&mut self, _ctx: &SimpleLinearQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplelinearquerystatement(&mut self, _ctx: &SimpleLinearQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplequerystatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplequerystatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primitivequerystatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primitivequerystatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_matchstatement(&mut self, _ctx: &MatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_matchstatement(&mut self, _ctx: &MatchStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplematchstatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplematchstatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_optionalmatchstatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_optionalmatchstatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_optionaloperand(&mut self, _ctx: &OptionalOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_optionaloperand(&mut self, _ctx: &OptionalOperandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_matchstatementblock(&mut self, _ctx: &MatchStatementBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_matchstatementblock(&mut self, _ctx: &MatchStatementBlockContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callquerystatement(&mut self, _ctx: &CallQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callquerystatement(&mut self, _ctx: &CallQueryStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_filterstatement(&mut self, _ctx: &FilterStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_filterstatement(&mut self, _ctx: &FilterStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_letstatement(&mut self, _ctx: &LetStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_letstatement(&mut self, _ctx: &LetStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_letvariabledefinitionlist(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_letvariabledefinitionlist(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_letvariabledefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_letvariabledefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_forstatement(&mut self, _ctx: &ForStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_forstatement(&mut self, _ctx: &ForStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_foritem(&mut self, _ctx: &ForItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_foritem(&mut self, _ctx: &ForItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_foritemalias(&mut self, _ctx: &ForItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_foritemalias(&mut self, _ctx: &ForItemAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_forordinalityoroffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_forordinalityoroffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_orderbyandpagestatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_orderbyandpagestatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primitiveresultstatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primitiveresultstatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_returnstatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_returnstatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_returnstatementbody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_returnstatementbody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_returnitemlist(&mut self, _ctx: &ReturnItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_returnitemlist(&mut self, _ctx: &ReturnItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_returnitem(&mut self, _ctx: &ReturnItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_returnitem(&mut self, _ctx: &ReturnItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_returnitemalias(&mut self, _ctx: &ReturnItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_returnitemalias(&mut self, _ctx: &ReturnItemAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectstatement(&mut self, _ctx: &SelectStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectstatement(&mut self, _ctx: &SelectStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectitemlist(&mut self, _ctx: &SelectItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectitemlist(&mut self, _ctx: &SelectItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectitem(&mut self, _ctx: &SelectItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectitem(&mut self, _ctx: &SelectItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectitemalias(&mut self, _ctx: &SelectItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectitemalias(&mut self, _ctx: &SelectItemAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_havingclause(&mut self, _ctx: &HavingClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_havingclause(&mut self, _ctx: &HavingClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectstatementbody(&mut self, _ctx: &SelectStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectstatementbody(&mut self, _ctx: &SelectStatementBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectgraphmatchlist(&mut self, _ctx: &SelectGraphMatchListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectgraphmatchlist(&mut self, _ctx: &SelectGraphMatchListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectgraphmatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectgraphmatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectqueryspecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectqueryspecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callprocedurestatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callprocedurestatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedurecall(&mut self, _ctx: &ProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedurecall(&mut self, _ctx: &ProcedureCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inlineprocedurecall(&mut self, _ctx: &InlineProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inlineprocedurecall(&mut self, _ctx: &InlineProcedureCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variablescopeclause(&mut self, _ctx: &VariableScopeClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variablescopeclause(&mut self, _ctx: &VariableScopeClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingvariablereferencelist(&mut self, _ctx: &BindingVariableReferenceListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingvariablereferencelist(&mut self, _ctx: &BindingVariableReferenceListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_namedprocedurecall(&mut self, _ctx: &NamedProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_namedprocedurecall(&mut self, _ctx: &NamedProcedureCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedureargumentlist(&mut self, _ctx: &ProcedureArgumentListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedureargumentlist(&mut self, _ctx: &ProcedureArgumentListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedureargument(&mut self, _ctx: &ProcedureArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedureargument(&mut self, _ctx: &ProcedureArgumentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_usegraphclause(&mut self, _ctx: &UseGraphClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_usegraphclause(&mut self, _ctx: &UseGraphClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atschemaclause(&mut self, _ctx: &AtSchemaClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atschemaclause(&mut self, _ctx: &AtSchemaClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingvariablereference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingvariablereference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementvariablereference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementvariablereference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathvariablereference(&mut self, _ctx: &PathVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathvariablereference(&mut self, _ctx: &PathVariableReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternbindingtable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternbindingtable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternyieldclause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternyieldclause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternyielditemlist(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternyielditemlist(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternyielditem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternyielditem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpattern(&mut self, _ctx: &GraphPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpattern(&mut self, _ctx: &GraphPatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_matchmode(&mut self, _ctx: &MatchModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_matchmode(&mut self, _ctx: &MatchModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_repeatableelementsmatchmode(&mut self, _ctx: &RepeatableElementsMatchModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_repeatableelementsmatchmode(&mut self, _ctx: &RepeatableElementsMatchModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_differentedgesmatchmode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_differentedgesmatchmode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementbindingsorelements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementbindingsorelements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgebindingsoredges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgebindingsoredges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathpatternlist(&mut self, _ctx: &PathPatternListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathpatternlist(&mut self, _ctx: &PathPatternListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathpattern(&mut self, _ctx: &PathPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathpattern(&mut self, _ctx: &PathPatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathvariabledeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathvariabledeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_keepclause(&mut self, _ctx: &KeepClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_keepclause(&mut self, _ctx: &KeepClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternwhereclause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternwhereclause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathpatternprefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathpatternprefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathmodeprefix(&mut self, _ctx: &PathModePrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathmodeprefix(&mut self, _ctx: &PathModePrefixContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathmode(&mut self, _ctx: &PathModeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathmode(&mut self, _ctx: &PathModeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathsearchprefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathsearchprefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_allpathsearch(&mut self, _ctx: &AllPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_allpathsearch(&mut self, _ctx: &AllPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathorpaths(&mut self, _ctx: &PathOrPathsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathorpaths(&mut self, _ctx: &PathOrPathsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_anypathsearch(&mut self, _ctx: &AnyPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_anypathsearch(&mut self, _ctx: &AnyPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberofpaths(&mut self, _ctx: &NumberOfPathsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberofpaths(&mut self, _ctx: &NumberOfPathsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_shortestpathsearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_shortestpathsearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_allshortestpathsearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_allshortestpathsearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_anyshortestpathsearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_anyshortestpathsearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_countedshortestpathsearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_countedshortestpathsearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_countedshortestgroupsearch(&mut self, _ctx: &CountedShortestGroupSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_countedshortestgroupsearch(&mut self, _ctx: &CountedShortestGroupSearchContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberofgroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberofgroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathpatternexpression(&mut self, _ctx: &PathPatternExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathpatternexpression(&mut self, _ctx: &PathPatternExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathmultisetalternation(&mut self, _ctx: &PathMultisetAlternationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathmultisetalternation(&mut self, _ctx: &PathMultisetAlternationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathpatternunion(&mut self, _ctx: &PathPatternUnionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathpatternunion(&mut self, _ctx: &PathPatternUnionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathterm(&mut self, _ctx: &PathTermContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathterm(&mut self, _ctx: &PathTermContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathfactor(&mut self, _ctx: &PathFactorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathfactor(&mut self, _ctx: &PathFactorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_quantifiedpathprimary(&mut self, _ctx: &QuantifiedPathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_quantifiedpathprimary(&mut self, _ctx: &QuantifiedPathPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_questionedpathprimary(&mut self, _ctx: &QuestionedPathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_questionedpathprimary(&mut self, _ctx: &QuestionedPathPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathprimary(&mut self, _ctx: &PathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathprimary(&mut self, _ctx: &PathPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementpattern(&mut self, _ctx: &ElementPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementpattern(&mut self, _ctx: &ElementPatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodepattern(&mut self, _ctx: &NodePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodepattern(&mut self, _ctx: &NodePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementpatternfiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementpatternfiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementvariabledeclaration(&mut self, _ctx: &ElementVariableDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementvariabledeclaration(&mut self, _ctx: &ElementVariableDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_islabelexpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_islabelexpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_isorcolon(&mut self, _ctx: &IsOrColonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_isorcolon(&mut self, _ctx: &IsOrColonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementpatternpredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementpatternpredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementpatternwhereclause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementpatternwhereclause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementpropertyspecification(&mut self, _ctx: &ElementPropertySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementpropertyspecification(&mut self, _ctx: &ElementPropertySpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertykeyvaluepairlist(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertykeyvaluepairlist(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertykeyvaluepair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertykeyvaluepair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgepattern(&mut self, _ctx: &EdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgepattern(&mut self, _ctx: &EdgePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgepattern(&mut self, _ctx: &FullEdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgepattern(&mut self, _ctx: &FullEdgePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgepointingleft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgepointingleft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgeundirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgeundirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgepointingright(&mut self, _ctx: &FullEdgePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgepointingright(&mut self, _ctx: &FullEdgePointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgeleftorundirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgeleftorundirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgeundirectedorright(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgeundirectedorright(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgeleftorright(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgeleftorright(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgeanydirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgeanydirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgepointingleft(&mut self, _ctx: &AbbreviatedEdgePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgepointingleft(&mut self, _ctx: &AbbreviatedEdgePointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgeundirected(&mut self, _ctx: &AbbreviatedEdgeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgeundirected(&mut self, _ctx: &AbbreviatedEdgeUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgepointingright(&mut self, _ctx: &AbbreviatedEdgePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgepointingright(&mut self, _ctx: &AbbreviatedEdgePointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgeleftorundirected(&mut self, _ctx: &AbbreviatedEdgeLeftOrUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgeleftorundirected(&mut self, _ctx: &AbbreviatedEdgeLeftOrUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgeundirectedorright(&mut self, _ctx: &AbbreviatedEdgeUndirectedOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgeundirectedorright(&mut self, _ctx: &AbbreviatedEdgeUndirectedOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgeleftorright(&mut self, _ctx: &AbbreviatedEdgeLeftOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgeleftorright(&mut self, _ctx: &AbbreviatedEdgeLeftOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgeanydirection(&mut self, _ctx: &AbbreviatedEdgeAnyDirectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgeanydirection(&mut self, _ctx: &AbbreviatedEdgeAnyDirectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedpathpatternexpression(&mut self, _ctx: &ParenthesizedPathPatternExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedpathpatternexpression(&mut self, _ctx: &ParenthesizedPathPatternExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_subpathvariabledeclaration(&mut self, _ctx: &SubpathVariableDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_subpathvariabledeclaration(&mut self, _ctx: &SubpathVariableDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedpathpatternwhereclause(&mut self, _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedpathpatternwhereclause(&mut self, _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertgraphpattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertgraphpattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertpathpatternlist(&mut self, _ctx: &InsertPathPatternListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertpathpatternlist(&mut self, _ctx: &InsertPathPatternListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertpathpattern(&mut self, _ctx: &InsertPathPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertpathpattern(&mut self, _ctx: &InsertPathPatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertnodepattern(&mut self, _ctx: &InsertNodePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertnodepattern(&mut self, _ctx: &InsertNodePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertedgepattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertedgepattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertedgepointingleft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertedgepointingleft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertedgepointingright(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertedgepointingright(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertedgeundirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertedgeundirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_insertelementpatternfiller(&mut self, _ctx: &InsertElementPatternFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_insertelementpatternfiller(&mut self, _ctx: &InsertElementPatternFillerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelandpropertysetspecification(&mut self, _ctx: &LabelAndPropertySetSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelandpropertysetspecification(&mut self, _ctx: &LabelAndPropertySetSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelexpression(&mut self, _ctx: &LabelExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelexpression(&mut self, _ctx: &LabelExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelterm(&mut self, _ctx: &LabelTermContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelterm(&mut self, _ctx: &LabelTermContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelfactor(&mut self, _ctx: &LabelFactorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelfactor(&mut self, _ctx: &LabelFactorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelprimary(&mut self, _ctx: &LabelPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelprimary(&mut self, _ctx: &LabelPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_wildcardlabel(&mut self, _ctx: &WildcardLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_wildcardlabel(&mut self, _ctx: &WildcardLabelContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedlabelexpression(&mut self, _ctx: &ParenthesizedLabelExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedlabelexpression(&mut self, _ctx: &ParenthesizedLabelExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphpatternquantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphpatternquantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedquantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedquantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_generalquantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_generalquantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_lowerbound(&mut self, _ctx: &LowerBoundContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lowerbound(&mut self, _ctx: &LowerBoundContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_upperbound(&mut self, _ctx: &UpperBoundContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_upperbound(&mut self, _ctx: &UpperBoundContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedpathpatternexpression(&mut self, _ctx: &SimplifiedPathPatternExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedpathpatternexpression(&mut self, _ctx: &SimplifiedPathPatternExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingleft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingleft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingundirected(&mut self, _ctx: &SimplifiedDefaultingUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingundirected(&mut self, _ctx: &SimplifiedDefaultingUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingright(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingright(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingleftorundirected(&mut self, _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingleftorundirected(&mut self, _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingundirectedorright(&mut self, _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingundirectedorright(&mut self, _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultingleftorright(&mut self, _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultingleftorright(&mut self, _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddefaultinganydirection(&mut self, _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddefaultinganydirection(&mut self, _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedcontents(&mut self, _ctx: &SimplifiedContentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedcontents(&mut self, _ctx: &SimplifiedContentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedpathunion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedpathunion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedmultisetalternation(&mut self, _ctx: &SimplifiedMultisetAlternationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedmultisetalternation(&mut self, _ctx: &SimplifiedMultisetAlternationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedterm(&mut self, _ctx: &SimplifiedTermContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedterm(&mut self, _ctx: &SimplifiedTermContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedfactorlow(&mut self, _ctx: &SimplifiedFactorLowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedfactorlow(&mut self, _ctx: &SimplifiedFactorLowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedfactorhigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedfactorhigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedquantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedquantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedquestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedquestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedtertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedtertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifieddirectionoverride(&mut self, _ctx: &SimplifiedDirectionOverrideContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifieddirectionoverride(&mut self, _ctx: &SimplifiedDirectionOverrideContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideleft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideleft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideundirected(&mut self, _ctx: &SimplifiedOverrideUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideundirected(&mut self, _ctx: &SimplifiedOverrideUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideright(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideright(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideleftorundirected(&mut self, _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideleftorundirected(&mut self, _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideundirectedorright(&mut self, _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideundirectedorright(&mut self, _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideleftorright(&mut self, _ctx: &SimplifiedOverrideLeftOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideleftorright(&mut self, _ctx: &SimplifiedOverrideLeftOrRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedoverrideanydirection(&mut self, _ctx: &SimplifiedOverrideAnyDirectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedoverrideanydirection(&mut self, _ctx: &SimplifiedOverrideAnyDirectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedsecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedsecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiednegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiednegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedprimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedprimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_whereclause(&mut self, _ctx: &WhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_whereclause(&mut self, _ctx: &WhereClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_yieldclause(&mut self, _ctx: &YieldClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_yieldclause(&mut self, _ctx: &YieldClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_yielditemlist(&mut self, _ctx: &YieldItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_yielditemlist(&mut self, _ctx: &YieldItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_yielditem(&mut self, _ctx: &YieldItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_yielditem(&mut self, _ctx: &YieldItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_yielditemname(&mut self, _ctx: &YieldItemNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_yielditemname(&mut self, _ctx: &YieldItemNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_yielditemalias(&mut self, _ctx: &YieldItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_yielditemalias(&mut self, _ctx: &YieldItemAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_groupbyclause(&mut self, _ctx: &GroupByClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_groupbyclause(&mut self, _ctx: &GroupByClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_groupingelementlist(&mut self, _ctx: &GroupingElementListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_groupingelementlist(&mut self, _ctx: &GroupingElementListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_groupingelement(&mut self, _ctx: &GroupingElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_groupingelement(&mut self, _ctx: &GroupingElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_emptygroupingset(&mut self, _ctx: &EmptyGroupingSetContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_emptygroupingset(&mut self, _ctx: &EmptyGroupingSetContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_orderbyclause(&mut self, _ctx: &OrderByClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_orderbyclause(&mut self, _ctx: &OrderByClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlcountallfunction(&mut self, _ctx: &GqlCountAllFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlcountallfunction(&mut self, _ctx: &GqlCountAllFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlgeneralsetfunction(&mut self, _ctx: &GqlGeneralSetFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlgeneralsetfunction(&mut self, _ctx: &GqlGeneralSetFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlbinarysetfunction(&mut self, _ctx: &GqlBinarySetFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlbinarysetfunction(&mut self, _ctx: &GqlBinarySetFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_generalsetfunctiontype(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_generalsetfunctiontype(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_setquantifier(&mut self, _ctx: &SetQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_setquantifier(&mut self, _ctx: &SetQuantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binarysetfunctiontype(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binarysetfunctiontype(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sortspecificationlist(&mut self, _ctx: &SortSpecificationListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sortspecificationlist(&mut self, _ctx: &SortSpecificationListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sortspecification(&mut self, _ctx: &SortSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sortspecification(&mut self, _ctx: &SortSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sortkey(&mut self, _ctx: &SortKeyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sortkey(&mut self, _ctx: &SortKeyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_orderingspecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_orderingspecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nullordering(&mut self, _ctx: &NullOrderingContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nullordering(&mut self, _ctx: &NullOrderingContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_limitclause(&mut self, _ctx: &LimitClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_limitclause(&mut self, _ctx: &LimitClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_offsetclause(&mut self, _ctx: &OffsetClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_offsetclause(&mut self, _ctx: &OffsetClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_offsetsynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_offsetsynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nestedgraphtypespecification(&mut self, _ctx: &NestedGraphTypeSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nestedgraphtypespecification(&mut self, _ctx: &NestedGraphTypeSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphtypespecificationbody(&mut self, _ctx: &GraphTypeSpecificationBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphtypespecificationbody(&mut self, _ctx: &GraphTypeSpecificationBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementtypedefinitionlist(&mut self, _ctx: &ElementTypeDefinitionListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementtypedefinitionlist(&mut self, _ctx: &ElementTypeDefinitionListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementtypedefinition(&mut self, _ctx: &ElementTypeDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementtypedefinition(&mut self, _ctx: &ElementTypeDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypedefinition(&mut self, _ctx: &NodeTypeDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypedefinition(&mut self, _ctx: &NodeTypeDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypepattern(&mut self, _ctx: &NodeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypepattern(&mut self, _ctx: &NodeTypePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypephrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypephrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypename(&mut self, _ctx: &NodeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypename(&mut self, _ctx: &NodeTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypefiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypefiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypelabelsetdefinition(&mut self, _ctx: &NodeTypeLabelSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypelabelsetdefinition(&mut self, _ctx: &NodeTypeLabelSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypepropertytypesetdefinition(&mut self, _ctx: &NodeTypePropertyTypeSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypepropertytypesetdefinition(&mut self, _ctx: &NodeTypePropertyTypeSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypedefinition(&mut self, _ctx: &EdgeTypeDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypedefinition(&mut self, _ctx: &EdgeTypeDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypepattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypepattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypephrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypephrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypename(&mut self, _ctx: &EdgeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypename(&mut self, _ctx: &EdgeTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypefiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypefiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypelabelsetdefinition(&mut self, _ctx: &EdgeTypeLabelSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypelabelsetdefinition(&mut self, _ctx: &EdgeTypeLabelSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgetypepropertytypesetdefinition(&mut self, _ctx: &EdgeTypePropertyTypeSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgetypepropertytypesetdefinition(&mut self, _ctx: &EdgeTypePropertyTypeSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgetypepattern(&mut self, _ctx: &FullEdgeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgetypepattern(&mut self, _ctx: &FullEdgeTypePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgetypepatternpointingright(&mut self, _ctx: &FullEdgeTypePatternPointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgetypepatternpointingright(&mut self, _ctx: &FullEdgeTypePatternPointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgetypepatternpointingleft(&mut self, _ctx: &FullEdgeTypePatternPointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgetypepatternpointingleft(&mut self, _ctx: &FullEdgeTypePatternPointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fulledgetypepatternundirected(&mut self, _ctx: &FullEdgeTypePatternUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fulledgetypepatternundirected(&mut self, _ctx: &FullEdgeTypePatternUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arctypepointingright(&mut self, _ctx: &ArcTypePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arctypepointingright(&mut self, _ctx: &ArcTypePointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arctypepointingleft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arctypepointingleft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arctypeundirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arctypeundirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arctypefiller(&mut self, _ctx: &ArcTypeFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arctypefiller(&mut self, _ctx: &ArcTypeFillerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgetypepattern(&mut self, _ctx: &AbbreviatedEdgeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgetypepattern(&mut self, _ctx: &AbbreviatedEdgeTypePatternContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgetypepatternpointingright(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgetypepatternpointingright(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgetypepatternpointingleft(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgetypepatternpointingleft(&mut self, _ctx: &AbbreviatedEdgeTypePatternPointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatededgetypepatternundirected(&mut self, _ctx: &AbbreviatedEdgeTypePatternUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatededgetypepatternundirected(&mut self, _ctx: &AbbreviatedEdgeTypePatternUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodetypereference(&mut self, _ctx: &NodeTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodetypereference(&mut self, _ctx: &NodeTypeReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sourcenodetypereference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sourcenodetypereference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_destinationnodetypereference(&mut self, _ctx: &DestinationNodeTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_destinationnodetypereference(&mut self, _ctx: &DestinationNodeTypeReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgekind(&mut self, _ctx: &EdgeKindContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgekind(&mut self, _ctx: &EdgeKindContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endpointdefinition(&mut self, _ctx: &EndpointDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endpointdefinition(&mut self, _ctx: &EndpointDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endpointpairdefinition(&mut self, _ctx: &EndpointPairDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endpointpairdefinition(&mut self, _ctx: &EndpointPairDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endpointpairdefinitionpointingright(&mut self, _ctx: &EndpointPairDefinitionPointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endpointpairdefinitionpointingright(&mut self, _ctx: &EndpointPairDefinitionPointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endpointpairdefinitionpointingleft(&mut self, _ctx: &EndpointPairDefinitionPointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endpointpairdefinitionpointingleft(&mut self, _ctx: &EndpointPairDefinitionPointingLeftContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_endpointpairdefinitionundirected(&mut self, _ctx: &EndpointPairDefinitionUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_endpointpairdefinitionundirected(&mut self, _ctx: &EndpointPairDefinitionUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_connectorpointingright(&mut self, _ctx: &ConnectorPointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_connectorpointingright(&mut self, _ctx: &ConnectorPointingRightContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_connectorundirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_connectorundirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sourcenodetypename(&mut self, _ctx: &SourceNodeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sourcenodetypename(&mut self, _ctx: &SourceNodeTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_destinationnodetypename(&mut self, _ctx: &DestinationNodeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_destinationnodetypename(&mut self, _ctx: &DestinationNodeTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labelsetdefinition(&mut self, _ctx: &LabelSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labelsetdefinition(&mut self, _ctx: &LabelSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertytypesetdefinition(&mut self, _ctx: &PropertyTypeSetDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertytypesetdefinition(&mut self, _ctx: &PropertyTypeSetDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertytypedefinitionlist(&mut self, _ctx: &PropertyTypeDefinitionListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertytypedefinitionlist(&mut self, _ctx: &PropertyTypeDefinitionListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertytypedefinition(&mut self, _ctx: &PropertyTypeDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertytypedefinition(&mut self, _ctx: &PropertyTypeDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertyvaluetype(&mut self, _ctx: &PropertyValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertyvaluetype(&mut self, _ctx: &PropertyValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtabletype(&mut self, _ctx: &BindingTableTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtabletype(&mut self, _ctx: &BindingTableTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dynamicpropertyvaluetype(&mut self, _ctx: &DynamicPropertyValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dynamicpropertyvaluetype(&mut self, _ctx: &DynamicPropertyValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_predeftype(&mut self, _ctx: &PredefTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predeftype(&mut self, _ctx: &PredefTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listtype1(&mut self, _ctx: &ListType1Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listtype1(&mut self, _ctx: &ListType1Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_recordtype1(&mut self, _ctx: &RecordType1Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_recordtype1(&mut self, _ctx: &RecordType1Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_closeddynamicuniontype2(&mut self, _ctx: &ClosedDynamicUnionType2Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_closeddynamicuniontype2(&mut self, _ctx: &ClosedDynamicUnionType2Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_recordtype2(&mut self, _ctx: &RecordType2Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_recordtype2(&mut self, _ctx: &RecordType2Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listtype2(&mut self, _ctx: &ListType2Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listtype2(&mut self, _ctx: &ListType2Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathtype(&mut self, _ctx: &PathTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathtype(&mut self, _ctx: &PathTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opendynamicuniontype(&mut self, _ctx: &OpenDynamicUnionTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opendynamicuniontype(&mut self, _ctx: &OpenDynamicUnionTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_closeddynamicuniontype1(&mut self, _ctx: &ClosedDynamicUnionType1Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_closeddynamicuniontype1(&mut self, _ctx: &ClosedDynamicUnionType1Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typed(&mut self, _ctx: &TypedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typed(&mut self, _ctx: &TypedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_predefinedtype(&mut self, _ctx: &PredefinedTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predefinedtype(&mut self, _ctx: &PredefinedTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_booleantype(&mut self, _ctx: &BooleanTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_booleantype(&mut self, _ctx: &BooleanTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_characterstringtype(&mut self, _ctx: &CharacterStringTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_characterstringtype(&mut self, _ctx: &CharacterStringTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bytestringtype(&mut self, _ctx: &ByteStringTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bytestringtype(&mut self, _ctx: &ByteStringTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_minlength(&mut self, _ctx: &MinLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_minlength(&mut self, _ctx: &MinLengthContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_maxlength(&mut self, _ctx: &MaxLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_maxlength(&mut self, _ctx: &MaxLengthContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedlength(&mut self, _ctx: &FixedLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedlength(&mut self, _ctx: &FixedLengthContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numerictype(&mut self, _ctx: &NumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numerictype(&mut self, _ctx: &NumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_exactnumerictype(&mut self, _ctx: &ExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_exactnumerictype(&mut self, _ctx: &ExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binaryexactnumerictype(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binaryexactnumerictype(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_signedbinaryexactnumerictype(&mut self, _ctx: &SignedBinaryExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_signedbinaryexactnumerictype(&mut self, _ctx: &SignedBinaryExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unsignedbinaryexactnumerictype(&mut self, _ctx: &UnsignedBinaryExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unsignedbinaryexactnumerictype(&mut self, _ctx: &UnsignedBinaryExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_verbosebinaryexactnumerictype(&mut self, _ctx: &VerboseBinaryExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_verbosebinaryexactnumerictype(&mut self, _ctx: &VerboseBinaryExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decimalexactnumerictype(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decimalexactnumerictype(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precision(&mut self, _ctx: &PrecisionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precision(&mut self, _ctx: &PrecisionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_scale(&mut self, _ctx: &ScaleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_scale(&mut self, _ctx: &ScaleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_approximatenumerictype(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_approximatenumerictype(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_temporaltype(&mut self, _ctx: &TemporalTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_temporaltype(&mut self, _ctx: &TemporalTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_temporalinstanttype(&mut self, _ctx: &TemporalInstantTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_temporalinstanttype(&mut self, _ctx: &TemporalInstantTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_temporaldurationtype(&mut self, _ctx: &TemporalDurationTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_temporaldurationtype(&mut self, _ctx: &TemporalDurationTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimetype(&mut self, _ctx: &DatetimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimetype(&mut self, _ctx: &DatetimeTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_localdatetimetype(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_localdatetimetype(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetype(&mut self, _ctx: &DateTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetype(&mut self, _ctx: &DateTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timetype(&mut self, _ctx: &TimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timetype(&mut self, _ctx: &TimeTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_localtimetype(&mut self, _ctx: &LocaltimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_localtimetype(&mut self, _ctx: &LocaltimeTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_durationtype(&mut self, _ctx: &DurationTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_durationtype(&mut self, _ctx: &DurationTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_referencevaluetype(&mut self, _ctx: &ReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_referencevaluetype(&mut self, _ctx: &ReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphreferencevaluetype(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphreferencevaluetype(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_closedgraphreferencevaluetype(&mut self, _ctx: &ClosedGraphReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_closedgraphreferencevaluetype(&mut self, _ctx: &ClosedGraphReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opengraphreferencevaluetype(&mut self, _ctx: &OpenGraphReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opengraphreferencevaluetype(&mut self, _ctx: &OpenGraphReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtablereferencevaluetype(&mut self, _ctx: &BindingTableReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtablereferencevaluetype(&mut self, _ctx: &BindingTableReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nodereferencevaluetype(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nodereferencevaluetype(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_closednodereferencevaluetype(&mut self, _ctx: &ClosedNodeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_closednodereferencevaluetype(&mut self, _ctx: &ClosedNodeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_opennodereferencevaluetype(&mut self, _ctx: &OpenNodeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opennodereferencevaluetype(&mut self, _ctx: &OpenNodeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_edgereferencevaluetype(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_edgereferencevaluetype(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_closededgereferencevaluetype(&mut self, _ctx: &ClosedEdgeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_closededgereferencevaluetype(&mut self, _ctx: &ClosedEdgeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_openedgereferencevaluetype(&mut self, _ctx: &OpenEdgeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_openedgereferencevaluetype(&mut self, _ctx: &OpenEdgeReferenceValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listvaluetypename(&mut self, _ctx: &ListValueTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listvaluetypename(&mut self, _ctx: &ListValueTypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listvaluetypenamesynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listvaluetypenamesynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fieldtypesspecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fieldtypesspecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fieldtypelist(&mut self, _ctx: &FieldTypeListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fieldtypelist(&mut self, _ctx: &FieldTypeListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathvaluetype(&mut self, _ctx: &PathValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathvaluetype(&mut self, _ctx: &PathValueTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_notnull(&mut self, _ctx: &NotNullContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_notnull(&mut self, _ctx: &NotNullContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fieldtype(&mut self, _ctx: &FieldTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fieldtype(&mut self, _ctx: &FieldTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_schemareference(&mut self, _ctx: &SchemaReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_schemareference(&mut self, _ctx: &SchemaReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_absolutecatalogschemareference(&mut self, _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_absolutecatalogschemareference(&mut self, _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_catalogschemaparentandname(&mut self, _ctx: &CatalogSchemaParentAndNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_catalogschemaparentandname(&mut self, _ctx: &CatalogSchemaParentAndNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_relativecatalogschemareference(&mut self, _ctx: &RelativeCatalogSchemaReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_relativecatalogschemareference(&mut self, _ctx: &RelativeCatalogSchemaReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_predefinedschemareference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predefinedschemareference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_absolutedirectorypath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_absolutedirectorypath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_relativedirectorypath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_relativedirectorypath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simpledirectorypath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simpledirectorypath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphreference(&mut self, _ctx: &GraphReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphreference(&mut self, _ctx: &GraphReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cataloggraphparentandname(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cataloggraphparentandname(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_homegraph(&mut self, _ctx: &HomeGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_homegraph(&mut self, _ctx: &HomeGraphContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_graphtypereference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_graphtypereference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cataloggraphtypeparentandname(&mut self, _ctx: &CatalogGraphTypeParentAndNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cataloggraphtypeparentandname(&mut self, _ctx: &CatalogGraphTypeParentAndNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bindingtablereference(&mut self, _ctx: &BindingTableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bindingtablereference(&mut self, _ctx: &BindingTableReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_catalogbindingtableparentandname(&mut self, _ctx: &CatalogBindingTableParentAndNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_catalogbindingtableparentandname(&mut self, _ctx: &CatalogBindingTableParentAndNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_procedurereference(&mut self, _ctx: &ProcedureReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_procedurereference(&mut self, _ctx: &ProcedureReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_catalogprocedureparentandname(&mut self, _ctx: &CatalogProcedureParentAndNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_catalogprocedureparentandname(&mut self, _ctx: &CatalogProcedureParentAndNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_catalogobjectparentreference(&mut self, _ctx: &CatalogObjectParentReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_catalogobjectparentreference(&mut self, _ctx: &CatalogObjectParentReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_referenceparameter(&mut self, _ctx: &ReferenceParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_referenceparameter(&mut self, _ctx: &ReferenceParameterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_externalobjectreference(&mut self, _ctx: &ExternalObjectReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_externalobjectreference(&mut self, _ctx: &ExternalObjectReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comparisonpredicatecond(&mut self, _ctx: &ComparisonPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comparisonpredicatecond(&mut self, _ctx: &ComparisonPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compop(&mut self, _ctx: &CompOpContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compop(&mut self, _ctx: &CompOpContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nullpredicatecond(&mut self, _ctx: &NullPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nullpredicatecond(&mut self, _ctx: &NullPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_normalizedpredicatecond(&mut self, _ctx: &NormalizedPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_normalizedpredicatecond(&mut self, _ctx: &NormalizedPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_directedpredicatecond(&mut self, _ctx: &DirectedPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_directedpredicatecond(&mut self, _ctx: &DirectedPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labeledpredicatecond(&mut self, _ctx: &LabeledPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labeledpredicatecond(&mut self, _ctx: &LabeledPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sourcedestinationpredicatecond(&mut self, _ctx: &SourceDestinationPredicateCondContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sourcedestinationpredicatecond(&mut self, _ctx: &SourceDestinationPredicateCondContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unsignedvaluespecification(&mut self, _ctx: &UnsignedValueSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unsignedvaluespecification(&mut self, _ctx: &UnsignedValueSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unsignedintegerspecification(&mut self, _ctx: &UnsignedIntegerSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unsignedintegerspecification(&mut self, _ctx: &UnsignedIntegerSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parametervaluespecification(&mut self, _ctx: &ParameterValueSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parametervaluespecification(&mut self, _ctx: &ParameterValueSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_predefinedparameter(&mut self, _ctx: &PredefinedParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predefinedparameter(&mut self, _ctx: &PredefinedParameterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlpredicateexpression(&mut self, _ctx: &GqlPredicateExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlpredicateexpression(&mut self, _ctx: &GqlPredicateExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlnotexpression(&mut self, _ctx: &GqlNotExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlnotexpression(&mut self, _ctx: &GqlNotExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllogicalxorexpression(&mut self, _ctx: &GqlLogicalXorExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllogicalxorexpression(&mut self, _ctx: &GqlLogicalXorExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllogicalorexpression(&mut self, _ctx: &GqlLogicalOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllogicalorexpression(&mut self, _ctx: &GqlLogicalOrExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllogicalandexpression(&mut self, _ctx: &GqlLogicalAndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllogicalandexpression(&mut self, _ctx: &GqlLogicalAndExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlatomexpression(&mut self, _ctx: &GqlAtomExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlatomexpression(&mut self, _ctx: &GqlAtomExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlnormalizedexpression(&mut self, _ctx: &GqlNormalizedExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlnormalizedexpression(&mut self, _ctx: &GqlNormalizedExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsameexpression(&mut self, _ctx: &GqlSameExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsameexpression(&mut self, _ctx: &GqlSameExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqldirectedexpression(&mut self, _ctx: &GqlDirectedExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqldirectedexpression(&mut self, _ctx: &GqlDirectedExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlbooleantestexpression(&mut self, _ctx: &GqlBooleanTestExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlbooleantestexpression(&mut self, _ctx: &GqlBooleanTestExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlgraphrefvalueexpression(&mut self, _ctx: &GqlGraphRefValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlgraphrefvalueexpression(&mut self, _ctx: &GqlGraphRefValueExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlalldifferentexpression(&mut self, _ctx: &GqlAllDifferentExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlalldifferentexpression(&mut self, _ctx: &GqlAllDifferentExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlexistsexpression(&mut self, _ctx: &GqlExistsExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlexistsexpression(&mut self, _ctx: &GqlExistsExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlletexpression(&mut self, _ctx: &GqlLetExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlletexpression(&mut self, _ctx: &GqlLetExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlcomparisonexpression(&mut self, _ctx: &GqlComparisonExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlcomparisonexpression(&mut self, _ctx: &GqlComparisonExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllabeledexpression(&mut self, _ctx: &GqlLabeledExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllabeledexpression(&mut self, _ctx: &GqlLabeledExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlpropertyexistexpression(&mut self, _ctx: &GqlPropertyExistExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlpropertyexistexpression(&mut self, _ctx: &GqlPropertyExistExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlnullexpression(&mut self, _ctx: &GqlNullExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlnullexpression(&mut self, _ctx: &GqlNullExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsourcedestinationexpression(&mut self, _ctx: &GqlSourceDestinationExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsourcedestinationexpression(&mut self, _ctx: &GqlSourceDestinationExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlbindingtablevalueexpression(&mut self, _ctx: &GqlBindingTableValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlbindingtablevalueexpression(&mut self, _ctx: &GqlBindingTableValueExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlparameterexpression(&mut self, _ctx: &GqlParameterExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlparameterexpression(&mut self, _ctx: &GqlParameterExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlunaryexpression(&mut self, _ctx: &GqlUnaryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlunaryexpression(&mut self, _ctx: &GqlUnaryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlpropertyreference(&mut self, _ctx: &GqlPropertyReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlpropertyreference(&mut self, _ctx: &GqlPropertyReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlhigharithmeticexpression(&mut self, _ctx: &GqlHighArithmeticExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlhigharithmeticexpression(&mut self, _ctx: &GqlHighArithmeticExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlvaluequeryexpression(&mut self, _ctx: &GqlValueQueryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlvaluequeryexpression(&mut self, _ctx: &GqlValueQueryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlliteralexpression(&mut self, _ctx: &GqlLiteralExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlliteralexpression(&mut self, _ctx: &GqlLiteralExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlcollectionexpression(&mut self, _ctx: &GqlCollectionExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlcollectionexpression(&mut self, _ctx: &GqlCollectionExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlconcatenationexpression(&mut self, _ctx: &GqlConcatenationExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlconcatenationexpression(&mut self, _ctx: &GqlConcatenationExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllowarithmeticexpression(&mut self, _ctx: &GqlLowArithmeticExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllowarithmeticexpression(&mut self, _ctx: &GqlLowArithmeticExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlparenthesizedexpression(&mut self, _ctx: &GqlParenthesizedExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlparenthesizedexpression(&mut self, _ctx: &GqlParenthesizedExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlfunctionexpression(&mut self, _ctx: &GqlFunctionExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlfunctionexpression(&mut self, _ctx: &GqlFunctionExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlvariableexpression(&mut self, _ctx: &GqlVariableExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlvariableexpression(&mut self, _ctx: &GqlVariableExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_truthvalue(&mut self, _ctx: &TruthValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_truthvalue(&mut self, _ctx: &TruthValueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryoperator(&mut self, _ctx: &UnaryOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryoperator(&mut self, _ctx: &UnaryOperatorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functioncall(&mut self, _ctx: &FunctionCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functioncall(&mut self, _ctx: &FunctionCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqloneargscalarfunction(&mut self, _ctx: &GqlOneArgScalarFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqloneargscalarfunction(&mut self, _ctx: &GqlOneArgScalarFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqltwoargscalarfunction(&mut self, _ctx: &GqlTwoArgScalarFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqltwoargscalarfunction(&mut self, _ctx: &GqlTwoArgScalarFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functionparameter(&mut self, _ctx: &FunctionParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functionparameter(&mut self, _ctx: &FunctionParameterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_propertyreference(&mut self, _ctx: &PropertyReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_propertyreference(&mut self, _ctx: &PropertyReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_oneargnumericfunctionname(&mut self, _ctx: &OneArgNumericFunctionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_oneargnumericfunctionname(&mut self, _ctx: &OneArgNumericFunctionNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_twoargnumericfunctionname(&mut self, _ctx: &TwoArgNumericFunctionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_twoargnumericfunctionname(&mut self, _ctx: &TwoArgNumericFunctionNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsubstringfunction(&mut self, _ctx: &GqlSubstringFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsubstringfunction(&mut self, _ctx: &GqlSubstringFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlfoldstringfunction(&mut self, _ctx: &GqlFoldStringFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlfoldstringfunction(&mut self, _ctx: &GqlFoldStringFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsingletrimstringfunction(&mut self, _ctx: &GqlSingleTrimStringFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsingletrimstringfunction(&mut self, _ctx: &GqlSingleTrimStringFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlmultitrimstringfunction(&mut self, _ctx: &GqlMultiTrimStringFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlmultitrimstringfunction(&mut self, _ctx: &GqlMultiTrimStringFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlnormstringfunction(&mut self, _ctx: &GqlNormStringFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlnormstringfunction(&mut self, _ctx: &GqlNormStringFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqllisttrimfunction(&mut self, _ctx: &GqlListTrimFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqllisttrimfunction(&mut self, _ctx: &GqlListTrimFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlelementsofpathfunction(&mut self, _ctx: &GqlElementsOfPathFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlelementsofpathfunction(&mut self, _ctx: &GqlElementsOfPathFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlnullifcasefunction(&mut self, _ctx: &GqlNullIfCaseFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlnullifcasefunction(&mut self, _ctx: &GqlNullIfCaseFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlcoalescecasefunction(&mut self, _ctx: &GqlCoalesceCaseFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlcoalescecasefunction(&mut self, _ctx: &GqlCoalesceCaseFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsimplecasefunction(&mut self, _ctx: &GqlSimpleCaseFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsimplecasefunction(&mut self, _ctx: &GqlSimpleCaseFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqlsearchedcasefunction(&mut self, _ctx: &GqlSearchedCaseFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqlsearchedcasefunction(&mut self, _ctx: &GqlSearchedCaseFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_simplewhenclause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_simplewhenclause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_searchedwhenclause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_searchedwhenclause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elseclause(&mut self, _ctx: &ElseClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elseclause(&mut self, _ctx: &ElseClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_whenoperand(&mut self, _ctx: &WhenOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_whenoperand(&mut self, _ctx: &WhenOperandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_castfunction(&mut self, _ctx: &CastFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_castfunction(&mut self, _ctx: &CastFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementidfunction(&mut self, _ctx: &ElementIdFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementidfunction(&mut self, _ctx: &ElementIdFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimevaluefunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimevaluefunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datefunction(&mut self, _ctx: &DateFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datefunction(&mut self, _ctx: &DateFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timefunction(&mut self, _ctx: &TimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timefunction(&mut self, _ctx: &TimeFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_localtimefunction(&mut self, _ctx: &LocalTimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_localtimefunction(&mut self, _ctx: &LocalTimeFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimefunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimefunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_localdatetimefunction(&mut self, _ctx: &LocalDatetimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_localdatetimefunction(&mut self, _ctx: &LocalDatetimeFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datefunctionparameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datefunctionparameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timefunctionparameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timefunctionparameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimefunctionparameters(&mut self, _ctx: &DatetimeFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimefunctionparameters(&mut self, _ctx: &DatetimeFunctionParametersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datestring(&mut self, _ctx: &DateStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datestring(&mut self, _ctx: &DateStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timestring(&mut self, _ctx: &TimeStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timestring(&mut self, _ctx: &TimeStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetimestring(&mut self, _ctx: &DatetimeStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetimestring(&mut self, _ctx: &DatetimeStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqldatetimesubtractionfunction(&mut self, _ctx: &GqlDatetimeSubtractionFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqldatetimesubtractionfunction(&mut self, _ctx: &GqlDatetimeSubtractionFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gqldurationfunction(&mut self, _ctx: &GqlDurationFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gqldurationfunction(&mut self, _ctx: &GqlDurationFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_durationfunctionparameters(&mut self, _ctx: &DurationFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_durationfunctionparameters(&mut self, _ctx: &DurationFunctionParametersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_durationstring(&mut self, _ctx: &DurationStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_durationstring(&mut self, _ctx: &DurationStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_generalfunction(&mut self, _ctx: &GeneralFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_generalfunction(&mut self, _ctx: &GeneralFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_collectionvalueconstructor(&mut self, _ctx: &CollectionValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_collectionvalueconstructor(&mut self, _ctx: &CollectionValueConstructorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_trimspecification(&mut self, _ctx: &TrimSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_trimspecification(&mut self, _ctx: &TrimSpecificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_normalform(&mut self, _ctx: &NormalFormContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_normalform(&mut self, _ctx: &NormalFormContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listvalueconstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listvalueconstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_recordvalueconstructor(&mut self, _ctx: &RecordValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_recordvalueconstructor(&mut self, _ctx: &RecordValueConstructorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_field(&mut self, _ctx: &FieldContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_field(&mut self, _ctx: &FieldContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pathvalueconstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pathvalueconstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unsignedliteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unsignedliteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_generalliteral(&mut self, _ctx: &GeneralLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_generalliteral(&mut self, _ctx: &GeneralLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listliteral(&mut self, _ctx: &ListLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listliteral(&mut self, _ctx: &ListLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_recordliteral(&mut self, _ctx: &RecordLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_recordliteral(&mut self, _ctx: &RecordLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_recordfieldliteral(&mut self, _ctx: &RecordFieldLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  GQLBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_recordfieldliteral(&mut self, _ctx: &RecordFieldLiteralContext<'input>) {}


}