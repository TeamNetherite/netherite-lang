//! `lib.rs` - implements parser for Ry source files.
use std::mem::take;

use ry_ast::token::*;
use ry_ast::*;
use ry_lexer::Lexer;

use crate::error::ParserError;

pub mod error;

mod r#enum;
mod expression;
mod function_decl;
mod r#impl;
mod imports;
mod statement;
mod struct_decl;
mod trait_decl;
mod r#type;

#[macro_use]
mod macros;

pub struct Parser<'c> {
    lexer: Lexer<'c>,
    previous: Option<Token>,
    current: Token,
}

pub(crate) type ParserResult<T> = Result<T, ParserError>;

impl<'c> Parser<'c> {
    pub fn new(contents: &'c str) -> Self {
        let mut lexer = Lexer::new(contents);

        let current = lexer.next().unwrap();

        Self {
            lexer,
            previous: None,
            current,
        }
    }

    fn check_scanning_error(&mut self) -> ParserResult<()> {
        if let RawToken::Invalid(e) = self.current.value {
            Err(ParserError::ErrorToken(
                (e, self.current.span.clone()).into(),
            ))
        } else {
            Ok(())
        }
    }

    fn advance0(&mut self) -> ParserResult<()> {
        self.check_scanning_error()?;

        self.previous = Some(take(&mut self.current));
        self.current = self.lexer.next().unwrap();

        Ok(())
    }

    fn advance(&mut self) -> ParserResult<()> {
        self.check_scanning_error()?;

        self.previous = Some(take(&mut self.current));
        self.current = self.lexer.next_no_comments().unwrap();

        Ok(())
    }

    pub(crate) fn consume_fst_docstring(&mut self) -> ParserResult<(String, String)> {
        let (mut module_docstring, mut local_docstring) = ("".to_owned(), "".to_owned());
        loop {
            if let RawToken::Comment(s) = &self.current.value {
                if let Some(stripped) = s.strip_prefix('!') {
                    module_docstring.push_str(stripped.trim());
                    module_docstring.push('\n');
                } else if let Some(stripped) = s.strip_prefix('/') {
                    local_docstring.push_str(stripped.trim());
                    local_docstring.push('\n');
                }
            } else {
                module_docstring.pop();
                local_docstring.pop();
                return Ok((module_docstring, local_docstring));
            }

            self.advance0()?;
        }
    }

    pub(crate) fn consume_local_docstring(&mut self) -> ParserResult<String> {
        let mut result = "".to_owned();

        loop {
            if let RawToken::Comment(s) = &self.current.value {
                if let Some(stripped) = s.strip_prefix('/') {
                    result.push_str(stripped.trim());
                    result.push('\n');
                }
            } else {
                result.pop();
                return Ok(result);
            }

            self.advance0()?;
        }
    }

    pub fn parse(&mut self) -> ParserResult<ProgramUnit> {
        let module_docstring = self.consume_fst_docstring()?;
        Ok(ProgramUnit {
            docstring: module_docstring.0,
            imports: self.parse_imports()?,
            top_level_statements: self.parse_top_level_statements(module_docstring.1)?,
        })
    }

    fn parse_top_level_statements(
        &mut self,
        mut local_docstring: String,
    ) -> ParserResult<Vec<(String, TopLevelStatement)>> {
        let mut top_level_statements = vec![];

        loop {
            top_level_statements.push((
                local_docstring,
                match self.current.value {
                    RawToken::Fun => self.parse_function_declaration(None)?,
                    RawToken::Struct => self.parse_struct_declaration(None)?,
                    RawToken::Trait => self.parse_trait_declaration(None)?,
                    RawToken::Enum => self.parse_enum_declaration(None)?,
                    RawToken::Impl => self.parse_impl()?,
                    RawToken::Pub => {
                        self.advance()?;

                        self.check_scanning_error()?;

                        match self.current.value {
                            RawToken::Fun => {
                                self.parse_function_declaration(Some(self.current.span.clone()))?
                            }
                            RawToken::Struct => {
                                self.parse_struct_declaration(Some(self.current.span.clone()))?
                            }
                            RawToken::Trait => {
                                self.parse_trait_declaration(Some(self.current.span.clone()))?
                            }
                            RawToken::Enum => {
                                self.parse_enum_declaration(Some(self.current.span.clone()))?
                            }
                            _ => {
                                return Err(ParserError::UnexpectedToken(
                                    self.current.clone(),
                                    "top level declaration after `pub`".to_owned(),
                                    None,
                                ));
                            }
                        }
                    }
                    RawToken::Import => {
                        let start = self.current.span.range.start;

                        self.parse_import()?;

                        let end = self.current.span.range.end;
                        self.advance()?; // ';'

                        return Err(ParserError::ImportAfterTopLevelStatement(
                            (start..end).into(),
                        ));
                    }
                    RawToken::EndOfFile => break,
                    _ => {
                        let err = Err(ParserError::UnexpectedToken(
                            self.current.clone(),
                            "top level declaration".to_owned(),
                            None,
                        ));
                        self.advance()?;
                        return err;
                    }
                },
            ));

            local_docstring = self.consume_local_docstring()?;
        }

        Ok(top_level_statements)
    }
}
