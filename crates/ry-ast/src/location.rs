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

impl Into<Span> for Range<usize> {
    fn into(self) -> Span {
        Span::from(self)
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

impl<T> Into<WithSpan<T>> for (T, Span) {
    fn into(self) -> WithSpan<T> {
        WithSpan::new(self.0, self.1)
    }
}
