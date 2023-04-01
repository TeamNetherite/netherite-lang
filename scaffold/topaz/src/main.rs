use clap::{arg, Command};
use codespan_reporting::files::SimpleFiles;
use std::{fs, process::exit};
use std::fmt::Debug;
use topaz_ast::file::TopazFile;
use topaz_ast::location::WithSpan;
use topaz_parser_next::lex::{Lexer, Token};
use topaz_parser_next::Parse;
use topaz_report::{Reporter, ReporterState};

fn cli() -> Command {
    Command::new("topaz")
        .about("The Topaz compiler toolchain.\nCopyright 2023 - the Topaz team.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("lex")
                .about("Convert the source code into list of tokens")
                .arg(arg!(<PATH> "source file path"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("parse")
                .about("Convert the source code into AST and print it")
                .arg(arg!(<PATH> "source file path"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("graphviz")
                .about("Parse source code and print AST in graphviz format")
                .arg(arg!(<PATH> "source file path"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("analyze")
                .about("Parse source code and analyze AST using static analysis tools")
                .arg(arg!(<PATH> "source file path"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let reporter = ReporterState::default();

    let mut files = SimpleFiles::<&str, &str>::new();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("lex", sub_matches)) => {
            let filepath = sub_matches.get_one::<String>("PATH").unwrap();

            match fs::read_to_string(filepath) {
                Ok(contents) => {
                    let mut lexer = Lexer::new(&contents);

                    let mut token_index = 0;

                    loop {
                        let token = lexer.next();
                        if let Some(Ok(token)) = token {
                            let token: WithSpan<Token> = token.into();
                            println!(
                                "{token_index}: {token:#?}",
                            );

                            token_index += 1;
                        }
                    }
                }
                Err(_) => {
                    reporter.emit_global_error("cannot read given file");
                    exit(1);
                }
            }
        }
        Some(("parse", sub_matches)) => {
            let filepath = sub_matches.get_one::<String>("PATH").unwrap();

            match fs::read_to_string(filepath) {
                Ok(contents) => {
                    let file_id = files.add(filepath, &contents);
                    let mut ast = TopazFile::parse(&contents);

                    match ast {
                        Ok(file) => {
                            println!("{:#?}", file);
                        }
                        Err(e) => {
                            println!("{e}");
                            reporter
                                .emit_global_error("cannot output AST due to the previous errors");

                            exit(1);
                        }
                    }
                }
                Err(_) => {
                    reporter.emit_global_error("cannot read given file");
                    exit(1);
                }
            }
        }
        Some(("analyze", sub_matches)) => {
            /*
            let filepath = sub_matches.get_one::<String>("PATH").unwrap();

            match fs::read_to_string(filepath) {
                Ok(contents) => {
                    let file_id = files.add(filepath, &contents);
                    let mut parser = Parser::new(&contents);

                    let ast = parser.parse();

                    match ast {
                        Ok(program_unit) => {
                            let mut analyzer = StaticAnalyzer::new(file_id, program_unit, &files);
                            analyzer.analyze();
                            for e in &analyzer.output {
                                e.1.emit_diagnostic(&reporter, &files, e.0);
                            }
                        }
                        Err(e) => {
                            e.emit_diagnostic(&reporter, &files, file_id);

                            reporter
                                .emit_global_error("cannot output AST due to the previous errors");

                            exit(1);
                        }
                    }
                }
                Err(_) => {
                    reporter.emit_global_error("cannot read given file");
                    exit(1);
                }
            }
             */
            println!("no analyze for you :))))))))))0")
        }
        Some(("graphviz", sub_matches)) => {
            /*
            let filepath = sub_matches.get_one::<String>("PATH").unwrap();
            match fs::read_to_string(filepath) {
                Ok(contents) => {
                    let file_id = files.add(filepath, &contents);
                    let mut parser = Parser::new(&contents);

                    let ast = parser.parse();

                    match ast {
                        Ok(program_unit) => {
                            let mut translator =
                                topaz_ast_to_graphviz::GraphvizTranslatorState::new();
                            translator.ast_to_graphviz(&program_unit);
                        }
                        Err(e) => {
                            e.emit_diagnostic(&reporter, &files, file_id);

                            reporter
                                .emit_global_error("cannot output AST due to the previous errors");

                            exit(1);
                        }
                    }
                }
                Err(_) => {
                    reporter.emit_global_error("cannot read given file");
                    exit(1);
                }
            }
             */

            println!("graphviz is dead")
        }
        _ => {}
    }
}
