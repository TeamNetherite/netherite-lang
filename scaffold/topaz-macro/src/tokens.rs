use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, GenericParam, Generics, Item, ItemEnum, ItemStruct, Token};
use syn::spanned::Spanned;

pub fn tokens_impl(input: TokenStream, args: TokenStream) -> syn::Result<TokenStream> {
    let der_dbg = if args.is_empty() { quote!(#[derive(std::fmt::Debug)]) } else { quote!() };
    let parsed = Parser::parse2(Item::parse, input)?;
    match &parsed {
        Item::Struct(ItemStruct {
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
        })
        | Item::Enum(ItemEnum {
            ident,
            generics:
                Generics {
                    lt_token: lt,
                    params,
                    gt_token: gt,
                    where_clause,
                },
            ..
        }) => {
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
                        .collect::<Punctuated<_, Token! [,]>>()
                        .into_token_stream(),
                )
            };
            Ok(quote! {
                #der_dbg
                #parsed
                impl#lt#params#gt crate::private::_Tokens for #ident#lt#real_params#gt #where_clause {}
            })
        },

        _ => Err(syn::Error::new(parsed.span(), "item must be an enum or a struct"))
    }
}
