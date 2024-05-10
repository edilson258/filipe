use crate::frontend::{
    ast::{Identifier, Precedence, Stmt},
    parser::Parser,
    token::Token,
};

pub fn parse_forloop_stmt(p: &mut Parser) -> Option<Stmt> {
    p.bump();
    let loop_cursor_name = match p.parse_identifier() {
        Some(identifier) => {
            let Identifier(name) = identifier;
            name
        }
        None => {
            p.error_handler.set_identifier_error(&p.curr_token);
            return None;
        }
    };

    if !p.bump_expected_next(&Token::In) {
        return None;
    }

    p.bump();
    let iterable = match p.parse_expr(Precedence::Lowest) {
        Some(expr) => expr,
        None => return None,
    };
    p.bump();

    let block = match p.parse_block_stmt() {
        Some(block) => block,
        None => return None,
    };

    Some(Stmt::ForLoop {
        cursor: loop_cursor_name,
        iterable,
        block,
    })
}
