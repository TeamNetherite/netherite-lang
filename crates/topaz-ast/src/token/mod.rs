pub mod keyword;
pub mod prefix;
pub mod punctuation;
pub mod delim;

pub use keyword::*;
pub use prefix::*;
pub use punctuation::*;

macro make_token_macro($([$token:tt] => $path:path),*) {
    pub macro Token {
        $([$token] => (::topaz_ast::token::$path),)*
    }
}

/*
#[macro_export]
macro_rules! Token {
    // Prefixes and punctuation
    [,] => ($crate::token::punctuation::Comma);
    [:] => ($crate::token::punctuation::Colon);
    [;] => ($crate::punctuation::Semi);
    [&] => ($crate::prefix::Ref);
    [->] => ($crate::punctuation::Arrow);
    // Keywords
    [mut] => ($crate::keyword::Mut);
    [func] => ($crate::keyword::Func);
    [let] => ($crate::keyword::Let);
    [maybe] => ($crate::keyword::Maybe);
    [some] => ($crate::keyword::Some);
    [nope] => ($crate::keyword::Nope);
}
 */

make_token_macro! {
    [,] => punctuation::Comma,
    [:] => punctuation::Colon,
    [;] => punctuation::Semi,
    [&] => prefix::Ref,
    [->] => punctuation::Arrow,

    [mut] => keyword::Mut,
    [func] => keyword::Func,
    [let] => keyword::Let,
    [maybe] => keyword::Maybe,
    [some] => keyword::Some,
    [nope] => keyword::Nope
}

pub(self) macro default_token_struct($($name:ident);*$(;)?) {
    $(
    #[derive(Default)]
    pub struct $name;
    )*
}
