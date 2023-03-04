use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::*;
use ry_ast::{location::*, token::*};

impl<'c> Parser<'c> {
    pub(crate) fn parse_struct_declaration(
        &mut self,
        public: Option<Span>,
    ) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'struct'

        check_token0!(
            self,
            "identifier for struct name",
            RawToken::Identifier(_),
            "struct declaration"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // 'name'

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenBrace, "struct declaration")?;

        self.advance()?; // '{'

        let members = self.parse_struct_members()?;

        check_token!(self, RawToken::CloseBrace, "struct declaration")?;

        self.advance0()?; // '}'

        Ok(TopLevelStatement::StructDecl(StructDecl {
            generic_annotations,
            public,
            name,
            members,
        }))
    }

    fn parse_struct_member(&mut self) -> ParserResult<StructMemberDef> {
        let mut public = None;

        if self.current.value.is(&RawToken::Pub) {
            public = Some(self.current.span.clone());
            self.advance()?;
        }

        check_token0!(
            self,
            "identifier for struct member name or '}'",
            RawToken::Identifier(_),
            "struct definition"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?;

        let ty = self.parse_type()?;

        check_token!(self, RawToken::Semicolon, "struct member definition")?;

        self.advance()?; // ';'

        Ok(StructMemberDef { public, name, ty })
    }

    fn parse_struct_members(&mut self) -> ParserResult<Vec<StructMemberDef>> {
        let mut members = vec![];

        while !self.current.value.is(&RawToken::CloseBrace) {
            members.push(self.parse_struct_member()?);
        }

        Ok(members)
    }
}
