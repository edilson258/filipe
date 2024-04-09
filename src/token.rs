#[derive(Debug, Clone)]
pub enum Token {
    Illegal(char),
    Eof,

    Lparen,
    Rparen,
    
    Plus,
    Minus,
    Mult,
    Div,

    String(String),
    Integer(i64),

    Identifier(String),
}
