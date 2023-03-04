//! `token.rs` - defines the token which represents grammatical unit of Ry
//! source text.

use std::mem::discriminant;

use derive_more::Display;
use phf::phf_map;

use num_traits::ToPrimitive;

use crate::location::WithSpan;
use crate::precedence::Precedence;

/// Represents error that lexer can fail with.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
pub enum LexerError {
    #[display(fmt = "unexpected character `{_0}`")]
    UnexpectedChar(char),
    #[display(fmt = "unterminated wrapped identifier literal")]
    UnterminatedWrappedIdentifierLiteral,
    #[display(fmt = "empty wrapped identifier literal")]
    EmptyWrappedIdentifierLiteral,
    #[display(fmt = "unterminated string literal")]
    UnterminatedStringLiteral,
    #[display(fmt = "unknown escape sequence")]
    UnknownEscapeSequence,
    #[display(fmt = "empty escape sequence")]
    EmptyEscapeSequence,
    #[display(fmt = "expected closing bracket (`}}`) in unicode escape sequence")]
    ExpectedCloseBracketInUnicodeEscapeSequence,
    #[display(fmt = "expected opening bracket (`{{`) in unicode escape sequence")]
    ExpectedOpenBracketInUnicodeEscapeSequence,
    #[display(fmt = "expected hexadecimal digit in unicode escape sequence")]
    ExpectedDigitInUnicodeEscapeSequence,
    #[display(fmt = "such unicode character does not exists")]
    InvalidUnicodeEscapeSequence,
    #[display(fmt = "expected closing bracket (`}}`) in byte escape sequence")]
    ExpectedCloseBracketInByteEscapeSequence,
    #[display(fmt = "expected opening bracket (`{{`) in byte escape sequence")]
    ExpectedOpenBracketInByteEscapeSequence,
    #[display(fmt = "expected hexadecimal digit in byte escape sequence")]
    ExpectedDigitInByteEscapeSequence,
    #[display(fmt = "such byte does not exists")]
    InvalidByteEscapeSequence,
    #[display(fmt = "invalid radix point")]
    InvalidRadixPoint,
    #[display(fmt = "has no digits")]
    HasNoDigits,
    #[display(fmt = "exponent requires decimal mantissa")]
    ExponentRequiresDecimalMantissa,
    #[display(fmt = "exponent has no digits")]
    ExponentHasNoDigits,
    #[display(fmt = "digit doesn't correspond to the base")]
    InvalidDigit,
    #[display(fmt = "number parsing error (overflow is possible)")]
    NumberParserError,
    #[display(fmt = "underscore must seperate successive digits")]
    UnderscoreMustSeperateSuccessiveDigits,
}

/// Represents low level types, which other types are constructed with. Examples: `i8`, `i16`, `i32`, `u16`.
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

/// Wether the number is integer, float or imaginary literal.
#[derive(PartialEq, Debug)]
pub enum NumberKind {
    Invalid,
    Int,
    Float,
    Imag,
}

/// Represents token without a specific location in source text.
#[derive(Clone, Debug, PartialEq, Display, Default)]
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
    Bool(bool),

    #[default]
    #[display(fmt = "`+`")]
    Plus,
    #[display(fmt = "`-`")]
    Minus,
    #[display(fmt = "`*`")]
    Asterisk,
    #[display(fmt = "`/`")]
    Slash,
    #[display(fmt = "`!`")]
    Bang,
    #[display(fmt = "`!!`")]
    BangBang,

    #[display(fmt = "{_0}")]
    PrimaryType(PrimaryType),

    #[display(fmt = "`import`")]
    Import,
    #[display(fmt = "`pub`")]
    Pub,
    #[display(fmt = "`fun`")]
    Fun,
    #[display(fmt = "`struct`")]
    Struct,
    #[display(fmt = "`implement`")]
    Implement,
    #[display(fmt = "`interface`")]
    Interface,
    #[display(fmt = "`return`")]
    Return,
    #[display(fmt = "`defer`")]
    Defer,
    #[display(fmt = "`impl`")]
    Impl,
    #[display(fmt = "`impls`")]
    Impls,
    #[display(fmt = "`enum`")]
    Enum,
    #[display(fmt = "`if`")]
    If,
    #[display(fmt = "`else`")]
    Else,
    #[display(fmt = "`while`")]
    While,
    #[display(fmt = "`var`")]
    Var,

    #[display(fmt = "`?`")]
    QuestionMark,

    #[display(fmt = "`>`")]
    GreaterThan,
    #[display(fmt = "`>=`")]
    GreaterThanOrEq,
    #[display(fmt = "`<`")]
    LessThan,
    #[display(fmt = "`<=`")]
    LessThanOrEq,
    #[display(fmt = "`=`")]
    Assign,
    #[display(fmt = "`==`")]
    Eq,
    #[display(fmt = "`!=`")]
    NotEq,

    #[display(fmt = "`>>`")]
    RightShift,
    #[display(fmt = "`<<`")]
    LeftShift,
    #[display(fmt = "`|`")]
    Or,
    #[display(fmt = "`&`")]
    And,
    #[display(fmt = "`^`")]
    Xor,
    #[display(fmt = "`~`")]
    Not,

    #[display(fmt = "`||`")]
    OrOr,
    #[display(fmt = "`&&`")]
    AndAnd,

    #[display(fmt = "`$`")]
    Dollar,

    #[display(fmt = "`+=`")]
    PlusEq,
    #[display(fmt = "`-=`")]
    MinusEq,
    #[display(fmt = "`*=`")]
    AsteriskEq,
    #[display(fmt = "`/=`")]
    SlashEq,
    #[display(fmt = "`^=`")]
    XorEq,
    #[display(fmt = "`|=`")]
    OrEq,

    #[display(fmt = "`(`")]
    OpenParent,
    #[display(fmt = "`)`")]
    CloseParent,
    #[display(fmt = "`[`")]
    OpenBracket,
    #[display(fmt = "`]`")]
    CloseBracket,
    #[display(fmt = "`{{`")]
    OpenBrace,
    #[display(fmt = "`}}`")]
    CloseBrace,

    #[display(fmt = "`,`")]
    Comma,
    #[display(fmt = "`.`")]
    Dot,
    #[display(fmt = "`;`")]
    Semicolon,
    #[display(fmt = "`:`")]
    Colon,
    #[display(fmt = "`::`")]
    DoubleColon,

    #[display(fmt = "`++`")]
    PlusPlus,
    #[display(fmt = "`--`")]
    MinusMinus,
    #[display(fmt = "`**`")]
    AsteriskAsterisk,

    #[display(fmt = "`%`")]
    Percent,
    #[display(fmt = "`?:`")]
    Elvis,

    #[display(fmt = "`@`")]
    AtSign,

    #[display(fmt = "comment")]
    Comment(String),

    #[display(fmt = "end of file")]
    EndOfFile,

    #[display(fmt = "invalid token")]
    Invalid(LexerError),
}

impl RawToken {
    pub fn to_precedence(&self) -> i8 {
        match self {
            Self::Elvis => Precedence::Elvis,
            Self::OrOr => Precedence::OrOr,
            Self::AndAnd => Precedence::AndAnd,
            Self::Or => Precedence::Or,
            Self::Xor => Precedence::Xor,
            Self::And => Precedence::And,
            Self::Eq | Self::NotEq => Precedence::Eq,
            Self::Assign
            | Self::PlusEq
            | Self::MinusEq
            | Self::AsteriskEq
            | Self::SlashEq
            | Self::OrEq
            | Self::XorEq => Precedence::Assign,
            Self::LessThan | Self::LessThanOrEq | Self::GreaterThan | Self::GreaterThanOrEq => {
                Precedence::LessOrGreater
            }
            Self::Dollar => Precedence::Dollar,
            Self::LeftShift | Self::RightShift => Precedence::LeftRightShift,
            Self::Plus | Self::Minus => Precedence::Sum,
            Self::Asterisk | Self::Slash => Precedence::Product,
            Self::AsteriskAsterisk => Precedence::Power,
            Self::Percent => Precedence::Mod,
            Self::OpenParent => Precedence::Call,
            Self::OpenBracket | Self::Dot => Precedence::Index,
            Self::Not
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Bang
            | Self::QuestionMark
            | Self::BangBang => Precedence::PrefixOrPostfix,
            _ => Precedence::Lowest,
        }
        .to_i8()
        .unwrap()
    }

    pub fn ident(&self) -> Option<String> {
        if let RawToken::Identifier(i) = self {
            Some(i.into())
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<String> {
        if let RawToken::String(s) = &self {
            Some(s.into())
        } else {
            None
        }
    }

    pub fn bool(&self) -> Option<bool> {
        if let RawToken::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<u64> {
        if let RawToken::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn float(&self) -> Option<f64> {
        if let RawToken::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn imag(&self) -> Option<f64> {
        if let RawToken::Imag(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn is(&self, raw: &Self) -> bool {
        discriminant(self) == discriminant(raw)
    }
}

pub type Token = WithSpan<RawToken>;

/// List of reserved Ry names: keywords, boolean literals & etc..
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
    "true" => RawToken::Bool(true),
    "false" => RawToken::Bool(false),
    "import" => RawToken::Import,
    "pub" => RawToken::Pub,
    "fun" => RawToken::Fun,
    "struct" => RawToken::Struct,
    "implement" => RawToken::Implement,
    "interface" => RawToken::Interface,
    "return" => RawToken::Return,
    "defer" => RawToken::Defer,
    "impl" => RawToken::Impl,
    "enum" => RawToken::Enum,
    "if" => RawToken::If,
    "else" => RawToken::Else,
    "while" => RawToken::While,
    "var" => RawToken::Var,
};
