// Generated from ../grammar/IconScript.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


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
	pub const VARIABLE:isize=15; 
	pub const FLOAT:isize=16; 
	pub const IDENTIFIER:isize=17; 
	pub const COMMENT:isize=18; 
	pub const WS:isize=19;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;19] = [
		"T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
		"T__9", "T__10", "T__11", "T__12", "T__13", "VARIABLE", "FLOAT", "IDENTIFIER", 
		"COMMENT", "WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;15] = [
		None, Some("'+'"), Some("','"), Some("'='"), Some("'{'"), Some("'}'"), 
		Some("'icon'"), Some("'a'"), Some("'e'"), Some("'l'"), Some("'r'"), Some("'m'"), 
		Some("'w'"), Some("'subtract'"), Some("'fill'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;20]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, Some("VARIABLE"), Some("FLOAT"), Some("IDENTIFIER"), 
		Some("COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct IconScriptLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,IconScriptLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for IconScriptLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for IconScriptLexer<'input,Input>{
	type Target = BaseLexer<'input,IconScriptLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for IconScriptLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> IconScriptLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "IconScriptLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				IconScriptLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> IconScriptLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		IconScriptLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct IconScriptLexerActions {
}

impl IconScriptLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,IconScriptLexerActions,Input,LocalTokenFactory<'input>>> for IconScriptLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> IconScriptLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,IconScriptLexerActions,Input,LocalTokenFactory<'input>>> for IconScriptLexerActions{
}
impl<'input> TokenAware<'input> for IconScriptLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for IconScriptLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x15\x7a\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\
		\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\
		\x04\x13\x09\x13\x04\x14\x09\x14\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\
		\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\
		\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\
		\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
		\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\
		\x03\x10\x03\x10\x03\x10\x03\x11\x05\x11\x57\x0a\x11\x03\x11\x06\x11\x5a\
		\x0a\x11\x0d\x11\x0e\x11\x5b\x03\x11\x03\x11\x07\x11\x60\x0a\x11\x0c\x11\
		\x0e\x11\x63\x0b\x11\x05\x11\x65\x0a\x11\x03\x12\x03\x12\x07\x12\x69\x0a\
		\x12\x0c\x12\x0e\x12\x6c\x0b\x12\x03\x13\x03\x13\x07\x13\x70\x0a\x13\x0c\
		\x13\x0e\x13\x73\x0b\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x14\
		\x02\x02\x15\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\x11\
		\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\x1d\x10\x1f\x11\x21\x12\x23\
		\x13\x25\x14\x27\x15\x03\x02\x07\x03\x02\x32\x3b\x05\x02\x43\x5c\x61\x61\
		\x63\x7c\x06\x02\x32\x3b\x43\x5c\x61\x61\x63\x7c\x04\x02\x0c\x0c\x0f\x0f\
		\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x02\x7f\x02\x03\x03\x02\x02\x02\x02\x05\
		\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\
		\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\
		\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\
		\x03\x02\x02\x02\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\
		\x03\x02\x02\x02\x02\x1f\x03\x02\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\
		\x03\x02\x02\x02\x02\x25\x03\x02\x02\x02\x02\x27\x03\x02\x02\x02\x03\x29\
		\x03\x02\x02\x02\x05\x2b\x03\x02\x02\x02\x07\x2d\x03\x02\x02\x02\x09\x2f\
		\x03\x02\x02\x02\x0b\x31\x03\x02\x02\x02\x0d\x33\x03\x02\x02\x02\x0f\x38\
		\x03\x02\x02\x02\x11\x3a\x03\x02\x02\x02\x13\x3c\x03\x02\x02\x02\x15\x3e\
		\x03\x02\x02\x02\x17\x40\x03\x02\x02\x02\x19\x42\x03\x02\x02\x02\x1b\x44\
		\x03\x02\x02\x02\x1d\x4d\x03\x02\x02\x02\x1f\x52\x03\x02\x02\x02\x21\x56\
		\x03\x02\x02\x02\x23\x66\x03\x02\x02\x02\x25\x6d\x03\x02\x02\x02\x27\x76\
		\x03\x02\x02\x02\x29\x2a\x07\x2d\x02\x02\x2a\x04\x03\x02\x02\x02\x2b\x2c\
		\x07\x2e\x02\x02\x2c\x06\x03\x02\x02\x02\x2d\x2e\x07\x3f\x02\x02\x2e\x08\
		\x03\x02\x02\x02\x2f\x30\x07\x7d\x02\x02\x30\x0a\x03\x02\x02\x02\x31\x32\
		\x07\x7f\x02\x02\x32\x0c\x03\x02\x02\x02\x33\x34\x07\x6b\x02\x02\x34\x35\
		\x07\x65\x02\x02\x35\x36\x07\x71\x02\x02\x36\x37\x07\x70\x02\x02\x37\x0e\
		\x03\x02\x02\x02\x38\x39\x07\x63\x02\x02\x39\x10\x03\x02\x02\x02\x3a\x3b\
		\x07\x67\x02\x02\x3b\x12\x03\x02\x02\x02\x3c\x3d\x07\x6e\x02\x02\x3d\x14\
		\x03\x02\x02\x02\x3e\x3f\x07\x74\x02\x02\x3f\x16\x03\x02\x02\x02\x40\x41\
		\x07\x6f\x02\x02\x41\x18\x03\x02\x02\x02\x42\x43\x07\x79\x02\x02\x43\x1a\
		\x03\x02\x02\x02\x44\x45\x07\x75\x02\x02\x45\x46\x07\x77\x02\x02\x46\x47\
		\x07\x64\x02\x02\x47\x48\x07\x76\x02\x02\x48\x49\x07\x74\x02\x02\x49\x4a\
		\x07\x63\x02\x02\x4a\x4b\x07\x65\x02\x02\x4b\x4c\x07\x76\x02\x02\x4c\x1c\
		\x03\x02\x02\x02\x4d\x4e\x07\x68\x02\x02\x4e\x4f\x07\x6b\x02\x02\x4f\x50\
		\x07\x6e\x02\x02\x50\x51\x07\x6e\x02\x02\x51\x1e\x03\x02\x02\x02\x52\x53\
		\x07\x42\x02\x02\x53\x54\x05\x23\x12\x02\x54\x20\x03\x02\x02\x02\x55\x57\
		\x07\x2f\x02\x02\x56\x55\x03\x02\x02\x02\x56\x57\x03\x02\x02\x02\x57\x59\
		\x03\x02\x02\x02\x58\x5a\x09\x02\x02\x02\x59\x58\x03\x02\x02\x02\x5a\x5b\
		\x03\x02\x02\x02\x5b\x59\x03\x02\x02\x02\x5b\x5c\x03\x02\x02\x02\x5c\x64\
		\x03\x02\x02\x02\x5d\x61\x07\x30\x02\x02\x5e\x60\x09\x02\x02\x02\x5f\x5e\
		\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x61\x62\
		\x03\x02\x02\x02\x62\x65\x03\x02\x02\x02\x63\x61\x03\x02\x02\x02\x64\x5d\
		\x03\x02\x02\x02\x64\x65\x03\x02\x02\x02\x65\x22\x03\x02\x02\x02\x66\x6a\
		\x09\x03\x02\x02\x67\x69\x09\x04\x02\x02\x68\x67\x03\x02\x02\x02\x69\x6c\
		\x03\x02\x02\x02\x6a\x68\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\x6b\x24\
		\x03\x02\x02\x02\x6c\x6a\x03\x02\x02\x02\x6d\x71\x07\x25\x02\x02\x6e\x70\
		\x0a\x05\x02\x02\x6f\x6e\x03\x02\x02\x02\x70\x73\x03\x02\x02\x02\x71\x6f\
		\x03\x02\x02\x02\x71\x72\x03\x02\x02\x02\x72\x74\x03\x02\x02\x02\x73\x71\
		\x03\x02\x02\x02\x74\x75\x08\x13\x02\x02\x75\x26\x03\x02\x02\x02\x76\x77\
		\x09\x06\x02\x02\x77\x78\x03\x02\x02\x02\x78\x79\x08\x14\x02\x02\x79\x28\
		\x03\x02\x02\x02\x09\x02\x56\x5b\x61\x64\x6a\x71\x03\x08\x02\x02";
