use core::fmt;

use super::runtime_error::RuntimeError;
use super::stdlib::FilipeArray;
use super::type_system::Type;
use super::BlockStmt;

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
    Array(FilipeArray, Type),
    UserDefinedFunction {
        name: String,
        params: FunctionParams,
        body: BlockStmt,
        return_type: Type,
    },
    BuiltInFunction(BuiltInFunction),
    Range {
        start: i64,
        end: i64,
    },
}

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    pub is_assignable: bool,
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
            Self::Range { start, end } => write!(f, "range({start}, {end})"),
            Self::UserDefinedFunction {
                name,
                params: _,
                body: _,
                return_type: _,
            } => write!(f, "[User Defined Function] {name}"),
            Self::Array(array, _) => write!(f, "{}", array),
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
            Self::Array => write!(f, "Array"),
            Type::Unknown => write!(f, "[Unknown Type]"),
        }
    }
}
