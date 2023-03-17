use crate::{error::ParserError, macros::*, Parser, ParserResult};

use num_traits::ToPrimitive;
use ry_ast::*;
use ry_ast::{precedence::Precedence, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_expression(&mut self, precedence: i8) -> ParserResult<Expression> {
        let mut left = self.parse_prefix()?;

        while precedence < self.current.value.to_precedence() {
            left = match &self.current.value {
                binop_pattern!() => {
                    let start = left.span.start;

                    let op = self.current.clone();
                    let precedence = self.current.value.to_precedence();
                    self.advance()?; // op

                    let right = self.parse_expression(precedence)?;

                    let end = self.current.span.end;

                    Box::new(RawExpression::Binary(left, op, right)).with_span(start..end)
                }
                RawToken::OpenParent => {
                    let start = left.span.start;

                    self.advance()?; // '('

                    let arguments = parse_list!(
                        self,
                        "call arguments list",
                        &RawToken::CloseParent,
                        false,
                        || self.parse_expression(Precedence::Lowest.to_i8().unwrap())
                    );

                    let end = self.previous.as_ref().unwrap().span.end;

                    Box::new(RawExpression::Call(vec![], left, arguments)).with_span(start..end)
                }
                RawToken::Dot => {
                    self.advance()?; // `.`

                    if self.current.value.is(RawToken::LessThan) {
                        let start = left.span.start;

                        let generics = self.parse_type_generic_part()?;

                        check_token!(self, RawToken::OpenParent, "call")?;

                        self.advance()?; // '('

                        let arguments = parse_list!(
                            self,
                            "generics for call",
                            &RawToken::CloseParent,
                            false,
                            || self.parse_expression(Precedence::Lowest.to_i8().unwrap())
                        );

                        let end = self.previous.as_ref().unwrap().span.end;

                        Box::new(RawExpression::Call(
                            if let Some(v) = generics { v } else { vec![] },
                            left,
                            arguments,
                        ))
                        .with_span(start..end)
                    } else {
                        let start = left.span.start;

                        check_token0!(
                            self,
                            "identifier for property name",
                            RawToken::Identifier(_),
                            "property"
                        )?;

                        let name = self.get_name();
                        let end = self.current.span.end;

                        self.advance()?; // id

                        Box::new(RawExpression::Property(left, name)).with_span(start..end)
                    }
                }
                RawToken::OpenBracket => {
                    let start = left.span.start;

                    self.advance()?; // '['

                    let inner_expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                    check_token!(self, RawToken::CloseBracket, "index")?;

                    let end = self.current.span.end;

                    self.advance()?; // ']'

                    Box::new(RawExpression::Index(left, inner_expr)).with_span(start..end)
                }
                postfixop_pattern!() => {
                    let right = self.current.clone();
                    let span = left.span.start..self.current.span.end;

                    self.advance()?; // right

                    Box::new(RawExpression::PrefixOrPostfix(right, left)).with_span(span)
                }
                RawToken::As => {
                    self.advance()?; // as

                    let r#type = self.parse_type()?;

                    let span = left.span.start..self.previous.as_ref().unwrap().span.end;

                    Box::new(RawExpression::As(left, r#type)).with_span(span)
                }
                _ => break,
            };
        }

        Ok(left)
    }

    pub(crate) fn parse_prefix(&mut self) -> ParserResult<Expression> {
        self.check_scanning_error()?;

        match &self.current.value {
            RawToken::Int(i) => {
                let value = *i;
                let span = self.current.span;

                self.advance()?; // int

                Ok(Box::new(RawExpression::Int(value)).with_span(span))
            }
            RawToken::Float(f) => {
                let value = *f;
                let span = self.current.span;

                self.advance()?; // float

                Ok(Box::new(RawExpression::Float(value)).with_span(span))
            }
            RawToken::Imag(i) => {
                let value = *i;
                let span = self.current.span;

                self.advance()?; // imag

                Ok(Box::new(RawExpression::Imag(value)).with_span(span))
            }
            RawToken::String(s) => {
                let value = s.to_owned();
                let span = self.current.span;

                self.advance()?; // string

                Ok(Box::new(RawExpression::String(value)).with_span(span))
            }
            RawToken::Char(c) => {
                let value = *c;
                let span = self.current.span;

                self.advance()?; // char

                Ok(Box::new(RawExpression::Char(value)).with_span(span))
            }
            RawToken::Bool(b) => {
                let value = *b;
                let span = self.current.span;

                self.advance()?; // bool

                Ok(Box::new(RawExpression::Bool(value)).with_span(span))
            }
            RawToken::Bang
            | RawToken::Not
            | RawToken::PlusPlus
            | RawToken::MinusMinus
            | RawToken::Minus
            | RawToken::Plus => {
                let left = self.current.clone();
                let start = left.span.start;
                self.advance()?; // left

                let expr = self.parse_expression(Precedence::PrefixOrPostfix.to_i8().unwrap())?;
                let end = expr.span.end;

                Ok(Box::new(RawExpression::PrefixOrPostfix(left, expr)).with_span(start..end))
            }
            RawToken::OpenParent => {
                self.advance()?; // '('

                let expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                check_token!(self, RawToken::CloseParent, "parenthesized expression")?;

                self.advance()?; // ')'

                Ok(expr)
            }
            RawToken::OpenBracket => {
                let start = self.current.span.start;
                self.advance()?; // '['

                let list =
                    parse_list!(self, "list literal", &RawToken::CloseBracket, false, || {
                        self.parse_expression(Precedence::Lowest.to_i8().unwrap())
                    });

                let end = self.previous.as_ref().unwrap().span.end;

                Ok(Box::new(RawExpression::List(list)).with_span(start..end))
            }
            RawToken::Identifier(_) => {
                let n = self.parse_name()?;

                Ok(Box::new(RawExpression::StaticName(n.value)).with_span(n.span))
            }
            RawToken::If => {
                let start = self.current.span.start;
                self.advance()?;

                let if_condition = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;
                let if_statements_block = self.parse_statements_block(false)?;

                let mut else_statements_block = None;
                let mut else_if_chains = vec![];

                while self.current.value.is(RawToken::Else) {
                    self.advance()?; // else

                    if !self.current.value.is(RawToken::If) {
                        else_statements_block = Some(self.parse_statements_block(false)?);
                        break;
                    }

                    self.advance()?; // if

                    let else_if_condition =
                        self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                    let else_if_statements_block = self.parse_statements_block(false)?;

                    else_if_chains.push((else_if_condition, else_if_statements_block));
                }

                let end = self.current.span.end;

                Ok(Box::new(RawExpression::If(
                    (if_condition, if_statements_block),
                    else_if_chains,
                    else_statements_block,
                ))
                .with_span(start..end))
            }
            RawToken::While => {
                let start = self.current.span.start;

                self.advance()?; // 'while'

                let condition = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;
                let block = self.parse_statements_block(false)?;

                let end = self.current.span.end;

                Ok(Box::new(RawExpression::While(condition, block)).with_span(start..end))
            }
            _ => Err(ParserError::UnexpectedToken(
                self.current.clone(),
                "expression".to_owned(),
                None,
            )),
        }
    }
}
