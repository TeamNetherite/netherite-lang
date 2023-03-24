use crate::path::Path;
use crate::private::_Tokens;
use crate::punctuated::Punctuated;
use crate::token::delim::{AngleBracket, Parentheses, Surround};
use crate::util::unit_impl;
use crate::Token;

pub enum PrimitiveType {
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
}

pub struct TypePath {
    pub path: Path,
    pub arguments: Box<TypeArguments>,
}
pub struct TypeReference {
    pub ref_token: Token![&],
    pub mutability: Option<Token![mut]>,
    pub referenced: Box<Type>,
}
pub struct TypeFunc {
    pub func_token: Token![func],
    pub arguments: Box<TypeArguments>,
}

pub struct TypeMaybeUnknown {
    pub maybe_token: Token![maybe],
    pub real_type: Box<Type>,
}
pub struct TypeMaybeSome {
    pub some_token: Token![some],
    pub real_type: Box<Type>,
}
pub enum TypeMaybe {
    Unknown(TypeMaybeUnknown),
    Some(TypeMaybeSome),
    Nope,
}

pub enum Type {
    Path(TypePath),
    Reference(TypeReference),
    Func(TypeFunc),
    Maybe(TypeMaybe),
}

impl Type {
    pub fn type_arguments(&self) -> &TypeArguments {
        match self {
            Type::Path(path) => &path.arguments,
            Type::Reference(reference) => &reference.referenced.type_arguments(),
            Type::Func(func) => &func.arguments,
            Type::Maybe(maybe) => match maybe {
                TypeMaybe::Unknown(TypeMaybeUnknown { real_type, .. })
                | TypeMaybe::Some(TypeMaybeSome { real_type, .. }) => &real_type.type_arguments(),
                _ => &TypeArguments::None,
            },
        }
    }
}

unit_impl!(_Tokens [
    Type, TypePath, TypeReference, TypeFunc,
    TypeMaybe, TypeMaybeUnknown, TypeMaybeSome,
    TypeArguments, ParenthesizedTypeArguments, NormalTypeArguments
]);

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

pub struct ParenthesizedTypeArguments {
    pub arguments: Surround<Parentheses, TypeArgs>,
    pub arrow_token: Token![->],
    pub return_type: Type,
}

pub struct NormalTypeArguments(pub Surround<AngleBracket, TypeArgs>);
