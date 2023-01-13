pub mod location;
pub mod token;

use crate::ast::location::Spanned;
use crate::ast::token::PrimaryType;

pub struct ProgramUnit<'a> {
    pub namespace: Namespace<'a>,
    pub imports: Vec<Spanned<'a, Import<'a>>>,
    pub top_level_statements: Vec<Spanned<'a, TopLevelStatement<'a>>>,
}

pub struct Namespace<'a> {
    pub namespace: Spanned<'a, String>,
}

pub struct Import<'a> {
    pub filename: Spanned<'a, String>,
}

pub enum TopLevelStatement<'a> {
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
}

pub struct FunctionDeclaration<'a> {
    pub name: Spanned<'a, String>,
    pub params: Vec<FunctionParam<'a>>,
    pub return_type: Type<'a>,
}

pub struct FunctionParam<'a> {
    pub name: Spanned<'a, String>,
    pub ty: Type<'a>,
}

pub enum Type<'a> {
    PrimaryType(Spanned<'a, PrimaryType>),
    ArrayType(Spanned<'a, Box<Type<'a>>>),
    PointerType(Spanned<'a, Box<Type<'a>>>),
}

pub enum Expression {}
