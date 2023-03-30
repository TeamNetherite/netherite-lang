use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::{Ident, Token};

pub(crate) fn lowercase_impl(input: TokenStream) -> syn::Result<TokenStream> {
    Ok(Ident::new(
        &Parser::parse2(Ident::parse, input)?
            .to_string()
            .to_lowercase(),
        Span::call_site(),
    )
    .into_token_stream())
}

pub(crate) fn charify(input: TokenStream) -> TokenStream {
    Literal::string(&input.to_string()).into_token_stream()
}
