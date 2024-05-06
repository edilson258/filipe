use crate::evaluator::{
    object_to_type, type_system::expr_type_to_object_type, Evaluator, Expr, ExprType, Identifier,
    LetStmtFlags, Literal, Object, Type,
};

pub fn eval_let_stmt(
    e: &mut Evaluator,
    name: &Identifier,
    expr_type: &Option<ExprType>,
    expr: &Option<Expr>,
    flags: &LetStmtFlags,
) {
    let Identifier(name) = name;

    if expr_type.is_none() && expr.is_none() {
        e.error_handler.set_type_error(format!(
            "cannot infer type of '{}', define it's type or initialize it",
            &name
        ));
        return;
    }

    if flags.is_array {
        if expr_type.is_none() {
            e.error_handler
                .set_type_error(format!("Missing items type of '{}' array", name));
            return;
        }
        if expr.is_none() {
            e.error_handler
                .set_type_error(format!("Missing init value of '{}' array", name));
            return;
        }

        eval_array_declaration(
            e,
            name,
            expr_type.to_owned().unwrap(),
            expr.to_owned().unwrap(),
        );
        return;
    }

    if expr_type.is_none() {
        eval_let_by_type_inference(e, name, &mut expr.clone().unwrap());
        return;
    }

    let provided_type = expr_type_to_object_type(&expr_type.clone().unwrap());
    if expr.is_none() {
        add_to_env(e, name, Object::Null, provided_type);
        return;
    }

    let evaluated_expr = match e.eval_expr(&mut expr.clone().unwrap()) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };

    if provided_type != object_to_type(&evaluated_expr) {
        e.error_handler.set_type_error(format!(
            "assigning value of type {} to variable '{name}' which has type {provided_type}",
            object_to_type(&evaluated_expr)
        ));
        return;
    }

    add_to_env(e, name, evaluated_expr, provided_type);
}

fn eval_let_by_type_inference(e: &mut Evaluator, name: &String, expr: &mut Expr) {
    let evaluated_expr = match e.eval_expr(expr) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };
    let infered_type = object_to_type(&evaluated_expr);
    add_to_env(e, name, evaluated_expr, infered_type);
}

fn add_to_env(e: &mut Evaluator, name: &String, object: Object, type_: Type) {
    if !e
        .env
        .borrow_mut()
        .add_entry(name.clone(), object, type_, true)
    {
        e.error_handler
            .set_name_error(format!("'{}' is already declared", name));
    }
}

fn eval_array_declaration(e: &mut Evaluator, name: &String, items_type: ExprType, init: Expr) {
    let items_type = match expr_type_to_object_type(&items_type) {
        Type::Array => {
            e.error_handler
                .set_type_error("Nested arrays not allowed".to_string());
            return;
        }
        type_ => type_,
    };

    let mut expr_array = match init {
        Expr::Literal(Literal::Array(expr)) => expr,
        _ => {
            e.error_handler
                .set_type_error(format!("Array '{}' miss initialized", name));
            return;
        }
    };

    let array = match e.eval_array_literal(Some(items_type), &mut expr_array) {
        Some(object) => object,
        None => return,
    };

    add_to_env(e, name, array, Type::Array);
}
