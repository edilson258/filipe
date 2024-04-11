use core::fmt;

use crate::ast::{Expr, Identifier, Infix, Literal, Precedence, Program, Stmt};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Clone)]
pub enum ParseErrorKind {
    Unexpected,
    SytaxError,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseErrorKind::Unexpected => write!(f, "[Unexpected Token]"),
            ParseErrorKind::SytaxError => write!(f, "[Sytax Error]"),
        }
    }
}

#[derive(Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type ParseErrors = Vec<ParseError>;

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::Eof,
            next_token: Token::Eof,
            errors: vec![],
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

        while !self.current_token_is(&Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {},
            }
            self.bump();
        }

        program
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.curr_token {
            Token::Let => self.parse_let_stmt(),
            Token::Func => self.parse_func_stmt(),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::Identifier(_) => self.bump(),
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::SytaxError,
                    msg: format!("'let' statment must be follwed by identifier"),
                });
                return None;
            }
        }

        let name = match self.parse_identifier() {
            Some(name) => name,
            None => {
                return None;
            }
        };

        if self.next_token_is(&Token::Semicolon) || self.next_token_is(&Token::Eof) {
            self.bump();
            return Some(Stmt::Let(name, None));
        }

        if !self.expect_next_token(&Token::Equal) {
            return None;
        }
        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Let(name, Some(expr)))
    }

    fn parse_func_stmt(&mut self) -> Option<Stmt> {
        match self.next_token {
            Token::Identifier(_) => self.bump(),
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::SytaxError,
                    msg: format!("function's name not provided"),
                });
                return None;
            }
        }

        let fn_ident = match self.parse_identifier() {
            Some(ident) => ident,
            None => return None,
        };

        if !self.expect_next_token(&Token::Lparen) {
            return None;
        }

        let fn_params = match self.parse_func_params() {
            Some(params) => params,
            None => return None,
        };

        if !self.expect_next_token(&Token::Lbrace) {
            return None;
        }

        let body = match self.parse_block_stmt() {
            Some(block) => block,
            None => return None
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Func(fn_ident, fn_params, body))
    }

    fn parse_func_params(&mut self) -> Option<Vec<Identifier>> {
        let mut params: Vec<Identifier> = vec![];

        if self.next_token_is(&Token::Rparen) {
            self.bump();
            return Some(params);
        }

        self.bump();
        match self.parse_identifier() {
            Some(ident) => params.push(ident),
            _ => return None
        }

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_identifier() {
                Some(ident) => params.push(ident),
                _ => return None
            };
        }

        if !self.expect_next_token(&Token::Rparen) {
            return None;
        }

        Some(params)
    }

    fn parse_block_stmt(&mut self) -> Option<Vec<Stmt>> {
        self.bump();
        let mut body: Vec<Stmt> = vec![];
        while !self.current_token_is(&Token::Rbrace) && !self.current_token_is(&Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => body.push(stmt),
                _ => return None
            }
            self.bump();
        }
        Some(body)
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
            _ => {
                let token = self.curr_token.clone();
                self.unexpected_token_error(&token);
                return None;
            }
        };

        /* TODO: ensure that the next expr is sematicly expected
         * based on the current expr which is `left`, to avoid this:
         *      print hello -> follwed identifiers
         *      "foo" "bar" OR 10 10 -> follwed literals
         */

        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.next_token {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
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
                self.errors.push(ParseError {
                    kind: ParseErrorKind::Unexpected,
                    msg: "Left side of assignment must an identifier".to_string(),
                });
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

        return Some(Expr::Call {
            func: Box::new(func),
            args,
        });
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

        if !self.expect_next_token(&Token::Rparen) {
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
            _ => Precedence::Lowest,
        }
    }

    fn current_token_precedence(&self) -> Precedence {
        Self::token_to_precedence(&self.curr_token)
    }

    fn next_token_precedence(&self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    fn expect_next_token(&mut self, token: &Token) -> bool {
        if self.next_token_is(token) {
            self.bump();
            return true;
        }
        self.next_token_error(&token);
        false
    }

    fn current_token_is(&self, token: &Token) -> bool {
        self.curr_token == *token
    }

    fn next_token_is(&self, token: &Token) -> bool {
        if self.next_token == *token {
            return true;
        }
        false
    }

    fn unexpected_token_error(&mut self, token: &Token) {
        self.errors.push(ParseError {
            kind: ParseErrorKind::Unexpected,
            msg: format!("{}", token),
        })
    }

    fn next_token_error(&mut self, token: &Token) {
        self.errors.push(ParseError {
            kind: ParseErrorKind::SytaxError,
            msg: format!("expected {} but found {}", token, self.next_token),
        })
    }

    pub fn get_errors(&self) -> ParseErrors {
        self.errors.clone()
    }
}
