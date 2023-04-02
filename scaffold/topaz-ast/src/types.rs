use crate::path::Path;
use crate::punctuated::Punctuated;
use crate::token::delim::{AngleBracket, Parentheses, Surround};
use crate::util::unit_impl;
use crate::Token;
#[tokens]
#[derive(Eq, PartialEq)]
pub enum PrimitiveType {
    /// `()`
    Void,
    // Numbers
    Usize,
    Isize,
    Ubyte,
    Byte,
    Ushort,
    Short,
    Uint,
    Int,
    Ulong,
    Long,
    /// u128
    Uexplod,
    /// i128
    Explod,

    Char,
    String
}
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypePath {
    pub path: Path,
    pub arguments: Box<TypeArguments>,
}
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypeReference {
    pub ref_token: Token![&],
    pub mutability: Option<Token![mut]>,
    pub referenced: Box<Type>,
}
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypeFunc {
    pub func_token: Token![func],
    pub arguments: Box<TypeArguments>,
}
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypeMaybeUnknown {
    pub maybe_token: Token![maybe],
    pub real_type: Box<Type>,
}
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypeMaybeSome {
    pub some_token: Token![some],
    pub real_type: Box<Type>,
}
#[tokens]
#[derive(Eq, PartialEq)]
pub enum TypeMaybe {
    Unknown(TypeMaybeUnknown),
    Some(TypeMaybeSome),
    Nope,
}

#[tokens]
#[derive(Eq, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Path(TypePath),
    Reference(TypeReference),
    Func(TypeFunc),
    Maybe(TypeMaybe),
}

impl Type {
    #[must_use]
    pub fn type_arguments(&self) -> &TypeArguments {
        match self {
            Self::Path(path) => &path.arguments,
            Self::Reference(reference) => reference.referenced.type_arguments(),
            Self::Func(func) => &func.arguments,
            Self::Maybe(maybe) => match maybe {
                TypeMaybe::Unknown(TypeMaybeUnknown { real_type, .. })
                | TypeMaybe::Some(TypeMaybeSome { real_type, .. }) => real_type.type_arguments(),
                TypeMaybe::Nope => &TypeArguments::None,
            },
            Self::Primitive(_) => &TypeArguments::None
        }
    }
}

#[tokens]
#[derive(Eq, PartialEq)]
pub enum TypeArguments {
    /// No type arguments
    None,
    /// `(int) -> int` \
    /// usually used after `func`, like this: \
    /// `func(long) -> int`
    Parenthesized(ParenthesizedTypeArguments),
    /// `<A, B, C>`
    Normal(NormalTypeArguments),
}

pub type TypeArgs = Punctuated<Type, Token![,]>;

#[tokens]
#[derive(Eq, PartialEq)]
pub struct ParenthesizedTypeArguments {
    pub arguments: Surround<Parentheses, TypeArgs>,
    pub arrow_token: Token![->],
    pub return_type: Type,
}

#[tokens]
#[derive(Eq, PartialEq)]
pub struct NormalTypeArguments(pub Surround<AngleBracket, TypeArgs>);
