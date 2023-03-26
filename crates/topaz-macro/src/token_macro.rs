use crate::Bracketed;
use itertools::Itertools;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Ident, LitStr, Path, Token};

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
    let thing = Parser::parse2(EveryThing::parse, input)?;
    let rules = &thing.rules.2;
    let enum_name = Ident::new("SingleToken", Span::call_site());

    let rules_for_phf: Vec<TokenStream> = rules
        .iter()
        .filter(|EverythingRule(_, _, _, a)| *a != RuleType::Delimiter)
        .map(|EverythingRule(_, token, en, _)| {
            let repr = LitStr::new(
                &token.segments.last().unwrap().ident.to_string(),
                Span::call_site(),
            );
            quote!(#repr => #enum_name::#en(crate::token::#token)) // "ok" => SingleToken::Ok(topaz_ast::token::Ok)
        })
        .collect();
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
        .map(|EverythingRule(repr, token, _, _)| quote!([#repr] => ($crate::token::#token)))
        .collect();

    let rule_typed: HashMap<RuleType, Vec<EverythingRule>> = thing.rules.2
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

    let phfs: Vec<TokenStream> = rule_typed
        .into_iter()
        .map(|(rt, rs)| {
            let en_name = rt.enum_name();
            let name = rt.name();
            let ty = if rt == RuleType::Delimiter {
                quote!(char)
            } else {
                quote!(&'static str)
            };

            let pf: Vec<TokenStream> = rs
                .into_iter()
                .flat_map(|EverythingRule(token, tp, en, _)| {
                    let a = quote!(#en_name::#en(crate::token::#tp));
                    if rt == RuleType::Delimiter {
                        let token = token.to_string();
                        let mut token = token.chars();
                        let [token1, token2] = [token.next().unwrap(), token.next().unwrap()];
                        if token1 == token2 {
                            return vec![quote!(#token1 => #a)];
                        }
                        let [token1, token2] =
                            [Literal::character(token1), Literal::character(token2)];
                        vec![quote!(#token1 => #a), quote!(#token2 => #a)]
                    } else {
                        let token = Literal::string(&token.to_string());
                        vec![quote!(#token => #a)]
                    }
                })
                .collect();

            quote! {
                pub static #name: phf::Map<#ty, #en_name> = phf::phf_map! {
                    #(#pf,)*
                };
            }
        })
        .collect();

    Ok(quote! {
        #(#rule_enums)*

        #[derive(Clone, Copy, derive_more::Display, derive_more::From, logos::Logos)]
        pub enum #enum_name {
            #(#enum_variants,)*

        }

        #(#phfs)*

        pub static EVERYTHING: phf::Map<&'static str, #enum_name> = phf::phf_map! {
            #(#rules_for_phf,)*
        };

        macro_rules! Token {
            #(#token_macro;)*
        }

        pub(crate) use Token;
    })
}
