use super::{object::Object, ExprType};

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Null,
    Void,
    Int,
    Float,
    String,
    Boolean,
    Function,
    Range,
    TypeAnnot,
    Array(Option<Box<Type>>),
}

pub fn expr_type_to_object_type(var_type: &ExprType) -> Type {
    match var_type {
        ExprType::String => Type::String,
        ExprType::Boolean => Type::Boolean,
        ExprType::Void => Type::Void,
        ExprType::Int => Type::Int,
        ExprType::Float => Type::Float,
        ExprType::Array(items_type) => {
            Type::Array(Some(Box::new(expr_type_to_object_type(&items_type))))
        }
    }
}

pub fn object_to_type(object: &Object) -> Type {
    match object {
        Object::Null => Type::Null,
        Object::String(_) => Type::String,
        Object::Boolean(_) => Type::Boolean,
        Object::BuiltInFunction(_) => Type::Function,
        Object::UserDefinedFunction {
            params: _,
            body: _,
            return_type: _,
        } => Type::Function,
        Object::RetVal(val) => object_to_type(&val),
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
        },
    }
}
