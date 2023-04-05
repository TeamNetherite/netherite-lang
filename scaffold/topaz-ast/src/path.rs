use crate::ident::Ident;
use crate::punctuated::Punctuated;
use crate::token::stream::{ToTokens, TokenStream};
use crate::Token;
use derive_more::From;
use std::fmt::{Display, Formatter};

#[tokens]
#[derive(Eq, From)]
pub struct Path(PathInner);

#[tokens]
#[derive(Eq, PartialEq)]
pub enum PathInner {
    Namespace(Ident, Option<AsClause>),
    Curly(Ident, Punctuated<Ident, Token![,]>),
}



#[tokens]
#[derive(Eq, PartialEq)]
pub struct AsClause(pub Token![as], pub Ident);

impl ToTokens for AsClause {
    fn write_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(&self.0);
        tokens.append(&self.1);
    }
}

impl Display for AsClause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_tokens().fmt(f)
    }
}

impl Path {
    pub fn namespace(namespace: Ident) -> Self {
        Self(PathInner::Namespace(namespace, None))
    }
}

impl From<Ident> for Path {
    fn from(value: Ident) -> Self {
        Path::namespace(value)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub enum CallPath {
    OnModule(Path),
    OnObject(Path),
}
