use std::path::Path;

use lexer::{lexer::Lexer, token::RawToken};

mod lexer;

fn main() {
    let mut lexer = Lexer::new(Path::new("<test>"), "true()");
    loop {
        let token = lexer.next();
        if token.clone().unwrap().raw == RawToken::EndOfFile {
            break;
        }

        println!("{}", token.unwrap().raw);
    }
}
