use crate::ast::location::*;
use crate::ast::token::*;
use crate::ast::*;
use crate::Lexer;
use ariadne::Fmt;
use ariadne::Source;
use ariadne::{ColorGenerator, Label, Report, ReportKind};
use phf::phf_map;
use std::mem;

type prefixParseFunction = fn() -> Expression;
type infixParseFunction = fn(Expression) -> Expression;

pub static PREFIX_PARSE_FUNCTIONS: phf::Map<&'static str, prefixParseFunction> = phf_map! {};
pub static INFIX_PARSE_FUNCTIONS: phf::Map<&'static str, infixParseFunction> = phf_map! {};

pub struct Parser<'a> {
    filename: &'a str,
    contents: &'a str,

    lexer: Lexer<'a>,

    current: Token<'a>,
    peek: Token<'a>,
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
        if let RawToken::Invalid(e) = self.current.raw {
            Report::build(ReportKind::Error, self.filename, 0)
                .with_code(0)
                .with_message("scanning error".to_owned())
                .with_label(
                    Label::new((self.filename, self.current.span.range.clone()))
                        .with_message(e.to_string())
                        .with_color(ColorGenerator::new().next()),
                )
                .finish()
                .print((self.filename, Source::from(self.contents)))
                .unwrap();
        }
    }

    #[inline]
    pub fn advance(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_no_comments().unwrap();

        self.check_scanning_error();
    }

    pub fn parse(&mut self) -> Option<ProgramUnit<'a>> {
        let namespace = self.parse_namespace_declaration()?;
        // let imports = self.parse_imports();

        Some(ProgramUnit {
            namespace,
            imports: self.parse_imports()?,
            top_level_statements: vec![],
        })
    }

    fn parse_namespace_declaration(&mut self) -> Option<Namespace<'a>> {
        self.check_current_token(RawToken::Namespace, name!("namespace declaration"))?;

        self.advance(); // namespace

        self.check_current_token(
            RawToken::Identifier("".to_owned()),
            name!("namespace declaration"),
        )?;

        let namespace = self.parse_name()?;

        self.check_current_token(RawToken::Semicolon, name!("namespace declaration"))?;

        self.advance(); // ';'

        Some(Namespace { namespace })
    }

    fn parse_imports(&mut self) -> Option<Vec<Spanned<'a, Import<'a>>>> {
        let mut imports = vec![];

        while let RawToken::Import = &self.current.raw {
            let start = self.current.span.range.start;
            let import = self.parse_import()?;
            let end = self.current.span.range.end;

            let span = Span::new(self.filename, start, end);

            imports.push(Spanned::new(import, span));
        }

        Some(imports)
    }

    fn parse_import(&mut self) -> Option<Import<'a>> {
        self.advance(); // import

        self.check_current_token(RawToken::String("".to_owned()), name!("import"))?;

        let span = self.current.span.clone();

        let mut value = String::from("");

        if let RawToken::String(s) = &self.current.raw {
            value.push_str(s)
        }

        self.advance(); // "name"

        self.check_current_token(RawToken::Semicolon, name!("import"))?;

        self.advance(); // ';'

        Some(Import {
            filename: Spanned::new(value.to_owned(), span),
        })
    }

    fn parse_name(&mut self) -> Option<Spanned<'a, String>> {
        let start = self.current.span.range.start;

        let mut name = String::from("");

        if let RawToken::Identifier(n) = &self.current.raw {
            name.push_str(n.as_str());
            name.push(':');
        }

        self.advance(); // id

        while let RawToken::Colon = &self.current.raw {
            self.advance(); // ':'

            self.check_current_token(RawToken::Identifier("".to_owned()), name!("name"))?;

            if let RawToken::Identifier(s) = &self.current.raw {
                name.push_str(s.as_str());
                name.push(':');
            }

            self.advance();
        }

        let end = self.current.span.range.end;

        name.pop();

        Some(Spanned::new(name, Span::new(self.filename, start, end)))
    }

    fn parse_primary_type(&mut self) -> Type {
        todo!()
    }

    fn parse_custom_type(&mut self) -> Type {
        todo!()
    }

    fn parse_array_type(&mut self) -> Type {
        todo!()
    }

    fn parse_pointer_type(&mut self) -> Type {
        todo!()
    }

    #[inline]
    fn check_current_token(
        &mut self,
        expected: RawToken,
        expected_for: Option<String>,
    ) -> Option<()> {
        let c = ColorGenerator::new().next();

        if mem::discriminant(&self.current.raw) != mem::discriminant(&expected) {
            let mut label_message = format!("expected {}", expected.to_string().fg(c));

            if let Some(_) = expected_for {
                label_message.push_str(format!(" for `{}`", expected_for.unwrap().fg(c)).as_str());
            }

            Report::build(ReportKind::Error, self.filename, 0)
                .with_code(1)
                .with_message(format!("unexpected {}", self.current.raw))
                .with_label(
                    Label::new((self.filename, self.current.span.range.clone()))
                        .with_message(label_message)
                        .with_color(c),
                )
                .finish()
                .print((self.filename, Source::from(self.contents)))
                .unwrap();

            None
        } else {
            Some(())
        }
    }

    #[inline]
    fn check_peek_token(&mut self, expected: RawToken, expected_for: Option<String>) -> Option<()> {
        let c = ColorGenerator::new().next();

        if mem::discriminant(&self.peek.raw) != mem::discriminant(&expected) {
            let mut label_message = format!("expected {}", expected.to_string().fg(c));

            if let Some(_) = expected_for {
                label_message.push_str(format!(" for {}", expected_for.unwrap()).as_str());
            }

            Report::build(ReportKind::Error, self.filename, 0)
                .with_code(1)
                .with_message(format!("unexpected {}", self.peek.raw))
                .with_label(
                    Label::new((self.filename, self.peek.span.range.clone()))
                        .with_message(label_message)
                        .with_color(c),
                )
                .finish()
                .print((self.filename, Source::from(self.contents)))
                .unwrap();

            None
        } else {
            Some(())
        }
    }
}
