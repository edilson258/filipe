use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::runtime::{context::Context, flstdlib::builtins, Runtime};
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

    let env = Context::from(builtins(), None);
    let mut evaltr = Runtime::new(Rc::new(RefCell::new(env)));
    evaltr.eval(program);
}
