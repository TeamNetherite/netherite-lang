use std::path::Path;

mod ast;
mod lexer;

use crate::ast::token::RawToken;
use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(Path::new("<test>"), "0.23e");

    loop {
        let token = lexer.next();
        if token.clone().unwrap().raw == RawToken::EndOfFile {
            break;
        }

        println!("{}", token.unwrap().raw);

        // if let RawToken::String(s) = token.unwrap().raw {
        // println!("{}", s);
        // }
    }
    // let a: f64 = ".".parse().unwrap();
    // println!("{}", a);
}
