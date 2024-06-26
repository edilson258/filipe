use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    Lbracket,
    Rbracket,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Colon,
    Dot,
    
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Percet,

    DoubleEqual,
    GratherThan,
    GratherOrEqual,
    LessThan,
    LessOrEqual,
    Bang,
    NotEqual,
    DoublePlus,
    DoubleMinus,

    Int(i64),
    Float(f64),
    String(String),
    True,
    False,

    Let,
    If,
    For,
    In,
    Else,
    Func,
    Null,
    Import,
    Return,
    ClassArray,
    Identifier(String),

    TypeInt,
    TypeFloat,
    TypeString,
    TypeBoolean,
    TypeVoid,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
            Self::Eof => write!(f, "EOF"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Lbracket => write!(f, "["),
            Self::Rbracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::String(val) => write!(f, "{}", val),
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
            Self::TypeString => write!(f, "[Type Annotation] string"),
            Self::TypeBoolean => write!(f, "[Type Annotation] boolean"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Bang => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::DoublePlus => write!(f, "++"),
            Self::DoubleMinus => write!(f, "--"),
            Self::Percet => write!(f, "%"),
            Self::For => write!(f, "for"),
            Self::In => write!(f, "in"),
            Self::TypeInt => write!(f, "[Type Annotation] int"),
            Self::TypeFloat => write!(f, "[Type Annotation] float"),
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::TypeVoid => write!(f, "[Type Annotation] void"),
            Self::ClassArray => write!(f, "[Built-in Class] Array"),
            Self::Dot => write!(f, "."),
            Self::Import => write!(f, "import"),
        }
    }
}
