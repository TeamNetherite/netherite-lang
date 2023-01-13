pub mod location;
pub mod token;

use crate::ast::location::Spanned;
use crate::ast::token::PrimaryType;

#[derive(Debug)]
pub struct ProgramUnit<'a> {
    pub namespace: Namespace<'a>,
    pub imports: Vec<Spanned<'a, Import<'a>>>,
    pub top_level_statements: Vec<Spanned<'a, TopLevelStatement<'a>>>,
}

#[derive(Debug)]
pub struct Namespace<'a> {
    pub namespace: Spanned<'a, String>,
}

#[derive(Debug)]
pub struct Import<'a> {
    pub filename: Spanned<'a, String>,
}

#[derive(Debug)]
pub enum TopLevelStatement<'a> {
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
}

#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    pub name: Spanned<'a, String>,
    pub params: Vec<FunctionParam<'a>>,
    pub return_type: Type<'a>,
}

#[derive(Debug)]
pub struct FunctionParam<'a> {
    pub name: Spanned<'a, String>,
    pub ty: Type<'a>,
}

#[derive(Debug)]
pub enum Type<'a> {
    PrimaryType(Spanned<'a, PrimaryType>),
    ArrayType(Spanned<'a, Box<Type<'a>>>),
    PointerType(Spanned<'a, Box<Type<'a>>>),
}

#[derive(Debug)]
pub enum Expression {}
