mod object;

use crate::ast::*;
use object::Object;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut output: Option<Object> = None;
        for stmt in program {
            match self.eval_stmt(stmt) {
                Some(o) => output = Some(o),
                None => {}
            }
        }
        output
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr)
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => Some(self.eval_literal_expr(literal)),
            Expr::Identifier(_) => None,
            Expr::Infix(lhs, infix, rhs) => {
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
                if lhs.is_some() && rhs.is_some() {
                    return self.eval_infix_expr(lhs.unwrap(), infix, rhs.unwrap());
                }
                return None;
            },
            _ => None
        }
    }

    fn eval_infix_expr(&mut self, lhs: Object, infix: Infix, rhs: Object) -> Option<Object> {
        match lhs {
            Object::Number(lhs_val) => {
                if let Object::Number(rhs_val) = rhs {
                    return Some(self.eval_infix_number_expr(lhs_val, infix, rhs_val))
                } else {
                    eprintln!("type mismtach: {} {} {}", lhs, infix, rhs);
                    return None;
                }
            },
            _ => None
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
}
