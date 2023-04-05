use topaz_macro::lowercase_ident;

mod private {
    use crate::Tokens;

    pub trait Keyword: Tokens {
        const REPR: &'static str;
    }
    pub trait PathPartKeyword: Keyword {}
}

pub trait PathPartKeyword: private::PathPartKeyword + Keyword {}
impl<K: private::PathPartKeyword> PathPartKeyword for K {}

pub trait Keyword: private::Keyword {}
impl<K: private::Keyword> Keyword for K {}

macro_rules! kw {
    ($name:ident) => {
        #[tokens]
        #[derive(Default, Copy, Clone, Eq, PartialEq)]
        pub struct $name;
        impl private::Keyword for $name {
            const REPR: &'static str = stringify!(lowercase_ident!($name));
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use private::Keyword;
                f.write_str(Self::REPR)
            }
        }
    };
    (pathpart $name:ident) => {
        kw!($name);

        impl private::PathPartKeyword for $name {}
    };
}

/// The `func` keyword used to declare a function or to use a closure/function pointer type
kw!(Func);
/// The `mut` keyword.
kw!(Mut);
/// The `let` keyword. Usually used to declare a variable.
kw!(Let);
/// The `maybe` keyword.
/// Declares a type, like `maybe T`,
/// which means a value of type T could be present (value or `some value`),
/// or could be absent (`nope`).
/// Let's express a `maybe T` type, as it would be done in Kotlin - `T?`,
/// then `some T` would be just `T`,
/// and `nope` would be Nothing?.
kw!(Maybe);
/// The `some` keyword.
/// Either used to explicitly denote the presence of a value in the place of a `maybe` type,
/// or could be used as a type to require overriders
/// (the super type of an item should be `maybe T`) to specify a value (`nope` could not be used)
kw!(Some);
/// The `nope` keyword.
/// Either used to explicitly denote the absence of a value
/// (similar to `Nothing` in Kotlin) in the place of a `maybe`
/// type,
/// or could be used to specify the absence of a value,
/// for example as a function argument, or a value of a variable.
kw!(Nope);
kw!(Import);
kw!(TypeAlias);
kw!(pathpart This);
kw!(pathpart Super);
kw!(pathpart Gem);
kw!(As);