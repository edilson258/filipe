use crate::frontend::ast::ExprType;

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
    Module,
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
