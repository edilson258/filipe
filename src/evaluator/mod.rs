pub mod environment;
mod evaluators;
pub mod flstdlib;
pub mod object;
mod runtime_error;
mod stdlib;
mod type_system;

use std::{cell::RefCell, rc::Rc};

use crate::ast::*;
use environment::Environment;
use evaluators::func_call_evaluator::eval_call_expr;
use evaluators::func_def_evaluator::eval_func_def;
use evaluators::let_evaluator::eval_let_stmt;
use object::Object;
use runtime_error::RuntimeErrorHandler;
use stdlib::FilipeArray;
use type_system::{has_same_type, object_to_type, Type};

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
    pub error_handler: RuntimeErrorHandler,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self {
            env,
            error_handler: RuntimeErrorHandler::new(),
        }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut output: Option<Object> = None;
        for stmt in program {
            let object = self.eval_stmt(&stmt);
            if self.error_handler.has_error() {
                eprintln!("{}", self.error_handler.get_error().unwrap());
                return None;
            }
            output = object;
        }
        output
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Option<Object> {
        match stmt {
            Stmt::Let(name, var_type, expr, flags) => {
                eval_let_stmt(self, name, var_type, expr, flags);
                None
            }
            Stmt::Func(identifier, params, body, ret_type) => {
                eval_func_def(self, identifier, params, body, ret_type);
                None
            }
            Stmt::Return(expr) => self.eval_return(expr),
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_stmt(condition, consequence, alternative),
            Stmt::ForLoop {
                cursor,
                iterable,
                block,
            } => self.eval_forloop_stmt(cursor, iterable, block),
        }
    }

    fn eval_forloop_stmt(
        &mut self,
        cursor: &String,
        iterable: &Expr,
        block: &BlockStmt,
    ) -> Option<Object> {
        let iterable_object = match self.eval_expr(iterable) {
            Some(object) => object,
            None => return None,
        };
        match iterable_object {
            Object::Range { start, end } => self.eval_range_forloop(cursor, start, end, block),
            _ => {
                self.error_handler
                    .set_type_error(format!("for loop works only with range (for now)"));
                return None;
            }
        }
    }

    fn eval_range_forloop(
        &mut self,
        cursor: &String,
        start: i64,
        end: i64,
        block: &BlockStmt,
    ) -> Option<Object> {
        let global_scope = Rc::clone(&self.env);
        let block_scope = Environment::empty(Some(Rc::clone(&global_scope)));
        self.env = Rc::new(RefCell::new(block_scope));

        self.env
            .borrow_mut()
            .add_entry(cursor.clone(), Object::Int(start), Type::Int, true);

        for _ in start..end {
            self.eval_block_stmt(block);
            let old_val = match self.env.borrow().resolve(&cursor).unwrap().value {
                Object::Int(val) => val,
                _ => return None,
            };
            self.env
                .borrow_mut()
                .update_entry(&cursor, Object::Int(old_val + 1));
        }

        self.env = global_scope;
        None
    }

    fn is_truthy(&mut self, object: Object) -> bool {
        match object {
            Object::Null | Object::Boolean(false) => false,
            Object::Int(val) => val != 0,
            Object::Float(val) => val != 0.0,
            _ => true,
        }
    }

    fn eval_if_stmt(
        &mut self,
        condition: &Expr,
        consequence: &BlockStmt,
        alternative: &Option<BlockStmt>,
    ) -> Option<Object> {
        let evaluated_cond = match self.eval_expr(condition) {
            Some(object) => object,
            None => return None,
        };

        if self.is_truthy(evaluated_cond) {
            return self.eval_block_stmt(consequence);
        }

        if alternative.is_some() {
            return self.eval_block_stmt(&alternative.clone().unwrap());
        }

        None
    }

    fn eval_expr(&mut self, expr: &Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => self.eval_literal_expr(literal),
            Expr::Identifier(identifier) => self.resolve_identfier(identifier),
            Expr::Call(func, args) => eval_call_expr(self, func, args),
            Expr::Infix(lhs, infix, rhs) => self.eval_infix_expr(lhs, infix, rhs),
            Expr::Prefix(prefix, expr) => self.eval_prefix_expr(prefix, expr),
            Expr::Postfix(expr, postfix) => self.eval_postfix_expr(expr, postfix),
            Expr::Assign(identifier, expr) => self.eval_assign_expr(identifier, expr),
        }
    }

    fn eval_postfix_expr(&mut self, expr: &Expr, postfix: &Postfix) -> Option<Object> {
        let evaluated_expr = match self.eval_expr(expr) {
            Some(object) => object,
            None => return None,
        };

        let old_value = match evaluated_expr {
            Object::Int(val) => val,
            _ => {
                self.error_handler.set_type_error(format!(
                    "'{}' operation is only allowed for type 'number'",
                    postfix
                ));
                return None;
            }
        };

        match postfix {
            Postfix::Increment => Some(Object::Int(old_value + 1)),
            Postfix::Decrement => Some(Object::Int(old_value - 1)),
        }
    }

    fn eval_prefix_expr(&mut self, prefix: &Prefix, expr: &Expr) -> Option<Object> {
        let evaluated_expr = match self.eval_expr(expr) {
            Some(object) => object,
            None => return None,
        };

        match prefix {
            Prefix::Not => self.eval_not_prefix(&evaluated_expr),
            Prefix::Plus => self.eval_plus_prefix(prefix, &evaluated_expr),
            Prefix::Minus => self.eval_minus_prefix(prefix, &evaluated_expr),
        }
    }

    fn eval_not_prefix(&mut self, evaluated_expr: &Object) -> Option<Object> {
        match evaluated_expr {
            Object::Null => Some(Object::Boolean(true)),
            Object::Boolean(val) => Some(Object::Boolean(!val)),
            _ => Some(Object::Boolean(false)),
        }
    }

    fn eval_plus_prefix(&mut self, prefix: &Prefix, evaluated_expr: &Object) -> Option<Object> {
        match evaluated_expr {
            Object::Int(val) => Some(Object::Int(val.clone())),
            Object::Float(val) => Some(Object::Float(val.clone())),
            _ => {
                self.error_handler
                    .set_type_error(format!("'{}' prefix is for type number", prefix));
                return None;
            }
        }
    }

    fn eval_minus_prefix(&mut self, prefix: &Prefix, evaluated_expr: &Object) -> Option<Object> {
        match evaluated_expr {
            Object::Int(val) => Some(Object::Int(-val)),
            Object::Float(val) => Some(Object::Float(-val)),
            _ => {
                self.error_handler
                    .set_type_error(format!("'{}' prefix is for type number", prefix));
                return None;
            }
        }
    }

    fn eval_return(&mut self, expr: &Option<Expr>) -> Option<Object> {
        if expr.is_none() {
            return Some(Object::RetVal(Box::new(Object::Null)));
        }
        match self.eval_expr(&expr.clone().unwrap()) {
            Some(object) => Some(Object::RetVal(Box::new(object))),
            None => None,
        }
    }

    fn eval_assign_expr(&mut self, identifier: &Identifier, expr: &Expr) -> Option<Object> {
        let Identifier(name) = identifier;
        if !self.env.borrow().is_declared(&name) {
            self.error_handler
                .set_name_error(format!("'{}' is not declared", &name));
            return None;
        }
        let value = match self.eval_expr(expr) {
            Some(value) => value,
            None => return None,
        };

        if let Some(Object::Array(_, old_items_type)) = self.resolve_identfier(&identifier) {
            return self.assign_array(name, old_items_type, value);
        }

        let old_value_type = self.env.borrow().get_typeof(&name).unwrap();
        let new_value_type = object_to_type(&value);

        if old_value_type != new_value_type {
            self.error_handler.set_type_error(format!(
                "can't assign value of type '{}' with value of type '{}'",
                old_value_type, new_value_type
            ));
            return None;
        }

        if !self.env.borrow_mut().update_entry(&name, value) {
            self.error_handler
                .set_name_error(format!("'{}' is not assignable", &name));
        }
        None
    }

    fn assign_array(&mut self, name: &String, old_type: Type, value: Object) -> Option<Object> {
        let (new_data, new_type) = match &value {
            Object::Array(new_data, new_type) => (new_data, new_type),
            _ => {
                println!("rhs is not array");
                return None;
            }
        };

        if new_type == &Type::Unknown {
            self.env.borrow_mut().update_entry(&name, Object::Array(new_data.to_owned(), old_type));
            return None;
        }

        if new_type != &old_type {
            self.error_handler.set_type_error(format!(
                "Assign array of type '{}' to array '{}' which has type '{}'",
                new_type, name, old_type
            ));
            return None;
        }

        self.env.borrow_mut().update_entry(&name, value);
        None
    }

    fn eval_block_stmt(&mut self, block: &BlockStmt) -> Option<Object> {
        let mut res = None;

        for stmt in block {
            match self.eval_stmt(stmt) {
                Some(Object::RetVal(object)) => return Some(*object),
                object => res = object,
            }
        }

        res
    }

    fn eval_infix_expr(&mut self, lhs: &Expr, infix: &Infix, rhs: &Expr) -> Option<Object> {
        let lhs = self.eval_expr(lhs);
        let rhs = self.eval_expr(rhs);

        if lhs.is_none() || rhs.is_none() {
            return None;
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        if !has_same_type(&lhs, &rhs) {
            self.error_handler.set_type_error(format!(
                "'{}' operation not allowed between types {} and {}",
                infix,
                object_to_type(&lhs),
                object_to_type(&rhs),
            ));
            return None;
        }

        match lhs {
            Object::Int(lval) => {
                if let Object::Int(rval) = rhs {
                    return Some(self.eval_infix_int_expr(lval, infix, rval));
                }
                None
            }
            Object::Float(lval) => {
                if let Object::Float(rval) = rhs {
                    return Some(self.eval_infix_float_expr(lval, infix, rval));
                }
                None
            }
            Object::String(lval) => {
                if let Object::String(rval) = rhs {
                    return Some(self.eval_infix_string_expr(&lval, infix, &rval));
                }
                None
            }
            Object::Boolean(lval) => {
                if let Object::Boolean(rval) = rhs {
                    return Some(self.eval_infix_bool_expr(&lval, infix, &rval));
                }
                None
            }
            _ => None,
        }
    }

    fn eval_infix_string_expr(&mut self, lhs: &String, infix: &Infix, rhs: &String) -> Object {
        match infix {
            Infix::Plus => Object::String(lhs.clone() + rhs),
            Infix::NotEqual => Object::Boolean(lhs != rhs),
            Infix::Equal => Object::Boolean(lhs == rhs),
            _ => {
                self.error_handler.set_type_error(format!(
                    "'{}' operation not implemented for type string",
                    infix
                ));
                Object::Null
            }
        }
    }

    fn eval_infix_int_expr(&mut self, lhs_val: i64, infix: &Infix, rhs_val: i64) -> Object {
        match infix {
            Infix::Plus => Object::Int(lhs_val + rhs_val),
            Infix::Minus => Object::Int(lhs_val - rhs_val),
            Infix::Devide => Object::Int(lhs_val / rhs_val),
            Infix::Multiply => Object::Int(lhs_val * rhs_val),
            Infix::Remainder => Object::Int(lhs_val % rhs_val),
            Infix::Equal => Object::Boolean(lhs_val == rhs_val),
            Infix::LessThan => Object::Boolean(lhs_val < rhs_val),
            Infix::LessOrEqual => Object::Boolean(lhs_val <= rhs_val),
            Infix::GratherThan => Object::Boolean(lhs_val > rhs_val),
            Infix::GratherOrEqual => Object::Boolean(lhs_val >= rhs_val),
            Infix::NotEqual => Object::Boolean(lhs_val != rhs_val),
        }
    }

    fn eval_infix_float_expr(&mut self, lhs_val: f64, infix: &Infix, rhs_val: f64) -> Object {
        match infix {
            Infix::Plus => Object::Float(lhs_val + rhs_val),
            Infix::Minus => Object::Float(lhs_val - rhs_val),
            Infix::Devide => Object::Float(lhs_val / rhs_val),
            Infix::Multiply => Object::Float(lhs_val * rhs_val),
            Infix::Remainder => Object::Float(lhs_val % rhs_val),
            Infix::Equal => Object::Boolean(lhs_val == rhs_val),
            Infix::LessThan => Object::Boolean(lhs_val < rhs_val),
            Infix::LessOrEqual => Object::Boolean(lhs_val <= rhs_val),
            Infix::GratherThan => Object::Boolean(lhs_val > rhs_val),
            Infix::GratherOrEqual => Object::Boolean(lhs_val >= rhs_val),
            Infix::NotEqual => Object::Boolean(lhs_val != rhs_val),
        }
    }

    fn eval_infix_bool_expr(&mut self, lhs_val: &bool, infix: &Infix, rhs_val: &bool) -> Object {
        match infix {
            Infix::Equal => Object::Boolean(lhs_val == rhs_val),
            Infix::LessThan => Object::Boolean(lhs_val < rhs_val),
            Infix::LessOrEqual => Object::Boolean(lhs_val <= rhs_val),
            Infix::GratherThan => Object::Boolean(lhs_val > rhs_val),
            Infix::GratherOrEqual => Object::Boolean(lhs_val >= rhs_val),
            Infix::NotEqual => Object::Boolean(lhs_val != rhs_val),
            _ => {
                self.error_handler.set_type_error(format!(
                    "'{}' operation not implemented for type boolean",
                    infix
                ));
                Object::Null
            }
        }
    }

    fn eval_literal_expr(&mut self, literal: &Literal) -> Option<Object> {
        match literal {
            Literal::String(val) => Some(Object::String(val.clone())),
            Literal::Boolean(val) => Some(Object::Boolean(*val)),
            Literal::Null => Some(Object::Null),
            Literal::Int(val) => Some(Object::Int(*val)),
            Literal::Float(val) => Some(Object::Float(*val)),
            Literal::Array(exprs) => self.eval_array_literal(exprs),
        }
    }

    fn eval_array_literal(&mut self, expr_array: &Vec<Expr>) -> Option<Object> {
        if expr_array.is_empty() {
            return Some(Object::Array(FilipeArray::new(vec![]), Type::Unknown));
        }

        let mut expr_array = expr_array.to_owned();
        let first_item = match self.eval_expr(&expr_array.remove(0)) {
            Some(object) => object,
            None => return None,
        };

        let first_item_type = object_to_type(&first_item);

        let mut objects: Vec<Object> = vec![];
        objects.push(first_item);

        for expr in expr_array {
            let item = match self.eval_expr(&expr) {
                Some(obj) => obj,
                None => return None,
            };

            if first_item_type != object_to_type(&item) {
                self.error_handler
                    .set_type_error("Array item's type mismatch".to_string());
                return None;
            }
            objects.push(item);
        }
        Some(Object::Array(FilipeArray::new(objects), first_item_type))
    }

    fn resolve_identfier(&mut self, identifier: &Identifier) -> Option<Object> {
        let Identifier(name) = identifier;
        let object = match self.env.borrow().resolve(&name) {
            Some(object) => object,
            None => {
                self.error_handler
                    .set_name_error(format!("'{}' is not declared", &name));
                return None;
            }
        };
        Some(object.value)
    }

    fn expr_to_identifier(expr: &Expr) -> Option<Identifier> {
        match expr {
            Expr::Identifier(ident) => Some(ident.clone()),
            _ => None,
        }
    }
}
