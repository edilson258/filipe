use crate::evaluator::object::{FunctionParam, FunctionParams, Object};
use crate::evaluator::type_system::{expr_type_to_object_type, Type};
use crate::evaluator::{BlockStmt, Evaluator, ExprType, Identifier};

pub fn eval_func_def(
    e: &mut Evaluator,
    identifier: &Identifier,
    params: &Vec<(Identifier, ExprType)>,
    body: &BlockStmt,
    ret_type: &ExprType,
) {
    let Identifier(name) = identifier;
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
        name: name.clone(),
        params,
        body: body.clone(),
        return_type,
    };
    if !e
        .env
        .borrow_mut().add_entry(name.clone(), function_object, Type::Function, false)
    {
        e.error_handler
            .set_name_error(format!("'{}' is already declared", name));
    }
}
