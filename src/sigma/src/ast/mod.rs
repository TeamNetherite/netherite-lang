pub mod location;
pub mod token;

use crate::ast::location::WithSpan;
use crate::ast::token::PrimaryType;

#[derive(Debug, PartialEq)]
pub struct ProgramUnit<'a> {
    pub namespace: Box<Namespace<'a>>,
    pub imports: Vec<Box<WithSpan<'a, Import<'a>>>>,
    pub top_level_statements: Vec<Box<WithSpan<'a, TopLevelStatement<'a>>>>,
}

#[derive(Debug, PartialEq)]
pub struct Namespace<'a> {
    pub namespace: Box<WithSpan<'a, String>>,
}

#[derive(Debug, PartialEq)]
pub struct Import<'a> {
    pub filename: Box<WithSpan<'a, String>>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement<'a> {
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'a> {
    pub name: Box<WithSpan<'a, String>>,
    pub params: Vec<FunctionParam<'a>>,
    pub return_type: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam<'a> {
    pub name: Box<WithSpan<'a, String>>,
    pub ty: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    PrimaryType(Box<WithSpan<'a, PrimaryType>>),
    ArrayType(Box<WithSpan<'a, Box<Type<'a>>>>),
    PointerType(Box<WithSpan<'a, Box<Type<'a>>>>),
    CustomType(Box<WithSpan<'a, String>>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {}
