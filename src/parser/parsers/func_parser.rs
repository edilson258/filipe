use super::super::{ExprType, Identifier, Parser, Stmt};
use crate::{parser::ParserErrorKind, token::Token};

pub fn parse_func_stmt(p: &mut Parser) -> Option<Stmt> {
    let fn_identifier = match p.next_token.clone() {
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

    let return_type = match p.parse_type() {
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
    Some(Stmt::Func(fn_identifier, fn_params, body, return_type))
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
    let param_type = match p.parse_type() {
        Some(type_) => type_,
        None => return None,
    };

    if param_type == ExprType::Void {
        p.error_handler.set_error(
            ParserErrorKind::SyntaxError,
            format!("Function parameter can't not be of type 'void'"),
        );
        return None;
    }

    params.push((identifier, param_type));

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
        let param_type = match p.parse_type() {
            Some(type_) => type_,
            None => return None,
        };
        if param_type == ExprType::Void {
            p.error_handler.set_error(
                ParserErrorKind::SyntaxError,
                format!("Function parameter can't not be of type 'void'"),
            );
            return None;
        }
        params.push((identifier, param_type));
    }
    if !p.bump_expected_next(&Token::Rparen) {
        return None;
    }
    Some(params)
}
