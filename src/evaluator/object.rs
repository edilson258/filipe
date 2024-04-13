use core::fmt;

use super::{BlockStmt, Identifier, RuntimeError};

pub enum BuiltInFuncRetVal {
    Object(Object),
    Error(RuntimeError),
}
type BuiltInFunc = fn(Vec<Object>) -> BuiltInFuncRetVal;

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Null,
    Number,
    String,
    Boolean,
    Function,
    TypeAnnot,
}

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    BuiltinFn(BuiltInFunc),
    Func(String, Vec<Identifier>, BlockStmt),
    Null,
    RetVal(Box<Object>),
    Type(Type),
}

pub fn object_to_type(object: &Object) -> Type {
    match object {
        Object::Null => Type::Null,
        Object::String(_) => Type::String,
        Object::Number(_) => Type::Number,
        Object::Boolean(_) => Type::Boolean,
        Object::BuiltinFn(_) => Type::Function,
        Object::Func(_, _, _) => Type::Function,
        Object::RetVal(val) => object_to_type(&val),
        Object::Type(_) => Type::TypeAnnot,
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "'{}'", val),
            Self::Number(val) => write!(f, "{}", val),
            Self::BuiltinFn(_) => write!(f, "[Builtin Function]"),
            Self::Null => write!(f, "null"),
            Self::Func(name, _, _) => write!(f, "[Defined Function] '{name}'"),
            Self::RetVal(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Type(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Number => write!(f, "number"),
            Self::Boolean => write!(f, "boolean"),
            Self::String => write!(f, "string"),
            Self::Function => write!(f, "function"),
            Self::TypeAnnot => write!(f, "[Type Annotation]"),
        }
    }
}
