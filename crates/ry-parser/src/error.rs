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
    /// [`bool`] is weather it is method declaration or definition.
    UnnecessaryVisibilityQualifier(Span, Span, bool),

    /// Appears when `import` keyword is found after top level statement(-s)
    /// [`Span`] here is location of `import` statement.
    ImportAfterTopLevelStatement(Span),
}

impl<'source> Reporter<'source> for ParserError {
    fn build_diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        match self {
            Self::ErrorToken(t) => Diagnostic::error()
                .with_message("scanning error occured")
                .with_code("E000")
                .with_labels(vec![
                    Label::primary(file_id, t.span.range.clone()).with_message(t.value.to_string())
                ]),
            Self::UnexpectedToken(got, expected, node_name) => {
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
            Self::UnnecessaryVisibilityQualifier(pub_span, method_name_span, declaration) => {
                let declaration = if *declaration { "declaration" } else { "definition" };
                Diagnostic::error()
                    .with_message(
                        format!("unnecessary visibility qualifier in method {}", declaration)
                    )
                    .with_labels(vec![
                        Label::primary(file_id, pub_span.range.clone())
                            .with_message("consider removing `pub`"),
                        Label::secondary(file_id, method_name_span.range.clone())
                            .with_message(format!("in this method {}", declaration)),
                    ])
                    .with_code("E002")
                    .with_notes(vec![
                        "note: by default, methods within a trait possess public visibility,\nthereby obviating the need for utilizing the `pub` keyword\nat the outset of their declarations.".to_owned()
                    ])
            }
            Self::UnexpectedTokenExpectedX(got, expected, node_name) => {
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
