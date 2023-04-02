#![feature(default_free_fn)]
#![feature(box_patterns)]

use lalrpop_util::ParseError;
use topaz_ast::location::WithSpan;
use topaz_ast::punctuated::Punctuated;
use topaz_ast::Tokens;
lalrpop_util::lalrpop_mod!(pub(crate) grammar, "/src/grammar.rs");

pub mod lex;
pub mod parse;

pub type Result<T> = core::result::Result<T, TopazParseError>;

fn display_box(err: &ParseError<usize, lex::Token, TopazParseError>) -> String {
    format!("{:#?}", err)
}

#[derive(thiserror::Error, Debug)]
pub enum TopazParseError {
    #[error("Lexer error: {0}")]
    Lexer(WithSpan<String>),
    #[error("Parser error: {}", display_box(.0))]
    Parse(Box<ParseError<usize, lex::Token, TopazParseError>>)
}

pub type ParseStream<'a> = &'a str;

pub trait Parse {
    fn parse(input: ParseStream) -> Result<Self> where Self: Sized;
}

fn make_real<T: Tokens, P: Tokens>(mut t: (Vec<T>, T)) -> Punctuated<T, P> {
    t.0.push(t.1);
    Punctuated::from_segments(t.0)
}
