use crate::ident::Ident;
use crate::path::Path;
use crate::punctuated::Punctuated;
use crate::Token;
use crate::token::delim::{Parentheses, Surround};

#[derive(Default)]
pub enum Visibility {
    #[default]
    Public,
    Private(Option<Surround<Parentheses, Punctuated<Path, Token![,]>>>),
}

impl Visibility {
    #[allow(non_snake_case)]
    pub fn Internal() -> Visibility {
        Visibility::Private(Some(Ident::from(Token![gem]).into()))
    }
}
