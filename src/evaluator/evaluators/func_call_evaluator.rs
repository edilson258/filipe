use std::cell::RefCell;
use std::rc::Rc;

use super::super::object::*;
use crate::evaluator::environment::Environment;
use crate::evaluator::type_system::{expr_to_type, object_to_type, Type};
use crate::evaluator::{Evaluator, Expr, Identifier};

pub fn eval_call_expr(
    e: &mut Evaluator,
    func_ident: &Expr,
    provided_args: &Vec<Expr>,
) -> Option<Object> {
    let mut checked_args: Vec<ObjectInfo> = vec![];

    for arg in provided_args {
        let arg = match e.eval_expr(arg) {
            Some(object) => ObjectInfo {
                is_assignable: true,
                type_: expr_to_type(e, arg).unwrap_or(object_to_type(&object)),
                value: object,
            },
            None => return None,
        };
        checked_args.push(arg);
    }

    let func_name = match Evaluator::expr_to_identifier(&func_ident) {
        Some(identifier) => {
            let Identifier(name) = identifier;
            name
        }
        None => {
            e.error_handler
                .set_type_error(format!("invalid function name {:?}", func_ident));
            return None;
        }
    };

    let func_object = match e.eval_expr(func_ident) {
        Some(expr) => expr,
        None => return None,
    };

    let (name, params, body, expected_ret_type) = match func_object.clone() {
        Object::BuiltInFunction(builtin_fn) => match builtin_fn(checked_args) {
            BuiltInFuncReturnValue::Object(object) => return Some(object),
            BuiltInFuncReturnValue::Error(err) => {
                e.error_handler.set_error(err.kind, err.msg);
                return None;
            }
        },
        Object::UserDefinedFunction {
            name,
            params,
            body,
            return_type,
        } => (name, params, body, return_type),
        _ => {
            e.error_handler
                .set_type_error(format!("'{}' is not callable", func_name));
            return None;
        }
    };

    if params.len() != checked_args.len() {
        e.error_handler.set_type_error(format!(
            "Function '{}' expecteds {} args but provided {}",
            name,
            params.len(),
            checked_args.len()
        ));
        return None;
    }

    let global_scope = Rc::clone(&e.env);
    let mut fn_scope = Environment::empty(Some(Rc::clone(&global_scope)));

    for (_, (FunctionParam { name, type_ }, object_info)) in
        params.iter().zip(checked_args).enumerate()
    {
        if type_ != &object_info.type_ {
            e.error_handler.set_type_error(format!(
                "passing argument of type '{}' to parameter of type '{}'",
                &object_info.type_, type_
            ));
            return None;
        }

        fn_scope.add_entry(name.clone(), object_info.value, object_info.type_, true);
    }

    e.env = Rc::new(RefCell::new(fn_scope));
    let returned_value = e.eval_block_stmt(&body).unwrap_or(Object::Null);
    let provided_type = object_to_type(&returned_value);

    if (expected_ret_type != provided_type)
        && !types_are_equivalents(&expected_ret_type, &provided_type)
    {
        e.error_handler.set_type_error(format!(
            "function '{}' must return '{}' but found '{}'",
            name, expected_ret_type, provided_type
        ));
        return None;
    }

    e.env = global_scope;
    Some(returned_value)
}

fn types_are_equivalents(lhs: &Type, rhs: &Type) -> bool {
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
