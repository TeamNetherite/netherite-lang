pub mod location;
pub mod token;

use crate::ast::location::Spanned;
use crate::ast::token::PrimaryType;

#[derive(Debug, PartialEq)]
pub struct ProgramUnit<'a> {
    pub namespace: Namespace<'a>,
    pub imports: Vec<Box<Spanned<'a, Import<'a>>>>,
    pub top_level_statements: Vec<Box<Spanned<'a, TopLevelStatement<'a>>>>,
}

#[derive(Debug, PartialEq)]
pub struct Namespace<'a> {
    pub namespace: Box<Spanned<'a, String>>,
}

#[derive(Debug, PartialEq)]
pub struct Import<'a> {
    pub filename: Box<Spanned<'a, String>>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement<'a> {
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'a> {
    pub name: Box<Spanned<'a, String>>,
    pub params: Vec<FunctionParam<'a>>,
    pub return_type: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam<'a> {
    pub name: Box<Spanned<'a, String>>,
    pub ty: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    PrimaryType(Box<Spanned<'a, PrimaryType>>),
    ArrayType(Box<Spanned<'a, Box<Type<'a>>>>),
    PointerType(Box<Spanned<'a, Box<Type<'a>>>>),
    CustomType(Box<Spanned<'a, String>>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {}
