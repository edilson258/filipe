use super::Module;
use crate::runtime::{
    object::{BuiltInFuncReturnValue, Object, ObjectInfo},
    runtime_error::{ErrorKind, RuntimeError},
};

use std::{collections::HashMap, f64::consts::PI};

pub fn module_math() -> Object {
    let mut math_fields: HashMap<String, Object> = HashMap::new();
    math_fields.insert("PI".to_string(), Object::Float(PI));
    math_fields.insert(
        "sqrt".to_string(),
        Object::BuiltInFunction(module_math_sqrt),
    );
    Object::Module(Module::make("Math".to_string(), math_fields))
}

fn module_math_sqrt(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!(
                "Math.sqrt(x) expects 1 argument but provided {}",
                args.len()
            ),
        });
    }

    match args[0].value {
        Object::Int(x) => BuiltInFuncReturnValue::Object(Object::Float(f64::sqrt(x as f64))),
        Object::Float(x) => BuiltInFuncReturnValue::Object(Object::Float(f64::sqrt(x))),
        _ => BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("Math.sqrt(x) expects argument of type int or float"),
        }),
    }
}
