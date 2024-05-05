use core::fmt;

use crate::{
    ast::{Expr, Literal},
    token::Token,
};

#[derive(Clone)]
pub enum ParserErrorKind {
    SyntaxError,
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParserErrorKind::SyntaxError => write!(f, "[Syntax Error]"),
        }
    }
}

#[derive(Clone)]
pub struct ParserError {
    kind: ParserErrorKind,
    msg: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub struct ParserErrorHandler {
    error: Option<ParserError>,
}

impl ParserErrorHandler {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn has_error(&self) -> bool {
        if self.error.is_some() {
            return true;
        }
        false
    }

    pub fn set_error(&mut self, kind: ParserErrorKind, msg: String) {
        self.error = Some(ParserError { kind, msg });
    }

    pub fn get_error(&self) -> Option<ParserError> {
        if self.error.is_none() {
            return None;
        }
        Some(self.error.clone().unwrap())
    }

    pub fn set_invalid_left_side_of_assignment_error(&mut self, left: Expr) {
        match left {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Boolean(val) => {
                        self.error = Some(ParserError {
                            kind: ParserErrorKind::SyntaxError,
                            msg: format!("'{}' is not assignable", val),
                        });
                        return;
                    }
                    _ => {
                        self.error = Some(ParserError {
                            kind: ParserErrorKind::SyntaxError,
                            msg: format!("cannot assign to literal here. Maybe you meant '==' instead of '='?"),
                        });
                        return;
                    }
                };
            }
            _ => {}
        }
        self.error = Some(ParserError {
            kind: ParserErrorKind::SyntaxError,
            msg: format!("Left side of assignment must an identifier"),
        });
    }

    pub fn set_identifier_error(&mut self, token: &Token) {
        match token {
            Token::Func => {
                self.error = Some(ParserError {
                    kind: ParserErrorKind::SyntaxError,
                    msg: format!("'define' keyword cannot be used as identifier"),
                });
                return;
            }
            _ => {}
        }
        self.error = Some(ParserError {
            kind: ParserErrorKind::SyntaxError,
            msg: format!("'{token}' cannot be used as identifier"),
        });
    }

    pub fn set_expected_but_provided_error(&mut self, expected: &Token, provided: &Token) {
        self.error = Some(ParserError {
            kind: ParserErrorKind::SyntaxError,
            msg: format!("expected '{}' but provided '{}'", expected, provided),
        });
    }

    pub fn set_unexpexted_token_error(&mut self, token: &Token) {
        self.error = Some(ParserError {
            kind: ParserErrorKind::SyntaxError,
            msg: format!("unexpected {}", token),
        });
    }

    pub fn set_not_type_annot_error(&mut self, token: &Token) {
        self.error = Some(ParserError {
            kind: ParserErrorKind::SyntaxError,
            msg: format!("expected type annotation but provided {}", token),
        });
    }
}
