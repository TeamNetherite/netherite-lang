use crate::Tokens;
use std::default::default;

mod private {
    pub trait Sealed {
        const REPR: (char, char);
    }
}

pub trait Delim: private::Sealed {}
impl<T: private::Sealed> Delim for T {}

macro_rules! delimiter {
    ($repr_a:literal $repr_b:literal $name:ident $ized:ident) => {
        #[derive(Default, Tokens, Copy, Clone)]
        pub struct $name;

        impl private::Sealed for $name {
            const REPR: (char, char) = ($repr_a, $repr_b);
        }

        pub type $ized<T> = Surround<$name, T>;
    };

    ($repr_a:literal $repr_b:literal $name:ident $ized:ident $inside:ty) => {
        #[derive(Default, Tokens, Copy, Clone)]
        pub struct $name;

        impl private::Sealed for $name {
            const REPR: (char, char) = ($repr_a, $repr_b);
        }

        pub type $ized = Surround<$name, $inside>;
    };
}

/// The `(` / `)` delimiter
delimiter!('(' ')' Parentheses Parenthesized);

/// The `{` / `}` delimiter
delimiter!('{' '}' Curly Braced);

/// The `[` / `]` delimiter
delimiter!('[' ']' Brackets Bracketed);

/// The `<` / `>` delimiter
delimiter!('<' '>' AngleBracket AngleBracketed);

/// The `"` / `"` delimiter
delimiter!('"' '"' StringDelim StringLit String);

/// The `'` / `'` delimiter
delimiter!('\'' '\'' CharDelim CharLit char);

pub enum Delimiter<D: Delim> {
    Left(D),
    Right(D),
}

impl<D: Delim> Delimiter<D> {
    pub const fn repr(&self) -> char {
        match self {
            Self::Left(_) => D::REPR.0,
            Self::Right(_) => D::REPR.1,
        }
    }
}

#[derive(Tokens)]
pub struct Surround<D: Delim, Content: Tokens>(D, Content, D);

#[allow(clippy::missing_const_for_fn)]
impl<D: Delim, Content: Tokens> Surround<D, Content> {
    pub const fn new(delim_start: D, content: Content, delim_end: D) -> Self {
        Self(delim_start, content, delim_end)
    }

    pub const fn delimiters(&self) -> (&D, &D) {
        (&self.0, &self.2)
    }
    pub const fn tuple(&self) -> (&D, &Content, &D) {
        (&self.0, &self.1, &self.2)
    }
    pub const fn content(&self) -> &Content {
        &self.1
    }

    pub fn into_delimiters(self) -> (D, D) {
        (self.0, self.2)
    }
    pub fn into_tuple(self) -> (D, Content, D) {
        (self.0, self.1, self.2)
    }
    pub fn into_content(self) -> Content {
        self.1
    }
}

impl<D: Delim + Default, Content: Tokens> Surround<D, Content> {
    pub fn new_default(content: Content) -> Self {
        Self(default(), content, default())
    }
}

auto trait Sn {}
impl<D: Delim, C: Tokens> !Sn for Surround<D, C> {}

impl<D: Delim + Default, Content: Tokens, Real: Into<Content> + Sn> From<Real>
    for Surround<D, Content>
{
    fn from(real: Real) -> Self {
        Self::new_default(real.into())
    }
}
impl<D: Delim + Default, Content: Tokens + Default> Default for Surround<D, Content> {
    fn default() -> Self {
        Self::new_default(default())
    }
}
