use std::cell::RefCell;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::evaluator::environment::Environment;
use crate::evaluator::flstdlib::builtins;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn eval_repl_line(line: String, env: Rc<RefCell<Environment>>) {
    if line == String::from(".help") {
        println!("We only support arthimetics for now");
        return;
    }

    let input = line.chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();

    if p.has_error() {
        println!("{}", p.get_error().unwrap());
        return;
    };

    let mut evaltr = Evaluator::new(env);
    let evaluated = evaltr.eval(program);

    if evaluated.is_none() {
        return;
    }

    println!("{}", evaluated.unwrap());
}

pub fn repl() {
    println!("Welcome to filipe v0.1.");
    println!("Type \".help\" for more information.");

    let mut rl = DefaultEditor::new().unwrap();
    let env = Rc::new(RefCell::new(Environment::from(builtins(), None)));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                eval_repl_line(line, Rc::clone(&env));
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
}
