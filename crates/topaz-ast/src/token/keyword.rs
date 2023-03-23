use crate::token::{default_token_struct};

/// The `func` keyword used to declare a function or to use a closure/function pointer type
default_token_struct!(Func);
/// The `mut` keyword.
default_token_struct!(Mut);
/// The `let` keyword. Usually used to declare a variable.
default_token_struct!(Let);
