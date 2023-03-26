use crate::statement::Statement;
use crate::token::delim::{Curly, Surround};

pub struct Block(pub Surround<Curly, Vec<Statement>>);
