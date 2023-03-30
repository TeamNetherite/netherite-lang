mod impls;
pub use impls::*;

pub type Result<T> = core::result::Result<T, ParseError>;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("LALRPOP parser error: {0}")]
    Parser(#[from] lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'static>, &'static str>),
}

pub type ParseStream<'a> = &'a str;

pub trait Parse {
    fn parse(stream: ParseStream) -> Result<Self>
    where
        Self: Sized;
}

pub(self) macro parse_impl($($real:path as $parser_name:ident),*$(,)?) {
    $(
    impl crate::parse::Parse for $real {
        fn parse(stream: &str) -> crate::parse::Result<Self> {
            Ok(crate::grammar::$parser_name::new().parse(stream)?)
        }
    }
    )*
}

