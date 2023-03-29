use crate::Bracketed;
use itertools::Itertools;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Fields, Ident, ItemStruct, LitStr, Path, Token, Type, TypeMacro, Visibility};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum RuleType {
    Punctuation,
    Keyword,
    Delimiter,
    Prefix,
}

impl RuleType {
    fn name(&self) -> Ident {
        Ident::new(
            self.normal_name().to_uppercase().as_str(),
            Span::call_site(),
        )
    }

    fn enum_name(&self) -> Ident {
        Ident::new(
            titlecase::titlecase(self.normal_name()).as_str(),
            Span::call_site(),
        )
    }

    fn normal_name(&self) -> &'static str {
        match self {
            Self::Punctuation => "punctuations",
            Self::Keyword => "keywords",
            Self::Delimiter => "delimiters",
            Self::Prefix => "prefixes",
        }
    }
}

struct EverythingRule(TokenStream, Path, Ident, RuleType);

impl Parse for EverythingRule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let repr = input.parse::<Bracketed<TokenStream>>()?.0;
        input.parse::<Token![=>]>()?;
        let path: Path = input.parse()?;
        let ident = path.segments.last().unwrap().ident.clone();
        let rule_type = match path.segments.first().unwrap().ident.to_string().as_str() {
            "punctuation" => RuleType::Punctuation,
            "keyword" => RuleType::Keyword,
            "delim" => RuleType::Delimiter,
            "prefix" => RuleType::Prefix,
            _ => unreachable!(),
        };
        Ok(EverythingRule(repr, path, ident, rule_type))
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(rules);
}

struct EveryThing {
    rules: (kw::rules, Token![=], Punctuated<EverythingRule, Token![,]>),
}

impl Parse for EveryThing {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            rules: (
                input.parse()?,
                input.parse()?,
                Punctuated::parse_terminated(input)?,
            ),
        })
    }
}

pub fn everything_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let rules = Parser::parse2(
        Punctuated::<EverythingRule, Token![,]>::parse_terminated,
        input,
    )?;
    let enum_name = Ident::new("SingleToken", Span::call_site());
    let enum_variants: Vec<TokenStream> = rules
        .iter()
        .map(|EverythingRule(repr, token, en, _)| {
            let repr = LitStr::new(
                &repr.to_string().replace('{', "{{").replace('}', "}}"),
                repr.span(),
            );

            quote! {
                #[display(fmt = #repr)]
                #en(crate::token::#token)
            }
        })
        .collect();
    let token_macro: Vec<TokenStream> = rules
        .iter()
        .filter(|EverythingRule(_, _, _, a)| *a != RuleType::Delimiter)
        .map(|EverythingRule(repr, token, _, _)| quote!([#repr] => (crate::token::#token)))
        .collect();

    let rule_typed: HashMap<RuleType, Vec<EverythingRule>> = rules
        .into_iter()
        .group_by(|a| a.3)
        .into_iter()
        .map(|(a, b)| (a, b.collect_vec()))
        .collect();

    let rule_enums: Vec<TokenStream> = rule_typed
        .iter()
        .map(|(rt, rs)| {
            let en_name = rt.enum_name();
            let variants: Vec<TokenStream> = rs
                .iter()
                .map(|EverythingRule(_, tp, en, _)| quote!(#en(crate::token::#tp)))
                .collect();

            quote! {
                #[derive(Copy, Clone)]
                pub enum #en_name {
                    #(#variants,)*
                }
            }
        })
        .collect();

    Ok(quote! {
        #(#rule_enums)*

        #[derive(Clone, Copy, derive_more::Display, derive_more::From)]
        pub enum #enum_name {
            #(#enum_variants,)*
        }

        pub macro Token {
            #(#token_macro,)*
        }
    })
}
