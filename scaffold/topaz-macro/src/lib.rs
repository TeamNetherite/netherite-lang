#![feature(decl_macro)]
#![feature(string_leak)]

mod ident_manipulation;
mod token_macro;
mod tokens;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{braced, bracketed, parse_macro_input, Block, DeriveInput, Path, LitChar, Token, Lit, LitStr};

pub(crate) fn ast_crate() -> TokenStream2 {
    match crate_name("topaz-ast").ok() {
        Some(FoundCrate::Itself) => quote!(crate),
        Some(FoundCrate::Name(name)) => quote!(#name),
        None => quote!(compile_error!("topaz-ast not found")),
    }
}

#[proc_macro]
pub fn lowercase_ident(input: TokenStream) -> TokenStream {
    ident_manipulation::lowercase_impl(input.into()).into_into()
}

#[proc_macro]
pub fn charify(input: TokenStream) -> TokenStream {
    ident_manipulation::charify(input.into()).into()
}

#[proc_macro]
#[doc(hidden)]
#[deprecated]
pub fn everything(input: TokenStream) -> TokenStream {
    token_macro::everything_impl(input.into()).into_into()
}

#[proc_macro_attribute]
#[doc(hidden)]
pub fn tokens(args: TokenStream, input: TokenStream) -> TokenStream {
    tokens::tokens_impl(input.into(), args.into()).into_into()
}

#[proc_macro]
pub fn char_concat(chars: TokenStream) -> TokenStream {
    let chars = parse_macro_input!(chars with Punctuated::<LitChar, Token![,]>::parse_terminated);
    let chars = chars.into_iter().map(|a| a.value()).collect::<String>();
    LitStr::new(&chars, Span::call_site()).into_token_stream().into()
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
        self.map(|a| a.into_into())
            .unwrap_or_else(|e| e.to_compile_error().into_into())
    }
}

pub(crate) struct Bracketed<T>(T); // {T}

impl<T: Parse> Parse for Bracketed<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        bracketed!(content in input);

        Ok(Bracketed(T::parse(&content)?))
    }
}

impl<T: Parse, P: Parse> Bracketed<Punctuated<T, P>> {
    pub(crate) fn parse_punct(input: ParseStream) -> syn::Result<Self> {
        let content;

        bracketed!(content in input);

        Ok(Bracketed(Punctuated::parse_terminated(&content)?))
    }
}

pub(crate) struct Braced<T>(T); // {T}

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
