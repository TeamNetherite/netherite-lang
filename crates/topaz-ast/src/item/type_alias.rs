use crate::ident::Ident;
use crate::types::{Type, TypeArguments};
use crate::visibility::Visibility;

/// ```tp
/// public typealias Why<T: Visit> = maybe (T|some int);
/// ```
#[derive(Tokens)]
pub struct TypeAlias(
    pub Visibility,
    pub Token![typealias],
    pub TypeArguments,
    pub Ident,
    /// the aliased type
    pub Box<Type>,
);
