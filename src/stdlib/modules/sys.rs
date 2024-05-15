use super::Module;
use crate::runtime::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use crate::runtime::runtime_error::{ErrorKind, RuntimeError};
use std::collections::HashMap;

pub fn module_sys() -> Object {
    let mut fields: HashMap<String, Object> = HashMap::new();
    fields.insert("exit".to_string(), Object::BuiltInFunction(exit));
    Object::Module(Module::make("sys".to_string(), fields))
}

fn exit(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.is_empty() {
        std::process::exit(0);
    }

    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!(
                "'exit' expects 0 or 1 argument but {} were provided",
                args.len()
            ),
        });
    }

    match args[0].value.clone() {
        Object::Int(val) => std::process::exit(val.value as i32),
        _ => BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: "'exit' only accepts an integer argument".to_string(),
        }),
    }
}
