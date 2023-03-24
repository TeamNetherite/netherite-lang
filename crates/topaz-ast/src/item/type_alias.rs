use crate::ident::Ident;
use crate::token::TypeAlias;
use crate::types::Type;
use crate::visibility::Visibility;

/// ```tp
/// public typealias Why = maybe ();
/// ```
pub struct TypeAlias {
    pub visibility: Visibility,
    pub typealias_token: TypeAlias,
    pub name: Ident,
    pub aliased: Box<Type>
}
