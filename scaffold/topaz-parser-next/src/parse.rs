use lalrpop_util::ParseError;
use super::{grammar, Parse, Result};
use crate::{ParseStream, TopazParseError};
use topaz_ast as ast;

macro_rules! impl_parse {
    ($($parser:ident as $id:path),*) => {
        $(impl crate::Parse for $id {
            fn parse(stream: &str) -> crate::Result<Self> {
                deparse(crate::grammar::$parser::new().parse(crate::lex::Lexer::new(stream)))
            }
        })*
    };
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn deparse<T>(result: std::result::Result<T, ParseError<usize, crate::lex::Token, TopazParseError>>) -> Result<T> {
    result.map_err(
        |err| if let ParseError::User {
            error,
        } = err { error } else { TopazParseError::Parse(Box::new(err)) },
    )
}

impl_parse! {
    FileParser as ast::file::TopazFile,
    VisibilityParser as ast::visibility::Visibility
}
