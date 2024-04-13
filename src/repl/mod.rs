use crate::evaluator::environment::Environment;
use crate::evaluator::flstdlib::builtins;
use crate::evaluator::object::Object;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn eval_repl_line(line: String, env: &mut Environment) {
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

    match evaluated.unwrap() {
        Object::Number(val) => println!("{val}"),
        Object::String(val) => println!("\"{val}\""),
        Object::BuiltinFn(_) => println!("[Builtin Function]"),
        Object::Null => println!("null"),
        Object::Boolean(val) => println!("{val}"),
        Object::Type(val) => println!("{val}"),
        _ => {}
    }
}

pub fn repl() {
    println!("Welcome to filipe v0.1.");
    println!("Type \".help\" for more information.");

    let mut rl = DefaultEditor::new().unwrap();
    let mut env = Environment::from(builtins(), None);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                eval_repl_line(line, &mut env);
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
