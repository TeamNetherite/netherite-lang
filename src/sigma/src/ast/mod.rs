pub mod location;
pub mod token;

use crate::ast::location::WithSpan;
use crate::ast::token::PrimaryType;

#[derive(Debug, PartialEq)]
pub struct ProgramUnit<'a> {
    pub namespace: WithSpan<'a, String>,
    pub imports: Vec<WithSpan<'a, Import<'a>>>,
    pub top_level_statements: Vec<WithSpan<'a, TopLevelStatement<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct Import<'a> {
    pub filename: WithSpan<'a, String>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement<'a> {
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
    StructDeclaration(Box<StructDeclaration<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'a> {
    pub public: bool,
    pub name: WithSpan<'a, String>,
    pub params: Vec<WithSpan<'a, FunctionParam<'a>>>,
    pub return_type: Option<Type<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct StructDeclaration<'a> {
    pub public: bool,
    pub name: WithSpan<'a, String>,
    pub members: Vec<WithSpan<'a, StructMember<'a>>>,
    pub methods: Vec<FunctionDeclaration<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct StructMember<'a> {
    pub public: bool,
    pub mutable: bool,
    pub name: WithSpan<'a, String>,
    pub ty: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam<'a> {
    pub name: WithSpan<'a, String>,
    pub ty: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    PrimaryType(WithSpan<'a, PrimaryType>),
    ArrayType(WithSpan<'a, Box<Type<'a>>>),
    PointerType(WithSpan<'a, Box<Type<'a>>>),
    CustomType(WithSpan<'a, String>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {}
