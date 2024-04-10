use std::process::exit;

use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::utils::read_file;

pub fn run(path: &str) {
    let input = match read_file(path) {
        Some(contents) => contents,
        None => exit(1),
    };

    let input = input.chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();

    if p.get_errors().len() > 0 {
        for e in p.get_errors() {
            println!("{}", e);
        }
        exit(1);
    };

    let mut evaltr = Evaluator::new();
    let evaluated = evaltr.eval(program);
    if evaluated.is_none() {
        return;
    }
    println!("{}", evaluated.unwrap());
}
