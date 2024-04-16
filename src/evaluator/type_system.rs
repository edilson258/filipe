use super::{object::Object, Evaluator, Expr, ExprType, Identifier, Literal};

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Null,
    Number,
    String,
    Boolean,
    Function,
    TypeAnnot,
}

pub fn expr_type_to_object_type(var_type: &ExprType) -> Type {
    match var_type {
        ExprType::String => Type::String,
        ExprType::Number => Type::Number,
        ExprType::Boolean => Type::Boolean,
        ExprType::Null => Type::Null,
    }
}

pub fn expr_to_type(e: &mut Evaluator, expr: &Expr) -> Option<Type> {
    match expr {
        Expr::Literal(literal) => match literal {
            Literal::String(_) => return Some(Type::String),
            Literal::Null => return Some(Type::Null),
            Literal::Number(_) => return Some(Type::Number),
            Literal::Boolean(_) => return Some(Type::Boolean),
        },
        Expr::Identifier(identifier) => return identifier_to_type(e, identifier),
        _ => {
            return None;
        }
    }
}

pub fn has_same_type(lhs: &Object, rhs: &Object) -> bool {
    object_to_type(lhs) == object_to_type(rhs)
}

pub fn identifier_to_type(e: &mut Evaluator, identifier: &Identifier) -> Option<Type> {
    let Identifier(name) = identifier;

    match e.env.get_typeof(&name) {
        Some(type_) => Some(type_),
        None => {
            e.error_handler
                .set_name_error(format!("'{}' is not declared", &name));
            return None;
        }
    }
}

pub fn object_to_type(object: &Object) -> Type {
    match object {
        Object::Null => Type::Null,
        Object::String(_) => Type::String,
        Object::Number(_) => Type::Number,
        Object::Boolean(_) => Type::Boolean,
        Object::BuiltInFunction(_) => Type::Function,
        Object::UserDefinedFunction {
            name: _,
            params: _,
            body: _,
            return_type: _,
        } => Type::Function,
        Object::RetVal(val) => object_to_type(&val),
        Object::Type(_) => Type::TypeAnnot,
    }
}
