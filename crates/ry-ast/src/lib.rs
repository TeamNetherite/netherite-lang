//! `lib.rs` - defines AST nodes and additional stuff.
pub mod location;
pub mod precedence;
pub mod token;

use std::collections::HashMap;

use location::{Span, WithSpan};
use token::{PrimaryType, Token};

/// Represents Ry source file.
#[derive(Debug, PartialEq)]
pub struct ProgramUnit {
    pub docstring: String,
    pub imports: Vec<Import>,
    pub top_level_statements: Vec<(TopLevelStatement, String)>,
}

/// Represents import declaration.
#[derive(Debug, PartialEq)]
pub struct Import {
    pub filename: WithSpan<String>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement {
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    InterfaceDecl(InterfaceDecl),
    Impl(Impl),
    EnumDecl(EnumDecl),
}

/// Function declaration top level statement
#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub def: FunctionDef,
    pub stmts: Vec<Statement>,
}

pub type GenericAnnotation = (WithSpan<String>, Option<Type>);
pub type GenericAnnotations = Vec<GenericAnnotation>;

/// Function definition statement
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<Type>,
}

#[derive(Debug, PartialEq)]
pub struct StructDecl {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub members: Vec<StructMemberDef>,
}

#[derive(Debug, PartialEq)]
pub struct Impl {
    pub for_what: (WithSpan<String>, GenericAnnotations),
    pub impl_what: Vec<(WithSpan<String>, WithSpan<String>)>,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceDecl {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub methods: Vec<InterfaceMethodDef>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceMethodDef {
    pub name: WithSpan<String>,
    pub generic_annotations: GenericAnnotations,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<Type>,
}

#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub variants: Vec<WithSpan<String>>,
}

#[derive(Debug, PartialEq)]
pub struct StructMemberDef {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub ty: Type,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam {
    pub name: WithSpan<String>,
    pub ty: Type,
    pub default_value: Option<Expression>,
}

pub type Type = WithSpan<Box<RawType>>;

#[derive(Debug, PartialEq)]
pub enum RawType {
    Primary(WithSpan<PrimaryType>),
    Array(Type),
    Pointer(Type),
    Custom(WithSpan<String>, Vec<Type>),
    Generic(WithSpan<String>),
    Impls(Type),
    Option(Type),
}

pub type StatementsBlock = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    ExpressionWithoutSemicolon(Expression),
    Return(Expression),
    Defer(Expression),
}

impl Statement {
    pub fn expression(self) -> Option<Expression> {
        match self {
            Self::Expression(e) => Some(e),
            _ => None,
        }
    }
}

pub type Expression = WithSpan<Box<RawExpression>>;

#[derive(Debug, PartialEq)]
pub enum RawExpression {
    String(String),
    Int(u64),
    Float(f64),
    Imag(f64),
    Bool(bool),
    StaticName(String),
    List(Vec<Expression>),
    Binary(Expression, Token, Expression),
    PrefixOrPostfix(Token, Expression),
    Property(Expression, WithSpan<String>),
    Struct(
        WithSpan<String>,
        HashMap<String, (Span, WithSpan<Expression>)>,
    ),
    Map(HashMap<String, (Span, WithSpan<Expression>)>),
    Call(Vec<Type>, Expression, Vec<Expression>),
    Index(Expression, Expression),
    If(
        (Expression, Vec<Statement>),
        Vec<(Expression, Vec<Statement>)>,
        Option<Vec<Statement>>,
    ),
}

impl RawExpression {
    pub fn must_have_semicolon_at_the_end(&self) -> bool {
        match self {
            RawExpression::If(_, _, _) => false,
            _ => true,
        }
    }
}
