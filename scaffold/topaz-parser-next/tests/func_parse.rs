#![feature(default_free_fn)]

use std::default::default;
use topaz_ast::block::Block;
use topaz_ast::expr::{Expr, ExprLit};
use topaz_ast::file::TopazFile;
use topaz_ast::ident::Ident;
use topaz_ast::item::func::Func;
use topaz_ast::item::Item;
use topaz_ast::literal::{Literal, LiteralString};
use topaz_ast::path::Path;
use topaz_ast::punctuated::Punctuated;
use topaz_ast::statement::func_call::{FuncCallArg, FuncCallStmt};
use topaz_ast::statement::Statement;
use topaz_ast::token::{StringLit, Surround};
use topaz_parser_next::Parse;

const EXAMPLE_FILE: &str = r#"
func main() {
    println("Hello world!");
}
"#;

#[test]
pub fn test_func_parse() {
    println!("INPUT: {EXAMPLE_FILE}");
    let parsed = TopazFile::parse(EXAMPLE_FILE);
    println!("PARSED: {parsed:#?}");
    assert!(parsed.is_ok());
    assert_eq!(
        parsed.unwrap(),
        TopazFile {
            items: vec![Item::Func(Func(
                default(),
                default(),
                Ident::new("main"),
                Vec::new(),
                None,
                Block(Surround::new(vec![Statement::FuncCall(FuncCallStmt(
                    Path::parse("println").unwrap(),
                    Surround::new(Punctuated::single(FuncCallArg(
                        None,
                        Expr::Literal(ExprLit(Literal::String(LiteralString(StringLit::new(
                            String::from("Hello world!")
                        )))))
                    )))
                ))]))
            ))]
        }
    );
}
