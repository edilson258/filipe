use crate::{
    ast::{BlockStmt, Precedence, Stmt},
    parser::Parser,
    token::Token,
};

pub fn parse_if_stmt(p: &mut Parser) -> Option<Stmt> {
    if !p.bump_expected_next(&Token::Lparen) {
        return None;
    }

    p.bump();

    let condition = match p.parse_expr(Precedence::Lowest) {
        Some(expr) => expr,
        None => return None,
    };

    if !p.bump_expected_next(&Token::Rparen) {
        return None;
    }
    p.bump();

    let consequence = match p.parse_block_stmt() {
        Some(block) => block,
        None => return None,
    };

    let alternative: Option<BlockStmt> = match p.next_token_is(&Token::Else) {
        true => {
            p.bump();
            p.bump();
            p.parse_block_stmt()
        }
        false => None,
    };

    if p.next_token_is(&Token::Semicolon) {
        p.bump();
    }

    Some(Stmt::If {
        condition,
        consequence,
        alternative,
    })
}
