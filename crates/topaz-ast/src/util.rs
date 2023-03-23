pub macro unit_impl($tr:ident [$($implementor:ident),*]) {
    $(
    impl $tr for $implementor {}
    )*
}
