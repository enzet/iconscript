// Generated from ../grammar/IconScript.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::iconscriptlistener::*;
use super::iconscriptvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const VARIABLE:isize=16; 
		pub const FLOAT:isize=17; 
		pub const IDENTIFIER:isize=18; 
		pub const COMMENT:isize=19; 
		pub const WS:isize=20;
	pub const RULE_script:usize = 0; 
	pub const RULE_position:usize = 1; 
	pub const RULE_expression:usize = 2; 
	pub const RULE_assignment:usize = 3; 
	pub const RULE_scope:usize = 4; 
	pub const RULE_commands:usize = 5; 
	pub const RULE_icon:usize = 6; 
	pub const RULE_command:usize = 7; 
	pub const RULE_arc:usize = 8; 
	pub const RULE_circle:usize = 9; 
	pub const RULE_line:usize = 10; 
	pub const RULE_rectangle:usize = 11; 
	pub const RULE_name:usize = 12; 
	pub const RULE_setPosition:usize = 13; 
	pub const RULE_setWidth:usize = 14; 
	pub const RULE_setRemove:usize = 15; 
	pub const RULE_setFill:usize = 16;
	pub const ruleNames: [&'static str; 17] =  [
		"script", "position", "expression", "assignment", "scope", "commands", 
		"icon", "command", "arc", "circle", "line", "rectangle", "name", "setPosition", 
		"setWidth", "setRemove", "setFill"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;16] = [
		None, Some("'+'"), Some("','"), Some("'='"), Some("'{'"), Some("'}'"), 
		Some("'icon'"), Some("'a'"), Some("'e'"), Some("'l'"), Some("'lf'"), Some("'r'"), 
		Some("'m'"), Some("'w'"), Some("'subtract'"), Some("'fill'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;21]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, Some("VARIABLE"), Some("FLOAT"), Some("IDENTIFIER"), 
		Some("COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,IconScriptParserExt<'input>, I, IconScriptParserContextType , dyn IconScriptListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type IconScriptTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, IconScriptParserContextType , dyn IconScriptListener<'input> + 'a>;

/// Parser for IconScript grammar
pub struct IconScriptParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				IconScriptParserExt{
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

impl<'input, I> IconScriptParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> IconScriptParser<'input, I, DefaultErrorStrategy<'input,IconScriptParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for IconScriptParser
pub trait IconScriptParserContext<'input>:
	for<'x> Listenable<dyn IconScriptListener<'input> + 'x > + 
	for<'x> Visitable<dyn IconScriptVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=IconScriptParserContextType>
{}

antlr_rust::coerce_from!{ 'input : IconScriptParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn IconScriptParserContext<'input> + 'input
where
    T: IconScriptVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn IconScriptVisitor<'input> + 'x))
    }
}

impl<'input> IconScriptParserContext<'input> for TerminalNode<'input,IconScriptParserContextType> {}
impl<'input> IconScriptParserContext<'input> for ErrorNode<'input,IconScriptParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn IconScriptParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn IconScriptListener<'input> + 'input }

pub struct IconScriptParserContextType;
antlr_rust::tid!{IconScriptParserContextType}

impl<'input> ParserNodeType<'input> for IconScriptParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn IconScriptParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct IconScriptParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> IconScriptParserExt<'input>{
}
antlr_rust::tid! { IconScriptParserExt<'a> }

impl<'input> TokenAware<'input> for IconScriptParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for IconScriptParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for IconScriptParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "IconScript.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- script ----------------
pub type ScriptContextAll<'input> = ScriptContext<'input>;


pub type ScriptContext<'input> = BaseParserRuleContext<'input,ScriptContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for ScriptContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for ScriptContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_script(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_script(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for ScriptContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_script(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScriptContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script }
}
antlr_rust::tid!{ScriptContextExt<'a>}

impl<'input> ScriptContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScriptContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScriptContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScriptContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<ScriptContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ScriptContextAttrs<'input> for ScriptContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn script(&mut self,)
	-> Result<Rc<ScriptContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScriptContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_script);
        let mut _localctx: Rc<ScriptContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(37);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 || _la==IDENTIFIER {
				{
				{
				/*InvokeRule expression*/
				recog.base.set_state(34);
				recog.expression()?;

				}
				}
				recog.base.set_state(39);
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- position ----------------
pub type PositionContextAll<'input> = PositionContext<'input>;


pub type PositionContext<'input> = BaseParserRuleContext<'input,PositionContextExt<'input>>;

#[derive(Clone)]
pub struct PositionContextExt<'input>{
	pub relative: Option<TokenType<'input>>,
	pub x: Option<TokenType<'input>>,
	pub y: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for PositionContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for PositionContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_position(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_position(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for PositionContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_position(self);
	}
}

impl<'input> CustomRuleContext<'input> for PositionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_position }
	//fn type_rule_index() -> usize where Self: Sized { RULE_position }
}
antlr_rust::tid!{PositionContextExt<'a>}

impl<'input> PositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PositionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PositionContextExt{
				relative: None, x: None, y: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PositionContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<PositionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token FLOAT in current rule
fn FLOAT_all(&self) -> Vec<Rc<TerminalNode<'input,IconScriptParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token FLOAT, starting from 0.
/// Returns `None` if number of children corresponding to token FLOAT is less or equal than `i`.
fn FLOAT(&self, i: usize) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, i)
}

}

impl<'input> PositionContextAttrs<'input> for PositionContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn position(&mut self,)
	-> Result<Rc<PositionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PositionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_position);
        let mut _localctx: Rc<PositionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__0 {
				{
				recog.base.set_state(40);
				let tmp = recog.base.match_token(T__0,&mut recog.err_handler)?;
				 cast_mut::<_,PositionContext >(&mut _localctx).relative = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(43);
			let tmp = recog.base.match_token(FLOAT,&mut recog.err_handler)?;
			 cast_mut::<_,PositionContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(44);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(45);
			let tmp = recog.base.match_token(FLOAT,&mut recog.err_handler)?;
			 cast_mut::<_,PositionContext >(&mut _localctx).y = Some(tmp.clone());
			  

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn icon(&self) -> Option<Rc<IconContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(49);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignment*/
					recog.base.set_state(47);
					recog.assignment()?;

					}
				}

			 T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule icon*/
					recog.base.set_state(48);
					recog.icon()?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignment ----------------
pub type AssignmentContextAll<'input> = AssignmentContext<'input>;


pub type AssignmentContext<'input> = BaseParserRuleContext<'input,AssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentContextExt<'input>{
	pub left: Option<TokenType<'input>>,
	pub right: Option<Rc<CommandsContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for AssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for AssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignment(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_assignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for AssignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_assignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment }
}
antlr_rust::tid!{AssignmentContextExt<'a>}

impl<'input> AssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentContextExt{
				left: None, 
				right: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<AssignmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn commands(&self) -> Option<Rc<CommandsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment(&mut self,)
	-> Result<Rc<AssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(51);
			let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
			 cast_mut::<_,AssignmentContext >(&mut _localctx).left = Some(tmp.clone());
			  

			recog.base.set_state(52);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule commands*/
			recog.base.set_state(53);
			let tmp = recog.commands()?;
			 cast_mut::<_,AssignmentContext >(&mut _localctx).right = Some(tmp.clone());
			  

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scope ----------------
pub type ScopeContextAll<'input> = ScopeContext<'input>;


pub type ScopeContext<'input> = BaseParserRuleContext<'input,ScopeContextExt<'input>>;

#[derive(Clone)]
pub struct ScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for ScopeContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for ScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scope(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_scope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for ScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_scope(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scope }
}
antlr_rust::tid!{ScopeContextExt<'a>}

impl<'input> ScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScopeContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<ScopeContextExt<'input>>{

fn commands(&self) -> Option<Rc<CommandsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ScopeContextAttrs<'input> for ScopeContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scope(&mut self,)
	-> Result<Rc<ScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_scope);
        let mut _localctx: Rc<ScopeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(55);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule commands*/
			recog.base.set_state(56);
			recog.commands()?;

			recog.base.set_state(57);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- commands ----------------
pub type CommandsContextAll<'input> = CommandsContext<'input>;


pub type CommandsContext<'input> = BaseParserRuleContext<'input,CommandsContextExt<'input>>;

#[derive(Clone)]
pub struct CommandsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for CommandsContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for CommandsContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_commands(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_commands(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for CommandsContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_commands(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommandsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_commands }
	//fn type_rule_index() -> usize where Self: Sized { RULE_commands }
}
antlr_rust::tid!{CommandsContextExt<'a>}

impl<'input> CommandsContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommandsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommandsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommandsContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<CommandsContextExt<'input>>{

fn command_all(&self) ->  Vec<Rc<CommandContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn command(&self, i: usize) -> Option<Rc<CommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn scope_all(&self) ->  Vec<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scope(&self, i: usize) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CommandsContextAttrs<'input> for CommandsContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn commands(&mut self,)
	-> Result<Rc<CommandsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommandsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_commands);
        let mut _localctx: Rc<CommandsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(61); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					recog.base.set_state(61);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 | T__13 | T__14 |
					 VARIABLE | IDENTIFIER 
						=> {
							{
							/*InvokeRule command*/
							recog.base.set_state(59);
							recog.command()?;

							}
						}

					 T__3 
						=> {
							{
							/*InvokeRule scope*/
							recog.base.set_state(60);
							recog.scope()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(63); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- icon ----------------
pub type IconContextAll<'input> = IconContext<'input>;


pub type IconContext<'input> = BaseParserRuleContext<'input,IconContextExt<'input>>;

#[derive(Clone)]
pub struct IconContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for IconContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for IconContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_icon(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_icon(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for IconContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_icon(self);
	}
}

impl<'input> CustomRuleContext<'input> for IconContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_icon }
	//fn type_rule_index() -> usize where Self: Sized { RULE_icon }
}
antlr_rust::tid!{IconContextExt<'a>}

impl<'input> IconContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IconContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IconContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IconContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<IconContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn commands(&self) -> Option<Rc<CommandsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn command(&self) -> Option<Rc<CommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IconContextAttrs<'input> for IconContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn icon(&mut self,)
	-> Result<Rc<IconContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IconContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_icon);
        let mut _localctx: Rc<IconContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(77);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(65);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule name*/
					recog.base.set_state(66);
					recog.name()?;

					recog.base.set_state(67);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					recog.base.set_state(68);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					/*InvokeRule commands*/
					recog.base.set_state(69);
					recog.commands()?;

					recog.base.set_state(70);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(72);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule name*/
					recog.base.set_state(73);
					recog.name()?;

					recog.base.set_state(74);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule command*/
					recog.base.set_state(75);
					recog.command()?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- command ----------------
pub type CommandContextAll<'input> = CommandContext<'input>;


pub type CommandContext<'input> = BaseParserRuleContext<'input,CommandContextExt<'input>>;

#[derive(Clone)]
pub struct CommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for CommandContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for CommandContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_command(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_command(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for CommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_command(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_command }
	//fn type_rule_index() -> usize where Self: Sized { RULE_command }
}
antlr_rust::tid!{CommandContextExt<'a>}

impl<'input> CommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommandContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<CommandContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VARIABLE
/// Returns `None` if there is no child corresponding to token VARIABLE
fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(VARIABLE, 0)
}
fn arc(&self) -> Option<Rc<ArcContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn circle(&self) -> Option<Rc<CircleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn line(&self) -> Option<Rc<LineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rectangle(&self) -> Option<Rc<RectangleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setPosition(&self) -> Option<Rc<SetPositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setRemove(&self) -> Option<Rc<SetRemoveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setWidth(&self) -> Option<Rc<SetWidthContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setFill(&self) -> Option<Rc<SetFillContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CommandContextAttrs<'input> for CommandContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn command(&mut self,)
	-> Result<Rc<CommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_command);
        let mut _localctx: Rc<CommandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(89);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule name*/
					recog.base.set_state(79);
					recog.name()?;

					}
				}

			 VARIABLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(80);
					recog.base.match_token(VARIABLE,&mut recog.err_handler)?;

					}
				}

			 T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule arc*/
					recog.base.set_state(81);
					recog.arc()?;

					}
				}

			 T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule circle*/
					recog.base.set_state(82);
					recog.circle()?;

					}
				}

			 T__8 | T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule line*/
					recog.base.set_state(83);
					recog.line()?;

					}
				}

			 T__10 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule rectangle*/
					recog.base.set_state(84);
					recog.rectangle()?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule setPosition*/
					recog.base.set_state(85);
					recog.setPosition()?;

					}
				}

			 T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule setRemove*/
					recog.base.set_state(86);
					recog.setRemove()?;

					}
				}

			 T__12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule setWidth*/
					recog.base.set_state(87);
					recog.setWidth()?;

					}
				}

			 T__14 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule setFill*/
					recog.base.set_state(88);
					recog.setFill()?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arc ----------------
pub type ArcContextAll<'input> = ArcContext<'input>;


pub type ArcContext<'input> = BaseParserRuleContext<'input,ArcContextExt<'input>>;

#[derive(Clone)]
pub struct ArcContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for ArcContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for ArcContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arc(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_arc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for ArcContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_arc(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArcContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arc }
}
antlr_rust::tid!{ArcContextExt<'a>}

impl<'input> ArcContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArcContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArcContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArcContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<ArcContextExt<'input>>{

fn position(&self) -> Option<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token FLOAT in current rule
fn FLOAT_all(&self) -> Vec<Rc<TerminalNode<'input,IconScriptParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token FLOAT, starting from 0.
/// Returns `None` if number of children corresponding to token FLOAT is less or equal than `i`.
fn FLOAT(&self, i: usize) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, i)
}

}

impl<'input> ArcContextAttrs<'input> for ArcContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arc(&mut self,)
	-> Result<Rc<ArcContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArcContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_arc);
        let mut _localctx: Rc<ArcContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(91);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			/*InvokeRule position*/
			recog.base.set_state(92);
			recog.position()?;

			recog.base.set_state(93);
			recog.base.match_token(FLOAT,&mut recog.err_handler)?;

			recog.base.set_state(94);
			recog.base.match_token(FLOAT,&mut recog.err_handler)?;

			recog.base.set_state(95);
			recog.base.match_token(FLOAT,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- circle ----------------
pub type CircleContextAll<'input> = CircleContext<'input>;


pub type CircleContext<'input> = BaseParserRuleContext<'input,CircleContextExt<'input>>;

#[derive(Clone)]
pub struct CircleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for CircleContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for CircleContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_circle(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_circle(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for CircleContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_circle(self);
	}
}

impl<'input> CustomRuleContext<'input> for CircleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_circle }
	//fn type_rule_index() -> usize where Self: Sized { RULE_circle }
}
antlr_rust::tid!{CircleContextExt<'a>}

impl<'input> CircleContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CircleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CircleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CircleContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<CircleContextExt<'input>>{

fn position(&self) -> Option<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}

}

impl<'input> CircleContextAttrs<'input> for CircleContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn circle(&mut self,)
	-> Result<Rc<CircleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CircleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_circle);
        let mut _localctx: Rc<CircleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(97);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			/*InvokeRule position*/
			recog.base.set_state(98);
			recog.position()?;

			recog.base.set_state(99);
			recog.base.match_token(FLOAT,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- line ----------------
pub type LineContextAll<'input> = LineContext<'input>;


pub type LineContext<'input> = BaseParserRuleContext<'input,LineContextExt<'input>>;

#[derive(Clone)]
pub struct LineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for LineContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for LineContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_line(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_line(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for LineContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_line(self);
	}
}

impl<'input> CustomRuleContext<'input> for LineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_line }
	//fn type_rule_index() -> usize where Self: Sized { RULE_line }
}
antlr_rust::tid!{LineContextExt<'a>}

impl<'input> LineContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LineContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<LineContextExt<'input>>{

fn position_all(&self) ->  Vec<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn position(&self, i: usize) -> Option<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LineContextAttrs<'input> for LineContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn line(&mut self,)
	-> Result<Rc<LineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_line);
        let mut _localctx: Rc<LineContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(101);
			_la = recog.base.input.la(1);
			if { !(_la==T__8 || _la==T__9) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(103); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule position*/
				recog.base.set_state(102);
				recog.position()?;

				}
				}
				recog.base.set_state(105); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__0 || _la==FLOAT) {break}
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rectangle ----------------
pub type RectangleContextAll<'input> = RectangleContext<'input>;


pub type RectangleContext<'input> = BaseParserRuleContext<'input,RectangleContextExt<'input>>;

#[derive(Clone)]
pub struct RectangleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for RectangleContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for RectangleContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rectangle(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_rectangle(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for RectangleContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_rectangle(self);
	}
}

impl<'input> CustomRuleContext<'input> for RectangleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rectangle }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rectangle }
}
antlr_rust::tid!{RectangleContextExt<'a>}

impl<'input> RectangleContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RectangleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RectangleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RectangleContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<RectangleContextExt<'input>>{

fn position_all(&self) ->  Vec<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn position(&self, i: usize) -> Option<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RectangleContextAttrs<'input> for RectangleContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rectangle(&mut self,)
	-> Result<Rc<RectangleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RectangleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_rectangle);
        let mut _localctx: Rc<RectangleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule position*/
			recog.base.set_state(108);
			recog.position()?;

			/*InvokeRule position*/
			recog.base.set_state(109);
			recog.position()?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;


pub type NameContext<'input> = BaseParserRuleContext<'input,NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for NameContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for NameContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_name(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for NameContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr_rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NameContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<NameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> NameContextAttrs<'input> for NameContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn name(&mut self,)
	-> Result<Rc<NameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(111);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- setPosition ----------------
pub type SetPositionContextAll<'input> = SetPositionContext<'input>;


pub type SetPositionContext<'input> = BaseParserRuleContext<'input,SetPositionContextExt<'input>>;

#[derive(Clone)]
pub struct SetPositionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for SetPositionContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for SetPositionContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setPosition(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_setPosition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for SetPositionContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_setPosition(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetPositionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setPosition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setPosition }
}
antlr_rust::tid!{SetPositionContextExt<'a>}

impl<'input> SetPositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetPositionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetPositionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetPositionContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<SetPositionContextExt<'input>>{

fn position(&self) -> Option<Rc<PositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SetPositionContextAttrs<'input> for SetPositionContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setPosition(&mut self,)
	-> Result<Rc<SetPositionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetPositionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_setPosition);
        let mut _localctx: Rc<SetPositionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(113);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule position*/
			recog.base.set_state(114);
			recog.position()?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- setWidth ----------------
pub type SetWidthContextAll<'input> = SetWidthContext<'input>;


pub type SetWidthContext<'input> = BaseParserRuleContext<'input,SetWidthContextExt<'input>>;

#[derive(Clone)]
pub struct SetWidthContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for SetWidthContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for SetWidthContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setWidth(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_setWidth(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for SetWidthContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_setWidth(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetWidthContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setWidth }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setWidth }
}
antlr_rust::tid!{SetWidthContextExt<'a>}

impl<'input> SetWidthContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetWidthContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetWidthContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetWidthContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<SetWidthContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,IconScriptParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}

}

impl<'input> SetWidthContextAttrs<'input> for SetWidthContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setWidth(&mut self,)
	-> Result<Rc<SetWidthContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetWidthContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_setWidth);
        let mut _localctx: Rc<SetWidthContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(116);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(117);
			recog.base.match_token(FLOAT,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- setRemove ----------------
pub type SetRemoveContextAll<'input> = SetRemoveContext<'input>;


pub type SetRemoveContext<'input> = BaseParserRuleContext<'input,SetRemoveContextExt<'input>>;

#[derive(Clone)]
pub struct SetRemoveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for SetRemoveContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for SetRemoveContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setRemove(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_setRemove(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for SetRemoveContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_setRemove(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetRemoveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setRemove }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setRemove }
}
antlr_rust::tid!{SetRemoveContextExt<'a>}

impl<'input> SetRemoveContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetRemoveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetRemoveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetRemoveContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<SetRemoveContextExt<'input>>{


}

impl<'input> SetRemoveContextAttrs<'input> for SetRemoveContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setRemove(&mut self,)
	-> Result<Rc<SetRemoveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetRemoveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_setRemove);
        let mut _localctx: Rc<SetRemoveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(119);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- setFill ----------------
pub type SetFillContextAll<'input> = SetFillContext<'input>;


pub type SetFillContext<'input> = BaseParserRuleContext<'input,SetFillContextExt<'input>>;

#[derive(Clone)]
pub struct SetFillContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> IconScriptParserContext<'input> for SetFillContext<'input>{}

impl<'input,'a> Listenable<dyn IconScriptListener<'input> + 'a> for SetFillContext<'input>{
		fn enter(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setFill(self);
		}
		fn exit(&self,listener: &mut (dyn IconScriptListener<'input> + 'a)) {
			listener.exit_setFill(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn IconScriptVisitor<'input> + 'a> for SetFillContext<'input>{
	fn accept(&self,visitor: &mut (dyn IconScriptVisitor<'input> + 'a)) {
		visitor.visit_setFill(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetFillContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = IconScriptParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setFill }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setFill }
}
antlr_rust::tid!{SetFillContextExt<'a>}

impl<'input> SetFillContextExt<'input>{
	fn new(parent: Option<Rc<dyn IconScriptParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetFillContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetFillContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetFillContextAttrs<'input>: IconScriptParserContext<'input> + BorrowMut<SetFillContextExt<'input>>{


}

impl<'input> SetFillContextAttrs<'input> for SetFillContext<'input>{}

impl<'input, I, H> IconScriptParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setFill(&mut self,)
	-> Result<Rc<SetFillContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetFillContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_setFill);
        let mut _localctx: Rc<SetFillContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(121);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x16\x7e\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x03\x02\
	\x07\x02\x26\x0a\x02\x0c\x02\x0e\x02\x29\x0b\x02\x03\x03\x05\x03\x2c\x0a\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x05\x04\x34\x0a\x04\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\
	\x03\x07\x06\x07\x40\x0a\x07\x0d\x07\x0e\x07\x41\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x05\x08\x50\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x05\x09\x5c\x0a\x09\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\
	\x06\x0c\x6a\x0a\x0c\x0d\x0c\x0e\x0c\x6b\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\x11\
	\x03\x11\x03\x12\x03\x12\x03\x12\x02\x02\x13\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x02\x03\x03\x02\x0b\x0c\x02\x7c\
	\x02\x27\x03\x02\x02\x02\x04\x2b\x03\x02\x02\x02\x06\x33\x03\x02\x02\x02\
	\x08\x35\x03\x02\x02\x02\x0a\x39\x03\x02\x02\x02\x0c\x3f\x03\x02\x02\x02\
	\x0e\x4f\x03\x02\x02\x02\x10\x5b\x03\x02\x02\x02\x12\x5d\x03\x02\x02\x02\
	\x14\x63\x03\x02\x02\x02\x16\x67\x03\x02\x02\x02\x18\x6d\x03\x02\x02\x02\
	\x1a\x71\x03\x02\x02\x02\x1c\x73\x03\x02\x02\x02\x1e\x76\x03\x02\x02\x02\
	\x20\x79\x03\x02\x02\x02\x22\x7b\x03\x02\x02\x02\x24\x26\x05\x06\x04\x02\
	\x25\x24\x03\x02\x02\x02\x26\x29\x03\x02\x02\x02\x27\x25\x03\x02\x02\x02\
	\x27\x28\x03\x02\x02\x02\x28\x03\x03\x02\x02\x02\x29\x27\x03\x02\x02\x02\
	\x2a\x2c\x07\x03\x02\x02\x2b\x2a\x03\x02\x02\x02\x2b\x2c\x03\x02\x02\x02\
	\x2c\x2d\x03\x02\x02\x02\x2d\x2e\x07\x13\x02\x02\x2e\x2f\x07\x04\x02\x02\
	\x2f\x30\x07\x13\x02\x02\x30\x05\x03\x02\x02\x02\x31\x34\x05\x08\x05\x02\
	\x32\x34\x05\x0e\x08\x02\x33\x31\x03\x02\x02\x02\x33\x32\x03\x02\x02\x02\
	\x34\x07\x03\x02\x02\x02\x35\x36\x07\x14\x02\x02\x36\x37\x07\x05\x02\x02\
	\x37\x38\x05\x0c\x07\x02\x38\x09\x03\x02\x02\x02\x39\x3a\x07\x06\x02\x02\
	\x3a\x3b\x05\x0c\x07\x02\x3b\x3c\x07\x07\x02\x02\x3c\x0b\x03\x02\x02\x02\
	\x3d\x40\x05\x10\x09\x02\x3e\x40\x05\x0a\x06\x02\x3f\x3d\x03\x02\x02\x02\
	\x3f\x3e\x03\x02\x02\x02\x40\x41\x03\x02\x02\x02\x41\x3f\x03\x02\x02\x02\
	\x41\x42\x03\x02\x02\x02\x42\x0d\x03\x02\x02\x02\x43\x44\x07\x08\x02\x02\
	\x44\x45\x05\x1a\x0e\x02\x45\x46\x07\x05\x02\x02\x46\x47\x07\x06\x02\x02\
	\x47\x48\x05\x0c\x07\x02\x48\x49\x07\x07\x02\x02\x49\x50\x03\x02\x02\x02\
	\x4a\x4b\x07\x08\x02\x02\x4b\x4c\x05\x1a\x0e\x02\x4c\x4d\x07\x05\x02\x02\
	\x4d\x4e\x05\x10\x09\x02\x4e\x50\x03\x02\x02\x02\x4f\x43\x03\x02\x02\x02\
	\x4f\x4a\x03\x02\x02\x02\x50\x0f\x03\x02\x02\x02\x51\x5c\x05\x1a\x0e\x02\
	\x52\x5c\x07\x12\x02\x02\x53\x5c\x05\x12\x0a\x02\x54\x5c\x05\x14\x0b\x02\
	\x55\x5c\x05\x16\x0c\x02\x56\x5c\x05\x18\x0d\x02\x57\x5c\x05\x1c\x0f\x02\
	\x58\x5c\x05\x20\x11\x02\x59\x5c\x05\x1e\x10\x02\x5a\x5c\x05\x22\x12\x02\
	\x5b\x51\x03\x02\x02\x02\x5b\x52\x03\x02\x02\x02\x5b\x53\x03\x02\x02\x02\
	\x5b\x54\x03\x02\x02\x02\x5b\x55\x03\x02\x02\x02\x5b\x56\x03\x02\x02\x02\
	\x5b\x57\x03\x02\x02\x02\x5b\x58\x03\x02\x02\x02\x5b\x59\x03\x02\x02\x02\
	\x5b\x5a\x03\x02\x02\x02\x5c\x11\x03\x02\x02\x02\x5d\x5e\x07\x09\x02\x02\
	\x5e\x5f\x05\x04\x03\x02\x5f\x60\x07\x13\x02\x02\x60\x61\x07\x13\x02\x02\
	\x61\x62\x07\x13\x02\x02\x62\x13\x03\x02\x02\x02\x63\x64\x07\x0a\x02\x02\
	\x64\x65\x05\x04\x03\x02\x65\x66\x07\x13\x02\x02\x66\x15\x03\x02\x02\x02\
	\x67\x69\x09\x02\x02\x02\x68\x6a\x05\x04\x03\x02\x69\x68\x03\x02\x02\x02\
	\x6a\x6b\x03\x02\x02\x02\x6b\x69\x03\x02\x02\x02\x6b\x6c\x03\x02\x02\x02\
	\x6c\x17\x03\x02\x02\x02\x6d\x6e\x07\x0d\x02\x02\x6e\x6f\x05\x04\x03\x02\
	\x6f\x70\x05\x04\x03\x02\x70\x19\x03\x02\x02\x02\x71\x72\x07\x14\x02\x02\
	\x72\x1b\x03\x02\x02\x02\x73\x74\x07\x0e\x02\x02\x74\x75\x05\x04\x03\x02\
	\x75\x1d\x03\x02\x02\x02\x76\x77\x07\x0f\x02\x02\x77\x78\x07\x13\x02\x02\
	\x78\x1f\x03\x02\x02\x02\x79\x7a\x07\x10\x02\x02\x7a\x21\x03\x02\x02\x02\
	\x7b\x7c\x07\x11\x02\x02\x7c\x23\x03\x02\x02\x02\x0a\x27\x2b\x33\x3f\x41\
	\x4f\x5b\x6b";

