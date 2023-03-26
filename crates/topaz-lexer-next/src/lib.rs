#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

fn why() {

}

// CODE :: fn why() {}
// LEXER :: FN "why" OPENING_PAR CLOSING_PAR OPENING_CURLY CLOSING_CURLY
// PARSER :: FnDef { visibility: Visibility::Private, name: Ident("why"), block: Block(vec![]) }
