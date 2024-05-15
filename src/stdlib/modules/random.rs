use rand::{self, Rng};

use super::Module;
use crate::runtime::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use crate::runtime::runtime_error::{ErrorKind, RuntimeError};
use crate::runtime::type_system::Type;
use crate::stdlib::primitives::make_integer;
use std::collections::HashMap;

pub fn module_random() -> Object {
    let mut fields: HashMap<String, Object> = HashMap::new();
    fields.insert("randint".to_string(), Object::BuiltInFunction(randint));
    Object::Module(Module::make("random".to_string(), fields))
}

fn randint(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 2 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("randint expects 2 arguments but provided {}", args.len()),
        });
    }

    if args[0].value.ask_type() != Type::Int || args[1].value.ask_type() != Type::Int {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("randint expects values of type int"),
        });
    }

    let (begin, end) = match args[0].value.clone() {
        Object::Int(begin) => match args[1].value.clone() {
            Object::Int(end) => (begin.value, end.value),
            _ => (0, 0),
        },
        _ => (0, 0),
    };

    if begin >= end {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("Invalid range: end must be bigger than start"),
        });
    }

    let out = rand::thread_rng().gen_range(begin..end);
    return BuiltInFuncReturnValue::Object(Object::Int(make_integer(out)));
}
