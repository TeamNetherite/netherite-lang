#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("Bad character: `{0}`")]
    BadCharacter(char),
    #[error("Something else: {0}")]
    SomethingElse(&'static str)
}

pub type LexResult<T> = Result<T, LexError>;
