use proc_macro2::TokenStream;
use syn::{Ident, Block, Token, TypePath, Path};
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use crate::Bracketed;

struct VisitorField(Path, Ident, Option<Block>); // ProgramUnit file {}

impl Parse for VisitorField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<TypePath>()?;
        let name = input.parse::<Ident>()?;

        if input.peek(Bracket) {
            Ok(VisitorField(ty.path, name, None))
        } else {
            Ok(VisitorField(ty.path, name, Some(input.parse()?)))
        }
    }
}

struct VisitorDef(Ident, Bracketed<Punctuated<VisitorField, Token![;]>>);

impl Parse for VisitorDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {

    }
}

pub(crate) fn visitor_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let input = Parser::parse2(Vec::<VisitorDef>::parse, input)?;
}
