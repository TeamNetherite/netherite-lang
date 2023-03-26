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

pub trait Require<T, E> {
    type Output;

    fn require<F: Fn(&T) -> bool, EF: Fn(&T) -> E>(self, check: F, error: EF) -> Self::Output;
}

impl<T, E, E2> Require<T, E2> for Result<T, E>
where
    E2: From<E>,
{
    type Output = Result<T, E2>;

    fn require<F: Fn(&T) -> bool, EF: Fn(&T) -> E2>(self, check: F, error: EF) -> Result<T, E2> {
        match self {
            Ok(real) => {
                if check(&real) {
                    Ok(real)
                } else {
                    Err(error(&real))
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}

pub fn apply<T, F: Fn(&T)>(thing: T, f: F) -> T {
    f(&thing);
    thing
}

pub fn apply_mut<T, F: Fn(&mut T)>(mut thing: T, f: F) -> T {
    f(&mut thing);

    thing
}
