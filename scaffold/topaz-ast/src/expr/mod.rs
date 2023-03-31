use crate::literal::Literal;
use crate::path::{DottedPath, Path};
use crate::statement::func_call::FuncCallStmt;
use crate::Token;

pub enum Expr {
    Literal(ExprLit),
    Borrow(ExprBorrow),
    VariableAccess(ExprVarAccess),
    ConstAccess(ExprConstAccess),
    FuncCall(FuncCallStmt)
}

pub struct ExprLit(pub Literal);

/// crate::THING
pub struct ExprConstAccess(pub Path);

/// thing
pub struct ExprVarAccess(pub DottedPath);

/// &thing or &mut thing
pub struct ExprBorrow(pub Token![&], pub Option<Token![mut]>, pub Box<Expr>);
