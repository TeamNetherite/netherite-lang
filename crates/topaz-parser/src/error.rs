use codespan_reporting::diagnostic::{Diagnostic, Label};

use thiserror::Error;
use topaz_ast::location::*;
use topaz_ast::tokens::{LexerError, RawToken, Token};
use topaz_report::Reporter;

#[derive(Debug, Error)]
pub enum ParserError {
    /// Error appeared in lexing stage.
    #[error("scanning error appeared in the process")]
    ErrorToken(WithSpan<LexerError>),

    /// Unexpected tokens [`Token`] in AST Node called
    /// [`Option<String>`], expected [`String`].
    #[error("unexpected tokens `{0:?}` in `{2:?}`, expected `{1}`")]
    UnexpectedToken(Token, String, Option<String>),

    /// Unexpected tokens [`Token`], expected [`RawToken`]
    /// in AST Node called [`String`]  if [`Option<String>`]
    /// is `Some(String)`.
    #[error("unexpected tokens `{0:?}`, expected `{1:?}` in `{2:?}`")]
    UnexpectedTokenExpectedX(Token, RawToken, Option<String>),
}

impl<'source> Reporter<'source> for ParserError {
    fn build_diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        match self {
            Self::ErrorToken(t) => Diagnostic::error()
                .with_message("scanning error occured")
                .with_code("E000")
                .with_labels(vec![
                    Label::primary(file_id, t.span).with_message(t.value.to_string())
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
                        Label::primary(file_id, got.span).with_message(label_message)
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
                        Label::primary(file_id, got.span).with_message(label_message)
                    ])
            }
        }
    }
}
