// Generated from Cypher.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::cypherlistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const Cypher_T__0:i32=1; 
		pub const Cypher_T__1:i32=2; 
		pub const Cypher_T__2:i32=3; 
		pub const Cypher_T__3:i32=4; 
		pub const Cypher_T__4:i32=5; 
		pub const Cypher_T__5:i32=6; 
		pub const Cypher_T__6:i32=7; 
		pub const Cypher_T__7:i32=8; 
		pub const Cypher_T__8:i32=9; 
		pub const Cypher_T__9:i32=10; 
		pub const Cypher_T__10:i32=11; 
		pub const Cypher_T__11:i32=12; 
		pub const Cypher_T__12:i32=13; 
		pub const Cypher_T__13:i32=14; 
		pub const Cypher_T__14:i32=15; 
		pub const Cypher_T__15:i32=16; 
		pub const Cypher_T__16:i32=17; 
		pub const Cypher_T__17:i32=18; 
		pub const Cypher_T__18:i32=19; 
		pub const Cypher_T__19:i32=20; 
		pub const Cypher_T__20:i32=21; 
		pub const Cypher_T__21:i32=22; 
		pub const Cypher_T__22:i32=23; 
		pub const Cypher_T__23:i32=24; 
		pub const Cypher_T__24:i32=25; 
		pub const Cypher_T__25:i32=26; 
		pub const Cypher_T__26:i32=27; 
		pub const Cypher_T__27:i32=28; 
		pub const Cypher_T__28:i32=29; 
		pub const Cypher_T__29:i32=30; 
		pub const Cypher_T__30:i32=31; 
		pub const Cypher_T__31:i32=32; 
		pub const Cypher_T__32:i32=33; 
		pub const Cypher_T__33:i32=34; 
		pub const Cypher_T__34:i32=35; 
		pub const Cypher_T__35:i32=36; 
		pub const Cypher_T__36:i32=37; 
		pub const Cypher_T__37:i32=38; 
		pub const Cypher_T__38:i32=39; 
		pub const Cypher_T__39:i32=40; 
		pub const Cypher_T__40:i32=41; 
		pub const Cypher_T__41:i32=42; 
		pub const Cypher_T__42:i32=43; 
		pub const Cypher_T__43:i32=44; 
		pub const Cypher_T__44:i32=45; 
		pub const Cypher_UNION:i32=46; 
		pub const Cypher_ALL:i32=47; 
		pub const Cypher_OPTIONAL:i32=48; 
		pub const Cypher_MATCH:i32=49; 
		pub const Cypher_UNWIND:i32=50; 
		pub const Cypher_AS:i32=51; 
		pub const Cypher_MERGE:i32=52; 
		pub const Cypher_ON:i32=53; 
		pub const Cypher_CREATE:i32=54; 
		pub const Cypher_SET:i32=55; 
		pub const Cypher_DETACH:i32=56; 
		pub const Cypher_DELETE:i32=57; 
		pub const Cypher_REMOVE:i32=58; 
		pub const Cypher_CALL:i32=59; 
		pub const Cypher_YIELD:i32=60; 
		pub const Cypher_WITH:i32=61; 
		pub const Cypher_RETURN:i32=62; 
		pub const Cypher_DISTINCT:i32=63; 
		pub const Cypher_ORDER:i32=64; 
		pub const Cypher_BY:i32=65; 
		pub const Cypher_L_SKIP:i32=66; 
		pub const Cypher_LIMIT:i32=67; 
		pub const Cypher_ASCENDING:i32=68; 
		pub const Cypher_ASC:i32=69; 
		pub const Cypher_DESCENDING:i32=70; 
		pub const Cypher_DESC:i32=71; 
		pub const Cypher_WHERE:i32=72; 
		pub const Cypher_OR:i32=73; 
		pub const Cypher_XOR:i32=74; 
		pub const Cypher_AND:i32=75; 
		pub const Cypher_NOT:i32=76; 
		pub const Cypher_IN:i32=77; 
		pub const Cypher_STARTS:i32=78; 
		pub const Cypher_ENDS:i32=79; 
		pub const Cypher_CONTAINS:i32=80; 
		pub const Cypher_IS:i32=81; 
		pub const Cypher_NULL:i32=82; 
		pub const Cypher_COUNT:i32=83; 
		pub const Cypher_ANY:i32=84; 
		pub const Cypher_NONE:i32=85; 
		pub const Cypher_SINGLE:i32=86; 
		pub const Cypher_TRUE:i32=87; 
		pub const Cypher_FALSE:i32=88; 
		pub const Cypher_EXISTS:i32=89; 
		pub const Cypher_CASE:i32=90; 
		pub const Cypher_ELSE:i32=91; 
		pub const Cypher_END:i32=92; 
		pub const Cypher_WHEN:i32=93; 
		pub const Cypher_THEN:i32=94; 
		pub const Cypher_StringLiteral:i32=95; 
		pub const Cypher_EscapedChar:i32=96; 
		pub const Cypher_HexInteger:i32=97; 
		pub const Cypher_DecimalInteger:i32=98; 
		pub const Cypher_OctalInteger:i32=99; 
		pub const Cypher_HexLetter:i32=100; 
		pub const Cypher_HexDigit:i32=101; 
		pub const Cypher_Digit:i32=102; 
		pub const Cypher_NonZeroDigit:i32=103; 
		pub const Cypher_NonZeroOctDigit:i32=104; 
		pub const Cypher_OctDigit:i32=105; 
		pub const Cypher_ZeroDigit:i32=106; 
		pub const Cypher_ExponentDecimalReal:i32=107; 
		pub const Cypher_RegularDecimalReal:i32=108; 
		pub const Cypher_CONSTRAINT:i32=109; 
		pub const Cypher_DO:i32=110; 
		pub const Cypher_FOR:i32=111; 
		pub const Cypher_REQUIRE:i32=112; 
		pub const Cypher_UNIQUE:i32=113; 
		pub const Cypher_MANDATORY:i32=114; 
		pub const Cypher_SCALAR:i32=115; 
		pub const Cypher_OF:i32=116; 
		pub const Cypher_ADD:i32=117; 
		pub const Cypher_DROP:i32=118; 
		pub const Cypher_FILTER:i32=119; 
		pub const Cypher_EXTRACT:i32=120; 
		pub const Cypher_UnescapedSymbolicName:i32=121; 
		pub const Cypher_IdentifierStart:i32=122; 
		pub const Cypher_IdentifierPart:i32=123; 
		pub const Cypher_EscapedSymbolicName:i32=124; 
		pub const Cypher_SP:i32=125; 
		pub const Cypher_WHITESPACE:i32=126; 
		pub const Cypher_Comment:i32=127;
	pub const Cypher_EOF:i32=EOF;
	pub const RULE_oC_Cypher:usize = 0; 
	pub const RULE_oC_Statement:usize = 1; 
	pub const RULE_oC_Query:usize = 2; 
	pub const RULE_oC_RegularQuery:usize = 3; 
	pub const RULE_oC_Union:usize = 4; 
	pub const RULE_oC_SingleQuery:usize = 5; 
	pub const RULE_oC_SinglePartQuery:usize = 6; 
	pub const RULE_oC_MultiPartQuery:usize = 7; 
	pub const RULE_oC_UpdatingClause:usize = 8; 
	pub const RULE_oC_ReadingClause:usize = 9; 
	pub const RULE_oC_Match:usize = 10; 
	pub const RULE_oC_Unwind:usize = 11; 
	pub const RULE_oC_Merge:usize = 12; 
	pub const RULE_oC_MergeAction:usize = 13; 
	pub const RULE_oC_Create:usize = 14; 
	pub const RULE_oC_Set:usize = 15; 
	pub const RULE_oC_SetItem:usize = 16; 
	pub const RULE_oC_Delete:usize = 17; 
	pub const RULE_oC_Remove:usize = 18; 
	pub const RULE_oC_RemoveItem:usize = 19; 
	pub const RULE_oC_InQueryCall:usize = 20; 
	pub const RULE_oC_StandaloneCall:usize = 21; 
	pub const RULE_oC_YieldItems:usize = 22; 
	pub const RULE_oC_YieldItem:usize = 23; 
	pub const RULE_oC_With:usize = 24; 
	pub const RULE_oC_Return:usize = 25; 
	pub const RULE_oC_ProjectionBody:usize = 26; 
	pub const RULE_oC_ProjectionItems:usize = 27; 
	pub const RULE_oC_ProjectionItem:usize = 28; 
	pub const RULE_oC_Order:usize = 29; 
	pub const RULE_oC_Skip:usize = 30; 
	pub const RULE_oC_Limit:usize = 31; 
	pub const RULE_oC_SortItem:usize = 32; 
	pub const RULE_oC_Where:usize = 33; 
	pub const RULE_oC_Pattern:usize = 34; 
	pub const RULE_oC_PatternPart:usize = 35; 
	pub const RULE_oC_AnonymousPatternPart:usize = 36; 
	pub const RULE_oC_PatternElement:usize = 37; 
	pub const RULE_oC_NodePattern:usize = 38; 
	pub const RULE_oC_PatternElementChain:usize = 39; 
	pub const RULE_oC_RelationshipPattern:usize = 40; 
	pub const RULE_oC_RelationshipDetail:usize = 41; 
	pub const RULE_oC_Properties:usize = 42; 
	pub const RULE_oC_RelationshipTypes:usize = 43; 
	pub const RULE_oC_NodeLabels:usize = 44; 
	pub const RULE_oC_NodeLabel:usize = 45; 
	pub const RULE_oC_RangeLiteral:usize = 46; 
	pub const RULE_oC_LabelName:usize = 47; 
	pub const RULE_oC_RelTypeName:usize = 48; 
	pub const RULE_oC_Expression:usize = 49; 
	pub const RULE_oC_OrExpression:usize = 50; 
	pub const RULE_oC_XorExpression:usize = 51; 
	pub const RULE_oC_AndExpression:usize = 52; 
	pub const RULE_oC_NotExpression:usize = 53; 
	pub const RULE_oC_ComparisonExpression:usize = 54; 
	pub const RULE_oC_AddOrSubtractExpression:usize = 55; 
	pub const RULE_oC_MultiplyDivideModuloExpression:usize = 56; 
	pub const RULE_oC_PowerOfExpression:usize = 57; 
	pub const RULE_oC_UnaryAddOrSubtractExpression:usize = 58; 
	pub const RULE_oC_StringListNullOperatorExpression:usize = 59; 
	pub const RULE_oC_ListOperatorExpression:usize = 60; 
	pub const RULE_oC_StringOperatorExpression:usize = 61; 
	pub const RULE_oC_NullOperatorExpression:usize = 62; 
	pub const RULE_oC_PropertyOrLabelsExpression:usize = 63; 
	pub const RULE_oC_Atom:usize = 64; 
	pub const RULE_oC_Literal:usize = 65; 
	pub const RULE_oC_BooleanLiteral:usize = 66; 
	pub const RULE_oC_ListLiteral:usize = 67; 
	pub const RULE_oC_PartialComparisonExpression:usize = 68; 
	pub const RULE_oC_ParenthesizedExpression:usize = 69; 
	pub const RULE_oC_RelationshipsPattern:usize = 70; 
	pub const RULE_oC_FilterExpression:usize = 71; 
	pub const RULE_oC_IdInColl:usize = 72; 
	pub const RULE_oC_FunctionInvocation:usize = 73; 
	pub const RULE_oC_FunctionName:usize = 74; 
	pub const RULE_oC_ExplicitProcedureInvocation:usize = 75; 
	pub const RULE_oC_ImplicitProcedureInvocation:usize = 76; 
	pub const RULE_oC_ProcedureResultField:usize = 77; 
	pub const RULE_oC_ProcedureName:usize = 78; 
	pub const RULE_oC_Namespace:usize = 79; 
	pub const RULE_oC_ListComprehension:usize = 80; 
	pub const RULE_oC_PatternComprehension:usize = 81; 
	pub const RULE_oC_PropertyLookup:usize = 82; 
	pub const RULE_oC_CaseExpression:usize = 83; 
	pub const RULE_oC_CaseAlternatives:usize = 84; 
	pub const RULE_oC_Variable:usize = 85; 
	pub const RULE_oC_NumberLiteral:usize = 86; 
	pub const RULE_oC_MapLiteral:usize = 87; 
	pub const RULE_oC_Parameter:usize = 88; 
	pub const RULE_oC_PropertyExpression:usize = 89; 
	pub const RULE_oC_PropertyKeyName:usize = 90; 
	pub const RULE_oC_IntegerLiteral:usize = 91; 
	pub const RULE_oC_DoubleLiteral:usize = 92; 
	pub const RULE_oC_SchemaName:usize = 93; 
	pub const RULE_oC_ReservedWord:usize = 94; 
	pub const RULE_oC_SymbolicName:usize = 95; 
	pub const RULE_oC_LeftArrowHead:usize = 96; 
	pub const RULE_oC_RightArrowHead:usize = 97; 
	pub const RULE_oC_Dash:usize = 98;
	pub const ruleNames: [&'static str; 99] =  [
		"oC_Cypher", "oC_Statement", "oC_Query", "oC_RegularQuery", "oC_Union", 
		"oC_SingleQuery", "oC_SinglePartQuery", "oC_MultiPartQuery", "oC_UpdatingClause", 
		"oC_ReadingClause", "oC_Match", "oC_Unwind", "oC_Merge", "oC_MergeAction", 
		"oC_Create", "oC_Set", "oC_SetItem", "oC_Delete", "oC_Remove", "oC_RemoveItem", 
		"oC_InQueryCall", "oC_StandaloneCall", "oC_YieldItems", "oC_YieldItem", 
		"oC_With", "oC_Return", "oC_ProjectionBody", "oC_ProjectionItems", "oC_ProjectionItem", 
		"oC_Order", "oC_Skip", "oC_Limit", "oC_SortItem", "oC_Where", "oC_Pattern", 
		"oC_PatternPart", "oC_AnonymousPatternPart", "oC_PatternElement", "oC_NodePattern", 
		"oC_PatternElementChain", "oC_RelationshipPattern", "oC_RelationshipDetail", 
		"oC_Properties", "oC_RelationshipTypes", "oC_NodeLabels", "oC_NodeLabel", 
		"oC_RangeLiteral", "oC_LabelName", "oC_RelTypeName", "oC_Expression", 
		"oC_OrExpression", "oC_XorExpression", "oC_AndExpression", "oC_NotExpression", 
		"oC_ComparisonExpression", "oC_AddOrSubtractExpression", "oC_MultiplyDivideModuloExpression", 
		"oC_PowerOfExpression", "oC_UnaryAddOrSubtractExpression", "oC_StringListNullOperatorExpression", 
		"oC_ListOperatorExpression", "oC_StringOperatorExpression", "oC_NullOperatorExpression", 
		"oC_PropertyOrLabelsExpression", "oC_Atom", "oC_Literal", "oC_BooleanLiteral", 
		"oC_ListLiteral", "oC_PartialComparisonExpression", "oC_ParenthesizedExpression", 
		"oC_RelationshipsPattern", "oC_FilterExpression", "oC_IdInColl", "oC_FunctionInvocation", 
		"oC_FunctionName", "oC_ExplicitProcedureInvocation", "oC_ImplicitProcedureInvocation", 
		"oC_ProcedureResultField", "oC_ProcedureName", "oC_Namespace", "oC_ListComprehension", 
		"oC_PatternComprehension", "oC_PropertyLookup", "oC_CaseExpression", "oC_CaseAlternatives", 
		"oC_Variable", "oC_NumberLiteral", "oC_MapLiteral", "oC_Parameter", "oC_PropertyExpression", 
		"oC_PropertyKeyName", "oC_IntegerLiteral", "oC_DoubleLiteral", "oC_SchemaName", 
		"oC_ReservedWord", "oC_SymbolicName", "oC_LeftArrowHead", "oC_RightArrowHead", 
		"oC_Dash"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;107] = [
		None, Some("';'"), Some("','"), Some("'='"), Some("'+='"), Some("'*'"), 
		Some("'('"), Some("')'"), Some("'['"), Some("']'"), Some("':'"), Some("'|'"), 
		Some("'..'"), Some("'+'"), Some("'-'"), Some("'/'"), Some("'%'"), Some("'^'"), 
		Some("'<>'"), Some("'<'"), Some("'>'"), Some("'<='"), Some("'>='"), Some("'.'"), 
		Some("'{'"), Some("'}'"), Some("'$'"), Some("'\\u27E8'"), Some("'\\u3008'"), 
		Some("'\\uFE64'"), Some("'\\uFF1C'"), Some("'\\u27E9'"), Some("'\\u3009'"), 
		Some("'\\uFE65'"), Some("'\\uFF1E'"), Some("'\\u00AD'"), Some("'\\u2010'"), 
		Some("'\\u2011'"), Some("'\\u2012'"), Some("'\\u2013'"), Some("'\\u2014'"), 
		Some("'\\u2015'"), Some("'\\u2212'"), Some("'\\uFE58'"), Some("'\\uFE63'"), 
		Some("'\\uFF0D'"), None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, Some("'0'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;128]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, Some("UNION"), 
		Some("ALL"), Some("OPTIONAL"), Some("MATCH"), Some("UNWIND"), Some("AS"), 
		Some("MERGE"), Some("ON"), Some("CREATE"), Some("SET"), Some("DETACH"), 
		Some("DELETE"), Some("REMOVE"), Some("CALL"), Some("YIELD"), Some("WITH"), 
		Some("RETURN"), Some("DISTINCT"), Some("ORDER"), Some("BY"), Some("L_SKIP"), 
		Some("LIMIT"), Some("ASCENDING"), Some("ASC"), Some("DESCENDING"), Some("DESC"), 
		Some("WHERE"), Some("OR"), Some("XOR"), Some("AND"), Some("NOT"), Some("IN"), 
		Some("STARTS"), Some("ENDS"), Some("CONTAINS"), Some("IS"), Some("NULL"), 
		Some("COUNT"), Some("ANY"), Some("NONE"), Some("SINGLE"), Some("TRUE"), 
		Some("FALSE"), Some("EXISTS"), Some("CASE"), Some("ELSE"), Some("END"), 
		Some("WHEN"), Some("THEN"), Some("StringLiteral"), Some("EscapedChar"), 
		Some("HexInteger"), Some("DecimalInteger"), Some("OctalInteger"), Some("HexLetter"), 
		Some("HexDigit"), Some("Digit"), Some("NonZeroDigit"), Some("NonZeroOctDigit"), 
		Some("OctDigit"), Some("ZeroDigit"), Some("ExponentDecimalReal"), Some("RegularDecimalReal"), 
		Some("CONSTRAINT"), Some("DO"), Some("FOR"), Some("REQUIRE"), Some("UNIQUE"), 
		Some("MANDATORY"), Some("SCALAR"), Some("OF"), Some("ADD"), Some("DROP"), 
		Some("FILTER"), Some("EXTRACT"), Some("UnescapedSymbolicName"), Some("IdentifierStart"), 
		Some("IdentifierPart"), Some("EscapedSymbolicName"), Some("SP"), Some("WHITESPACE"), 
		Some("Comment")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,CypherParserExt<'input>, I, CypherParserContextType , dyn CypherListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type CypherTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, CypherParserContextType , dyn CypherListener<'input> + 'a>;

/// Parser for Cypher grammar
pub struct CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				CypherParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for CypherParser
pub trait CypherParserContext<'input>:
	for<'x> Listenable<dyn CypherListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=CypherParserContextType>
{}

antlr4rust::coerce_from!{ 'input : CypherParserContext<'input> }

impl<'input> CypherParserContext<'input> for TerminalNode<'input,CypherParserContextType> {}
impl<'input> CypherParserContext<'input> for ErrorNode<'input,CypherParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn CypherParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn CypherListener<'input> + 'input }

pub struct CypherParserContextType;
antlr4rust::tid!{CypherParserContextType}

impl<'input> ParserNodeType<'input> for CypherParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn CypherParserContext<'input> + 'input;
}

impl<'input, I> Deref for CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CypherParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> CypherParserExt<'input>{
}
antlr4rust::tid! { CypherParserExt<'a> }

impl<'input> TokenAware<'input> for CypherParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for CypherParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for CypherParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Cypher.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- oC_Cypher ----------------
pub type OC_CypherContextAll<'input> = OC_CypherContext<'input>;


pub type OC_CypherContext<'input> = BaseParserRuleContext<'input,OC_CypherContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CypherContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_CypherContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CypherContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Cypher(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Cypher(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_CypherContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Cypher }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Cypher }
}
antlr4rust::tid!{OC_CypherContextExt<'a>}

impl<'input> OC_CypherContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_CypherContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_CypherContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_CypherContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_CypherContextExt<'input>>{

fn oC_Statement(&self) -> Option<Rc<OC_StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_CypherContextAttrs<'input> for OC_CypherContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Cypher(&mut self,)
	-> Result<Rc<OC_CypherContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_CypherContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_oC_Cypher);
        let mut _localctx: Rc<OC_CypherContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(199);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(198);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Statement*/
			recog.base.set_state(201);
			recog.oC_Statement()?;

			recog.base.set_state(206);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(203);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(202);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(205);
					recog.base.match_token(Cypher_T__0,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(208);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(211);
			recog.base.match_token(Cypher_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Statement ----------------
pub type OC_StatementContextAll<'input> = OC_StatementContext<'input>;


pub type OC_StatementContext<'input> = BaseParserRuleContext<'input,OC_StatementContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_StatementContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Statement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Statement }
}
antlr4rust::tid!{OC_StatementContextExt<'a>}

impl<'input> OC_StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_StatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_StatementContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_StatementContextExt<'input>>{

fn oC_Query(&self) -> Option<Rc<OC_QueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_StatementContextAttrs<'input> for OC_StatementContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Statement(&mut self,)
	-> Result<Rc<OC_StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_oC_Statement);
        let mut _localctx: Rc<OC_StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Query*/
			recog.base.set_state(213);
			recog.oC_Query()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Query ----------------
pub type OC_QueryContextAll<'input> = OC_QueryContext<'input>;


pub type OC_QueryContext<'input> = BaseParserRuleContext<'input,OC_QueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_QueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_QueryContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_QueryContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Query(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Query(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_QueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Query }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Query }
}
antlr4rust::tid!{OC_QueryContextExt<'a>}

impl<'input> OC_QueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_QueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_QueryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_QueryContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_QueryContextExt<'input>>{

fn oC_RegularQuery(&self) -> Option<Rc<OC_RegularQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_StandaloneCall(&self) -> Option<Rc<OC_StandaloneCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_QueryContextAttrs<'input> for OC_QueryContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Query(&mut self,)
	-> Result<Rc<OC_QueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_QueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_oC_Query);
        let mut _localctx: Rc<OC_QueryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_RegularQuery*/
					recog.base.set_state(215);
					recog.oC_RegularQuery()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_StandaloneCall*/
					recog.base.set_state(216);
					recog.oC_StandaloneCall()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RegularQuery ----------------
pub type OC_RegularQueryContextAll<'input> = OC_RegularQueryContext<'input>;


pub type OC_RegularQueryContext<'input> = BaseParserRuleContext<'input,OC_RegularQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RegularQueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RegularQueryContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RegularQueryContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RegularQuery(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RegularQuery(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RegularQueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RegularQuery }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RegularQuery }
}
antlr4rust::tid!{OC_RegularQueryContextExt<'a>}

impl<'input> OC_RegularQueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RegularQueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RegularQueryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RegularQueryContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RegularQueryContextExt<'input>>{

fn oC_SingleQuery(&self) -> Option<Rc<OC_SingleQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Union_all(&self) ->  Vec<Rc<OC_UnionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Union(&self, i: usize) -> Option<Rc<OC_UnionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_RegularQueryContextAttrs<'input> for OC_RegularQueryContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RegularQuery(&mut self,)
	-> Result<Rc<OC_RegularQueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RegularQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_oC_RegularQuery);
        let mut _localctx: Rc<OC_RegularQueryContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SingleQuery*/
			recog.base.set_state(219);
			recog.oC_SingleQuery()?;

			recog.base.set_state(226);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(221);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(220);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Union*/
					recog.base.set_state(223);
					recog.oC_Union()?;

					}
					} 
				}
				recog.base.set_state(228);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Union ----------------
pub type OC_UnionContextAll<'input> = OC_UnionContext<'input>;


pub type OC_UnionContext<'input> = BaseParserRuleContext<'input,OC_UnionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_UnionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UnionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Union(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Union(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_UnionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Union }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Union }
}
antlr4rust::tid!{OC_UnionContextExt<'a>}

impl<'input> OC_UnionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_UnionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_UnionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_UnionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_UnionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNION
/// Returns `None` if there is no child corresponding to token UNION
fn UNION(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UNION, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token ALL
/// Returns `None` if there is no child corresponding to token ALL
fn ALL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ALL, 0)
}
fn oC_SingleQuery(&self) -> Option<Rc<OC_SingleQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_UnionContextAttrs<'input> for OC_UnionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Union(&mut self,)
	-> Result<Rc<OC_UnionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_UnionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_oC_Union);
        let mut _localctx: Rc<OC_UnionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(241);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(229);
					recog.base.match_token(Cypher_UNION,&mut recog.err_handler)?;

					recog.base.set_state(230);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(231);
					recog.base.match_token(Cypher_ALL,&mut recog.err_handler)?;

					recog.base.set_state(233);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(232);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_SingleQuery*/
					recog.base.set_state(235);
					recog.oC_SingleQuery()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(236);
					recog.base.match_token(Cypher_UNION,&mut recog.err_handler)?;

					recog.base.set_state(238);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(237);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_SingleQuery*/
					recog.base.set_state(240);
					recog.oC_SingleQuery()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SingleQuery ----------------
pub type OC_SingleQueryContextAll<'input> = OC_SingleQueryContext<'input>;


pub type OC_SingleQueryContext<'input> = BaseParserRuleContext<'input,OC_SingleQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SingleQueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SingleQueryContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SingleQueryContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SingleQuery(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SingleQuery(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SingleQueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SingleQuery }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SingleQuery }
}
antlr4rust::tid!{OC_SingleQueryContextExt<'a>}

impl<'input> OC_SingleQueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SingleQueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SingleQueryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SingleQueryContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SingleQueryContextExt<'input>>{

fn oC_SinglePartQuery(&self) -> Option<Rc<OC_SinglePartQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_MultiPartQuery(&self) -> Option<Rc<OC_MultiPartQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_SingleQueryContextAttrs<'input> for OC_SingleQueryContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SingleQuery(&mut self,)
	-> Result<Rc<OC_SingleQueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SingleQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_oC_SingleQuery);
        let mut _localctx: Rc<OC_SingleQueryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(245);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_SinglePartQuery*/
					recog.base.set_state(243);
					recog.oC_SinglePartQuery()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_MultiPartQuery*/
					recog.base.set_state(244);
					recog.oC_MultiPartQuery()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SinglePartQuery ----------------
pub type OC_SinglePartQueryContextAll<'input> = OC_SinglePartQueryContext<'input>;


pub type OC_SinglePartQueryContext<'input> = BaseParserRuleContext<'input,OC_SinglePartQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SinglePartQueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SinglePartQueryContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SinglePartQueryContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SinglePartQuery(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SinglePartQuery(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SinglePartQueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SinglePartQuery }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SinglePartQuery }
}
antlr4rust::tid!{OC_SinglePartQueryContextExt<'a>}

impl<'input> OC_SinglePartQueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SinglePartQueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SinglePartQueryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SinglePartQueryContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SinglePartQueryContextExt<'input>>{

fn oC_Return(&self) -> Option<Rc<OC_ReturnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ReadingClause_all(&self) ->  Vec<Rc<OC_ReadingClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_ReadingClause(&self, i: usize) -> Option<Rc<OC_ReadingClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_UpdatingClause_all(&self) ->  Vec<Rc<OC_UpdatingClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_UpdatingClause(&self, i: usize) -> Option<Rc<OC_UpdatingClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_SinglePartQueryContextAttrs<'input> for OC_SinglePartQueryContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SinglePartQuery(&mut self,)
	-> Result<Rc<OC_SinglePartQueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SinglePartQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_oC_SinglePartQuery);
        let mut _localctx: Rc<OC_SinglePartQueryContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(253);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 48)) & !0x3f) == 0 && ((1usize << (_la - 48)) & 2055) != 0) {
						{
						{
						/*InvokeRule oC_ReadingClause*/
						recog.base.set_state(247);
						recog.oC_ReadingClause()?;

						recog.base.set_state(249);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==Cypher_SP {
							{
							recog.base.set_state(248);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(255);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule oC_Return*/
					recog.base.set_state(256);
					recog.oC_Return()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(263);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 48)) & !0x3f) == 0 && ((1usize << (_la - 48)) & 2055) != 0) {
						{
						{
						/*InvokeRule oC_ReadingClause*/
						recog.base.set_state(257);
						recog.oC_ReadingClause()?;

						recog.base.set_state(259);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==Cypher_SP {
							{
							recog.base.set_state(258);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(265);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule oC_UpdatingClause*/
					recog.base.set_state(266);
					recog.oC_UpdatingClause()?;

					recog.base.set_state(273);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(268);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(267);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_UpdatingClause*/
							recog.base.set_state(270);
							recog.oC_UpdatingClause()?;

							}
							} 
						}
						recog.base.set_state(275);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
					}
					recog.base.set_state(280);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(277);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(276);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_Return*/
							recog.base.set_state(279);
							recog.oC_Return()?;

							}
						}

						_ => {}
					}
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_MultiPartQuery ----------------
pub type OC_MultiPartQueryContextAll<'input> = OC_MultiPartQueryContext<'input>;


pub type OC_MultiPartQueryContext<'input> = BaseParserRuleContext<'input,OC_MultiPartQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MultiPartQueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MultiPartQueryContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MultiPartQueryContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_MultiPartQuery(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_MultiPartQuery(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MultiPartQueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_MultiPartQuery }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_MultiPartQuery }
}
antlr4rust::tid!{OC_MultiPartQueryContextExt<'a>}

impl<'input> OC_MultiPartQueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MultiPartQueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MultiPartQueryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MultiPartQueryContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MultiPartQueryContextExt<'input>>{

fn oC_SinglePartQuery(&self) -> Option<Rc<OC_SinglePartQueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_With_all(&self) ->  Vec<Rc<OC_WithContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_With(&self, i: usize) -> Option<Rc<OC_WithContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_ReadingClause_all(&self) ->  Vec<Rc<OC_ReadingClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_ReadingClause(&self, i: usize) -> Option<Rc<OC_ReadingClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_UpdatingClause_all(&self) ->  Vec<Rc<OC_UpdatingClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_UpdatingClause(&self, i: usize) -> Option<Rc<OC_UpdatingClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_MultiPartQueryContextAttrs<'input> for OC_MultiPartQueryContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_MultiPartQuery(&mut self,)
	-> Result<Rc<OC_MultiPartQueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MultiPartQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_oC_MultiPartQuery);
        let mut _localctx: Rc<OC_MultiPartQueryContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(306); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(290);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 48)) & !0x3f) == 0 && ((1usize << (_la - 48)) & 2055) != 0) {
						{
						{
						/*InvokeRule oC_ReadingClause*/
						recog.base.set_state(284);
						recog.oC_ReadingClause()?;

						recog.base.set_state(286);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==Cypher_SP {
							{
							recog.base.set_state(285);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(292);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(299);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 52)) & !0x3f) == 0 && ((1usize << (_la - 52)) & 125) != 0) {
						{
						{
						/*InvokeRule oC_UpdatingClause*/
						recog.base.set_state(293);
						recog.oC_UpdatingClause()?;

						recog.base.set_state(295);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==Cypher_SP {
							{
							recog.base.set_state(294);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(301);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule oC_With*/
					recog.base.set_state(302);
					recog.oC_With()?;

					recog.base.set_state(304);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(303);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(308); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			/*InvokeRule oC_SinglePartQuery*/
			recog.base.set_state(310);
			recog.oC_SinglePartQuery()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_UpdatingClause ----------------
pub type OC_UpdatingClauseContextAll<'input> = OC_UpdatingClauseContext<'input>;


pub type OC_UpdatingClauseContext<'input> = BaseParserRuleContext<'input,OC_UpdatingClauseContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UpdatingClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_UpdatingClauseContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UpdatingClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_UpdatingClause(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_UpdatingClause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_UpdatingClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_UpdatingClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_UpdatingClause }
}
antlr4rust::tid!{OC_UpdatingClauseContextExt<'a>}

impl<'input> OC_UpdatingClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_UpdatingClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_UpdatingClauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_UpdatingClauseContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_UpdatingClauseContextExt<'input>>{

fn oC_Create(&self) -> Option<Rc<OC_CreateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Merge(&self) -> Option<Rc<OC_MergeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Delete(&self) -> Option<Rc<OC_DeleteContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Set(&self) -> Option<Rc<OC_SetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Remove(&self) -> Option<Rc<OC_RemoveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_UpdatingClauseContextAttrs<'input> for OC_UpdatingClauseContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_UpdatingClause(&mut self,)
	-> Result<Rc<OC_UpdatingClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_UpdatingClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_oC_UpdatingClause);
        let mut _localctx: Rc<OC_UpdatingClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(317);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_CREATE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_Create*/
					recog.base.set_state(312);
					recog.oC_Create()?;

					}
				}

			Cypher_MERGE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_Merge*/
					recog.base.set_state(313);
					recog.oC_Merge()?;

					}
				}

			Cypher_DETACH |Cypher_DELETE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule oC_Delete*/
					recog.base.set_state(314);
					recog.oC_Delete()?;

					}
				}

			Cypher_SET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule oC_Set*/
					recog.base.set_state(315);
					recog.oC_Set()?;

					}
				}

			Cypher_REMOVE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule oC_Remove*/
					recog.base.set_state(316);
					recog.oC_Remove()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ReadingClause ----------------
pub type OC_ReadingClauseContextAll<'input> = OC_ReadingClauseContext<'input>;


pub type OC_ReadingClauseContext<'input> = BaseParserRuleContext<'input,OC_ReadingClauseContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReadingClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ReadingClauseContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReadingClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ReadingClause(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ReadingClause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ReadingClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ReadingClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ReadingClause }
}
antlr4rust::tid!{OC_ReadingClauseContextExt<'a>}

impl<'input> OC_ReadingClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ReadingClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ReadingClauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ReadingClauseContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ReadingClauseContextExt<'input>>{

fn oC_Match(&self) -> Option<Rc<OC_MatchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Unwind(&self) -> Option<Rc<OC_UnwindContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_InQueryCall(&self) -> Option<Rc<OC_InQueryCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ReadingClauseContextAttrs<'input> for OC_ReadingClauseContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ReadingClause(&mut self,)
	-> Result<Rc<OC_ReadingClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ReadingClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_oC_ReadingClause);
        let mut _localctx: Rc<OC_ReadingClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(322);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_OPTIONAL |Cypher_MATCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_Match*/
					recog.base.set_state(319);
					recog.oC_Match()?;

					}
				}

			Cypher_UNWIND 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_Unwind*/
					recog.base.set_state(320);
					recog.oC_Unwind()?;

					}
				}

			Cypher_CALL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule oC_InQueryCall*/
					recog.base.set_state(321);
					recog.oC_InQueryCall()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Match ----------------
pub type OC_MatchContextAll<'input> = OC_MatchContext<'input>;


pub type OC_MatchContext<'input> = BaseParserRuleContext<'input,OC_MatchContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MatchContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MatchContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MatchContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Match(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Match(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MatchContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Match }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Match }
}
antlr4rust::tid!{OC_MatchContextExt<'a>}

impl<'input> OC_MatchContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MatchContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MatchContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MatchContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MatchContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MATCH, 0)
}
fn oC_Pattern(&self) -> Option<Rc<OC_PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONAL
/// Returns `None` if there is no child corresponding to token OPTIONAL
fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OPTIONAL, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_MatchContextAttrs<'input> for OC_MatchContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Match(&mut self,)
	-> Result<Rc<OC_MatchContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MatchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_oC_Match);
        let mut _localctx: Rc<OC_MatchContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(326);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_OPTIONAL {
				{
				recog.base.set_state(324);
				recog.base.match_token(Cypher_OPTIONAL,&mut recog.err_handler)?;

				recog.base.set_state(325);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(328);
			recog.base.match_token(Cypher_MATCH,&mut recog.err_handler)?;

			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(329);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Pattern*/
			recog.base.set_state(332);
			recog.oC_Pattern()?;

			recog.base.set_state(337);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(334);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(333);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Where*/
					recog.base.set_state(336);
					recog.oC_Where()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Unwind ----------------
pub type OC_UnwindContextAll<'input> = OC_UnwindContext<'input>;


pub type OC_UnwindContext<'input> = BaseParserRuleContext<'input,OC_UnwindContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnwindContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_UnwindContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UnwindContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Unwind(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Unwind(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_UnwindContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Unwind }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Unwind }
}
antlr4rust::tid!{OC_UnwindContextExt<'a>}

impl<'input> OC_UnwindContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_UnwindContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_UnwindContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_UnwindContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_UnwindContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNWIND
/// Returns `None` if there is no child corresponding to token UNWIND
fn UNWIND(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UNWIND, 0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AS, 0)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_UnwindContextAttrs<'input> for OC_UnwindContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Unwind(&mut self,)
	-> Result<Rc<OC_UnwindContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_UnwindContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_oC_Unwind);
        let mut _localctx: Rc<OC_UnwindContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(339);
			recog.base.match_token(Cypher_UNWIND,&mut recog.err_handler)?;

			recog.base.set_state(341);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(340);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(343);
			recog.oC_Expression()?;

			recog.base.set_state(344);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			recog.base.set_state(345);
			recog.base.match_token(Cypher_AS,&mut recog.err_handler)?;

			recog.base.set_state(346);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_Variable*/
			recog.base.set_state(347);
			recog.oC_Variable()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Merge ----------------
pub type OC_MergeContextAll<'input> = OC_MergeContext<'input>;


pub type OC_MergeContext<'input> = BaseParserRuleContext<'input,OC_MergeContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MergeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MergeContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MergeContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Merge(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Merge(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MergeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Merge }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Merge }
}
antlr4rust::tid!{OC_MergeContextExt<'a>}

impl<'input> OC_MergeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MergeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MergeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MergeContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MergeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MERGE
/// Returns `None` if there is no child corresponding to token MERGE
fn MERGE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MERGE, 0)
}
fn oC_PatternPart(&self) -> Option<Rc<OC_PatternPartContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_MergeAction_all(&self) ->  Vec<Rc<OC_MergeActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_MergeAction(&self, i: usize) -> Option<Rc<OC_MergeActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_MergeContextAttrs<'input> for OC_MergeContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Merge(&mut self,)
	-> Result<Rc<OC_MergeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MergeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_oC_Merge);
        let mut _localctx: Rc<OC_MergeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(349);
			recog.base.match_token(Cypher_MERGE,&mut recog.err_handler)?;

			recog.base.set_state(351);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(350);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_PatternPart*/
			recog.base.set_state(353);
			recog.oC_PatternPart()?;

			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(354);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_MergeAction*/
					recog.base.set_state(355);
					recog.oC_MergeAction()?;

					}
					} 
				}
				recog.base.set_state(360);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_MergeAction ----------------
pub type OC_MergeActionContextAll<'input> = OC_MergeActionContext<'input>;


pub type OC_MergeActionContext<'input> = BaseParserRuleContext<'input,OC_MergeActionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MergeActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MergeActionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MergeActionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_MergeAction(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_MergeAction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MergeActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_MergeAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_MergeAction }
}
antlr4rust::tid!{OC_MergeActionContextExt<'a>}

impl<'input> OC_MergeActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MergeActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MergeActionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MergeActionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MergeActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ON
/// Returns `None` if there is no child corresponding to token ON
fn ON(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ON, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MATCH, 0)
}
fn oC_Set(&self) -> Option<Rc<OC_SetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CREATE
/// Returns `None` if there is no child corresponding to token CREATE
fn CREATE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CREATE, 0)
}

}

impl<'input> OC_MergeActionContextAttrs<'input> for OC_MergeActionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_MergeAction(&mut self,)
	-> Result<Rc<OC_MergeActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MergeActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_oC_MergeAction);
        let mut _localctx: Rc<OC_MergeActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(371);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(361);
					recog.base.match_token(Cypher_ON,&mut recog.err_handler)?;

					recog.base.set_state(362);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(363);
					recog.base.match_token(Cypher_MATCH,&mut recog.err_handler)?;

					recog.base.set_state(364);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Set*/
					recog.base.set_state(365);
					recog.oC_Set()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(366);
					recog.base.match_token(Cypher_ON,&mut recog.err_handler)?;

					recog.base.set_state(367);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(368);
					recog.base.match_token(Cypher_CREATE,&mut recog.err_handler)?;

					recog.base.set_state(369);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Set*/
					recog.base.set_state(370);
					recog.oC_Set()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Create ----------------
pub type OC_CreateContextAll<'input> = OC_CreateContext<'input>;


pub type OC_CreateContext<'input> = BaseParserRuleContext<'input,OC_CreateContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CreateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_CreateContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CreateContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Create(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Create(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_CreateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Create }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Create }
}
antlr4rust::tid!{OC_CreateContextExt<'a>}

impl<'input> OC_CreateContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_CreateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_CreateContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_CreateContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_CreateContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CREATE
/// Returns `None` if there is no child corresponding to token CREATE
fn CREATE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CREATE, 0)
}
fn oC_Pattern(&self) -> Option<Rc<OC_PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_CreateContextAttrs<'input> for OC_CreateContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Create(&mut self,)
	-> Result<Rc<OC_CreateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_CreateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_oC_Create);
        let mut _localctx: Rc<OC_CreateContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(373);
			recog.base.match_token(Cypher_CREATE,&mut recog.err_handler)?;

			recog.base.set_state(375);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(374);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Pattern*/
			recog.base.set_state(377);
			recog.oC_Pattern()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Set ----------------
pub type OC_SetContextAll<'input> = OC_SetContext<'input>;


pub type OC_SetContext<'input> = BaseParserRuleContext<'input,OC_SetContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SetContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SetContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Set(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Set(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Set }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Set }
}
antlr4rust::tid!{OC_SetContextExt<'a>}

impl<'input> OC_SetContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SetContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SetContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SET
/// Returns `None` if there is no child corresponding to token SET
fn SET(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SET, 0)
}
fn oC_SetItem_all(&self) ->  Vec<Rc<OC_SetItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_SetItem(&self, i: usize) -> Option<Rc<OC_SetItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_SetContextAttrs<'input> for OC_SetContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Set(&mut self,)
	-> Result<Rc<OC_SetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_oC_Set);
        let mut _localctx: Rc<OC_SetContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(379);
			recog.base.match_token(Cypher_SET,&mut recog.err_handler)?;

			recog.base.set_state(381);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(380);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_SetItem*/
			recog.base.set_state(383);
			recog.oC_SetItem()?;

			recog.base.set_state(388);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Cypher_T__1 {
				{
				{
				recog.base.set_state(384);
				recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

				/*InvokeRule oC_SetItem*/
				recog.base.set_state(385);
				recog.oC_SetItem()?;

				}
				}
				recog.base.set_state(390);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SetItem ----------------
pub type OC_SetItemContextAll<'input> = OC_SetItemContext<'input>;


pub type OC_SetItemContext<'input> = BaseParserRuleContext<'input,OC_SetItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SetItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SetItemContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SetItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SetItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SetItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SetItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SetItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SetItem }
}
antlr4rust::tid!{OC_SetItemContextExt<'a>}

impl<'input> OC_SetItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SetItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SetItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SetItemContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SetItemContextExt<'input>>{

fn oC_PropertyExpression(&self) -> Option<Rc<OC_PropertyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_SetItemContextAttrs<'input> for OC_SetItemContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SetItem(&mut self,)
	-> Result<Rc<OC_SetItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SetItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_oC_SetItem);
        let mut _localctx: Rc<OC_SetItemContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(427);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_PropertyExpression*/
					recog.base.set_state(391);
					recog.oC_PropertyExpression()?;

					recog.base.set_state(393);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(392);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(395);
					recog.base.match_token(Cypher_T__2,&mut recog.err_handler)?;

					recog.base.set_state(397);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(396);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(399);
					recog.oC_Expression()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(401);
					recog.oC_Variable()?;

					recog.base.set_state(403);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(402);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(405);
					recog.base.match_token(Cypher_T__2,&mut recog.err_handler)?;

					recog.base.set_state(407);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(406);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(409);
					recog.oC_Expression()?;

					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(411);
					recog.oC_Variable()?;

					recog.base.set_state(413);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(412);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(415);
					recog.base.match_token(Cypher_T__3,&mut recog.err_handler)?;

					recog.base.set_state(417);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(416);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(419);
					recog.oC_Expression()?;

					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(421);
					recog.oC_Variable()?;

					recog.base.set_state(423);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(422);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_NodeLabels*/
					recog.base.set_state(425);
					recog.oC_NodeLabels()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Delete ----------------
pub type OC_DeleteContextAll<'input> = OC_DeleteContext<'input>;


pub type OC_DeleteContext<'input> = BaseParserRuleContext<'input,OC_DeleteContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DeleteContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_DeleteContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DeleteContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Delete(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Delete(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_DeleteContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Delete }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Delete }
}
antlr4rust::tid!{OC_DeleteContextExt<'a>}

impl<'input> OC_DeleteContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_DeleteContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_DeleteContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_DeleteContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_DeleteContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DELETE
/// Returns `None` if there is no child corresponding to token DELETE
fn DELETE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DELETE, 0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DETACH
/// Returns `None` if there is no child corresponding to token DETACH
fn DETACH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DETACH, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_DeleteContextAttrs<'input> for OC_DeleteContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Delete(&mut self,)
	-> Result<Rc<OC_DeleteContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_DeleteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_oC_Delete);
        let mut _localctx: Rc<OC_DeleteContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(431);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_DETACH {
				{
				recog.base.set_state(429);
				recog.base.match_token(Cypher_DETACH,&mut recog.err_handler)?;

				recog.base.set_state(430);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(433);
			recog.base.match_token(Cypher_DELETE,&mut recog.err_handler)?;

			recog.base.set_state(435);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(434);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(437);
			recog.oC_Expression()?;

			recog.base.set_state(448);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(439);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(438);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(441);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(443);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(442);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(445);
					recog.oC_Expression()?;

					}
					} 
				}
				recog.base.set_state(450);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Remove ----------------
pub type OC_RemoveContextAll<'input> = OC_RemoveContext<'input>;


pub type OC_RemoveContext<'input> = BaseParserRuleContext<'input,OC_RemoveContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RemoveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RemoveContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RemoveContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Remove(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Remove(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RemoveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Remove }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Remove }
}
antlr4rust::tid!{OC_RemoveContextExt<'a>}

impl<'input> OC_RemoveContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RemoveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RemoveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RemoveContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RemoveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REMOVE
/// Returns `None` if there is no child corresponding to token REMOVE
fn REMOVE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_REMOVE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_RemoveItem_all(&self) ->  Vec<Rc<OC_RemoveItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_RemoveItem(&self, i: usize) -> Option<Rc<OC_RemoveItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_RemoveContextAttrs<'input> for OC_RemoveContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Remove(&mut self,)
	-> Result<Rc<OC_RemoveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RemoveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_oC_Remove);
        let mut _localctx: Rc<OC_RemoveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(451);
			recog.base.match_token(Cypher_REMOVE,&mut recog.err_handler)?;

			recog.base.set_state(452);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_RemoveItem*/
			recog.base.set_state(453);
			recog.oC_RemoveItem()?;

			recog.base.set_state(464);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(455);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(454);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(457);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(459);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(458);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_RemoveItem*/
					recog.base.set_state(461);
					recog.oC_RemoveItem()?;

					}
					} 
				}
				recog.base.set_state(466);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RemoveItem ----------------
pub type OC_RemoveItemContextAll<'input> = OC_RemoveItemContext<'input>;


pub type OC_RemoveItemContext<'input> = BaseParserRuleContext<'input,OC_RemoveItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RemoveItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RemoveItemContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RemoveItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RemoveItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RemoveItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RemoveItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RemoveItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RemoveItem }
}
antlr4rust::tid!{OC_RemoveItemContextExt<'a>}

impl<'input> OC_RemoveItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RemoveItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RemoveItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RemoveItemContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RemoveItemContextExt<'input>>{

fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PropertyExpression(&self) -> Option<Rc<OC_PropertyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_RemoveItemContextAttrs<'input> for OC_RemoveItemContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RemoveItem(&mut self,)
	-> Result<Rc<OC_RemoveItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RemoveItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_oC_RemoveItem);
        let mut _localctx: Rc<OC_RemoveItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(471);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(467);
					recog.oC_Variable()?;

					/*InvokeRule oC_NodeLabels*/
					recog.base.set_state(468);
					recog.oC_NodeLabels()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_PropertyExpression*/
					recog.base.set_state(470);
					recog.oC_PropertyExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_InQueryCall ----------------
pub type OC_InQueryCallContextAll<'input> = OC_InQueryCallContext<'input>;


pub type OC_InQueryCallContext<'input> = BaseParserRuleContext<'input,OC_InQueryCallContextExt<'input>>;

#[derive(Clone)]
pub struct OC_InQueryCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_InQueryCallContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_InQueryCallContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_InQueryCall(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_InQueryCall(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_InQueryCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_InQueryCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_InQueryCall }
}
antlr4rust::tid!{OC_InQueryCallContextExt<'a>}

impl<'input> OC_InQueryCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_InQueryCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_InQueryCallContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_InQueryCallContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_InQueryCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CALL, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_ExplicitProcedureInvocation(&self) -> Option<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token YIELD
/// Returns `None` if there is no child corresponding to token YIELD
fn YIELD(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_YIELD, 0)
}
fn oC_YieldItems(&self) -> Option<Rc<OC_YieldItemsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_InQueryCallContextAttrs<'input> for OC_InQueryCallContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_InQueryCall(&mut self,)
	-> Result<Rc<OC_InQueryCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_InQueryCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_oC_InQueryCall);
        let mut _localctx: Rc<OC_InQueryCallContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(473);
			recog.base.match_token(Cypher_CALL,&mut recog.err_handler)?;

			recog.base.set_state(474);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_ExplicitProcedureInvocation*/
			recog.base.set_state(475);
			recog.oC_ExplicitProcedureInvocation()?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(477);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(476);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(479);
					recog.base.match_token(Cypher_YIELD,&mut recog.err_handler)?;

					recog.base.set_state(480);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_YieldItems*/
					recog.base.set_state(481);
					recog.oC_YieldItems()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_StandaloneCall ----------------
pub type OC_StandaloneCallContextAll<'input> = OC_StandaloneCallContext<'input>;


pub type OC_StandaloneCallContext<'input> = BaseParserRuleContext<'input,OC_StandaloneCallContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StandaloneCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_StandaloneCallContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StandaloneCallContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_StandaloneCall(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_StandaloneCall(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_StandaloneCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_StandaloneCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_StandaloneCall }
}
antlr4rust::tid!{OC_StandaloneCallContextExt<'a>}

impl<'input> OC_StandaloneCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_StandaloneCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_StandaloneCallContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_StandaloneCallContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_StandaloneCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CALL, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_ExplicitProcedureInvocation(&self) -> Option<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ImplicitProcedureInvocation(&self) -> Option<Rc<OC_ImplicitProcedureInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token YIELD
/// Returns `None` if there is no child corresponding to token YIELD
fn YIELD(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_YIELD, 0)
}
fn oC_YieldItems(&self) -> Option<Rc<OC_YieldItemsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_StandaloneCallContextAttrs<'input> for OC_StandaloneCallContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_StandaloneCall(&mut self,)
	-> Result<Rc<OC_StandaloneCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_StandaloneCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_oC_StandaloneCall);
        let mut _localctx: Rc<OC_StandaloneCallContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(484);
			recog.base.match_token(Cypher_CALL,&mut recog.err_handler)?;

			recog.base.set_state(485);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			recog.base.set_state(488);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(58,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule oC_ExplicitProcedureInvocation*/
					recog.base.set_state(486);
					recog.oC_ExplicitProcedureInvocation()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule oC_ImplicitProcedureInvocation*/
					recog.base.set_state(487);
					recog.oC_ImplicitProcedureInvocation()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(494);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(59,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(490);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(491);
					recog.base.match_token(Cypher_YIELD,&mut recog.err_handler)?;

					recog.base.set_state(492);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_YieldItems*/
					recog.base.set_state(493);
					recog.oC_YieldItems()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_YieldItems ----------------
pub type OC_YieldItemsContextAll<'input> = OC_YieldItemsContext<'input>;


pub type OC_YieldItemsContext<'input> = BaseParserRuleContext<'input,OC_YieldItemsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_YieldItemsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_YieldItemsContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_YieldItemsContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_YieldItems(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_YieldItems(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_YieldItemsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_YieldItems }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_YieldItems }
}
antlr4rust::tid!{OC_YieldItemsContextExt<'a>}

impl<'input> OC_YieldItemsContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_YieldItemsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_YieldItemsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_YieldItemsContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_YieldItemsContextExt<'input>>{

fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_YieldItem_all(&self) ->  Vec<Rc<OC_YieldItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_YieldItem(&self, i: usize) -> Option<Rc<OC_YieldItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_YieldItemsContextAttrs<'input> for OC_YieldItemsContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_YieldItems(&mut self,)
	-> Result<Rc<OC_YieldItemsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_YieldItemsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_oC_YieldItems);
        let mut _localctx: Rc<OC_YieldItemsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(511);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_T__4 
				=> {
					{
					recog.base.set_state(496);
					recog.base.match_token(Cypher_T__4,&mut recog.err_handler)?;

					}
				}

			Cypher_COUNT |Cypher_ANY |Cypher_NONE |Cypher_SINGLE |Cypher_HexLetter |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					{
					{
					/*InvokeRule oC_YieldItem*/
					recog.base.set_state(497);
					recog.oC_YieldItem()?;

					recog.base.set_state(508);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(499);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(498);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(501);
							recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

							recog.base.set_state(503);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(502);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_YieldItem*/
							recog.base.set_state(505);
							recog.oC_YieldItem()?;

							}
							} 
						}
						recog.base.set_state(510);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(517);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(514);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(513);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Where*/
					recog.base.set_state(516);
					recog.oC_Where()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_YieldItem ----------------
pub type OC_YieldItemContextAll<'input> = OC_YieldItemContext<'input>;


pub type OC_YieldItemContext<'input> = BaseParserRuleContext<'input,OC_YieldItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_YieldItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_YieldItemContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_YieldItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_YieldItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_YieldItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_YieldItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_YieldItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_YieldItem }
}
antlr4rust::tid!{OC_YieldItemContextExt<'a>}

impl<'input> OC_YieldItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_YieldItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_YieldItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_YieldItemContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_YieldItemContextExt<'input>>{

fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ProcedureResultField(&self) -> Option<Rc<OC_ProcedureResultFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AS, 0)
}

}

impl<'input> OC_YieldItemContextAttrs<'input> for OC_YieldItemContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_YieldItem(&mut self,)
	-> Result<Rc<OC_YieldItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_YieldItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_oC_YieldItem);
        let mut _localctx: Rc<OC_YieldItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(524);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule oC_ProcedureResultField*/
					recog.base.set_state(519);
					recog.oC_ProcedureResultField()?;

					recog.base.set_state(520);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(521);
					recog.base.match_token(Cypher_AS,&mut recog.err_handler)?;

					recog.base.set_state(522);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule oC_Variable*/
			recog.base.set_state(526);
			recog.oC_Variable()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_With ----------------
pub type OC_WithContextAll<'input> = OC_WithContext<'input>;


pub type OC_WithContext<'input> = BaseParserRuleContext<'input,OC_WithContextExt<'input>>;

#[derive(Clone)]
pub struct OC_WithContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_WithContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_WithContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_With(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_With(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_WithContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_With }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_With }
}
antlr4rust::tid!{OC_WithContextExt<'a>}

impl<'input> OC_WithContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_WithContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_WithContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_WithContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_WithContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WITH, 0)
}
fn oC_ProjectionBody(&self) -> Option<Rc<OC_ProjectionBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_WithContextAttrs<'input> for OC_WithContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_With(&mut self,)
	-> Result<Rc<OC_WithContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_WithContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_oC_With);
        let mut _localctx: Rc<OC_WithContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(528);
			recog.base.match_token(Cypher_WITH,&mut recog.err_handler)?;

			/*InvokeRule oC_ProjectionBody*/
			recog.base.set_state(529);
			recog.oC_ProjectionBody()?;

			recog.base.set_state(534);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(68,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(531);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(530);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Where*/
					recog.base.set_state(533);
					recog.oC_Where()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Return ----------------
pub type OC_ReturnContextAll<'input> = OC_ReturnContext<'input>;


pub type OC_ReturnContext<'input> = BaseParserRuleContext<'input,OC_ReturnContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReturnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ReturnContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReturnContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Return(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Return(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ReturnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Return }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Return }
}
antlr4rust::tid!{OC_ReturnContextExt<'a>}

impl<'input> OC_ReturnContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ReturnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ReturnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ReturnContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ReturnContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_RETURN, 0)
}
fn oC_ProjectionBody(&self) -> Option<Rc<OC_ProjectionBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ReturnContextAttrs<'input> for OC_ReturnContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Return(&mut self,)
	-> Result<Rc<OC_ReturnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ReturnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_oC_Return);
        let mut _localctx: Rc<OC_ReturnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(536);
			recog.base.match_token(Cypher_RETURN,&mut recog.err_handler)?;

			/*InvokeRule oC_ProjectionBody*/
			recog.base.set_state(537);
			recog.oC_ProjectionBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ProjectionBody ----------------
pub type OC_ProjectionBodyContextAll<'input> = OC_ProjectionBodyContext<'input>;


pub type OC_ProjectionBodyContext<'input> = BaseParserRuleContext<'input,OC_ProjectionBodyContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ProjectionBodyContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ProjectionBody(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ProjectionBody(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ProjectionBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionBody }
}
antlr4rust::tid!{OC_ProjectionBodyContextExt<'a>}

impl<'input> OC_ProjectionBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ProjectionBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ProjectionBodyContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ProjectionBodyContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ProjectionBodyContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_ProjectionItems(&self) -> Option<Rc<OC_ProjectionItemsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DISTINCT
/// Returns `None` if there is no child corresponding to token DISTINCT
fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DISTINCT, 0)
}
fn oC_Order(&self) -> Option<Rc<OC_OrderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Skip(&self) -> Option<Rc<OC_SkipContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Limit(&self) -> Option<Rc<OC_LimitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ProjectionBodyContextAttrs<'input> for OC_ProjectionBodyContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ProjectionBody(&mut self,)
	-> Result<Rc<OC_ProjectionBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ProjectionBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_oC_ProjectionBody);
        let mut _localctx: Rc<OC_ProjectionBodyContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(70,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(540);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(539);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(542);
					recog.base.match_token(Cypher_DISTINCT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(545);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_ProjectionItems*/
			recog.base.set_state(546);
			recog.oC_ProjectionItems()?;

			recog.base.set_state(549);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(547);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Order*/
					recog.base.set_state(548);
					recog.oC_Order()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(553);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(72,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(551);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Skip*/
					recog.base.set_state(552);
					recog.oC_Skip()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(557);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(73,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(555);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Limit*/
					recog.base.set_state(556);
					recog.oC_Limit()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ProjectionItems ----------------
pub type OC_ProjectionItemsContextAll<'input> = OC_ProjectionItemsContext<'input>;


pub type OC_ProjectionItemsContext<'input> = BaseParserRuleContext<'input,OC_ProjectionItemsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionItemsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ProjectionItemsContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionItemsContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ProjectionItems(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ProjectionItems(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionItemsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ProjectionItems }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionItems }
}
antlr4rust::tid!{OC_ProjectionItemsContextExt<'a>}

impl<'input> OC_ProjectionItemsContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ProjectionItemsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ProjectionItemsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ProjectionItemsContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ProjectionItemsContextExt<'input>>{

fn oC_ProjectionItem_all(&self) ->  Vec<Rc<OC_ProjectionItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_ProjectionItem(&self, i: usize) -> Option<Rc<OC_ProjectionItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_ProjectionItemsContextAttrs<'input> for OC_ProjectionItemsContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ProjectionItems(&mut self,)
	-> Result<Rc<OC_ProjectionItemsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ProjectionItemsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_oC_ProjectionItems);
        let mut _localctx: Rc<OC_ProjectionItemsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(587);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(559);
					recog.base.match_token(Cypher_T__4,&mut recog.err_handler)?;

					recog.base.set_state(570);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(76,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(561);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(560);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(563);
							recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

							recog.base.set_state(565);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(564);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_ProjectionItem*/
							recog.base.set_state(567);
							recog.oC_ProjectionItem()?;

							}
							} 
						}
						recog.base.set_state(572);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(76,&mut recog.base)?;
					}
					}
					}
				}

			Cypher_T__5 |Cypher_T__7 |Cypher_T__12 |Cypher_T__13 |Cypher_T__23 |Cypher_T__25 |
			Cypher_ALL |Cypher_NOT |Cypher_NULL |Cypher_COUNT |Cypher_ANY |Cypher_NONE |
			Cypher_SINGLE |Cypher_TRUE |Cypher_FALSE |Cypher_EXISTS |Cypher_CASE |
			Cypher_StringLiteral |Cypher_HexInteger |Cypher_DecimalInteger |Cypher_OctalInteger |
			Cypher_HexLetter |Cypher_ExponentDecimalReal |Cypher_RegularDecimalReal |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					/*InvokeRule oC_ProjectionItem*/
					recog.base.set_state(573);
					recog.oC_ProjectionItem()?;

					recog.base.set_state(584);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(79,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(575);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(574);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(577);
							recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

							recog.base.set_state(579);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(578);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_ProjectionItem*/
							recog.base.set_state(581);
							recog.oC_ProjectionItem()?;

							}
							} 
						}
						recog.base.set_state(586);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(79,&mut recog.base)?;
					}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ProjectionItem ----------------
pub type OC_ProjectionItemContextAll<'input> = OC_ProjectionItemContext<'input>;


pub type OC_ProjectionItemContext<'input> = BaseParserRuleContext<'input,OC_ProjectionItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ProjectionItemContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ProjectionItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ProjectionItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ProjectionItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionItem }
}
antlr4rust::tid!{OC_ProjectionItemContextExt<'a>}

impl<'input> OC_ProjectionItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ProjectionItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ProjectionItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ProjectionItemContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ProjectionItemContextExt<'input>>{

fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AS, 0)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ProjectionItemContextAttrs<'input> for OC_ProjectionItemContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ProjectionItem(&mut self,)
	-> Result<Rc<OC_ProjectionItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ProjectionItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_oC_ProjectionItem);
        let mut _localctx: Rc<OC_ProjectionItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(596);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(81,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_Expression*/
					recog.base.set_state(589);
					recog.oC_Expression()?;

					recog.base.set_state(590);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(591);
					recog.base.match_token(Cypher_AS,&mut recog.err_handler)?;

					recog.base.set_state(592);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_Variable*/
					recog.base.set_state(593);
					recog.oC_Variable()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_Expression*/
					recog.base.set_state(595);
					recog.oC_Expression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Order ----------------
pub type OC_OrderContextAll<'input> = OC_OrderContext<'input>;


pub type OC_OrderContext<'input> = BaseParserRuleContext<'input,OC_OrderContextExt<'input>>;

#[derive(Clone)]
pub struct OC_OrderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_OrderContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_OrderContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Order(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Order(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_OrderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Order }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Order }
}
antlr4rust::tid!{OC_OrderContextExt<'a>}

impl<'input> OC_OrderContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_OrderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_OrderContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_OrderContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_OrderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ORDER
/// Returns `None` if there is no child corresponding to token ORDER
fn ORDER(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ORDER, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_BY, 0)
}
fn oC_SortItem_all(&self) ->  Vec<Rc<OC_SortItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_SortItem(&self, i: usize) -> Option<Rc<OC_SortItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_OrderContextAttrs<'input> for OC_OrderContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Order(&mut self,)
	-> Result<Rc<OC_OrderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_OrderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_oC_Order);
        let mut _localctx: Rc<OC_OrderContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(598);
			recog.base.match_token(Cypher_ORDER,&mut recog.err_handler)?;

			recog.base.set_state(599);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			recog.base.set_state(600);
			recog.base.match_token(Cypher_BY,&mut recog.err_handler)?;

			recog.base.set_state(601);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_SortItem*/
			recog.base.set_state(602);
			recog.oC_SortItem()?;

			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Cypher_T__1 {
				{
				{
				recog.base.set_state(603);
				recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

				recog.base.set_state(605);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(604);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule oC_SortItem*/
				recog.base.set_state(607);
				recog.oC_SortItem()?;

				}
				}
				recog.base.set_state(612);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Skip ----------------
pub type OC_SkipContextAll<'input> = OC_SkipContext<'input>;


pub type OC_SkipContext<'input> = BaseParserRuleContext<'input,OC_SkipContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SkipContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SkipContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SkipContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Skip(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Skip(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SkipContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Skip }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Skip }
}
antlr4rust::tid!{OC_SkipContextExt<'a>}

impl<'input> OC_SkipContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SkipContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SkipContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SkipContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SkipContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_SKIP
/// Returns `None` if there is no child corresponding to token L_SKIP
fn L_SKIP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_L_SKIP, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_SkipContextAttrs<'input> for OC_SkipContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Skip(&mut self,)
	-> Result<Rc<OC_SkipContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SkipContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_oC_Skip);
        let mut _localctx: Rc<OC_SkipContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(613);
			recog.base.match_token(Cypher_L_SKIP,&mut recog.err_handler)?;

			recog.base.set_state(614);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_Expression*/
			recog.base.set_state(615);
			recog.oC_Expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Limit ----------------
pub type OC_LimitContextAll<'input> = OC_LimitContext<'input>;


pub type OC_LimitContext<'input> = BaseParserRuleContext<'input,OC_LimitContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LimitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_LimitContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LimitContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Limit(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Limit(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_LimitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Limit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Limit }
}
antlr4rust::tid!{OC_LimitContextExt<'a>}

impl<'input> OC_LimitContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_LimitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_LimitContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_LimitContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_LimitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LIMIT
/// Returns `None` if there is no child corresponding to token LIMIT
fn LIMIT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_LIMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_LimitContextAttrs<'input> for OC_LimitContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Limit(&mut self,)
	-> Result<Rc<OC_LimitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_LimitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_oC_Limit);
        let mut _localctx: Rc<OC_LimitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(617);
			recog.base.match_token(Cypher_LIMIT,&mut recog.err_handler)?;

			recog.base.set_state(618);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_Expression*/
			recog.base.set_state(619);
			recog.oC_Expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SortItem ----------------
pub type OC_SortItemContextAll<'input> = OC_SortItemContext<'input>;


pub type OC_SortItemContext<'input> = BaseParserRuleContext<'input,OC_SortItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SortItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SortItemContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SortItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SortItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SortItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SortItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SortItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SortItem }
}
antlr4rust::tid!{OC_SortItemContextExt<'a>}

impl<'input> OC_SortItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SortItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SortItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SortItemContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SortItemContextExt<'input>>{

fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASCENDING
/// Returns `None` if there is no child corresponding to token ASCENDING
fn ASCENDING(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ASCENDING, 0)
}
/// Retrieves first TerminalNode corresponding to token ASC
/// Returns `None` if there is no child corresponding to token ASC
fn ASC(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ASC, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCENDING
/// Returns `None` if there is no child corresponding to token DESCENDING
fn DESCENDING(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DESCENDING, 0)
}
/// Retrieves first TerminalNode corresponding to token DESC
/// Returns `None` if there is no child corresponding to token DESC
fn DESC(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DESC, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_SortItemContextAttrs<'input> for OC_SortItemContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SortItem(&mut self,)
	-> Result<Rc<OC_SortItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SortItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_oC_SortItem);
        let mut _localctx: Rc<OC_SortItemContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Expression*/
			recog.base.set_state(621);
			recog.oC_Expression()?;

			recog.base.set_state(626);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(85,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(623);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(622);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(625);
					_la = recog.base.input.la(1);
					if { !(((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & 15) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Where ----------------
pub type OC_WhereContextAll<'input> = OC_WhereContext<'input>;


pub type OC_WhereContext<'input> = BaseParserRuleContext<'input,OC_WhereContextExt<'input>>;

#[derive(Clone)]
pub struct OC_WhereContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_WhereContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_WhereContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Where(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Where(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_WhereContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Where }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Where }
}
antlr4rust::tid!{OC_WhereContextExt<'a>}

impl<'input> OC_WhereContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_WhereContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_WhereContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_WhereContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_WhereContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WHERE, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_WhereContextAttrs<'input> for OC_WhereContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Where(&mut self,)
	-> Result<Rc<OC_WhereContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_WhereContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_oC_Where);
        let mut _localctx: Rc<OC_WhereContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(628);
			recog.base.match_token(Cypher_WHERE,&mut recog.err_handler)?;

			recog.base.set_state(629);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_Expression*/
			recog.base.set_state(630);
			recog.oC_Expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Pattern ----------------
pub type OC_PatternContextAll<'input> = OC_PatternContext<'input>;


pub type OC_PatternContext<'input> = BaseParserRuleContext<'input,OC_PatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PatternContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Pattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Pattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Pattern }
}
antlr4rust::tid!{OC_PatternContextExt<'a>}

impl<'input> OC_PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PatternContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PatternContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PatternContextExt<'input>>{

fn oC_PatternPart_all(&self) ->  Vec<Rc<OC_PatternPartContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PatternPart(&self, i: usize) -> Option<Rc<OC_PatternPartContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_PatternContextAttrs<'input> for OC_PatternContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Pattern(&mut self,)
	-> Result<Rc<OC_PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_oC_Pattern);
        let mut _localctx: Rc<OC_PatternContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_PatternPart*/
			recog.base.set_state(632);
			recog.oC_PatternPart()?;

			recog.base.set_state(643);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(634);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(633);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(636);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(638);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(637);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PatternPart*/
					recog.base.set_state(640);
					recog.oC_PatternPart()?;

					}
					} 
				}
				recog.base.set_state(645);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PatternPart ----------------
pub type OC_PatternPartContextAll<'input> = OC_PatternPartContext<'input>;


pub type OC_PatternPartContext<'input> = BaseParserRuleContext<'input,OC_PatternPartContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternPartContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PatternPartContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternPartContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PatternPart(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PatternPart(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PatternPartContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PatternPart }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternPart }
}
antlr4rust::tid!{OC_PatternPartContextExt<'a>}

impl<'input> OC_PatternPartContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PatternPartContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PatternPartContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PatternPartContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PatternPartContextExt<'input>>{

fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_AnonymousPatternPart(&self) -> Option<Rc<OC_AnonymousPatternPartContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_PatternPartContextAttrs<'input> for OC_PatternPartContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PatternPart(&mut self,)
	-> Result<Rc<OC_PatternPartContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PatternPartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_oC_PatternPart);
        let mut _localctx: Rc<OC_PatternPartContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(657);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_COUNT |Cypher_ANY |Cypher_NONE |Cypher_SINGLE |Cypher_HexLetter |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(646);
					recog.oC_Variable()?;

					recog.base.set_state(648);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(647);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(650);
					recog.base.match_token(Cypher_T__2,&mut recog.err_handler)?;

					recog.base.set_state(652);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(651);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AnonymousPatternPart*/
					recog.base.set_state(654);
					recog.oC_AnonymousPatternPart()?;

					}
					}
				}

			Cypher_T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_AnonymousPatternPart*/
					recog.base.set_state(656);
					recog.oC_AnonymousPatternPart()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_AnonymousPatternPart ----------------
pub type OC_AnonymousPatternPartContextAll<'input> = OC_AnonymousPatternPartContext<'input>;


pub type OC_AnonymousPatternPartContext<'input> = BaseParserRuleContext<'input,OC_AnonymousPatternPartContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AnonymousPatternPartContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_AnonymousPatternPartContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AnonymousPatternPartContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_AnonymousPatternPart(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_AnonymousPatternPart(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_AnonymousPatternPartContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_AnonymousPatternPart }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_AnonymousPatternPart }
}
antlr4rust::tid!{OC_AnonymousPatternPartContextExt<'a>}

impl<'input> OC_AnonymousPatternPartContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_AnonymousPatternPartContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_AnonymousPatternPartContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_AnonymousPatternPartContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_AnonymousPatternPartContextExt<'input>>{

fn oC_PatternElement(&self) -> Option<Rc<OC_PatternElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_AnonymousPatternPartContextAttrs<'input> for OC_AnonymousPatternPartContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_AnonymousPatternPart(&mut self,)
	-> Result<Rc<OC_AnonymousPatternPartContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_AnonymousPatternPartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_oC_AnonymousPatternPart);
        let mut _localctx: Rc<OC_AnonymousPatternPartContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_PatternElement*/
			recog.base.set_state(659);
			recog.oC_PatternElement()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PatternElement ----------------
pub type OC_PatternElementContextAll<'input> = OC_PatternElementContext<'input>;


pub type OC_PatternElementContext<'input> = BaseParserRuleContext<'input,OC_PatternElementContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PatternElementContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternElementContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PatternElement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PatternElement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PatternElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PatternElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternElement }
}
antlr4rust::tid!{OC_PatternElementContextExt<'a>}

impl<'input> OC_PatternElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PatternElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PatternElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PatternElementContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PatternElementContextExt<'input>>{

fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PatternElementChain_all(&self) ->  Vec<Rc<OC_PatternElementChainContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PatternElementChain(&self, i: usize) -> Option<Rc<OC_PatternElementChainContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_PatternElement(&self) -> Option<Rc<OC_PatternElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_PatternElementContextAttrs<'input> for OC_PatternElementContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PatternElement(&mut self,)
	-> Result<Rc<OC_PatternElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PatternElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_oC_PatternElement);
        let mut _localctx: Rc<OC_PatternElementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(675);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(94,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_NodePattern*/
					recog.base.set_state(661);
					recog.oC_NodePattern()?;

					recog.base.set_state(668);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(663);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(662);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_PatternElementChain*/
							recog.base.set_state(665);
							recog.oC_PatternElementChain()?;

							}
							} 
						}
						recog.base.set_state(670);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
					}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(671);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					/*InvokeRule oC_PatternElement*/
					recog.base.set_state(672);
					recog.oC_PatternElement()?;

					recog.base.set_state(673);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NodePattern ----------------
pub type OC_NodePatternContextAll<'input> = OC_NodePatternContext<'input>;


pub type OC_NodePatternContext<'input> = BaseParserRuleContext<'input,OC_NodePatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodePatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NodePatternContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodePatternContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NodePattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NodePattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NodePatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NodePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodePattern }
}
antlr4rust::tid!{OC_NodePatternContextExt<'a>}

impl<'input> OC_NodePatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NodePatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NodePatternContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NodePatternContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NodePatternContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Properties(&self) -> Option<Rc<OC_PropertiesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_NodePatternContextAttrs<'input> for OC_NodePatternContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NodePattern(&mut self,)
	-> Result<Rc<OC_NodePatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NodePatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_oC_NodePattern);
        let mut _localctx: Rc<OC_NodePatternContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(677);
			recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

			recog.base.set_state(679);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(678);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(685);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 83)) & !0x3f) == 0 && ((1usize << (_la - 83)) & 131087) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Variable*/
				recog.base.set_state(681);
				recog.oC_Variable()?;

				recog.base.set_state(683);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(682);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(691);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__9 {
				{
				/*InvokeRule oC_NodeLabels*/
				recog.base.set_state(687);
				recog.oC_NodeLabels()?;

				recog.base.set_state(689);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(688);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(697);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__23 || _la==Cypher_T__25 {
				{
				/*InvokeRule oC_Properties*/
				recog.base.set_state(693);
				recog.oC_Properties()?;

				recog.base.set_state(695);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(694);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(699);
			recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PatternElementChain ----------------
pub type OC_PatternElementChainContextAll<'input> = OC_PatternElementChainContext<'input>;


pub type OC_PatternElementChainContext<'input> = BaseParserRuleContext<'input,OC_PatternElementChainContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternElementChainContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PatternElementChainContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternElementChainContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PatternElementChain(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PatternElementChain(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PatternElementChainContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PatternElementChain }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternElementChain }
}
antlr4rust::tid!{OC_PatternElementChainContextExt<'a>}

impl<'input> OC_PatternElementChainContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PatternElementChainContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PatternElementChainContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PatternElementChainContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PatternElementChainContextExt<'input>>{

fn oC_RelationshipPattern(&self) -> Option<Rc<OC_RelationshipPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_PatternElementChainContextAttrs<'input> for OC_PatternElementChainContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PatternElementChain(&mut self,)
	-> Result<Rc<OC_PatternElementChainContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PatternElementChainContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_oC_PatternElementChain);
        let mut _localctx: Rc<OC_PatternElementChainContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_RelationshipPattern*/
			recog.base.set_state(701);
			recog.oC_RelationshipPattern()?;

			recog.base.set_state(703);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(702);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_NodePattern*/
			recog.base.set_state(705);
			recog.oC_NodePattern()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RelationshipPattern ----------------
pub type OC_RelationshipPatternContextAll<'input> = OC_RelationshipPatternContext<'input>;


pub type OC_RelationshipPatternContext<'input> = BaseParserRuleContext<'input,OC_RelationshipPatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipPatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RelationshipPatternContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelationshipPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RelationshipPattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RelationshipPattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RelationshipPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipPattern }
}
antlr4rust::tid!{OC_RelationshipPatternContextExt<'a>}

impl<'input> OC_RelationshipPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RelationshipPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RelationshipPatternContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RelationshipPatternContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RelationshipPatternContextExt<'input>>{

fn oC_LeftArrowHead(&self) -> Option<Rc<OC_LeftArrowHeadContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Dash_all(&self) ->  Vec<Rc<OC_DashContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Dash(&self, i: usize) -> Option<Rc<OC_DashContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_RightArrowHead(&self) -> Option<Rc<OC_RightArrowHeadContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_RelationshipDetail(&self) -> Option<Rc<OC_RelationshipDetailContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_RelationshipPatternContextAttrs<'input> for OC_RelationshipPatternContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RelationshipPattern(&mut self,)
	-> Result<Rc<OC_RelationshipPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RelationshipPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_oC_RelationshipPattern);
        let mut _localctx: Rc<OC_RelationshipPatternContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(771);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(119,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_LeftArrowHead*/
					recog.base.set_state(707);
					recog.oC_LeftArrowHead()?;

					recog.base.set_state(709);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(708);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(711);
					recog.oC_Dash()?;

					recog.base.set_state(713);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(104,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(712);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					recog.base.set_state(716);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_T__7 {
						{
						/*InvokeRule oC_RelationshipDetail*/
						recog.base.set_state(715);
						recog.oC_RelationshipDetail()?;

						}
					}

					recog.base.set_state(719);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(718);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(721);
					recog.oC_Dash()?;

					recog.base.set_state(723);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(722);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_RightArrowHead*/
					recog.base.set_state(725);
					recog.oC_RightArrowHead()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					/*InvokeRule oC_LeftArrowHead*/
					recog.base.set_state(727);
					recog.oC_LeftArrowHead()?;

					recog.base.set_state(729);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(728);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(731);
					recog.oC_Dash()?;

					recog.base.set_state(733);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(109,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(732);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					recog.base.set_state(736);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_T__7 {
						{
						/*InvokeRule oC_RelationshipDetail*/
						recog.base.set_state(735);
						recog.oC_RelationshipDetail()?;

						}
					}

					recog.base.set_state(739);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(738);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(741);
					recog.oC_Dash()?;

					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					/*InvokeRule oC_Dash*/
					recog.base.set_state(743);
					recog.oC_Dash()?;

					recog.base.set_state(745);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(112,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(744);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					recog.base.set_state(748);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_T__7 {
						{
						/*InvokeRule oC_RelationshipDetail*/
						recog.base.set_state(747);
						recog.oC_RelationshipDetail()?;

						}
					}

					recog.base.set_state(751);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(750);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(753);
					recog.oC_Dash()?;

					recog.base.set_state(755);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(754);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_RightArrowHead*/
					recog.base.set_state(757);
					recog.oC_RightArrowHead()?;

					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					{
					/*InvokeRule oC_Dash*/
					recog.base.set_state(759);
					recog.oC_Dash()?;

					recog.base.set_state(761);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(116,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(760);
							recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					recog.base.set_state(764);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_T__7 {
						{
						/*InvokeRule oC_RelationshipDetail*/
						recog.base.set_state(763);
						recog.oC_RelationshipDetail()?;

						}
					}

					recog.base.set_state(767);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(766);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Dash*/
					recog.base.set_state(769);
					recog.oC_Dash()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RelationshipDetail ----------------
pub type OC_RelationshipDetailContextAll<'input> = OC_RelationshipDetailContext<'input>;


pub type OC_RelationshipDetailContext<'input> = BaseParserRuleContext<'input,OC_RelationshipDetailContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipDetailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RelationshipDetailContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelationshipDetailContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RelationshipDetail(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RelationshipDetail(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipDetailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RelationshipDetail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipDetail }
}
antlr4rust::tid!{OC_RelationshipDetailContextExt<'a>}

impl<'input> OC_RelationshipDetailContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RelationshipDetailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RelationshipDetailContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RelationshipDetailContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RelationshipDetailContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_RelationshipTypes(&self) -> Option<Rc<OC_RelationshipTypesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_RangeLiteral(&self) -> Option<Rc<OC_RangeLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Properties(&self) -> Option<Rc<OC_PropertiesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_RelationshipDetailContextAttrs<'input> for OC_RelationshipDetailContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RelationshipDetail(&mut self,)
	-> Result<Rc<OC_RelationshipDetailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RelationshipDetailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_oC_RelationshipDetail);
        let mut _localctx: Rc<OC_RelationshipDetailContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(773);
			recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

			recog.base.set_state(775);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(774);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(781);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 83)) & !0x3f) == 0 && ((1usize << (_la - 83)) & 131087) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Variable*/
				recog.base.set_state(777);
				recog.oC_Variable()?;

				recog.base.set_state(779);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(778);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(787);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__9 {
				{
				/*InvokeRule oC_RelationshipTypes*/
				recog.base.set_state(783);
				recog.oC_RelationshipTypes()?;

				recog.base.set_state(785);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(784);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(790);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__4 {
				{
				/*InvokeRule oC_RangeLiteral*/
				recog.base.set_state(789);
				recog.oC_RangeLiteral()?;

				}
			}

			recog.base.set_state(796);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__23 || _la==Cypher_T__25 {
				{
				/*InvokeRule oC_Properties*/
				recog.base.set_state(792);
				recog.oC_Properties()?;

				recog.base.set_state(794);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(793);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(798);
			recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Properties ----------------
pub type OC_PropertiesContextAll<'input> = OC_PropertiesContext<'input>;


pub type OC_PropertiesContext<'input> = BaseParserRuleContext<'input,OC_PropertiesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertiesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PropertiesContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertiesContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Properties(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Properties(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PropertiesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Properties }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Properties }
}
antlr4rust::tid!{OC_PropertiesContextExt<'a>}

impl<'input> OC_PropertiesContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PropertiesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PropertiesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PropertiesContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PropertiesContextExt<'input>>{

fn oC_MapLiteral(&self) -> Option<Rc<OC_MapLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Parameter(&self) -> Option<Rc<OC_ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_PropertiesContextAttrs<'input> for OC_PropertiesContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Properties(&mut self,)
	-> Result<Rc<OC_PropertiesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PropertiesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_oC_Properties);
        let mut _localctx: Rc<OC_PropertiesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(802);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_T__23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_MapLiteral*/
					recog.base.set_state(800);
					recog.oC_MapLiteral()?;

					}
				}

			Cypher_T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_Parameter*/
					recog.base.set_state(801);
					recog.oC_Parameter()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RelationshipTypes ----------------
pub type OC_RelationshipTypesContextAll<'input> = OC_RelationshipTypesContext<'input>;


pub type OC_RelationshipTypesContext<'input> = BaseParserRuleContext<'input,OC_RelationshipTypesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipTypesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RelationshipTypesContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelationshipTypesContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RelationshipTypes(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RelationshipTypes(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipTypesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RelationshipTypes }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipTypes }
}
antlr4rust::tid!{OC_RelationshipTypesContextExt<'a>}

impl<'input> OC_RelationshipTypesContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RelationshipTypesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RelationshipTypesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RelationshipTypesContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RelationshipTypesContextExt<'input>>{

fn oC_RelTypeName_all(&self) ->  Vec<Rc<OC_RelTypeNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_RelTypeName(&self, i: usize) -> Option<Rc<OC_RelTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_RelationshipTypesContextAttrs<'input> for OC_RelationshipTypesContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RelationshipTypes(&mut self,)
	-> Result<Rc<OC_RelationshipTypesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RelationshipTypesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_oC_RelationshipTypes);
        let mut _localctx: Rc<OC_RelationshipTypesContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(804);
			recog.base.match_token(Cypher_T__9,&mut recog.err_handler)?;

			recog.base.set_state(806);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(805);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_RelTypeName*/
			recog.base.set_state(808);
			recog.oC_RelTypeName()?;

			recog.base.set_state(822);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(133,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(810);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(809);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(812);
					recog.base.match_token(Cypher_T__10,&mut recog.err_handler)?;

					recog.base.set_state(814);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_T__9 {
						{
						recog.base.set_state(813);
						recog.base.match_token(Cypher_T__9,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(817);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(816);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_RelTypeName*/
					recog.base.set_state(819);
					recog.oC_RelTypeName()?;

					}
					} 
				}
				recog.base.set_state(824);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(133,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NodeLabels ----------------
pub type OC_NodeLabelsContextAll<'input> = OC_NodeLabelsContext<'input>;


pub type OC_NodeLabelsContext<'input> = BaseParserRuleContext<'input,OC_NodeLabelsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodeLabelsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NodeLabelsContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodeLabelsContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NodeLabels(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NodeLabels(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NodeLabelsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NodeLabels }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodeLabels }
}
antlr4rust::tid!{OC_NodeLabelsContextExt<'a>}

impl<'input> OC_NodeLabelsContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NodeLabelsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NodeLabelsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NodeLabelsContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NodeLabelsContextExt<'input>>{

fn oC_NodeLabel_all(&self) ->  Vec<Rc<OC_NodeLabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_NodeLabel(&self, i: usize) -> Option<Rc<OC_NodeLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_NodeLabelsContextAttrs<'input> for OC_NodeLabelsContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NodeLabels(&mut self,)
	-> Result<Rc<OC_NodeLabelsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NodeLabelsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_oC_NodeLabels);
        let mut _localctx: Rc<OC_NodeLabelsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_NodeLabel*/
			recog.base.set_state(825);
			recog.oC_NodeLabel()?;

			recog.base.set_state(832);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(135,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(827);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(826);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_NodeLabel*/
					recog.base.set_state(829);
					recog.oC_NodeLabel()?;

					}
					} 
				}
				recog.base.set_state(834);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(135,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NodeLabel ----------------
pub type OC_NodeLabelContextAll<'input> = OC_NodeLabelContext<'input>;


pub type OC_NodeLabelContext<'input> = BaseParserRuleContext<'input,OC_NodeLabelContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodeLabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NodeLabelContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodeLabelContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NodeLabel(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NodeLabel(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NodeLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NodeLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodeLabel }
}
antlr4rust::tid!{OC_NodeLabelContextExt<'a>}

impl<'input> OC_NodeLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NodeLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NodeLabelContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NodeLabelContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NodeLabelContextExt<'input>>{

fn oC_LabelName(&self) -> Option<Rc<OC_LabelNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_NodeLabelContextAttrs<'input> for OC_NodeLabelContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NodeLabel(&mut self,)
	-> Result<Rc<OC_NodeLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NodeLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_oC_NodeLabel);
        let mut _localctx: Rc<OC_NodeLabelContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(835);
			recog.base.match_token(Cypher_T__9,&mut recog.err_handler)?;

			recog.base.set_state(837);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(836);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_LabelName*/
			recog.base.set_state(839);
			recog.oC_LabelName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RangeLiteral ----------------
pub type OC_RangeLiteralContextAll<'input> = OC_RangeLiteralContext<'input>;


pub type OC_RangeLiteralContext<'input> = BaseParserRuleContext<'input,OC_RangeLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RangeLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RangeLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RangeLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RangeLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RangeLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RangeLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RangeLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RangeLiteral }
}
antlr4rust::tid!{OC_RangeLiteralContextExt<'a>}

impl<'input> OC_RangeLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RangeLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RangeLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RangeLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RangeLiteralContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_IntegerLiteral_all(&self) ->  Vec<Rc<OC_IntegerLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_IntegerLiteral(&self, i: usize) -> Option<Rc<OC_IntegerLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_RangeLiteralContextAttrs<'input> for OC_RangeLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RangeLiteral(&mut self,)
	-> Result<Rc<OC_RangeLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RangeLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_oC_RangeLiteral);
        let mut _localctx: Rc<OC_RangeLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(841);
			recog.base.match_token(Cypher_T__4,&mut recog.err_handler)?;

			recog.base.set_state(843);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(842);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(849);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & 7) != 0) {
				{
				/*InvokeRule oC_IntegerLiteral*/
				recog.base.set_state(845);
				recog.oC_IntegerLiteral()?;

				recog.base.set_state(847);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(846);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(861);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_T__11 {
				{
				recog.base.set_state(851);
				recog.base.match_token(Cypher_T__11,&mut recog.err_handler)?;

				recog.base.set_state(853);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(852);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(859);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & 7) != 0) {
					{
					/*InvokeRule oC_IntegerLiteral*/
					recog.base.set_state(855);
					recog.oC_IntegerLiteral()?;

					recog.base.set_state(857);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(856);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
				}

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_LabelName ----------------
pub type OC_LabelNameContextAll<'input> = OC_LabelNameContext<'input>;


pub type OC_LabelNameContext<'input> = BaseParserRuleContext<'input,OC_LabelNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LabelNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_LabelNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LabelNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_LabelName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_LabelName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_LabelNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_LabelName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_LabelName }
}
antlr4rust::tid!{OC_LabelNameContextExt<'a>}

impl<'input> OC_LabelNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_LabelNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_LabelNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_LabelNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_LabelNameContextExt<'input>>{

fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_LabelNameContextAttrs<'input> for OC_LabelNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_LabelName(&mut self,)
	-> Result<Rc<OC_LabelNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_LabelNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_oC_LabelName);
        let mut _localctx: Rc<OC_LabelNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SchemaName*/
			recog.base.set_state(863);
			recog.oC_SchemaName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RelTypeName ----------------
pub type OC_RelTypeNameContextAll<'input> = OC_RelTypeNameContext<'input>;


pub type OC_RelTypeNameContext<'input> = BaseParserRuleContext<'input,OC_RelTypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelTypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RelTypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelTypeNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RelTypeName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RelTypeName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RelTypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RelTypeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelTypeName }
}
antlr4rust::tid!{OC_RelTypeNameContextExt<'a>}

impl<'input> OC_RelTypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RelTypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RelTypeNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RelTypeNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RelTypeNameContextExt<'input>>{

fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_RelTypeNameContextAttrs<'input> for OC_RelTypeNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RelTypeName(&mut self,)
	-> Result<Rc<OC_RelTypeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RelTypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_oC_RelTypeName);
        let mut _localctx: Rc<OC_RelTypeNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SchemaName*/
			recog.base.set_state(865);
			recog.oC_SchemaName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Expression ----------------
pub type OC_ExpressionContextAll<'input> = OC_ExpressionContext<'input>;


pub type OC_ExpressionContext<'input> = BaseParserRuleContext<'input,OC_ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Expression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Expression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Expression }
}
antlr4rust::tid!{OC_ExpressionContextExt<'a>}

impl<'input> OC_ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ExpressionContextExt<'input>>{

fn oC_OrExpression(&self) -> Option<Rc<OC_OrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ExpressionContextAttrs<'input> for OC_ExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Expression(&mut self,)
	-> Result<Rc<OC_ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_oC_Expression);
        let mut _localctx: Rc<OC_ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_OrExpression*/
			recog.base.set_state(867);
			recog.oC_OrExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_OrExpression ----------------
pub type OC_OrExpressionContextAll<'input> = OC_OrExpressionContext<'input>;


pub type OC_OrExpressionContext<'input> = BaseParserRuleContext<'input,OC_OrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_OrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_OrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_OrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_OrExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_OrExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_OrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_OrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_OrExpression }
}
antlr4rust::tid!{OC_OrExpressionContextExt<'a>}

impl<'input> OC_OrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_OrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_OrExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_OrExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_OrExpressionContextExt<'input>>{

fn oC_XorExpression_all(&self) ->  Vec<Rc<OC_XorExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_XorExpression(&self, i: usize) -> Option<Rc<OC_XorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OR, i)
}

}

impl<'input> OC_OrExpressionContextAttrs<'input> for OC_OrExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_OrExpression(&mut self,)
	-> Result<Rc<OC_OrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_OrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_oC_OrExpression);
        let mut _localctx: Rc<OC_OrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_XorExpression*/
			recog.base.set_state(869);
			recog.oC_XorExpression()?;

			recog.base.set_state(876);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(144,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(870);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(871);
					recog.base.match_token(Cypher_OR,&mut recog.err_handler)?;

					recog.base.set_state(872);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_XorExpression*/
					recog.base.set_state(873);
					recog.oC_XorExpression()?;

					}
					} 
				}
				recog.base.set_state(878);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(144,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_XorExpression ----------------
pub type OC_XorExpressionContextAll<'input> = OC_XorExpressionContext<'input>;


pub type OC_XorExpressionContext<'input> = BaseParserRuleContext<'input,OC_XorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_XorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_XorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_XorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_XorExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_XorExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_XorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_XorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_XorExpression }
}
antlr4rust::tid!{OC_XorExpressionContextExt<'a>}

impl<'input> OC_XorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_XorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_XorExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_XorExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_XorExpressionContextExt<'input>>{

fn oC_AndExpression_all(&self) ->  Vec<Rc<OC_AndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_AndExpression(&self, i: usize) -> Option<Rc<OC_AndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves all `TerminalNode`s corresponding to token XOR in current rule
fn XOR_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token XOR, starting from 0.
/// Returns `None` if number of children corresponding to token XOR is less or equal than `i`.
fn XOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_XOR, i)
}

}

impl<'input> OC_XorExpressionContextAttrs<'input> for OC_XorExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_XorExpression(&mut self,)
	-> Result<Rc<OC_XorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_XorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_oC_XorExpression);
        let mut _localctx: Rc<OC_XorExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_AndExpression*/
			recog.base.set_state(879);
			recog.oC_AndExpression()?;

			recog.base.set_state(886);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(145,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(880);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(881);
					recog.base.match_token(Cypher_XOR,&mut recog.err_handler)?;

					recog.base.set_state(882);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_AndExpression*/
					recog.base.set_state(883);
					recog.oC_AndExpression()?;

					}
					} 
				}
				recog.base.set_state(888);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(145,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_AndExpression ----------------
pub type OC_AndExpressionContextAll<'input> = OC_AndExpressionContext<'input>;


pub type OC_AndExpressionContext<'input> = BaseParserRuleContext<'input,OC_AndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_AndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AndExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_AndExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_AndExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_AndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_AndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_AndExpression }
}
antlr4rust::tid!{OC_AndExpressionContextExt<'a>}

impl<'input> OC_AndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_AndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_AndExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_AndExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_AndExpressionContextExt<'input>>{

fn oC_NotExpression_all(&self) ->  Vec<Rc<OC_NotExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_NotExpression(&self, i: usize) -> Option<Rc<OC_NotExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AND, i)
}

}

impl<'input> OC_AndExpressionContextAttrs<'input> for OC_AndExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_AndExpression(&mut self,)
	-> Result<Rc<OC_AndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_AndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_oC_AndExpression);
        let mut _localctx: Rc<OC_AndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_NotExpression*/
			recog.base.set_state(889);
			recog.oC_NotExpression()?;

			recog.base.set_state(896);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(146,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(890);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(891);
					recog.base.match_token(Cypher_AND,&mut recog.err_handler)?;

					recog.base.set_state(892);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					/*InvokeRule oC_NotExpression*/
					recog.base.set_state(893);
					recog.oC_NotExpression()?;

					}
					} 
				}
				recog.base.set_state(898);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(146,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NotExpression ----------------
pub type OC_NotExpressionContextAll<'input> = OC_NotExpressionContext<'input>;


pub type OC_NotExpressionContext<'input> = BaseParserRuleContext<'input,OC_NotExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NotExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NotExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NotExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NotExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NotExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NotExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NotExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NotExpression }
}
antlr4rust::tid!{OC_NotExpressionContextExt<'a>}

impl<'input> OC_NotExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NotExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NotExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NotExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NotExpressionContextExt<'input>>{

fn oC_ComparisonExpression(&self) -> Option<Rc<OC_ComparisonExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NOT in current rule
fn NOT_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NOT, starting from 0.
/// Returns `None` if number of children corresponding to token NOT is less or equal than `i`.
fn NOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NOT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_NotExpressionContextAttrs<'input> for OC_NotExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NotExpression(&mut self,)
	-> Result<Rc<OC_NotExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NotExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_oC_NotExpression);
        let mut _localctx: Rc<OC_NotExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(905);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Cypher_NOT {
				{
				{
				recog.base.set_state(899);
				recog.base.match_token(Cypher_NOT,&mut recog.err_handler)?;

				recog.base.set_state(901);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(900);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
				}
				recog.base.set_state(907);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule oC_ComparisonExpression*/
			recog.base.set_state(908);
			recog.oC_ComparisonExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ComparisonExpression ----------------
pub type OC_ComparisonExpressionContextAll<'input> = OC_ComparisonExpressionContext<'input>;


pub type OC_ComparisonExpressionContext<'input> = BaseParserRuleContext<'input,OC_ComparisonExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ComparisonExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ComparisonExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ComparisonExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ComparisonExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ComparisonExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ComparisonExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ComparisonExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ComparisonExpression }
}
antlr4rust::tid!{OC_ComparisonExpressionContextExt<'a>}

impl<'input> OC_ComparisonExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ComparisonExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ComparisonExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ComparisonExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ComparisonExpressionContextExt<'input>>{

fn oC_AddOrSubtractExpression(&self) -> Option<Rc<OC_AddOrSubtractExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PartialComparisonExpression_all(&self) ->  Vec<Rc<OC_PartialComparisonExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PartialComparisonExpression(&self, i: usize) -> Option<Rc<OC_PartialComparisonExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_ComparisonExpressionContextAttrs<'input> for OC_ComparisonExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ComparisonExpression(&mut self,)
	-> Result<Rc<OC_ComparisonExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ComparisonExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_oC_ComparisonExpression);
        let mut _localctx: Rc<OC_ComparisonExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_AddOrSubtractExpression*/
			recog.base.set_state(910);
			recog.oC_AddOrSubtractExpression()?;

			recog.base.set_state(917);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(150,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(912);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(911);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PartialComparisonExpression*/
					recog.base.set_state(914);
					recog.oC_PartialComparisonExpression()?;

					}
					} 
				}
				recog.base.set_state(919);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(150,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_AddOrSubtractExpression ----------------
pub type OC_AddOrSubtractExpressionContextAll<'input> = OC_AddOrSubtractExpressionContext<'input>;


pub type OC_AddOrSubtractExpressionContext<'input> = BaseParserRuleContext<'input,OC_AddOrSubtractExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AddOrSubtractExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_AddOrSubtractExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AddOrSubtractExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_AddOrSubtractExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_AddOrSubtractExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_AddOrSubtractExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_AddOrSubtractExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_AddOrSubtractExpression }
}
antlr4rust::tid!{OC_AddOrSubtractExpressionContextExt<'a>}

impl<'input> OC_AddOrSubtractExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_AddOrSubtractExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_AddOrSubtractExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_AddOrSubtractExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_AddOrSubtractExpressionContextExt<'input>>{

fn oC_MultiplyDivideModuloExpression_all(&self) ->  Vec<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_MultiplyDivideModuloExpression(&self, i: usize) -> Option<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_AddOrSubtractExpressionContextAttrs<'input> for OC_AddOrSubtractExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_AddOrSubtractExpression(&mut self,)
	-> Result<Rc<OC_AddOrSubtractExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_AddOrSubtractExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_oC_AddOrSubtractExpression);
        let mut _localctx: Rc<OC_AddOrSubtractExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_MultiplyDivideModuloExpression*/
			recog.base.set_state(920);
			recog.oC_MultiplyDivideModuloExpression()?;

			recog.base.set_state(939);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(156,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(937);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(155,&mut recog.base)? {
						1 =>{
							{
							{
							recog.base.set_state(922);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(921);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(924);
							recog.base.match_token(Cypher_T__12,&mut recog.err_handler)?;

							recog.base.set_state(926);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(925);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_MultiplyDivideModuloExpression*/
							recog.base.set_state(928);
							recog.oC_MultiplyDivideModuloExpression()?;

							}
							}
						}
					,
						2 =>{
							{
							{
							recog.base.set_state(930);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(929);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(932);
							recog.base.match_token(Cypher_T__13,&mut recog.err_handler)?;

							recog.base.set_state(934);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(933);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_MultiplyDivideModuloExpression*/
							recog.base.set_state(936);
							recog.oC_MultiplyDivideModuloExpression()?;

							}
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(941);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(156,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_MultiplyDivideModuloExpression ----------------
pub type OC_MultiplyDivideModuloExpressionContextAll<'input> = OC_MultiplyDivideModuloExpressionContext<'input>;


pub type OC_MultiplyDivideModuloExpressionContext<'input> = BaseParserRuleContext<'input,OC_MultiplyDivideModuloExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MultiplyDivideModuloExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MultiplyDivideModuloExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MultiplyDivideModuloExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_MultiplyDivideModuloExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_MultiplyDivideModuloExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MultiplyDivideModuloExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_MultiplyDivideModuloExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_MultiplyDivideModuloExpression }
}
antlr4rust::tid!{OC_MultiplyDivideModuloExpressionContextExt<'a>}

impl<'input> OC_MultiplyDivideModuloExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MultiplyDivideModuloExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MultiplyDivideModuloExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MultiplyDivideModuloExpressionContextExt<'input>>{

fn oC_PowerOfExpression_all(&self) ->  Vec<Rc<OC_PowerOfExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PowerOfExpression(&self, i: usize) -> Option<Rc<OC_PowerOfExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_MultiplyDivideModuloExpressionContextAttrs<'input> for OC_MultiplyDivideModuloExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_MultiplyDivideModuloExpression(&mut self,)
	-> Result<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MultiplyDivideModuloExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_oC_MultiplyDivideModuloExpression);
        let mut _localctx: Rc<OC_MultiplyDivideModuloExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_PowerOfExpression*/
			recog.base.set_state(942);
			recog.oC_PowerOfExpression()?;

			recog.base.set_state(969);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(164,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(967);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(163,&mut recog.base)? {
						1 =>{
							{
							{
							recog.base.set_state(944);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(943);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(946);
							recog.base.match_token(Cypher_T__4,&mut recog.err_handler)?;

							recog.base.set_state(948);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(947);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_PowerOfExpression*/
							recog.base.set_state(950);
							recog.oC_PowerOfExpression()?;

							}
							}
						}
					,
						2 =>{
							{
							{
							recog.base.set_state(952);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(951);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(954);
							recog.base.match_token(Cypher_T__14,&mut recog.err_handler)?;

							recog.base.set_state(956);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(955);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_PowerOfExpression*/
							recog.base.set_state(958);
							recog.oC_PowerOfExpression()?;

							}
							}
						}
					,
						3 =>{
							{
							{
							recog.base.set_state(960);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(959);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(962);
							recog.base.match_token(Cypher_T__15,&mut recog.err_handler)?;

							recog.base.set_state(964);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(963);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_PowerOfExpression*/
							recog.base.set_state(966);
							recog.oC_PowerOfExpression()?;

							}
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(971);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(164,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PowerOfExpression ----------------
pub type OC_PowerOfExpressionContextAll<'input> = OC_PowerOfExpressionContext<'input>;


pub type OC_PowerOfExpressionContext<'input> = BaseParserRuleContext<'input,OC_PowerOfExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PowerOfExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PowerOfExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PowerOfExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PowerOfExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PowerOfExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PowerOfExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PowerOfExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PowerOfExpression }
}
antlr4rust::tid!{OC_PowerOfExpressionContextExt<'a>}

impl<'input> OC_PowerOfExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PowerOfExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PowerOfExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PowerOfExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PowerOfExpressionContextExt<'input>>{

fn oC_UnaryAddOrSubtractExpression_all(&self) ->  Vec<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_UnaryAddOrSubtractExpression(&self, i: usize) -> Option<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_PowerOfExpressionContextAttrs<'input> for OC_PowerOfExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PowerOfExpression(&mut self,)
	-> Result<Rc<OC_PowerOfExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PowerOfExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_oC_PowerOfExpression);
        let mut _localctx: Rc<OC_PowerOfExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_UnaryAddOrSubtractExpression*/
			recog.base.set_state(972);
			recog.oC_UnaryAddOrSubtractExpression()?;

			recog.base.set_state(983);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(167,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(974);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(973);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(976);
					recog.base.match_token(Cypher_T__16,&mut recog.err_handler)?;

					recog.base.set_state(978);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(977);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_UnaryAddOrSubtractExpression*/
					recog.base.set_state(980);
					recog.oC_UnaryAddOrSubtractExpression()?;

					}
					} 
				}
				recog.base.set_state(985);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(167,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_UnaryAddOrSubtractExpression ----------------
pub type OC_UnaryAddOrSubtractExpressionContextAll<'input> = OC_UnaryAddOrSubtractExpressionContext<'input>;


pub type OC_UnaryAddOrSubtractExpressionContext<'input> = BaseParserRuleContext<'input,OC_UnaryAddOrSubtractExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnaryAddOrSubtractExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_UnaryAddOrSubtractExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UnaryAddOrSubtractExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_UnaryAddOrSubtractExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_UnaryAddOrSubtractExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_UnaryAddOrSubtractExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_UnaryAddOrSubtractExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_UnaryAddOrSubtractExpression }
}
antlr4rust::tid!{OC_UnaryAddOrSubtractExpressionContextExt<'a>}

impl<'input> OC_UnaryAddOrSubtractExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_UnaryAddOrSubtractExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_UnaryAddOrSubtractExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_UnaryAddOrSubtractExpressionContextExt<'input>>{

fn oC_StringListNullOperatorExpression(&self) -> Option<Rc<OC_StringListNullOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_UnaryAddOrSubtractExpressionContextAttrs<'input> for OC_UnaryAddOrSubtractExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_UnaryAddOrSubtractExpression(&mut self,)
	-> Result<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_UnaryAddOrSubtractExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_oC_UnaryAddOrSubtractExpression);
        let mut _localctx: Rc<OC_UnaryAddOrSubtractExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(992);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Cypher_T__12 || _la==Cypher_T__13 {
				{
				{
				recog.base.set_state(986);
				_la = recog.base.input.la(1);
				if { !(_la==Cypher_T__12 || _la==Cypher_T__13) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				recog.base.set_state(988);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(987);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
				}
				recog.base.set_state(994);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule oC_StringListNullOperatorExpression*/
			recog.base.set_state(995);
			recog.oC_StringListNullOperatorExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_StringListNullOperatorExpression ----------------
pub type OC_StringListNullOperatorExpressionContextAll<'input> = OC_StringListNullOperatorExpressionContext<'input>;


pub type OC_StringListNullOperatorExpressionContext<'input> = BaseParserRuleContext<'input,OC_StringListNullOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StringListNullOperatorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_StringListNullOperatorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StringListNullOperatorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_StringListNullOperatorExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_StringListNullOperatorExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_StringListNullOperatorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_StringListNullOperatorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_StringListNullOperatorExpression }
}
antlr4rust::tid!{OC_StringListNullOperatorExpressionContextExt<'a>}

impl<'input> OC_StringListNullOperatorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_StringListNullOperatorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_StringListNullOperatorExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_StringListNullOperatorExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_StringListNullOperatorExpressionContextExt<'input>>{

fn oC_PropertyOrLabelsExpression(&self) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_StringOperatorExpression_all(&self) ->  Vec<Rc<OC_StringOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_StringOperatorExpression(&self, i: usize) -> Option<Rc<OC_StringOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_ListOperatorExpression_all(&self) ->  Vec<Rc<OC_ListOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_ListOperatorExpression(&self, i: usize) -> Option<Rc<OC_ListOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_NullOperatorExpression_all(&self) ->  Vec<Rc<OC_NullOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_NullOperatorExpression(&self, i: usize) -> Option<Rc<OC_NullOperatorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_StringListNullOperatorExpressionContextAttrs<'input> for OC_StringListNullOperatorExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_StringListNullOperatorExpression(&mut self,)
	-> Result<Rc<OC_StringListNullOperatorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_StringListNullOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_oC_StringListNullOperatorExpression);
        let mut _localctx: Rc<OC_StringListNullOperatorExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_PropertyOrLabelsExpression*/
			recog.base.set_state(997);
			recog.oC_PropertyOrLabelsExpression()?;

			recog.base.set_state(1003);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(171,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(1001);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(170,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule oC_StringOperatorExpression*/
							recog.base.set_state(998);
							recog.oC_StringOperatorExpression()?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule oC_ListOperatorExpression*/
							recog.base.set_state(999);
							recog.oC_ListOperatorExpression()?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule oC_NullOperatorExpression*/
							recog.base.set_state(1000);
							recog.oC_NullOperatorExpression()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1005);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(171,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ListOperatorExpression ----------------
pub type OC_ListOperatorExpressionContextAll<'input> = OC_ListOperatorExpressionContext<'input>;


pub type OC_ListOperatorExpressionContext<'input> = BaseParserRuleContext<'input,OC_ListOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListOperatorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ListOperatorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ListOperatorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ListOperatorExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ListOperatorExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ListOperatorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ListOperatorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListOperatorExpression }
}
antlr4rust::tid!{OC_ListOperatorExpressionContextExt<'a>}

impl<'input> OC_ListOperatorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ListOperatorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ListOperatorExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ListOperatorExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ListOperatorExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_IN, 0)
}
fn oC_PropertyOrLabelsExpression(&self) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_ListOperatorExpressionContextAttrs<'input> for OC_ListOperatorExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ListOperatorExpression(&mut self,)
	-> Result<Rc<OC_ListOperatorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ListOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_oC_ListOperatorExpression);
        let mut _localctx: Rc<OC_ListOperatorExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1031);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(177,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(1006);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1007);
					recog.base.match_token(Cypher_IN,&mut recog.err_handler)?;

					recog.base.set_state(1009);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1008);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PropertyOrLabelsExpression*/
					recog.base.set_state(1011);
					recog.oC_PropertyOrLabelsExpression()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(1013);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1012);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1015);
					recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1016);
					recog.oC_Expression()?;

					recog.base.set_state(1017);
					recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					recog.base.set_state(1020);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1019);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1022);
					recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

					recog.base.set_state(1024);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 83910976) != 0) || _la==Cypher_ALL || _la==Cypher_NOT || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & 101163519) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
						{
						/*InvokeRule oC_Expression*/
						recog.base.set_state(1023);
						recog.oC_Expression()?;

						}
					}

					recog.base.set_state(1026);
					recog.base.match_token(Cypher_T__11,&mut recog.err_handler)?;

					recog.base.set_state(1028);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 83910976) != 0) || _la==Cypher_ALL || _la==Cypher_NOT || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & 101163519) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
						{
						/*InvokeRule oC_Expression*/
						recog.base.set_state(1027);
						recog.oC_Expression()?;

						}
					}

					recog.base.set_state(1030);
					recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_StringOperatorExpression ----------------
pub type OC_StringOperatorExpressionContextAll<'input> = OC_StringOperatorExpressionContext<'input>;


pub type OC_StringOperatorExpressionContext<'input> = BaseParserRuleContext<'input,OC_StringOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StringOperatorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_StringOperatorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StringOperatorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_StringOperatorExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_StringOperatorExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_StringOperatorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_StringOperatorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_StringOperatorExpression }
}
antlr4rust::tid!{OC_StringOperatorExpressionContextExt<'a>}

impl<'input> OC_StringOperatorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_StringOperatorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_StringOperatorExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_StringOperatorExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_StringOperatorExpressionContextExt<'input>>{

fn oC_PropertyOrLabelsExpression(&self) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token STARTS
/// Returns `None` if there is no child corresponding to token STARTS
fn STARTS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_STARTS, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDS
/// Returns `None` if there is no child corresponding to token ENDS
fn ENDS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ENDS, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CONTAINS, 0)
}

}

impl<'input> OC_StringOperatorExpressionContextAttrs<'input> for OC_StringOperatorExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_StringOperatorExpression(&mut self,)
	-> Result<Rc<OC_StringOperatorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_StringOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_oC_StringOperatorExpression);
        let mut _localctx: Rc<OC_StringOperatorExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1043);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(178,&mut recog.base)? {
				1 =>{
					{
					{
					recog.base.set_state(1033);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1034);
					recog.base.match_token(Cypher_STARTS,&mut recog.err_handler)?;

					recog.base.set_state(1035);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1036);
					recog.base.match_token(Cypher_WITH,&mut recog.err_handler)?;

					}
					}
				}
			,
				2 =>{
					{
					{
					recog.base.set_state(1037);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1038);
					recog.base.match_token(Cypher_ENDS,&mut recog.err_handler)?;

					recog.base.set_state(1039);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1040);
					recog.base.match_token(Cypher_WITH,&mut recog.err_handler)?;

					}
					}
				}
			,
				3 =>{
					{
					{
					recog.base.set_state(1041);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1042);
					recog.base.match_token(Cypher_CONTAINS,&mut recog.err_handler)?;

					}
					}
				}

				_ => {}
			}
			recog.base.set_state(1046);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1045);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_PropertyOrLabelsExpression*/
			recog.base.set_state(1048);
			recog.oC_PropertyOrLabelsExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NullOperatorExpression ----------------
pub type OC_NullOperatorExpressionContextAll<'input> = OC_NullOperatorExpressionContext<'input>;


pub type OC_NullOperatorExpressionContext<'input> = BaseParserRuleContext<'input,OC_NullOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NullOperatorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NullOperatorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NullOperatorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NullOperatorExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NullOperatorExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NullOperatorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NullOperatorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NullOperatorExpression }
}
antlr4rust::tid!{OC_NullOperatorExpressionContextExt<'a>}

impl<'input> OC_NullOperatorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NullOperatorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NullOperatorExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NullOperatorExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NullOperatorExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NOT, 0)
}

}

impl<'input> OC_NullOperatorExpressionContextAttrs<'input> for OC_NullOperatorExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NullOperatorExpression(&mut self,)
	-> Result<Rc<OC_NullOperatorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NullOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_oC_NullOperatorExpression);
        let mut _localctx: Rc<OC_NullOperatorExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1060);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(180,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(1050);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1051);
					recog.base.match_token(Cypher_IS,&mut recog.err_handler)?;

					recog.base.set_state(1052);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1053);
					recog.base.match_token(Cypher_NULL,&mut recog.err_handler)?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(1054);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1055);
					recog.base.match_token(Cypher_IS,&mut recog.err_handler)?;

					recog.base.set_state(1056);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1057);
					recog.base.match_token(Cypher_NOT,&mut recog.err_handler)?;

					recog.base.set_state(1058);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					recog.base.set_state(1059);
					recog.base.match_token(Cypher_NULL,&mut recog.err_handler)?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PropertyOrLabelsExpression ----------------
pub type OC_PropertyOrLabelsExpressionContextAll<'input> = OC_PropertyOrLabelsExpressionContext<'input>;


pub type OC_PropertyOrLabelsExpressionContext<'input> = BaseParserRuleContext<'input,OC_PropertyOrLabelsExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyOrLabelsExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PropertyOrLabelsExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyOrLabelsExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PropertyOrLabelsExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PropertyOrLabelsExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PropertyOrLabelsExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PropertyOrLabelsExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyOrLabelsExpression }
}
antlr4rust::tid!{OC_PropertyOrLabelsExpressionContextExt<'a>}

impl<'input> OC_PropertyOrLabelsExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PropertyOrLabelsExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PropertyOrLabelsExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PropertyOrLabelsExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PropertyOrLabelsExpressionContextExt<'input>>{

fn oC_Atom(&self) -> Option<Rc<OC_AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PropertyLookup_all(&self) ->  Vec<Rc<OC_PropertyLookupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PropertyLookup(&self, i: usize) -> Option<Rc<OC_PropertyLookupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_PropertyOrLabelsExpressionContextAttrs<'input> for OC_PropertyOrLabelsExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PropertyOrLabelsExpression(&mut self,)
	-> Result<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PropertyOrLabelsExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_oC_PropertyOrLabelsExpression);
        let mut _localctx: Rc<OC_PropertyOrLabelsExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Atom*/
			recog.base.set_state(1062);
			recog.oC_Atom()?;

			recog.base.set_state(1069);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(182,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(1064);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1063);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PropertyLookup*/
					recog.base.set_state(1066);
					recog.oC_PropertyLookup()?;

					}
					} 
				}
				recog.base.set_state(1071);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(182,&mut recog.base)?;
			}
			recog.base.set_state(1076);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(184,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1073);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1072);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_NodeLabels*/
					recog.base.set_state(1075);
					recog.oC_NodeLabels()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Atom ----------------
pub type OC_AtomContextAll<'input> = OC_AtomContext<'input>;


pub type OC_AtomContext<'input> = BaseParserRuleContext<'input,OC_AtomContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_AtomContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Atom(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Atom }
}
antlr4rust::tid!{OC_AtomContextExt<'a>}

impl<'input> OC_AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_AtomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_AtomContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_AtomContextExt<'input>>{

fn oC_Literal(&self) -> Option<Rc<OC_LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Parameter(&self) -> Option<Rc<OC_ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_CaseExpression(&self) -> Option<Rc<OC_CaseExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_COUNT, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_ListComprehension(&self) -> Option<Rc<OC_ListComprehensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PatternComprehension(&self) -> Option<Rc<OC_PatternComprehensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ALL
/// Returns `None` if there is no child corresponding to token ALL
fn ALL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ALL, 0)
}
fn oC_FilterExpression(&self) -> Option<Rc<OC_FilterExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token NONE
/// Returns `None` if there is no child corresponding to token NONE
fn NONE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NONE, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE
/// Returns `None` if there is no child corresponding to token SINGLE
fn SINGLE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SINGLE, 0)
}
fn oC_RelationshipsPattern(&self) -> Option<Rc<OC_RelationshipsPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ParenthesizedExpression(&self) -> Option<Rc<OC_ParenthesizedExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_FunctionInvocation(&self) -> Option<Rc<OC_FunctionInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_AtomContextAttrs<'input> for OC_AtomContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Atom(&mut self,)
	-> Result<Rc<OC_AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_oC_Atom);
        let mut _localctx: Rc<OC_AtomContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1156);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(200,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_Literal*/
					recog.base.set_state(1078);
					recog.oC_Literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_Parameter*/
					recog.base.set_state(1079);
					recog.oC_Parameter()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule oC_CaseExpression*/
					recog.base.set_state(1080);
					recog.oC_CaseExpression()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					{
					recog.base.set_state(1081);
					recog.base.match_token(Cypher_COUNT,&mut recog.err_handler)?;

					recog.base.set_state(1083);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1082);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1085);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					recog.base.set_state(1087);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1086);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1089);
					recog.base.match_token(Cypher_T__4,&mut recog.err_handler)?;

					recog.base.set_state(1091);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1090);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1093);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule oC_ListComprehension*/
					recog.base.set_state(1094);
					recog.oC_ListComprehension()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule oC_PatternComprehension*/
					recog.base.set_state(1095);
					recog.oC_PatternComprehension()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					{
					recog.base.set_state(1096);
					recog.base.match_token(Cypher_ALL,&mut recog.err_handler)?;

					recog.base.set_state(1098);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1097);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1100);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					recog.base.set_state(1102);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1101);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_FilterExpression*/
					recog.base.set_state(1104);
					recog.oC_FilterExpression()?;

					recog.base.set_state(1106);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1105);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1108);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					{
					recog.base.set_state(1110);
					recog.base.match_token(Cypher_ANY,&mut recog.err_handler)?;

					recog.base.set_state(1112);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1111);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1114);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					recog.base.set_state(1116);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1115);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_FilterExpression*/
					recog.base.set_state(1118);
					recog.oC_FilterExpression()?;

					recog.base.set_state(1120);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1119);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1122);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					{
					recog.base.set_state(1124);
					recog.base.match_token(Cypher_NONE,&mut recog.err_handler)?;

					recog.base.set_state(1126);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1125);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1128);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					recog.base.set_state(1130);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1129);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_FilterExpression*/
					recog.base.set_state(1132);
					recog.oC_FilterExpression()?;

					recog.base.set_state(1134);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1133);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1136);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					{
					recog.base.set_state(1138);
					recog.base.match_token(Cypher_SINGLE,&mut recog.err_handler)?;

					recog.base.set_state(1140);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1139);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1142);
					recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

					recog.base.set_state(1144);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1143);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_FilterExpression*/
					recog.base.set_state(1146);
					recog.oC_FilterExpression()?;

					recog.base.set_state(1148);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1147);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1150);
					recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

					}
					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule oC_RelationshipsPattern*/
					recog.base.set_state(1152);
					recog.oC_RelationshipsPattern()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule oC_ParenthesizedExpression*/
					recog.base.set_state(1153);
					recog.oC_ParenthesizedExpression()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					/*InvokeRule oC_FunctionInvocation*/
					recog.base.set_state(1154);
					recog.oC_FunctionInvocation()?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14)?;
					recog.base.enter_outer_alt(None, 14)?;
					{
					/*InvokeRule oC_Variable*/
					recog.base.set_state(1155);
					recog.oC_Variable()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Literal ----------------
pub type OC_LiteralContextAll<'input> = OC_LiteralContext<'input>;


pub type OC_LiteralContext<'input> = BaseParserRuleContext<'input,OC_LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Literal(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Literal(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Literal }
}
antlr4rust::tid!{OC_LiteralContextExt<'a>}

impl<'input> OC_LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_LiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_LiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_LiteralContextExt<'input>>{

fn oC_NumberLiteral(&self) -> Option<Rc<OC_NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_StringLiteral, 0)
}
fn oC_BooleanLiteral(&self) -> Option<Rc<OC_BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NULL, 0)
}
fn oC_MapLiteral(&self) -> Option<Rc<OC_MapLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ListLiteral(&self) -> Option<Rc<OC_ListLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_LiteralContextAttrs<'input> for OC_LiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Literal(&mut self,)
	-> Result<Rc<OC_LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_oC_Literal);
        let mut _localctx: Rc<OC_LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1164);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_HexInteger |Cypher_DecimalInteger |Cypher_OctalInteger |Cypher_ExponentDecimalReal |
			Cypher_RegularDecimalReal 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_NumberLiteral*/
					recog.base.set_state(1158);
					recog.oC_NumberLiteral()?;

					}
				}

			Cypher_StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1159);
					recog.base.match_token(Cypher_StringLiteral,&mut recog.err_handler)?;

					}
				}

			Cypher_TRUE |Cypher_FALSE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule oC_BooleanLiteral*/
					recog.base.set_state(1160);
					recog.oC_BooleanLiteral()?;

					}
				}

			Cypher_NULL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1161);
					recog.base.match_token(Cypher_NULL,&mut recog.err_handler)?;

					}
				}

			Cypher_T__23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule oC_MapLiteral*/
					recog.base.set_state(1162);
					recog.oC_MapLiteral()?;

					}
				}

			Cypher_T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule oC_ListLiteral*/
					recog.base.set_state(1163);
					recog.oC_ListLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_BooleanLiteral ----------------
pub type OC_BooleanLiteralContextAll<'input> = OC_BooleanLiteralContext<'input>;


pub type OC_BooleanLiteralContext<'input> = BaseParserRuleContext<'input,OC_BooleanLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_BooleanLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_BooleanLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_BooleanLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_BooleanLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_BooleanLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_BooleanLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_BooleanLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_BooleanLiteral }
}
antlr4rust::tid!{OC_BooleanLiteralContextExt<'a>}

impl<'input> OC_BooleanLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_BooleanLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_BooleanLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_BooleanLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_BooleanLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_FALSE, 0)
}

}

impl<'input> OC_BooleanLiteralContextAttrs<'input> for OC_BooleanLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_BooleanLiteral(&mut self,)
	-> Result<Rc<OC_BooleanLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_BooleanLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_oC_BooleanLiteral);
        let mut _localctx: Rc<OC_BooleanLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1166);
			_la = recog.base.input.la(1);
			if { !(_la==Cypher_TRUE || _la==Cypher_FALSE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ListLiteral ----------------
pub type OC_ListLiteralContextAll<'input> = OC_ListLiteralContext<'input>;


pub type OC_ListLiteralContext<'input> = BaseParserRuleContext<'input,OC_ListLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ListLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ListLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ListLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ListLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ListLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ListLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListLiteral }
}
antlr4rust::tid!{OC_ListLiteralContextExt<'a>}

impl<'input> OC_ListLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ListLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ListLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ListLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ListLiteralContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_ListLiteralContextAttrs<'input> for OC_ListLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ListLiteral(&mut self,)
	-> Result<Rc<OC_ListLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ListLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_oC_ListLiteral);
        let mut _localctx: Rc<OC_ListLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1168);
			recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

			recog.base.set_state(1170);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1169);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1189);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 83910976) != 0) || _la==Cypher_ALL || _la==Cypher_NOT || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & 101163519) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Expression*/
				recog.base.set_state(1172);
				recog.oC_Expression()?;

				recog.base.set_state(1174);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1173);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1186);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Cypher_T__1 {
					{
					{
					recog.base.set_state(1176);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(1178);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1177);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1180);
					recog.oC_Expression()?;

					recog.base.set_state(1182);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1181);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
					}
					recog.base.set_state(1188);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1191);
			recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PartialComparisonExpression ----------------
pub type OC_PartialComparisonExpressionContextAll<'input> = OC_PartialComparisonExpressionContext<'input>;


pub type OC_PartialComparisonExpressionContext<'input> = BaseParserRuleContext<'input,OC_PartialComparisonExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PartialComparisonExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PartialComparisonExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PartialComparisonExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PartialComparisonExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PartialComparisonExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PartialComparisonExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PartialComparisonExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PartialComparisonExpression }
}
antlr4rust::tid!{OC_PartialComparisonExpressionContextExt<'a>}

impl<'input> OC_PartialComparisonExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PartialComparisonExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PartialComparisonExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PartialComparisonExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PartialComparisonExpressionContextExt<'input>>{

fn oC_AddOrSubtractExpression(&self) -> Option<Rc<OC_AddOrSubtractExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_PartialComparisonExpressionContextAttrs<'input> for OC_PartialComparisonExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PartialComparisonExpression(&mut self,)
	-> Result<Rc<OC_PartialComparisonExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PartialComparisonExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_oC_PartialComparisonExpression);
        let mut _localctx: Rc<OC_PartialComparisonExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1223);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_T__2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					recog.base.set_state(1193);
					recog.base.match_token(Cypher_T__2,&mut recog.err_handler)?;

					recog.base.set_state(1195);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1194);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1197);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

			Cypher_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					{
					recog.base.set_state(1198);
					recog.base.match_token(Cypher_T__17,&mut recog.err_handler)?;

					recog.base.set_state(1200);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1199);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1202);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

			Cypher_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					{
					recog.base.set_state(1203);
					recog.base.match_token(Cypher_T__18,&mut recog.err_handler)?;

					recog.base.set_state(1205);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1204);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1207);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

			Cypher_T__19 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					{
					recog.base.set_state(1208);
					recog.base.match_token(Cypher_T__19,&mut recog.err_handler)?;

					recog.base.set_state(1210);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1209);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1212);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

			Cypher_T__20 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					{
					recog.base.set_state(1213);
					recog.base.match_token(Cypher_T__20,&mut recog.err_handler)?;

					recog.base.set_state(1215);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1214);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1217);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

			Cypher_T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					{
					recog.base.set_state(1218);
					recog.base.match_token(Cypher_T__21,&mut recog.err_handler)?;

					recog.base.set_state(1220);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1219);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_AddOrSubtractExpression*/
					recog.base.set_state(1222);
					recog.oC_AddOrSubtractExpression()?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ParenthesizedExpression ----------------
pub type OC_ParenthesizedExpressionContextAll<'input> = OC_ParenthesizedExpressionContext<'input>;


pub type OC_ParenthesizedExpressionContext<'input> = BaseParserRuleContext<'input,OC_ParenthesizedExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ParenthesizedExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ParenthesizedExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ParenthesizedExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ParenthesizedExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ParenthesizedExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ParenthesizedExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ParenthesizedExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ParenthesizedExpression }
}
antlr4rust::tid!{OC_ParenthesizedExpressionContextExt<'a>}

impl<'input> OC_ParenthesizedExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ParenthesizedExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ParenthesizedExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ParenthesizedExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ParenthesizedExpressionContextExt<'input>>{

fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_ParenthesizedExpressionContextAttrs<'input> for OC_ParenthesizedExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ParenthesizedExpression(&mut self,)
	-> Result<Rc<OC_ParenthesizedExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ParenthesizedExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_oC_ParenthesizedExpression);
        let mut _localctx: Rc<OC_ParenthesizedExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1225);
			recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

			recog.base.set_state(1227);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1226);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(1229);
			recog.oC_Expression()?;

			recog.base.set_state(1231);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1230);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1233);
			recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RelationshipsPattern ----------------
pub type OC_RelationshipsPatternContextAll<'input> = OC_RelationshipsPatternContext<'input>;


pub type OC_RelationshipsPatternContext<'input> = BaseParserRuleContext<'input,OC_RelationshipsPatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipsPatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RelationshipsPatternContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelationshipsPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RelationshipsPattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RelationshipsPattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipsPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RelationshipsPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipsPattern }
}
antlr4rust::tid!{OC_RelationshipsPatternContextExt<'a>}

impl<'input> OC_RelationshipsPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RelationshipsPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RelationshipsPatternContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RelationshipsPatternContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RelationshipsPatternContextExt<'input>>{

fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PatternElementChain_all(&self) ->  Vec<Rc<OC_PatternElementChainContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PatternElementChain(&self, i: usize) -> Option<Rc<OC_PatternElementChainContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_RelationshipsPatternContextAttrs<'input> for OC_RelationshipsPatternContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RelationshipsPattern(&mut self,)
	-> Result<Rc<OC_RelationshipsPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RelationshipsPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_oC_RelationshipsPattern);
        let mut _localctx: Rc<OC_RelationshipsPatternContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_NodePattern*/
			recog.base.set_state(1235);
			recog.oC_NodePattern()?;

			recog.base.set_state(1240); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(1237);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1236);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PatternElementChain*/
					recog.base.set_state(1239);
					recog.oC_PatternElementChain()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(1242); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(218,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_FilterExpression ----------------
pub type OC_FilterExpressionContextAll<'input> = OC_FilterExpressionContext<'input>;


pub type OC_FilterExpressionContext<'input> = BaseParserRuleContext<'input,OC_FilterExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FilterExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_FilterExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_FilterExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_FilterExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_FilterExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_FilterExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_FilterExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_FilterExpression }
}
antlr4rust::tid!{OC_FilterExpressionContextExt<'a>}

impl<'input> OC_FilterExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_FilterExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_FilterExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_FilterExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_FilterExpressionContextExt<'input>>{

fn oC_IdInColl(&self) -> Option<Rc<OC_IdInCollContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_FilterExpressionContextAttrs<'input> for OC_FilterExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_FilterExpression(&mut self,)
	-> Result<Rc<OC_FilterExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_FilterExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_oC_FilterExpression);
        let mut _localctx: Rc<OC_FilterExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_IdInColl*/
			recog.base.set_state(1244);
			recog.oC_IdInColl()?;

			recog.base.set_state(1249);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(220,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1246);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1245);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Where*/
					recog.base.set_state(1248);
					recog.oC_Where()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_IdInColl ----------------
pub type OC_IdInCollContextAll<'input> = OC_IdInCollContext<'input>;


pub type OC_IdInCollContext<'input> = BaseParserRuleContext<'input,OC_IdInCollContextExt<'input>>;

#[derive(Clone)]
pub struct OC_IdInCollContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_IdInCollContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_IdInCollContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_IdInColl(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_IdInColl(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_IdInCollContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_IdInColl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_IdInColl }
}
antlr4rust::tid!{OC_IdInCollContextExt<'a>}

impl<'input> OC_IdInCollContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_IdInCollContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_IdInCollContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_IdInCollContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_IdInCollContextExt<'input>>{

fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_IN, 0)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_IdInCollContextAttrs<'input> for OC_IdInCollContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_IdInColl(&mut self,)
	-> Result<Rc<OC_IdInCollContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_IdInCollContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_oC_IdInColl);
        let mut _localctx: Rc<OC_IdInCollContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Variable*/
			recog.base.set_state(1251);
			recog.oC_Variable()?;

			recog.base.set_state(1252);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			recog.base.set_state(1253);
			recog.base.match_token(Cypher_IN,&mut recog.err_handler)?;

			recog.base.set_state(1254);
			recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

			/*InvokeRule oC_Expression*/
			recog.base.set_state(1255);
			recog.oC_Expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_FunctionInvocation ----------------
pub type OC_FunctionInvocationContextAll<'input> = OC_FunctionInvocationContext<'input>;


pub type OC_FunctionInvocationContext<'input> = BaseParserRuleContext<'input,OC_FunctionInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FunctionInvocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_FunctionInvocationContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_FunctionInvocationContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_FunctionInvocation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_FunctionInvocation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_FunctionInvocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_FunctionInvocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_FunctionInvocation }
}
antlr4rust::tid!{OC_FunctionInvocationContextExt<'a>}

impl<'input> OC_FunctionInvocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_FunctionInvocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_FunctionInvocationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_FunctionInvocationContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_FunctionInvocationContextExt<'input>>{

fn oC_FunctionName(&self) -> Option<Rc<OC_FunctionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token DISTINCT
/// Returns `None` if there is no child corresponding to token DISTINCT
fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DISTINCT, 0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_FunctionInvocationContextAttrs<'input> for OC_FunctionInvocationContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_FunctionInvocation(&mut self,)
	-> Result<Rc<OC_FunctionInvocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_FunctionInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_oC_FunctionInvocation);
        let mut _localctx: Rc<OC_FunctionInvocationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_FunctionName*/
			recog.base.set_state(1257);
			recog.oC_FunctionName()?;

			recog.base.set_state(1259);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1258);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1261);
			recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

			recog.base.set_state(1263);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1262);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1269);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_DISTINCT {
				{
				recog.base.set_state(1265);
				recog.base.match_token(Cypher_DISTINCT,&mut recog.err_handler)?;

				recog.base.set_state(1267);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1266);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1288);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 83910976) != 0) || _la==Cypher_ALL || _la==Cypher_NOT || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & 101163519) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Expression*/
				recog.base.set_state(1271);
				recog.oC_Expression()?;

				recog.base.set_state(1273);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1272);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1285);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Cypher_T__1 {
					{
					{
					recog.base.set_state(1275);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(1277);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1276);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1279);
					recog.oC_Expression()?;

					recog.base.set_state(1281);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1280);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
					}
					recog.base.set_state(1287);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1290);
			recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_FunctionName ----------------
pub type OC_FunctionNameContextAll<'input> = OC_FunctionNameContext<'input>;


pub type OC_FunctionNameContext<'input> = BaseParserRuleContext<'input,OC_FunctionNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FunctionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_FunctionNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_FunctionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_FunctionName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_FunctionName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_FunctionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_FunctionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_FunctionName }
}
antlr4rust::tid!{OC_FunctionNameContextExt<'a>}

impl<'input> OC_FunctionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_FunctionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_FunctionNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_FunctionNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_FunctionNameContextExt<'input>>{

fn oC_Namespace(&self) -> Option<Rc<OC_NamespaceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EXISTS
/// Returns `None` if there is no child corresponding to token EXISTS
fn EXISTS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_EXISTS, 0)
}

}

impl<'input> OC_FunctionNameContextAttrs<'input> for OC_FunctionNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_FunctionName(&mut self,)
	-> Result<Rc<OC_FunctionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_FunctionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_oC_FunctionName);
        let mut _localctx: Rc<OC_FunctionNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1296);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_COUNT |Cypher_ANY |Cypher_NONE |Cypher_SINGLE |Cypher_HexLetter |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule oC_Namespace*/
					recog.base.set_state(1292);
					recog.oC_Namespace()?;

					/*InvokeRule oC_SymbolicName*/
					recog.base.set_state(1293);
					recog.oC_SymbolicName()?;

					}
					}
				}

			Cypher_EXISTS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1295);
					recog.base.match_token(Cypher_EXISTS,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ExplicitProcedureInvocation ----------------
pub type OC_ExplicitProcedureInvocationContextAll<'input> = OC_ExplicitProcedureInvocationContext<'input>;


pub type OC_ExplicitProcedureInvocationContext<'input> = BaseParserRuleContext<'input,OC_ExplicitProcedureInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ExplicitProcedureInvocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ExplicitProcedureInvocationContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ExplicitProcedureInvocationContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ExplicitProcedureInvocation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ExplicitProcedureInvocation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ExplicitProcedureInvocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ExplicitProcedureInvocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ExplicitProcedureInvocation }
}
antlr4rust::tid!{OC_ExplicitProcedureInvocationContextExt<'a>}

impl<'input> OC_ExplicitProcedureInvocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ExplicitProcedureInvocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ExplicitProcedureInvocationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ExplicitProcedureInvocationContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ExplicitProcedureInvocationContextExt<'input>>{

fn oC_ProcedureName(&self) -> Option<Rc<OC_ProcedureNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_ExplicitProcedureInvocationContextAttrs<'input> for OC_ExplicitProcedureInvocationContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ExplicitProcedureInvocation(&mut self,)
	-> Result<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ExplicitProcedureInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_oC_ExplicitProcedureInvocation);
        let mut _localctx: Rc<OC_ExplicitProcedureInvocationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_ProcedureName*/
			recog.base.set_state(1298);
			recog.oC_ProcedureName()?;

			recog.base.set_state(1300);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1299);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1302);
			recog.base.match_token(Cypher_T__5,&mut recog.err_handler)?;

			recog.base.set_state(1304);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1303);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1323);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 83910976) != 0) || _la==Cypher_ALL || _la==Cypher_NOT || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & 101163519) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Expression*/
				recog.base.set_state(1306);
				recog.oC_Expression()?;

				recog.base.set_state(1308);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1307);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1320);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Cypher_T__1 {
					{
					{
					recog.base.set_state(1310);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(1312);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1311);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1314);
					recog.oC_Expression()?;

					recog.base.set_state(1316);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1315);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
					}
					recog.base.set_state(1322);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1325);
			recog.base.match_token(Cypher_T__6,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ImplicitProcedureInvocation ----------------
pub type OC_ImplicitProcedureInvocationContextAll<'input> = OC_ImplicitProcedureInvocationContext<'input>;


pub type OC_ImplicitProcedureInvocationContext<'input> = BaseParserRuleContext<'input,OC_ImplicitProcedureInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ImplicitProcedureInvocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ImplicitProcedureInvocationContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ImplicitProcedureInvocationContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ImplicitProcedureInvocation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ImplicitProcedureInvocation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ImplicitProcedureInvocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ImplicitProcedureInvocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ImplicitProcedureInvocation }
}
antlr4rust::tid!{OC_ImplicitProcedureInvocationContextExt<'a>}

impl<'input> OC_ImplicitProcedureInvocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ImplicitProcedureInvocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ImplicitProcedureInvocationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ImplicitProcedureInvocationContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ImplicitProcedureInvocationContextExt<'input>>{

fn oC_ProcedureName(&self) -> Option<Rc<OC_ProcedureNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ImplicitProcedureInvocationContextAttrs<'input> for OC_ImplicitProcedureInvocationContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ImplicitProcedureInvocation(&mut self,)
	-> Result<Rc<OC_ImplicitProcedureInvocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ImplicitProcedureInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_oC_ImplicitProcedureInvocation);
        let mut _localctx: Rc<OC_ImplicitProcedureInvocationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_ProcedureName*/
			recog.base.set_state(1327);
			recog.oC_ProcedureName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ProcedureResultField ----------------
pub type OC_ProcedureResultFieldContextAll<'input> = OC_ProcedureResultFieldContext<'input>;


pub type OC_ProcedureResultFieldContext<'input> = BaseParserRuleContext<'input,OC_ProcedureResultFieldContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProcedureResultFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ProcedureResultFieldContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProcedureResultFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ProcedureResultField(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ProcedureResultField(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ProcedureResultFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ProcedureResultField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProcedureResultField }
}
antlr4rust::tid!{OC_ProcedureResultFieldContextExt<'a>}

impl<'input> OC_ProcedureResultFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ProcedureResultFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ProcedureResultFieldContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ProcedureResultFieldContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ProcedureResultFieldContextExt<'input>>{

fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ProcedureResultFieldContextAttrs<'input> for OC_ProcedureResultFieldContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ProcedureResultField(&mut self,)
	-> Result<Rc<OC_ProcedureResultFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ProcedureResultFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_oC_ProcedureResultField);
        let mut _localctx: Rc<OC_ProcedureResultFieldContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SymbolicName*/
			recog.base.set_state(1329);
			recog.oC_SymbolicName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ProcedureName ----------------
pub type OC_ProcedureNameContextAll<'input> = OC_ProcedureNameContext<'input>;


pub type OC_ProcedureNameContext<'input> = BaseParserRuleContext<'input,OC_ProcedureNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProcedureNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ProcedureNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProcedureNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ProcedureName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ProcedureName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ProcedureNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ProcedureName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProcedureName }
}
antlr4rust::tid!{OC_ProcedureNameContextExt<'a>}

impl<'input> OC_ProcedureNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ProcedureNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ProcedureNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ProcedureNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ProcedureNameContextExt<'input>>{

fn oC_Namespace(&self) -> Option<Rc<OC_NamespaceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ProcedureNameContextAttrs<'input> for OC_ProcedureNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ProcedureName(&mut self,)
	-> Result<Rc<OC_ProcedureNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ProcedureNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_oC_ProcedureName);
        let mut _localctx: Rc<OC_ProcedureNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Namespace*/
			recog.base.set_state(1331);
			recog.oC_Namespace()?;

			/*InvokeRule oC_SymbolicName*/
			recog.base.set_state(1332);
			recog.oC_SymbolicName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Namespace ----------------
pub type OC_NamespaceContextAll<'input> = OC_NamespaceContext<'input>;


pub type OC_NamespaceContext<'input> = BaseParserRuleContext<'input,OC_NamespaceContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NamespaceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NamespaceContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NamespaceContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Namespace(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Namespace(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NamespaceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Namespace }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Namespace }
}
antlr4rust::tid!{OC_NamespaceContextExt<'a>}

impl<'input> OC_NamespaceContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NamespaceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NamespaceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NamespaceContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NamespaceContextExt<'input>>{

fn oC_SymbolicName_all(&self) ->  Vec<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_SymbolicName(&self, i: usize) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_NamespaceContextAttrs<'input> for OC_NamespaceContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Namespace(&mut self,)
	-> Result<Rc<OC_NamespaceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NamespaceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_oC_Namespace);
        let mut _localctx: Rc<OC_NamespaceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1339);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(238,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule oC_SymbolicName*/
					recog.base.set_state(1334);
					recog.oC_SymbolicName()?;

					recog.base.set_state(1335);
					recog.base.match_token(Cypher_T__22,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(1341);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(238,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ListComprehension ----------------
pub type OC_ListComprehensionContextAll<'input> = OC_ListComprehensionContext<'input>;


pub type OC_ListComprehensionContext<'input> = BaseParserRuleContext<'input,OC_ListComprehensionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListComprehensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ListComprehensionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ListComprehensionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ListComprehension(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ListComprehension(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ListComprehensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ListComprehension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListComprehension }
}
antlr4rust::tid!{OC_ListComprehensionContextExt<'a>}

impl<'input> OC_ListComprehensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ListComprehensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ListComprehensionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ListComprehensionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ListComprehensionContextExt<'input>>{

fn oC_FilterExpression(&self) -> Option<Rc<OC_FilterExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_ListComprehensionContextAttrs<'input> for OC_ListComprehensionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ListComprehension(&mut self,)
	-> Result<Rc<OC_ListComprehensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ListComprehensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_oC_ListComprehension);
        let mut _localctx: Rc<OC_ListComprehensionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1342);
			recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

			recog.base.set_state(1344);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1343);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_FilterExpression*/
			recog.base.set_state(1346);
			recog.oC_FilterExpression()?;

			recog.base.set_state(1355);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(242,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1348);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1347);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1350);
					recog.base.match_token(Cypher_T__10,&mut recog.err_handler)?;

					recog.base.set_state(1352);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1351);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1354);
					recog.oC_Expression()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1358);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1357);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1360);
			recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PatternComprehension ----------------
pub type OC_PatternComprehensionContextAll<'input> = OC_PatternComprehensionContext<'input>;


pub type OC_PatternComprehensionContext<'input> = BaseParserRuleContext<'input,OC_PatternComprehensionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternComprehensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PatternComprehensionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternComprehensionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PatternComprehension(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PatternComprehension(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PatternComprehensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PatternComprehension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternComprehension }
}
antlr4rust::tid!{OC_PatternComprehensionContextExt<'a>}

impl<'input> OC_PatternComprehensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PatternComprehensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PatternComprehensionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PatternComprehensionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PatternComprehensionContextExt<'input>>{

fn oC_RelationshipsPattern(&self) -> Option<Rc<OC_RelationshipsPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WHERE, 0)
}

}

impl<'input> OC_PatternComprehensionContextAttrs<'input> for OC_PatternComprehensionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PatternComprehension(&mut self,)
	-> Result<Rc<OC_PatternComprehensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PatternComprehensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_oC_PatternComprehension);
        let mut _localctx: Rc<OC_PatternComprehensionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1362);
			recog.base.match_token(Cypher_T__7,&mut recog.err_handler)?;

			recog.base.set_state(1364);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1363);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1374);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 83)) & !0x3f) == 0 && ((1usize << (_la - 83)) & 131087) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0) {
				{
				/*InvokeRule oC_Variable*/
				recog.base.set_state(1366);
				recog.oC_Variable()?;

				recog.base.set_state(1368);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1367);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1370);
				recog.base.match_token(Cypher_T__2,&mut recog.err_handler)?;

				recog.base.set_state(1372);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1371);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			/*InvokeRule oC_RelationshipsPattern*/
			recog.base.set_state(1376);
			recog.oC_RelationshipsPattern()?;

			recog.base.set_state(1378);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1377);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1388);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_WHERE {
				{
				recog.base.set_state(1380);
				recog.base.match_token(Cypher_WHERE,&mut recog.err_handler)?;

				recog.base.set_state(1382);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1381);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule oC_Expression*/
				recog.base.set_state(1384);
				recog.oC_Expression()?;

				recog.base.set_state(1386);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1385);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1390);
			recog.base.match_token(Cypher_T__10,&mut recog.err_handler)?;

			recog.base.set_state(1392);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1391);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(1394);
			recog.oC_Expression()?;

			recog.base.set_state(1396);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1395);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1398);
			recog.base.match_token(Cypher_T__8,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PropertyLookup ----------------
pub type OC_PropertyLookupContextAll<'input> = OC_PropertyLookupContext<'input>;


pub type OC_PropertyLookupContext<'input> = BaseParserRuleContext<'input,OC_PropertyLookupContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyLookupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PropertyLookupContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyLookupContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PropertyLookup(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PropertyLookup(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PropertyLookupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PropertyLookup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyLookup }
}
antlr4rust::tid!{OC_PropertyLookupContextExt<'a>}

impl<'input> OC_PropertyLookupContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PropertyLookupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PropertyLookupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PropertyLookupContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PropertyLookupContextExt<'input>>{

fn oC_PropertyKeyName(&self) -> Option<Rc<OC_PropertyKeyNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, 0)
}

}

impl<'input> OC_PropertyLookupContextAttrs<'input> for OC_PropertyLookupContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PropertyLookup(&mut self,)
	-> Result<Rc<OC_PropertyLookupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PropertyLookupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_oC_PropertyLookup);
        let mut _localctx: Rc<OC_PropertyLookupContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1400);
			recog.base.match_token(Cypher_T__22,&mut recog.err_handler)?;

			recog.base.set_state(1402);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1401);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			{
			/*InvokeRule oC_PropertyKeyName*/
			recog.base.set_state(1404);
			recog.oC_PropertyKeyName()?;

			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_CaseExpression ----------------
pub type OC_CaseExpressionContextAll<'input> = OC_CaseExpressionContext<'input>;


pub type OC_CaseExpressionContext<'input> = BaseParserRuleContext<'input,OC_CaseExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CaseExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_CaseExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CaseExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_CaseExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_CaseExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_CaseExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_CaseExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_CaseExpression }
}
antlr4rust::tid!{OC_CaseExpressionContextExt<'a>}

impl<'input> OC_CaseExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_CaseExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_CaseExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_CaseExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_CaseExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_END, 0)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ELSE, 0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
/// Retrieves first TerminalNode corresponding to token CASE
/// Returns `None` if there is no child corresponding to token CASE
fn CASE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CASE, 0)
}
fn oC_CaseAlternatives_all(&self) ->  Vec<Rc<OC_CaseAlternativesContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_CaseAlternatives(&self, i: usize) -> Option<Rc<OC_CaseAlternativesContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_CaseExpressionContextAttrs<'input> for OC_CaseExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_CaseExpression(&mut self,)
	-> Result<Rc<OC_CaseExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_CaseExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_oC_CaseExpression);
        let mut _localctx: Rc<OC_CaseExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1428);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(260,&mut recog.base)? {
				1 =>{
					{
					{
					recog.base.set_state(1406);
					recog.base.match_token(Cypher_CASE,&mut recog.err_handler)?;

					recog.base.set_state(1411); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							recog.base.set_state(1408);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(1407);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_CaseAlternatives*/
							recog.base.set_state(1410);
							recog.oC_CaseAlternatives()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(1413); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(256,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
					}
				}
			,
				2 =>{
					{
					{
					recog.base.set_state(1415);
					recog.base.match_token(Cypher_CASE,&mut recog.err_handler)?;

					recog.base.set_state(1417);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1416);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1419);
					recog.oC_Expression()?;

					recog.base.set_state(1424); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							recog.base.set_state(1421);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==Cypher_SP {
								{
								recog.base.set_state(1420);
								recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule oC_CaseAlternatives*/
							recog.base.set_state(1423);
							recog.oC_CaseAlternatives()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(1426); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(259,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
					}
				}

				_ => {}
			}
			recog.base.set_state(1438);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(263,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1431);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1430);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1433);
					recog.base.match_token(Cypher_ELSE,&mut recog.err_handler)?;

					recog.base.set_state(1435);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1434);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1437);
					recog.oC_Expression()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1441);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1440);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1443);
			recog.base.match_token(Cypher_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_CaseAlternatives ----------------
pub type OC_CaseAlternativesContextAll<'input> = OC_CaseAlternativesContext<'input>;


pub type OC_CaseAlternativesContext<'input> = BaseParserRuleContext<'input,OC_CaseAlternativesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CaseAlternativesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_CaseAlternativesContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CaseAlternativesContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_CaseAlternatives(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_CaseAlternatives(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_CaseAlternativesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_CaseAlternatives }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_CaseAlternatives }
}
antlr4rust::tid!{OC_CaseAlternativesContextExt<'a>}

impl<'input> OC_CaseAlternativesContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_CaseAlternativesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_CaseAlternativesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_CaseAlternativesContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_CaseAlternativesContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WHEN, 0)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token THEN
/// Returns `None` if there is no child corresponding to token THEN
fn THEN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_THEN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_CaseAlternativesContextAttrs<'input> for OC_CaseAlternativesContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_CaseAlternatives(&mut self,)
	-> Result<Rc<OC_CaseAlternativesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_CaseAlternativesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_oC_CaseAlternatives);
        let mut _localctx: Rc<OC_CaseAlternativesContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1445);
			recog.base.match_token(Cypher_WHEN,&mut recog.err_handler)?;

			recog.base.set_state(1447);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1446);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(1449);
			recog.oC_Expression()?;

			recog.base.set_state(1451);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1450);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1453);
			recog.base.match_token(Cypher_THEN,&mut recog.err_handler)?;

			recog.base.set_state(1455);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1454);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule oC_Expression*/
			recog.base.set_state(1457);
			recog.oC_Expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Variable ----------------
pub type OC_VariableContextAll<'input> = OC_VariableContext<'input>;


pub type OC_VariableContext<'input> = BaseParserRuleContext<'input,OC_VariableContextExt<'input>>;

#[derive(Clone)]
pub struct OC_VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_VariableContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Variable(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Variable }
}
antlr4rust::tid!{OC_VariableContextExt<'a>}

impl<'input> OC_VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_VariableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_VariableContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_VariableContextExt<'input>>{

fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_VariableContextAttrs<'input> for OC_VariableContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Variable(&mut self,)
	-> Result<Rc<OC_VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_oC_Variable);
        let mut _localctx: Rc<OC_VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SymbolicName*/
			recog.base.set_state(1459);
			recog.oC_SymbolicName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_NumberLiteral ----------------
pub type OC_NumberLiteralContextAll<'input> = OC_NumberLiteralContext<'input>;


pub type OC_NumberLiteralContext<'input> = BaseParserRuleContext<'input,OC_NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NumberLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NumberLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_NumberLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_NumberLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_NumberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_NumberLiteral }
}
antlr4rust::tid!{OC_NumberLiteralContextExt<'a>}

impl<'input> OC_NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_NumberLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_NumberLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_NumberLiteralContextExt<'input>>{

fn oC_DoubleLiteral(&self) -> Option<Rc<OC_DoubleLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_IntegerLiteral(&self) -> Option<Rc<OC_IntegerLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_NumberLiteralContextAttrs<'input> for OC_NumberLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_NumberLiteral(&mut self,)
	-> Result<Rc<OC_NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_oC_NumberLiteral);
        let mut _localctx: Rc<OC_NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1463);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_ExponentDecimalReal |Cypher_RegularDecimalReal 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_DoubleLiteral*/
					recog.base.set_state(1461);
					recog.oC_DoubleLiteral()?;

					}
				}

			Cypher_HexInteger |Cypher_DecimalInteger |Cypher_OctalInteger 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_IntegerLiteral*/
					recog.base.set_state(1462);
					recog.oC_IntegerLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_MapLiteral ----------------
pub type OC_MapLiteralContextAll<'input> = OC_MapLiteralContext<'input>;


pub type OC_MapLiteralContext<'input> = BaseParserRuleContext<'input,OC_MapLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MapLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_MapLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MapLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_MapLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_MapLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_MapLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_MapLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_MapLiteral }
}
antlr4rust::tid!{OC_MapLiteralContextExt<'a>}

impl<'input> OC_MapLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_MapLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_MapLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_MapLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_MapLiteralContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}
fn oC_PropertyKeyName_all(&self) ->  Vec<Rc<OC_PropertyKeyNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PropertyKeyName(&self, i: usize) -> Option<Rc<OC_PropertyKeyNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oC_Expression_all(&self) ->  Vec<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OC_MapLiteralContextAttrs<'input> for OC_MapLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_MapLiteral(&mut self,)
	-> Result<Rc<OC_MapLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_MapLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_oC_MapLiteral);
        let mut _localctx: Rc<OC_MapLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1465);
			recog.base.match_token(Cypher_T__23,&mut recog.err_handler)?;

			recog.base.set_state(1467);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Cypher_SP {
				{
				recog.base.set_state(1466);
				recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1502);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & 4294942719) != 0) || ((((_la - 78)) & !0x3f) == 0 && ((1usize << (_la - 78)) & 2151809023) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 20479) != 0) {
				{
				/*InvokeRule oC_PropertyKeyName*/
				recog.base.set_state(1469);
				recog.oC_PropertyKeyName()?;

				recog.base.set_state(1471);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1470);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1473);
				recog.base.match_token(Cypher_T__9,&mut recog.err_handler)?;

				recog.base.set_state(1475);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1474);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule oC_Expression*/
				recog.base.set_state(1477);
				recog.oC_Expression()?;

				recog.base.set_state(1479);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Cypher_SP {
					{
					recog.base.set_state(1478);
					recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

					}
				}

				recog.base.set_state(1499);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Cypher_T__1 {
					{
					{
					recog.base.set_state(1481);
					recog.base.match_token(Cypher_T__1,&mut recog.err_handler)?;

					recog.base.set_state(1483);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1482);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PropertyKeyName*/
					recog.base.set_state(1485);
					recog.oC_PropertyKeyName()?;

					recog.base.set_state(1487);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1486);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1489);
					recog.base.match_token(Cypher_T__9,&mut recog.err_handler)?;

					recog.base.set_state(1491);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1490);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_Expression*/
					recog.base.set_state(1493);
					recog.oC_Expression()?;

					recog.base.set_state(1495);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1494);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					}
					}
					recog.base.set_state(1501);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1504);
			recog.base.match_token(Cypher_T__24,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Parameter ----------------
pub type OC_ParameterContextAll<'input> = OC_ParameterContext<'input>;


pub type OC_ParameterContext<'input> = BaseParserRuleContext<'input,OC_ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Parameter(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Parameter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Parameter }
}
antlr4rust::tid!{OC_ParameterContextExt<'a>}

impl<'input> OC_ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ParameterContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ParameterContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ParameterContextExt<'input>>{

fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DecimalInteger
/// Returns `None` if there is no child corresponding to token DecimalInteger
fn DecimalInteger(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DecimalInteger, 0)
}

}

impl<'input> OC_ParameterContextAttrs<'input> for OC_ParameterContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Parameter(&mut self,)
	-> Result<Rc<OC_ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_oC_Parameter);
        let mut _localctx: Rc<OC_ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1506);
			recog.base.match_token(Cypher_T__25,&mut recog.err_handler)?;

			recog.base.set_state(1509);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_COUNT |Cypher_ANY |Cypher_NONE |Cypher_SINGLE |Cypher_HexLetter |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					{
					/*InvokeRule oC_SymbolicName*/
					recog.base.set_state(1507);
					recog.oC_SymbolicName()?;

					}
				}

			Cypher_DecimalInteger 
				=> {
					{
					recog.base.set_state(1508);
					recog.base.match_token(Cypher_DecimalInteger,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PropertyExpression ----------------
pub type OC_PropertyExpressionContextAll<'input> = OC_PropertyExpressionContext<'input>;


pub type OC_PropertyExpressionContext<'input> = BaseParserRuleContext<'input,OC_PropertyExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PropertyExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PropertyExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PropertyExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PropertyExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PropertyExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyExpression }
}
antlr4rust::tid!{OC_PropertyExpressionContextExt<'a>}

impl<'input> OC_PropertyExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PropertyExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PropertyExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PropertyExpressionContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PropertyExpressionContextExt<'input>>{

fn oC_Atom(&self) -> Option<Rc<OC_AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_PropertyLookup_all(&self) ->  Vec<Rc<OC_PropertyLookupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oC_PropertyLookup(&self, i: usize) -> Option<Rc<OC_PropertyLookupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SP in current rule
fn SP_all(&self) -> Vec<Rc<TerminalNode<'input,CypherParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
/// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SP, i)
}

}

impl<'input> OC_PropertyExpressionContextAttrs<'input> for OC_PropertyExpressionContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PropertyExpression(&mut self,)
	-> Result<Rc<OC_PropertyExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PropertyExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_oC_PropertyExpression);
        let mut _localctx: Rc<OC_PropertyExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_Atom*/
			recog.base.set_state(1511);
			recog.oC_Atom()?;

			recog.base.set_state(1516); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(1513);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Cypher_SP {
						{
						recog.base.set_state(1512);
						recog.base.match_token(Cypher_SP,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule oC_PropertyLookup*/
					recog.base.set_state(1515);
					recog.oC_PropertyLookup()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(1518); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(281,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_PropertyKeyName ----------------
pub type OC_PropertyKeyNameContextAll<'input> = OC_PropertyKeyNameContext<'input>;


pub type OC_PropertyKeyNameContext<'input> = BaseParserRuleContext<'input,OC_PropertyKeyNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyKeyNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_PropertyKeyNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyKeyNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_PropertyKeyName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_PropertyKeyName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_PropertyKeyNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_PropertyKeyName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyKeyName }
}
antlr4rust::tid!{OC_PropertyKeyNameContextExt<'a>}

impl<'input> OC_PropertyKeyNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_PropertyKeyNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_PropertyKeyNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_PropertyKeyNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_PropertyKeyNameContextExt<'input>>{

fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_PropertyKeyNameContextAttrs<'input> for OC_PropertyKeyNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_PropertyKeyName(&mut self,)
	-> Result<Rc<OC_PropertyKeyNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_PropertyKeyNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_oC_PropertyKeyName);
        let mut _localctx: Rc<OC_PropertyKeyNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule oC_SchemaName*/
			recog.base.set_state(1520);
			recog.oC_SchemaName()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_IntegerLiteral ----------------
pub type OC_IntegerLiteralContextAll<'input> = OC_IntegerLiteralContext<'input>;


pub type OC_IntegerLiteralContext<'input> = BaseParserRuleContext<'input,OC_IntegerLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_IntegerLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_IntegerLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_IntegerLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_IntegerLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_IntegerLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_IntegerLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_IntegerLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_IntegerLiteral }
}
antlr4rust::tid!{OC_IntegerLiteralContextExt<'a>}

impl<'input> OC_IntegerLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_IntegerLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_IntegerLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_IntegerLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_IntegerLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HexInteger
/// Returns `None` if there is no child corresponding to token HexInteger
fn HexInteger(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_HexInteger, 0)
}
/// Retrieves first TerminalNode corresponding to token OctalInteger
/// Returns `None` if there is no child corresponding to token OctalInteger
fn OctalInteger(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OctalInteger, 0)
}
/// Retrieves first TerminalNode corresponding to token DecimalInteger
/// Returns `None` if there is no child corresponding to token DecimalInteger
fn DecimalInteger(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DecimalInteger, 0)
}

}

impl<'input> OC_IntegerLiteralContextAttrs<'input> for OC_IntegerLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_IntegerLiteral(&mut self,)
	-> Result<Rc<OC_IntegerLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_IntegerLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_oC_IntegerLiteral);
        let mut _localctx: Rc<OC_IntegerLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1522);
			_la = recog.base.input.la(1);
			if { !(((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & 7) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_DoubleLiteral ----------------
pub type OC_DoubleLiteralContextAll<'input> = OC_DoubleLiteralContext<'input>;


pub type OC_DoubleLiteralContext<'input> = BaseParserRuleContext<'input,OC_DoubleLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DoubleLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_DoubleLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DoubleLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_DoubleLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_DoubleLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_DoubleLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_DoubleLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_DoubleLiteral }
}
antlr4rust::tid!{OC_DoubleLiteralContextExt<'a>}

impl<'input> OC_DoubleLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_DoubleLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_DoubleLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_DoubleLiteralContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_DoubleLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ExponentDecimalReal
/// Returns `None` if there is no child corresponding to token ExponentDecimalReal
fn ExponentDecimalReal(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ExponentDecimalReal, 0)
}
/// Retrieves first TerminalNode corresponding to token RegularDecimalReal
/// Returns `None` if there is no child corresponding to token RegularDecimalReal
fn RegularDecimalReal(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_RegularDecimalReal, 0)
}

}

impl<'input> OC_DoubleLiteralContextAttrs<'input> for OC_DoubleLiteralContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_DoubleLiteral(&mut self,)
	-> Result<Rc<OC_DoubleLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_DoubleLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_oC_DoubleLiteral);
        let mut _localctx: Rc<OC_DoubleLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1524);
			_la = recog.base.input.la(1);
			if { !(_la==Cypher_ExponentDecimalReal || _la==Cypher_RegularDecimalReal) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SchemaName ----------------
pub type OC_SchemaNameContextAll<'input> = OC_SchemaNameContext<'input>;


pub type OC_SchemaNameContext<'input> = BaseParserRuleContext<'input,OC_SchemaNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SchemaNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SchemaNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SchemaNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SchemaName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SchemaName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SchemaNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SchemaName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SchemaName }
}
antlr4rust::tid!{OC_SchemaNameContextExt<'a>}

impl<'input> OC_SchemaNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SchemaNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SchemaNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SchemaNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SchemaNameContextExt<'input>>{

fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oC_ReservedWord(&self) -> Option<Rc<OC_ReservedWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OC_SchemaNameContextAttrs<'input> for OC_SchemaNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SchemaName(&mut self,)
	-> Result<Rc<OC_SchemaNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SchemaNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_oC_SchemaName);
        let mut _localctx: Rc<OC_SchemaNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1528);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Cypher_COUNT |Cypher_ANY |Cypher_NONE |Cypher_SINGLE |Cypher_HexLetter |
			Cypher_FILTER |Cypher_EXTRACT |Cypher_UnescapedSymbolicName |Cypher_EscapedSymbolicName 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule oC_SymbolicName*/
					recog.base.set_state(1526);
					recog.oC_SymbolicName()?;

					}
				}

			Cypher_UNION |Cypher_ALL |Cypher_OPTIONAL |Cypher_MATCH |Cypher_UNWIND |
			Cypher_AS |Cypher_MERGE |Cypher_ON |Cypher_CREATE |Cypher_SET |Cypher_DETACH |
			Cypher_DELETE |Cypher_REMOVE |Cypher_WITH |Cypher_RETURN |Cypher_DISTINCT |
			Cypher_ORDER |Cypher_BY |Cypher_L_SKIP |Cypher_LIMIT |Cypher_ASCENDING |
			Cypher_ASC |Cypher_DESCENDING |Cypher_DESC |Cypher_WHERE |Cypher_OR |
			Cypher_XOR |Cypher_AND |Cypher_NOT |Cypher_IN |Cypher_STARTS |Cypher_ENDS |
			Cypher_CONTAINS |Cypher_IS |Cypher_NULL |Cypher_TRUE |Cypher_FALSE |Cypher_EXISTS |
			Cypher_CASE |Cypher_ELSE |Cypher_END |Cypher_WHEN |Cypher_THEN |Cypher_CONSTRAINT |
			Cypher_DO |Cypher_FOR |Cypher_REQUIRE |Cypher_UNIQUE |Cypher_MANDATORY |
			Cypher_SCALAR |Cypher_OF |Cypher_ADD |Cypher_DROP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule oC_ReservedWord*/
					recog.base.set_state(1527);
					recog.oC_ReservedWord()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_ReservedWord ----------------
pub type OC_ReservedWordContextAll<'input> = OC_ReservedWordContext<'input>;


pub type OC_ReservedWordContext<'input> = BaseParserRuleContext<'input,OC_ReservedWordContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReservedWordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_ReservedWordContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReservedWordContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_ReservedWord(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_ReservedWord(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_ReservedWordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_ReservedWord }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_ReservedWord }
}
antlr4rust::tid!{OC_ReservedWordContextExt<'a>}

impl<'input> OC_ReservedWordContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_ReservedWordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_ReservedWordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_ReservedWordContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_ReservedWordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ALL
/// Returns `None` if there is no child corresponding to token ALL
fn ALL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ALL, 0)
}
/// Retrieves first TerminalNode corresponding to token ASC
/// Returns `None` if there is no child corresponding to token ASC
fn ASC(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ASC, 0)
}
/// Retrieves first TerminalNode corresponding to token ASCENDING
/// Returns `None` if there is no child corresponding to token ASCENDING
fn ASCENDING(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ASCENDING, 0)
}
/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token CREATE
/// Returns `None` if there is no child corresponding to token CREATE
fn CREATE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CREATE, 0)
}
/// Retrieves first TerminalNode corresponding to token DELETE
/// Returns `None` if there is no child corresponding to token DELETE
fn DELETE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DELETE, 0)
}
/// Retrieves first TerminalNode corresponding to token DESC
/// Returns `None` if there is no child corresponding to token DESC
fn DESC(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DESC, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCENDING
/// Returns `None` if there is no child corresponding to token DESCENDING
fn DESCENDING(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DESCENDING, 0)
}
/// Retrieves first TerminalNode corresponding to token DETACH
/// Returns `None` if there is no child corresponding to token DETACH
fn DETACH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DETACH, 0)
}
/// Retrieves first TerminalNode corresponding to token EXISTS
/// Returns `None` if there is no child corresponding to token EXISTS
fn EXISTS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_EXISTS, 0)
}
/// Retrieves first TerminalNode corresponding to token LIMIT
/// Returns `None` if there is no child corresponding to token LIMIT
fn LIMIT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_LIMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MATCH, 0)
}
/// Retrieves first TerminalNode corresponding to token MERGE
/// Returns `None` if there is no child corresponding to token MERGE
fn MERGE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MERGE, 0)
}
/// Retrieves first TerminalNode corresponding to token ON
/// Returns `None` if there is no child corresponding to token ON
fn ON(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ON, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONAL
/// Returns `None` if there is no child corresponding to token OPTIONAL
fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OPTIONAL, 0)
}
/// Retrieves first TerminalNode corresponding to token ORDER
/// Returns `None` if there is no child corresponding to token ORDER
fn ORDER(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ORDER, 0)
}
/// Retrieves first TerminalNode corresponding to token REMOVE
/// Returns `None` if there is no child corresponding to token REMOVE
fn REMOVE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_REMOVE, 0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_RETURN, 0)
}
/// Retrieves first TerminalNode corresponding to token SET
/// Returns `None` if there is no child corresponding to token SET
fn SET(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_SKIP
/// Returns `None` if there is no child corresponding to token L_SKIP
fn L_SKIP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_L_SKIP, 0)
}
/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WHERE, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token UNION
/// Returns `None` if there is no child corresponding to token UNION
fn UNION(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UNION, 0)
}
/// Retrieves first TerminalNode corresponding to token UNWIND
/// Returns `None` if there is no child corresponding to token UNWIND
fn UNWIND(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UNWIND, 0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AND, 0)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_AS, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token DISTINCT
/// Returns `None` if there is no child corresponding to token DISTINCT
fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DISTINCT, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDS
/// Returns `None` if there is no child corresponding to token ENDS
fn ENDS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ENDS, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NOT, 0)
}
/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OR, 0)
}
/// Retrieves first TerminalNode corresponding to token STARTS
/// Returns `None` if there is no child corresponding to token STARTS
fn STARTS(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_STARTS, 0)
}
/// Retrieves first TerminalNode corresponding to token XOR
/// Returns `None` if there is no child corresponding to token XOR
fn XOR(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_XOR, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_FALSE, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token CONSTRAINT
/// Returns `None` if there is no child corresponding to token CONSTRAINT
fn CONSTRAINT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CONSTRAINT, 0)
}
/// Retrieves first TerminalNode corresponding to token DO
/// Returns `None` if there is no child corresponding to token DO
fn DO(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DO, 0)
}
/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token REQUIRE
/// Returns `None` if there is no child corresponding to token REQUIRE
fn REQUIRE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_REQUIRE, 0)
}
/// Retrieves first TerminalNode corresponding to token UNIQUE
/// Returns `None` if there is no child corresponding to token UNIQUE
fn UNIQUE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UNIQUE, 0)
}
/// Retrieves first TerminalNode corresponding to token CASE
/// Returns `None` if there is no child corresponding to token CASE
fn CASE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_CASE, 0)
}
/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_WHEN, 0)
}
/// Retrieves first TerminalNode corresponding to token THEN
/// Returns `None` if there is no child corresponding to token THEN
fn THEN(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_THEN, 0)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_END, 0)
}
/// Retrieves first TerminalNode corresponding to token MANDATORY
/// Returns `None` if there is no child corresponding to token MANDATORY
fn MANDATORY(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_MANDATORY, 0)
}
/// Retrieves first TerminalNode corresponding to token SCALAR
/// Returns `None` if there is no child corresponding to token SCALAR
fn SCALAR(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SCALAR, 0)
}
/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_OF, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token DROP
/// Returns `None` if there is no child corresponding to token DROP
fn DROP(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_DROP, 0)
}

}

impl<'input> OC_ReservedWordContextAttrs<'input> for OC_ReservedWordContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_ReservedWord(&mut self,)
	-> Result<Rc<OC_ReservedWordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_ReservedWordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_oC_ReservedWord);
        let mut _localctx: Rc<OC_ReservedWordContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1530);
			_la = recog.base.input.la(1);
			if { !(((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & 4294942719) != 0) || ((((_la - 78)) & !0x3f) == 0 && ((1usize << (_la - 78)) & 2147614239) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 511) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_SymbolicName ----------------
pub type OC_SymbolicNameContextAll<'input> = OC_SymbolicNameContext<'input>;


pub type OC_SymbolicNameContext<'input> = BaseParserRuleContext<'input,OC_SymbolicNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SymbolicNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_SymbolicNameContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SymbolicNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_SymbolicName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_SymbolicName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_SymbolicNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_SymbolicName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_SymbolicName }
}
antlr4rust::tid!{OC_SymbolicNameContextExt<'a>}

impl<'input> OC_SymbolicNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_SymbolicNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_SymbolicNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_SymbolicNameContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_SymbolicNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UnescapedSymbolicName
/// Returns `None` if there is no child corresponding to token UnescapedSymbolicName
fn UnescapedSymbolicName(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_UnescapedSymbolicName, 0)
}
/// Retrieves first TerminalNode corresponding to token EscapedSymbolicName
/// Returns `None` if there is no child corresponding to token EscapedSymbolicName
fn EscapedSymbolicName(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_EscapedSymbolicName, 0)
}
/// Retrieves first TerminalNode corresponding to token HexLetter
/// Returns `None` if there is no child corresponding to token HexLetter
fn HexLetter(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_HexLetter, 0)
}
/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_COUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token FILTER
/// Returns `None` if there is no child corresponding to token FILTER
fn FILTER(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_FILTER, 0)
}
/// Retrieves first TerminalNode corresponding to token EXTRACT
/// Returns `None` if there is no child corresponding to token EXTRACT
fn EXTRACT(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_EXTRACT, 0)
}
/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token NONE
/// Returns `None` if there is no child corresponding to token NONE
fn NONE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_NONE, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE
/// Returns `None` if there is no child corresponding to token SINGLE
fn SINGLE(&self) -> Option<Rc<TerminalNode<'input,CypherParserContextType>>> where Self:Sized{
	self.get_token(Cypher_SINGLE, 0)
}

}

impl<'input> OC_SymbolicNameContextAttrs<'input> for OC_SymbolicNameContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_SymbolicName(&mut self,)
	-> Result<Rc<OC_SymbolicNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_SymbolicNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_oC_SymbolicName);
        let mut _localctx: Rc<OC_SymbolicNameContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1532);
			_la = recog.base.input.la(1);
			if { !(((((_la - 83)) & !0x3f) == 0 && ((1usize << (_la - 83)) & 131087) != 0) || ((((_la - 119)) & !0x3f) == 0 && ((1usize << (_la - 119)) & 39) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_LeftArrowHead ----------------
pub type OC_LeftArrowHeadContextAll<'input> = OC_LeftArrowHeadContext<'input>;


pub type OC_LeftArrowHeadContext<'input> = BaseParserRuleContext<'input,OC_LeftArrowHeadContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LeftArrowHeadContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_LeftArrowHeadContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LeftArrowHeadContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_LeftArrowHead(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_LeftArrowHead(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_LeftArrowHeadContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_LeftArrowHead }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_LeftArrowHead }
}
antlr4rust::tid!{OC_LeftArrowHeadContextExt<'a>}

impl<'input> OC_LeftArrowHeadContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_LeftArrowHeadContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_LeftArrowHeadContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_LeftArrowHeadContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_LeftArrowHeadContextExt<'input>>{


}

impl<'input> OC_LeftArrowHeadContextAttrs<'input> for OC_LeftArrowHeadContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_LeftArrowHead(&mut self,)
	-> Result<Rc<OC_LeftArrowHeadContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_LeftArrowHeadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_oC_LeftArrowHead);
        let mut _localctx: Rc<OC_LeftArrowHeadContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1534);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 2013790208) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_RightArrowHead ----------------
pub type OC_RightArrowHeadContextAll<'input> = OC_RightArrowHeadContext<'input>;


pub type OC_RightArrowHeadContext<'input> = BaseParserRuleContext<'input,OC_RightArrowHeadContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RightArrowHeadContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_RightArrowHeadContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RightArrowHeadContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_RightArrowHead(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_RightArrowHead(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_RightArrowHeadContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_RightArrowHead }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_RightArrowHead }
}
antlr4rust::tid!{OC_RightArrowHeadContextExt<'a>}

impl<'input> OC_RightArrowHeadContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_RightArrowHeadContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_RightArrowHeadContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_RightArrowHeadContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_RightArrowHeadContextExt<'input>>{


}

impl<'input> OC_RightArrowHeadContextAttrs<'input> for OC_RightArrowHeadContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_RightArrowHead(&mut self,)
	-> Result<Rc<OC_RightArrowHeadContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_RightArrowHeadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_oC_RightArrowHead);
        let mut _localctx: Rc<OC_RightArrowHeadContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1536);
			_la = recog.base.input.la(1);
			if { !(((((_la - 20)) & !0x3f) == 0 && ((1usize << (_la - 20)) & 30721) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- oC_Dash ----------------
pub type OC_DashContextAll<'input> = OC_DashContext<'input>;


pub type OC_DashContext<'input> = BaseParserRuleContext<'input,OC_DashContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DashContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CypherParserContext<'input> for OC_DashContext<'input>{}

impl<'input,'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DashContext<'input>{
		fn enter(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_oC_Dash(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CypherListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_oC_Dash(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OC_DashContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CypherParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oC_Dash }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oC_Dash }
}
antlr4rust::tid!{OC_DashContextExt<'a>}

impl<'input> OC_DashContextExt<'input>{
	fn new(parent: Option<Rc<dyn CypherParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OC_DashContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OC_DashContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OC_DashContextAttrs<'input>: CypherParserContext<'input> + BorrowMut<OC_DashContextExt<'input>>{


}

impl<'input> OC_DashContextAttrs<'input> for OC_DashContext<'input>{}

impl<'input, I> CypherParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn oC_Dash(&mut self,)
	-> Result<Rc<OC_DashContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OC_DashContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_oC_Dash);
        let mut _localctx: Rc<OC_DashContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1538);
			_la = recog.base.input.la(1);
			if { !(((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & 4292870145) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 127, 1541, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7, 84, 2, 85, 
		7, 85, 2, 86, 7, 86, 2, 87, 7, 87, 2, 88, 7, 88, 2, 89, 7, 89, 2, 90, 
		7, 90, 2, 91, 7, 91, 2, 92, 7, 92, 2, 93, 7, 93, 2, 94, 7, 94, 2, 95, 
		7, 95, 2, 96, 7, 96, 2, 97, 7, 97, 2, 98, 7, 98, 1, 0, 3, 0, 200, 8, 0, 
		1, 0, 1, 0, 3, 0, 204, 8, 0, 1, 0, 3, 0, 207, 8, 0, 1, 0, 3, 0, 210, 8, 
		0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 3, 2, 218, 8, 2, 1, 3, 1, 3, 3, 
		3, 222, 8, 3, 1, 3, 5, 3, 225, 8, 3, 10, 3, 12, 3, 228, 9, 3, 1, 4, 1, 
		4, 1, 4, 1, 4, 3, 4, 234, 8, 4, 1, 4, 1, 4, 1, 4, 3, 4, 239, 8, 4, 1, 
		4, 3, 4, 242, 8, 4, 1, 5, 1, 5, 3, 5, 246, 8, 5, 1, 6, 1, 6, 3, 6, 250, 
		8, 6, 5, 6, 252, 8, 6, 10, 6, 12, 6, 255, 9, 6, 1, 6, 1, 6, 1, 6, 3, 6, 
		260, 8, 6, 5, 6, 262, 8, 6, 10, 6, 12, 6, 265, 9, 6, 1, 6, 1, 6, 3, 6, 
		269, 8, 6, 1, 6, 5, 6, 272, 8, 6, 10, 6, 12, 6, 275, 9, 6, 1, 6, 3, 6, 
		278, 8, 6, 1, 6, 3, 6, 281, 8, 6, 3, 6, 283, 8, 6, 1, 7, 1, 7, 3, 7, 287, 
		8, 7, 5, 7, 289, 8, 7, 10, 7, 12, 7, 292, 9, 7, 1, 7, 1, 7, 3, 7, 296, 
		8, 7, 5, 7, 298, 8, 7, 10, 7, 12, 7, 301, 9, 7, 1, 7, 1, 7, 3, 7, 305, 
		8, 7, 4, 7, 307, 8, 7, 11, 7, 12, 7, 308, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 
		1, 8, 1, 8, 3, 8, 318, 8, 8, 1, 9, 1, 9, 1, 9, 3, 9, 323, 8, 9, 1, 10, 
		1, 10, 3, 10, 327, 8, 10, 1, 10, 1, 10, 3, 10, 331, 8, 10, 1, 10, 1, 10, 
		3, 10, 335, 8, 10, 1, 10, 3, 10, 338, 8, 10, 1, 11, 1, 11, 3, 11, 342, 
		8, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 12, 1, 12, 3, 12, 
		352, 8, 12, 1, 12, 1, 12, 1, 12, 5, 12, 357, 8, 12, 10, 12, 12, 12, 360, 
		9, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 
		1, 13, 3, 13, 372, 8, 13, 1, 14, 1, 14, 3, 14, 376, 8, 14, 1, 14, 1, 14, 
		1, 15, 1, 15, 3, 15, 382, 8, 15, 1, 15, 1, 15, 1, 15, 5, 15, 387, 8, 15, 
		10, 15, 12, 15, 390, 9, 15, 1, 16, 1, 16, 3, 16, 394, 8, 16, 1, 16, 1, 
		16, 3, 16, 398, 8, 16, 1, 16, 1, 16, 1, 16, 1, 16, 3, 16, 404, 8, 16, 
		1, 16, 1, 16, 3, 16, 408, 8, 16, 1, 16, 1, 16, 1, 16, 1, 16, 3, 16, 414, 
		8, 16, 1, 16, 1, 16, 3, 16, 418, 8, 16, 1, 16, 1, 16, 1, 16, 1, 16, 3, 
		16, 424, 8, 16, 1, 16, 1, 16, 3, 16, 428, 8, 16, 1, 17, 1, 17, 3, 17, 
		432, 8, 17, 1, 17, 1, 17, 3, 17, 436, 8, 17, 1, 17, 1, 17, 3, 17, 440, 
		8, 17, 1, 17, 1, 17, 3, 17, 444, 8, 17, 1, 17, 5, 17, 447, 8, 17, 10, 
		17, 12, 17, 450, 9, 17, 1, 18, 1, 18, 1, 18, 1, 18, 3, 18, 456, 8, 18, 
		1, 18, 1, 18, 3, 18, 460, 8, 18, 1, 18, 5, 18, 463, 8, 18, 10, 18, 12, 
		18, 466, 9, 18, 1, 19, 1, 19, 1, 19, 1, 19, 3, 19, 472, 8, 19, 1, 20, 
		1, 20, 1, 20, 1, 20, 3, 20, 478, 8, 20, 1, 20, 1, 20, 1, 20, 3, 20, 483, 
		8, 20, 1, 21, 1, 21, 1, 21, 1, 21, 3, 21, 489, 8, 21, 1, 21, 1, 21, 1, 
		21, 1, 21, 3, 21, 495, 8, 21, 1, 22, 1, 22, 1, 22, 3, 22, 500, 8, 22, 
		1, 22, 1, 22, 3, 22, 504, 8, 22, 1, 22, 5, 22, 507, 8, 22, 10, 22, 12, 
		22, 510, 9, 22, 3, 22, 512, 8, 22, 1, 22, 3, 22, 515, 8, 22, 1, 22, 3, 
		22, 518, 8, 22, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 3, 23, 525, 8, 23, 
		1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 3, 24, 532, 8, 24, 1, 24, 3, 24, 535, 
		8, 24, 1, 25, 1, 25, 1, 25, 1, 26, 3, 26, 541, 8, 26, 1, 26, 3, 26, 544, 
		8, 26, 1, 26, 1, 26, 1, 26, 1, 26, 3, 26, 550, 8, 26, 1, 26, 1, 26, 3, 
		26, 554, 8, 26, 1, 26, 1, 26, 3, 26, 558, 8, 26, 1, 27, 1, 27, 3, 27, 
		562, 8, 27, 1, 27, 1, 27, 3, 27, 566, 8, 27, 1, 27, 5, 27, 569, 8, 27, 
		10, 27, 12, 27, 572, 9, 27, 1, 27, 1, 27, 3, 27, 576, 8, 27, 1, 27, 1, 
		27, 3, 27, 580, 8, 27, 1, 27, 5, 27, 583, 8, 27, 10, 27, 12, 27, 586, 
		9, 27, 3, 27, 588, 8, 27, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 
		28, 3, 28, 597, 8, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 
		3, 29, 606, 8, 29, 1, 29, 5, 29, 609, 8, 29, 10, 29, 12, 29, 612, 9, 29, 
		1, 30, 1, 30, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 1, 32, 
		3, 32, 624, 8, 32, 1, 32, 3, 32, 627, 8, 32, 1, 33, 1, 33, 1, 33, 1, 33, 
		1, 34, 1, 34, 3, 34, 635, 8, 34, 1, 34, 1, 34, 3, 34, 639, 8, 34, 1, 34, 
		5, 34, 642, 8, 34, 10, 34, 12, 34, 645, 9, 34, 1, 35, 1, 35, 3, 35, 649, 
		8, 35, 1, 35, 1, 35, 3, 35, 653, 8, 35, 1, 35, 1, 35, 1, 35, 3, 35, 658, 
		8, 35, 1, 36, 1, 36, 1, 37, 1, 37, 3, 37, 664, 8, 37, 1, 37, 5, 37, 667, 
		8, 37, 10, 37, 12, 37, 670, 9, 37, 1, 37, 1, 37, 1, 37, 1, 37, 3, 37, 
		676, 8, 37, 1, 38, 1, 38, 3, 38, 680, 8, 38, 1, 38, 1, 38, 3, 38, 684, 
		8, 38, 3, 38, 686, 8, 38, 1, 38, 1, 38, 3, 38, 690, 8, 38, 3, 38, 692, 
		8, 38, 1, 38, 1, 38, 3, 38, 696, 8, 38, 3, 38, 698, 8, 38, 1, 38, 1, 38, 
		1, 39, 1, 39, 3, 39, 704, 8, 39, 1, 39, 1, 39, 1, 40, 1, 40, 3, 40, 710, 
		8, 40, 1, 40, 1, 40, 3, 40, 714, 8, 40, 1, 40, 3, 40, 717, 8, 40, 1, 40, 
		3, 40, 720, 8, 40, 1, 40, 1, 40, 3, 40, 724, 8, 40, 1, 40, 1, 40, 1, 40, 
		1, 40, 3, 40, 730, 8, 40, 1, 40, 1, 40, 3, 40, 734, 8, 40, 1, 40, 3, 40, 
		737, 8, 40, 1, 40, 3, 40, 740, 8, 40, 1, 40, 1, 40, 1, 40, 1, 40, 3, 40, 
		746, 8, 40, 1, 40, 3, 40, 749, 8, 40, 1, 40, 3, 40, 752, 8, 40, 1, 40, 
		1, 40, 3, 40, 756, 8, 40, 1, 40, 1, 40, 1, 40, 1, 40, 3, 40, 762, 8, 40, 
		1, 40, 3, 40, 765, 8, 40, 1, 40, 3, 40, 768, 8, 40, 1, 40, 1, 40, 3, 40, 
		772, 8, 40, 1, 41, 1, 41, 3, 41, 776, 8, 41, 1, 41, 1, 41, 3, 41, 780, 
		8, 41, 3, 41, 782, 8, 41, 1, 41, 1, 41, 3, 41, 786, 8, 41, 3, 41, 788, 
		8, 41, 1, 41, 3, 41, 791, 8, 41, 1, 41, 1, 41, 3, 41, 795, 8, 41, 3, 41, 
		797, 8, 41, 1, 41, 1, 41, 1, 42, 1, 42, 3, 42, 803, 8, 42, 1, 43, 1, 43, 
		3, 43, 807, 8, 43, 1, 43, 1, 43, 3, 43, 811, 8, 43, 1, 43, 1, 43, 3, 43, 
		815, 8, 43, 1, 43, 3, 43, 818, 8, 43, 1, 43, 5, 43, 821, 8, 43, 10, 43, 
		12, 43, 824, 9, 43, 1, 44, 1, 44, 3, 44, 828, 8, 44, 1, 44, 5, 44, 831, 
		8, 44, 10, 44, 12, 44, 834, 9, 44, 1, 45, 1, 45, 3, 45, 838, 8, 45, 1, 
		45, 1, 45, 1, 46, 1, 46, 3, 46, 844, 8, 46, 1, 46, 1, 46, 3, 46, 848, 
		8, 46, 3, 46, 850, 8, 46, 1, 46, 1, 46, 3, 46, 854, 8, 46, 1, 46, 1, 46, 
		3, 46, 858, 8, 46, 3, 46, 860, 8, 46, 3, 46, 862, 8, 46, 1, 47, 1, 47, 
		1, 48, 1, 48, 1, 49, 1, 49, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 5, 50, 
		875, 8, 50, 10, 50, 12, 50, 878, 9, 50, 1, 51, 1, 51, 1, 51, 1, 51, 1, 
		51, 5, 51, 885, 8, 51, 10, 51, 12, 51, 888, 9, 51, 1, 52, 1, 52, 1, 52, 
		1, 52, 1, 52, 5, 52, 895, 8, 52, 10, 52, 12, 52, 898, 9, 52, 1, 53, 1, 
		53, 3, 53, 902, 8, 53, 5, 53, 904, 8, 53, 10, 53, 12, 53, 907, 9, 53, 
		1, 53, 1, 53, 1, 54, 1, 54, 3, 54, 913, 8, 54, 1, 54, 5, 54, 916, 8, 54, 
		10, 54, 12, 54, 919, 9, 54, 1, 55, 1, 55, 3, 55, 923, 8, 55, 1, 55, 1, 
		55, 3, 55, 927, 8, 55, 1, 55, 1, 55, 3, 55, 931, 8, 55, 1, 55, 1, 55, 
		3, 55, 935, 8, 55, 1, 55, 5, 55, 938, 8, 55, 10, 55, 12, 55, 941, 9, 55, 
		1, 56, 1, 56, 3, 56, 945, 8, 56, 1, 56, 1, 56, 3, 56, 949, 8, 56, 1, 56, 
		1, 56, 3, 56, 953, 8, 56, 1, 56, 1, 56, 3, 56, 957, 8, 56, 1, 56, 1, 56, 
		3, 56, 961, 8, 56, 1, 56, 1, 56, 3, 56, 965, 8, 56, 1, 56, 5, 56, 968, 
		8, 56, 10, 56, 12, 56, 971, 9, 56, 1, 57, 1, 57, 3, 57, 975, 8, 57, 1, 
		57, 1, 57, 3, 57, 979, 8, 57, 1, 57, 5, 57, 982, 8, 57, 10, 57, 12, 57, 
		985, 9, 57, 1, 58, 1, 58, 3, 58, 989, 8, 58, 5, 58, 991, 8, 58, 10, 58, 
		12, 58, 994, 9, 58, 1, 58, 1, 58, 1, 59, 1, 59, 1, 59, 1, 59, 5, 59, 1002, 
		8, 59, 10, 59, 12, 59, 1005, 9, 59, 1, 60, 1, 60, 1, 60, 3, 60, 1010, 
		8, 60, 1, 60, 1, 60, 3, 60, 1014, 8, 60, 1, 60, 1, 60, 1, 60, 1, 60, 1, 
		60, 3, 60, 1021, 8, 60, 1, 60, 1, 60, 3, 60, 1025, 8, 60, 1, 60, 1, 60, 
		3, 60, 1029, 8, 60, 1, 60, 3, 60, 1032, 8, 60, 1, 61, 1, 61, 1, 61, 1, 
		61, 1, 61, 1, 61, 1, 61, 1, 61, 1, 61, 1, 61, 3, 61, 1044, 8, 61, 1, 61, 
		3, 61, 1047, 8, 61, 1, 61, 1, 61, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 
		62, 1, 62, 1, 62, 1, 62, 1, 62, 3, 62, 1061, 8, 62, 1, 63, 1, 63, 3, 63, 
		1065, 8, 63, 1, 63, 5, 63, 1068, 8, 63, 10, 63, 12, 63, 1071, 9, 63, 1, 
		63, 3, 63, 1074, 8, 63, 1, 63, 3, 63, 1077, 8, 63, 1, 64, 1, 64, 1, 64, 
		1, 64, 1, 64, 3, 64, 1084, 8, 64, 1, 64, 1, 64, 3, 64, 1088, 8, 64, 1, 
		64, 1, 64, 3, 64, 1092, 8, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 
		1099, 8, 64, 1, 64, 1, 64, 3, 64, 1103, 8, 64, 1, 64, 1, 64, 3, 64, 1107, 
		8, 64, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 1113, 8, 64, 1, 64, 1, 64, 3, 
		64, 1117, 8, 64, 1, 64, 1, 64, 3, 64, 1121, 8, 64, 1, 64, 1, 64, 1, 64, 
		1, 64, 3, 64, 1127, 8, 64, 1, 64, 1, 64, 3, 64, 1131, 8, 64, 1, 64, 1, 
		64, 3, 64, 1135, 8, 64, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 1141, 8, 64, 
		1, 64, 1, 64, 3, 64, 1145, 8, 64, 1, 64, 1, 64, 3, 64, 1149, 8, 64, 1, 
		64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 1157, 8, 64, 1, 65, 1, 65, 
		1, 65, 1, 65, 1, 65, 1, 65, 3, 65, 1165, 8, 65, 1, 66, 1, 66, 1, 67, 1, 
		67, 3, 67, 1171, 8, 67, 1, 67, 1, 67, 3, 67, 1175, 8, 67, 1, 67, 1, 67, 
		3, 67, 1179, 8, 67, 1, 67, 1, 67, 3, 67, 1183, 8, 67, 5, 67, 1185, 8, 
		67, 10, 67, 12, 67, 1188, 9, 67, 3, 67, 1190, 8, 67, 1, 67, 1, 67, 1, 
		68, 1, 68, 3, 68, 1196, 8, 68, 1, 68, 1, 68, 1, 68, 3, 68, 1201, 8, 68, 
		1, 68, 1, 68, 1, 68, 3, 68, 1206, 8, 68, 1, 68, 1, 68, 1, 68, 3, 68, 1211, 
		8, 68, 1, 68, 1, 68, 1, 68, 3, 68, 1216, 8, 68, 1, 68, 1, 68, 1, 68, 3, 
		68, 1221, 8, 68, 1, 68, 3, 68, 1224, 8, 68, 1, 69, 1, 69, 3, 69, 1228, 
		8, 69, 1, 69, 1, 69, 3, 69, 1232, 8, 69, 1, 69, 1, 69, 1, 70, 1, 70, 3, 
		70, 1238, 8, 70, 1, 70, 4, 70, 1241, 8, 70, 11, 70, 12, 70, 1242, 1, 71, 
		1, 71, 3, 71, 1247, 8, 71, 1, 71, 3, 71, 1250, 8, 71, 1, 72, 1, 72, 1, 
		72, 1, 72, 1, 72, 1, 72, 1, 73, 1, 73, 3, 73, 1260, 8, 73, 1, 73, 1, 73, 
		3, 73, 1264, 8, 73, 1, 73, 1, 73, 3, 73, 1268, 8, 73, 3, 73, 1270, 8, 
		73, 1, 73, 1, 73, 3, 73, 1274, 8, 73, 1, 73, 1, 73, 3, 73, 1278, 8, 73, 
		1, 73, 1, 73, 3, 73, 1282, 8, 73, 5, 73, 1284, 8, 73, 10, 73, 12, 73, 
		1287, 9, 73, 3, 73, 1289, 8, 73, 1, 73, 1, 73, 1, 74, 1, 74, 1, 74, 1, 
		74, 3, 74, 1297, 8, 74, 1, 75, 1, 75, 3, 75, 1301, 8, 75, 1, 75, 1, 75, 
		3, 75, 1305, 8, 75, 1, 75, 1, 75, 3, 75, 1309, 8, 75, 1, 75, 1, 75, 3, 
		75, 1313, 8, 75, 1, 75, 1, 75, 3, 75, 1317, 8, 75, 5, 75, 1319, 8, 75, 
		10, 75, 12, 75, 1322, 9, 75, 3, 75, 1324, 8, 75, 1, 75, 1, 75, 1, 76, 
		1, 76, 1, 77, 1, 77, 1, 78, 1, 78, 1, 78, 1, 79, 1, 79, 1, 79, 5, 79, 
		1338, 8, 79, 10, 79, 12, 79, 1341, 9, 79, 1, 80, 1, 80, 3, 80, 1345, 8, 
		80, 1, 80, 1, 80, 3, 80, 1349, 8, 80, 1, 80, 1, 80, 3, 80, 1353, 8, 80, 
		1, 80, 3, 80, 1356, 8, 80, 1, 80, 3, 80, 1359, 8, 80, 1, 80, 1, 80, 1, 
		81, 1, 81, 3, 81, 1365, 8, 81, 1, 81, 1, 81, 3, 81, 1369, 8, 81, 1, 81, 
		1, 81, 3, 81, 1373, 8, 81, 3, 81, 1375, 8, 81, 1, 81, 1, 81, 3, 81, 1379, 
		8, 81, 1, 81, 1, 81, 3, 81, 1383, 8, 81, 1, 81, 1, 81, 3, 81, 1387, 8, 
		81, 3, 81, 1389, 8, 81, 1, 81, 1, 81, 3, 81, 1393, 8, 81, 1, 81, 1, 81, 
		3, 81, 1397, 8, 81, 1, 81, 1, 81, 1, 82, 1, 82, 3, 82, 1403, 8, 82, 1, 
		82, 1, 82, 1, 83, 1, 83, 3, 83, 1409, 8, 83, 1, 83, 4, 83, 1412, 8, 83, 
		11, 83, 12, 83, 1413, 1, 83, 1, 83, 3, 83, 1418, 8, 83, 1, 83, 1, 83, 
		3, 83, 1422, 8, 83, 1, 83, 4, 83, 1425, 8, 83, 11, 83, 12, 83, 1426, 3, 
		83, 1429, 8, 83, 1, 83, 3, 83, 1432, 8, 83, 1, 83, 1, 83, 3, 83, 1436, 
		8, 83, 1, 83, 3, 83, 1439, 8, 83, 1, 83, 3, 83, 1442, 8, 83, 1, 83, 1, 
		83, 1, 84, 1, 84, 3, 84, 1448, 8, 84, 1, 84, 1, 84, 3, 84, 1452, 8, 84, 
		1, 84, 1, 84, 3, 84, 1456, 8, 84, 1, 84, 1, 84, 1, 85, 1, 85, 1, 86, 1, 
		86, 3, 86, 1464, 8, 86, 1, 87, 1, 87, 3, 87, 1468, 8, 87, 1, 87, 1, 87, 
		3, 87, 1472, 8, 87, 1, 87, 1, 87, 3, 87, 1476, 8, 87, 1, 87, 1, 87, 3, 
		87, 1480, 8, 87, 1, 87, 1, 87, 3, 87, 1484, 8, 87, 1, 87, 1, 87, 3, 87, 
		1488, 8, 87, 1, 87, 1, 87, 3, 87, 1492, 8, 87, 1, 87, 1, 87, 3, 87, 1496, 
		8, 87, 5, 87, 1498, 8, 87, 10, 87, 12, 87, 1501, 9, 87, 3, 87, 1503, 8, 
		87, 1, 87, 1, 87, 1, 88, 1, 88, 1, 88, 3, 88, 1510, 8, 88, 1, 89, 1, 89, 
		3, 89, 1514, 8, 89, 1, 89, 4, 89, 1517, 8, 89, 11, 89, 12, 89, 1518, 1, 
		90, 1, 90, 1, 91, 1, 91, 1, 92, 1, 92, 1, 93, 1, 93, 3, 93, 1529, 8, 93, 
		1, 94, 1, 94, 1, 95, 1, 95, 1, 96, 1, 96, 1, 97, 1, 97, 1, 98, 1, 98, 
		1, 98, 0, 0, 99, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 
		30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 
		66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 100, 
		102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 128, 
		130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 
		158, 160, 162, 164, 166, 168, 170, 172, 174, 176, 178, 180, 182, 184, 
		186, 188, 190, 192, 194, 196, 0, 10, 1, 0, 68, 71, 1, 0, 13, 14, 1, 0, 
		87, 88, 1, 0, 97, 99, 1, 0, 107, 108, 4, 0, 46, 58, 61, 82, 87, 94, 109, 
		118, 4, 0, 83, 86, 100, 100, 119, 121, 124, 124, 2, 0, 19, 19, 27, 30, 
		2, 0, 20, 20, 31, 34, 2, 0, 14, 14, 35, 45, 1756, 0, 199, 1, 0, 0, 0, 
		2, 213, 1, 0, 0, 0, 4, 217, 1, 0, 0, 0, 6, 219, 1, 0, 0, 0, 8, 241, 1, 
		0, 0, 0, 10, 245, 1, 0, 0, 0, 12, 282, 1, 0, 0, 0, 14, 306, 1, 0, 0, 0, 
		16, 317, 1, 0, 0, 0, 18, 322, 1, 0, 0, 0, 20, 326, 1, 0, 0, 0, 22, 339, 
		1, 0, 0, 0, 24, 349, 1, 0, 0, 0, 26, 371, 1, 0, 0, 0, 28, 373, 1, 0, 0, 
		0, 30, 379, 1, 0, 0, 0, 32, 427, 1, 0, 0, 0, 34, 431, 1, 0, 0, 0, 36, 
		451, 1, 0, 0, 0, 38, 471, 1, 0, 0, 0, 40, 473, 1, 0, 0, 0, 42, 484, 1, 
		0, 0, 0, 44, 511, 1, 0, 0, 0, 46, 524, 1, 0, 0, 0, 48, 528, 1, 0, 0, 0, 
		50, 536, 1, 0, 0, 0, 52, 543, 1, 0, 0, 0, 54, 587, 1, 0, 0, 0, 56, 596, 
		1, 0, 0, 0, 58, 598, 1, 0, 0, 0, 60, 613, 1, 0, 0, 0, 62, 617, 1, 0, 0, 
		0, 64, 621, 1, 0, 0, 0, 66, 628, 1, 0, 0, 0, 68, 632, 1, 0, 0, 0, 70, 
		657, 1, 0, 0, 0, 72, 659, 1, 0, 0, 0, 74, 675, 1, 0, 0, 0, 76, 677, 1, 
		0, 0, 0, 78, 701, 1, 0, 0, 0, 80, 771, 1, 0, 0, 0, 82, 773, 1, 0, 0, 0, 
		84, 802, 1, 0, 0, 0, 86, 804, 1, 0, 0, 0, 88, 825, 1, 0, 0, 0, 90, 835, 
		1, 0, 0, 0, 92, 841, 1, 0, 0, 0, 94, 863, 1, 0, 0, 0, 96, 865, 1, 0, 0, 
		0, 98, 867, 1, 0, 0, 0, 100, 869, 1, 0, 0, 0, 102, 879, 1, 0, 0, 0, 104, 
		889, 1, 0, 0, 0, 106, 905, 1, 0, 0, 0, 108, 910, 1, 0, 0, 0, 110, 920, 
		1, 0, 0, 0, 112, 942, 1, 0, 0, 0, 114, 972, 1, 0, 0, 0, 116, 992, 1, 0, 
		0, 0, 118, 997, 1, 0, 0, 0, 120, 1031, 1, 0, 0, 0, 122, 1043, 1, 0, 0, 
		0, 124, 1060, 1, 0, 0, 0, 126, 1062, 1, 0, 0, 0, 128, 1156, 1, 0, 0, 0, 
		130, 1164, 1, 0, 0, 0, 132, 1166, 1, 0, 0, 0, 134, 1168, 1, 0, 0, 0, 136, 
		1223, 1, 0, 0, 0, 138, 1225, 1, 0, 0, 0, 140, 1235, 1, 0, 0, 0, 142, 1244, 
		1, 0, 0, 0, 144, 1251, 1, 0, 0, 0, 146, 1257, 1, 0, 0, 0, 148, 1296, 1, 
		0, 0, 0, 150, 1298, 1, 0, 0, 0, 152, 1327, 1, 0, 0, 0, 154, 1329, 1, 0, 
		0, 0, 156, 1331, 1, 0, 0, 0, 158, 1339, 1, 0, 0, 0, 160, 1342, 1, 0, 0, 
		0, 162, 1362, 1, 0, 0, 0, 164, 1400, 1, 0, 0, 0, 166, 1428, 1, 0, 0, 0, 
		168, 1445, 1, 0, 0, 0, 170, 1459, 1, 0, 0, 0, 172, 1463, 1, 0, 0, 0, 174, 
		1465, 1, 0, 0, 0, 176, 1506, 1, 0, 0, 0, 178, 1511, 1, 0, 0, 0, 180, 1520, 
		1, 0, 0, 0, 182, 1522, 1, 0, 0, 0, 184, 1524, 1, 0, 0, 0, 186, 1528, 1, 
		0, 0, 0, 188, 1530, 1, 0, 0, 0, 190, 1532, 1, 0, 0, 0, 192, 1534, 1, 0, 
		0, 0, 194, 1536, 1, 0, 0, 0, 196, 1538, 1, 0, 0, 0, 198, 200, 5, 125, 
		0, 0, 199, 198, 1, 0, 0, 0, 199, 200, 1, 0, 0, 0, 200, 201, 1, 0, 0, 0, 
		201, 206, 3, 2, 1, 0, 202, 204, 5, 125, 0, 0, 203, 202, 1, 0, 0, 0, 203, 
		204, 1, 0, 0, 0, 204, 205, 1, 0, 0, 0, 205, 207, 5, 1, 0, 0, 206, 203, 
		1, 0, 0, 0, 206, 207, 1, 0, 0, 0, 207, 209, 1, 0, 0, 0, 208, 210, 5, 125, 
		0, 0, 209, 208, 1, 0, 0, 0, 209, 210, 1, 0, 0, 0, 210, 211, 1, 0, 0, 0, 
		211, 212, 5, 0, 0, 1, 212, 1, 1, 0, 0, 0, 213, 214, 3, 4, 2, 0, 214, 3, 
		1, 0, 0, 0, 215, 218, 3, 6, 3, 0, 216, 218, 3, 42, 21, 0, 217, 215, 1, 
		0, 0, 0, 217, 216, 1, 0, 0, 0, 218, 5, 1, 0, 0, 0, 219, 226, 3, 10, 5, 
		0, 220, 222, 5, 125, 0, 0, 221, 220, 1, 0, 0, 0, 221, 222, 1, 0, 0, 0, 
		222, 223, 1, 0, 0, 0, 223, 225, 3, 8, 4, 0, 224, 221, 1, 0, 0, 0, 225, 
		228, 1, 0, 0, 0, 226, 224, 1, 0, 0, 0, 226, 227, 1, 0, 0, 0, 227, 7, 1, 
		0, 0, 0, 228, 226, 1, 0, 0, 0, 229, 230, 5, 46, 0, 0, 230, 231, 5, 125, 
		0, 0, 231, 233, 5, 47, 0, 0, 232, 234, 5, 125, 0, 0, 233, 232, 1, 0, 0, 
		0, 233, 234, 1, 0, 0, 0, 234, 235, 1, 0, 0, 0, 235, 242, 3, 10, 5, 0, 
		236, 238, 5, 46, 0, 0, 237, 239, 5, 125, 0, 0, 238, 237, 1, 0, 0, 0, 238, 
		239, 1, 0, 0, 0, 239, 240, 1, 0, 0, 0, 240, 242, 3, 10, 5, 0, 241, 229, 
		1, 0, 0, 0, 241, 236, 1, 0, 0, 0, 242, 9, 1, 0, 0, 0, 243, 246, 3, 12, 
		6, 0, 244, 246, 3, 14, 7, 0, 245, 243, 1, 0, 0, 0, 245, 244, 1, 0, 0, 
		0, 246, 11, 1, 0, 0, 0, 247, 249, 3, 18, 9, 0, 248, 250, 5, 125, 0, 0, 
		249, 248, 1, 0, 0, 0, 249, 250, 1, 0, 0, 0, 250, 252, 1, 0, 0, 0, 251, 
		247, 1, 0, 0, 0, 252, 255, 1, 0, 0, 0, 253, 251, 1, 0, 0, 0, 253, 254, 
		1, 0, 0, 0, 254, 256, 1, 0, 0, 0, 255, 253, 1, 0, 0, 0, 256, 283, 3, 50, 
		25, 0, 257, 259, 3, 18, 9, 0, 258, 260, 5, 125, 0, 0, 259, 258, 1, 0, 
		0, 0, 259, 260, 1, 0, 0, 0, 260, 262, 1, 0, 0, 0, 261, 257, 1, 0, 0, 0, 
		262, 265, 1, 0, 0, 0, 263, 261, 1, 0, 0, 0, 263, 264, 1, 0, 0, 0, 264, 
		266, 1, 0, 0, 0, 265, 263, 1, 0, 0, 0, 266, 273, 3, 16, 8, 0, 267, 269, 
		5, 125, 0, 0, 268, 267, 1, 0, 0, 0, 268, 269, 1, 0, 0, 0, 269, 270, 1, 
		0, 0, 0, 270, 272, 3, 16, 8, 0, 271, 268, 1, 0, 0, 0, 272, 275, 1, 0, 
		0, 0, 273, 271, 1, 0, 0, 0, 273, 274, 1, 0, 0, 0, 274, 280, 1, 0, 0, 0, 
		275, 273, 1, 0, 0, 0, 276, 278, 5, 125, 0, 0, 277, 276, 1, 0, 0, 0, 277, 
		278, 1, 0, 0, 0, 278, 279, 1, 0, 0, 0, 279, 281, 3, 50, 25, 0, 280, 277, 
		1, 0, 0, 0, 280, 281, 1, 0, 0, 0, 281, 283, 1, 0, 0, 0, 282, 253, 1, 0, 
		0, 0, 282, 263, 1, 0, 0, 0, 283, 13, 1, 0, 0, 0, 284, 286, 3, 18, 9, 0, 
		285, 287, 5, 125, 0, 0, 286, 285, 1, 0, 0, 0, 286, 287, 1, 0, 0, 0, 287, 
		289, 1, 0, 0, 0, 288, 284, 1, 0, 0, 0, 289, 292, 1, 0, 0, 0, 290, 288, 
		1, 0, 0, 0, 290, 291, 1, 0, 0, 0, 291, 299, 1, 0, 0, 0, 292, 290, 1, 0, 
		0, 0, 293, 295, 3, 16, 8, 0, 294, 296, 5, 125, 0, 0, 295, 294, 1, 0, 0, 
		0, 295, 296, 1, 0, 0, 0, 296, 298, 1, 0, 0, 0, 297, 293, 1, 0, 0, 0, 298, 
		301, 1, 0, 0, 0, 299, 297, 1, 0, 0, 0, 299, 300, 1, 0, 0, 0, 300, 302, 
		1, 0, 0, 0, 301, 299, 1, 0, 0, 0, 302, 304, 3, 48, 24, 0, 303, 305, 5, 
		125, 0, 0, 304, 303, 1, 0, 0, 0, 304, 305, 1, 0, 0, 0, 305, 307, 1, 0, 
		0, 0, 306, 290, 1, 0, 0, 0, 307, 308, 1, 0, 0, 0, 308, 306, 1, 0, 0, 0, 
		308, 309, 1, 0, 0, 0, 309, 310, 1, 0, 0, 0, 310, 311, 3, 12, 6, 0, 311, 
		15, 1, 0, 0, 0, 312, 318, 3, 28, 14, 0, 313, 318, 3, 24, 12, 0, 314, 318, 
		3, 34, 17, 0, 315, 318, 3, 30, 15, 0, 316, 318, 3, 36, 18, 0, 317, 312, 
		1, 0, 0, 0, 317, 313, 1, 0, 0, 0, 317, 314, 1, 0, 0, 0, 317, 315, 1, 0, 
		0, 0, 317, 316, 1, 0, 0, 0, 318, 17, 1, 0, 0, 0, 319, 323, 3, 20, 10, 
		0, 320, 323, 3, 22, 11, 0, 321, 323, 3, 40, 20, 0, 322, 319, 1, 0, 0, 
		0, 322, 320, 1, 0, 0, 0, 322, 321, 1, 0, 0, 0, 323, 19, 1, 0, 0, 0, 324, 
		325, 5, 48, 0, 0, 325, 327, 5, 125, 0, 0, 326, 324, 1, 0, 0, 0, 326, 327, 
		1, 0, 0, 0, 327, 328, 1, 0, 0, 0, 328, 330, 5, 49, 0, 0, 329, 331, 5, 
		125, 0, 0, 330, 329, 1, 0, 0, 0, 330, 331, 1, 0, 0, 0, 331, 332, 1, 0, 
		0, 0, 332, 337, 3, 68, 34, 0, 333, 335, 5, 125, 0, 0, 334, 333, 1, 0, 
		0, 0, 334, 335, 1, 0, 0, 0, 335, 336, 1, 0, 0, 0, 336, 338, 3, 66, 33, 
		0, 337, 334, 1, 0, 0, 0, 337, 338, 1, 0, 0, 0, 338, 21, 1, 0, 0, 0, 339, 
		341, 5, 50, 0, 0, 340, 342, 5, 125, 0, 0, 341, 340, 1, 0, 0, 0, 341, 342, 
		1, 0, 0, 0, 342, 343, 1, 0, 0, 0, 343, 344, 3, 98, 49, 0, 344, 345, 5, 
		125, 0, 0, 345, 346, 5, 51, 0, 0, 346, 347, 5, 125, 0, 0, 347, 348, 3, 
		170, 85, 0, 348, 23, 1, 0, 0, 0, 349, 351, 5, 52, 0, 0, 350, 352, 5, 125, 
		0, 0, 351, 350, 1, 0, 0, 0, 351, 352, 1, 0, 0, 0, 352, 353, 1, 0, 0, 0, 
		353, 358, 3, 70, 35, 0, 354, 355, 5, 125, 0, 0, 355, 357, 3, 26, 13, 0, 
		356, 354, 1, 0, 0, 0, 357, 360, 1, 0, 0, 0, 358, 356, 1, 0, 0, 0, 358, 
		359, 1, 0, 0, 0, 359, 25, 1, 0, 0, 0, 360, 358, 1, 0, 0, 0, 361, 362, 
		5, 53, 0, 0, 362, 363, 5, 125, 0, 0, 363, 364, 5, 49, 0, 0, 364, 365, 
		5, 125, 0, 0, 365, 372, 3, 30, 15, 0, 366, 367, 5, 53, 0, 0, 367, 368, 
		5, 125, 0, 0, 368, 369, 5, 54, 0, 0, 369, 370, 5, 125, 0, 0, 370, 372, 
		3, 30, 15, 0, 371, 361, 1, 0, 0, 0, 371, 366, 1, 0, 0, 0, 372, 27, 1, 
		0, 0, 0, 373, 375, 5, 54, 0, 0, 374, 376, 5, 125, 0, 0, 375, 374, 1, 0, 
		0, 0, 375, 376, 1, 0, 0, 0, 376, 377, 1, 0, 0, 0, 377, 378, 3, 68, 34, 
		0, 378, 29, 1, 0, 0, 0, 379, 381, 5, 55, 0, 0, 380, 382, 5, 125, 0, 0, 
		381, 380, 1, 0, 0, 0, 381, 382, 1, 0, 0, 0, 382, 383, 1, 0, 0, 0, 383, 
		388, 3, 32, 16, 0, 384, 385, 5, 2, 0, 0, 385, 387, 3, 32, 16, 0, 386, 
		384, 1, 0, 0, 0, 387, 390, 1, 0, 0, 0, 388, 386, 1, 0, 0, 0, 388, 389, 
		1, 0, 0, 0, 389, 31, 1, 0, 0, 0, 390, 388, 1, 0, 0, 0, 391, 393, 3, 178, 
		89, 0, 392, 394, 5, 125, 0, 0, 393, 392, 1, 0, 0, 0, 393, 394, 1, 0, 0, 
		0, 394, 395, 1, 0, 0, 0, 395, 397, 5, 3, 0, 0, 396, 398, 5, 125, 0, 0, 
		397, 396, 1, 0, 0, 0, 397, 398, 1, 0, 0, 0, 398, 399, 1, 0, 0, 0, 399, 
		400, 3, 98, 49, 0, 400, 428, 1, 0, 0, 0, 401, 403, 3, 170, 85, 0, 402, 
		404, 5, 125, 0, 0, 403, 402, 1, 0, 0, 0, 403, 404, 1, 0, 0, 0, 404, 405, 
		1, 0, 0, 0, 405, 407, 5, 3, 0, 0, 406, 408, 5, 125, 0, 0, 407, 406, 1, 
		0, 0, 0, 407, 408, 1, 0, 0, 0, 408, 409, 1, 0, 0, 0, 409, 410, 3, 98, 
		49, 0, 410, 428, 1, 0, 0, 0, 411, 413, 3, 170, 85, 0, 412, 414, 5, 125, 
		0, 0, 413, 412, 1, 0, 0, 0, 413, 414, 1, 0, 0, 0, 414, 415, 1, 0, 0, 0, 
		415, 417, 5, 4, 0, 0, 416, 418, 5, 125, 0, 0, 417, 416, 1, 0, 0, 0, 417, 
		418, 1, 0, 0, 0, 418, 419, 1, 0, 0, 0, 419, 420, 3, 98, 49, 0, 420, 428, 
		1, 0, 0, 0, 421, 423, 3, 170, 85, 0, 422, 424, 5, 125, 0, 0, 423, 422, 
		1, 0, 0, 0, 423, 424, 1, 0, 0, 0, 424, 425, 1, 0, 0, 0, 425, 426, 3, 88, 
		44, 0, 426, 428, 1, 0, 0, 0, 427, 391, 1, 0, 0, 0, 427, 401, 1, 0, 0, 
		0, 427, 411, 1, 0, 0, 0, 427, 421, 1, 0, 0, 0, 428, 33, 1, 0, 0, 0, 429, 
		430, 5, 56, 0, 0, 430, 432, 5, 125, 0, 0, 431, 429, 1, 0, 0, 0, 431, 432, 
		1, 0, 0, 0, 432, 433, 1, 0, 0, 0, 433, 435, 5, 57, 0, 0, 434, 436, 5, 
		125, 0, 0, 435, 434, 1, 0, 0, 0, 435, 436, 1, 0, 0, 0, 436, 437, 1, 0, 
		0, 0, 437, 448, 3, 98, 49, 0, 438, 440, 5, 125, 0, 0, 439, 438, 1, 0, 
		0, 0, 439, 440, 1, 0, 0, 0, 440, 441, 1, 0, 0, 0, 441, 443, 5, 2, 0, 0, 
		442, 444, 5, 125, 0, 0, 443, 442, 1, 0, 0, 0, 443, 444, 1, 0, 0, 0, 444, 
		445, 1, 0, 0, 0, 445, 447, 3, 98, 49, 0, 446, 439, 1, 0, 0, 0, 447, 450, 
		1, 0, 0, 0, 448, 446, 1, 0, 0, 0, 448, 449, 1, 0, 0, 0, 449, 35, 1, 0, 
		0, 0, 450, 448, 1, 0, 0, 0, 451, 452, 5, 58, 0, 0, 452, 453, 5, 125, 0, 
		0, 453, 464, 3, 38, 19, 0, 454, 456, 5, 125, 0, 0, 455, 454, 1, 0, 0, 
		0, 455, 456, 1, 0, 0, 0, 456, 457, 1, 0, 0, 0, 457, 459, 5, 2, 0, 0, 458, 
		460, 5, 125, 0, 0, 459, 458, 1, 0, 0, 0, 459, 460, 1, 0, 0, 0, 460, 461, 
		1, 0, 0, 0, 461, 463, 3, 38, 19, 0, 462, 455, 1, 0, 0, 0, 463, 466, 1, 
		0, 0, 0, 464, 462, 1, 0, 0, 0, 464, 465, 1, 0, 0, 0, 465, 37, 1, 0, 0, 
		0, 466, 464, 1, 0, 0, 0, 467, 468, 3, 170, 85, 0, 468, 469, 3, 88, 44, 
		0, 469, 472, 1, 0, 0, 0, 470, 472, 3, 178, 89, 0, 471, 467, 1, 0, 0, 0, 
		471, 470, 1, 0, 0, 0, 472, 39, 1, 0, 0, 0, 473, 474, 5, 59, 0, 0, 474, 
		475, 5, 125, 0, 0, 475, 482, 3, 150, 75, 0, 476, 478, 5, 125, 0, 0, 477, 
		476, 1, 0, 0, 0, 477, 478, 1, 0, 0, 0, 478, 479, 1, 0, 0, 0, 479, 480, 
		5, 60, 0, 0, 480, 481, 5, 125, 0, 0, 481, 483, 3, 44, 22, 0, 482, 477, 
		1, 0, 0, 0, 482, 483, 1, 0, 0, 0, 483, 41, 1, 0, 0, 0, 484, 485, 5, 59, 
		0, 0, 485, 488, 5, 125, 0, 0, 486, 489, 3, 150, 75, 0, 487, 489, 3, 152, 
		76, 0, 488, 486, 1, 0, 0, 0, 488, 487, 1, 0, 0, 0, 489, 494, 1, 0, 0, 
		0, 490, 491, 5, 125, 0, 0, 491, 492, 5, 60, 0, 0, 492, 493, 5, 125, 0, 
		0, 493, 495, 3, 44, 22, 0, 494, 490, 1, 0, 0, 0, 494, 495, 1, 0, 0, 0, 
		495, 43, 1, 0, 0, 0, 496, 512, 5, 5, 0, 0, 497, 508, 3, 46, 23, 0, 498, 
		500, 5, 125, 0, 0, 499, 498, 1, 0, 0, 0, 499, 500, 1, 0, 0, 0, 500, 501, 
		1, 0, 0, 0, 501, 503, 5, 2, 0, 0, 502, 504, 5, 125, 0, 0, 503, 502, 1, 
		0, 0, 0, 503, 504, 1, 0, 0, 0, 504, 505, 1, 0, 0, 0, 505, 507, 3, 46, 
		23, 0, 506, 499, 1, 0, 0, 0, 507, 510, 1, 0, 0, 0, 508, 506, 1, 0, 0, 
		0, 508, 509, 1, 0, 0, 0, 509, 512, 1, 0, 0, 0, 510, 508, 1, 0, 0, 0, 511, 
		496, 1, 0, 0, 0, 511, 497, 1, 0, 0, 0, 512, 517, 1, 0, 0, 0, 513, 515, 
		5, 125, 0, 0, 514, 513, 1, 0, 0, 0, 514, 515, 1, 0, 0, 0, 515, 516, 1, 
		0, 0, 0, 516, 518, 3, 66, 33, 0, 517, 514, 1, 0, 0, 0, 517, 518, 1, 0, 
		0, 0, 518, 45, 1, 0, 0, 0, 519, 520, 3, 154, 77, 0, 520, 521, 5, 125, 
		0, 0, 521, 522, 5, 51, 0, 0, 522, 523, 5, 125, 0, 0, 523, 525, 1, 0, 0, 
		0, 524, 519, 1, 0, 0, 0, 524, 525, 1, 0, 0, 0, 525, 526, 1, 0, 0, 0, 526, 
		527, 3, 170, 85, 0, 527, 47, 1, 0, 0, 0, 528, 529, 5, 61, 0, 0, 529, 534, 
		3, 52, 26, 0, 530, 532, 5, 125, 0, 0, 531, 530, 1, 0, 0, 0, 531, 532, 
		1, 0, 0, 0, 532, 533, 1, 0, 0, 0, 533, 535, 3, 66, 33, 0, 534, 531, 1, 
		0, 0, 0, 534, 535, 1, 0, 0, 0, 535, 49, 1, 0, 0, 0, 536, 537, 5, 62, 0, 
		0, 537, 538, 3, 52, 26, 0, 538, 51, 1, 0, 0, 0, 539, 541, 5, 125, 0, 0, 
		540, 539, 1, 0, 0, 0, 540, 541, 1, 0, 0, 0, 541, 542, 1, 0, 0, 0, 542, 
		544, 5, 63, 0, 0, 543, 540, 1, 0, 0, 0, 543, 544, 1, 0, 0, 0, 544, 545, 
		1, 0, 0, 0, 545, 546, 5, 125, 0, 0, 546, 549, 3, 54, 27, 0, 547, 548, 
		5, 125, 0, 0, 548, 550, 3, 58, 29, 0, 549, 547, 1, 0, 0, 0, 549, 550, 
		1, 0, 0, 0, 550, 553, 1, 0, 0, 0, 551, 552, 5, 125, 0, 0, 552, 554, 3, 
		60, 30, 0, 553, 551, 1, 0, 0, 0, 553, 554, 1, 0, 0, 0, 554, 557, 1, 0, 
		0, 0, 555, 556, 5, 125, 0, 0, 556, 558, 3, 62, 31, 0, 557, 555, 1, 0, 
		0, 0, 557, 558, 1, 0, 0, 0, 558, 53, 1, 0, 0, 0, 559, 570, 5, 5, 0, 0, 
		560, 562, 5, 125, 0, 0, 561, 560, 1, 0, 0, 0, 561, 562, 1, 0, 0, 0, 562, 
		563, 1, 0, 0, 0, 563, 565, 5, 2, 0, 0, 564, 566, 5, 125, 0, 0, 565, 564, 
		1, 0, 0, 0, 565, 566, 1, 0, 0, 0, 566, 567, 1, 0, 0, 0, 567, 569, 3, 56, 
		28, 0, 568, 561, 1, 0, 0, 0, 569, 572, 1, 0, 0, 0, 570, 568, 1, 0, 0, 
		0, 570, 571, 1, 0, 0, 0, 571, 588, 1, 0, 0, 0, 572, 570, 1, 0, 0, 0, 573, 
		584, 3, 56, 28, 0, 574, 576, 5, 125, 0, 0, 575, 574, 1, 0, 0, 0, 575, 
		576, 1, 0, 0, 0, 576, 577, 1, 0, 0, 0, 577, 579, 5, 2, 0, 0, 578, 580, 
		5, 125, 0, 0, 579, 578, 1, 0, 0, 0, 579, 580, 1, 0, 0, 0, 580, 581, 1, 
		0, 0, 0, 581, 583, 3, 56, 28, 0, 582, 575, 1, 0, 0, 0, 583, 586, 1, 0, 
		0, 0, 584, 582, 1, 0, 0, 0, 584, 585, 1, 0, 0, 0, 585, 588, 1, 0, 0, 0, 
		586, 584, 1, 0, 0, 0, 587, 559, 1, 0, 0, 0, 587, 573, 1, 0, 0, 0, 588, 
		55, 1, 0, 0, 0, 589, 590, 3, 98, 49, 0, 590, 591, 5, 125, 0, 0, 591, 592, 
		5, 51, 0, 0, 592, 593, 5, 125, 0, 0, 593, 594, 3, 170, 85, 0, 594, 597, 
		1, 0, 0, 0, 595, 597, 3, 98, 49, 0, 596, 589, 1, 0, 0, 0, 596, 595, 1, 
		0, 0, 0, 597, 57, 1, 0, 0, 0, 598, 599, 5, 64, 0, 0, 599, 600, 5, 125, 
		0, 0, 600, 601, 5, 65, 0, 0, 601, 602, 5, 125, 0, 0, 602, 610, 3, 64, 
		32, 0, 603, 605, 5, 2, 0, 0, 604, 606, 5, 125, 0, 0, 605, 604, 1, 0, 0, 
		0, 605, 606, 1, 0, 0, 0, 606, 607, 1, 0, 0, 0, 607, 609, 3, 64, 32, 0, 
		608, 603, 1, 0, 0, 0, 609, 612, 1, 0, 0, 0, 610, 608, 1, 0, 0, 0, 610, 
		611, 1, 0, 0, 0, 611, 59, 1, 0, 0, 0, 612, 610, 1, 0, 0, 0, 613, 614, 
		5, 66, 0, 0, 614, 615, 5, 125, 0, 0, 615, 616, 3, 98, 49, 0, 616, 61, 
		1, 0, 0, 0, 617, 618, 5, 67, 0, 0, 618, 619, 5, 125, 0, 0, 619, 620, 3, 
		98, 49, 0, 620, 63, 1, 0, 0, 0, 621, 626, 3, 98, 49, 0, 622, 624, 5, 125, 
		0, 0, 623, 622, 1, 0, 0, 0, 623, 624, 1, 0, 0, 0, 624, 625, 1, 0, 0, 0, 
		625, 627, 7, 0, 0, 0, 626, 623, 1, 0, 0, 0, 626, 627, 1, 0, 0, 0, 627, 
		65, 1, 0, 0, 0, 628, 629, 5, 72, 0, 0, 629, 630, 5, 125, 0, 0, 630, 631, 
		3, 98, 49, 0, 631, 67, 1, 0, 0, 0, 632, 643, 3, 70, 35, 0, 633, 635, 5, 
		125, 0, 0, 634, 633, 1, 0, 0, 0, 634, 635, 1, 0, 0, 0, 635, 636, 1, 0, 
		0, 0, 636, 638, 5, 2, 0, 0, 637, 639, 5, 125, 0, 0, 638, 637, 1, 0, 0, 
		0, 638, 639, 1, 0, 0, 0, 639, 640, 1, 0, 0, 0, 640, 642, 3, 70, 35, 0, 
		641, 634, 1, 0, 0, 0, 642, 645, 1, 0, 0, 0, 643, 641, 1, 0, 0, 0, 643, 
		644, 1, 0, 0, 0, 644, 69, 1, 0, 0, 0, 645, 643, 1, 0, 0, 0, 646, 648, 
		3, 170, 85, 0, 647, 649, 5, 125, 0, 0, 648, 647, 1, 0, 0, 0, 648, 649, 
		1, 0, 0, 0, 649, 650, 1, 0, 0, 0, 650, 652, 5, 3, 0, 0, 651, 653, 5, 125, 
		0, 0, 652, 651, 1, 0, 0, 0, 652, 653, 1, 0, 0, 0, 653, 654, 1, 0, 0, 0, 
		654, 655, 3, 72, 36, 0, 655, 658, 1, 0, 0, 0, 656, 658, 3, 72, 36, 0, 
		657, 646, 1, 0, 0, 0, 657, 656, 1, 0, 0, 0, 658, 71, 1, 0, 0, 0, 659, 
		660, 3, 74, 37, 0, 660, 73, 1, 0, 0, 0, 661, 668, 3, 76, 38, 0, 662, 664, 
		5, 125, 0, 0, 663, 662, 1, 0, 0, 0, 663, 664, 1, 0, 0, 0, 664, 665, 1, 
		0, 0, 0, 665, 667, 3, 78, 39, 0, 666, 663, 1, 0, 0, 0, 667, 670, 1, 0, 
		0, 0, 668, 666, 1, 0, 0, 0, 668, 669, 1, 0, 0, 0, 669, 676, 1, 0, 0, 0, 
		670, 668, 1, 0, 0, 0, 671, 672, 5, 6, 0, 0, 672, 673, 3, 74, 37, 0, 673, 
		674, 5, 7, 0, 0, 674, 676, 1, 0, 0, 0, 675, 661, 1, 0, 0, 0, 675, 671, 
		1, 0, 0, 0, 676, 75, 1, 0, 0, 0, 677, 679, 5, 6, 0, 0, 678, 680, 5, 125, 
		0, 0, 679, 678, 1, 0, 0, 0, 679, 680, 1, 0, 0, 0, 680, 685, 1, 0, 0, 0, 
		681, 683, 3, 170, 85, 0, 682, 684, 5, 125, 0, 0, 683, 682, 1, 0, 0, 0, 
		683, 684, 1, 0, 0, 0, 684, 686, 1, 0, 0, 0, 685, 681, 1, 0, 0, 0, 685, 
		686, 1, 0, 0, 0, 686, 691, 1, 0, 0, 0, 687, 689, 3, 88, 44, 0, 688, 690, 
		5, 125, 0, 0, 689, 688, 1, 0, 0, 0, 689, 690, 1, 0, 0, 0, 690, 692, 1, 
		0, 0, 0, 691, 687, 1, 0, 0, 0, 691, 692, 1, 0, 0, 0, 692, 697, 1, 0, 0, 
		0, 693, 695, 3, 84, 42, 0, 694, 696, 5, 125, 0, 0, 695, 694, 1, 0, 0, 
		0, 695, 696, 1, 0, 0, 0, 696, 698, 1, 0, 0, 0, 697, 693, 1, 0, 0, 0, 697, 
		698, 1, 0, 0, 0, 698, 699, 1, 0, 0, 0, 699, 700, 5, 7, 0, 0, 700, 77, 
		1, 0, 0, 0, 701, 703, 3, 80, 40, 0, 702, 704, 5, 125, 0, 0, 703, 702, 
		1, 0, 0, 0, 703, 704, 1, 0, 0, 0, 704, 705, 1, 0, 0, 0, 705, 706, 3, 76, 
		38, 0, 706, 79, 1, 0, 0, 0, 707, 709, 3, 192, 96, 0, 708, 710, 5, 125, 
		0, 0, 709, 708, 1, 0, 0, 0, 709, 710, 1, 0, 0, 0, 710, 711, 1, 0, 0, 0, 
		711, 713, 3, 196, 98, 0, 712, 714, 5, 125, 0, 0, 713, 712, 1, 0, 0, 0, 
		713, 714, 1, 0, 0, 0, 714, 716, 1, 0, 0, 0, 715, 717, 3, 82, 41, 0, 716, 
		715, 1, 0, 0, 0, 716, 717, 1, 0, 0, 0, 717, 719, 1, 0, 0, 0, 718, 720, 
		5, 125, 0, 0, 719, 718, 1, 0, 0, 0, 719, 720, 1, 0, 0, 0, 720, 721, 1, 
		0, 0, 0, 721, 723, 3, 196, 98, 0, 722, 724, 5, 125, 0, 0, 723, 722, 1, 
		0, 0, 0, 723, 724, 1, 0, 0, 0, 724, 725, 1, 0, 0, 0, 725, 726, 3, 194, 
		97, 0, 726, 772, 1, 0, 0, 0, 727, 729, 3, 192, 96, 0, 728, 730, 5, 125, 
		0, 0, 729, 728, 1, 0, 0, 0, 729, 730, 1, 0, 0, 0, 730, 731, 1, 0, 0, 0, 
		731, 733, 3, 196, 98, 0, 732, 734, 5, 125, 0, 0, 733, 732, 1, 0, 0, 0, 
		733, 734, 1, 0, 0, 0, 734, 736, 1, 0, 0, 0, 735, 737, 3, 82, 41, 0, 736, 
		735, 1, 0, 0, 0, 736, 737, 1, 0, 0, 0, 737, 739, 1, 0, 0, 0, 738, 740, 
		5, 125, 0, 0, 739, 738, 1, 0, 0, 0, 739, 740, 1, 0, 0, 0, 740, 741, 1, 
		0, 0, 0, 741, 742, 3, 196, 98, 0, 742, 772, 1, 0, 0, 0, 743, 745, 3, 196, 
		98, 0, 744, 746, 5, 125, 0, 0, 745, 744, 1, 0, 0, 0, 745, 746, 1, 0, 0, 
		0, 746, 748, 1, 0, 0, 0, 747, 749, 3, 82, 41, 0, 748, 747, 1, 0, 0, 0, 
		748, 749, 1, 0, 0, 0, 749, 751, 1, 0, 0, 0, 750, 752, 5, 125, 0, 0, 751, 
		750, 1, 0, 0, 0, 751, 752, 1, 0, 0, 0, 752, 753, 1, 0, 0, 0, 753, 755, 
		3, 196, 98, 0, 754, 756, 5, 125, 0, 0, 755, 754, 1, 0, 0, 0, 755, 756, 
		1, 0, 0, 0, 756, 757, 1, 0, 0, 0, 757, 758, 3, 194, 97, 0, 758, 772, 1, 
		0, 0, 0, 759, 761, 3, 196, 98, 0, 760, 762, 5, 125, 0, 0, 761, 760, 1, 
		0, 0, 0, 761, 762, 1, 0, 0, 0, 762, 764, 1, 0, 0, 0, 763, 765, 3, 82, 
		41, 0, 764, 763, 1, 0, 0, 0, 764, 765, 1, 0, 0, 0, 765, 767, 1, 0, 0, 
		0, 766, 768, 5, 125, 0, 0, 767, 766, 1, 0, 0, 0, 767, 768, 1, 0, 0, 0, 
		768, 769, 1, 0, 0, 0, 769, 770, 3, 196, 98, 0, 770, 772, 1, 0, 0, 0, 771, 
		707, 1, 0, 0, 0, 771, 727, 1, 0, 0, 0, 771, 743, 1, 0, 0, 0, 771, 759, 
		1, 0, 0, 0, 772, 81, 1, 0, 0, 0, 773, 775, 5, 8, 0, 0, 774, 776, 5, 125, 
		0, 0, 775, 774, 1, 0, 0, 0, 775, 776, 1, 0, 0, 0, 776, 781, 1, 0, 0, 0, 
		777, 779, 3, 170, 85, 0, 778, 780, 5, 125, 0, 0, 779, 778, 1, 0, 0, 0, 
		779, 780, 1, 0, 0, 0, 780, 782, 1, 0, 0, 0, 781, 777, 1, 0, 0, 0, 781, 
		782, 1, 0, 0, 0, 782, 787, 1, 0, 0, 0, 783, 785, 3, 86, 43, 0, 784, 786, 
		5, 125, 0, 0, 785, 784, 1, 0, 0, 0, 785, 786, 1, 0, 0, 0, 786, 788, 1, 
		0, 0, 0, 787, 783, 1, 0, 0, 0, 787, 788, 1, 0, 0, 0, 788, 790, 1, 0, 0, 
		0, 789, 791, 3, 92, 46, 0, 790, 789, 1, 0, 0, 0, 790, 791, 1, 0, 0, 0, 
		791, 796, 1, 0, 0, 0, 792, 794, 3, 84, 42, 0, 793, 795, 5, 125, 0, 0, 
		794, 793, 1, 0, 0, 0, 794, 795, 1, 0, 0, 0, 795, 797, 1, 0, 0, 0, 796, 
		792, 1, 0, 0, 0, 796, 797, 1, 0, 0, 0, 797, 798, 1, 0, 0, 0, 798, 799, 
		5, 9, 0, 0, 799, 83, 1, 0, 0, 0, 800, 803, 3, 174, 87, 0, 801, 803, 3, 
		176, 88, 0, 802, 800, 1, 0, 0, 0, 802, 801, 1, 0, 0, 0, 803, 85, 1, 0, 
		0, 0, 804, 806, 5, 10, 0, 0, 805, 807, 5, 125, 0, 0, 806, 805, 1, 0, 0, 
		0, 806, 807, 1, 0, 0, 0, 807, 808, 1, 0, 0, 0, 808, 822, 3, 96, 48, 0, 
		809, 811, 5, 125, 0, 0, 810, 809, 1, 0, 0, 0, 810, 811, 1, 0, 0, 0, 811, 
		812, 1, 0, 0, 0, 812, 814, 5, 11, 0, 0, 813, 815, 5, 10, 0, 0, 814, 813, 
		1, 0, 0, 0, 814, 815, 1, 0, 0, 0, 815, 817, 1, 0, 0, 0, 816, 818, 5, 125, 
		0, 0, 817, 816, 1, 0, 0, 0, 817, 818, 1, 0, 0, 0, 818, 819, 1, 0, 0, 0, 
		819, 821, 3, 96, 48, 0, 820, 810, 1, 0, 0, 0, 821, 824, 1, 0, 0, 0, 822, 
		820, 1, 0, 0, 0, 822, 823, 1, 0, 0, 0, 823, 87, 1, 0, 0, 0, 824, 822, 
		1, 0, 0, 0, 825, 832, 3, 90, 45, 0, 826, 828, 5, 125, 0, 0, 827, 826, 
		1, 0, 0, 0, 827, 828, 1, 0, 0, 0, 828, 829, 1, 0, 0, 0, 829, 831, 3, 90, 
		45, 0, 830, 827, 1, 0, 0, 0, 831, 834, 1, 0, 0, 0, 832, 830, 1, 0, 0, 
		0, 832, 833, 1, 0, 0, 0, 833, 89, 1, 0, 0, 0, 834, 832, 1, 0, 0, 0, 835, 
		837, 5, 10, 0, 0, 836, 838, 5, 125, 0, 0, 837, 836, 1, 0, 0, 0, 837, 838, 
		1, 0, 0, 0, 838, 839, 1, 0, 0, 0, 839, 840, 3, 94, 47, 0, 840, 91, 1, 
		0, 0, 0, 841, 843, 5, 5, 0, 0, 842, 844, 5, 125, 0, 0, 843, 842, 1, 0, 
		0, 0, 843, 844, 1, 0, 0, 0, 844, 849, 1, 0, 0, 0, 845, 847, 3, 182, 91, 
		0, 846, 848, 5, 125, 0, 0, 847, 846, 1, 0, 0, 0, 847, 848, 1, 0, 0, 0, 
		848, 850, 1, 0, 0, 0, 849, 845, 1, 0, 0, 0, 849, 850, 1, 0, 0, 0, 850, 
		861, 1, 0, 0, 0, 851, 853, 5, 12, 0, 0, 852, 854, 5, 125, 0, 0, 853, 852, 
		1, 0, 0, 0, 853, 854, 1, 0, 0, 0, 854, 859, 1, 0, 0, 0, 855, 857, 3, 182, 
		91, 0, 856, 858, 5, 125, 0, 0, 857, 856, 1, 0, 0, 0, 857, 858, 1, 0, 0, 
		0, 858, 860, 1, 0, 0, 0, 859, 855, 1, 0, 0, 0, 859, 860, 1, 0, 0, 0, 860, 
		862, 1, 0, 0, 0, 861, 851, 1, 0, 0, 0, 861, 862, 1, 0, 0, 0, 862, 93, 
		1, 0, 0, 0, 863, 864, 3, 186, 93, 0, 864, 95, 1, 0, 0, 0, 865, 866, 3, 
		186, 93, 0, 866, 97, 1, 0, 0, 0, 867, 868, 3, 100, 50, 0, 868, 99, 1, 
		0, 0, 0, 869, 876, 3, 102, 51, 0, 870, 871, 5, 125, 0, 0, 871, 872, 5, 
		73, 0, 0, 872, 873, 5, 125, 0, 0, 873, 875, 3, 102, 51, 0, 874, 870, 1, 
		0, 0, 0, 875, 878, 1, 0, 0, 0, 876, 874, 1, 0, 0, 0, 876, 877, 1, 0, 0, 
		0, 877, 101, 1, 0, 0, 0, 878, 876, 1, 0, 0, 0, 879, 886, 3, 104, 52, 0, 
		880, 881, 5, 125, 0, 0, 881, 882, 5, 74, 0, 0, 882, 883, 5, 125, 0, 0, 
		883, 885, 3, 104, 52, 0, 884, 880, 1, 0, 0, 0, 885, 888, 1, 0, 0, 0, 886, 
		884, 1, 0, 0, 0, 886, 887, 1, 0, 0, 0, 887, 103, 1, 0, 0, 0, 888, 886, 
		1, 0, 0, 0, 889, 896, 3, 106, 53, 0, 890, 891, 5, 125, 0, 0, 891, 892, 
		5, 75, 0, 0, 892, 893, 5, 125, 0, 0, 893, 895, 3, 106, 53, 0, 894, 890, 
		1, 0, 0, 0, 895, 898, 1, 0, 0, 0, 896, 894, 1, 0, 0, 0, 896, 897, 1, 0, 
		0, 0, 897, 105, 1, 0, 0, 0, 898, 896, 1, 0, 0, 0, 899, 901, 5, 76, 0, 
		0, 900, 902, 5, 125, 0, 0, 901, 900, 1, 0, 0, 0, 901, 902, 1, 0, 0, 0, 
		902, 904, 1, 0, 0, 0, 903, 899, 1, 0, 0, 0, 904, 907, 1, 0, 0, 0, 905, 
		903, 1, 0, 0, 0, 905, 906, 1, 0, 0, 0, 906, 908, 1, 0, 0, 0, 907, 905, 
		1, 0, 0, 0, 908, 909, 3, 108, 54, 0, 909, 107, 1, 0, 0, 0, 910, 917, 3, 
		110, 55, 0, 911, 913, 5, 125, 0, 0, 912, 911, 1, 0, 0, 0, 912, 913, 1, 
		0, 0, 0, 913, 914, 1, 0, 0, 0, 914, 916, 3, 136, 68, 0, 915, 912, 1, 0, 
		0, 0, 916, 919, 1, 0, 0, 0, 917, 915, 1, 0, 0, 0, 917, 918, 1, 0, 0, 0, 
		918, 109, 1, 0, 0, 0, 919, 917, 1, 0, 0, 0, 920, 939, 3, 112, 56, 0, 921, 
		923, 5, 125, 0, 0, 922, 921, 1, 0, 0, 0, 922, 923, 1, 0, 0, 0, 923, 924, 
		1, 0, 0, 0, 924, 926, 5, 13, 0, 0, 925, 927, 5, 125, 0, 0, 926, 925, 1, 
		0, 0, 0, 926, 927, 1, 0, 0, 0, 927, 928, 1, 0, 0, 0, 928, 938, 3, 112, 
		56, 0, 929, 931, 5, 125, 0, 0, 930, 929, 1, 0, 0, 0, 930, 931, 1, 0, 0, 
		0, 931, 932, 1, 0, 0, 0, 932, 934, 5, 14, 0, 0, 933, 935, 5, 125, 0, 0, 
		934, 933, 1, 0, 0, 0, 934, 935, 1, 0, 0, 0, 935, 936, 1, 0, 0, 0, 936, 
		938, 3, 112, 56, 0, 937, 922, 1, 0, 0, 0, 937, 930, 1, 0, 0, 0, 938, 941, 
		1, 0, 0, 0, 939, 937, 1, 0, 0, 0, 939, 940, 1, 0, 0, 0, 940, 111, 1, 0, 
		0, 0, 941, 939, 1, 0, 0, 0, 942, 969, 3, 114, 57, 0, 943, 945, 5, 125, 
		0, 0, 944, 943, 1, 0, 0, 0, 944, 945, 1, 0, 0, 0, 945, 946, 1, 0, 0, 0, 
		946, 948, 5, 5, 0, 0, 947, 949, 5, 125, 0, 0, 948, 947, 1, 0, 0, 0, 948, 
		949, 1, 0, 0, 0, 949, 950, 1, 0, 0, 0, 950, 968, 3, 114, 57, 0, 951, 953, 
		5, 125, 0, 0, 952, 951, 1, 0, 0, 0, 952, 953, 1, 0, 0, 0, 953, 954, 1, 
		0, 0, 0, 954, 956, 5, 15, 0, 0, 955, 957, 5, 125, 0, 0, 956, 955, 1, 0, 
		0, 0, 956, 957, 1, 0, 0, 0, 957, 958, 1, 0, 0, 0, 958, 968, 3, 114, 57, 
		0, 959, 961, 5, 125, 0, 0, 960, 959, 1, 0, 0, 0, 960, 961, 1, 0, 0, 0, 
		961, 962, 1, 0, 0, 0, 962, 964, 5, 16, 0, 0, 963, 965, 5, 125, 0, 0, 964, 
		963, 1, 0, 0, 0, 964, 965, 1, 0, 0, 0, 965, 966, 1, 0, 0, 0, 966, 968, 
		3, 114, 57, 0, 967, 944, 1, 0, 0, 0, 967, 952, 1, 0, 0, 0, 967, 960, 1, 
		0, 0, 0, 968, 971, 1, 0, 0, 0, 969, 967, 1, 0, 0, 0, 969, 970, 1, 0, 0, 
		0, 970, 113, 1, 0, 0, 0, 971, 969, 1, 0, 0, 0, 972, 983, 3, 116, 58, 0, 
		973, 975, 5, 125, 0, 0, 974, 973, 1, 0, 0, 0, 974, 975, 1, 0, 0, 0, 975, 
		976, 1, 0, 0, 0, 976, 978, 5, 17, 0, 0, 977, 979, 5, 125, 0, 0, 978, 977, 
		1, 0, 0, 0, 978, 979, 1, 0, 0, 0, 979, 980, 1, 0, 0, 0, 980, 982, 3, 116, 
		58, 0, 981, 974, 1, 0, 0, 0, 982, 985, 1, 0, 0, 0, 983, 981, 1, 0, 0, 
		0, 983, 984, 1, 0, 0, 0, 984, 115, 1, 0, 0, 0, 985, 983, 1, 0, 0, 0, 986, 
		988, 7, 1, 0, 0, 987, 989, 5, 125, 0, 0, 988, 987, 1, 0, 0, 0, 988, 989, 
		1, 0, 0, 0, 989, 991, 1, 0, 0, 0, 990, 986, 1, 0, 0, 0, 991, 994, 1, 0, 
		0, 0, 992, 990, 1, 0, 0, 0, 992, 993, 1, 0, 0, 0, 993, 995, 1, 0, 0, 0, 
		994, 992, 1, 0, 0, 0, 995, 996, 3, 118, 59, 0, 996, 117, 1, 0, 0, 0, 997, 
		1003, 3, 126, 63, 0, 998, 1002, 3, 122, 61, 0, 999, 1002, 3, 120, 60, 
		0, 1000, 1002, 3, 124, 62, 0, 1001, 998, 1, 0, 0, 0, 1001, 999, 1, 0, 
		0, 0, 1001, 1000, 1, 0, 0, 0, 1002, 1005, 1, 0, 0, 0, 1003, 1001, 1, 0, 
		0, 0, 1003, 1004, 1, 0, 0, 0, 1004, 119, 1, 0, 0, 0, 1005, 1003, 1, 0, 
		0, 0, 1006, 1007, 5, 125, 0, 0, 1007, 1009, 5, 77, 0, 0, 1008, 1010, 5, 
		125, 0, 0, 1009, 1008, 1, 0, 0, 0, 1009, 1010, 1, 0, 0, 0, 1010, 1011, 
		1, 0, 0, 0, 1011, 1032, 3, 126, 63, 0, 1012, 1014, 5, 125, 0, 0, 1013, 
		1012, 1, 0, 0, 0, 1013, 1014, 1, 0, 0, 0, 1014, 1015, 1, 0, 0, 0, 1015, 
		1016, 5, 8, 0, 0, 1016, 1017, 3, 98, 49, 0, 1017, 1018, 5, 9, 0, 0, 1018, 
		1032, 1, 0, 0, 0, 1019, 1021, 5, 125, 0, 0, 1020, 1019, 1, 0, 0, 0, 1020, 
		1021, 1, 0, 0, 0, 1021, 1022, 1, 0, 0, 0, 1022, 1024, 5, 8, 0, 0, 1023, 
		1025, 3, 98, 49, 0, 1024, 1023, 1, 0, 0, 0, 1024, 1025, 1, 0, 0, 0, 1025, 
		1026, 1, 0, 0, 0, 1026, 1028, 5, 12, 0, 0, 1027, 1029, 3, 98, 49, 0, 1028, 
		1027, 1, 0, 0, 0, 1028, 1029, 1, 0, 0, 0, 1029, 1030, 1, 0, 0, 0, 1030, 
		1032, 5, 9, 0, 0, 1031, 1006, 1, 0, 0, 0, 1031, 1013, 1, 0, 0, 0, 1031, 
		1020, 1, 0, 0, 0, 1032, 121, 1, 0, 0, 0, 1033, 1034, 5, 125, 0, 0, 1034, 
		1035, 5, 78, 0, 0, 1035, 1036, 5, 125, 0, 0, 1036, 1044, 5, 61, 0, 0, 
		1037, 1038, 5, 125, 0, 0, 1038, 1039, 5, 79, 0, 0, 1039, 1040, 5, 125, 
		0, 0, 1040, 1044, 5, 61, 0, 0, 1041, 1042, 5, 125, 0, 0, 1042, 1044, 5, 
		80, 0, 0, 1043, 1033, 1, 0, 0, 0, 1043, 1037, 1, 0, 0, 0, 1043, 1041, 
		1, 0, 0, 0, 1044, 1046, 1, 0, 0, 0, 1045, 1047, 5, 125, 0, 0, 1046, 1045, 
		1, 0, 0, 0, 1046, 1047, 1, 0, 0, 0, 1047, 1048, 1, 0, 0, 0, 1048, 1049, 
		3, 126, 63, 0, 1049, 123, 1, 0, 0, 0, 1050, 1051, 5, 125, 0, 0, 1051, 
		1052, 5, 81, 0, 0, 1052, 1053, 5, 125, 0, 0, 1053, 1061, 5, 82, 0, 0, 
		1054, 1055, 5, 125, 0, 0, 1055, 1056, 5, 81, 0, 0, 1056, 1057, 5, 125, 
		0, 0, 1057, 1058, 5, 76, 0, 0, 1058, 1059, 5, 125, 0, 0, 1059, 1061, 5, 
		82, 0, 0, 1060, 1050, 1, 0, 0, 0, 1060, 1054, 1, 0, 0, 0, 1061, 125, 1, 
		0, 0, 0, 1062, 1069, 3, 128, 64, 0, 1063, 1065, 5, 125, 0, 0, 1064, 1063, 
		1, 0, 0, 0, 1064, 1065, 1, 0, 0, 0, 1065, 1066, 1, 0, 0, 0, 1066, 1068, 
		3, 164, 82, 0, 1067, 1064, 1, 0, 0, 0, 1068, 1071, 1, 0, 0, 0, 1069, 1067, 
		1, 0, 0, 0, 1069, 1070, 1, 0, 0, 0, 1070, 1076, 1, 0, 0, 0, 1071, 1069, 
		1, 0, 0, 0, 1072, 1074, 5, 125, 0, 0, 1073, 1072, 1, 0, 0, 0, 1073, 1074, 
		1, 0, 0, 0, 1074, 1075, 1, 0, 0, 0, 1075, 1077, 3, 88, 44, 0, 1076, 1073, 
		1, 0, 0, 0, 1076, 1077, 1, 0, 0, 0, 1077, 127, 1, 0, 0, 0, 1078, 1157, 
		3, 130, 65, 0, 1079, 1157, 3, 176, 88, 0, 1080, 1157, 3, 166, 83, 0, 1081, 
		1083, 5, 83, 0, 0, 1082, 1084, 5, 125, 0, 0, 1083, 1082, 1, 0, 0, 0, 1083, 
		1084, 1, 0, 0, 0, 1084, 1085, 1, 0, 0, 0, 1085, 1087, 5, 6, 0, 0, 1086, 
		1088, 5, 125, 0, 0, 1087, 1086, 1, 0, 0, 0, 1087, 1088, 1, 0, 0, 0, 1088, 
		1089, 1, 0, 0, 0, 1089, 1091, 5, 5, 0, 0, 1090, 1092, 5, 125, 0, 0, 1091, 
		1090, 1, 0, 0, 0, 1091, 1092, 1, 0, 0, 0, 1092, 1093, 1, 0, 0, 0, 1093, 
		1157, 5, 7, 0, 0, 1094, 1157, 3, 160, 80, 0, 1095, 1157, 3, 162, 81, 0, 
		1096, 1098, 5, 47, 0, 0, 1097, 1099, 5, 125, 0, 0, 1098, 1097, 1, 0, 0, 
		0, 1098, 1099, 1, 0, 0, 0, 1099, 1100, 1, 0, 0, 0, 1100, 1102, 5, 6, 0, 
		0, 1101, 1103, 5, 125, 0, 0, 1102, 1101, 1, 0, 0, 0, 1102, 1103, 1, 0, 
		0, 0, 1103, 1104, 1, 0, 0, 0, 1104, 1106, 3, 142, 71, 0, 1105, 1107, 5, 
		125, 0, 0, 1106, 1105, 1, 0, 0, 0, 1106, 1107, 1, 0, 0, 0, 1107, 1108, 
		1, 0, 0, 0, 1108, 1109, 5, 7, 0, 0, 1109, 1157, 1, 0, 0, 0, 1110, 1112, 
		5, 84, 0, 0, 1111, 1113, 5, 125, 0, 0, 1112, 1111, 1, 0, 0, 0, 1112, 1113, 
		1, 0, 0, 0, 1113, 1114, 1, 0, 0, 0, 1114, 1116, 5, 6, 0, 0, 1115, 1117, 
		5, 125, 0, 0, 1116, 1115, 1, 0, 0, 0, 1116, 1117, 1, 0, 0, 0, 1117, 1118, 
		1, 0, 0, 0, 1118, 1120, 3, 142, 71, 0, 1119, 1121, 5, 125, 0, 0, 1120, 
		1119, 1, 0, 0, 0, 1120, 1121, 1, 0, 0, 0, 1121, 1122, 1, 0, 0, 0, 1122, 
		1123, 5, 7, 0, 0, 1123, 1157, 1, 0, 0, 0, 1124, 1126, 5, 85, 0, 0, 1125, 
		1127, 5, 125, 0, 0, 1126, 1125, 1, 0, 0, 0, 1126, 1127, 1, 0, 0, 0, 1127, 
		1128, 1, 0, 0, 0, 1128, 1130, 5, 6, 0, 0, 1129, 1131, 5, 125, 0, 0, 1130, 
		1129, 1, 0, 0, 0, 1130, 1131, 1, 0, 0, 0, 1131, 1132, 1, 0, 0, 0, 1132, 
		1134, 3, 142, 71, 0, 1133, 1135, 5, 125, 0, 0, 1134, 1133, 1, 0, 0, 0, 
		1134, 1135, 1, 0, 0, 0, 1135, 1136, 1, 0, 0, 0, 1136, 1137, 5, 7, 0, 0, 
		1137, 1157, 1, 0, 0, 0, 1138, 1140, 5, 86, 0, 0, 1139, 1141, 5, 125, 0, 
		0, 1140, 1139, 1, 0, 0, 0, 1140, 1141, 1, 0, 0, 0, 1141, 1142, 1, 0, 0, 
		0, 1142, 1144, 5, 6, 0, 0, 1143, 1145, 5, 125, 0, 0, 1144, 1143, 1, 0, 
		0, 0, 1144, 1145, 1, 0, 0, 0, 1145, 1146, 1, 0, 0, 0, 1146, 1148, 3, 142, 
		71, 0, 1147, 1149, 5, 125, 0, 0, 1148, 1147, 1, 0, 0, 0, 1148, 1149, 1, 
		0, 0, 0, 1149, 1150, 1, 0, 0, 0, 1150, 1151, 5, 7, 0, 0, 1151, 1157, 1, 
		0, 0, 0, 1152, 1157, 3, 140, 70, 0, 1153, 1157, 3, 138, 69, 0, 1154, 1157, 
		3, 146, 73, 0, 1155, 1157, 3, 170, 85, 0, 1156, 1078, 1, 0, 0, 0, 1156, 
		1079, 1, 0, 0, 0, 1156, 1080, 1, 0, 0, 0, 1156, 1081, 1, 0, 0, 0, 1156, 
		1094, 1, 0, 0, 0, 1156, 1095, 1, 0, 0, 0, 1156, 1096, 1, 0, 0, 0, 1156, 
		1110, 1, 0, 0, 0, 1156, 1124, 1, 0, 0, 0, 1156, 1138, 1, 0, 0, 0, 1156, 
		1152, 1, 0, 0, 0, 1156, 1153, 1, 0, 0, 0, 1156, 1154, 1, 0, 0, 0, 1156, 
		1155, 1, 0, 0, 0, 1157, 129, 1, 0, 0, 0, 1158, 1165, 3, 172, 86, 0, 1159, 
		1165, 5, 95, 0, 0, 1160, 1165, 3, 132, 66, 0, 1161, 1165, 5, 82, 0, 0, 
		1162, 1165, 3, 174, 87, 0, 1163, 1165, 3, 134, 67, 0, 1164, 1158, 1, 0, 
		0, 0, 1164, 1159, 1, 0, 0, 0, 1164, 1160, 1, 0, 0, 0, 1164, 1161, 1, 0, 
		0, 0, 1164, 1162, 1, 0, 0, 0, 1164, 1163, 1, 0, 0, 0, 1165, 131, 1, 0, 
		0, 0, 1166, 1167, 7, 2, 0, 0, 1167, 133, 1, 0, 0, 0, 1168, 1170, 5, 8, 
		0, 0, 1169, 1171, 5, 125, 0, 0, 1170, 1169, 1, 0, 0, 0, 1170, 1171, 1, 
		0, 0, 0, 1171, 1189, 1, 0, 0, 0, 1172, 1174, 3, 98, 49, 0, 1173, 1175, 
		5, 125, 0, 0, 1174, 1173, 1, 0, 0, 0, 1174, 1175, 1, 0, 0, 0, 1175, 1186, 
		1, 0, 0, 0, 1176, 1178, 5, 2, 0, 0, 1177, 1179, 5, 125, 0, 0, 1178, 1177, 
		1, 0, 0, 0, 1178, 1179, 1, 0, 0, 0, 1179, 1180, 1, 0, 0, 0, 1180, 1182, 
		3, 98, 49, 0, 1181, 1183, 5, 125, 0, 0, 1182, 1181, 1, 0, 0, 0, 1182, 
		1183, 1, 0, 0, 0, 1183, 1185, 1, 0, 0, 0, 1184, 1176, 1, 0, 0, 0, 1185, 
		1188, 1, 0, 0, 0, 1186, 1184, 1, 0, 0, 0, 1186, 1187, 1, 0, 0, 0, 1187, 
		1190, 1, 0, 0, 0, 1188, 1186, 1, 0, 0, 0, 1189, 1172, 1, 0, 0, 0, 1189, 
		1190, 1, 0, 0, 0, 1190, 1191, 1, 0, 0, 0, 1191, 1192, 5, 9, 0, 0, 1192, 
		135, 1, 0, 0, 0, 1193, 1195, 5, 3, 0, 0, 1194, 1196, 5, 125, 0, 0, 1195, 
		1194, 1, 0, 0, 0, 1195, 1196, 1, 0, 0, 0, 1196, 1197, 1, 0, 0, 0, 1197, 
		1224, 3, 110, 55, 0, 1198, 1200, 5, 18, 0, 0, 1199, 1201, 5, 125, 0, 0, 
		1200, 1199, 1, 0, 0, 0, 1200, 1201, 1, 0, 0, 0, 1201, 1202, 1, 0, 0, 0, 
		1202, 1224, 3, 110, 55, 0, 1203, 1205, 5, 19, 0, 0, 1204, 1206, 5, 125, 
		0, 0, 1205, 1204, 1, 0, 0, 0, 1205, 1206, 1, 0, 0, 0, 1206, 1207, 1, 0, 
		0, 0, 1207, 1224, 3, 110, 55, 0, 1208, 1210, 5, 20, 0, 0, 1209, 1211, 
		5, 125, 0, 0, 1210, 1209, 1, 0, 0, 0, 1210, 1211, 1, 0, 0, 0, 1211, 1212, 
		1, 0, 0, 0, 1212, 1224, 3, 110, 55, 0, 1213, 1215, 5, 21, 0, 0, 1214, 
		1216, 5, 125, 0, 0, 1215, 1214, 1, 0, 0, 0, 1215, 1216, 1, 0, 0, 0, 1216, 
		1217, 1, 0, 0, 0, 1217, 1224, 3, 110, 55, 0, 1218, 1220, 5, 22, 0, 0, 
		1219, 1221, 5, 125, 0, 0, 1220, 1219, 1, 0, 0, 0, 1220, 1221, 1, 0, 0, 
		0, 1221, 1222, 1, 0, 0, 0, 1222, 1224, 3, 110, 55, 0, 1223, 1193, 1, 0, 
		0, 0, 1223, 1198, 1, 0, 0, 0, 1223, 1203, 1, 0, 0, 0, 1223, 1208, 1, 0, 
		0, 0, 1223, 1213, 1, 0, 0, 0, 1223, 1218, 1, 0, 0, 0, 1224, 137, 1, 0, 
		0, 0, 1225, 1227, 5, 6, 0, 0, 1226, 1228, 5, 125, 0, 0, 1227, 1226, 1, 
		0, 0, 0, 1227, 1228, 1, 0, 0, 0, 1228, 1229, 1, 0, 0, 0, 1229, 1231, 3, 
		98, 49, 0, 1230, 1232, 5, 125, 0, 0, 1231, 1230, 1, 0, 0, 0, 1231, 1232, 
		1, 0, 0, 0, 1232, 1233, 1, 0, 0, 0, 1233, 1234, 5, 7, 0, 0, 1234, 139, 
		1, 0, 0, 0, 1235, 1240, 3, 76, 38, 0, 1236, 1238, 5, 125, 0, 0, 1237, 
		1236, 1, 0, 0, 0, 1237, 1238, 1, 0, 0, 0, 1238, 1239, 1, 0, 0, 0, 1239, 
		1241, 3, 78, 39, 0, 1240, 1237, 1, 0, 0, 0, 1241, 1242, 1, 0, 0, 0, 1242, 
		1240, 1, 0, 0, 0, 1242, 1243, 1, 0, 0, 0, 1243, 141, 1, 0, 0, 0, 1244, 
		1249, 3, 144, 72, 0, 1245, 1247, 5, 125, 0, 0, 1246, 1245, 1, 0, 0, 0, 
		1246, 1247, 1, 0, 0, 0, 1247, 1248, 1, 0, 0, 0, 1248, 1250, 3, 66, 33, 
		0, 1249, 1246, 1, 0, 0, 0, 1249, 1250, 1, 0, 0, 0, 1250, 143, 1, 0, 0, 
		0, 1251, 1252, 3, 170, 85, 0, 1252, 1253, 5, 125, 0, 0, 1253, 1254, 5, 
		77, 0, 0, 1254, 1255, 5, 125, 0, 0, 1255, 1256, 3, 98, 49, 0, 1256, 145, 
		1, 0, 0, 0, 1257, 1259, 3, 148, 74, 0, 1258, 1260, 5, 125, 0, 0, 1259, 
		1258, 1, 0, 0, 0, 1259, 1260, 1, 0, 0, 0, 1260, 1261, 1, 0, 0, 0, 1261, 
		1263, 5, 6, 0, 0, 1262, 1264, 5, 125, 0, 0, 1263, 1262, 1, 0, 0, 0, 1263, 
		1264, 1, 0, 0, 0, 1264, 1269, 1, 0, 0, 0, 1265, 1267, 5, 63, 0, 0, 1266, 
		1268, 5, 125, 0, 0, 1267, 1266, 1, 0, 0, 0, 1267, 1268, 1, 0, 0, 0, 1268, 
		1270, 1, 0, 0, 0, 1269, 1265, 1, 0, 0, 0, 1269, 1270, 1, 0, 0, 0, 1270, 
		1288, 1, 0, 0, 0, 1271, 1273, 3, 98, 49, 0, 1272, 1274, 5, 125, 0, 0, 
		1273, 1272, 1, 0, 0, 0, 1273, 1274, 1, 0, 0, 0, 1274, 1285, 1, 0, 0, 0, 
		1275, 1277, 5, 2, 0, 0, 1276, 1278, 5, 125, 0, 0, 1277, 1276, 1, 0, 0, 
		0, 1277, 1278, 1, 0, 0, 0, 1278, 1279, 1, 0, 0, 0, 1279, 1281, 3, 98, 
		49, 0, 1280, 1282, 5, 125, 0, 0, 1281, 1280, 1, 0, 0, 0, 1281, 1282, 1, 
		0, 0, 0, 1282, 1284, 1, 0, 0, 0, 1283, 1275, 1, 0, 0, 0, 1284, 1287, 1, 
		0, 0, 0, 1285, 1283, 1, 0, 0, 0, 1285, 1286, 1, 0, 0, 0, 1286, 1289, 1, 
		0, 0, 0, 1287, 1285, 1, 0, 0, 0, 1288, 1271, 1, 0, 0, 0, 1288, 1289, 1, 
		0, 0, 0, 1289, 1290, 1, 0, 0, 0, 1290, 1291, 5, 7, 0, 0, 1291, 147, 1, 
		0, 0, 0, 1292, 1293, 3, 158, 79, 0, 1293, 1294, 3, 190, 95, 0, 1294, 1297, 
		1, 0, 0, 0, 1295, 1297, 5, 89, 0, 0, 1296, 1292, 1, 0, 0, 0, 1296, 1295, 
		1, 0, 0, 0, 1297, 149, 1, 0, 0, 0, 1298, 1300, 3, 156, 78, 0, 1299, 1301, 
		5, 125, 0, 0, 1300, 1299, 1, 0, 0, 0, 1300, 1301, 1, 0, 0, 0, 1301, 1302, 
		1, 0, 0, 0, 1302, 1304, 5, 6, 0, 0, 1303, 1305, 5, 125, 0, 0, 1304, 1303, 
		1, 0, 0, 0, 1304, 1305, 1, 0, 0, 0, 1305, 1323, 1, 0, 0, 0, 1306, 1308, 
		3, 98, 49, 0, 1307, 1309, 5, 125, 0, 0, 1308, 1307, 1, 0, 0, 0, 1308, 
		1309, 1, 0, 0, 0, 1309, 1320, 1, 0, 0, 0, 1310, 1312, 5, 2, 0, 0, 1311, 
		1313, 5, 125, 0, 0, 1312, 1311, 1, 0, 0, 0, 1312, 1313, 1, 0, 0, 0, 1313, 
		1314, 1, 0, 0, 0, 1314, 1316, 3, 98, 49, 0, 1315, 1317, 5, 125, 0, 0, 
		1316, 1315, 1, 0, 0, 0, 1316, 1317, 1, 0, 0, 0, 1317, 1319, 1, 0, 0, 0, 
		1318, 1310, 1, 0, 0, 0, 1319, 1322, 1, 0, 0, 0, 1320, 1318, 1, 0, 0, 0, 
		1320, 1321, 1, 0, 0, 0, 1321, 1324, 1, 0, 0, 0, 1322, 1320, 1, 0, 0, 0, 
		1323, 1306, 1, 0, 0, 0, 1323, 1324, 1, 0, 0, 0, 1324, 1325, 1, 0, 0, 0, 
		1325, 1326, 5, 7, 0, 0, 1326, 151, 1, 0, 0, 0, 1327, 1328, 3, 156, 78, 
		0, 1328, 153, 1, 0, 0, 0, 1329, 1330, 3, 190, 95, 0, 1330, 155, 1, 0, 
		0, 0, 1331, 1332, 3, 158, 79, 0, 1332, 1333, 3, 190, 95, 0, 1333, 157, 
		1, 0, 0, 0, 1334, 1335, 3, 190, 95, 0, 1335, 1336, 5, 23, 0, 0, 1336, 
		1338, 1, 0, 0, 0, 1337, 1334, 1, 0, 0, 0, 1338, 1341, 1, 0, 0, 0, 1339, 
		1337, 1, 0, 0, 0, 1339, 1340, 1, 0, 0, 0, 1340, 159, 1, 0, 0, 0, 1341, 
		1339, 1, 0, 0, 0, 1342, 1344, 5, 8, 0, 0, 1343, 1345, 5, 125, 0, 0, 1344, 
		1343, 1, 0, 0, 0, 1344, 1345, 1, 0, 0, 0, 1345, 1346, 1, 0, 0, 0, 1346, 
		1355, 3, 142, 71, 0, 1347, 1349, 5, 125, 0, 0, 1348, 1347, 1, 0, 0, 0, 
		1348, 1349, 1, 0, 0, 0, 1349, 1350, 1, 0, 0, 0, 1350, 1352, 5, 11, 0, 
		0, 1351, 1353, 5, 125, 0, 0, 1352, 1351, 1, 0, 0, 0, 1352, 1353, 1, 0, 
		0, 0, 1353, 1354, 1, 0, 0, 0, 1354, 1356, 3, 98, 49, 0, 1355, 1348, 1, 
		0, 0, 0, 1355, 1356, 1, 0, 0, 0, 1356, 1358, 1, 0, 0, 0, 1357, 1359, 5, 
		125, 0, 0, 1358, 1357, 1, 0, 0, 0, 1358, 1359, 1, 0, 0, 0, 1359, 1360, 
		1, 0, 0, 0, 1360, 1361, 5, 9, 0, 0, 1361, 161, 1, 0, 0, 0, 1362, 1364, 
		5, 8, 0, 0, 1363, 1365, 5, 125, 0, 0, 1364, 1363, 1, 0, 0, 0, 1364, 1365, 
		1, 0, 0, 0, 1365, 1374, 1, 0, 0, 0, 1366, 1368, 3, 170, 85, 0, 1367, 1369, 
		5, 125, 0, 0, 1368, 1367, 1, 0, 0, 0, 1368, 1369, 1, 0, 0, 0, 1369, 1370, 
		1, 0, 0, 0, 1370, 1372, 5, 3, 0, 0, 1371, 1373, 5, 125, 0, 0, 1372, 1371, 
		1, 0, 0, 0, 1372, 1373, 1, 0, 0, 0, 1373, 1375, 1, 0, 0, 0, 1374, 1366, 
		1, 0, 0, 0, 1374, 1375, 1, 0, 0, 0, 1375, 1376, 1, 0, 0, 0, 1376, 1378, 
		3, 140, 70, 0, 1377, 1379, 5, 125, 0, 0, 1378, 1377, 1, 0, 0, 0, 1378, 
		1379, 1, 0, 0, 0, 1379, 1388, 1, 0, 0, 0, 1380, 1382, 5, 72, 0, 0, 1381, 
		1383, 5, 125, 0, 0, 1382, 1381, 1, 0, 0, 0, 1382, 1383, 1, 0, 0, 0, 1383, 
		1384, 1, 0, 0, 0, 1384, 1386, 3, 98, 49, 0, 1385, 1387, 5, 125, 0, 0, 
		1386, 1385, 1, 0, 0, 0, 1386, 1387, 1, 0, 0, 0, 1387, 1389, 1, 0, 0, 0, 
		1388, 1380, 1, 0, 0, 0, 1388, 1389, 1, 0, 0, 0, 1389, 1390, 1, 0, 0, 0, 
		1390, 1392, 5, 11, 0, 0, 1391, 1393, 5, 125, 0, 0, 1392, 1391, 1, 0, 0, 
		0, 1392, 1393, 1, 0, 0, 0, 1393, 1394, 1, 0, 0, 0, 1394, 1396, 3, 98, 
		49, 0, 1395, 1397, 5, 125, 0, 0, 1396, 1395, 1, 0, 0, 0, 1396, 1397, 1, 
		0, 0, 0, 1397, 1398, 1, 0, 0, 0, 1398, 1399, 5, 9, 0, 0, 1399, 163, 1, 
		0, 0, 0, 1400, 1402, 5, 23, 0, 0, 1401, 1403, 5, 125, 0, 0, 1402, 1401, 
		1, 0, 0, 0, 1402, 1403, 1, 0, 0, 0, 1403, 1404, 1, 0, 0, 0, 1404, 1405, 
		3, 180, 90, 0, 1405, 165, 1, 0, 0, 0, 1406, 1411, 5, 90, 0, 0, 1407, 1409, 
		5, 125, 0, 0, 1408, 1407, 1, 0, 0, 0, 1408, 1409, 1, 0, 0, 0, 1409, 1410, 
		1, 0, 0, 0, 1410, 1412, 3, 168, 84, 0, 1411, 1408, 1, 0, 0, 0, 1412, 1413, 
		1, 0, 0, 0, 1413, 1411, 1, 0, 0, 0, 1413, 1414, 1, 0, 0, 0, 1414, 1429, 
		1, 0, 0, 0, 1415, 1417, 5, 90, 0, 0, 1416, 1418, 5, 125, 0, 0, 1417, 1416, 
		1, 0, 0, 0, 1417, 1418, 1, 0, 0, 0, 1418, 1419, 1, 0, 0, 0, 1419, 1424, 
		3, 98, 49, 0, 1420, 1422, 5, 125, 0, 0, 1421, 1420, 1, 0, 0, 0, 1421, 
		1422, 1, 0, 0, 0, 1422, 1423, 1, 0, 0, 0, 1423, 1425, 3, 168, 84, 0, 1424, 
		1421, 1, 0, 0, 0, 1425, 1426, 1, 0, 0, 0, 1426, 1424, 1, 0, 0, 0, 1426, 
		1427, 1, 0, 0, 0, 1427, 1429, 1, 0, 0, 0, 1428, 1406, 1, 0, 0, 0, 1428, 
		1415, 1, 0, 0, 0, 1429, 1438, 1, 0, 0, 0, 1430, 1432, 5, 125, 0, 0, 1431, 
		1430, 1, 0, 0, 0, 1431, 1432, 1, 0, 0, 0, 1432, 1433, 1, 0, 0, 0, 1433, 
		1435, 5, 91, 0, 0, 1434, 1436, 5, 125, 0, 0, 1435, 1434, 1, 0, 0, 0, 1435, 
		1436, 1, 0, 0, 0, 1436, 1437, 1, 0, 0, 0, 1437, 1439, 3, 98, 49, 0, 1438, 
		1431, 1, 0, 0, 0, 1438, 1439, 1, 0, 0, 0, 1439, 1441, 1, 0, 0, 0, 1440, 
		1442, 5, 125, 0, 0, 1441, 1440, 1, 0, 0, 0, 1441, 1442, 1, 0, 0, 0, 1442, 
		1443, 1, 0, 0, 0, 1443, 1444, 5, 92, 0, 0, 1444, 167, 1, 0, 0, 0, 1445, 
		1447, 5, 93, 0, 0, 1446, 1448, 5, 125, 0, 0, 1447, 1446, 1, 0, 0, 0, 1447, 
		1448, 1, 0, 0, 0, 1448, 1449, 1, 0, 0, 0, 1449, 1451, 3, 98, 49, 0, 1450, 
		1452, 5, 125, 0, 0, 1451, 1450, 1, 0, 0, 0, 1451, 1452, 1, 0, 0, 0, 1452, 
		1453, 1, 0, 0, 0, 1453, 1455, 5, 94, 0, 0, 1454, 1456, 5, 125, 0, 0, 1455, 
		1454, 1, 0, 0, 0, 1455, 1456, 1, 0, 0, 0, 1456, 1457, 1, 0, 0, 0, 1457, 
		1458, 3, 98, 49, 0, 1458, 169, 1, 0, 0, 0, 1459, 1460, 3, 190, 95, 0, 
		1460, 171, 1, 0, 0, 0, 1461, 1464, 3, 184, 92, 0, 1462, 1464, 3, 182, 
		91, 0, 1463, 1461, 1, 0, 0, 0, 1463, 1462, 1, 0, 0, 0, 1464, 173, 1, 0, 
		0, 0, 1465, 1467, 5, 24, 0, 0, 1466, 1468, 5, 125, 0, 0, 1467, 1466, 1, 
		0, 0, 0, 1467, 1468, 1, 0, 0, 0, 1468, 1502, 1, 0, 0, 0, 1469, 1471, 3, 
		180, 90, 0, 1470, 1472, 5, 125, 0, 0, 1471, 1470, 1, 0, 0, 0, 1471, 1472, 
		1, 0, 0, 0, 1472, 1473, 1, 0, 0, 0, 1473, 1475, 5, 10, 0, 0, 1474, 1476, 
		5, 125, 0, 0, 1475, 1474, 1, 0, 0, 0, 1475, 1476, 1, 0, 0, 0, 1476, 1477, 
		1, 0, 0, 0, 1477, 1479, 3, 98, 49, 0, 1478, 1480, 5, 125, 0, 0, 1479, 
		1478, 1, 0, 0, 0, 1479, 1480, 1, 0, 0, 0, 1480, 1499, 1, 0, 0, 0, 1481, 
		1483, 5, 2, 0, 0, 1482, 1484, 5, 125, 0, 0, 1483, 1482, 1, 0, 0, 0, 1483, 
		1484, 1, 0, 0, 0, 1484, 1485, 1, 0, 0, 0, 1485, 1487, 3, 180, 90, 0, 1486, 
		1488, 5, 125, 0, 0, 1487, 1486, 1, 0, 0, 0, 1487, 1488, 1, 0, 0, 0, 1488, 
		1489, 1, 0, 0, 0, 1489, 1491, 5, 10, 0, 0, 1490, 1492, 5, 125, 0, 0, 1491, 
		1490, 1, 0, 0, 0, 1491, 1492, 1, 0, 0, 0, 1492, 1493, 1, 0, 0, 0, 1493, 
		1495, 3, 98, 49, 0, 1494, 1496, 5, 125, 0, 0, 1495, 1494, 1, 0, 0, 0, 
		1495, 1496, 1, 0, 0, 0, 1496, 1498, 1, 0, 0, 0, 1497, 1481, 1, 0, 0, 0, 
		1498, 1501, 1, 0, 0, 0, 1499, 1497, 1, 0, 0, 0, 1499, 1500, 1, 0, 0, 0, 
		1500, 1503, 1, 0, 0, 0, 1501, 1499, 1, 0, 0, 0, 1502, 1469, 1, 0, 0, 0, 
		1502, 1503, 1, 0, 0, 0, 1503, 1504, 1, 0, 0, 0, 1504, 1505, 5, 25, 0, 
		0, 1505, 175, 1, 0, 0, 0, 1506, 1509, 5, 26, 0, 0, 1507, 1510, 3, 190, 
		95, 0, 1508, 1510, 5, 98, 0, 0, 1509, 1507, 1, 0, 0, 0, 1509, 1508, 1, 
		0, 0, 0, 1510, 177, 1, 0, 0, 0, 1511, 1516, 3, 128, 64, 0, 1512, 1514, 
		5, 125, 0, 0, 1513, 1512, 1, 0, 0, 0, 1513, 1514, 1, 0, 0, 0, 1514, 1515, 
		1, 0, 0, 0, 1515, 1517, 3, 164, 82, 0, 1516, 1513, 1, 0, 0, 0, 1517, 1518, 
		1, 0, 0, 0, 1518, 1516, 1, 0, 0, 0, 1518, 1519, 1, 0, 0, 0, 1519, 179, 
		1, 0, 0, 0, 1520, 1521, 3, 186, 93, 0, 1521, 181, 1, 0, 0, 0, 1522, 1523, 
		7, 3, 0, 0, 1523, 183, 1, 0, 0, 0, 1524, 1525, 7, 4, 0, 0, 1525, 185, 
		1, 0, 0, 0, 1526, 1529, 3, 190, 95, 0, 1527, 1529, 3, 188, 94, 0, 1528, 
		1526, 1, 0, 0, 0, 1528, 1527, 1, 0, 0, 0, 1529, 187, 1, 0, 0, 0, 1530, 
		1531, 7, 5, 0, 0, 1531, 189, 1, 0, 0, 0, 1532, 1533, 7, 6, 0, 0, 1533, 
		191, 1, 0, 0, 0, 1534, 1535, 7, 7, 0, 0, 1535, 193, 1, 0, 0, 0, 1536, 
		1537, 7, 8, 0, 0, 1537, 195, 1, 0, 0, 0, 1538, 1539, 7, 9, 0, 0, 1539, 
		197, 1, 0, 0, 0, 283, 199, 203, 206, 209, 217, 221, 226, 233, 238, 241, 
		245, 249, 253, 259, 263, 268, 273, 277, 280, 282, 286, 290, 295, 299, 
		304, 308, 317, 322, 326, 330, 334, 337, 341, 351, 358, 371, 375, 381, 
		388, 393, 397, 403, 407, 413, 417, 423, 427, 431, 435, 439, 443, 448, 
		455, 459, 464, 471, 477, 482, 488, 494, 499, 503, 508, 511, 514, 517, 
		524, 531, 534, 540, 543, 549, 553, 557, 561, 565, 570, 575, 579, 584, 
		587, 596, 605, 610, 623, 626, 634, 638, 643, 648, 652, 657, 663, 668, 
		675, 679, 683, 685, 689, 691, 695, 697, 703, 709, 713, 716, 719, 723, 
		729, 733, 736, 739, 745, 748, 751, 755, 761, 764, 767, 771, 775, 779, 
		781, 785, 787, 790, 794, 796, 802, 806, 810, 814, 817, 822, 827, 832, 
		837, 843, 847, 849, 853, 857, 859, 861, 876, 886, 896, 901, 905, 912, 
		917, 922, 926, 930, 934, 937, 939, 944, 948, 952, 956, 960, 964, 967, 
		969, 974, 978, 983, 988, 992, 1001, 1003, 1009, 1013, 1020, 1024, 1028, 
		1031, 1043, 1046, 1060, 1064, 1069, 1073, 1076, 1083, 1087, 1091, 1098, 
		1102, 1106, 1112, 1116, 1120, 1126, 1130, 1134, 1140, 1144, 1148, 1156, 
		1164, 1170, 1174, 1178, 1182, 1186, 1189, 1195, 1200, 1205, 1210, 1215, 
		1220, 1223, 1227, 1231, 1237, 1242, 1246, 1249, 1259, 1263, 1267, 1269, 
		1273, 1277, 1281, 1285, 1288, 1296, 1300, 1304, 1308, 1312, 1316, 1320, 
		1323, 1339, 1344, 1348, 1352, 1355, 1358, 1364, 1368, 1372, 1374, 1378, 
		1382, 1386, 1388, 1392, 1396, 1402, 1408, 1413, 1417, 1421, 1426, 1428, 
		1431, 1435, 1438, 1441, 1447, 1451, 1455, 1463, 1467, 1471, 1475, 1479, 
		1483, 1487, 1491, 1495, 1499, 1502, 1509, 1513, 1518, 1528
	];
}
