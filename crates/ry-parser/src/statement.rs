use std::ops::Deref;

use crate::{error::ParserError, macros::*, Parser, ParserResult};

use num_traits::ToPrimitive;

use ry_ast::*;
use ry_ast::{precedence::Precedence, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_statements_block(
        &mut self,
        top_level: bool,
    ) -> ParserResult<StatementsBlock> {
        check_token!(self, RawToken::OpenBrace, "statements block")?;

        self.advance()?; // '{'

        let mut stmts = vec![];

        while !self.current.value.is(RawToken::CloseBrace) {
            let (stmt, last) = self.parse_statement()?;

            stmts.push(stmt);

            if last {
                break;
            }
        }

        check_token!(self, RawToken::CloseBrace, "statements block")?;

        if top_level {
            self.advance0()?;
        } else {
            self.advance()?;
        }

        Ok(stmts)
    }

    fn parse_statement(&mut self) -> ParserResult<(Statement, bool)> {
        let mut last_statement_in_block = false;
        let mut must_have_semicolon_at_the_end = true;

        let statement = match self.current.value {
            RawToken::Return => {
                self.advance()?; // return

                let expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                Ok(Statement::Return(expr))
            }
            RawToken::Defer => {
                self.advance()?; // defer

                let expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                Ok(Statement::Defer(expr))
            }
            RawToken::Var => {
                self.advance()?; // var

                check_token0!(self, "identifier", RawToken::Identifier(_), "var statement")?;

                let name = self.get_name();

                self.advance()?; // id

                let mut r#type = None;

                if !self.current.value.is(RawToken::Assign) {
                    r#type = Some(self.parse_type()?);
                }

                check_token!(self, RawToken::Assign, "var statement")?;

                self.advance()?; // '='

                let value = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                Ok(Statement::Var(name, r#type, value))
            }
            _ => {
                let expression = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                must_have_semicolon_at_the_end =
                    expression.value.deref().must_have_semicolon_at_the_end();

                if !self.current.value.is(RawToken::Semicolon) && must_have_semicolon_at_the_end {
                    last_statement_in_block = true;
                }

                if last_statement_in_block || !must_have_semicolon_at_the_end {
                    Ok(Statement::ExpressionWithoutSemicolon(expression))
                } else {
                    Ok(Statement::Expression(expression))
                }
            }
        }?;

        if !last_statement_in_block && must_have_semicolon_at_the_end {
            check_token!(self, RawToken::Semicolon, "end of the statement")?;
            self.advance()?; // ';'
        }

        Ok((statement, last_statement_in_block))
    }
}
