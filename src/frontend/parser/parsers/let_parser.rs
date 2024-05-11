use super::super::super::{
    ast::{Expr, ExprType, Literal},
    parser::ParserErrorKind,
    token::Token,
};
use super::super::{Identifier, Parser, Precedence, Stmt};

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

        if !p.current_token_is(&Token::Equal) {
            return Some(Stmt::Let(Identifier(var_name), Some(var_type), None));
        }

        p.bump();
        let expr = match p.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        return Some(Stmt::Let(Identifier(var_name), Some(var_type), Some(expr)));
    }

    if !p.current_token_is(&Token::Equal) {
        return Some(Stmt::Let(Identifier(var_name), None, None));
    }

    p.bump();
    let expr = match p.parse_expr(Precedence::Lowest) {
        Some(expr) => expr,
        None => return None,
    };

    if let Expr::Literal(Literal::Array(_)) = &expr {
        return Some(Stmt::Let(Identifier(var_name), None, Some(expr)));
    }

    Some(Stmt::Let(Identifier(var_name), None, Some(expr)))
}
