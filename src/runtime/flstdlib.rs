use super::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use super::runtime_error::{ErrorKind, RuntimeError};
use super::type_system::Type;
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, ObjectInfo> {
    let mut builtin_list: HashMap<String, ObjectInfo> = HashMap::new();

    builtin_list.insert(
        "exit".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_exit),
        },
    );

    builtin_list.insert(
        "typeof".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_typeof),
        },
    );

    builtin_list.insert(
        "range".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_range),
        },
    );

    builtin_list.insert(
        "true".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Boolean,
            value: Object::Boolean(true),
        },
    );

    builtin_list.insert(
        "false".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Boolean,
            value: Object::Boolean(false),
        },
    );

    builtin_list.insert(
        "null".to_string(),
        ObjectInfo {
            is_mut: false,
            type_: Type::Null,
            value: Object::Null,
        },
    );

    builtin_list
}

fn filipe_exit(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
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

fn filipe_typeof(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::TypeError,
            msg: format!("'typeof' expects 1 arg but {} were provided", args.len()),
        });
    }

    BuiltInFuncReturnValue::Object(Object::Type(args[0].type_.clone()))
}

fn filipe_range(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() > 3 || args.len() < 2 {
        return BuiltInFuncReturnValue::Error({
            RuntimeError {
                kind: ErrorKind::TypeError,
                msg: format!(
                    "function 'range' takes 2 or 3 argus but {} were provided",
                    args.len()
                ),
            }
        });
    }

    for item in args.clone().into_iter() {
        if item.type_ != Type::Int {
            return BuiltInFuncReturnValue::Error({
                RuntimeError {
                    kind: ErrorKind::TypeError,
                    msg: format!("args for function 'range' must be of type number"),
                }
            });
        }
    }

    let mut built_args = vec![];

    for item in args {
        let value = match item.value {
            Object::Int(x) => x.value,
            _ => 0,
        };

        built_args.push(value)
    }
    if built_args.len() < 3 {
        built_args.push(1)
    };

    BuiltInFuncReturnValue::Object(Object::Range {
        start: built_args[0],
        end: built_args[1],
        step: built_args[2],
    })
}
