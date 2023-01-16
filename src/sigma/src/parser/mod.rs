use crate::ast::location::*;
use crate::ast::token::*;
use crate::ast::*;
use crate::lexer::Lexer;
use ariadne::Fmt;
use ariadne::Source;
use ariadne::{ColorGenerator, Label, Report, ReportKind};
use phf::phf_map;
use std::mem;

pub static PREFIX_PARSE_FUNCTIONS: phf::Map<&'static str, fn() -> Expression> = phf_map! {};
pub static INFIX_PARSE_FUNCTIONS: phf::Map<&'static str, fn(Expression) -> Expression> =
    phf_map! {};

pub struct Parser<'a> {
    filename: &'a str,
    contents: &'a str,

    lexer: Lexer<'a>,

    current: Token<'a>,
    peek: Token<'a>,
}

macro_rules! unwrap_enum {
    ($target: expr, $path: path) => {{
        if let $path(a) = $target {
            a
        } else {
            unreachable!();
        }
    }};
}

macro_rules! name {
    ($s: expr) => {
        Some($s.to_owned())
    };
}

impl<'a> Parser<'a> {
    pub fn new(filename: &'a str, contents: &'a str) -> Self {
        let mut lexer = Lexer::new(filename, contents);

        let current = lexer.next_no_comments().unwrap();
        let peek = lexer.next_no_comments().unwrap();

        let parser = Self {
            filename,
            contents,
            lexer,
            current,
            peek,
        };

        parser.check_scanning_error();

        parser
    }

    pub fn check_scanning_error(&self) {
        if let RawToken::Invalid(e) = self.current.value {
            Report::build(
                ReportKind::Error,
                self.filename,
                self.current.span.range.start,
            )
            .with_code(0)
            .with_message("scanning error".to_owned())
            .with_label(
                Label::new((self.filename, self.current.span.range.to_owned()))
                    .with_message(e.to_string())
                    .with_color(ColorGenerator::new().next()),
            )
            .finish()
            .print((self.filename, Source::from(self.contents.to_owned() + " ")))
            .unwrap();
        }
    }

    #[inline]
    pub fn advance(&mut self) {
        self.current = self.peek.to_owned();
        self.peek = self.lexer.next_no_comments().unwrap();

        self.check_scanning_error();
    }

    pub fn parse(&mut self) -> Option<ProgramUnit<'a>> {
        let namespace = self.parse_namespace_declaration()?;
        // let imports = self.parse_imports();

        Some(ProgramUnit {
            namespace,
            imports: self.parse_imports()?,
            top_level_statements: self.parse_top_level_statements()?,
        })
    }

    fn parse_namespace_declaration(&mut self) -> Option<WithSpan<'a, String>> {
        self.check_current_token(RawToken::Namespace, name!("namespace declaration"))?;

        self.advance(); // namespace

        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("namespace declaration"),
        )?;

        let namespace = self.parse_name()?;

        self.check_current_token(RawToken::Semicolon, name!("namespace declaration"))?;

        self.advance(); // ';'

        Some(namespace)
    }

    fn parse_imports(&mut self) -> Option<Vec<WithSpan<'a, Import<'a>>>> {
        let mut imports = vec![];

        while let RawToken::Import = &self.current.value {
            let start = self.current.span.range.start;
            let import = self.parse_import()?;
            let end = self.current.span.range.end;

            let span = Span::new(self.filename, start, end);

            imports.push(WithSpan::new(import, span));
        }

        Some(imports)
    }

    fn parse_import(&mut self) -> Option<Import<'a>> {
        self.advance(); // import

        self.check_current_token(RawToken::String("".to_owned()), name!("import"))?;

        let span = self.current.span.clone();

        let value = unwrap_enum!(&self.current.value, RawToken::String).to_owned();

        self.advance(); // "name"

        self.check_current_token(RawToken::Semicolon, name!("import"))?;

        self.advance(); // ';'

        Some(Import {
            filename: WithSpan::new(value.to_owned(), span),
        })
    }

    fn parse_top_level_statements(&mut self) -> Option<Vec<WithSpan<'a, TopLevelStatement<'a>>>> {
        let mut top_level_statements = vec![];

        loop {
            match &self.current.value {
                &RawToken::Fun => {
                    top_level_statements.push(self.parse_function_declaration(false)?);
                }
                &RawToken::Struct => {
                    top_level_statements.push(self.parse_struct_declaration(false)?);
                }
                &RawToken::Pub => match &self.peek.value {
                    &RawToken::Fun => {
                        self.advance();
                        top_level_statements.push(self.parse_function_declaration(true)?);
                    }
                    &RawToken::Struct => {
                        self.advance();
                        top_level_statements.push(self.parse_struct_declaration(true)?);
                    }
                    _ => {
                        self.unexpected_token_error(
                            true,
                            &[RawToken::Fun, RawToken::Struct],
                            name!("top level declaration"),
                        );
                        self.advance();
                        return None;
                    }
                },
                &RawToken::EndOfFile => break,
                _ => {
                    println!("ok");
                    self.unexpected_token_error(
                        false,
                        &[RawToken::Fun, RawToken::Struct, RawToken::Pub],
                        name!("top level declaration"),
                    );
                    self.advance();
                    return None;
                }
            }
        }

        Some(top_level_statements)
    }

    fn parse_struct_declaration(
        &mut self,
        public: bool,
    ) -> Option<WithSpan<'a, TopLevelStatement<'a>>> {
        let start = self.current.span.range.start;

        self.advance(); // 'struct'

        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("struct declaration"),
        )?;

        let name = unwrap_enum!(&self.current.value, RawToken::Identifier).to_owned();
        let name_span = self.current.span.clone();

        self.advance(); // 'name'

        self.check_current_token(RawToken::OpenBrace, name!("struct declaration"))?;

        self.advance(); // '{'

        let members = self.parse_struct_members()?;

        self.check_current_token(RawToken::CloseBrace, name!("struct declaration"))?;

        self.advance(); // '}'

        let end = self.current.span.range.end;

        Some(WithSpan::new(
            TopLevelStatement::StructDeclaration(StructDeclaration {
                public,
                name: WithSpan::new(name, name_span),
                members,
            }),
            Span::new(self.filename, start, end),
        ))
    }

    fn parse_struct_member(&mut self) -> Option<WithSpan<'a, StructMember<'a>>> {
        let start = self.current.span.range.start;

        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("structure member declaration"),
        )?;

        let name = unwrap_enum!(&self.current.value, RawToken::Identifier).to_owned();
        let name_span = self.current.span.to_owned();

        let end = self.current.span.range.end;

        None
    }

    fn parse_struct_members(&mut self) -> Option<Vec<WithSpan<'a, StructMember<'a>>>> {
        None
    }

    fn parse_struct_methods(&mut self) -> Option<Vec<FunctionDeclaration<'a>>> {
        None
    }

    fn parse_function_declaration(
        &mut self,
        public: bool,
    ) -> Option<WithSpan<'a, TopLevelStatement<'a>>> {
        let start = self.current.span.range.start;

        self.advance(); // 'fun'

        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("function declaration"),
        )?;

        let name = unwrap_enum!(&self.current.value, RawToken::Identifier).to_owned();
        let name_span = self.current.span.clone();

        self.advance(); // name

        self.check_current_token(RawToken::OpenParent, name!("function declaration"))?;

        self.advance(); // '('

        let arguments = self.parse_function_arguments()?;

        self.check_current_token(RawToken::CloseParent, name!("function declaration"))?;

        self.advance(); // ')'

        let mut return_type = None;

        if std::mem::discriminant(&self.current.value)
            != std::mem::discriminant(&RawToken::OpenBrace)
        {
            return_type = Some(self.parse_type(true)?);
        }

        self.check_current_token(RawToken::OpenBrace, name!("function declaration"))?;

        self.advance(); // '{'

        self.check_current_token(RawToken::CloseBrace, name!("function declaration"))?;

        self.advance(); // '}'

        let end = self.current.span.range.end;

        Some(WithSpan::new(
            TopLevelStatement::FunctionDeclaration(FunctionDeclaration {
                name: WithSpan::new(name, name_span),
                params: arguments,
                public,
                return_type,
            }),
            Span::new(self.filename, start, end),
        ))
    }

    fn parse_function_arguments(&mut self) -> Option<Vec<WithSpan<'a, FunctionParam<'a>>>> {
        let mut arguments = vec![];

        if std::mem::discriminant(&self.current.value)
            == std::mem::discriminant(&RawToken::CloseParent)
        {
            return Some(arguments);
        }

        loop {
            let start = self.current.span.range.start;
            let arg = self.parse_function_argument()?;
            let end = self.current.span.range.end;
            arguments.push(WithSpan::new(arg, Span::new(self.filename, start, end)));

            if std::mem::discriminant(&self.current.value)
                != std::mem::discriminant(&RawToken::Comma)
            {
                return Some(arguments);
            }

            self.advance();
        }
    }

    fn parse_function_argument(&mut self) -> Option<FunctionParam<'a>> {
        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("function argument"),
        )?;
        let name = unwrap_enum!(&self.current.value, RawToken::Identifier).to_owned();
        let span = self.current.span.to_owned();
        self.advance(); // name

        let ty = self.parse_type(false)?;

        Some(FunctionParam {
            name: WithSpan::new(name, span),
            ty,
        })
    }

    fn parse_name(&mut self) -> Option<WithSpan<'a, String>> {
        let start = self.current.span.range.start;

        let mut name = unwrap_enum!(&self.current.value, RawToken::Identifier).to_owned();
        name.push(':');

        self.advance(); // id

        while let RawToken::Colon = &self.current.value {
            self.advance(); // ':'

            self.check_current_token(RawToken::Identifier("".to_owned()), name!("name"))?;

            name.push_str(unwrap_enum!(&self.current.value, RawToken::Identifier));
            name.push(':');

            self.advance();
        }

        let end = self.current.span.range.end;

        name.pop();

        Some(WithSpan::new(name, Span::new(self.filename, start, end)))
    }

    fn parse_type(&mut self, return_type: bool) -> Option<Type<'a>> {
        match &self.current.value {
            RawToken::Identifier(_) => self.parse_custom_type(),
            RawToken::Asterisk => self.parse_pointer_type(),
            RawToken::OpenBracket => self.parse_array_type(),
            RawToken::PrimaryType(t) => {
                let r = Some(Type::PrimaryType(WithSpan::new(
                    *t,
                    self.current.span.to_owned(),
                )));
                self.advance();
                r
            }
            _ => {
                let mut _message = "";

                if return_type {
                    _message = "expected identifier, '*', '[', primary type, '{'";
                } else {
                    _message = "expected identifier, '*', '[', primary type";
                }

                Report::build(ReportKind::Error, self.filename, 0)
                    .with_code(1)
                    .with_message(format!("unexpected {}", self.current.value))
                    .with_label(
                        Label::new((self.filename, self.current.span.range.to_owned()))
                            .with_message(_message.to_owned())
                            .with_color(ColorGenerator::new().next()),
                    )
                    .finish()
                    .print((self.filename, Source::from(self.contents)))
                    .unwrap();

                None
            }
        }
    }

    fn parse_custom_type(&mut self) -> Option<Type<'a>> {
        let name = self.parse_name()?;
        Some(Type::CustomType(name))
    }

    fn parse_array_type(&mut self) -> Option<Type<'a>> {
        let start = self.current.span.range.start;

        self.advance(); // '['

        self.check_current_token(RawToken::CloseBracket, name!("array type"))?;
        self.advance(); // ']'

        let inner_type = Box::new(self.parse_type(false)?);

        let end = self.current.span.range.end;

        Some(Type::ArrayType(WithSpan::new(
            inner_type,
            Span::new(self.filename, start, end),
        )))
    }

    fn parse_pointer_type(&mut self) -> Option<Type<'a>> {
        let start = self.current.span.range.start;

        self.advance(); // '*'

        let inner_type = Box::new(self.parse_type(false)?);

        let end = self.current.span.range.end;

        Some(Type::PointerType(WithSpan::new(
            inner_type,
            Span::new(self.filename, start, end),
        )))
    }

    fn unexpected_token_error(
        &mut self,
        peek: bool,
        expected: &[RawToken],
        expected_for: Option<String>,
    ) {
        let c = ColorGenerator::new().next();

        let mut label_message = format!(
            "expected {}",
            expected
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .fg(c)
        );

        if let Some(_) = expected_for {
            label_message.push_str(format!(" for `{}`", expected_for.unwrap().fg(c)).as_str());
        }

        if peek {
            Report::build(ReportKind::Error, self.filename, self.peek.span.range.start)
                .with_code(1)
                .with_message(format!("unexpected {}", self.peek.value))
                .with_label(
                    Label::new((self.filename, self.peek.span.range.to_owned()))
                        .with_message(label_message)
                        .with_color(c),
                )
                .finish()
                .print((self.filename, Source::from(self.contents.to_owned() + " ")))
                .unwrap();
        } else {
            Report::build(
                ReportKind::Error,
                self.filename,
                self.current.span.range.start,
            )
            .with_code(1)
            .with_message(format!("unexpected {}", self.current.value))
            .with_label(
                Label::new((self.filename, self.current.span.range.to_owned()))
                    .with_message(label_message)
                    .with_color(c),
            )
            .finish()
            .print((self.filename, Source::from(self.contents.to_owned() + " ")))
            .unwrap();
        }
    }

    fn check_current_token(
        &mut self,
        expected: RawToken,
        expected_for: Option<String>,
    ) -> Option<()> {
        let c = ColorGenerator::new().next();

        if mem::discriminant(&self.current.value) != mem::discriminant(&expected) {
            let mut label_message = format!("expected {}", expected.to_string().fg(c));

            if let Some(_) = expected_for {
                label_message.push_str(format!(" for `{}`", expected_for.unwrap().fg(c)).as_str());
            }

            Report::build(
                ReportKind::Error,
                self.filename,
                self.current.span.range.start,
            )
            .with_code(1)
            .with_message(format!("unexpected {}", self.current.value))
            .with_label(
                Label::new((self.filename, self.current.span.range.to_owned()))
                    .with_message(label_message)
                    .with_color(c),
            )
            .finish()
            .print((self.filename, Source::from(self.contents.to_owned() + " ")))
            .unwrap();

            None
        } else {
            Some(())
        }
    }

    fn check_peek_token(&mut self, expected: RawToken, expected_for: Option<String>) -> Option<()> {
        let c = ColorGenerator::new().next();

        if mem::discriminant(&self.peek.value) != mem::discriminant(&expected) {
            let mut label_message = format!("expected {}", expected.to_string().fg(c));

            if let Some(_) = expected_for {
                label_message.push_str(format!(" for {}", expected_for.unwrap()).as_str());
            }

            Report::build(ReportKind::Error, self.filename, self.peek.span.range.start)
                .with_code(1)
                .with_message(format!("unexpected {}", self.peek.value))
                .with_label(
                    Label::new((self.filename, self.peek.span.range.to_owned()))
                        .with_message(label_message)
                        .with_color(c),
                )
                .finish()
                .print((self.filename, Source::from(self.contents.to_owned() + " ")))
                .unwrap();

            None
        } else {
            Some(())
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::location::*;
    use crate::ast::token::*;
    use crate::ast::*;
    use crate::parser::Parser;

    macro_rules! def_p {
        ($p: ident, $contents: expr) => {
            let mut $p = Parser::new("<test>", $contents);
        };
    }

    #[test]
    fn namespace_test() {
        def_p!(p, "namespace test;");
        assert_eq!(
            p.parse(),
            Some(ProgramUnit {
                namespace: WithSpan {
                    value: "test".to_owned(),
                    span: Span {
                        filename: "<test>",
                        range: 10..15
                    }
                },
                imports: vec![],
                top_level_statements: vec![]
            })
        )
    }

    #[test]
    fn namespace2_test() {
        def_p!(p, "namespace test:test2:test3;");
        assert_eq!(
            p.parse(),
            Some(ProgramUnit {
                namespace: WithSpan {
                    value: "test:test2:test3".to_owned(),
                    span: Span {
                        filename: "<test>",
                        range: 10..27
                    }
                },
                imports: vec![],
                top_level_statements: vec![]
            })
        )
    }

    #[test]
    fn import_test() {
        def_p!(p, "namespace test;\nimport \"test\";\nimport \"test2\";\n");
        assert_eq!(
            p.parse(),
            Some(ProgramUnit {
                namespace: WithSpan {
                    value: "test".to_owned(),
                    span: Span {
                        filename: "<test>",
                        range: 10..15
                    }
                },
                imports: vec![
                    WithSpan {
                        value: Import {
                            filename: WithSpan {
                                value: "test".to_owned(),
                                span: Span {
                                    filename: "<test>",
                                    range: 23..29
                                }
                            }
                        },
                        span: Span {
                            filename: "<test>",
                            range: 16..37
                        }
                    },
                    WithSpan {
                        value: Import {
                            filename: WithSpan {
                                value: "test2".to_owned(),
                                span: Span {
                                    filename: "<test>",
                                    range: 38..45
                                }
                            }
                        },
                        span: Span {
                            filename: "<test>",
                            range: 31..48
                        }
                    }
                ],
                top_level_statements: vec![]
            })
        )
    }

    #[test]
    fn function_decl_test() {
        def_p!(p, "namespace main;\npub fun main() {}");
        assert_eq!(
            p.parse(),
            Some(ProgramUnit {
                namespace: WithSpan {
                    value: "main".to_owned(),
                    span: Span {
                        filename: "<test>",
                        range: 10..15
                    }
                },
                imports: vec![],
                top_level_statements: vec![WithSpan {
                    value: TopLevelStatement::FunctionDeclaration(FunctionDeclaration {
                        public: true,
                        name: WithSpan {
                            value: "main".to_owned(),
                            span: Span {
                                filename: "<test>",
                                range: 24..28
                            }
                        },
                        params: vec![],
                        return_type: None
                    }),
                    span: Span {
                        filename: "<test>",
                        range: 20..34
                    }
                }]
            })
        )
    }

    #[test]
    fn function_decl2_test() {
        def_p!(p, "namespace main;\npub fun sum(a i32, b i32) i32 {}");
        assert_eq!(
            p.parse(),
            Some(ProgramUnit {
                namespace: WithSpan {
                    value: "main".to_owned(),
                    span: Span {
                        filename: "<test>",
                        range: 10..15
                    }
                },
                imports: vec![],
                top_level_statements: vec![WithSpan {
                    value: TopLevelStatement::FunctionDeclaration(FunctionDeclaration {
                        public: true,
                        name: WithSpan {
                            value: "sum".to_owned(),
                            span: Span {
                                filename: "<test>",
                                range: 24..27
                            }
                        },
                        params: vec![
                            WithSpan {
                                value: FunctionParam {
                                    name: WithSpan {
                                        value: "a".to_owned(),
                                        span: Span {
                                            filename: "<test>",
                                            range: 28..29
                                        }
                                    },
                                    ty: Type::PrimaryType(WithSpan {
                                        value: PrimaryType::I32,
                                        span: Span {
                                            filename: "<test>",
                                            range: 30..33
                                        }
                                    })
                                },
                                span: Span {
                                    filename: "<test>",
                                    range: 28..34
                                }
                            },
                            WithSpan {
                                value: FunctionParam {
                                    name: WithSpan {
                                        value: "b".to_owned(),
                                        span: Span {
                                            filename: "<test>",
                                            range: 35..36
                                        }
                                    },
                                    ty: Type::PrimaryType(WithSpan {
                                        value: PrimaryType::I32,
                                        span: Span {
                                            filename: "<test>",
                                            range: 37..40
                                        }
                                    })
                                },
                                span: Span {
                                    filename: "<test>",
                                    range: 35..41
                                }
                            }
                        ],
                        return_type: Some(Type::PrimaryType(WithSpan {
                            value: PrimaryType::I32,
                            span: Span {
                                filename: "<test>",
                                range: 42..45
                            }
                        }))
                    }),
                    span: Span {
                        filename: "<test>",
                        range: 20..49
                    }
                }]
            })
        )
    }
}
