use super::Module;
use crate::runtime::object::{BuiltInFuncReturnValue, Object, ObjectInfo};
use std::collections::HashMap;

pub fn module_io() -> Object {
    let mut io_fields: HashMap<String, Object> = HashMap::new();
    io_fields.insert("puts".to_string(), Object::BuiltInFunction(io_puts));
    Object::Module(Module::make("IO".to_string(), io_fields))
}

fn io_puts(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    for arg in args {
        match &arg.value {
            Object::Int(val) => print!("{}", val),
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
