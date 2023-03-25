use crate::statement::Statement;
use crate::token::delim::{Curly, Surround};

pub type Block = Surround<Curly, Vec<Statement>>;
