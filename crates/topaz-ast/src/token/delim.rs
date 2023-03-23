use std::default::default;
use crate::{_Tokens, Tokens};
use crate::token::default_token_struct;
use crate::util::unit_impl;

trait _Delim {}

pub trait Delim: _Delim {}
impl<T: _Delim> Delim for T {}
impl<T: _Delim> _Tokens for T {}

/// The `(` / `)` delimiter
default_token_struct!(Parentheses);

/// The `{` / `}` delimiter
default_token_struct!(Curly);

/// The `<` / `>` delimiter
default_token_struct!(AngleBracket);

unit_impl!(_Delim [Parentheses, Curly, AngleBracket]);

pub struct Surround<D: Delim, Content: Tokens> {
    delim_start: D,
    content: Content,
    delim_end: D
}

impl<D: Delim, Content: Tokens> Surround<D, Content> {
    pub fn new(delim_start: D, content: Content, delim_end: D) -> Self {
        Self {
            delim_start,
            content,
            delim_end
        }
    }

    pub fn delimiters(&self) -> (&D, &D) {
        (&self.delim_start, &self.delim_end)
    }
    pub fn tuple(&self) -> (&D, &Content, &D) {
        (&self.delim_start, &self.content, &self.delim_end)
    }
    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn into_delimiters(self) -> (D, D) {
        (self.delim_start, self.delim_end)
    }
    pub fn into_tuple(self) -> (D, Content, D) {
        (self.delim_start, self.content, self.delim_end)
    }
    pub fn into_content(self) -> Content {
        self.content
    }
}

impl<D: Delim + Default, Content: Tokens> Surround<D, Content> {
    pub fn new_default(content: Content) -> Self {
        Self {
            delim_start: default(),
            content,
            delim_end: default()
        }
    }
}

impl<D: Delim + Default, Content: Tokens + Default> Default for Surround<D, Content> {
    fn default() -> Self {
        Self::new_default(default())
    }
}
