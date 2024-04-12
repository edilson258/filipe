use super::object::Object;
use std::collections::HashMap;

pub type Builtins = HashMap<String, Object>;

pub fn builtins() -> Builtins {
    let mut builts = HashMap::new();
    builts.insert("print".to_string(), Object::Builtin(filipe_print));
    builts
}

fn filipe_print(args: Vec<Object>) -> Object {
    for arg in args {
        match &arg {
            Object::Number(val) => print!("{}", val),
            Object::String(val) => print!("{}", val),
            Object::Null => print!("null"),
            Object::Builtin(_) => print!("[Builtin Function]"),
            Object::Func(_, _, _) => print!("{}", arg),
            Object::RetVal(val) => print!("{}", val)
        }
    }
    println!();
    Object::Null
}
