use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::token::RawToken;
use ry_ast::*;

impl<'c> Parser<'c> {
    pub(crate) fn parse_impl(&mut self) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'impl'

        check_token0!(
            self,
            "identifier for implemented type name",
            RawToken::Identifier(_),
            "type implementation"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // 'name'

        let generic_annotations = self.parse_generic_annotations()?;

        if self.current.value.is(&RawToken::Colon) {
            self.advance()?;
        }

        check_token!(self, RawToken::OpenBrace, "type implementation")?;

        self.advance()?; // '{'

        // let methods = self.parse_interface_method_definitions(name_span.clone())?;

        check_token!(self, RawToken::CloseBrace, "type implementation")?;

        self.advance0()?; // '}'

        Ok(TopLevelStatement::Impl(Impl {
            for_what: (name, generic_annotations),
            impl_what: vec![],
            methods: vec![],
        }))
    }
}
