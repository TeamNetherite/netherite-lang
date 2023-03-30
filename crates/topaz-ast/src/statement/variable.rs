use crate::expr::Expr;
use crate::ident::Ident;
use crate::{Token, types::Type};

/// ```tp
/// let mut why: (maybe (maybe int), maybe String) = (some (some 10), some "");
/// ```
#[derive(Tokens)]
pub struct LetStmt(
    pub Token![let],
    pub Option<Token![mut]>, // mutability
    pub Ident, // name
    pub Option<(Token![:], Type)>, // type
    pub Option<(Token![=], Expr)>, // initializer
);
