use crate::token::delim::{CharLit, StringLit};
pub mod number;

#[derive(Tokens)]
pub enum Literal {
    String(LiteralString),
    Number(number::LiteralNumber),
    Char(LiteralChar)
}

#[derive(Tokens)]
pub struct LiteralString(pub StringLit);

impl<S: ToString> From<S> for LiteralString {
    fn from(value: S) -> Self {
        Self(StringLit::new_default(value.to_string()))
    }
}

#[derive(Tokens)]
pub struct LiteralChar(pub CharLit);

impl<C: Into<char>> From<C> for LiteralChar {
    fn from(value: C) -> Self {
        Self(CharLit::new_default(value.into()))
    }
}