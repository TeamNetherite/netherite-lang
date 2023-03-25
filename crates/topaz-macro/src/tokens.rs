use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, GenericParam, Generics, Token};

pub fn derive_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident,
        generics:
            Generics {
                lt_token: lt,
                params,
                gt_token: gt,
                where_clause,
                ..
            },
        ..
    } = Parser::parse2(DeriveInput::parse, input)?;
    let real_params = if params.is_empty() {
        None
    } else {
        Some(
            params
                .iter()
                .map(|a| match a {
                    GenericParam::Type(ty) => ty.ident.to_token_stream(),
                    GenericParam::Lifetime(lifetime) => lifetime.lifetime.to_token_stream(),
                    GenericParam::Const(const_param) => const_param.ident.to_token_stream(),
                })
                .collect::<Punctuated<_, Token![,]>>()
                .into_token_stream(),
        )
    };
    Ok(quote! {
        impl#lt#params#gt crate::private::_Tokens for #ident#lt#real_params#gt #where_clause {}
    })
}
