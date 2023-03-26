#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("Bad character: `{0}`")]
    BadCharacter(char),
    #[error("Bad character sequence: `{0}`")]
    BadSequence(String),
    #[error("Unexpected `{0}`, expected item")]
    ExpectedItem(String),
    #[error("Unexpected token `{1}`, expected {0}")]
    Unexpected(&'static str, String),
    #[error("Something else: {0}")]
    SomethingElse(&'static str),
    #[error("End of file reached!")]
    EOF
}

pub type LexResult<T> = Result<T, LexError>;
