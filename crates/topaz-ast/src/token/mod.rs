pub mod delim;
pub mod keyword;
pub mod prefix;
pub mod punctuation;

pub use keyword::*;
pub use prefix::*;
pub use punctuation::*;

pub use topaz_macro::Token;

pub(self) macro default_token_struct($($name:ident);*$(;)?) {
    $(
    #[derive(Default)]
    pub struct $name;

    impl crate::private::_Tokens for $name {}
    )*
}
