pub mod environment;
pub mod flstdlib;
pub mod object;

use core::fmt;

use crate::ast::*;
use environment::Environment;
use object::Object;

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
struct RuntimeError {
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
            Stmt::Func(identifier, params, body) => {
                let Identifier(name) = identifier;
                self.env.set(name.clone(), Object::Func(name, params, body));
                Some(Object::Null)
            }
            Stmt::Return(expr) => {
                if expr.is_none() {
                    return Some(Object::Null);
                }
                self.eval_expr(expr.unwrap())
            }
            Stmt::Expr(expr) => self.eval_expr(expr),
        }
    }

    fn eval_let_stmt(&mut self, name: Identifier, expr: Option<Expr>) -> Option<Object> {
        let Identifier(name) = name;
        if expr.is_none() {
            self.env.set(name, Object::Null);
            return None;
        }
        let value = match self.eval_expr(expr.unwrap()) {
            Some(obj) => obj,
            _ => return None,
        };
        self.env.set(name, value);
        None
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
        self.env.set(name, value);
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
            Object::Builtin(builtin_fn) => {
                return Some(builtin_fn(args));
            }
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
            fn_scope.set(name.clone(), o);
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
                Some(Object::RetVal(val)) => return Object::RetVal(val),
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
        match lhs.clone().unwrap() {
            Object::Number(lhs_val) => {
                if let Object::Number(rhs_val) = rhs.clone().unwrap() {
                    Some(self.eval_infix_number_expr(lhs_val, infix, rhs_val))
                } else {
                    self.set_error(
                        RuntimeErrorKind::TypeError,
                        format!(
                            "'{}' operation not allowed between {} and {}",
                            infix,
                            lhs.unwrap(),
                            rhs.unwrap()
                        ),
                    );
                    return None;
                }
            }
            Object::String(lhs_val) => {
                if let Object::String(rhs_val) = rhs.clone().unwrap() {
                    Some(self.eval_infix_string_expr(lhs_val, infix, rhs_val))
                } else {
                    self.set_error(
                        RuntimeErrorKind::TypeError,
                        format!(
                            "'{}' operation not allowed between {} and {}",
                            infix,
                            lhs.unwrap(),
                            rhs.unwrap()
                        ),
                    );
                    return None;
                }
            }
            _ => {
                self.set_error(
                    RuntimeErrorKind::TypeError,
                    format!(
                        "'{}' operation not allowed between {} and {}",
                        infix,
                        lhs.unwrap(),
                        rhs.unwrap()
                    ),
                );
                None
            }
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
        }
    }

    fn eval_literal_expr(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::String(val) => Object::String(val),
            Literal::Number(val) => Object::Number(val as f64),
        }
    }

    fn resolve_identfier(&mut self, identifier: Identifier) -> Option<Object> {
        let Identifier(name) = identifier;
        let obj = match self.env.resolve(&name) {
            Some(o) => Some(o),
            None => {
                self.set_error(
                    RuntimeErrorKind::NameError,
                    format!("undeclared '{}'", name),
                );
                return None;
            }
        };
        obj
    }

    fn get_error(&self) -> Option<RuntimeError> {
        return self.error.clone();
    }

    fn set_error(&mut self, kind: RuntimeErrorKind, msg: String) {
        self.error = Some(RuntimeError { kind, msg });
    }
}
