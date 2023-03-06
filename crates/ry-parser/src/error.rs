use codespan_reporting::diagnostic::{Diagnostic, Label};

use ry_ast::location::*;
use ry_ast::token::{LexerError, RawToken, Token};
use ry_report::Reporter;

#[derive(Debug)]
pub enum ParserError {
    /// Error appeared in lexing stage.
    ErrorToken(WithSpan<LexerError>),

    /// Unexpected token [`Token`] in AST Node called
    /// [`Option<String>`], expected [`String`].
    UnexpectedToken(Token, String, Option<String>),

    /// Unexpected token [`Token`], expected [`RawToken`]
    /// in AST Node called [`String`]  if [`Option<String>`]
    /// is `Some(String)`.
    UnexpectedTokenExpectedX(Token, RawToken, Option<String>),

    /// Appears when you try to define trait method as public.
    /// 1-st [`Span`] is location of `pub` keyword.
    /// 2-nd [`Span`] is location of method name.
    /// 3-rd [`Span`] is location of trait name in which method is defined.
    UnnecessaryVisibilityQualifier(Span, Span, Span),

    /// Appears when `import` keyword is found after top level statement(-s)
    /// [`Span`] here is location of `import` statement.
    ImportAfterTopLevelStatement(Span),
}

impl<'source> Reporter<'source> for ParserError {
    fn build_diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        match self {
            Self::ErrorToken(ref t) => Diagnostic::error()
                .with_message("scanning error occured")
                .with_code("E000")
                .with_labels(vec![
                    Label::primary(file_id, t.span.range.clone()).with_message(t.value.to_string())
                ]),
            Self::UnexpectedToken(ref got, ref expected, ref node_name) => {
                let mut label_message = format!("expected {expected}");

                if let Some(node_name_s) = node_name {
                    label_message.push_str(format!(" in {node_name_s}").as_str());
                }

                Diagnostic::error()
                    .with_message(format!("unexpected {}", got.value))
                    .with_code("E001")
                    .with_labels(vec![
                        Label::primary(file_id, got.span.range.clone()).with_message(label_message)
                    ])
            }
            Self::UnnecessaryVisibilityQualifier(ref pub_span, ref method_name_span ,ref trait_name_span) => {
                Diagnostic::error()
                    .with_message(
                        "unnecessary visibility qualifier in trait method definition".to_owned(),
                    )
                    .with_labels(vec![
                        Label::primary(file_id, pub_span.range.clone())
                            .with_message("consider removing `pub`"),
                        Label::secondary(file_id, method_name_span.range.clone())
                            .with_message("in this method definition"),
                        Label::secondary(file_id, trait_name_span.range.clone())
                            .with_message("method is defined in this trait"),
                    ])
                    .with_code("E002")
                    .with_notes(vec![
                        "note: methods in trait are by default public. this is why their declarations\ndo not require using `pub` keyword in the beginning".to_owned()
                    ])
            }
            Self::UnexpectedTokenExpectedX(ref got, ref expected, ref node_name) => {
                let mut label_message = format!("expected {expected}");

                if let Some(node_name_s) = node_name {
                    label_message.push_str(format!(" for {node_name_s}").as_str());
                }

                Diagnostic::error()
                    .with_message(format!("expected {}, found {}", expected, got.value))
                    .with_code("E001")
                    .with_labels(vec![
                        Label::primary(file_id, got.span.range.clone()).with_message(label_message)
                    ])
            }
            Self::ImportAfterTopLevelStatement(name) => {
                Diagnostic::error()
                    .with_message("import statement is found after top level statement(-s)".to_owned())
                    .with_code("E003")
                    .with_labels(vec![
                        Label::primary(file_id, name.range.clone()).with_message("this import statement must not be here")
                    ])
                    .with_notes(vec!["note: imports are placed at the beginning of source file, so consider placing it there".to_owned()])
            }
        }
    }
}
