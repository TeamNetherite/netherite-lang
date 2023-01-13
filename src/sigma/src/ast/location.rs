use std::ops::Range;
use std::path::Path;

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

#[derive(Debug, PartialEq)]
pub struct Spanned<'a, T> {
    pub value: T,
    pub span: Span<'a>,
}

impl<'a, T> Spanned<'a, T> {
    pub fn new(value: T, span: Span<'a>) -> Box<Self> {
        Box::new(Self { value, span })
    }
}
