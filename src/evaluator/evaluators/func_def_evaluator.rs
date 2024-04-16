use crate::evaluator::object::{FunctionParam, FunctionParams, Object, Type};
use crate::evaluator::{BlockStmt, Evaluator, ExprType, Identifier};

pub fn eval_func_def(
    e: &mut Evaluator,
    identifier: Identifier,
    params: Vec<(Identifier, ExprType)>,
    body: BlockStmt,
    ret_type: ExprType,
) {
    let Identifier(name) = identifier;
    let params = params
        .iter()
        .map(|param| {
            let Identifier(param_name) = param.0.clone();
            let param_type = e.expr_type_to_object_type(param.1.clone());
            FunctionParam {
                name: param_name,
                type_: param_type,
            }
        })
        .collect::<FunctionParams>();
    let return_type = e.expr_type_to_object_type(ret_type);
    let function_object = Object::UserDefinedFunction {
        name: name.clone(),
        params,
        body,
        return_type,
    };
    if !e
        .env
        .add_entry(name.clone(), function_object, Type::Function, false)
    {
        e.error_handler
            .set_name_error(format!("'{}' is already declared", name));
    }
}
