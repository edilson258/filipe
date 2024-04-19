use super::super::{Identifier, Parser, Precedence, Stmt};
use crate::{ast::ExprType, token::Token};

pub fn parse_let_stmt(p: &mut Parser) -> Option<Stmt> {
    let ident_name = match p.next_token.clone() {
        Token::Identifier(val) => {
            p.bump();
            val
        }
        _ => {
            p.error_handler.set_identifier_error(&p.next_token);
            return None;
        }
    };

    if p.next_token_is(&Token::Colon) {
        p.bump();
        let next_token = p.next_token.clone();
        let ident_type = match token_to_type(p, &next_token) {
            Some(ident_type) => {
                p.bump();
                ident_type
            }
            None => return None,
        };

        if !p.next_token_is(&Token::Equal) {
            if p.next_token_is(&Token::Semicolon) {
                p.bump();
            }
            return Some(Stmt::Let(Identifier(ident_name), Some(ident_type), None));
        }

        if !p.bump_expected_next(&Token::Equal) {
            p.error_handler.set_not_type_annot_error(&next_token);
            return None;
        }

        p.bump();
        let expr = match p.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if p.next_token_is(&Token::Semicolon) {
            p.bump();
        }

        return Some(Stmt::Let(
            Identifier(ident_name),
            Some(ident_type),
            Some(expr),
        ));
    }

    if !p.bump_expected_next(&Token::Equal) {
        let next_token = p.next_token.clone();
        p.error_handler.set_not_type_annot_error(&next_token);
        return None;
    }

    p.bump();
    let expr = match p.parse_expr(Precedence::Lowest) {
        Some(expr) => expr,
        None => return None,
    };

    if p.next_token_is(&Token::Semicolon) {
        p.bump();
    }

    Some(Stmt::Let(Identifier(ident_name), None, Some(expr)))
}

fn token_to_type(p: &mut Parser, token: &Token) -> Option<ExprType> {
    match token {
        Token::TypeInt => Some(ExprType::Int),
        Token::TypeFloat => Some(ExprType::Float),
        Token::TypeString => Some(ExprType::String),
        Token::TypeBoolean => Some(ExprType::Boolean),
        _ => {
            p.error_handler.set_not_type_annot_error(token);
            return None;
        }
    }
}
