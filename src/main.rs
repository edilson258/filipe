mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn repl_line(line: String) {
    if line == String::from(".help") {
        println!("We only support arthimetics for now");
        return;
    }
    let input = line.chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();
    if p.get_errors().len() > 0 {
        for e in p.get_errors() {
            println!("{}", e);
        }
    } else {
        let mut evaltr = Evaluator::new();
        let evaluated = evaltr.eval(program);
        if evaluated.is_none() {
        } else {
            println!("{}", evaluated.unwrap());
        }
    }
}

fn main() -> Result<()> {
    println!("Welcome to filipe v0.1.");
    println!("Type \".help\" for more information.");

    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                repl_line(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
