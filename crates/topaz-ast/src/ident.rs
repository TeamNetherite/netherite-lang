use crate::token::{PathPartKeyword, EVERYTHING};
use once_cell::sync::Lazy;
use std::fmt::{Display, Formatter};
use string_interner::backend::BufferBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

pub const DENIED_CHARS: [char; 2] = ['&', '-'];
pub const STARTING_CHARS: [char; 1] = ['_'];

pub fn check_identifier(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }
    let first = ident.chars().next().unwrap();
    (first.is_ascii_alphabetic() || STARTING_CHARS.contains(&first))
        && ident.chars().all(|a| {
            EVERYTHING.keys().any(|k| {
                if k.len() == 1 {
                    k.chars().next().unwrap() == a
                } else {
                    false
                }
            }) && a.is_ascii()
        })
        && !EVERYTHING.contains_key(ident)
}

fn check_ident(ident: &str) -> &str {
    if !check_identifier(ident) {
        panic!("Identifier {} isn't valid!", ident);
    }

    ident
}

//pub struct Ident(Cow<'static, str>);
#[derive(Copy, Clone, Tokens)]
pub struct Ident(SymbolU32);

impl Ident {
    #[inline(always)]
    fn interner() -> &'static mut StringInterner<BufferBackend> {
        unsafe { Lazy::force_mut(&mut crate::INTERNER) }
    }

    /// # Panics
    /// Panics if the identifier is not valid.
    #[must_use]
    pub fn new(ident: &str) -> Self {
        Self(Self::interner().get_or_intern(check_ident(ident)))
    }
    #[must_use]
    pub fn new_static(ident: &'static str) -> Self {
        Self(Self::interner().get_or_intern_static(check_ident(ident)))
    }

    #[must_use]
    pub fn from_keyword<K: PathPartKeyword>() -> Self {
        Self::new_static(K::REPR)
    }

    #[must_use]
    pub fn keyword<K: PathPartKeyword>(_: K) -> Self {
        Self::from_keyword::<K>()
    }

    #[must_use]
    pub fn maybe_value(&self) -> Option<&str> {
        unsafe { Lazy::force(&crate::INTERNER) }.resolve(self.0)
    }

    #[must_use]
    /// # Panics
    /// Panics if a symbol was dropped.
    /// This should never happen.
    pub fn value(&self) -> &str {
        self.maybe_value()
            .unwrap_or_else(|| unreachable!("**[unreachable]** Symbol {:#?} was dropped", self.0))
    }
}

impl<K: PathPartKeyword> From<K> for Ident {
    fn from(_: K) -> Self {
        Self::from_keyword::<K>()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value())
    }
}
