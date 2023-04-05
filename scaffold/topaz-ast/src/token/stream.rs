use crate::location::{Span, WithSpan};
use crate::token::{SingleToken, TokenTree};
use crate::util::apply_mut;
use crate::WithSpannable;
use std::default::default;
use std::fmt::{Display, Formatter};
use std::marker::Destruct;

pub struct TokenStream {
    tokens: Vec<WithSpan<TokenTree>>,
    last_span: Span,
}

pub trait ToTokens {
    fn write_tokens(&self, tokens: &mut TokenStream);
    fn to_tokens(&self) -> TokenStream {
        apply_mut(TokenStream::new(), |a| self.write_tokens(a))
    }

    fn into_tokens(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_tokens()
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn write_tokens(&self, tokens: &mut TokenStream) {
        if let Some(value) = self {
            value.write_tokens(tokens)
        }
    }
}

impl TokenStream {
    fn calc_last_span(tokens: &Vec<WithSpan<TokenTree>>) -> Span {
        tokens.last().map(|a| a.span).unwrap_or(Span::new(0, 0))
    }

    pub fn new() -> Self {
        Self {
            tokens: default(),
            last_span: Span::new(0, 0),
        }
    }

    pub fn from_tokens(tokens: Vec<WithSpan<TokenTree>>) -> Self {
        let last_span = Self::calc_last_span(&tokens);
        Self { tokens, last_span }
    }

    pub fn append<T: ToTokens>(&mut self, token: &T) {
        token.write_tokens(&mut self);
    }

    pub fn append_token(&mut self, token: TokenTree, span: impl Into<Span>) {
        self.tokens.push(token.with_span(span))
    }

    pub fn append_single(&mut self, token: SingleToken, n: usize) {
        self.append_single_spanned(token, self.next_span(n))
    }

    pub fn append_single_spanned(&mut self, token: SingleToken, span: impl Into<Span>) {
        self.append_token(TokenTree::Single(token), span)
    }

    pub fn next_span(&self, n: usize) -> Span {
        Span::from_location(self.last_span.end, n)
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in self.tokens {
            x.value.fmt(f)?;
        }

        Ok(())
    }
}

impl Clone for TokenStream {
    fn clone(&self) -> Self {
        TokenStream {
            tokens: self.tokens.clone(),
            last_span: self.last_span
        }
    }

    fn clone_from(&mut self, source: &Self) where Self: Destruct {
        for i in source.tokens {
            self.tokens.push(i)
        }

        self.last_span = source.last_span;
    }
}

impl ToTokens for TokenStream {
    fn write_tokens(&self, tokens: &mut TokenStream) {
        tokens.clone_from(self)
    }
}
