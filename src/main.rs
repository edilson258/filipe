mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod utils;
mod repl;
mod commands;

use std::{env, process::exit};

use commands::run_from_file;
use repl::repl;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    if cli_args.len() <= 1 {
        repl();
        return;
    }

    match cli_args[1].as_str() {
        "run" => {
            if cli_args.len() <= 2 {
                eprintln!("[ERROR]: Missing file path");
                exit(1);
            }
            run_from_file(&cli_args[2]);
        },
        "build" => {
        },
        _ => {
            eprintln!("[ERROR]: Unknown command {}", cli_args[1]);
            exit(1);
        }
    }
}
