//! `lib.rs` - defines AST nodes and additional stuff.
pub mod location;
pub mod precedence;
pub mod token;

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

pub type GenericAnnotation = (WithSpan<String>, Option<WithSpan<Box<Type>>>);
pub type GenericAnnotations = Vec<GenericAnnotation>;

/// Function definition statement
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<WithSpan<Box<Type>>>,
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
    pub return_type: Option<WithSpan<Box<Type>>>,
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
    pub ty: WithSpan<Box<Type>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam {
    pub name: WithSpan<String>,
    pub ty: WithSpan<Box<Type>>,
    pub default_value: Option<WithSpan<Box<Expression>>>,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Primary(WithSpan<PrimaryType>),
    Array(WithSpan<Box<Type>>),
    Pointer(WithSpan<Box<Type>>),
    Custom(WithSpan<String>, Vec<WithSpan<Box<Type>>>),
    Generic(WithSpan<String>),
    Impls(WithSpan<Box<Type>>),
    Option(WithSpan<Box<Type>>),
}

pub type StatementsBlock = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(WithSpan<Box<Expression>>),
    Return(WithSpan<Box<Expression>>),
    LastReturn(WithSpan<Box<Expression>>),
    Defer(WithSpan<Box<Expression>>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    String(String),
    Int(u64),
    Float(f64),
    Imag(f64),
    Bool(bool),
    StaticName(String),
    List(Vec<WithSpan<Box<Expression>>>),
    Binary(WithSpan<Box<Expression>>, Token, WithSpan<Box<Expression>>),
    PrefixOrPostfix(Token, WithSpan<Box<Expression>>),
    Property(WithSpan<Box<Expression>>, WithSpan<String>),

    // TODO: implement parsing for this one
    // #[allow(dead_code)]
    // Struct(
    //     WithSpan<String>,
    //     HashMap<String, (Span, WithSpan<Expression>)>,
    // ),
    Call(
        Vec<WithSpan<Box<Type>>>,
        WithSpan<Box<Expression>>,
        Vec<WithSpan<Box<Expression>>>,
    ),
    Index(WithSpan<Box<Expression>>, WithSpan<Box<Expression>>),
}
