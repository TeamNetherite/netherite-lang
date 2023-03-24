#![feature(decl_macro)]
#![feature(string_leak)]

mod token_macro;
mod ident_manipulation;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{parse_macro_input, DeriveInput, Block, bracketed, braced, Path};
use quote::quote;
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;

pub(crate) fn ast_crate() -> TokenStream2 {
    match crate_name("topaz-ast").ok() {
        Some(FoundCrate::Itself) => quote!(crate),
        Some(FoundCrate::Name(name)) => quote!(#name),
        None => quote!(compile_error!("topaz-ast not found"))
    }
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn Token(input: TokenStream) -> TokenStream {
    token_macro::token_impl(input.into()).into_into()
}

#[proc_macro]
pub fn lowercase_ident(input: TokenStream) -> TokenStream {
    ident_manipulation::lowercase_impl(input.into()).into_into()
}

pub(crate) trait IntoInto<T> {
    fn into_into(self) -> T;
}
impl IntoInto<TokenStream> for TokenStream2 {
    fn into_into(self) -> TokenStream {
        self.into()
    }
}

impl<TStream: IntoInto<TokenStream>> IntoInto<TokenStream> for Result<TStream, syn::Error> {
    fn into_into(self) -> TokenStream {
        self.map(|a| a.into_into()).unwrap_or_else(|e| e.to_compile_error().into_into())
    }
}

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
