use super::environment::ObjectInfo;
use super::object::{object_to_type, BuiltInFuncRetVal, Object, Type};
use super::{RuntimeError, RuntimeErrorKind};
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, ObjectInfo> {
    let mut builtin_list: HashMap<String, ObjectInfo> = HashMap::new();

    builtin_list.insert(
        "print".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::BuiltinFn(filipe_print),
        },
    );

    builtin_list.insert(
        "len".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::BuiltinFn(filipe_len),
        },
    );

    builtin_list.insert(
        "typeof".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::BuiltinFn(filipe_typeof),
        },
    );

    builtin_list.insert(
        "true".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::Boolean(true),
        },
    );

    builtin_list.insert(
        "false".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::Boolean(false),
        },
    );

    builtin_list.insert(
        "null".to_string(),
        ObjectInfo {
            is_assinable: false,
            value: Object::Null,
        },
    );

    builtin_list
}

fn filipe_print(args: Vec<Object>) -> BuiltInFuncRetVal {
    for arg in args {
        match &arg {
            Object::Number(val) => print!("{}", val),
            Object::String(val) => print!("{}", val),
            Object::Null => print!("null"),
            Object::BuiltinFn(_) => print!("[Builtin Function]"),
            Object::Func(_, _, _) => print!("{}", arg),
            Object::RetVal(val) => print!("{}", val),
            Object::Boolean(val) => print!("{}", val),
            Object::Type(val) => print!("{}", val),
        }
    }
    println!();
    BuiltInFuncRetVal::Object(Object::Null)
}

fn filipe_len(args: Vec<Object>) -> BuiltInFuncRetVal {
    if args.len() != 1 {
        return BuiltInFuncRetVal::Error(RuntimeError {
            kind: RuntimeErrorKind::TypeError,
            msg: format!("'len' expects 1 arg but {} were provided", args.len()),
        });
    }

    match args[0].clone() {
        Object::String(val) => BuiltInFuncRetVal::Object(Object::Number(val.len() as f64)),
        _ => BuiltInFuncRetVal::Error(RuntimeError {
            kind: RuntimeErrorKind::TypeError,
            msg: format!("'len' only accepts iterable types"),
        }),
    }
}

fn filipe_typeof(args: Vec<Object>) -> BuiltInFuncRetVal {
    if args.len() != 1 {
        return BuiltInFuncRetVal::Error(RuntimeError {
            kind: RuntimeErrorKind::TypeError,
            msg: format!("'typeof' expects 1 arg but {} were provided", args.len()),
        });
    }

    BuiltInFuncRetVal::Object(Object::Type(object_to_type(&args[0])))
}
