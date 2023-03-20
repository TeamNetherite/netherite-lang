mod visitor;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Block, bracketed};
use quote::quote;
use syn::parse::{Parse, ParseStream};


#[proc_macro]
pub fn _make_visitor(input: TokenStream) -> TokenStream {
    visitor::visitor_impl(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

pub(crate) struct Bracketed<T>(T);

impl<T: Parse> Parse for Bracketed<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        bracketed!(content in input);

        Ok(Bracketed(T::parse(&content)?))
    }
}
