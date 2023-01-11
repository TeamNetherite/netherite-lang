use crate::{error::ParserError, macros::*, Parser, ParserResult};

use num_traits::ToPrimitive;
use ry_ast::*;
use ry_ast::{location::WithSpan, precedence::Precedence, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_expression(
        &mut self,
        precedence: i8,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
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

    fn parse_prefix_expression(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let left = self.current.clone();
        let start = left.span.range.start;
        self.advance()?; // left

        let expr = self.parse_expression(Precedence::PrefixOrPostfix.to_i8().unwrap())?;
        let end = expr.span.range.end;

        Ok((
            Box::new(Expression::PrefixOrPostfix(left, expr)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_postfix(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
        let right = self.current.clone();
        let span = (left.span.range.start..self.current.span.range.end).into();

        self.advance()?; // right

        Ok((Box::new(Expression::PrefixOrPostfix(right, left)), span).into())
    }

    pub(crate) fn parse_prefix(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        self.check_scanning_error()?;

        match &self.current.value {
            RawToken::Int(_) => self.parse_integer(),
            RawToken::Float(_) => self.parse_float(),
            RawToken::Imag(_) => self.parse_imag(),
            RawToken::String(_) => self.parse_string(),
            RawToken::Bool(_) => self.parse_boolean(),
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

                Ok((Box::new(Expression::List(list)), (start..end).into()).into())
            }
            RawToken::Identifier(_) => {
                let n = self.parse_name()?;

                Ok((Box::new(Expression::StaticName(n.value)), n.span).into())
            }
            _ => Err(ParserError::UnexpectedToken(
                self.current.clone(),
                "expression".into(),
                None,
            )),
        }
    }

    fn parse_infix(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
        let start = left.span.range.start;

        let op = self.current.clone();
        let precedence = self.current.value.to_precedence();
        self.advance()?; // op

        let right = self.parse_expression(precedence)?;

        let end = self.current.span.range.end;

        Ok((
            Box::new(Expression::Binary(left, op, right)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_property(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
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
            Box::new(Expression::Property(left, name)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_function_arguments_expressions(
        &mut self,
    ) -> ParserResult<Vec<WithSpan<Box<Expression>>>> {
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

    fn parse_index(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
        let start = left.span.range.start;

        self.advance()?; // '['

        let inner_expr = self.parse_expression(Precedence::Lowest.to_i8().unwrap())?;

        check_token!(self, RawToken::CloseBracket, "index")?;

        let end = self.current.span.range.end;

        self.advance()?; // ']'

        Ok((
            Box::new(Expression::Index(left, inner_expr)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_call(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
        let start = left.span.range.start;

        let arguments = self.parse_function_arguments_expressions()?;
        let end = self.current.span.range.end;

        self.advance()?; // ')'

        Ok((
            Box::new(Expression::Call(vec![], left, arguments)),
            (start..end).into(),
        )
            .into())
    }

    fn parse_call_with_generics(
        &mut self,
        left: WithSpan<Box<Expression>>,
    ) -> ParserResult<WithSpan<Box<Expression>>> {
        let start = left.span.range.start;

        let generics = self.parse_type_generic_part()?;

        if generics.is_some() {
            self.advance()?; // '>'
        }

        let arguments = self.parse_function_arguments_expressions()?;

        let end = self.current.span.range.end;

        self.advance()?; // ')'

        Ok((
            Box::new(Expression::Call(
                if let Some(v) = generics { v } else { vec![] },
                left,
                arguments,
            )),
            (start..end).into(),
        )
            .into())
    }

    fn parse_boolean(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let b = self.current.value.bool().unwrap();
        let span = self.current.span.clone();

        self.advance()?; // bool

        Ok((Box::new(Expression::Bool(b)), span).into())
    }

    fn parse_integer(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let i = self.current.value.int().unwrap();
        let span = self.current.span.clone();

        self.advance()?; // int

        Ok((Box::new(Expression::Int(i)), span).into())
    }

    fn parse_string(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let s = self.current.value.string().unwrap();
        let span = self.current.span.clone();

        self.advance()?; // string

        Ok((Box::new(Expression::String(s)), span).into())
    }

    fn parse_float(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let f = self.current.value.float().unwrap();
        let span = self.current.span.clone();

        self.advance()?; // float

        Ok((Box::new(Expression::Float(f)), span).into())
    }

    fn parse_imag(&mut self) -> ParserResult<WithSpan<Box<Expression>>> {
        let i = self.current.value.imag().unwrap();
        let span = self.current.span.clone();

        self.advance()?; // imag

        Ok((Box::new(Expression::Imag(i)), span).into())
    }
}
