use core::fmt;

use super::runtime_error::RuntimeError;
use super::type_system::Type;
use super::BlockStmt;
use crate::stdlib::modules::Module;
use crate::stdlib::FilipeArray;

pub enum BuiltInFuncReturnValue {
    Object(Object),
    Error(RuntimeError),
}
type BuiltInFunction = fn(Vec<ObjectInfo>) -> BuiltInFuncReturnValue;

#[derive(PartialEq, Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub type_: Type,
}
pub type FunctionParams = Vec<FunctionParam>;

#[derive(Clone, Debug)]
pub enum Object {
    Null,
    Type(Type),
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    RetVal(Box<Object>),
    Array {
        inner: FilipeArray,
        items_type: Option<Type>,
    },
    UserDefinedFunction {
        params: FunctionParams,
        body: BlockStmt,
        return_type: Type,
    },
    BuiltInFunction(BuiltInFunction),
    Range {
        start: i64,
        end: i64,
        step: i64,
    },
    Module(Module),
}

impl Object {
    pub fn ask_type(&self) -> Type {
        match self {
            Object::Null => Type::Null,
            Object::String(_) => Type::String,
            Object::Boolean(_) => Type::Boolean,
            Object::BuiltInFunction(_) => Type::Function,
            Object::UserDefinedFunction {
                params: _,
                body: _,
                return_type: _,
            } => Type::Function,
            Object::RetVal(val) => val.ask_type(),
            Object::Type(_) => Type::TypeAnnot,
            Object::Range {
                start: _,
                end: _,
                step: _,
            } => Type::Range,
            Object::Int(_) => Type::Int,
            Object::Float(_) => Type::Float,
            Object::Array {
                inner: _,
                items_type,
            } => {
                if items_type.is_none() {
                    return Type::Array(None);
                }
                return Type::Array(Some(Box::new(items_type.clone().unwrap())));
            }

            Object::Module(_) => Type::Module,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    pub is_mut: bool,
    pub type_: Type,
    pub value: Object,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "'{}'", val),
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::BuiltInFunction(_) => write!(f, "[Builtin Function]"),
            Self::Null => write!(f, "null"),
            Self::RetVal(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Type(val) => write!(f, "{}", val),
            Self::Range { start, end, step } => write!(f, "range({start}, {end}, {step})"),
            Self::UserDefinedFunction {
                params: _,
                body: _,
                return_type: _,
            } => write!(f, "[User Defined Function]"),
            Self::Array {
                inner,
                items_type: _,
            } => write!(f, "{}", inner),
            Self::Module(m) => write!(f, "[Module] {}", m.name),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Void => write!(f, "void"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Boolean => write!(f, "boolean"),
            Self::String => write!(f, "string"),
            Self::Function => write!(f, "function"),
            Self::TypeAnnot => write!(f, "[Type Annotation]"),
            Self::Range => write!(f, "{}", self),
            Self::Array(items_type) => {
                if let Some(items_type) = items_type {
                    return write!(f, "Array<{}>", items_type);
                }
                write!(f, "Array<any>")
            }
            Self::Module => write!(f, "[Module]"),
        }
    }
}
