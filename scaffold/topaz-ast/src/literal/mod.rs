use crate::token::delim::{CharLit, StringLit};
pub mod number;

#[tokens]
#[derive(Eq, PartialEq)]
pub enum Literal {
    String(LiteralString),
    Number(number::LiteralNumber),
    Char(LiteralChar)
}

#[tokens]
#[derive(Eq, PartialEq)]
pub struct LiteralString(pub StringLit);

impl<S: ToString> From<S> for LiteralString {
    fn from(value: S) -> Self {
        Self(StringLit::new(value.to_string()))
    }
}

#[tokens]
#[derive(Eq, PartialEq)]
pub struct LiteralChar(pub CharLit);

impl<C: Into<char>> From<C> for LiteralChar {
    fn from(value: C) -> Self {
        Self(CharLit::new(value.into()))
    }
}
