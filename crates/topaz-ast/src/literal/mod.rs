use crate::token::delim::{StringLit};
pub mod number;

pub enum Literal {
    String(LiteralString),
    Number(number::LiteralNumber),
}

pub struct LiteralString(pub StringLit);
