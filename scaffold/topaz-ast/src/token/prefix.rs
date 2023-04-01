mod private {
    pub trait PrefixToken {
        const REPR: &'static str;
    }
}

pub trait PrefixToken: private::PrefixToken {}
impl<T: private::PrefixToken> PrefixToken for T {}

use private::PrefixToken as _PrefixToken;

macro_rules! prefix {
    ($repr:literal $name:ident) => {
        #[derive(Default, derive_more::Display, Copy, Clone, Debug)]
        #[display(fmt = $repr)]
        pub struct $name;
        impl _PrefixToken for $name {
            const REPR: &'static str = $repr;
        }
        impl crate::private::_Tokens for $name {}
    }
}

prefix!("&" Ref);
prefix!("b" B);
