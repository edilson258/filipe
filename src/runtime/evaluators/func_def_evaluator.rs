use crate::runtime::object::{FunctionParam, FunctionParams, Object};
use crate::runtime::type_system::{expr_type_to_object_type, Type};
use crate::runtime::{BlockStmt, ExprType, Identifier, Runtime};

pub fn eval_func_def(
    e: &mut Runtime,
    name: String,
    params: &Vec<(Identifier, ExprType)>,
    body: &BlockStmt,
    ret_type: &ExprType,
) {
    if e.env.borrow().has(&name) {
        e.error_handler
            .set_name_error(format!("'{}' is already declared", name));
        return;
    }

    let params = params
        .iter()
        .map(|param| {
            let Identifier(param_name) = param.0.clone();
            let param_type = expr_type_to_object_type(&param.1);
            FunctionParam {
                name: param_name,
                type_: param_type,
            }
        })
        .collect::<FunctionParams>();
    let return_type = expr_type_to_object_type(ret_type);
    let function_object = Object::UserDefinedFunction {
        params,
        body: body.clone(),
        return_type,
    };

    e.env
        .borrow_mut()
        .set(name, Type::Function, function_object, false);
}
