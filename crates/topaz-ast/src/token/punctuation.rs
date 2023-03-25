mod private {
    pub trait Sealed {
        const REPR: &'static str;
    }
}

pub trait Punctuation: private::Sealed {}
impl<P: private::Sealed> Punctuation for P {}

macro_rules! punctuation {
    ($($repr:literal $name:ident;)*) => {
        $(
        #[derive(Default, derive_more::Display)]
        #[display(fmt = $repr)]
        pub struct $name;

        impl private::Sealed for $name {
            const REPR: &'static str = $repr;
        }
        impl crate::private::_Tokens for $name {}
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
}
