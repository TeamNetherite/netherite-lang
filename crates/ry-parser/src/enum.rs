use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::*;
use ry_ast::{location::Span, token::RawToken};

impl<'c> Parser<'c> {
    pub(crate) fn parse_enum_declaration(
        &mut self,
        public: Option<Span>,
    ) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'enum'

        check_token0!(
            self,
            "identifier for enum name",
            RawToken::Identifier(_),
            "enum declaration"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // 'name'

        check_token!(self, RawToken::OpenBrace, "enum declaration")?;

        self.advance()?; // '{'

        let mut variants = vec![];

        parse_list_of_smth!(self, variants, &RawToken::CloseBrace, || {
            check_token0!(self, "identifier", RawToken::Identifier(_), "enum variant")?;

            let variant = (
                self.current.value.ident().unwrap(),
                self.current.span.clone(),
            )
                .into();

            self.advance()?; // id

            Ok(variant)
        });

        check_token!(self, RawToken::CloseBrace, "enum declaration")?;

        self.advance0()?; // '}'

        Ok(TopLevelStatement::EnumDecl(EnumDecl {
            public,
            name,
            variants,
        }))
    }
}
