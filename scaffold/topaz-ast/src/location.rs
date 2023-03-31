//! `location.rs` - Defines the Span struct for storing source
//! Locations throughout the compiler. Most notably, these locations
//! are passed around throughout the parser and are stored in each
//! AST node.
use std::fmt::{Display, Formatter};
use std::ops::Range;

use derive_more::Display;

/// Represents code block location in source text.
#[derive(Clone, Debug, PartialEq, Default, Copy, Display, Eq)]
#[display(fmt = "{start}..{end}")]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use] pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use] pub const fn from_location(location: usize, character_len: usize) -> Self {
        Self {
            start: location,
            end: location + character_len,
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(val: Range<usize>) -> Self {
        Self::new(val.start, val.end)
    }
}

/// Represents thing located in some [`Span`].
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

impl<T: Display> Display for WithSpan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` @ {}", self.value, self.span)
    }
}

impl<T> WithSpan<T> {
    pub const fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T> From<(T, Span)> for WithSpan<T> {
    fn from(val: (T, Span)) -> Self {
        Self::new(val.0, val.1)
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start..value.end
    }
}
