use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Colon,
    Semicolon,
    
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,

    DoubleEqual,
    GratherThan,
    GratherOrEqual,
    LessThan,
    LessOrEqual,
    Bang,
    NotEqual,
    DoublePlus,
    DoubleMinus,

    String(String),
    Number(f64),
    True,
    False,

    Identifier(String),
    Let,
    Func,
    Return,
    Null,
    If,
    Else,

    StringType,
    BooleanType,
    NumberType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
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
            Self::Number(val) => write!(f, "{}", val),
            Self::Identifier(name) => write!(f, "{}", name),
            Self::Let => write!(f, "let"),
            Self::Equal => write!(f, "="),
            Self::Func => write!(f, "[Defined Function]"),
            Self::Lbrace => write!(f, "{{"),
            Self::Rbrace => write!(f, "}}"),
            Self::Return => write!(f, "return"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::DoubleEqual => write!(f, "=="),
            Self::GratherThan => write!(f, ">"),
            Self::GratherOrEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessOrEqual => write!(f, "<="),
            Self::Null => write!(f, "null"),
            Self::Colon => write!(f, ":"),
            Self::StringType => write!(f, "[Type Annotation] string"),
            Self::NumberType => write!(f, "[Type Annotation] number"),
            Self::BooleanType => write!(f, "[Type Annotation] boolean"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Bang => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::DoublePlus => write!(f, "++"),
            Self::DoubleMinus => write!(f, "--"),
        }
    }
}
