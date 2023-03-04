//! `location.rs` - Defines the Span struct for storing source
//! Locations throughout the compiler. Most notably, these locations
//! are passed around throughout the parser and are stored in each
//! AST node.
use std::ops::Range;

/// Represents code block location in source text.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Span {
    pub range: Range<usize>,
}

impl Span {
    pub fn from(range: Range<usize>) -> Self {
        Self { range }
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self { range: start..end }
    }

    pub fn from_location(location: usize, character_len: usize) -> Self {
        Self {
            range: location..location + character_len,
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(val: Range<usize>) -> Self {
        Span::from(val)
    }
}

/// Represents thing located in some [`Span`].
#[derive(Debug, PartialEq, Clone, Default)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T> From<(T, Span)> for WithSpan<T> {
    fn from(val: (T, Span)) -> Self {
        WithSpan::new(val.0, val.1)
    }
}
