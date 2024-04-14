pub mod environment;
mod evaluators;
pub mod flstdlib;
pub mod object;
mod runtime_error;

use crate::ast::*;
use environment::Environment;
use evaluators::func_call_evaluator::eval_call_expr;
use evaluators::func_def_evaluator::eval_func_def;
use evaluators::let_evaluator::eval_let_stmt;
use object::Object;
use object::{object_to_type, Type};
use runtime_error::{RuntimeError, RuntimeErrorKind};

pub struct Evaluator<'a> {
    env: &'a mut Environment,
    error: Option<RuntimeError>,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self { env, error: None }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut output: Option<Object> = None;
        for stmt in program {
            let object = self.eval_stmt(stmt);
            if self.get_error().is_some() {
                eprintln!("{}", self.get_error().unwrap());
                return None;
            }
            output = object;
        }
        output
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Let(name, var_type, expr) => {
                eval_let_stmt(self, name, var_type, expr);
                None
            }
            Stmt::Func(identifier, params, body, ret_type) => {
                eval_func_def(self, identifier, params, body, ret_type);
                None
            }
            Stmt::Return(expr) => self.eval_return(expr),
            Stmt::Expr(expr) => self.eval_expr(expr),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => Some(self.eval_literal_expr(literal)),
            Expr::Identifier(identifier) => self.resolve_identfier(identifier),
            Expr::Call(func, args) => eval_call_expr(self, *func, args),
            Expr::Infix(lhs, infix, rhs) => self.eval_infix_expr(*lhs, infix, *rhs),
            Expr::Assign(identifier, expr) => self.eval_assign_expr(identifier, *expr),
        }
    }

    fn eval_return(&mut self, expr: Option<Expr>) -> Option<Object> {
        if expr.is_none() {
            return Some(Object::RetVal(Box::new(Object::Null)));
        }
        match self.eval_expr(expr.unwrap()) {
            Some(object) => Some(Object::RetVal(Box::new(object))),
            None => None,
        }
    }

    fn eval_assign_expr(&mut self, identifier: Identifier, expr: Expr) -> Option<Object> {
        let Identifier(name) = identifier;
        if !self.env.is_declared(&name) {
            self.set_error(
                RuntimeErrorKind::NameError,
                format!("{} is not declared", name),
            );
            return None;
        }
        let value = match self.eval_expr(expr) {
            Some(value) => value,
            None => return None,
        };
        if !self.env.update_entry(&name, value) {
            self.set_error(
                RuntimeErrorKind::NameError,
                format!("{} is not assignable", name),
            )
        }
        None
    }

    fn eval_block_stmt(&mut self, block: BlockStmt) -> Object {
        let mut res = None;
        for stmt in block {
            match self.eval_stmt(stmt) {
                Some(Object::RetVal(val)) => return *val,
                object => res = object,
            }
        }
        match res {
            Some(object) => object,
            None => Object::Null,
        }
    }

    fn eval_infix_expr(&mut self, lhs: Expr, infix: Infix, rhs: Expr) -> Option<Object> {
        let lhs = self.eval_expr(lhs);
        let rhs = self.eval_expr(rhs);

        if lhs.is_none() || rhs.is_none() {
            return None;
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        if !self.has_same_type(&lhs, &rhs) {
            self.set_error(
                RuntimeErrorKind::TypeError,
                format!(
                    "'{}' operation not allowed between types {} and {}",
                    infix,
                    object_to_type(&lhs),
                    object_to_type(&rhs),
                ),
            );
            return None;
        }

        match lhs {
            Object::Number(lval) => {
                if let Object::Number(rval) = rhs {
                    return Some(self.eval_infix_number_expr(lval, infix, rval));
                }
                None
            }
            Object::String(lval) => {
                if let Object::String(rval) = rhs {
                    return Some(self.eval_infix_string_expr(lval, infix, rval));
                }
                None
            }
            Object::Boolean(lval) => {
                if let Object::Boolean(rval) = rhs {
                    return Some(self.eval_infix_bool_expr(lval, infix, rval));
                }
                None
            }
            _ => None,
        }
    }

    fn eval_infix_string_expr(&mut self, lhs: String, infix: Infix, rhs: String) -> Object {
        match infix {
            Infix::Plus => Object::String(lhs + &rhs),
            _ => {
                self.set_error(
                    RuntimeErrorKind::InvalidOp,
                    format!(
                        "{} operation not allowed between '{}' and '{}'",
                        infix, lhs, rhs
                    ),
                );
                Object::Null
            }
        }
    }

    fn eval_infix_number_expr(&mut self, lhs_val: f64, infix: Infix, rhs_val: f64) -> Object {
        match infix {
            Infix::Plus => Object::Number(lhs_val + rhs_val),
            Infix::Minus => Object::Number(lhs_val - rhs_val),
            Infix::Devide => Object::Number(lhs_val / rhs_val),
            Infix::Multiply => Object::Number(lhs_val * rhs_val),
            Infix::Equal => Object::Boolean(lhs_val == rhs_val),
            Infix::LessThan => Object::Boolean(lhs_val < rhs_val),
            Infix::LessOrEqual => Object::Boolean(lhs_val <= rhs_val),
            Infix::GratherThan => Object::Boolean(lhs_val > rhs_val),
            Infix::GratherOrEqual => Object::Boolean(lhs_val >= rhs_val),
        }
    }

    fn eval_infix_bool_expr(&mut self, lhs_val: bool, infix: Infix, rhs_val: bool) -> Object {
        match infix {
            Infix::Equal => Object::Boolean(lhs_val == rhs_val),
            Infix::LessThan => Object::Boolean(lhs_val < rhs_val),
            Infix::LessOrEqual => Object::Boolean(lhs_val <= rhs_val),
            Infix::GratherThan => Object::Boolean(lhs_val > rhs_val),
            Infix::GratherOrEqual => Object::Boolean(lhs_val >= rhs_val),
            _ => {
                self.set_error(
                    RuntimeErrorKind::InvalidOp,
                    format!(
                        "{} operation not allowed between '{}' and '{}'",
                        infix, lhs_val, rhs_val
                    ),
                );
                Object::Null
            }
        }
    }

    fn eval_literal_expr(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::String(val) => Object::String(val),
            Literal::Number(val) => Object::Number(val as f64),
            Literal::Boolean(val) => Object::Boolean(val),
            Literal::Null => Object::Null,
        }
    }

    fn resolve_identfier(&mut self, identifier: Identifier) -> Option<Object> {
        let Identifier(name) = identifier;
        let object = match self.env.resolve(&name) {
            Some(object) => object,
            None => {
                self.set_error(
                    RuntimeErrorKind::NameError,
                    format!("undeclared '{}'", name),
                );
                return None;
            }
        };
        Some(object.value)
    }

    fn get_error(&self) -> Option<RuntimeError> {
        return self.error.clone();
    }

    fn set_error(&mut self, kind: RuntimeErrorKind, msg: String) {
        self.error = Some(RuntimeError { kind, msg });
    }

    fn expr_type_to_object_type(&mut self, var_type: ExprType) -> Type {
        match var_type {
            ExprType::String => Type::String,
            ExprType::Number => Type::Number,
            ExprType::Boolean => Type::Boolean,
            ExprType::Null => Type::Null,
        }
    }

    fn expr_to_type(&mut self, expr: Expr) -> Option<Type> {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::String(_) => return Some(Type::String),
                Literal::Null => return Some(Type::Null),
                Literal::Number(_) => return Some(Type::Number),
                Literal::Boolean(_) => return Some(Type::Boolean),
            },
            Expr::Identifier(identifier) => return self.identifier_to_type(identifier),
            _ => {
                return None;
            }
        }
    }

    fn has_same_type(&self, lhs: &Object, rhs: &Object) -> bool {
        object_to_type(lhs) == object_to_type(rhs)
    }

    fn identifier_to_type(&mut self, identifier: Identifier) -> Option<Type> {
        let Identifier(name) = identifier;

        match self.env.get_typeof(&name) {
            Some(type_) => Some(type_),
            None => {
                self.set_error(
                    RuntimeErrorKind::NameError,
                    format!("Couldn't resolve {}'s type maybe it's not declared", &name),
                );
                return None;
            }
        }
    }

    fn expr_to_identifier(expr: &Expr) -> Option<Identifier> {
        match expr {
            Expr::Identifier(ident) => Some(ident.clone()),
            _ => None,
        }
    }
}
