use crate::runtime::{
    object_to_type, type_system::expr_type_to_object_type, Expr, ExprType, Identifier,
    LetStmtFlags, Literal, Object, Runtime, Type,
};

pub fn eval_let_stmt(
    rt: &mut Runtime,
    identifier: Identifier,
    expr_type: Option<ExprType>,
    expr: Option<Expr>,
    flags: LetStmtFlags,
) {
    let Identifier(name) = identifier;

    if rt.env.borrow().is_declared(&name) {
        rt.error_handler
            .set_name_error(format!("'{}' already declared", name));
        return;
    }

    if expr_type.is_none() && expr.is_none() {
        rt.error_handler.set_type_error(format!(
            "cannot infer type of '{}', define it's type or initialize it",
            &name
        ));
        return;
    }

    if flags.is_array {
        return eval_array_declaration(rt, name, expr_type, expr.to_owned().unwrap());
    }

    if expr_type.is_none() {
        eval_let_by_type_inference(rt, name, expr.unwrap());
        return;
    }

    let provided_type = expr_type_to_object_type(&expr_type.clone().unwrap());
    if expr.is_none() {
        add_to_env(rt, &name, Object::Null, provided_type);
        return;
    }

    let evaluated_expr = match rt.eval_expr(expr.unwrap()) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };

    if provided_type != object_to_type(&evaluated_expr) {
        rt.error_handler.set_type_error(format!(
            "assigning value of type {} to variable '{name}' which has type {provided_type}",
            object_to_type(&evaluated_expr)
        ));
        return;
    }

    add_to_env(rt, &name, evaluated_expr, provided_type);
}

fn eval_let_by_type_inference(e: &mut Runtime, name: String, expr: Expr) {
    let evaluated_expr = match e.eval_expr(expr) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };
    let infered_type = object_to_type(&evaluated_expr);
    add_to_env(e, &name, evaluated_expr, infered_type);
}

fn add_to_env(e: &mut Runtime, name: &String, object: Object, type_: Type) {
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
    e: &mut Runtime,
    name: String,
    generic_type: Option<ExprType>,
    array_expr: Expr,
) {
    if generic_type.is_none() {
        let array_literal = match array_expr {
            Expr::Literal(Literal::Array(items)) => items,
            _ => {
                e.error_handler.set_type_error(format!("Invalid '{name}'"));
                return;
            }
        };

        let array_object = match e.eval_array_literal(array_literal) {
            Some(object) => object,
            None => return,
        };

        add_to_env(e, &name, array_object, Type::Array);
        return;
    }

    let generic_type = generic_type.unwrap();
        
    let expected_items_type = match expr_type_to_object_type(&generic_type) {
        Type::Array => {
            e.error_handler
                .set_type_error("Nested arrays not allowed".to_string());
            return;
        }
        type_ => type_,
    };

    let array_literal = match array_expr {
        Expr::Literal(Literal::Array(array_literal)) => array_literal,
        _ => {
            e.error_handler
                .set_type_error(format!("Array '{}' miss initialized", name));
            return;
        }
    };

    let array_object = match e.eval_array_literal(array_literal) {
        Some(object) => object,
        None => return,
    };

    if let Object::Array(array_data, provided_items_type) = &array_object {
        if provided_items_type == &Type::Unknown {
            let array_object = Object::Array(array_data.to_owned(), expected_items_type);
            add_to_env(e, &name, array_object, Type::Array);
            return;
        }

        if provided_items_type != &expected_items_type {
            e.error_handler.set_type_error(format!(
                "Defined '{}' to hold values of type '{}' but initialized with values of type '{}'",
                name, &expected_items_type, &provided_items_type
            ));
            return;
        }
        add_to_env(e, &name, array_object, Type::Array);
        return;
    }

    e.error_handler
        .set_type_error(format!("Expected 'Array' but provided '{}'", array_object));
}
