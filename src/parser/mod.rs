mod error_handler;
mod parsers;

use self::parsers::if_parser::parse_if_stmt;
use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;
use error_handler::*;
use parsers::forloop_parser::parse_forloop_stmt;
use parsers::func_parser::parse_func_stmt;
use parsers::let_parser::parse_let_stmt;

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
        let next_token = self.l.next_token();

        if next_token.is_err() {
            self.error_handler
                .set_error(ParserErrorKind::SyntaxError, next_token.err().unwrap());
            return;
        }

        self.curr_token = self.next_token.clone();
        self.next_token = next_token.unwrap();
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
        if self.has_error() {
            return None;
        }

        match self.curr_token {
            Token::Let => parse_let_stmt(self),
            Token::Func => parse_func_stmt(self),
            Token::Return => self.parse_return_stmt(),
            Token::If => parse_if_stmt(self),
            Token::For => parse_forloop_stmt(self),
            _ => self.parse_expr_stmt(),
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
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
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
            Some(expr) => Some(Stmt::Expr(expr)),
            None => None,
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        if self.has_error() {
            return None;
        }

        let mut left = match self.curr_token {
            Token::Identifier(_) => self.parse_identifier_expr(),
            Token::String(_) => self.parse_string_expr(),
            Token::Int(_) => self.parse_int_expr(),
            Token::Float(_) => self.parse_float_expr(),
            Token::True => Some(Expr::Literal(Literal::Boolean(true))),
            Token::False => Some(Expr::Literal(Literal::Boolean(false))),
            Token::Null => Some(Expr::Literal(Literal::Null)),
            Token::Bang | Token::Plus | Token::Minus => self.parse_prefix_expr(),
            _ => {
                let token = self.curr_token.clone();
                self.error_handler.set_unexpexted_token_error(&token);
                return None;
            }
        };

        while precedence < self.next_token_precedence() {
            if left.is_none() {
                return None;
            }
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::DoubleEqual
                | Token::NotEqual
                | Token::GratherThan
                | Token::LessThan
                | Token::Percet
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
                Token::DoublePlus | Token::DoubleMinus => {
                    self.bump();
                    left = self.parse_postfix_expr(left.unwrap());
                }
                _ => return left,
            }
        }
        left
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.curr_token {
            Token::Int(val) => Some(Expr::Literal(Literal::Int(val))),
            _ => return None,
        }
    }

    fn parse_float_expr(&mut self) -> Option<Expr> {
        match self.curr_token {
            Token::Float(val) => Some(Expr::Literal(Literal::Float(val))),
            _ => return None,
        }
    }

    fn parse_postfix_expr(&mut self, left: Expr) -> Option<Expr> {
        let postfix = match self.curr_token {
            Token::DoublePlus => Postfix::Increment,
            Token::DoubleMinus => Postfix::Decrement,
            _ => return None,
        };

        Some(Expr::Postfix(Box::new(left), postfix))
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.curr_token {
            Token::Bang => Prefix::Not,
            Token::Plus => Prefix::Plus,
            Token::Minus => Prefix::Minus,
            _ => return None,
        };

        self.bump();

        let expr = match self.parse_expr(Precedence::Prefix) {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expr::Prefix(prefix, Box::new(expr)))
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
            Token::Percet => Infix::Remainder,
            Token::NotEqual => Infix::NotEqual,
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

    fn parse_type(&mut self) -> Option<ExprType> {
        match self.curr_token {
            Token::TypeInt => Some(ExprType::Int),
            Token::TypeVoid => Some(ExprType::Void),
            Token::TypeFloat => Some(ExprType::Float),
            Token::TypeString => Some(ExprType::String),
            Token::TypeBoolean => Some(ExprType::Boolean),
            _ => {
                self.error_handler.set_not_type_annot_error(&self.curr_token);
                return None;
            }
        }
    }

    fn token_to_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash | Token::Percet => Precedence::Product,
            Token::Lparen => Precedence::Call,
            Token::Equal => Precedence::Assign,
            Token::DoubleEqual
            | Token::NotEqual
            | Token::LessThan
            | Token::LessOrEqual
            | Token::GratherThan
            | Token::GratherOrEqual => Precedence::Comparison,
            Token::DoublePlus | Token::DoubleMinus => Precedence::Postfix,
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
