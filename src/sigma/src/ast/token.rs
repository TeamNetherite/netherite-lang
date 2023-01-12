//! token.rs - defined the token which represents grammatical unit of Sigma
//! source text.

use crate::ast::location::Span;
use phf::phf_map;

use std::fmt::{self, Display};

/// Represents error that lexer can fail with
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedChar(char),
    UnterminatedWrappedIdentifierLiteral,
    UnterminatedStringLiteral,
    InvalidRadixPoint,
    HasNoDigits,
    ExponentRequiresDecimalMantissa,
    ExponentRequiresHexadecimalMantissa,
    ExponentHasNoDigits,
    HexadecimalMantissaRequiresPExponent,
    InvalidDigit,
    UnderscoreMustSeperateSuccessiveDigits,
}

/// Represents integer type, used in RawToken::Int.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,

    ISize,
    USize,
}

impl Display for IntegerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IntegerType::*;

        match self {
            I8 => write!(formatter, "i8"),
            I16 => write!(formatter, "i16"),
            I32 => write!(formatter, "i32"),
            I64 => write!(formatter, "i64"),
            ISize => write!(formatter, "isize"),
            U8 => write!(formatter, "u8"),
            U16 => write!(formatter, "u16"),
            U32 => write!(formatter, "u32"),
            U64 => write!(formatter, "u64"),
            USize => write!(formatter, "usize"),
        }
    }
}

/// Represents floating-point type, used in RawToken::Float.
#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum FloatType {
    F32,
    F64,
}

impl Display for FloatType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FloatType::*;

        match self {
            F32 => write!(formatter, "f32"),
            F64 => write!(formatter, "f64"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum NumberKind {
    Invalid,
    Int,
    Float,
    Imag,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RawToken {
    Identifier(String),
    String(String),
    Int(u64),
    Float(f64),
    Imag(f64),
    Char(char),
    Boolean(bool),

    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,

    IntType(IntegerType),
    FloatType(FloatType),

    GreaterThan,
    GreaterThanOrEq,
    LessThan,
    LessThanOrEq,
    Assign,
    Eq,
    NotEq,

    RightShift,
    LeftShift,
    Or,
    And,
    Xor,
    Not,

    OrOr,
    AndAnd,

    PlusEq,
    MinusEq,
    AsteriskEq,
    SlashEq,
    XorEq,
    OrEq,

    OpenParent,
    CloseParent,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    Comma,
    Dot,
    Semicolon,

    PlusPlus,
    MinusMinus,

    Comment(String),

    EndOfFile,
    Invalid(LexerError),
}

impl Display for RawToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RawToken::*;

        match self {
            Identifier(_) => write!(formatter, "identifier"),
            String(_) => write!(formatter, "string literal"),
            Int(_) => write!(formatter, "integer literal"),
            Float(_) => write!(formatter, "float literal"),
            Imag(_) => write!(formatter, "imaginary number literal"),
            Char(_) => write!(formatter, "character literal"),
            Boolean(_) => write!(formatter, "boolean literal"),
            Plus => write!(formatter, "'+'"),
            Minus => write!(formatter, "'-'"),
            Asterisk => write!(formatter, "'*'"),
            Slash => write!(formatter, "'/'"),
            Bang => write!(formatter, "'!'"),
            GreaterThan => write!(formatter, "'>'"),
            GreaterThanOrEq => write!(formatter, "'>='"),
            LessThan => write!(formatter, "'<'"),
            LessThanOrEq => write!(formatter, "'<='"),
            Assign => write!(formatter, "'='"),
            Eq => write!(formatter, "'=='"),
            NotEq => write!(formatter, "'!='"),
            RightShift => write!(formatter, "'>>'"),
            LeftShift => write!(formatter, "'<<'"),
            Or => write!(formatter, "'|'"),
            And => write!(formatter, "'&'"),
            Xor => write!(formatter, "'^'"),
            Not => write!(formatter, "'~'"),
            OrOr => write!(formatter, "'||'"),
            AndAnd => write!(formatter, "'&&'"),
            PlusEq => write!(formatter, "'+='"),
            MinusEq => write!(formatter, "'-='"),
            AsteriskEq => write!(formatter, "'*='"),
            SlashEq => write!(formatter, "'/='"),
            XorEq => write!(formatter, "'^='"),
            OrEq => write!(formatter, "'|='"),
            OpenParent => write!(formatter, "'('"),
            CloseParent => write!(formatter, "')'"),
            OpenBracket => write!(formatter, "'['"),
            CloseBracket => write!(formatter, "']'"),
            OpenBrace => write!(formatter, "'{{'"),
            CloseBrace => write!(formatter, "'}}'"),
            Comma => write!(formatter, "','"),
            Dot => write!(formatter, "'.'"),
            Semicolon => write!(formatter, "';'"),
            PlusPlus => write!(formatter, "'++'"),
            MinusMinus => write!(formatter, "'--'"),
            IntType(i) => write!(formatter, "'{}'", i),
            FloatType(f) => write!(formatter, "'{}'", f),
            Comment(_) => write!(formatter, "comment"),
            EndOfFile => write!(formatter, "end of file"),
            Invalid(_) => write!(formatter, "invalid"),
        }
    }
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
    "i8" => RawToken::IntType(IntegerType::I8),
    "i16" => RawToken::IntType(IntegerType::I16),
    "i32" => RawToken::IntType(IntegerType::I32),
    "i64" => RawToken::IntType(IntegerType::I64),
    "isize" => RawToken::IntType(IntegerType::ISize),
    "u8" => RawToken::IntType(IntegerType::U8),
    "u16" => RawToken::IntType(IntegerType::U16),
    "u32" => RawToken::IntType(IntegerType::U32),
    "u64" => RawToken::IntType(IntegerType::U64),
    "usize" => RawToken::IntType(IntegerType::USize),
    "true" => RawToken::Boolean(true),
    "false" => RawToken::Boolean(false),
    "f32" => RawToken::FloatType(FloatType::F32),
    "f64" => RawToken::FloatType(FloatType::F64),
};
