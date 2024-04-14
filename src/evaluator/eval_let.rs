use super::{
    object::{object_to_type, Object, Type},
    Evaluator, Expr, ExprType, Identifier, RuntimeErrorKind,
};

pub fn eval_let_stmt(
    e: &mut Evaluator,
    name: Identifier,
    expr_type: Option<ExprType>,
    expr: Option<Expr>,
) {
    let Identifier(name) = name;

    if expr_type.is_none() && expr.is_none() {
        e.set_error(
            RuntimeErrorKind::TypeError,
            format!(
                "cannot infer type of '{}', define it's type or initialize it",
                &name
            ),
        );
        return;
    }

    if expr_type.is_none() {
        eval_let_by_type_inference(e, name, expr.unwrap());
        return;
    }
    let provided_type = e.expr_type_to_object_type(expr_type.unwrap());
    if expr.is_none() {
        add_to_env(e, name, Object::Null, provided_type);
        return;
    }
    let evaluated_expr = match e.eval_expr(expr.unwrap()) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };
    add_to_env(e, name, evaluated_expr, provided_type);
}

fn eval_let_by_type_inference(e: &mut Evaluator, name: String, expr: Expr) {
    let evaluated_expr = match e.eval_expr(expr) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };
    let infered_type = object_to_type(&evaluated_expr);
    add_to_env(e, name, evaluated_expr, infered_type);
}

fn add_to_env(e: &mut Evaluator, name: String, object: Object, type_: Type) {
    if !e
        .env
        .add_entry(name.clone(), object, type_, true)
    {
        e.set_error(
            RuntimeErrorKind::NameError,
            format!("'{}' is already declared", &name),
        );
    }
}
