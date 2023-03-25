#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

pub mod error;

use crate::error::{LexError, LexResult};
use std::io::{BufReader, Cursor};
use std::str::Chars;
use topaz_ast::ident::Ident;
use topaz_ast::literal::{Literal, LiteralString};
use topaz_ast::token::delim::StringLit;
use topaz_ast::token::{SingleToken, TokenTree, EVERYTHING};
use topaz_ast::Token;

pub struct Lexer<'a> {
    start_location: u32,
    location: u32,
    current: char,
    next: char,
    str: &'a str,
    buffer: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();

        let current = chars.next().unwrap_or('\0');
        let next = chars.next().unwrap_or('\0');

        Self {
            current,
            next,
            str: input,
            buffer: chars,
            location: 0,
            start_location: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.current == '\0'
    }

    pub fn eol(&self) -> bool {
        self.eof() || self.current == '\n'
    }

    fn advance_whitespace(&mut self) {
        while self.current.is_whitespace() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        let previous = self.current;

        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');

        self.location += previous.len_utf8();
    }

    fn advance_while<F>(&mut self, mut f: F) -> &'a str
    where
        F: FnMut(char, char) -> bool,
    {
        while f(self.current, self.next) && !self.eof() {
            self.advance();
        }

        &self.str[self.start_location..self.location]
    }

    fn scan_str_lit(&mut self) -> LiteralString {
        self.advance();
        let str = self.advance_while(|current, _| current != '"');

        LiteralString(StringLit::new_default(str.to_string()))
    }

    fn scan_name(&mut self) -> Option<TokenTree> {
        if self.eof() {
            return None;
        }
        self.start_location = self.location;
        let name = self.advance_while(|current, _| current.is_alphanumeric() || current == '_');

        Some(match EVERYTHING.get(name) {
            Some(real) => TokenTree::Single(real.clone()),
            None => TokenTree::Identifier(Ident::new(name)),
        })
    }

    pub fn next(&mut self) -> LexResult<TokenTree> {
        match (self.current, self.next) {
            ('"', _) => Ok(TokenTree::Literal(Literal::String(self.scan_str_lit()))),
            (c, _) => self.scan_name().ok_or(LexError::BadCharacter(c)),
        }
    }
}
