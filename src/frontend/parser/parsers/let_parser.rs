use super::super::{Identifier, Parser, Precedence, Stmt};
use super::super::super::{
    ast::{Expr, ExprType, LetStmtFlags, Literal},
    parser::ParserErrorKind,
    token::Token,
};

pub fn parse_let_stmt(p: &mut Parser) -> Option<Stmt> {
    p.bump();

    let var_name = match p.curr_token.clone() {
        Token::Identifier(val) => val,
        _ => {
            p.error_handler.set_identifier_error(&p.next_token);
            return None;
        }
    };
    p.bump();

    if p.current_token_is(&Token::Colon) {
        p.bump();
        let var_type = match p.parse_type() {
            Some(type_) => type_,
            None => return None,
        };
        p.bump();

        if var_type == ExprType::Void {
            p.error_handler.set_error(
                ParserErrorKind::SyntaxError,
                format!("Variable '{}' can't be of type 'void'", &var_name),
            );
            return None;
        }

        if var_type == ExprType::Array {
            if !p.bump_expected_current(&Token::LessThan) {
                return None;
            }
            let generic_type = match p.parse_type() {
                Some(type_) => type_,
                None => return None,
            };
            p.bump();
            if !p.bump_expected_current(&Token::GratherThan) {
                return None;
            }

            if !p.current_token_is(&Token::Equal) {
                return Some(Stmt::Let(
                    Identifier(var_name),
                    Some(generic_type),
                    Some(Expr::Literal(Literal::Array(vec![]))),
                    LetStmtFlags { is_array: true },
                ));
            }
            p.bump();

            let array_expr = match p.parse_array_expr() {
                Some(items) => items,
                None => return None,
            };

            return Some(Stmt::Let(
                Identifier(var_name),
                Some(generic_type),
                Some(array_expr),
                LetStmtFlags { is_array: true },
            ));
        }

        if !p.current_token_is(&Token::Equal) {
            return Some(Stmt::Let(
                Identifier(var_name),
                Some(var_type),
                None,
                LetStmtFlags { is_array: false },
            ));
        }

        p.bump();
        let expr = match p.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        return Some(Stmt::Let(
            Identifier(var_name),
            Some(var_type),
            Some(expr),
            LetStmtFlags { is_array: false },
        ));
    }

    if !p.current_token_is(&Token::Equal) {
        p.error_handler.set_error(
            ParserErrorKind::TypeError,
            format!(
                "Missing type of '{}', provide it's type or initialize it",
                &var_name
            ),
        );
        return None;
    }

    p.bump();
    let expr = match p.parse_expr(Precedence::Lowest) {
        Some(expr) => expr,
        None => return None,
    };

    if let Expr::Literal(Literal::Array(expr_list)) = &expr {
        if expr_list.len() < 1 {
            p.error_handler.set_error(
                ParserErrorKind::TypeError,
                format!("Unable to infer type of array '{}'", &var_name),
            );
            return None;
        }

        return Some(Stmt::Let(
            Identifier(var_name),
            None,
            Some(expr),
            LetStmtFlags { is_array: true },
        ));
    }

    Some(Stmt::Let(
        Identifier(var_name),
        None,
        Some(expr),
        LetStmtFlags { is_array: false },
    ))
}
