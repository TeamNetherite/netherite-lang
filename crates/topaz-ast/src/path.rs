use crate::ident::Ident;
use crate::punctuated::Punctuated;
use crate::Token;

pub struct Path {
    pub segments: Punctuated<Ident, Token![,]>
}
