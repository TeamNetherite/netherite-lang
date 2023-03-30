use crate::grammar;

pub type Result<T> = core::result::Result<T, ParseError>;

#[derive(thiserror::Error)]
pub enum ParseError {

}

pub type ParseStream<'a> = &'a String;

pub trait Parse {
    fn parse(stream: ParseStream) -> Result<Self>;
}
