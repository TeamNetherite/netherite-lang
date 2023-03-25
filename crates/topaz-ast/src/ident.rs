use crate::token::PathPartKeyword;
use once_cell::sync::Lazy;
use std::fmt::{Display, Formatter};
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

#[inline(always)]
fn interner() -> &'static mut StringInterner {
    unsafe { Lazy::force_mut(&mut crate::INTERNER) }
}

//pub struct Ident(Cow<'static, str>);
#[derive(Copy, Clone, Tokens)]
pub struct Ident(SymbolU32);

impl Ident {
    #[must_use]
    pub fn new(ident: &str) -> Self {
        Self(interner().get_or_intern(ident))
    }
    #[must_use]
    pub fn new_static(ident: &'static str) -> Self {
        Self(interner().get_or_intern_static(ident))
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
