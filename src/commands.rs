use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

use crate::evaluator::environment::Environment;
use crate::evaluator::flstdlib::builtins;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::utils::read_file;

pub fn run_from_file(path: &str) {
    let input = match read_file(path) {
        Some(contents) => contents,
        None => exit(1),
    };

    let input = input.chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();

    if p.has_error() {
        println!("{}", p.get_error().unwrap());
        exit(1);
    };

    let env = Environment::from(builtins(), None);
    let mut evaltr = Evaluator::new(Rc::new(RefCell::new(env)));
    evaltr.eval(program);
}
