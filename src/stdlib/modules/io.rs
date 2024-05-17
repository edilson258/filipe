use super::Module;
use crate::runtime::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use crate::runtime::runtime_error::{ErrorKind, RuntimeError};
use crate::stdlib::primitives::make_string;
use std::collections::HashMap;
use std::io::Write;

pub fn module_io() -> Object {
    let mut io_fields: HashMap<String, Object> = HashMap::new();
    io_fields.insert("puts".to_string(), Object::BuiltInFunction(io_puts));
    io_fields.insert("gets".to_string(), Object::BuiltInFunction(io_gets));
    Object::Module(Module::make("io".to_string(), io_fields))
}

fn io_puts(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    for arg in args {
        match &arg.value {
            Object::Int(val) => print!("{}", val.value),
            Object::Float(val) => print!("{}", val),
            Object::String(val) => print!("{}", val.value),
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

fn io_gets(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    let prompt = match args.len() {
        0 => "".to_string(),
        1 => match args[0].value.clone() {
            Object::String(val) => val.value,
            _ => {
                return BuiltInFuncReturnValue::Error(RuntimeError {
                    kind: ErrorKind::ArgumentError,
                    msg: format!(
                        "'io.gets' expects argument of type string but provided type {}",
                        args[0].value.ask_type()
                    ),
                });
            }
        },
        _ => {
            return BuiltInFuncReturnValue::Error(RuntimeError {
                kind: ErrorKind::ArgumentError,
                msg: format!(
                    "'io.gets' expects 0 or 1 argument but provided '{}'",
                    args.len()
                ),
            });
        }
    };

    let mut buf = String::new();

    print!("{}", prompt);
    if std::io::stdout().flush().is_err() {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::IOError,
            msg: "Couldn't flush stdout".to_string(),
        });
    }

    if std::io::stdin().read_line(&mut buf).is_err() {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::IOError,
            msg: "Couldn't read from stdin".to_string(),
        });
    }

    BuiltInFuncReturnValue::Object(Object::String(make_string(buf.trim().to_string())))
}
