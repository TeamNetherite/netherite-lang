use crate::ident::Ident;
use crate::pattern::Pattern;
use crate::types::Type;

pub struct ItemFunc {
    name: Ident,
    arguments: Vec<FuncArg>
}

pub struct FuncArg {
    name: Pattern,
    return_type: Type
}
