use crate::token::{default_token_struct};

/// The `func` keyword used to declare a function or to use a closure/function pointer type
default_token_struct!(Func);
/// The `mut` keyword.
default_token_struct!(Mut);
/// The `let` keyword. Usually used to declare a variable.
default_token_struct!(Let);
/// The `maybe` keyword.
/// Declares a type, like `maybe T`,
/// which means a value of type T could be present (value or `some value`),
/// or could be absent (`nope`)
default_token_struct!(Maybe);
/// The `some` keyword.
/// Either used to explicitly denote the presence of a value in the place of a `maybe` type,
/// or could be used as a type to require overriders
/// (the super type of an item should be `maybe T`) to specify a value (`nope` could not be used)
default_token_struct!(Some);
/// The `nope` keyword.
/// Either used to explicitly denote the absence of a value in the place of a `maybe` type,
/// or could be used to specify the absence of a value,
/// for example as a function argument, or a value of a variable.
default_token_struct!(Nope);
