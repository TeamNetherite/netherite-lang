//! lexer.rs - implements lexer.
//!
//! Lexer implements `Iterator<Item = Token>`.

use std::{path::Path, str::Chars};

use super::location::Span;
use super::token::LexerError;
use crate::lexer::location::Location;
use crate::lexer::token::RawToken;
use crate::lexer::token::Token;
use crate::lexer::token::RESERVED;

pub struct Lexer<'a> {
    current: char,
    next: char,
    filename: &'a Path,
    contents: &'a str,
    chars: Chars<'a>,
    location: Location,
    start_location: Location,
}

type IterElem<'a> = Option<Token<'a>>;

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a Path, contents: &'a str) -> Self {
        let mut chars = contents.chars();

        let current = chars.next().unwrap_or('\0');
        let next = chars.next().unwrap_or('\0');

        Self {
            current,
            next,
            filename,
            contents,
            chars,
            location: Location::start(),
            start_location: Location::start(),
        }
    }

    fn eof(&self) -> bool {
        self.current == '\0'
    }

    fn skip_over_whitespaces(&mut self) {
        while self.current.is_whitespace() {
            self.advance();
        }
    }

    fn advance(&mut self) -> char {
        let previous = self.current;

        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');

        self.location.advance(previous.len_utf8(), previous == '\n');

        previous
    }

    fn char_location(&self, character_len: usize) -> Span<'a> {
        Span {
            filename: self.filename,
            start: self.location.clone(),
            end: Location {
                index: self.location.index + character_len,
                line: self.location.line,
                column: self.location.column + 1,
            },
        }
    }

    fn advance_with(&mut self, raw: RawToken) -> IterElem<'a> {
        self.advance();
        Some(Token::new(raw, self.char_location(1)))
    }

    fn advance_while<F>(&mut self, mut f: F) -> &'a str
    where
        F: FnMut(char, char) -> bool,
    {
        while f(self.current, self.next) && !self.eof() {
            self.advance();
        }

        &self.contents[self.start_location.index..self.location.index]
    }

    fn scan_name(&mut self) -> IterElem<'a> {
        self.start_location = self.location.clone();
        let name = self.advance_while(|current, _| current.is_alphanumeric() || current == '_');

        match RESERVED.get(name) {
            Some(reserved) => Some(Token::new(
                reserved.clone(),
                Span::new(self.filename, self.start_location, self.location),
            )),
            None => Some(Token::new(
                RawToken::Identifier(name.to_owned()),
                Span::new(self.filename, self.start_location, self.location),
            )),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_whitespace() {
            self.skip_over_whitespaces();
        }

        match (self.current, self.next) {
            ('\0', _) => self.advance_with(RawToken::EndOfFile),
            ('+', '+') => self.advance_with(RawToken::PlusPlus),
            ('+', '=') => self.advance_with(RawToken::PlusEq),
            ('+', _) => self.advance_with(RawToken::Plus),

            ('-', '-') => self.advance_with(RawToken::MinusMinus),
            ('-', '=') => self.advance_with(RawToken::MinusEq),
            ('-', _) => self.advance_with(RawToken::Minus),

            ('*', '=') => self.advance_with(RawToken::AsteriskEq),
            ('*', _) => self.advance_with(RawToken::Asterisk),

            ('/', '=') => self.advance_with(RawToken::SlashEq),
            ('/', _) => self.advance_with(RawToken::Slash),

            ('!', '=') => self.advance_with(RawToken::NotEq),
            ('!', _) => self.advance_with(RawToken::Bang),

            ('>', '>') => self.advance_with(RawToken::RightShift),
            ('>', '=') => self.advance_with(RawToken::GreaterThanOrEq),
            ('>', _) => self.advance_with(RawToken::GreaterThan),

            ('<', '<') => self.advance_with(RawToken::LeftShift),
            ('<', '=') => self.advance_with(RawToken::LessThanOrEq),
            ('<', _) => self.advance_with(RawToken::LessThan),

            ('=', '=') => self.advance_with(RawToken::Eq),
            ('=', _) => self.advance_with(RawToken::Assign),

            ('|', '=') => self.advance_with(RawToken::OrEq),
            ('|', '|') => self.advance_with(RawToken::OrOr),
            ('|', _) => self.advance_with(RawToken::Or),

            ('&', '&') => self.advance_with(RawToken::AndAnd),
            ('&', _) => self.advance_with(RawToken::And),

            ('^', '=') => self.advance_with(RawToken::XorEq),
            ('^', _) => self.advance_with(RawToken::Xor),

            ('~', '=') => self.advance_with(RawToken::NotEq),
            ('~', _) => self.advance_with(RawToken::Not),

            ('(', _) => self.advance_with(RawToken::OpenParent),
            (')', _) => self.advance_with(RawToken::CloseParent),

            ('[', _) => self.advance_with(RawToken::OpenBracket),
            (']', _) => self.advance_with(RawToken::CloseBracket),

            ('{', _) => self.advance_with(RawToken::OpenBrace),
            ('}', _) => self.advance_with(RawToken::CloseBrace),

            ('.', _) => self.advance_with(RawToken::Dot),
            (',', _) => self.advance_with(RawToken::Comma),
            (';', _) => self.advance_with(RawToken::Semicolon),

            (c, _) => {
                if c.is_ascii_digit() {
                    todo!("implement number scanning")
                } else if c.is_alphanumeric() || c == '_' {
                    return self.scan_name();
                }

                self.advance_with(RawToken::Invalid(LexerError::UnexpectedChar(c)))
            }
        }
    }
}
