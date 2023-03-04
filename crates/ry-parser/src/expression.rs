use crate::{error::ParserError, macros::*, Parser, ParserResult};

use num_traits::ToPrimitive;
use ry_ast::*;
use ry_ast::{precedence::Precedence, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_expression(&mut self, precedence: i8) -> ParserResult<Expression> {
        let mut left = self.parse_prefix()?;

        while precedence < self.current.value.to_precedence() {
            left = match &self.current.value {
                RawToken::Plus
                | RawToken::Minus
                | RawToken::Asterisk
                | RawToken::Slash
                | RawToken::Eq
                | RawToken::NotEq
                | RawToken::LessThan
                | RawToken::LessThanOrEq
                | RawToken::GreaterThan
                | RawToken::GreaterThanOrEq
                | RawToken::Assign
                | RawToken::OrEq
                | RawToken::XorEq
                | RawToken::PlusEq
                | RawToken::MinusEq
                | RawToken::SlashEq
                | RawToken::AsteriskEq
                | RawToken::AsteriskAsterisk
                | RawToken::Percent
                | RawToken::And
                | RawToken::Xor
                | RawToken::Or
                | RawToken::OrOr
                | RawToken::Elvis
                | RawToken::AndAnd
                | RawToken::LeftShift
                | RawToken::RightShift => self.parse_infix(left)?,
                RawToken::OpenParent => self.parse_call(left)?,
                RawToken::Dot => self.parse_property(left)?,
                RawToken::OpenBracket => self.parse_index(left)?,
                RawToken::QuestionMark
                | RawToken::PlusPlus
                | RawToken::MinusMinus
                | RawToken::BangBang => self.parse_postfix(left)?,
                RawToken::Dollar => {
                    self.advance()?;

                    self.parse_call_with_generics(left)?
                }
                _ => break,
            };
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> ParserResult<Expression> {
        let left = self.current.clone();
        let start = left.span.range.start;
        self.advance()?; // left

        let expr = self.parse_expression(Precedence::PrefixOrPostfix.to_i8().unwrap())?;
        let end = expr.span.range.end;

        Ok((
            Box::new(RawExpression::PrefixOrPostfix(left, expr)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_postfix(&mut self, left: Expression) -> ParserResult<Expression> {
        let right = self.current.clone();
        let span = (left.span.range.start..self.current.span.range.end).into();

        self.advance()?; // right

        Ok((Box::new(RawExpression::PrefixOrPostfix(right, left)), span).into())
    }

    pub(crate) fn parse_prefix(&mut self) -> ParserResult<Expression> {
        self.check_scanning_error()?;

        match &self.current.value {
            RawToken::Int(_) => {
                let i = self.current.value.int().unwrap();
                let span = self.current.span.clone();

                self.advance()?; // int

                Ok((Box::new(RawExpression::Int(i)), span).into())
            }
            RawToken::Float(_) => {
                let f = self.current.value.float().unwrap();
                let span = self.current.span.clone();

                self.advance()?; // float

                Ok((Box::new(RawExpression::Float(f)), span).into())
            }
            RawToken::Imag(_) => {
                let i = self.current.value.imag().unwrap();
                let span = self.current.span.clone();

                self.advance()?; // imag

                Ok((Box::new(RawExpression::Imag(i)), span).into())
            }
            RawToken::String(_) => {
                let s = self.current.value.string().unwrap();
                let span = self.current.span.clone();

                self.advance()?; // string

                Ok((Box::new(RawExpression::String(s)), span).into())
            }
            RawToken::Bool(_) => {
                let b = self.current.value.bool().unwrap();
                let span = self.current.span.clone();

                self.advance()?; // bool

                Ok((Box::new(RawExpression::Bool(b)), span).into())
            }
            RawToken::Bang
            | RawToken::Not
            | RawToken::PlusPlus
            | RawToken::MinusMinus
            | RawToken::Minus
            | RawToken::Plus => self.parse_prefix_expression(),
            RawToken::OpenParent => {
                self.advance()?; // '('

                let expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                check_token!(self, RawToken::CloseParent, "parenthesized expression")?;

                self.advance()?; // ')'

                Ok(expr)
            }
            RawToken::OpenBracket => {
                let start = self.current.span.range.start;
                self.advance()?; // '['

                let mut list = vec![];
                parse_list_of_smth!(self, list, &RawToken::CloseBracket, || {
                    self.parse_expression(Precedence::Lowest.to_i8().unwrap())
                });

                let end = self.current.span.range.end;
                self.advance()?; // ']'

                Ok((Box::new(RawExpression::List(list)), (start..end).into()).into())
            }
            RawToken::Identifier(_) => {
                let n = self.parse_name()?;

                Ok((Box::new(RawExpression::StaticName(n.value)), n.span).into())
            }
            RawToken::If => {
                let start = self.current.span.range.start;
                self.advance()?;

                let if_condition = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;
                let if_statements_block = self.parse_statements_block(false)?;

                let mut else_statements_block = None;
                let mut else_if_chains = vec![];

                while self.current.value.is(&RawToken::Else) {
                    self.advance()?; // else

                    if !self.current.value.is(&RawToken::If) {
                        else_statements_block = Some(self.parse_statements_block(false)?);
                        break;
                    }

                    self.advance()?; // if

                    let else_if_condition =
                        self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

                    let else_if_statements_block = self.parse_statements_block(false)?;

                    else_if_chains.push((else_if_condition, else_if_statements_block));
                }

                let end = self.current.span.range.end;

                Ok((
                    Box::new(RawExpression::If(
                        (if_condition, if_statements_block),
                        else_if_chains,
                        else_statements_block,
                    )),
                    (start..end).into(),
                )
                    .into())
            }
            _ => Err(ParserError::UnexpectedToken(
                self.current.clone(),
                "expression".into(),
                None,
            )),
        }
    }

    fn parse_infix(&mut self, left: Expression) -> ParserResult<Expression> {
        let start = left.span.range.start;

        let op = self.current.clone();
        let precedence = self.current.value.to_precedence();
        self.advance()?; // op

        let right = self.parse_expression(precedence)?;

        let end = self.current.span.range.end;

        Ok((
            Box::new(RawExpression::Binary(left, op, right)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_property(&mut self, left: Expression) -> ParserResult<Expression> {
        let start = left.span.range.start;

        self.advance()?; // '.'

        check_token0!(
            self,
            "identifier for property name",
            RawToken::Identifier(_),
            "property"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        let end = self.current.span.range.end;

        self.advance()?; // id

        Ok((
            Box::new(RawExpression::Property(left, name)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_function_arguments_expressions(&mut self) -> ParserResult<Vec<Expression>> {
        let mut l = vec![];

        self.advance()?; // '('

        if self.current.value.is(&RawToken::CloseParent) {
            return Ok(l);
        }

        l.push(self.parse_expression(Precedence::Lowest.to_i8().unwrap())?);

        while self.current.value.is(&RawToken::Comma) {
            self.advance()?;

            l.push(self.parse_expression(Precedence::Lowest.to_i8().unwrap())?);
        }

        check_token!(self, RawToken::CloseParent, "arguments")?;

        Ok(l)
    }

    fn parse_index(&mut self, left: Expression) -> ParserResult<Expression> {
        let start = left.span.range.start;

        self.advance()?; // '['

        let inner_expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

        check_token!(self, RawToken::CloseBracket, "index")?;

        let end = self.current.span.range.end;

        self.advance()?; // ']'

        Ok((
            Box::new(RawExpression::Index(left, inner_expr)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_call(&mut self, left: Expression) -> ParserResult<Expression> {
        let start = left.span.range.start;

        let arguments = self.parse_function_arguments_expressions()?;
        let end = self.current.span.range.end;

        self.advance()?; // ')'

        Ok((
            Box::new(RawExpression::Call(vec![], left, arguments)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_call_with_generics(&mut self, left: Expression) -> ParserResult<Expression> {
        let start = left.span.range.start;

        let generics = self.parse_type_generic_part()?;

        if generics.is_some() {
            self.advance()?; // '>'
        }

        let arguments = self.parse_function_arguments_expressions()?;

        let end = self.current.span.range.end;

        self.advance()?; // ')'

        Ok((
            Box::new(RawExpression::Call(
                if let Some(v) = generics { v } else { vec![] },
                left,
                arguments,
            )),
            (start..end).into(),
        )
            .into())
    }
}
