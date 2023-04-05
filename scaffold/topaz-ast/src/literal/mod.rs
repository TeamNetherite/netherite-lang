use derive_more::Display;
use crate::literal::number::LiteralNumber;
use crate::token::delim::{CharLit, StringLit};
pub mod number;

#[tokens]
#[derive(Eq, PartialEq, Clone, Display)]
pub enum Literal {
    String(LiteralString),
    Number(LiteralNumber),
    Char(LiteralChar)
}

auto trait NotLiteral {}

impl !NotLiteral for Literal {}
impl !NotLiteral for LiteralString {}
impl !NotLiteral for LiteralChar {}
impl !NotLiteral for LiteralNumber {}

#[tokens]
#[derive(Eq, PartialEq, Clone, Display)]
pub struct LiteralString(pub StringLit);

impl<S: ToString + NotLiteral> From<S> for LiteralString {
    fn from(value: S) -> Self {
        Self(StringLit::new(value.to_string()))
    }
}

#[tokens]
#[derive(Eq, PartialEq, Clone, Display)]
pub struct LiteralChar(pub CharLit);

impl<C: Into<char>> From<C> for LiteralChar {
    fn from(value: C) -> Self {
        Self(CharLit::new(value.into()))
    }
}
