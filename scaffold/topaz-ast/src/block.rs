use crate::statement::Statement;
use crate::token::delim::{Curly, Surround};

#[tokens]
#[derive(Eq, PartialEq)]
pub struct Block(pub Surround<Curly, Vec<Statement>>);
