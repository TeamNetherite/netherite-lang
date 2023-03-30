use crate::statement::Statement;
use crate::token::delim::{Curly, Surround};

#[derive(Tokens)]
pub struct Block(pub Surround<Curly, Vec<Statement>>);
