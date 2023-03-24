use crate::ident::Ident;
use crate::literal::Literal;
use crate::path::Path;
use crate::Token;

pub enum Expr {
    Literal(ExprLit),
    Borrow(ExprBorrow),
    VariableAccess(ExprVarAccess),
    ConstAccess(ExprConstAccess),
}

pub struct ExprLit(Literal);

pub struct ExprConstAccess {
    pub const_path: Path,
}

pub struct ExprVarAccess {
    pub variable_name: Ident,
}

/// &thing or &mut thing
pub struct ExprBorrow {
    pub ref_token: Token![&],
    pub mut_token: Option<Token![mut]>,
    pub referenced: Box<Expr>,
}
