mod private {
    use std::fmt::Display;
    use crate::Tokens;

    pub trait Sealed: Copy + Default + Display + Tokens {
        const REPR: &'static str;
    }
}

pub trait Punctuation: private::Sealed {}
impl<P: private::Sealed> Punctuation for P {}

macro_rules! punctuation {
    ($($repr:literal $name:ident;)*) => {
        $(
        #[tokens]
        #[derive(Default, derive_more::Display, Copy, Clone, Eq, PartialEq)]
        #[display(fmt = $repr)]
        pub struct $name;
        impl private::Sealed for $name {
            const REPR: &'static str = $repr;
        }
        )*
    }
}

punctuation! {
    "," Comma;
    ":" Colon;
    ";" Semi;
    "->" Arrow;
    "::" DoubleColon;
    "=" Equal;
    "." Dot;
    "+" Plus;
    "-" Minus;
}
