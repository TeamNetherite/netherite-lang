use crate::Bracketed;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Ident, LitStr, Path, Token};

struct EverythingRule(TokenStream, Path, Ident);

impl Parse for EverythingRule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let repr = input.parse::<Bracketed<TokenStream>>()?.0;
        input.parse::<Token![=>]>()?;
        let path: Path = input.parse()?;
        let ident = path.segments.last().unwrap().ident.clone();
        Ok(EverythingRule(repr, path, ident))
    }
}

pub fn everything_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let rules = Parser::parse2(
        Punctuated::<EverythingRule, Token![,]>::parse_terminated,
        input,
    )?;
    let enum_name = Ident::new("SingleToken", Span::call_site());

    let rules_for_phf: Vec<TokenStream> = rules
        .iter()
        .map(|EverythingRule(_, token, en)| {
            let repr = LitStr::new(
                &token.segments.last().unwrap().ident.to_string(),
                Span::call_site(),
            );
            quote!(#repr => #enum_name::#en(crate::token::#token)) // "ok" => SingleToken::Ok(topaz_ast::token::Ok)
        })
        .collect();
    let enum_variants: Vec<TokenStream> = rules
        .iter()
        .map(|EverythingRule(repr, token, en)| {
            let repr = LitStr::new(&repr.to_string(), repr.span());
            
            quote! {
                #[display(fmt = #repr)]
                #en(crate::token::#token)
            }
        })
        .collect();
    let token_macro: Vec<TokenStream> = rules
        .iter()
        .map(|EverythingRule(repr, token, _)| quote!([#repr] => (crate::token::#token)))
        .collect();

    Ok(quote! {
        #[derive(Clone, Copy, derive_more::Display)]
        pub enum #enum_name {
            #(#enum_variants,)*
        }
        pub static EVERYTHING: phf::Map<&'static str, #enum_name> = phf::phf_map! {
            #(#rules_for_phf,)*
        };

        pub macro Token {
            #(#token_macro,)*
        }
    })
}
