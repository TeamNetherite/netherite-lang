use crate::token::delim::{CharLit, StringLit};
pub mod number;

#[tokens]
pub enum Literal {
    String(LiteralString),
    Number(number::LiteralNumber),
    Char(LiteralChar)
}

#[tokens]
pub struct LiteralString(pub StringLit);

impl<S: ToString> From<S> for LiteralString {
    fn from(value: S) -> Self {
        Self(StringLit::new_default(value.to_string()))
    }
}

#[tokens]
pub struct LiteralChar(pub CharLit);

impl<C: Into<char>> From<C> for LiteralChar {
    fn from(value: C) -> Self {
        Self(CharLit::new_default(value.into()))
    }
}
