use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::location::WithSpan;
use ry_ast::*;
use ry_ast::{location::Span, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_trait_declaration(
        &mut self,
        public: Option<Span>,
    ) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'trait'

        check_token0!(
            self,
            "identifier for trait name",
            RawToken::Identifier(_),
            "trait declaration"
        )?;

        let name: WithSpan<String> = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // 'name'

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenBrace, "trait declaration")?;

        self.advance0()?; // '{'

        let methods = self.parse_trait_methods()?;

        check_token!(self, RawToken::CloseBrace, "trait declaration")?;

        self.advance0()?; // '}'

        Ok(TopLevelStatement::TraitDecl(TraitDecl {
            public,
            generic_annotations,
            name,
            methods,
        }))
    }

    pub(crate) fn parse_trait_methods(&mut self) -> ParserResult<Vec<(String, TraitMethod)>> {
        let mut definitions = vec![];

        let mut unnecessary_qualifier_error_span = None;

        while !self.current.value.is(&RawToken::CloseBrace) {
            self.consume_local_docstring()?;

            if self.current.value.is(&RawToken::Pub) {
                unnecessary_qualifier_error_span = Some(self.current.span.clone());
                self.advance()?;
            }

            let trait_def = self.parse_trait_method()?;
            let declaration = trait_def.body.is_some();
            let name_span = trait_def.name.span.clone();
            definitions.push((self.consume_local_docstring()?, trait_def));

            if let Some(s) = unnecessary_qualifier_error_span {
                return Err(ParserError::UnnecessaryVisibilityQualifier(
                    s,
                    name_span,
                    declaration,
                ));
            }
        }

        Ok(definitions)
    }

    fn parse_trait_method(&mut self) -> ParserResult<TraitMethod> {
        check_token!(self, RawToken::Fun, "trait method")?;

        self.advance()?; // 'fun'

        check_token0!(
            self,
            "identifier for method name",
            RawToken::Identifier(_),
            "trait method"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // name

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenParent, "trait method")?;

        self.advance()?; // '('

        let arguments = parse_list!(
            self,
            "trait method arguments",
            &RawToken::CloseParent,
            false,
            || self.parse_function_argument()
        );

        let mut return_type = None;

        if !self.current.value.is(&RawToken::Semicolon)
            && !self.current.value.is(&RawToken::OpenBrace)
        {
            return_type = Some(self.parse_type()?);
        }

        let mut body = None;

        match self.current.value {
            RawToken::Semicolon => self.advance0()?,
            RawToken::OpenBrace => {
                body = Some(self.parse_statements_block(true)?);
            }
            _ => {
                return Err(ParserError::UnexpectedToken(
                    self.current.clone(),
                    "`;` (for method definition) or `{` (for method declaration)".to_owned(),
                    Some("trait method".to_owned()),
                ));
            }
        }

        Ok(TraitMethod {
            name,
            generic_annotations,
            params: arguments,
            return_type,
            body,
        })
    }
}
