use crate::{error::ParserError, macros::*, Parser, ParserResult};

use ry_ast::token::RawToken;
use ry_ast::*;

impl<'c> Parser<'c> {
    /// TODO: fix the problem with comments and imports messed up
    pub(crate) fn parse_imports(&mut self) -> ParserResult<Vec<Import>> {
        let mut imports = vec![];

        while self.current.value.is(&RawToken::Import) {
            imports.push(self.parse_import()?);
            self.advance()?; // ';'
        }

        Ok(imports)
    }

    pub(crate) fn parse_import(&mut self) -> ParserResult<Import> {
        self.advance()?; // import

        check_token0!(self, "string for filepath", RawToken::String(_), "import")?;

        let filename = (
            self.current.value.string().unwrap(),
            self.current.span.clone(),
        )
            .into();

        self.advance()?; // "name"

        check_token!(self, RawToken::Semicolon, "import")?;

        Ok(Import { filename })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;
    use ry_ast::{
        location::{Span, WithSpan},
        Import,
    };

    #[test]
    pub fn single_import_test() {
        let contents = String::from("import \"test\";");
        let mut parser = Parser::new(&contents);
        let imports = parser.parse_imports();
        assert!(imports.is_ok());
        assert_eq!(
            vec![Import {
                filename: WithSpan {
                    value: "test".to_owned(),
                    span: Span { range: 7..13 }
                }
            }],
            imports.ok().unwrap()
        );
    }

    #[test]
    pub fn more_imports_test() {
        let contents = String::from("import \"test\"; import \"test2\"; import \"test3\";");
        let mut parser = Parser::new(&contents);
        let imports = parser.parse_imports();
        assert!(imports.is_ok());
        assert_eq!(
            vec![
                Import {
                    filename: WithSpan {
                        value: "test".to_owned(),
                        span: Span { range: 7..13 }
                    }
                },
                Import {
                    filename: WithSpan {
                        value: "test2".to_owned(),
                        span: Span { range: 22..29 }
                    }
                },
                Import {
                    filename: WithSpan {
                        value: "test3".to_owned(),
                        span: Span { range: 38..45 }
                    }
                }
            ],
            imports.ok().unwrap()
        );
    }
}
