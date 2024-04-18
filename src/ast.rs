use core::fmt;

pub type Program = Vec<Stmt>;
pub type BlockStmt = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum ExprType {
    Null,
    Int,
    Float,
    String,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Call(Box<Expr>, Vec<Expr>),
    Identifier(Identifier),
    Infix(Box<Expr>, Infix, Box<Expr>),
    Prefix(Prefix, Box<Expr>),
    Postfix(Box<Expr>, Postfix),
    Assign(Identifier, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Identifier, Option<ExprType>, Option<Expr>),
    Func(Identifier, Vec<(Identifier, ExprType)>, BlockStmt, ExprType),
    Return(Option<Expr>),
    If {
        condition: Expr,
        consequence: BlockStmt,
        alternative: Option<BlockStmt>,
    },
    ForLoop {
        cursor: String,
        iterable: Expr,
        block: BlockStmt,
    },
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Assign,     // foo = "bar"
    Comparison, // x > 6
    Sum,        // +
    Product,    // *
    Prefix,     // !true || -5
    Postfix,    // 69++ || 10--
    Call,       // myFunction(x)
}

#[derive(Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Devide,
    Multiply,
    Equal,
    NotEqual,
    LessThan,
    Remainder,
    LessOrEqual,
    GratherThan,
    GratherOrEqual,
}

#[derive(Debug, Clone)]
pub enum Prefix {
    Not,
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Postfix {
    Increment,
    Decrement,
}

impl fmt::Display for Postfix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Increment => write!(f, "++"),
            Self::Decrement => write!(f, "--"),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Prefix::Not => write!(f, "!"),
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
        }
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Devide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "=="),
            Infix::NotEqual => write!(f, "!="),
            Infix::LessThan => write!(f, "<"),
            Infix::LessOrEqual => write!(f, "<="),
            Infix::GratherThan => write!(f, ">"),
            Infix::GratherOrEqual => write!(f, ">="),
            Infix::Remainder => write!(f, "%"),
        }
    }
}
