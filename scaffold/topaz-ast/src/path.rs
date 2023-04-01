use crate::ident::Ident;
use crate::punctuated::Punctuated;
use crate::Token;
use std::string::ToString;

#[tokens]
pub struct Path(pub Punctuated<Ident, Token![.]>);

impl From<Ident> for Path {
    fn from(value: Ident) -> Self {
        Self(value.into())
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub enum CallPath {
    OnModule(Path),
    OnObject(Path)
}
