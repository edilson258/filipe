pub mod environment;
pub mod flstdlib;
pub mod object;

use crate::ast::*;
use environment::Environment;
use object::Object;

pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut output: Option<Object> = None;
        for stmt in program {
            match self.eval_stmt(stmt) {
                Some(Object::Error(err)) => {
                    eprintln!("[Error] {}", err);
                    break;
                }
                obj => output = obj,
            }
        }
        output
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => Some(self.eval_literal_expr(literal)),
            Expr::Identifier(identifier) => self.resolve_identfier(identifier),
            Expr::Call { func, args } => self.eval_call_expr(*func, args),
            Expr::Infix(lhs, infix, rhs) => {
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
                if lhs.is_some() && rhs.is_some() {
                    return self.eval_infix_expr(lhs.unwrap(), infix, rhs.unwrap());
                }
                Some(Object::Error(format!("Invalid operation {:?} {} {:?}", lhs, infix, rhs)))
            }
        }
    }

    fn eval_call_expr(&mut self, func: Expr, provided_args: Vec<Expr>) -> Option<Object> {
        let mut args: Vec<Object> = vec![];
        for arg in provided_args {
            let arg = match self.eval_expr(arg) {
                Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                Some(object) => object,
                None => Object::Null,
            };
            args.push(arg);
        }

        match self.eval_expr(func) {
            Some(Object::Builtin(arity, f)) => {
                if arity != args.len() {
                    Some(Object::Error(format!(
                        "Incorrect Arguments: expected {}, but provided {}",
                        arity,
                        args.len()
                    )));
                }
                Some(f(args))
            }
            _ => Some(Object::Error(format!("Is not callable"))),
        }
    }

    fn eval_infix_expr(&mut self, lhs: Object, infix: Infix, rhs: Object) -> Option<Object> {
        match lhs.clone() {
            Object::Number(lhs_val) => {
                if let Object::Number(rhs_val) = rhs {
                    Some(self.eval_infix_number_expr(lhs_val, infix, rhs_val))
                } else {
                    Some(Object::Error(format!(
                        "Type miss match {} {} {}",
                        lhs, infix, rhs
                    )))
                }
            }
            Object::String(lhs_val) => {
                if let Object::String(rhs_val) = rhs {
                    Some(self.eval_infix_string_expr(lhs_val, infix, rhs_val))
                } else {
                    Some(Object::Error(format!(
                        "Type miss match {} {} {}",
                        lhs, infix, rhs
                    )))
                }
            }
            _ => Some(Object::Error(format!("Invalid operation {} {} {}", lhs, infix, rhs))),
        }
    }

    fn eval_infix_string_expr(&mut self, lhs: String, infix: Infix, rhs: String) -> Object {
        match infix {
            Infix::Plus => Object::String(lhs + &rhs),
            _ => Object::Error(format!("Invalid operation {} {} {}", lhs, infix, lhs)),
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
        match self.env.resolve(&name) {
            Some(o) => Some(o),
            None => Some(Object::Error(format!("Name Error: Undeclared {}", name))),
        }
    }
}
