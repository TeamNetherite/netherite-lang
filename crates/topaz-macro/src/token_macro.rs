use crate::ast_crate;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

macro_rules! token_i {
    ($input:ident, $ast:ident; $([$tk:tt] => $real:path),*$(,)?) => {
        match $input.to_string().as_str() {
            $(
            stringify!($tk) => Ok(quote!(#$ast::token::$real)),)*
            a => Err(syn::Error::new($input.span(), format!("token {} wasn't found!", a)))
        }
    };
}

pub(crate) fn token_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let ast = ast_crate();

    token_i! {
        input, ast;

        // tokens
        [,] => punctuation::Comma,
        [:] => punctuation::Colon,
        [;] => punctuation::Semi,
        [->] => punctuation::Arrow,
        [::] => punctuation::DoubleColon,

        // prefixes
        [&] => prefix::Ref,
        [-] => prefix::Minus,
        [+] => prefix::Plus,

        // keywords
        [mut] => keyword::Mut,
        [func] => keyword::Func,
        [let] => keyword::Let,
        [maybe] => keyword::Maybe,
        [some] => keyword::Some,
        [nope] => keyword::Nope,
        [typealias] => keyword::TypeAlias,
        [this] => keyword::This,
        [super] => keyword::Super,
        [gem] => keyword::Gem,
    }
}
