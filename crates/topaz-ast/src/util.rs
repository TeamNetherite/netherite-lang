pub macro unit_impl($tr:path [$($implementor:ident),*]) {
    $(
    impl $tr for $implementor {}
    )*
}

pub trait GFrom<T> {
    fn g_from(value: T) -> Self;
}
pub trait GInto<T> {
    fn g_into(self) -> T;
}
