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

        let name = self.get_name();

        self.advance()?; // 'name'

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenBrace, "struct declaration")?;

        self.advance0()?; // '{'

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

        if self.current.value.is(RawToken::Pub) {
            public = Some(self.current.span);
            self.advance()?;
        }

        check_token0!(
            self,
            "identifier for struct member name or '}'",
            RawToken::Identifier(_),
            "struct definition"
        )?;

        let name = self.get_name();

        self.advance()?;

        let r#type = self.parse_type()?;

        check_token!(self, RawToken::Semicolon, "struct member definition")?;

        self.advance0()?; // ';'

        Ok(StructMemberDef {
            public,
            name,
            r#type,
        })
    }

    fn parse_struct_members(&mut self) -> ParserResult<Vec<(String, StructMemberDef)>> {
        let mut members = vec![];

        while !self.current.value.is(RawToken::CloseBrace) {
            members.push((self.consume_local_docstring()?, self.parse_struct_member()?));
        }

        Ok(members)
    }
}
