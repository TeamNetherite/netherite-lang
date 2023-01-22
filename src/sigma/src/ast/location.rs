//! location.rs - Defines the Span struct for storing source
//! Locations throughout the compiler. Most notably, these locations
//! are passed around throughout the parser and are stored in each
//! AST node.
use std::ops::Range;

/// Represents code block location in source text.
#[derive(Clone, Debug, PartialEq)]
pub struct Span<'a> {
    pub filename: &'a str,
    pub range: Range<usize>,
}

impl<'a> Span<'a> {
    pub fn new(filename: &'a str, start: usize, end: usize) -> Self {
        Self {
            filename,
            range: start..end,
        }
    }

    pub fn from_location(filename: &'a str, location: usize, character_len: usize) -> Self {
        Self {
            filename,
            range: location..location + character_len,
        }
    }
}

/// Represents thing `x` located in some specific [`Span`] (code block location).
#[derive(Debug, PartialEq, Clone)]
pub struct WithSpan<'a, T> {
    pub value: T,
    pub span: Span<'a>,
}

impl<'a, T> WithSpan<'a, T> {
    pub fn new(value: T, span: Span<'a>) -> Self {
        Self { value, span }
    }
}
