pub mod func;
pub mod type_alias;

pub enum Item {
    Func(func::ItemFunc),
    TypeAlias(type_alias::ItemTypeAlias)
}
