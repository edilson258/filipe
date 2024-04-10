use super::object::Object;
use std::collections::HashMap;

pub type Builtins = HashMap<String, Object>;

pub fn builtins() -> Builtins {
    let mut builts = HashMap::new();
    builts.insert("print".to_string(), Object::Builtin(1, filipe_print));
    builts
}

fn filipe_print(args: Vec<Object>) -> Object {
    for arg in args {
        match arg {
            Object::Number(val) => print!("{}", val),
            Object::String(val) => print!("{}", val),
            Object::Null => print!("null"),
            Object::Builtin(_, _) => print!("[Builtin Function]"),
            _ => println!("[ERROR]: Object is not printable"),
        }
    }
    println!();
    Object::Null
}
