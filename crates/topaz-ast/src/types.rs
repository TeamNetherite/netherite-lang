use crate::location::WithSpan;
use crate::path::Path;
use crate::punctuated::Punctuated;
use crate::token::delim::{AngleBracket, Parentheses, Surround};
use crate::util::unit_impl;
use crate::Token;

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

pub enum Type {
    Path(TypePath),
    Reference(TypeReference),
    Func(TypeFunc),
}

impl Type {
    pub fn type_arguments(&self) -> &TypeArguments {
        match self {
            Type::Path(path) => &path.arguments,
            Type::Reference(reference) => &reference.referenced.type_arguments(),
            Type::Func(func) => &func.arguments,
        }
    }
}

unit_impl!(_Tokens [
    Type, TypePath, TypeReference, TypeFunc,
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
