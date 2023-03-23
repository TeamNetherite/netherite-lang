pub mod keyword;
pub mod prefix;
pub mod punctuation;
pub mod delim;

pub use keyword::*;
pub use prefix::*;
pub use punctuation::*;

pub(crate) mod private {
    pub(crate) trait Token {}
}

pub trait Token {}

impl<T: private::Token> Token for T {}

#[macro_export]
macro_rules! Token {
    // Prefixes and punctuation
    [,] => ($crate::punctuation::Comma)
    [:] => ($crate::punctuation::Colon)
    [;] => ($crate::punctuation::Semi)
    [&] => ($crate::prefix::Ref)
    [->] => ($crate::punctuation::Arrow)
    // Keywords
    [mut] => ($crate::keyword::Mut)
    [func] => ($crate::keyword::Func)
    [let] => ($crate::keyword::Let)
}

pub(self) macro default_token_struct($($name:ident);*$(;)?) {
    $(
    #[derive(Default)]
    pub struct $name;
    )*
}
