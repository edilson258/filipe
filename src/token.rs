use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    Lparen,
    Rparen,
    Comma,
    Semicolon,
    
    Plus,
    Minus,
    Asterisk,
    Slash,

    String(String),
    Integer(i64),

    Identifier(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Illegal(val) => write!(f, "{}", val),
            Self::Eof => write!(f, "EOF"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::String(val) => write!(f, "{}", val),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Identifier(name) => write!(f, "{}", name),
        }
    }
}
