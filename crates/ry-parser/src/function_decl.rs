use crate::{error::ParserError, macros::*, Parser, ParserResult};

use num_traits::ToPrimitive;

use ry_ast::{location::Span, precedence::Precedence, token::RawToken, *};

impl<'c> Parser<'c> {
    pub(crate) fn parse_function_declaration(
        &mut self,
        public: Option<Span>,
    ) -> ParserResult<TopLevelStatement> {
        self.advance()?; // 'fun'

        check_token0!(
            self,
            "identifier for function name",
            RawToken::Identifier(_),
            "function declaration"
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // name

        let generic_annotations = self.parse_generic_annotations()?;

        check_token!(self, RawToken::OpenParent, "function declaration")?;

        self.advance()?; // '('

        let arguments = parse_list_of_smth!(self, &RawToken::CloseParent, || self
            .parse_function_argument());

        self.advance()?; // ')'

        let mut return_type = None;

        if !self.current.value.is(&RawToken::OpenBrace) {
            return_type = Some(self.parse_type(true, false)?);
        }

        let stmts = self.parse_statements_block(true)?;

        Ok(TopLevelStatement::FunctionDecl(FunctionDecl {
            def: FunctionDef {
                name,
                generic_annotations,
                params: arguments,
                public,
                return_type,
            },
            stmts,
        }))
    }

    pub(crate) fn parse_function_argument(&mut self) -> ParserResult<FunctionParam> {
        check_token0!(
            self,
            "identifier for argument name",
            RawToken::Identifier(_)
        )?;

        let name = (
            self.current.value.ident().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // name

        let ty = self.parse_type(false, false)?;

        let mut default_value = None;

        // argument's default value
        if self.current.value.is(&RawToken::Assign) {
            self.advance()?;

            default_value = Some(self.parse_expression(Precedence::Lowest.to_i8().unwrap())?);
        }

        Ok(FunctionParam {
            name,
            ty,
            default_value,
        })
    }
}
