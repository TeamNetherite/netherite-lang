use crate::expr::Expr;
use crate::ident::Ident;
use crate::path::Path;
use crate::punctuated::Punctuated;
use crate::Token;
use crate::token::delim::{Parentheses, Surround};

#[tokens]
pub struct FuncCallStmt(pub Path, pub Surround<Parentheses, Punctuated<FuncCallArg, Token![,]>>);

/// arg_name = some 8uexplod \
/// Some(8u128)
#[tokens]
pub struct FuncCallArg(pub Option<(Ident, Token![=])>, pub Expr);
