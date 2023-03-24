use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, Parser};
use syn::Ident;

pub(crate) fn lowercase_impl(input: TokenStream) -> syn::Result<TokenStream> {
    Ok(Ident::new(
        &Parser::parse2(Ident::parse, input)?
            .to_string()
            .to_lowercase(),
        Span::call_site(),
    )
    .into_token_stream())
}
