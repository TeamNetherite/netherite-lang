#![feature(decl_macro)]
#![feature(default_free_fn)]
#![feature(is_some_and)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![deny(deprecated)]
#![allow(unused_doc_comments)]
#![allow(clippy::module_name_repetitions)]
//! `lib.rs` - defines AST nodes and additional stuff.
pub mod location;
pub mod precedence;
pub mod tokens;

#[macro_use]
extern crate topaz_macro;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use string_interner::backend::BufferBackend;
use string_interner::StringInterner;

static mut INTERNER: Lazy<StringInterner<BufferBackend>> = Lazy::new(StringInterner::<BufferBackend>::new);

use crate::tokens::RawToken;
use crate::util::unit_impl;
use location::{Span, WithSpan};
use tokens::Token as SpannedToken;


#[macro_use]
pub mod token;

pub mod block;
pub mod expr;
pub mod file;
pub mod ident;
pub mod item;
pub mod literal;
pub mod parser;
pub mod path;
pub mod pattern;
pub mod punctuated;
pub mod statement;
pub mod types;
pub mod util;
pub mod visibility;
pub mod visit;

pub use token::Token;

pub(crate) mod private {
    pub trait _Tokens {}
}

pub trait Tokens: private::_Tokens {}

impl<T: private::_Tokens> Tokens for T {}

impl<T: Tokens> private::_Tokens for Vec<T> {}

unit_impl!(crate::private::_Tokens [char, String, i32, u8]);

/// Represents a Topaz source file.
#[derive(Debug, PartialEq)]
pub struct ProgramUnit {
    /// Global source file docstring
    pub docstring: String,

    pub imports: Vec<Import>,
    pub top_level_statements: Vec<(String, TopLevelStatement)>,
}

/// Import
///
/// ```tp
/// import std::io;
///        ------- `path`
/// ```
#[derive(Debug, PartialEq)]
pub struct Import {
    pub path: WithSpan<String>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement {
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    TraitDecl(TraitDecl),
    Impl(Impl),
    EnumDecl(EnumDecl),

    // just for better errors
    Import(Import),
}

/// Function declaration top level statement
///
/// ```tp
/// 1 | func print_sum<T: Number>(a: T, b: T): T
///   | ----------------------------------- `def`
/// 2 | {
///   |   ...
///   |   --- `stmts`
/// 7 | }
/// ```
#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub def: FunctionDef,
    pub stmts: Vec<Statement>,
}

pub type GenericAnnotation = (WithSpan<String>, Option<Type>);
pub type GenericAnnotations = Vec<GenericAnnotation>;

/// Function definition
///
/// ```tp
/// public func test<T: Number, M, A>(a: T, b: T) -> T
/// ---     ---- --------------- --------  - `return_type`
/// |          | |                      |
/// `public`   | `generic_annotations`  |
///        `name`                      `params`
/// ```
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<(RawToken, Type)>,
}

/// Struct declaration top level statement
///
/// ```ry
/// 1 | pub struct Test<B, C> {
///   | ---        ---- ----
///   | |          |       |
///   | `public`   |    `generic_annotations`
///   |            `name`
/// 2 |   /// documentation for the 1st member
///   |   ------------------------------------ `members.0.0`
/// 3 |   a B;
///   |   ---- `members.0.1`
/// 4 |
/// 5 |   ...
/// 6 | }
/// ```
#[derive(Debug, PartialEq)]
pub struct StructDecl {
    pub public: Option<Span>,
    pub generic_annotations: GenericAnnotations,
    pub name: WithSpan<String>,
    pub members: Vec<(String, StructMemberDef)>,
}

/// Trait implementation top level statement
///
/// ```ry
/// 1 | impl<A, B> Into<Tuple<A, B>> for Tuple<B, A> {
///   |     ------ -----------------     ----------- `type`
///   |     |                      |
///   |     |                 `trait`
///   |     `global_generic_annotations`
/// 2 |   ...
///   |   --- `methods`
/// 3 | }
/// ```
#[derive(Debug, PartialEq)]
pub struct Impl {
    pub public: Option<Span>,
    pub global_generic_annotations: GenericAnnotations,
    pub r#type: Type,
    pub r#trait: Option<Type>,
    pub methods: Vec<(String, TraitMethod)>,
}

/// Trait declaration top level statement
///
/// ```ry
/// 1 | pub trait Into<T> {
///   | ---       ---- - `generic_annotations`
///   | |            |
///   | `pub`    `name`
/// 2 |   ...
///   |   --- `methods`
/// 3 | }
/// ```
#[derive(Debug, PartialEq)]
pub struct TraitDecl {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub generic_annotations: GenericAnnotations,
    pub methods: Vec<(String, TraitMethod)>,
}

/// Trait method
///
/// ```ry
/// pub fun into<T>(self Self) T { ... }
/// ---     ---- -  ---------  -   --- `body`
/// |          | |          |  |
/// |          | |   `params` `return_type`
/// `public`   | `generic_annotations`
///        `name`
/// ```
#[derive(Debug, PartialEq)]
pub struct TraitMethod {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub generic_annotations: GenericAnnotations,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
}

/// Enum declaration top level statement
///
/// ```ry
/// 1 | pub enum Test {
///   | ---      ---- `name`
///   | |
///   | `public`
///   |
/// 2 |   Test1,
///   |   ----- `variants.0.1`
/// 3 |   /// Some funny documentation
///   |   ---------------------------- `variants.1.0`
/// 4 |   Test2,
///   |   ----- `variants.1.1`
/// 5 | }
/// ```
#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub variants: Vec<(String, WithSpan<String>)>,
}

/// ```ry
/// pub a [i32];
/// --- - ----- `type`
/// |   |
/// |   `name`
/// `public`
/// ```
#[derive(Debug, PartialEq)]
pub struct StructMemberDef {
    pub public: Option<Span>,
    pub name: WithSpan<String>,
    pub r#type: Type,
}

/// ```tp
/// pub func test(a:  i32 = 0) {}
///              -^---^^^- function param
///              | |     |
///              | |     `default_value`
///              | `type`
///              `name`
/// ```
#[derive(Debug, PartialEq)]
pub struct FunctionParam {
    pub name: WithSpan<String>,
    pub colon: RawToken,
    pub r#type: Type,
    pub default_value: Option<Expression>,
}

pub type Type = WithSpan<Box<RawType>>;

#[derive(Debug, PartialEq)]
pub enum RawType {
    Array(Type),
    Pointer(Type),
    Primary(WithSpan<String>, Vec<Type>),
    Generic(WithSpan<String>),
    Option(Type),
}

pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    ExpressionWithoutSemicolon(Expression),
    Return(Expression),
    Defer(Expression),
    Let(
        Option<SpannedToken>,
        WithSpan<String>,
        Option<Type>,
        Expression,
    ),
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
    Char(char),
    StaticName(String),
    List(Vec<Expression>),
    Binary(Expression, SpannedToken, Expression),
    As(Expression, Type),
    PrefixOrPostfix(SpannedToken, Expression),
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
    While(Expression, Block),
}

impl RawExpression {
    pub fn must_have_semicolon_at_the_end(&self) -> bool {
        !matches!(
            self,
            RawExpression::If(_, _, _) | RawExpression::While(_, _)
        )
    }
}

pub trait WithSpannable {
    fn with_span(self, span: impl Into<Span>) -> WithSpan<Self>
    where
        Self: Sized;
}

impl<T: Sized> WithSpannable for T {
    fn with_span(self, span: impl Into<Span>) -> WithSpan<Self> {
        WithSpan::new(self, span.into())
    }
}
