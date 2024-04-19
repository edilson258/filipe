use super::super::{ExprType, Identifier, Parser, Stmt};
use crate::token::Token;

pub fn parse_func_stmt(p: &mut Parser) -> Option<Stmt> {
    let fn_ident = match p.next_token.clone() {
        Token::Identifier(val) => {
            p.bump();
            Identifier(val)
        }
        _ => {
            p.error_handler.set_identifier_error(&p.next_token);
            return None;
        }
    };
    if !p.bump_expected_next(&Token::Lparen) {
        return None;
    }
    let fn_params = match parse_func_params(p) {
        Some(params) => params,
        None => return None,
    };

    if !p.bump_expected_next(&Token::Colon) {
        return None;
    }

    p.bump();

    let ret_type = match parse_type_annot(p) {
        Some(ret_type) => ret_type,
        None => return None,
    };

    if !p.bump_expected_next(&Token::Lbrace) {
        return None;
    }
    let body = match p.parse_block_stmt() {
        Some(block) => block,
        None => return None,
    };
    if p.next_token_is(&Token::Semicolon) {
        p.bump();
    }
    Some(Stmt::Func(fn_ident, fn_params, body, ret_type))
}

fn parse_func_params(p: &mut Parser) -> Option<Vec<(Identifier, ExprType)>> {
    let mut params: Vec<(Identifier, ExprType)> = vec![];
    if p.next_token_is(&Token::Rparen) {
        p.bump();
        return Some(params);
    }
    p.bump();
    let identifier = match p.parse_identifier() {
        Some(identifier) => identifier,
        _ => {
            p.error_handler.set_identifier_error(&p.curr_token);
            return None;
        }
    };
    p.bump();
    p.bump();
    let type_ = match parse_type_annot(p) {
        Some(type_) => type_,
        None => return None,
    };

    params.push((identifier, type_));

    while p.next_token_is(&Token::Comma) {
        p.bump();
        p.bump();

        let identifier = match p.parse_identifier() {
            Some(identifier) => identifier,
            _ => {
                p.error_handler.set_identifier_error(&p.curr_token);
                return None;
            }
        };
        p.bump();
        p.bump();
        let type_ = match parse_type_annot(p) {
            Some(type_) => type_,
            None => return None,
        };
        params.push((identifier, type_));
    }
    if !p.bump_expected_next(&Token::Rparen) {
        return None;
    }
    Some(params)
}

fn parse_type_annot(p: &mut Parser) -> Option<ExprType> {
    match p.curr_token {
        Token::Null => Some(ExprType::Null),
        Token::TypeInt => Some(ExprType::Int),
        Token::TypeFloat => Some(ExprType::Float),
        Token::TypeString => Some(ExprType::String),
        Token::TypeBoolean => Some(ExprType::Boolean),
        _ => {
            p.error_handler.set_not_type_annot_error(&p.curr_token);
            return None;
        }
    }
}
