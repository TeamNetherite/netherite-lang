use crate::private::_Tokens;
use crate::token::PathPartKeyword;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub struct Ident(Cow<'static, str>);
//pub struct Ident(SymbolU32, *const StringInterner);

impl Ident {
    pub fn new(identifier: String) -> Self {
        Ident(Cow::Owned(identifier))
    }

    pub fn new_static(ident: &'static str) -> Self {
        Ident(Cow::Borrowed(ident))
    }
    /*
    pub fn new(parser: &mut Parser, value: impl AsRef<str>) -> Self {
        Ident(parser.interner.get_or_intern(value), &parser.interner as *const StringInterner)
    }

    pub fn new_(value: impl AsRef<str>) -> (Parser, Self) {
        let mut parser = Parser::new();

        let this = Self::new(&mut parser, value);

        (parser, this)
    }
     */

    pub fn from_keyword<K: PathPartKeyword>() -> Self {
        Self::new_static(K::REPR)
    }

    pub fn keyword<K: PathPartKeyword>(_: K) -> Self {
        Self::from_keyword::<K>()
    }

    pub fn value<'a: 'static>(&'a self) -> &'static str {
        self.0.deref()
    }

    pub fn into_value(self) -> String {
        self.0.into_owned()
    }

    /*
    pub fn maybe_value(&self) -> Option<&str> {
        unsafe { self.1.as_ref().unwrap() }.resolve(self.0)
    }

    pub fn value(&self) -> &str {
        self.maybe_value().expect("symbol was dropped")
    }
     */
}

impl<K: PathPartKeyword> From<K> for Ident {
    fn from(_: K) -> Self {
        Self::from_keyword::<K>()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl _Tokens for Ident {}
