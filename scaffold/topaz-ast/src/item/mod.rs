pub mod func;
pub mod type_alias;
pub mod import;

#[derive(Tokens)]
pub enum Item {
    Import(import::Import),
    Func(func::Func),
    TypeAlias(type_alias::TypeAlias)
}
