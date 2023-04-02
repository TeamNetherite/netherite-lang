use crate::ident::Ident;
use crate::Token;
use crate::types::{Type, TypeArguments};
use crate::visibility::Visibility;

/// ```tp
/// public typealias Why<T: Visit> = maybe (T|some int);
/// ```
#[tokens]
#[derive(Eq, PartialEq)]
pub struct TypeAlias(
    pub Visibility,
    pub Token![typealias],
    pub TypeArguments,
    pub Ident,
    /// the aliased type
    pub Box<Type>,
);
