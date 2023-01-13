use crate::ast::location::Spanned;
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

        Self {
            filename,
            contents,
            lexer,
            current,
            peek,
        }
    }

    pub fn advance(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_no_comments().unwrap();
    }

    pub fn parse(&mut self) -> Option<ProgramUnit<'a>> {
        let namespace = self.parse_namespace_declaration()?;
        let imports = self.parse_imports();

        Some(ProgramUnit {
            namespace,
            imports: imports,
            top_level_statements: vec![],
        })
    }

    fn parse_namespace_declaration(&mut self) -> Option<Namespace<'a>> {
        self.check_current_token(RawToken::Namespace, name!("namespace declaration"))?;
        self.advance();
        self.check_current_token(
            RawToken::String("".to_owned()),
            name!("namespace declaration"),
        )?;
        self.advance();
        self.check_current_token(RawToken::Semicolon, name!("namespace declaration"))?;

        None
    }

    fn parse_imports(&mut self) -> Vec<Spanned<'a, Import<'a>>> {
        todo!()
    }

    fn parse_import(&mut self) -> Option<Spanned<'a, Import<'a>>> {
        todo!()
    }

    fn parse_name(&mut self) -> String {
        todo!()
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
                label_message.push_str(format!(" for {}", expected_for.unwrap().fg(c)).as_str());
            }

            Report::build(ReportKind::Error, self.filename, 0)
                .with_code(1)
                .with_message(format!("unexpected {}", self.current.raw))
                .with_label(
                    Label::new((
                        self.filename,
                        self.current.span.range.start..self.current.span.range.end,
                    ))
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
                    Label::new((
                        self.filename,
                        self.peek.span.range.start..self.peek.span.range.end,
                    ))
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
