mod func_parser;
mod let_parser;
mod parser_error_handler;

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;
use func_parser::parse_func_stmt;
use let_parser::parse_let_stmt;
use parser_error_handler::*;

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    error_handler: ParserErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::Eof,
            next_token: Token::Eof,
            error_handler: ParserErrorHandler::new(),
        };

        p.bump();
        p.bump();

        p
    }

    fn bump(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.l.next_token();
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];
        while !self.current_token_is(&Token::Eof) && !self.error_handler.has_error() {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.bump();
        }
        program
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.curr_token {
            Token::Let => parse_let_stmt(self),
            Token::Func => parse_func_stmt(self),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn token_to_type(&mut self, token: &Token) -> Option<ExprType> {
        match token {
            Token::StringType => Some(ExprType::String),
            Token::NumberType => Some(ExprType::Number),
            Token::BooleanType => Some(ExprType::Boolean),
            _ => {
                self.error_handler.set_not_type_annot_error(token);
                return None;
            }
        }
    }

    fn parse_type_annot(&mut self) -> Option<ExprType> {
        match self.curr_token {
            Token::NumberType => Some(ExprType::Number),
            Token::StringType => Some(ExprType::String),
            Token::BooleanType => Some(ExprType::Boolean),
            Token::Null => Some(ExprType::Null),
            _ => {
                self.error_handler.set_not_type_annot_error(&self.curr_token);
                return None;
            }
        }
    }

    fn parse_block_stmt(&mut self) -> Option<Vec<Stmt>> {
        self.bump();
        let mut block: Vec<Stmt> = vec![];
        while !self.current_token_is(&Token::Rbrace) && !self.current_token_is(&Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                _ => return None,
            }
            self.bump();
        }
        if !self.current_token_is(&Token::Rbrace) {
            self.error_handler
                .set_expected_but_provided_error(&Token::Rbrace, &self.curr_token);
            return None;
        }
        Some(block)
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.bump();
        if self.current_token_is(&Token::Semicolon) {
            return Some(Stmt::Return(None));
        }
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }
        Some(Stmt::Return(Some(expr)))
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        match &self.curr_token {
            Token::Identifier(name) => Some(Identifier(name.clone())),
            _ => None,
        }
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
                    self.bump();
                }
                Some(Stmt::Expr(expr))
            }
            None => None,
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = match self.curr_token {
            Token::Identifier(_) => self.parse_identifier_expr(),
            Token::String(_) => self.parse_string_expr(),
            Token::Number(_) => self.parse_number_expr(),
            Token::True => Some(Expr::Literal(Literal::Boolean(true))),
            Token::False => Some(Expr::Literal(Literal::Boolean(false))),
            Token::Null => Some(Expr::Literal(Literal::Null)),
            _ => {
                let token = self.curr_token.clone();
                self.error_handler.set_unexpexted_token_error(&token);
                return None;
            }
        };

        /* TODO: ensure that the next expr is sematicly expected
         * based on the current expr which is `left`, to avoid this:
         *      print hello -> follwed identifiers
         *      "foo" "bar" OR 10 10 -> follwed literals
         */

        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
            if left.is_none() {
                return None;
            }
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::DoubleEqual
                | Token::GratherThan
                | Token::LessThan
                | Token::GratherOrEqual
                | Token::LessOrEqual => {
                    self.bump();
                    left = self.parse_infix_expr(left.unwrap());
                }
                Token::Lparen => {
                    self.bump();
                    left = self.parse_call_expr(left.unwrap());
                }
                Token::Equal => {
                    self.bump();
                    left = self.parse_assign_expr(left.unwrap());
                }
                _ => return left,
            }
        }
        left
    }

    fn parse_assign_expr(&mut self, left: Expr) -> Option<Expr> {
        let identifier = match left {
            Expr::Identifier(identifier) => identifier,
            _ => {
                self.error_handler
                    .set_invalid_left_side_of_assignment_error(left);
                return None;
            }
        };
        self.bump();
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
        Some(Expr::Assign(identifier, Box::new(expr)))
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.curr_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Multiply,
            Token::Slash => Infix::Devide,
            Token::DoubleEqual => Infix::Equal,
            Token::LessThan => Infix::LessThan,
            Token::LessOrEqual => Infix::LessOrEqual,
            Token::GratherThan => Infix::GratherThan,
            Token::GratherOrEqual => Infix::GratherOrEqual,
            _ => return None,
        };
        let precedence = self.current_token_precedence();
        self.bump();
        match self.parse_expr(precedence) {
            Some(expr) => Some(Expr::Infix(Box::new(left), infix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::Rparen) {
            Some(exprs) => exprs,
            None => return None,
        };
        return Some(Expr::Call(Box::new(func), args));
    }

    fn parse_expr_list(&mut self, stop: Token) -> Option<Vec<Expr>> {
        let mut list: Vec<Expr> = vec![];
        if self.next_token_is(&stop) {
            self.bump();
            return Some(list);
        }
        self.bump();
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }
        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();
            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }
        if !self.bump_expected_next(&Token::Rparen) {
            return None;
        }
        Some(list)
    }

    fn parse_identifier_expr(&mut self) -> Option<Expr> {
        match self.parse_identifier() {
            Some(identifier) => Some(Expr::Identifier(identifier)),
            None => None,
        }
    }

    fn parse_string_expr(&self) -> Option<Expr> {
        match self.curr_token.clone() {
            Token::String(val) => Some(Expr::Literal(Literal::String(val))),
            _ => None,
        }
    }

    fn parse_number_expr(&self) -> Option<Expr> {
        match self.curr_token.clone() {
            Token::Number(val) => Some(Expr::Literal(Literal::Number(val))),
            _ => None,
        }
    }

    fn token_to_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::Lparen => Precedence::Call,
            Token::Equal => Precedence::Assign,
            Token::DoubleEqual
            | Token::LessThan
            | Token::LessOrEqual
            | Token::GratherThan
            | Token::GratherOrEqual => Precedence::Comparison,
            _ => Precedence::Lowest,
        }
    }

    fn current_token_precedence(&self) -> Precedence {
        Self::token_to_precedence(&self.curr_token)
    }

    fn next_token_precedence(&self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    fn bump_expected_next(&mut self, token: &Token) -> bool {
        if self.next_token_is(token) {
            self.bump();
            return true;
        }
        self.error_handler
            .set_expected_but_provided_error(token, &self.next_token);
        false
    }

    fn current_token_is(&self, token: &Token) -> bool {
        self.curr_token == *token
    }

    fn next_token_is(&self, token: &Token) -> bool {
        self.next_token == *token
    }

    pub fn get_error(&self) -> Option<ParserError> {
        self.error_handler.get_error()
    }

    pub fn has_error(&self) -> bool {
        self.error_handler.has_error()
    }
}
