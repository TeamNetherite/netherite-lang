use super::{Parse, Result, grammar};
use topaz_ast as ast;
use crate::ParseStream;

macro_rules! impl_parse {
    ($parser:ident as $id:path) => {
        impl crate::Parse for $id {
            fn parse(stream: &str) -> crate::Result<Self> {
                crate::grammar::$parser::new().parse(stream)
            }
        }
    }
}

impl Parse for ast::file::TopazFile {
    fn parse(input: ParseStream) -> Result<Self> where Self: Sized {
        grammar::FileParser::new().parse(input)
    }
}
