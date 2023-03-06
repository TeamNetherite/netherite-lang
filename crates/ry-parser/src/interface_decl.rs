use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::location::WithSpan;
use ry_ast::*;
use ry_ast::{location::Span, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_interface_declaration(
        &mut self,
        public: Option<Span>,
    ) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'interface'

        check_token0!(
            self,
            "identifier for interface name",
            RawToken::Identifier(_),
            "interface declaration"
        )?;

        let name: WithSpan<String> = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // 'name'

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenBrace, "interface declaration")?;

        self.advance()?; // '{'

        let methods = self.parse_interface_method_definitions(name.span.clone())?;

        check_token!(self, RawToken::CloseBrace, "interface declaration")?;

        self.advance0()?; // '}'

        Ok(TopLevelStatement::InterfaceDecl(InterfaceDecl {
            public,
            generic_annotations,
            name,
            methods,
        }))
    }

    fn parse_interface_method_definitions(
        &mut self,
        interface_name_span: Span,
    ) -> ParserResult<Vec<InterfaceMethodDef>> {
        let mut definitions = vec![];

        let mut unnecessary_qualifier_error_span = None;

        while !self.current.value.is(&RawToken::CloseBrace) {
            if self.current.value.is(&RawToken::Pub) {
                unnecessary_qualifier_error_span = Some(self.current.span.clone());
                self.advance()?;
            }

            let interface_def = self.parse_interface_method_definition()?;
            let name_span = interface_def.name.span.clone();
            definitions.push(interface_def);

            if let Some(s) = unnecessary_qualifier_error_span {
                return Err(ParserError::UnnecessaryVisibilityQualifier(
                    s,
                    name_span,
                    interface_name_span,
                ));
            }
        }

        Ok(definitions)
    }

    fn parse_interface_method_definition(&mut self) -> ParserResult<InterfaceMethodDef> {
        check_token!(self, RawToken::Fun, "interface method definition")?;

        self.advance()?; // 'fun'

        check_token0!(
            self,
            "identifier for method name",
            RawToken::Identifier(_),
            "interface method definition"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // name

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenParent, "interface method definition")?;

        self.advance()?; // '('

        let arguments = parse_list!(
            self,
            "interface method arguments",
            &RawToken::CloseParent,
            false,
            || self.parse_function_argument()
        );

        let mut return_type = None;

        if !self.current.value.is(&RawToken::Semicolon) {
            return_type = Some(self.parse_type()?);
        }

        check_token!(self, RawToken::Semicolon, "interface method definition")?;

        self.advance()?; // ';'

        Ok(InterfaceMethodDef {
            name,
            generic_annotations,
            params: arguments,
            return_type,
        })
    }
}
