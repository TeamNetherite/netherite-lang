use crate::literal::Literal;
use crate::path::Path;
use crate::statement::func_call::FuncCallStmt;
use crate::Token;

#[tokens]
#[derive(Eq, PartialEq)]
pub enum Expr {
    Literal(ExprLit),
    Borrow(ExprBorrow),
    VariableAccess(ExprVarAccess),
    ConstAccess(ExprConstAccess),
    FuncCall(FuncCallStmt)
}

#[tokens]
#[derive(Eq, PartialEq)]
pub struct ExprLit(pub Literal);

#[tokens]
#[derive(Eq, PartialEq)]
/// crate::THING
pub struct ExprConstAccess(pub Path);

#[tokens]
#[derive(Eq, PartialEq)]
/// thing
pub struct ExprVarAccess(pub Path);

#[tokens]
#[derive(Eq, PartialEq)]
/// &thing or &mut thing
pub struct ExprBorrow(pub Token![&], pub Option<Token![mut]>, pub Box<Expr>);
