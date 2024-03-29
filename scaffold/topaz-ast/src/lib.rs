#![feature(decl_macro)]
#![feature(default_free_fn)]
#![feature(is_some_and)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(concat_idents)]
#![feature(const_trait_impl)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![deny(deprecated)]
#![allow(unused_doc_comments)]
#![allow(clippy::module_name_repetitions)]
//! `lib.rs` - defines AST nodes and additional stuff.
pub mod location;

#[macro_use]
extern crate topaz_macro;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Debug;
use string_interner::backend::BufferBackend;
use string_interner::StringInterner;

static mut INTERNER: Lazy<StringInterner<BufferBackend>> =
    Lazy::new(StringInterner::<BufferBackend>::new);

use crate::util::unit_impl;
use location::{Span, WithSpan};

pub mod block;
pub mod expr;
pub mod file;
pub mod ident;
pub mod item;
pub mod literal;
pub mod path;
pub mod pattern;
pub mod punctuated;
pub mod statement;
#[macro_use]
pub mod token;
pub mod types;
pub mod util;
pub mod visibility;
pub mod visit;

pub use token::Token;

pub(crate) mod private {
    use std::fmt::Debug;

    pub trait _Tokens: Debug {}
}

pub(crate) use topaz_macro::tokens as Tokens;
pub trait Tokens: private::_Tokens {}

impl<T: private::_Tokens> Tokens for T {}

impl<T: Tokens> private::_Tokens for Vec<T> {}

pub trait CustomTokens: Debug {}

impl<T: CustomTokens> private::_Tokens for T {}

unit_impl!(crate::private::_Tokens [char, String, i32, u8]);

pub trait WithSpannable {
    fn with_span(self, span: impl Into<Span>) -> WithSpan<Self>
    where
        Self: Sized;
}

impl<T> WithSpannable for T {
    fn with_span(self, span: impl Into<Span>) -> WithSpan<Self> {
        WithSpan::new(self, span.into())
    }
}
