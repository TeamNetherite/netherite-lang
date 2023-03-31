use crate::expr::Expr;
use crate::ident::Ident;
use crate::path::CallPath;
use crate::punctuated::Punctuated;
use crate::Token;
use crate::token::delim::{Parentheses, Surround};

#[derive(Tokens)]
pub struct FuncCallStmt(pub CallPath, pub Surround<Parentheses, Punctuated<FuncCallArg, Token![,]>>);

/// arg_name = some 8uexplod \
/// Some(8u128)
#[derive(Tokens)]
pub struct FuncCallArg(pub Option<(Ident, Token![=])>, pub Expr);
