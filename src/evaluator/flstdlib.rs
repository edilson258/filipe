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
            Object::Number(val) => print!("{}", val),
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
        }
    }
    println!();
    BuiltInFuncReturnValue::Object(Object::Null)
}

fn filipe_len(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::TypeError,
            msg: format!("'len' expects 1 arg but {} were provided", args.len()),
        });
    }

    match args[0].value.clone() {
        Object::String(val) => BuiltInFuncReturnValue::Object(Object::Number(val.len() as f64)),
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
