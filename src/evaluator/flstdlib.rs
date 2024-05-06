use super::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use super::runtime_error::{ErrorKind, RuntimeError};
use super::type_system::Type;
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, ObjectInfo> {
    let mut builtin_list: HashMap<String, ObjectInfo> = HashMap::new();

    builtin_list.insert(
        "print".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_print),
        },
    );

    builtin_list.insert(
        "exit".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_exit),
        },
    );    

    builtin_list.insert(
        "len".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_len),
        },
    );

    builtin_list.insert(
        "typeof".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_typeof),
        },
    );

    builtin_list.insert(
        "range".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Function,
            value: Object::BuiltInFunction(filipe_range),
        },
    );

    builtin_list.insert(
        "true".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Boolean,
            value: Object::Boolean(true),
        },
    );

    builtin_list.insert(
        "false".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Boolean,
            value: Object::Boolean(false),
        },
    );

    builtin_list.insert(
        "null".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Null,
            value: Object::Null,
        },
    );

    builtin_list
}

fn filipe_print(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    for arg in args {
        match &arg.value {
            Object::Int(val) => print!("{}", val),
            Object::Float(val) => print!("{}", val),
            Object::String(val) => print!("{}", val),
            Object::Null => print!("null"),
            Object::BuiltInFunction(_) => print!("[Builtin Function]"),
            Object::UserDefinedFunction {
                name,
                params: _,
                body: _,
                return_type: _,
            } => print!("{}", name),
            Object::RetVal(val) => print!("{}", val),
            Object::Boolean(val) => print!("{}", val),
            Object::Type(val) => print!("{}", val),
            Object::Range { start: _, end: _ } => print!("{}", &arg.value),
            Object::Array(array, _) => print!("{}", array),
        }
    }
    println!();
    BuiltInFuncReturnValue::Object(Object::Null)
}

fn filipe_exit(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.is_empty() {
        std::process::exit(0);
    }

    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("'exit' expects 0 or 1 argument but {} were provided", args.len()),
        });
    }

    match args[0].value.clone() {
        Object::Int(val) => std::process::exit(val as i32),
        _ => BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: "'exit' only accepts an integer argument".to_string(),
        }),
    }
}

fn filipe_len(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::TypeError,
            msg: format!("'len' expects 1 arg but {} were provided", args.len()),
        });
    }

    match args[0].value.clone() {
        Object::String(val) => BuiltInFuncReturnValue::Object(Object::Int(val.len() as i64)),
        _ => BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::TypeError,
            msg: format!("'len' only accepts iterable types"),
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
    if args.len() != 2 {
        return BuiltInFuncReturnValue::Error({
            RuntimeError {
                kind: ErrorKind::TypeError,
                msg: format!(
                    "function 'range' takes 2 argus but {} were provided",
                    args.len()
                ),
            }
        });
    }

    if args[0].type_ != Type::Int || args[1].type_ != Type::Int {
        return BuiltInFuncReturnValue::Error({
            RuntimeError {
                kind: ErrorKind::TypeError,
                msg: format!("args for function 'range' must be of type number"),
            }
        });
    }

    let start = match args[0].value {
        Object::Int(x) => x,
        _ => 0,
    };

    let end = match args[1].value {
        Object::Int(x) => x,
        _ => 0,
    };

    BuiltInFuncReturnValue::Object(Object::Range { start, end })
}
