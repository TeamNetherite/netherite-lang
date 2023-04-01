use crate::TopazParseError;
use lalrpop_util::state_machine::TokenTriple;
use logos::{Logos, Span, SpannedIter};
use topaz_ast::ident::Ident;
use topaz_ast::{CustomTokens, WithSpannable};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Logos, PartialEq, Eq, Clone, Debug)]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("func")]
    Func,
    #[token("import")]
    Import,
    #[token("public")]
    Public,
    #[token("private")]
    Private,
    #[token("this")]
    This,
    #[token("gem")]
    Gem,

    // Punctuation
    #[token("::")]
    DoubleColon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("=")]
    Equal,
    #[token(".")]
    Dot,
    #[token("(")]
    OpenParentheses,
    #[token(")")]
    CloseParentheses,
    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,
    #[token(r#"""#)]
    StrDelim,
    #[token("->")]
    Arrow,
    #[token(";")]
    Semi,

    #[regex(r"[a-zA-Z_][a-zA-Z\d_]*", |lex| Ident::new(lex.slice()))]
    Ident(Ident),

    #[regex(r#""[^\n"]*""#, |lex| lex.slice().trim_matches('"').to_owned())]
    LitStr(String),

    #[token("byte")]
    TypeByte,
    #[token("ubyte")]
    TypeUbyte,
    #[token("int")]
    TypeInt,
    #[token("uint")]
    TypeUint,
    #[token("str")]
    TypeString,

    #[error]
    #[regex(r"[ \t\n]+", logos::skip)]
    Error,
}

impl CustomTokens for Token {}

#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("Invalid token: {0}")]
    Invalid(String),
}

pub struct Lexer<'w> {
    tokens: logos::Lexer<'w, Token>,
}

impl<'w> Lexer<'w> {
    pub fn new(input: &'w str) -> Self {
        Self {
            tokens: Token::lexer(input),
        }
    }
}

impl<'w> Iterator for Lexer<'w> {
    type Item = Spanned<Token, usize, TopazParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| match token {
            Token::Error => Err(TopazParseError::Lexer(
                self.tokens.slice().to_owned().with_span(self.tokens.span()),
            )),
            token => {
                let Span { start, end } = self.tokens.span();
                Ok((start, token, end))
            }
        })
    }
}
