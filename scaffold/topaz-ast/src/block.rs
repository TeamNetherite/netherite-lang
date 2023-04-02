use crate::statement::Statement;
use crate::token::delim::{Curly, Surround};

#[tokens]
pub struct Block(pub Surround<Curly, Vec<Statement>>);
