use std::cell::RefCell;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::evaluator::environment::Environment;
use crate::evaluator::flstdlib::builtins;
use crate::evaluator::object::Object;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

const REPL_HELPER: &str = r#"
Helper
    // define variable
    let name: string;
    let name = "Foo";
    let name: string = "Foo";

    // define function
    define sum(x: int, y: int): int { return x + y }
    define sayHello(subject: string): null { print("Hello, ", subject) }

    // if-else statments
    if true { // do something } else { // do something else }
    
    // for loops
    for counter in range(0, 10) { print(counter) }

    // Built-in functions
    len("Hello");
    typeof(10);

    // More: ...
    // arthimetics: +, -, /, *
    // postfix: x++, x--
    // prefix: !x, -x
    
    Note: in case of bug:
       report to: dev.258.edilson@gmail.com
       open issue: https://github.com/edilson258/filipe

    Happy Hacking!
"#;

fn eval_repl_line(line: String, env: Rc<RefCell<Environment>>) {
    if line == String::from(".help") {
        println!("{}", REPL_HELPER);
        return;
    }

    if line == String::from("exit()") {
        println!("Exiting...");
        std::process::exit(0);
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

    match evaluated.clone().unwrap() {
        Object::Null => {},
        _ => println!("{}", evaluated.unwrap())
    }
}

pub fn repl() {
    println!("Welcome to filipe v0.1.");
    println!("Type \".help\" for more information.");

    let mut rl = DefaultEditor::new().unwrap();
    let env = Rc::new(RefCell::new(Environment::from(builtins(), None)));

    loop {
        let readline = rl.readline("|> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                eval_repl_line(line, Rc::clone(&env));
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
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
