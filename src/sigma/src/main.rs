#![feature(box_syntax)]
#![feature(let_chains)]

mod ast;
mod lexer;
mod parser;

use ariadne::Color;

use crate::parser::Parser;
use std::{env, fs, process::exit, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: sigma <filename>");
        exit(1);
    }

    let filename = args[1].as_str();
    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut parser = Parser::new(filename, content.as_str());
            let start = Instant::now();
            let ast = parser.parse();

            if ast.is_some() {
                println!("parsed in {:?}", start.elapsed());
                println!("{:?}", ast);
            } else {
                println!(
                    "{}: cannot compile due to the previous errors.\n",
                    Color::Red.paint("Error")
                );
            }
        }
        Err(_) => {
            println!("{}: cannot read the file.\n", Color::Red.paint("Error"));
            exit(1);
        }
    }
}
