use super::FieldsManager;
use crate::runtime::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use crate::runtime::runtime_error::{ErrorKind, RuntimeError};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Primitive<T> {
    pub value: T,
    pub fields: FieldsManager,
}

impl<T> Primitive<T> {
    pub fn make(value: T, fields: HashMap<String, Object>) -> Primitive<T> {
        let primitive = Primitive {
            value,
            fields: FieldsManager::make(fields),
        };
        primitive
    }
}

pub fn make_string(value: String) -> Primitive<String> {
    let mut fields: HashMap<String, Object> = HashMap::new();
    fields.insert(
        "length".to_string(),
        Object::Int(make_integer(value.len() as i64)),
    );
    fields.insert(
        "as_int".to_string(),
        Object::BuiltInFunction(string_as_integer),
    );
    Primitive::<String>::make(value, fields)
}

fn string_as_integer(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("str.as_int takes 0 args but provided {}", args.len() - 1),
        });
    }

    let str = match args[0].value.clone() {
        Object::String(val) => val.value,
        _ => "".to_string(),
    };

    let int = match str.parse::<i64>() {
        Ok(val) => val,
        Err(_) => {
            return BuiltInFuncReturnValue::Error(RuntimeError {
                kind: ErrorKind::ArgumentError,
                msg: format!("Couldn't convert '{}' to int", str),
            });
        }
    };

    BuiltInFuncReturnValue::Object(Object::Int(make_integer(int)))
}

pub fn make_integer(value: i64) -> Primitive<i64> {
    let mut fields: HashMap<String, Object> = HashMap::new();
    fields.insert(
        "as_float".to_string(),
        Object::BuiltInFunction(integer_as_float),
    );
    Primitive::<i64>::make(value, fields)
}

fn integer_as_float(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!(
                "'int.as_float' takes 0 args but provided {}",
                args.len() - 1
            ),
        });
    }

    let int = match args[0].value.clone() {
        Object::Int(val) => val.value,
        _ => 0,
    };

    BuiltInFuncReturnValue::Object(Object::Float(f64::from(int as i32)))
}
