use crate::frontend::ast::{Expr, Identifier};
use crate::runtime::object::Object;
use crate::runtime::Runtime;
use crate::stdlib::FieldsManager;

use super::func_call_evaluator::eval_call;

pub fn eval_field_access(rt: &mut Runtime, src: Expr, target: Expr) -> Option<Object> {
    let src = match rt.eval_expr(src) {
        Some(object) => object,
        None => return None,
    };

    if let Object::String(prim) = src {
        return _eval(rt, prim.fields, target);
    }

    if let Object::Module(m) = src {
        return _eval(rt, m.fields, target);
    }

    if let Object::Array { inner, items_type: _ } = src {
        return _eval(rt, inner.fields, target);
    }

    rt.error_handler
        .set_sematic(format!("Field access not impl for type {}", src.ask_type()));
    None
}

fn _eval(rt: &mut Runtime, fields: FieldsManager, target: Expr) -> Option<Object> {
    match target {
        Expr::Call(expr, args) => {
            let fn_name = match *expr {
                Expr::Identifier(Identifier(name)) => name,
                _ => {
                    rt.error_handler
                        .set_sematic("Function name must be an identifier".to_string());
                    return None;
                }
            };

            let fn_object = match fields.access(&fn_name) {
                Some(object) => object,
                None => {
                    rt.error_handler
                        .set_name_error(format!("No method '{}' associated", fn_name));
                    return None;
                }
            };

            eval_call(rt, fn_name, fn_object, args)
        }
        Expr::Identifier(Identifier(name)) => {
            let field = fields.access(&name);
            if field.is_none() {
                rt.error_handler
                    .set_name_error(format!("No field associated"));
                return None;
            }
            field
        }
        _ => {
            rt.error_handler
                .set_sematic("Can only access fields or methods".to_string());
            return None;
        }
    }
}
