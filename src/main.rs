mod ast;
mod lexer;
mod token;
mod parser;
mod evaluator;

use std::process::exit;

use parser::Parser;
use lexer::Lexer;
use evaluator::Evaluator;

fn main() {
    let input = "2 + 3 * 4 / 3".to_string().chars().collect::<Vec<char>>();

    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();
    if p.get_errors().len() > 0 {
        for e in p.get_errors() {
            println!("{}", e);
        }
        exit(1);
    }

    let mut eval = Evaluator::new();
    println!("{}", eval.eval(program).unwrap());
}
