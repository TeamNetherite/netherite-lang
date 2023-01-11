use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::*;
use ry_ast::{
    location::{Span, WithSpan},
    token::*,
};

impl<'c> Parser<'c> {
    pub(crate) fn parse_name(&mut self) -> ParserResult<WithSpan<String>> {
        let start = self.current.span.range.start;

        let mut name = self.current.value.ident().unwrap();
        name.push_str("::");

        let mut end = self.current.span.range.end;

        self.advance()?; // id

        while self.current.value.is(&RawToken::DoubleColon) {
            self.advance()?; // '::'

            check_token0!(self, "identifier", RawToken::Identifier(_), "name")?;

            name.push_str(&self.current.value.ident().unwrap());
            name.push_str("::");

            end = self.current.span.range.end;

            self.advance()?; // id
        }

        name.pop();
        name.pop();

        Ok((name, (start..end).into()).into())
    }

    pub(crate) fn parse_type(
        &mut self,
        return_type: bool,
        function_definition: bool,
    ) -> ParserResult<WithSpan<Box<Type>>> {
        let start = self.current.span.range.start;

        let mut lhs = match &self.current.value {
            RawToken::Identifier(_) => self.parse_custom_type(),
            RawToken::Asterisk => self.parse_pointer_type(),
            RawToken::OpenBracket => self.parse_array_type(),
            RawToken::Impls => self.parse_impls_type(),
            RawToken::PrimaryType(t) => {
                let r = Ok(WithSpan::new(
                    Box::new(Type::Primary(WithSpan::new(*t, self.current.span.clone()))),
                    self.current.span.clone(),
                ));
                self.advance()?;
                r
            }
            _ => {
                if return_type {
                    if !function_definition {
                        Err(ParserError::UnexpectedToken(
                            self.current.clone(),
                            "function return type or '{'".into(),
                            None,
                        ))
                    } else {
                        Err(ParserError::UnexpectedToken(
                            self.current.clone(),
                            "return type or '{'".into(),
                            None,
                        ))
                    }
                } else {
                    Err(ParserError::UnexpectedToken(
                        self.current.clone(),
                        "type".into(),
                        None,
                    ))
                }
            }
        }?;

        while self.current.value.is(&RawToken::QuestionMark) {
            lhs = WithSpan::new(
                Box::new(Type::Option(lhs)),
                Span::new(start, self.current.span.range.end),
            );
            self.advance()?;
        }

        Ok(lhs)
    }

    fn parse_custom_type(&mut self) -> ParserResult<WithSpan<Box<Type>>> {
        let start = self.current.span.range.end;
        let name = self.parse_name()?;
        let generic_part = self.parse_type_generic_part()?;
        let end = self.current.span.range.end;

        if generic_part.is_some() {
            self.advance()?; // '>'
        }

        Ok(WithSpan::new(
            Box::new(Type::Custom(
                name,
                if let Some(v) = generic_part {
                    v
                } else {
                    vec![]
                },
            )),
            Span::new(start, end),
        ))
    }

    pub(crate) fn parse_type_generic_part(
        &mut self,
    ) -> ParserResult<Option<Vec<WithSpan<Box<Type>>>>> {
        if self.current.value.is(&RawToken::LessThan) {
            self.advance()?;

            let mut generic_part = vec![];

            if self.current.value.is(&RawToken::GreaterThan) {
                return Ok(Some(vec![]));
            }

            generic_part.push(self.parse_type(false, false)?);

            while self.current.value.is(&RawToken::Comma) {
                self.advance()?;
                generic_part.push(self.parse_type(false, false)?);
            }

            check_token!(self, RawToken::GreaterThan, "generic annotations")?;

            Ok(Some(generic_part))
        } else {
            Ok(None)
        }
    }

    fn parse_array_type(&mut self) -> ParserResult<WithSpan<Box<Type>>> {
        let start = self.current.span.range.start;

        self.advance()?; // '['

        let inner_type = self.parse_type(false, false)?;

        check_token!(self, RawToken::CloseBracket, "array type")?;

        let end = self.current.span.range.end;

        self.advance()?; // ']'

        Ok(WithSpan::new(
            Box::new(Type::Array(inner_type)),
            Span::new(start, end),
        ))
    }

    fn parse_pointer_type(&mut self) -> ParserResult<WithSpan<Box<Type>>> {
        let start = self.current.span.range.start;

        self.advance()?; // '*'

        let inner_type = self.parse_type(false, false)?;

        let end = self.current.span.range.end;

        Ok(WithSpan::new(
            Box::new(Type::Pointer(inner_type)),
            Span::new(start, end),
        ))
    }

    fn parse_impls_type(&mut self) -> ParserResult<WithSpan<Box<Type>>> {
        let start = self.current.span.range.start;

        self.advance()?; // '*'

        let inner_type = self.parse_type(false, false)?;

        let end = self.current.span.range.end;

        Ok(WithSpan::new(
            Box::new(Type::Impls(inner_type)),
            Span::new(start, end),
        ))
    }

    pub(crate) fn parse_generic_annotations(&mut self) -> ParserResult<GenericAnnotations> {
        let mut generics = vec![];

        if !self.current.value.is(&RawToken::LessThan) {
            return Ok(generics);
        }

        self.advance()?; // '<'

        if self.current.value.is(&RawToken::GreaterThan) {
            self.advance()?; // '>'
            return Ok(generics);
        }

        loop {
            check_token0!(
                self,
                "identifier",
                RawToken::Identifier(_),
                "generic annotation"
            )?;

            let generic = self.parse_generic()?;

            let mut constraint = None;

            if !self.current.value.is(&RawToken::Comma)
                && !self.current.value.is(&RawToken::GreaterThan)
            {
                constraint = Some(self.parse_type(false, false)?);
            }

            generics.push((generic, constraint));

            if !self.current.value.is(&RawToken::Comma) {
                check_token!(self, RawToken::GreaterThan, "generic annotations")?;

                self.advance()?; // >

                return Ok(generics);
            }

            self.advance()?;
        }
    }

    pub fn parse_generic(&mut self) -> ParserResult<WithSpan<String>> {
        let start = self.current.span.range.start;

        let name = self.current.value.ident().unwrap();
        let end = self.current.span.range.end;

        self.advance()?; // id

        Ok(WithSpan::new(name, Span::new(start, end)))
    }
}
