#![feature(default_free_fn)]

use topaz_ast::location::WithSpan;
use topaz_ast::punctuated::Punctuated;
use topaz_ast::Tokens;
lalrpop_util::lalrpop_mod!(pub(crate) grammar, "/src/grammar.rs");

pub mod lex;
pub mod parse;

pub type Result<T> = core::result::Result<T, TopazParseError>;

#[derive(thiserror::Error, Debug)]
pub enum TopazParseError {
    #[error("String delimit failed in `{0}`")]
    StringDelimitFailed(WithSpan<String>)
}

pub type ParseStream<'a> = &'a str;

pub trait Parse {
    fn parse(input: ParseStream) -> Result<Self> where Self: Sized;
}

fn make_real<T: Tokens, P: Tokens>(mut t: (Vec<T>, T)) -> Punctuated<T, P> {
    t.0.push(t.1);
    t.0
}
