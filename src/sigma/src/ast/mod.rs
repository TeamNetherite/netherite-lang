//! ast/mod.rs - defines AST nodes and additional stuff.
pub mod location;
pub mod token;

use crate::ast::location::WithSpan;
use crate::ast::token::PrimaryType;

/// Represents Sigma source file.
#[derive(Debug, PartialEq)]
pub struct ProgramUnit<'a> {
    /// Namespace declaration
    ///
    /// ```sigma
    /// namespace test::test2;
    ///           ^^^^^^^^^^^ Span
    /// ```
    pub namespace: WithSpan<'a, String>,

    /// Represents list of imports.
    ///
    /// ```sigma
    /// import "test";
    /// ^^^^^^^^^^^^^^ Span of 1st
    /// import "test2";
    /// ^^^^^^^^^^^^^^^ Span of 2st
    /// ```
    pub imports: Vec<WithSpan<'a, Import<'a>>>,

    /// List of top level statements.
    ///
    /// ```sigma
    /// pub fun main() {}
    /// ^^^^^^^^^^^^^^^^^ Span
    /// ```
    pub top_level_statements: Vec<WithSpan<'a, TopLevelStatement<'a>>>,
}

/// Represents import declaration.
#[derive(Debug, PartialEq)]
pub struct Import<'a> {
    /// Name of file imported
    ///
    /// ```sigma
    /// import "test.sigma";
    ///        ^^^^^^^^^^^^ Span
    /// ```
    pub filename: WithSpan<'a, String>,
}

/// Represents top level statement. Ex: `FunctionDeclaration`,
/// `StructDeclaration`, `StructImplementation`.
#[derive(Debug, PartialEq)]
pub enum TopLevelStatement<'a> {
    FunctionDeclaration(FunctionDeclaration<'a>),
    StructDeclaration(StructDeclaration<'a>),
    StructImplementation(StructImplementation<'a>),
}

/// Function declaration top level statement
#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'a> {
    /// Is function public or not.
    pub public: bool,

    /// Name of the function
    ///
    /// ```sigma
    /// pub fun factorial(n i64) { ... }
    ///         ^^^^^^^^^ Span
    /// ```
    pub name: WithSpan<'a, String>,

    /// Function params.
    ///
    /// ```sigma
    /// pub fun div(a f64, b f64) {}
    ///             ^^^^^ Span of 1st param
    ///                    ^^^^^ Span of 2st param
    /// ```
    pub params: Vec<WithSpan<'a, FunctionParam<'a>>>,

    /// Return type (can be None, because there are no `void` types in Sigma).
    pub return_type: Option<Type<'a>>,
}

/// Structure declaration top level statement.
#[derive(Debug, PartialEq)]
pub struct StructDeclaration<'a> {
    /// Is struct public or not.
    pub public: bool,

    /// Name of the struct:
    ///
    /// ```sigma
    /// pub struct Person { ... }
    ///            ^^^^^^ Span
    /// ```
    pub name: WithSpan<'a, String>,

    /// Struct members:
    ///
    /// ```sigma
    /// pub struct Point {
    ///     x f64;
    ///     ^^^^^^ Span of 1st struct member definition
    ///     y f64;
    ///     ^^^^^^ Span of 2st struct member definition
    /// }
    /// ```
    pub members: Vec<WithSpan<'a, StructMemberDefinition<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct StructImplementation<'a> {
    pub interface_name: Option<WithSpan<'a, String>>,
    pub methods: Vec<FunctionDeclaration<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct StructMemberDefinition<'a> {
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
