#![allow(deprecated)]
pub mod delim;
pub mod keyword;
pub mod prefix;
pub mod punctuation;

pub use keyword::*;
pub use prefix::*;
pub use punctuation::*;

use topaz_macro::everything;
use crate::ident::Ident;

pub enum TokenTree {
    Single(SingleToken),
    Literal(crate::literal::Literal),
    Identifier(Ident)
}

everything! {
    [,] => punctuation::Comma,
    [:] => punctuation::Colon,
    [;] => punctuation::Semi,
    [.] => punctuation::Dot,
    [->] => punctuation::Arrow,
    [::] => punctuation::DoubleColon,
    [=] => punctuation::Equal,
    [-] => punctuation::Minus,
    [+] => punctuation::Plus,
    // prefixes
    [&] => prefix::Ref,
    // keywords
    [mut] => keyword::Mut,
    [func] => keyword::Func,
    [let] => keyword::Let,
    [maybe] => keyword::Maybe,
    [some] => keyword::Some,
    [nope] => keyword::Nope,
    [typealias] => keyword::TypeAlias,
    [this] => keyword::This,
    [super] => keyword::Super,
    [gem] => keyword::Gem,
    [import] => keyword::Import
}
