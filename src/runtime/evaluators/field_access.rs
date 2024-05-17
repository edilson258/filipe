use super::func_call_evaluator::eval_call;
use crate::frontend::ast::{Expr, Identifier};
use crate::runtime::object::{Object, ObjectInfo};
use crate::runtime::type_system::Type;
use crate::runtime::Runtime;
use crate::stdlib::primitives::{make_integer, make_string};
use crate::stdlib::FieldsManager;

pub fn eval_field_access(rt: &mut Runtime, src: Expr, target: Expr) -> Option<Object> {
    let src = match rt.eval_expr(src) {
        Some(object) => object,
        None => return None,
    };

    match src.clone() {
        Object::Int(prim) => {
            return _eval(
                rt,
                prim.fields,
                target,
                src,
                vec![ObjectInfo {
                    is_mut: false,
                    type_: Type::Int,
                    value: Object::Int(make_integer(prim.value)),
                }],
            )
        }
        Object::String(prim) => {
            return _eval(
                rt,
                prim.fields,
                target,
                src,
                vec![ObjectInfo {
                    is_mut: false,
                    type_: Type::String,
                    value: Object::String(make_string(prim.value)),
                }],
            )
        }
        Object::Module(m) => {
            return _eval(rt, m.fields, target, src, vec![]);
        }
        Object::Array { inner, items_type } => {
            return _eval(
                rt,
                inner.fields.clone(),
                target,
                src,
                vec![ObjectInfo {
                    is_mut: false,
                    value: Object::Array {
                        inner,
                        items_type: items_type.clone(),
                    },
                    type_: Type::Array(if items_type.is_none() {
                        None
                    } else {
                        Some(Box::new(items_type.unwrap()))
                    }),
                }],
            );
        }
        _ => {
            rt.error_handler
                .set_sematic(format!("Field access not impl for type {}", src.ask_type()));
            None
        }
    }
}

fn _eval(
    rt: &mut Runtime,
    fields: FieldsManager,
    target: Expr,
    src: Object,
    extra_args: Vec<ObjectInfo>,
) -> Option<Object> {
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
                        .set_name_error(format!("No method '{}' associated with {}", fn_name, src));
                    return None;
                }
            };

            eval_call(rt, fn_name, fn_object, args, extra_args)
        }
        Expr::Identifier(Identifier(name)) => {
            let field = fields.access(&name);
            if field.is_none() {
                rt.error_handler
                    .set_name_error(format!("No field '{}' associated with {}", name, src));
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
