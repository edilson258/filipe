use std::cell::RefCell;
use std::rc::Rc;

use super::super::object::*;
use crate::runtime::context::{Context, ContextType};
use crate::runtime::type_system::Type;
use crate::runtime::{Expr, Identifier, Runtime};

pub fn eval_call_expr(
    e: &mut Runtime,
    func_ident: Expr,
    provided_args: Vec<Expr>,
) -> Option<Object> {
    let fn_name = match func_ident {
        Expr::Identifier(Identifier(name)) => name,
        _ => {
            e.error_handler
                .set_name_error(format!("Function name must be an identifier"));
            return None;
        }
    };

    let fn_object = match e.env.borrow().resolve(&fn_name) {
        Some(object) => object.value,
        None => {
            e.error_handler
                .set_name_error(format!("'{}' is not declared", fn_name));
            return None;
        }
    };

    let mut checked_args: Vec<ObjectInfo> = vec![];
    for arg in provided_args {
        let arg = match e.eval_expr(arg) {
            Some(object) => ObjectInfo {
                is_mut: true,
                type_: object.ask_type(),
                value: object,
            },
            None => return None,
        };
        checked_args.push(arg);
    }

    let (params, body, expected_ret_type) = match fn_object {
        Object::BuiltInFunction(builtin_fn) => match builtin_fn(checked_args) {
            BuiltInFuncReturnValue::Object(object) => return Some(object),
            BuiltInFuncReturnValue::Error(err) => {
                e.error_handler.set_error(err.kind, err.msg);
                return None;
            }
        },
        Object::UserDefinedFunction {
            params,
            body,
            return_type,
        } => (params, body, return_type),
        _ => {
            e.error_handler
                .set_type_error(format!("'{}' is not callable", fn_name));
            return None;
        }
    };

    if params.len() != checked_args.len() {
        e.error_handler.set_type_error(format!(
            "Function '{}' expecteds {} args but provided {}",
            fn_name,
            params.len(),
            checked_args.len()
        ));
        return None;
    }

    let global_scope = Rc::clone(&e.env);
    let mut fn_scope = Context::make_from(Rc::clone(&global_scope), ContextType::Function);

    for (_, (FunctionParam { name, type_ }, object_info)) in
        params.into_iter().zip(checked_args).enumerate()
    {
        if type_ != object_info.type_ {
            e.error_handler.set_type_error(format!(
                "Passing argument of type '{}' to parameter of type '{}'",
                object_info.type_, type_
            ));
            return None;
        }

        if !fn_scope.set(name.clone(), object_info.type_, object_info.value, true) {
            e.error_handler
                .set_name_error(format!("Param '{}' already declared", &name));
            return None;
        }
    }

    e.env = Rc::new(RefCell::new(fn_scope));
    let returned_value = e.eval_block_stmt(&body);

    if e.error_handler.has_error() {
        return None;
    }

    let returned_value_type = returned_value.clone().unwrap_or(Object::Null).ask_type();

    if (expected_ret_type != returned_value_type)
        && !is_types_equivalents(&expected_ret_type, &returned_value_type)
    {
        e.error_handler.set_type_error(format!(
            "Function '{}' must return '{}' but found '{}'",
            fn_name, expected_ret_type, returned_value_type,
        ));
        return None;
    }

    e.env = global_scope;
    returned_value
}

fn is_types_equivalents(lhs: &Type, rhs: &Type) -> bool {
    match lhs {
        Type::Void => match rhs {
            Type::Null => true,
            _ => false,
        },
        Type::Null => match rhs {
            Type::Void => true,
            _ => false,
        },
        _ => false,
    }
}
