#![deny(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

pub mod check;
pub mod error;
pub mod state;

use crate::error::{LexError, LexResult};
use state::LexState;
use std::io::{BufReader, Cursor};
use std::marker::PhantomData;
use std::str::Chars;
use topaz_ast::ident::Ident;
use topaz_ast::literal::{Literal, LiteralString};
use topaz_ast::token::delim::{CharLit, StringLit};
use topaz_ast::token::{
    Delimiters, Punctuation, SingleToken, TokenTree, DELIMITERS, EVERYTHING, KEYWORDS, PREFIXES,
    PUNCTUATIONS,
};
use topaz_ast::util::{apply_mut, Require};
use topaz_ast::Token;

pub struct Lexer<'a> {
    start_location: u32,
    location: u32,
    current: char,
    next: char,
    str: &'a str,
    buffer: Chars<'a>,
    state: LexState,
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
            state: LexState::WaitingItem,
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

    fn lookahead(&self, count: usize) -> Result<String, String> {
        let mut real = String::new();

        for i in 0..count {
            if let Some(c) = self.str.get(self.location + i) {
                real.push(*c);
            } else {
                return Err(real);
            }
        }

        Ok(real)
    }

    fn advance(&mut self) -> char {
        let previous = self.current;

        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');

        self.location += previous.len_utf8();

        self.current
    }

    fn advance_while<F>(&mut self, mut f: F) -> &'a str
    where
        F: FnMut(char, char) -> bool,
    {
        self.start_location = self.location;
        while f(self.current, self.next) && !self.eof() {
            self.advance();
        }

        &self.str[self.start_location..self.location]
    }

    #[inline]
    fn scan_kw(&mut self) -> LexResult<SingleToken> {
        let kw = self.advance_while(|a, _| a.is_ascii_alphabetic());
        EVERYTHING
            .get(kw)
            .map(|a| *a)
            .ok_or_else(LexError::ExpectedItem(kw.to_owned()))
    }

    #[inline]
    fn unexpected(real: &String, expected: &str) -> LexError {
        LexError::Unexpected(expected, real.clone())
    }

    fn scan_literal(&mut self, delimiter: Delimiters, seq: &'a str) -> Option<Literal> {
        match delimiter {
            Delimiters::StringDelim(_) => Some(Literal::String(seq.trim_matches("\"").into())),
            Delimiters::Parentheses(_) => self.scan_thing(seq.trim_matches(['(', ')'])).ok(),
            Delimiters::AngleBracket(_) => None,
            Delimiters::CharDelim(_) => Some(Literal::Char(
                seq.trim_matches("'").chars().next().unwrap().into(),
            )),
            Delimiters::Brackets(_) => todo!("array literal"),
            Delimiters::Curly(_) => None,
        }
    }

    fn scan_thing(&mut self, real: &'a str) -> LexResult<TokenTree> {
        (KEYWORDS.get(real).map(|a| TokenTree::Keyword(*a)))
            .or_else(|| PUNCTUATIONS.get(real).map(TokenTree::Punct))
            .or_else(|| PREFIXES.get(real).map(TokenTree::Prefix))
            .or_else(|| {
                DELIMITERS
                    .get(&real.chars().next().unwrap())
                    .map(|a| self.scan_literal(*a, real))
                    .map(TokenTree::Literal)
            })
            .or_else(|| Ident::new_checked(real).map(TokenTree::Identifier))
            .ok_or_else(|| LexError::BadSequence(real.to_owned()))
    }

    pub fn next_token(&mut self) -> LexResult<TokenTree> {
        if self.eof() {
            return Err(LexError::EOF);
        }

        let real = self.advance_while(|a| !a.is_whitespace());

        self.scan_thing(real)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult<TokenTree>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = Lexer::<'a>::next_token(self);

        if let Err(LexError::EOF) = &token {
            None
        } else {
            Some(token)
        }
    }
}
