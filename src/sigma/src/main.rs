use std::path::Path;

mod ast;
mod lexer;
mod parser;

use crate::parser::Parser;
use ast::token::RawToken;
use lexer::Lexer;

fn main() {
    // let mut lexer = Lexer::new(Path::new("<test>"), "0.23e");
    //
    // loop {
    //     let token = lexer.next();
    //     if token.clone().unwrap().raw == RawToken::EndOfFile {
    //         break;
    //     }
    //
    //     println!("{}", token.unwrap().raw);
    // }
    let mut parser = Parser::new("<test>", "namespace sd;");
    parser.parse();
}
