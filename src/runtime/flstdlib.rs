use super::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use super::runtime_error::{ErrorKind, RuntimeError};
use super::stdlib::module::Module;
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

    builtin_list.insert(
        "Math".to_string(),
        ObjectInfo {
            is_assignable: false,
            type_: Type::Module,
            value: module_math(),
        },
    );

    builtin_list
}

fn module_math() -> Object {
    let mut math_fields: HashMap<String, Object> = HashMap::new();
    math_fields.insert("PI".to_string(), Object::Float(3.1415));
    Object::Module(Module::make("Math".to_string(), math_fields))
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
                params: _,
                body: _,
                return_type: _,
            } => print!("{}", arg.value),
            Object::RetVal(val) => print!("{}", val),
            Object::Boolean(val) => print!("{}", val),
            Object::Type(val) => print!("{}", val),
            Object::Range {
                start: _,
                end: _,
                step: _,
            } => print!("{}", arg.value),
            Object::Array {
                inner,
                items_type: _,
            } => print!("{}", inner),
            Object::Module(_) => print!("{}", arg.value),
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
            msg: format!(
                "'exit' expects 0 or 1 argument but {} were provided",
                args.len()
            ),
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
            Object::Int(x) => x,
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
