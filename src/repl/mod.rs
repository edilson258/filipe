use std::cell::RefCell;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::runtime::context::Context;
use crate::runtime::flstdlib::builtins;
use crate::runtime::object::Object;
use crate::runtime::Runtime;

const REPL_HELPER: &str = r#"
Helper
    // define variable
    // @Note: type annotation is optional 
    //     when variables are initialized

    let name = "Jonh Harvard"
    let age: int = 85
    let lovesCoffe = true
    let height: float = 1.87

    // Arrays

    let xs: Array<int> = [1, 2, 3]
    let ys = ["y1", "y2", "y3"]

    // define function

    define sum(x: int, y: int): int {
        let result = x + y
        return result 
    }

    define sayHello(subject: string): void {
        print("Hello, ", subject)
    }

    // if-else statments

    if true { 
        // do something 
    } else { 
        // do something else 
    }
    
    // for loops
    for counter in range(0, 10) { 
        print(counter) 
    }

    // Built-in functions

    len("Hello")
    typeof(10)

    // More: ...
    // arthimetics: +, -, /, *
    // postfix: x++, x--
    // prefix: !x, -x
    
    Note: in case of bug:
       report to: dev.258.edilson@gmail.com
       open issue: https://github.com/edilson258/filipe

    Happy Hacking!
"#;

fn eval_repl_line(line: String, env: Rc<RefCell<Context>>) {
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

    let mut evaltr = Runtime::new(env);
    let evaluated = evaltr.eval(program);

    if evaluated.is_none() {
        return;
    }

    match evaluated.clone().unwrap() {
        Object::Null => {}
        _ => println!("{}", evaluated.unwrap()),
    }
}

pub fn repl() {
    println!("Welcome to filipe v0.1.");
    println!("Type \".help\" for more information.");

    let mut rl = DefaultEditor::new().unwrap();
    let env = Rc::new(RefCell::new(Context::make_global(builtins())));

    loop {
        let readline = read_line(&mut rl, "|> ");
        match readline {
            Some(line) => {
                let mut state = String::new();
                state.push_str(&line);
                if !balance_and_eval(&mut rl, state, Rc::clone(&env)) {
                    break;
                }
            },
            None => break
        }
    }
}

fn balance_and_eval(rl: &mut DefaultEditor, mut state: String, env: Rc<RefCell<Context>>) -> bool {
    loop {
        if is_buf_balanced(&state) {
            eval_repl_line(state.to_string(), env);
            return true;
        }
        match read_line(rl, "...") {
            Some(line) => state.push_str(&line),
            None => return false,
        }
    }
}

fn read_line(rl: &mut DefaultEditor, prompt: &str) -> Option<String> {
    let readline = rl.readline(prompt);
    match readline {
        Ok(line) => {
            let _ = rl.add_history_entry(line.as_str());
            Some(line)
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            println!("Exiting...");
            None
        }
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    }
}

fn is_buf_balanced(buf: &str) -> bool {
    let open_brace_count = buf.chars().filter(|c| *c == '{').count();
    let close_brace_count = buf.chars().filter(|c| *c == '}').count();
    open_brace_count <= close_brace_count
}
