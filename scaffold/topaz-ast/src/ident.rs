use crate::token::stream::{ToTokens, TokenStream};
use crate::token::{PathPartKeyword, SingleToken, TokenTree, EVERYTHING};
use once_cell::sync::Lazy;
use std::fmt::{Debug, Display, Formatter};
use string_interner::backend::BufferBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

pub const DENIED_CHARS: [char; 2] = ['&', '-'];
pub const STARTING_CHARS: [char; 1] = ['_'];

#[must_use]
pub fn check_identifier(ident: &str) -> bool {
    ident.chars().next().map_or(false, |first| {
        (first.is_ascii_alphabetic() || STARTING_CHARS.contains(&first))
            && !ident.chars().all(|a| {
                EVERYTHING.keys().any(|k| {
                    if k.len() == 1 {
                        k.chars().next().unwrap_or('\0') == a
                    } else {
                        false
                    }
                }) && a.is_ascii()
            })
            && !EVERYTHING.contains_key(ident)
    })
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn check_ident(ident: &str) -> &str {
    assert!(check_identifier(ident), "Identifier {ident} isn't valid!");

    ident
}

//pub struct Ident(Cow<'static, str>);
#[tokens(_)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Ident(SymbolU32);

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ident")
            .field("symbol", &self.0)
            .field("value", &self.maybe_value().unwrap_or("__BROKEN_SYMBOL__"))
            .finish()
    }
}

impl ToTokens for Ident {
    fn write_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_token(
            TokenTree::Ident(*self),
            tokens.next_span(self.value().len()),
        )
    }
}

impl Ident {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn interner() -> &'static mut StringInterner<BufferBackend> {
        unsafe { Lazy::force_mut(&mut crate::INTERNER) }
    }

    /// # Panics
    /// Panics if the identifier is not valid.
    ///
    /// A valid identifier conforms to this regex:
    ///
    /// `[a-zA-Z_][a-zA-Z\d_]*`
    /// (first character is an alphabetic or an underscore,
    /// others are alphanumeric or underscore)
    #[must_use]
    pub fn new(ident: &str) -> Self {
        Self(Self::interner().get_or_intern(check_ident(ident)))
    }
    #[must_use]
    pub fn new_static(ident: &'static str) -> Self {
        Self(Self::interner().get_or_intern_static(check_ident(ident)))
    }

    /// # Safety
    /// This function does not execute any unsafe code,
    /// it is marked so because creating unchecked identifiers could lead to undefined behavior.
    #[must_use]
    pub unsafe fn new_unchecked(ident: &str) -> Self {
        Self(Self::interner().get_or_intern(ident))
    }

    #[must_use]
    pub fn new_checked(ident: &str) -> Option<Self> {
        if !check_identifier(ident) {
            return None;
        }
        Some(Self(Self::interner().get_or_intern(ident)))
    }

    #[must_use]
    pub fn from_keyword<K: PathPartKeyword>() -> Self {
        Self(Self::interner().get_or_intern_static(K::REPR))
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
