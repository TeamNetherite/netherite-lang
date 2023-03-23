extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Block, bracketed, braced};
use quote::quote;
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;


pub(crate) struct Braced<T>(T);

impl<T: Parse> Parse for Braced<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        braced!(content in input);

        Ok(Braced(T::parse(&content)?))
    }
}

impl<T: Parse, P: Parse> Braced<Punctuated<T, P>> {
    pub(crate) fn parse_punct(input: ParseStream) -> syn::Result<Self> {
        let content;

        braced!(content in input);

        Ok(Braced(Punctuated::parse_terminated(&content)?))
    }
}
