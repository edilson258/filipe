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

    if e.env.borrow().is_declared(&name) {
        e.error_handler.set_name_error(format!("'{}' already declared", name));
        return;
    }

    if expr_type.is_none() && expr.is_none() {
        e.error_handler.set_type_error(format!(
            "cannot infer type of '{}', define it's type or initialize it",
            &name
        ));
        return;
    }

    if flags.is_array {
        if expr.is_none() {
            e.error_handler
                .set_type_error(format!("Missing init value of '{}' array", name));
            return;
        }

        eval_array_declaration(e, name, expr_type, expr.to_owned().unwrap());
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

fn eval_array_declaration(
    e: &mut Evaluator,
    name: &String,
    items_type: &Option<ExprType>,
    init: Expr,
) {
    if items_type.is_some() {
        let expected_items_type = match expr_type_to_object_type(&items_type.clone().unwrap()) {
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

        let array = match e.eval_array_literal(&mut expr_array) {
            Some(object) => object,
            None => return,
        };

        if let Object::Array(array_data, provided_items_type) = &array {
            if provided_items_type == &Type::Unknown {
                add_to_env(e, name, Object::Array(array_data.to_owned(), expected_items_type), Type::Array);
                return;
            }

            if provided_items_type != &expected_items_type {
                e.error_handler.set_type_error(format!(
                "Defined '{}' to hold values of type '{}' but initialized with values of type '{}'",
                name, &expected_items_type, &provided_items_type
            ));
                return;
            }
            add_to_env(e, name, array, Type::Array);
        } else {
            e.error_handler
                .set_type_error(format!("Expected 'Array' but provided '{}'", array))
        }
        return;
    }

    let array_literal = match init {
        Expr::Literal(Literal::Array(items)) => items,
        _ => {
            e.error_handler
                .set_type_error(format!("Array '{}' miss initialized", name));
            return;
        }
    };

    let array_object = match e.eval_array_literal(&array_literal) {
        Some(object) => object,
        None => return
    };

    add_to_env(e, name, array_object, Type::Array);
}
