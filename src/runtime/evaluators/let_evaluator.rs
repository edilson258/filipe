use crate::runtime::{
    object_to_type, stdlib::FilipeArray, type_system::expr_type_to_object_type, Expr, ExprType,
    Object, Runtime, Type,
};

pub fn eval_let_stmt(
    rt: &mut Runtime,
    name: String,
    expr_type: Option<ExprType>,
    expr: Option<Expr>,
) {
    if rt.env.borrow().has(&name) {
        rt.error_handler
            .set_name_error(format!("'{}' already declared", name));
        return;
    }

    if expr_type.is_none() && expr.is_none() {
        rt.error_handler.set_type_error(format!(
            "Can't infer type of '{}', define it's type or initialize it",
            &name
        ));
        return;
    }

    if expr_type.is_none() {
        eval_let_by_type_inference(rt, name, expr.unwrap());
        return;
    }

    let expected_type = expr_type_to_object_type(&expr_type.clone().unwrap());

    if Type::Void == expected_type {
        rt.error_handler
            .set_type_error(format!("Can't declared var of type 'void'"));
        return;
    }

    if let Type::Array(Some(generic)) = expected_type.clone() {

        if Type::Void == *generic {
            rt.error_handler
                .set_type_error(format!("Can't declared array of type 'void'"));
            return;
        }

        if expr.is_none() {
            add_to_env(
                rt,
                &name,
                Object::Array {
                    inner: FilipeArray::new(vec![]),
                    items_type: Some(*generic),
                },
                expected_type,
            );
            return;
        }

        let evaluated_expr = match rt.eval_expr(expr.unwrap()) {
            Some(object) => object,
            None => return,
        };

        let evaluated_expr_type = object_to_type(&evaluated_expr);

        if let Type::Array(None) = evaluated_expr_type {
            add_to_env(
                rt,
                &name,
                Object::Array {
                    inner: FilipeArray::new(vec![]),
                    items_type: Some(*generic),
                },
                expected_type,
            );
            return;
        }
        add_to_env(rt, &name, evaluated_expr, evaluated_expr_type);
        return;
    }

    if expr.is_none() {
        add_to_env(rt, &name, Object::Null, expected_type);
        return;
    }

    let evaluated_expr = match rt.eval_expr(expr.unwrap()) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };

    let evaluated_expr_type = object_to_type(&evaluated_expr);

    if expected_type != evaluated_expr_type {
        rt.error_handler.set_type_error(format!(
            "Assigning value of type {} to variable '{name}' which has type {}",
            expected_type,
            object_to_type(&evaluated_expr)
        ));
        return;
    }

    add_to_env(rt, &name, evaluated_expr, expected_type);
}

fn eval_let_by_type_inference(e: &mut Runtime, name: String, expr: Expr) {
    let evaluated_expr = match e.eval_expr(expr) {
        Some(evaluated_expr) => evaluated_expr,
        None => return,
    };

    if let Object::Array {
        inner: _,
        items_type: None,
    } = evaluated_expr
    {
        e.error_handler
            .set_type_error(format!("Can't infer type of array '{}'", name));
        return;
    }

    let infered_type = object_to_type(&evaluated_expr);
    add_to_env(e, &name, evaluated_expr, infered_type);
}

fn add_to_env(e: &mut Runtime, name: &String, object: Object, type_: Type) {
    e.env.borrow_mut().set(name.clone(), type_, object, true);
}
