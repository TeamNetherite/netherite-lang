pub mod func;
pub mod type_alias;

pub enum Item {
    Func(func::Func),
    TypeAlias(type_alias::TypeAlias)
}
