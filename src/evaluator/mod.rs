pub mod environment;
pub mod flstdlib;
pub mod object;

use core::fmt;

use crate::ast::*;
use environment::Environment;
use object::Object;

use self::object::{BuiltInFuncRetVal, object_to_type};

#[derive(Clone)]
enum RuntimeErrorKind {
    NameError,
    InvalidOp,
    TypeError,
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameError => write!(f, "[Name error]"),
            Self::InvalidOp => write!(f, "[Invalid Operation]"),
            Self::TypeError => write!(f, "[Type Error]"),
        }
    }
}

#[derive(Clone)]
pub struct RuntimeError {
    kind: RuntimeErrorKind,
    msg: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.msg)
    }
}

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
            Stmt::Let(name, expr) => self.eval_let_stmt(name, expr),
            Stmt::Func(identifier, params, body) => self.eval_func(identifier, params, body),
            Stmt::Return(expr) => self.eval_return(expr),
            Stmt::Expr(expr) => self.eval_expr(expr),
        }
    }

    fn eval_let_stmt(&mut self, name: Identifier, expr: Option<Expr>) -> Option<Object> {
        let Identifier(name) = name;
        if expr.is_none() {
            if !self.env.add_entry(name.clone(), Object::Null, true) {
                self.set_error(
                    RuntimeErrorKind::NameError,
                    format!("'{}' is already declared", name),
                );
            }
            return None;
        }
        let value = match self.eval_expr(expr.unwrap()) {
            Some(obj) => obj,
            _ => return None,
        };

        if !self.env.add_entry(name.clone(), value, true) {
            self.set_error(
                RuntimeErrorKind::NameError,
                format!("'{}' is already declared", name),
            );
        }
        return None;
    }

    fn eval_func(
        &mut self,
        identifier: Identifier,
        params: Vec<Identifier>,
        body: BlockStmt,
    ) -> Option<Object> {
        let Identifier(name) = identifier;
        if !self.env.add_entry(
            name.clone(),
            Object::Func(name.clone(), params, body),
            false,
        ) {
            self.set_error(
                RuntimeErrorKind::NameError,
                format!("'{}' is already declared", name),
            );
        }
        Some(Object::Null)
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

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => Some(self.eval_literal_expr(literal)),
            Expr::Identifier(identifier) => self.resolve_identfier(identifier),
            Expr::Call(func, args) => self.eval_call_expr(*func, args),
            Expr::Infix(lhs, infix, rhs) => self.eval_infix_expr(*lhs, infix, *rhs),
            Expr::Assign(identifier, expr) => self.eval_assign_expr(identifier, *expr),
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

    fn eval_call_expr(&mut self, func: Expr, provided_args: Vec<Expr>) -> Option<Object> {
        let mut args: Vec<Object> = vec![];
        for arg in provided_args {
            let arg = match self.eval_expr(arg) {
                Some(object) => object,
                None => return None,
            };
            args.push(arg);
        }

        let func_name = match Self::expr_to_identifier(&func) {
            Some(identifier) => {
                let Identifier(name) = identifier;
                name
            }
            None => {
                self.set_error(
                    RuntimeErrorKind::TypeError,
                    format!("invalid function name {:?}", func),
                );
                return None;
            }
        };

        let func_object = match self.eval_expr(func) {
            Some(expr) => expr,
            None => return None,
        };

        let (name, params, body) = match func_object.clone() {
            Object::BuiltinFn(builtin_fn) => match builtin_fn(args) {
                BuiltInFuncRetVal::Object(object) => return Some(object),
                BuiltInFuncRetVal::Error(err) => {
                    self.set_error(err.kind, err.msg);
                    return None;
                }
            },
            Object::Func(name, params, body) => (name, params, body),
            _ => {
                self.set_error(
                    RuntimeErrorKind::TypeError,
                    format!("'{}' is not callable", func_name),
                );
                return None;
            }
        };
        if params.len() != args.len() {
            self.set_error(
                RuntimeErrorKind::TypeError,
                format!(
                    "Function '{}' expecteds {} args but provided {}",
                    name,
                    params.len(),
                    args.len()
                ),
            );
            return None;
        }
        let global_scope = self.env.clone();
        let mut fn_scope = Environment::empty(Some(self.env.clone()));
        let list = params.iter().zip(args);
        for (_, (ident, o)) in list.enumerate() {
            let Identifier(name) = ident;
            fn_scope.add_entry(name.clone(), o, true);
        }
        *self.env = fn_scope;
        let ret_val = self.eval_block_stmt(body);
        *self.env = global_scope;
        Some(ret_val)
    }

    fn expr_to_identifier(expr: &Expr) -> Option<Identifier> {
        match expr {
            Expr::Identifier(ident) => Some(ident.clone()),
            _ => None,
        }
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
            },
            Object::String(lval) => {
                if let Object::String(rval) = rhs {
                    return Some(self.eval_infix_string_expr(lval, infix, rval));
                }
                None
            },
            Object::Boolean(lval) => {
                if let Object::Boolean(rval) = rhs {
                    return Some(self.eval_infix_bool_expr(lval, infix, rval));
                }
                None
            },
            _ => None
        }
    }

    fn has_same_type(&self, lhs: &Object, rhs: &Object) -> bool {
        object_to_type(lhs) == object_to_type(rhs)
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
}
