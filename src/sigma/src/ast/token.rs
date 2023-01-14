//! token.rs - defined the token which represents grammatical unit of Sigma
//! source text.

use crate::ast::location::Span;
use derive_more::Display;
use phf::phf_map;

use std::fmt::{self, Display};

/// Represents error that lexer can fail with
#[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
pub enum LexerError {
    #[display(fmt = "unexpected character '{_0}'")]
    UnexpectedChar(char),
    #[display(fmt = "unterminated wrapped identifier literal")]
    UnterminatedWrappedIdentifierLiteral,
    #[display(fmt = "unterminated string literal")]
    UnterminatedStringLiteral,
    #[display(fmt = "invalid radix point")]
    InvalidRadixPoint,
    #[display(fmt = "has no digits")]
    HasNoDigits,
    #[display(fmt = "exponent requires decimal mantissa")]
    ExponentRequiresDecimalMantissa,
    #[display(fmt = "exponent has no digits")]
    ExponentHasNoDigits,
    #[display(fmt = "invalid digit")]
    InvalidDigit,
    #[display(fmt = "underscore must seperate successive digits")]
    UnderscoreMustSeperateSuccessiveDigits,
}

/// Represents integer and float types, used in RawToken::Int.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
pub enum PrimaryType {
    #[display(fmt = "i8")]
    I8,
    #[display(fmt = "i16")]
    I16,
    #[display(fmt = "i32")]
    I32,
    #[display(fmt = "i64")]
    I64,
    #[display(fmt = "u8")]
    U8,
    #[display(fmt = "u16")]
    U16,
    #[display(fmt = "u32")]
    U32,
    #[display(fmt = "u64")]
    U64,

    #[display(fmt = "isize")]
    ISize,
    #[display(fmt = "usize")]
    USize,

    #[display(fmt = "f32")]
    F32,
    #[display(fmt = "f64")]
    F64,

    #[display(fmt = "complex")]
    Complex,
}

#[derive(PartialEq, Debug)]
pub enum NumberKind {
    Invalid,
    Int,
    Float,
    Imag,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum RawToken {
    #[display(fmt = "identifier")]
    Identifier(String),
    #[display(fmt = "string literal")]
    String(String),
    #[display(fmt = "integer literal")]
    Int(u64),
    #[display(fmt = "float literal")]
    Float(f64),
    #[display(fmt = "imaginary number literal")]
    Imag(f64),
    #[display(fmt = "character literal")]
    Char(char),
    #[display(fmt = "boolean literal")]
    Boolean(bool),

    #[display(fmt = "'+'")]
    Plus,
    #[display(fmt = "'-'")]
    Minus,
    #[display(fmt = "'*'")]
    Asterisk,
    #[display(fmt = "'/'")]
    Slash,
    #[display(fmt = "'!'")]
    Bang,

    #[display(fmt = "{_0}")]
    PrimaryType(PrimaryType),

    #[display(fmt = "'>'")]
    GreaterThan,
    #[display(fmt = "'>='")]
    GreaterThanOrEq,
    #[display(fmt = "'<'")]
    LessThan,
    #[display(fmt = "'<='")]
    LessThanOrEq,
    #[display(fmt = "'='")]
    Assign,
    #[display(fmt = "'=='")]
    Eq,
    #[display(fmt = "'!='")]
    NotEq,

    #[display(fmt = "'>>'")]
    RightShift,
    #[display(fmt = "'<<'")]
    LeftShift,
    #[display(fmt = "'|'")]
    Or,
    #[display(fmt = "'&'")]
    And,
    #[display(fmt = "'^'")]
    Xor,
    #[display(fmt = "'~'")]
    Not,

    #[display(fmt = "'||'")]
    OrOr,
    #[display(fmt = "'&&'")]
    AndAnd,

    #[display(fmt = "'+='")]
    PlusEq,
    #[display(fmt = "'-='")]
    MinusEq,
    #[display(fmt = "'*='")]
    AsteriskEq,
    #[display(fmt = "'/='")]
    SlashEq,
    #[display(fmt = "'^='")]
    XorEq,
    #[display(fmt = "'|='")]
    OrEq,

    #[display(fmt = "'('")]
    OpenParent,
    #[display(fmt = "')'")]
    CloseParent,
    #[display(fmt = "'['")]
    OpenBracket,
    #[display(fmt = "']'")]
    CloseBracket,
    #[display(fmt = "'{{'")]
    OpenBrace,
    #[display(fmt = "'}}'")]
    CloseBrace,

    #[display(fmt = "','")]
    Comma,
    #[display(fmt = "'.'")]
    Dot,
    #[display(fmt = "';'")]
    Semicolon,
    #[display(fmt = "':'")]
    Colon,

    #[display(fmt = "'++'")]
    PlusPlus,
    #[display(fmt = "'--'")]
    MinusMinus,

    #[display(fmt = "namespace")]
    Namespace,
    #[display(fmt = "import")]
    Import,
    #[display(fmt = "pub")]
    Pub,
    #[display(fmt = "fun")]
    Fun,
    #[display(fmt = "struct")]
    Struct,
    #[display(fmt = "mut")]
    Mut,

    #[display(fmt = "comment")]
    Comment(String),

    #[display(fmt = "end of file")]
    EndOfFile,

    #[display(fmt = "invalid token")]
    Invalid(LexerError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub raw: RawToken,
    pub span: Span<'a>,
}

impl<'a> Token<'a> {
    pub fn new(raw: RawToken, span: Span<'a>) -> Self {
        Self { raw, span }
    }
}

pub static RESERVED: phf::Map<&'static str, RawToken> = phf_map! {
    "i8" => RawToken::PrimaryType(PrimaryType::I8),
    "i16" => RawToken::PrimaryType(PrimaryType::I16),
    "i32" => RawToken::PrimaryType(PrimaryType::I32),
    "i64" => RawToken::PrimaryType(PrimaryType::I64),
    "isize" => RawToken::PrimaryType(PrimaryType::ISize),
    "u8" => RawToken::PrimaryType(PrimaryType::U8),
    "u16" => RawToken::PrimaryType(PrimaryType::U16),
    "u32" => RawToken::PrimaryType(PrimaryType::U32),
    "u64" => RawToken::PrimaryType(PrimaryType::U64),
    "usize" => RawToken::PrimaryType(PrimaryType::USize),
    "f32" => RawToken::PrimaryType(PrimaryType::F32),
    "f64" => RawToken::PrimaryType(PrimaryType::F64),
    "true" => RawToken::Boolean(true),
    "false" => RawToken::Boolean(false),
    "namespace" => RawToken::Namespace,
    "import" => RawToken::Import,
    "pub" => RawToken::Pub,
    "fun" => RawToken::Fun,
    "struct" => RawToken::Struct,
    "mut" => RawToken::Mut,
};
