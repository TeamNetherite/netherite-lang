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
mod interface_decl;
mod statement;
mod struct_decl;
mod r#type;

#[macro_use]
mod macros;

pub struct Parser<'c> {
    lexer: Lexer<'c>,
    current: Token,
    docstring_buffer: String,
}

pub(crate) type ParserResult<T> = Result<T, ParserError>;

impl<'c> Parser<'c> {
    pub fn new(contents: &'c str) -> Self {
        let mut lexer = Lexer::new(contents);

        let current = lexer.next().unwrap();

        Self {
            lexer,
            current,
            docstring_buffer: "".into(),
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

        self.current = self.lexer.next().unwrap();

        Ok(())
    }

    fn advance(&mut self) -> ParserResult<()> {
        self.check_scanning_error()?;

        self.current = self.lexer.next_no_comments().unwrap();

        Ok(())
    }

    fn consume_fst_docstring(&mut self) -> ParserResult<String> {
        let mut module_docstring = "".to_owned();
        loop {
            if let RawToken::Comment(s) = &self.current.value {
                if let Some(stripped) = s.strip_prefix('!') {
                    module_docstring.push_str(stripped.trim());
                    module_docstring.push('\n');
                } else if let Some(stripped) = s.strip_prefix('/') {
                    self.docstring_buffer.push_str(stripped.trim());
                    self.docstring_buffer.push('\n');
                }
            } else {
                module_docstring.pop();
                self.docstring_buffer.pop();
                return Ok(module_docstring);
            }

            self.advance0()?;
        }
    }

    fn consume_statement_docstring(&mut self) -> ParserResult<()> {
        loop {
            if let RawToken::Comment(s) = &self.current.value {
                if let Some(stripped) = s.strip_prefix('/') {
                    self.docstring_buffer.push_str(stripped.trim());
                    self.docstring_buffer.push('\n');
                }
            } else {
                self.docstring_buffer.pop();
                return Ok(());
            }

            self.advance0()?;
        }
    }

    pub fn parse(&mut self) -> ParserResult<ProgramUnit> {
        let module_docstring = self.consume_fst_docstring()?;
        Ok(ProgramUnit {
            docstring: module_docstring,
            imports: self.parse_imports()?,
            top_level_statements: self.parse_top_level_statements()?,
        })
    }

    fn parse_top_level_statements(&mut self) -> ParserResult<Vec<(TopLevelStatement, String)>> {
        let mut top_level_statements = vec![];

        loop {
            top_level_statements.push((
                match self.current.value {
                    RawToken::Fun => self.parse_function_declaration(None)?,
                    RawToken::Struct => self.parse_struct_declaration(None)?,
                    RawToken::Interface => self.parse_interface_declaration(None)?,
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
                            RawToken::Interface => {
                                self.parse_interface_declaration(Some(self.current.span.clone()))?
                            }
                            RawToken::Enum => {
                                self.parse_enum_declaration(Some(self.current.span.clone()))?
                            }
                            _ => {
                                return Err(ParserError::UnexpectedToken(
                                    self.current.clone(),
                                    "top level declaration after `pub`".into(),
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
                            "top level declaration".into(),
                            None,
                        ));
                        self.advance()?;
                        return err;
                    }
                },
                take(&mut self.docstring_buffer),
            ));

            self.consume_statement_docstring()?;
        }

        Ok(top_level_statements)
    }
}
