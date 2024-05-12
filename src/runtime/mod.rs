pub mod context;
mod evaluators;
pub mod flstdlib;
pub mod object;
pub mod runtime_error;
pub mod type_system;

use std::{cell::RefCell, rc::Rc};

use self::object::ObjectInfo;
use crate::frontend::ast::*;
use crate::stdlib::modules::Module;
use crate::stdlib::FilipeArray;
use context::{Context, ContextType};
use evaluators::func_call_evaluator::eval_call_expr;
use evaluators::func_def_evaluator::eval_func_def;
use evaluators::let_evaluator::eval_let_stmt;
use object::Object;
use runtime_error::RuntimeErrorHandler;
use type_system::Type;

pub struct Runtime {
    env: Rc<RefCell<Context>>,
    pub error_handler: RuntimeErrorHandler,
}

impl Runtime {
    pub fn new(env: Rc<RefCell<Context>>) -> Self {
        Self {
            env,
            error_handler: RuntimeErrorHandler::new(),
        }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut output: Option<Object> = None;
        for stmt in program {
            let object = self.eval_stmt(stmt);
            if self.error_handler.has_error() {
                eprintln!("{}", self.error_handler.get_error().unwrap());
                return None;
            }
            output = object;
        }
        output
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Let(Identifier(name), type_, expr) => {
                eval_let_stmt(self, name, type_, expr);
                None
            }
            Stmt::Func(Identifier(name), params, body, ret_type) => {
                eval_func_def(self, name, &params, &body, &ret_type);
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
        cursor: String,
        iterable: Expr,
        block: BlockStmt,
    ) -> Option<Object> {
        let iterable_object = match self.eval_expr(iterable) {
            Some(object) => object,
            None => return None,
        };
        match iterable_object {
            Object::Range { start, end, step } => {
                self.eval_range_forloop(cursor, start, end, step, block)
            }
            _ => {
                self.error_handler
                    .set_type_error(format!("for loop works only with range (for now)"));
                return None;
            }
        }
    }

    fn eval_range_forloop(
        &mut self,
        cursor: String,
        start: i64,
        end: i64,
        step: i64,
        block: BlockStmt,
    ) -> Option<Object> {
        let parent_scope = Rc::clone(&self.env);
        let loop_scope = Context::make_from(Rc::clone(&parent_scope), ContextType::Loop);
        self.env = Rc::new(RefCell::new(loop_scope));

        self.env
            .borrow_mut()
            .set(cursor.clone(), Type::Int, Object::Int(start), true);

        for _ in (start..end).step_by(step as usize) {
            let evalted_block = self.eval_block_stmt(&block);

            if self.error_handler.has_error() {
                return None;
            }

            if evalted_block.is_none() {
                continue;
            }

            match evalted_block.unwrap() {
                Object::RetVal(val) => return Some(Object::RetVal(val)),
                _ => {}
            }

            // update counter
            let old_val = match self.env.borrow().resolve(&cursor).unwrap().value {
                Object::Int(val) => val,
                _ => return None,
            };
            let incrementor = if step == 0 { 1 } else { step };
            self.env
                .borrow_mut()
                .mutate(cursor.clone(), Object::Int(old_val + incrementor));
        }

        self.env = parent_scope;
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
        condition: Expr,
        consequence: BlockStmt,
        alternative: Option<BlockStmt>,
    ) -> Option<Object> {
        let evaluated_cond = match self.eval_expr(condition) {
            Some(object) => object,
            None => return None,
        };

        let parent_scope = Rc::clone(&self.env);
        let ifelse_scope = Context::make_from(Rc::clone(&parent_scope), ContextType::IfElse);
        self.env = Rc::new(RefCell::new(ifelse_scope));

        if self.is_truthy(evaluated_cond) {
            return self.eval_block_stmt(&consequence);
        }

        if alternative.is_some() {
            return self.eval_block_stmt(&alternative.unwrap());
        }

        self.env = parent_scope;
        None
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => self.eval_literal_expr(literal),
            Expr::Identifier(identifier) => self.resolve_identfier(identifier),
            Expr::Call(func, args) => eval_call_expr(self, *func, args),
            Expr::Infix(lhs, infix, rhs) => self.eval_infix_expr(*lhs, infix, *rhs),
            Expr::Prefix(prefix, expr) => self.eval_prefix_expr(prefix, *expr),
            Expr::Postfix(expr, postfix) => self.eval_postfix_expr(*expr, postfix),
            Expr::Assign(identifier, expr) => self.eval_assign_expr(identifier, *expr),
            Expr::FieldAcc(src, target) => self.eval_field_access(*src, *target),
        }
    }

    fn eval_field_access(&mut self, src: Expr, target: Expr) -> Option<Object> {
        let src = match self.eval_expr(src) {
            Some(object) => object,
            None => return None,
        };

        match src {
            Object::Module(module) => self.eval_module_field_acc(module, target),
            _ => {
                self.error_handler
                    .set_sematic("Member access impl for modules only".to_string());
                return None;
            }
        }
    }

    fn eval_module_field_acc(&mut self, module: Module, target: Expr) -> Option<Object> {
        let target = match target {
            Expr::Identifier(Identifier(name)) => name,
            Expr::Call(fn_name_expr, args) => {
                return self.eval_module_field_acc_func(module, *fn_name_expr, args)
            }
            _ => {
                self.error_handler
                    .set_sematic("Can only access by identifier".to_string());
                return None;
            }
        };

        match module.acc_field(&target) {
            Some(field) => Some(field),
            None => {
                self.error_handler.set_name_error(format!(
                    "Module '{}' has no field '{}'",
                    module.name, target
                ));
                return None;
            }
        }
    }

    fn eval_module_field_acc_func(
        &mut self,
        module: Module,
        fn_name_expr: Expr,
        args: Vec<Expr>,
    ) -> Option<Object> {
        let fn_name = match fn_name_expr {
            Expr::Identifier(Identifier(name)) => name,
            _ => {
                self.error_handler
                    .set_sematic(format!("Missing method name"));
                return None;
            }
        };

        let fn_obj = match module.acc_field(&fn_name) {
            Some(object) => object,
            None => {
                self.error_handler.set_name_error(format!(
                    "No method '{}' associated with module '{}'",
                    fn_name, module.name
                ));
                return None;
            }
        };

        let evaluated_args = match self.eval_fn_call_args(args) {
            Some(evaluated_args) => evaluated_args,
            None => return None,
        };

        match fn_obj {
            Object::BuiltInFunction(method) => match method(evaluated_args) {
                object::BuiltInFuncReturnValue::Object(object) => Some(object),
                object::BuiltInFuncReturnValue::Error(err) => {
                    self.error_handler.set_error(err.kind, err.msg);
                    return None;
                }
            },
            _ => {
                self.error_handler
                    .set_type_error(format!("'{}' is not callable", fn_name));
                return None;
            }
        }
    }

    fn eval_fn_call_args(&mut self, args: Vec<Expr>) -> Option<Vec<ObjectInfo>> {
        let mut checked_args: Vec<ObjectInfo> = vec![];
        for arg in args {
            let arg = match self.eval_expr(arg) {
                Some(object) => ObjectInfo {
                    is_mut: true,
                    type_: object.ask_type(),
                    value: object,
                },
                None => return None,
            };
            checked_args.push(arg);
        }
        Some(checked_args)
    }

    fn eval_postfix_expr(&mut self, expr: Expr, postfix: Postfix) -> Option<Object> {
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

    fn eval_prefix_expr(&mut self, prefix: Prefix, expr: Expr) -> Option<Object> {
        let evaluated_expr = match self.eval_expr(expr) {
            Some(object) => object,
            None => return None,
        };

        match prefix {
            Prefix::Not => self.eval_not_prefix(evaluated_expr),
            Prefix::Plus => self.eval_plus_prefix(prefix, evaluated_expr),
            Prefix::Minus => self.eval_minus_prefix(prefix, evaluated_expr),
        }
    }

    fn eval_not_prefix(&mut self, evaluated_expr: Object) -> Option<Object> {
        match evaluated_expr {
            Object::Null => Some(Object::Boolean(true)),
            Object::Boolean(val) => Some(Object::Boolean(!val)),
            _ => Some(Object::Boolean(false)),
        }
    }

    fn eval_plus_prefix(&mut self, prefix: Prefix, evaluated_expr: Object) -> Option<Object> {
        match evaluated_expr {
            Object::Int(val) => Some(Object::Int(val)),
            Object::Float(val) => Some(Object::Float(val)),
            _ => {
                self.error_handler
                    .set_type_error(format!("'{}' prefix is for type number", prefix));
                return None;
            }
        }
    }

    fn eval_minus_prefix(&mut self, prefix: Prefix, evaluated_expr: Object) -> Option<Object> {
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

    fn eval_return(&mut self, expr: Option<Expr>) -> Option<Object> {
        if !self.env.borrow().in_context_type(ContextType::Function) {
            self.error_handler
                .set_sematic("'return' outside of function".to_string());
            return None;
        }

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
        let old_value = match self.env.borrow().resolve(&name) {
            Some(object) => object,
            None => {
                self.error_handler
                    .set_name_error(format!("'{}' is not declared", &name));
                return None;
            }
        };

        if !old_value.is_mut {
            self.error_handler
                .set_name_error(format!("'{}' is not assignable", name));
            return None;
        }

        let new_value = match self.eval_expr(expr) {
            Some(value) => value,
            None => return None,
        };

        if let Type::Array(Some(old_array_items_type)) = old_value.type_ {
            self.assign_array(name, *old_array_items_type, new_value);
            return None;
        }

        let new_value_type = new_value.ask_type();

        if old_value.type_ != new_value_type {
            self.error_handler.set_type_error(format!(
                "'{}' expects value of type '{}' but provided value of type '{}'",
                name, old_value.type_, new_value_type,
            ));
            return None;
        }

        self.env.borrow_mut().mutate(name, new_value);
        None
    }

    fn assign_array(
        &mut self,
        name: String,
        old_array_items_type: Type,
        new_array: Object,
    ) -> Option<Object> {
        let new_array_items_type = match new_array.ask_type() {
            Type::Array(opt_type) => opt_type,
            _ => {
                self.error_handler.set_type_error(format!(
                    "'{}' expects value of type Array<{}>",
                    name, old_array_items_type
                ));
                return None;
            }
        };

        if new_array_items_type.is_none() {
            self.env.borrow_mut().mutate(
                name,
                Object::Array {
                    inner: FilipeArray::new(vec![]),
                    items_type: Some(old_array_items_type),
                },
            );
            return None;
        }

        let new_array_items_type = *new_array_items_type.unwrap();

        if new_array_items_type != old_array_items_type {
            self.error_handler.set_type_error(format!(
                "'{}' expects array of type '{}' but provided array of type '{}'",
                name, old_array_items_type, new_array_items_type
            ));
            return None;
        }

        self.env.borrow_mut().mutate(name, new_array);
        None
    }

    fn eval_block_stmt(&mut self, block: &BlockStmt) -> Option<Object> {
        for stmt in block {
            if let Some(Object::RetVal(object)) = self.eval_stmt(stmt.clone()) {
                return Some(Object::RetVal(object));
            }

            if self.error_handler.has_error() {
                return None;
            }
        }
        Some(Object::Null)
    }

    fn eval_infix_expr(&mut self, lhs: Expr, infix: Infix, rhs: Expr) -> Option<Object> {
        let lhs = self.eval_expr(lhs);
        let rhs = self.eval_expr(rhs);

        if lhs.is_none() || rhs.is_none() {
            return None;
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        if lhs.ask_type() != rhs.ask_type() {
            self.error_handler.set_type_error(format!(
                "'{}' operation not allowed between types {} and {}",
                infix,
                lhs.ask_type(),
                rhs.ask_type(),
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
            Infix::Plus => Object::String(lhs.clone() + &rhs),
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

    fn eval_infix_int_expr(&mut self, lhs_val: i64, infix: Infix, rhs_val: i64) -> Object {
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

    fn eval_infix_float_expr(&mut self, lhs_val: f64, infix: Infix, rhs_val: f64) -> Object {
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

    fn eval_infix_bool_expr(&mut self, lhs_val: bool, infix: Infix, rhs_val: bool) -> Object {
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

    fn eval_literal_expr(&mut self, literal: Literal) -> Option<Object> {
        match literal {
            Literal::String(val) => Some(Object::String(val)),
            Literal::Boolean(val) => Some(Object::Boolean(val)),
            Literal::Null => Some(Object::Null),
            Literal::Int(val) => Some(Object::Int(val)),
            Literal::Float(val) => Some(Object::Float(val)),
            Literal::Array(val) => self.eval_array_literal(val),
        }
    }

    fn eval_array_literal(&mut self, array_literal: Vec<Expr>) -> Option<Object> {
        if array_literal.is_empty() {
            return Some(Object::Array {
                inner: FilipeArray::new(vec![]),
                items_type: None,
            });
        }

        let mut expr_array = array_literal.to_owned();
        let first_item = match self.eval_expr(expr_array.remove(0)) {
            Some(object) => object,
            None => return None,
        };

        let first_item_type = first_item.ask_type();

        let mut objects: Vec<Object> = vec![];
        objects.push(first_item);

        for expr in expr_array {
            let item = match self.eval_expr(expr) {
                Some(obj) => obj,
                None => return None,
            };

            if first_item_type != item.ask_type() {
                self.error_handler
                    .set_type_error("Array item's type mismatch".to_string());
                return None;
            }
            objects.push(item);
        }

        return Some(Object::Array {
            inner: FilipeArray::new(objects),
            items_type: Some(first_item_type),
        });
    }

    fn resolve_identfier(&mut self, identifier: Identifier) -> Option<Object> {
        let Identifier(name) = identifier;
        let meta_object = match self.env.borrow().resolve(&name) {
            Some(meta_object) => meta_object,
            None => {
                self.error_handler
                    .set_name_error(format!("'{}' is not declared", &name));
                return None;
            }
        };
        Some(meta_object.value)
    }
}
