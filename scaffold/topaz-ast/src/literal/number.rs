use crate::token::B;
use derive_more::Display;
use std::fmt::Display;

#[tokens]
#[derive(Eq, PartialEq)]
pub enum LiteralNumber {
    Binary(BinaryNumber),
}

impl crate::private::_Tokens for () {}

#[tokens]
#[derive(Eq, PartialEq)]
#[derive(Default, Display)]
pub enum NumberSign {
    #[default]
    #[display(fmt = "+")]
    Positive,
    #[display(fmt = "-")]
    Negative,
}

impl NumberSign {
    pub fn sign(&self) -> bool {
        matches!(self, Self::Positive)
    }
}

#[tokens]
#[derive(Eq, PartialEq)]
#[derive(Display)]
pub enum NumberSuffix {
    /// No suffix.
    /// Example:
    /// ```tp
    /// // infers to `int`
    /// let number_thing = 0x087ffa;
    /// ```
    #[display(fmt = "")]
    None,

    /// usize
    Usize,
    /// isize
    Isize,

    /// u8
    Ubyte,
    /// i8
    Byte,

    /// u16
    Ushort,
    /// i16
    Short,

    /// u32
    Uint,
    /// i32
    Int,

    /// u64
    Ulong,
    /// i64
    Long,

    /// u128
    Uexplod,
    /// i128
    Explod,
}

macro_rules! digits {
    ($($name:ident ::: $($number:literal $variant:ident),*);*$(;)?) => {
        $(
        #[tokens]
        #[derive(Eq, PartialEq)]
        #[derive(Display)]
        pub enum $name {
            $(
            #[display(fmt = stringify!($number))]
            $variant,)*
        }

        impl $name {
            #[must_use]
            pub const fn value(&self) -> u8 {
                match self {
                    $(
                    Self::$variant => $number,
                    )*
                }
            }
        }
        )*
    }
}

digits! {
    HexDigit :::
        0 Zero,
        1 One,
        2 Two,
        3 Three,
        4 Four,
        5 Five,
        6 Six,
        7 Seven,
        8 Eight,
        9 Nine,
        0xA Ten,
        0xB Eleven,
        0xC Twelve,
        0xD Thirteen,
        0xE Fourteen,
        0xF Fifteen;

    DecimalDigit :::
        0 Zero,
        1 One,
        2 Two,
        3 Three,
        4 Four,
        5 Five,
        6 Six,
        7 Seven,
        8 Eight,
        9 Nine;

    OctalDigit :::
        0 Zero,
        1 One,
        2 Two,
        3 Three,
        4 Four,
        5 Five,
        6 Six,
        7 Seven,
        8 Eight;

    BinaryDigit :::
        0 Zero,
        1 One;
}

impl BinaryDigit {
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        matches!(self, Self::One)
    }
}

#[must_use]
pub fn concat_digits<T: Display>(digits: &[T]) -> String {
    digits
        .iter()
        .fold(String::new(), |acc, cur| acc + cur.to_string().as_str())
}

#[derive(Display)]
#[display(fmt = "{}{}{}", "self.0", "concat_digits(&self.1)", "self.2")]
#[tokens]
#[derive(Eq, PartialEq)]
pub struct NormalNumber(pub NumberSign, pub Vec<DecimalDigit>, pub NumberSuffix);

#[derive(Display)]
#[display(
    fmt = "{}{}{}{}",
    "self.0",
    "self.1",
    "concat_digits(&self.2)",
    "self.3"
)]
#[tokens]
#[derive(Eq, PartialEq)]
pub struct BinaryNumber(
    pub NumberSign,
    pub B,
    pub Vec<BinaryDigit>,
    pub NumberSuffix,
);
