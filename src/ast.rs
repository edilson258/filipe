use core::fmt;

pub type Program = Vec<Stmt>;
pub type BlockStmt = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum ExprType {
    Null,
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Call(Box<Expr>, Vec<Expr>),
    Identifier(Identifier),
    Infix(Box<Expr>, Infix, Box<Expr>),
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
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Assign,     // foo = "bar"
    Comparison, // x > 6
    Sum,        // +
    Product,    // *
    Call,       // myFunction(x)
}

#[derive(Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Devide,
    Multiply,
    Equal,
    LessThan,
    LessOrEqual,
    GratherThan,
    GratherOrEqual,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Devide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "=="),
            Infix::LessThan => write!(f, "<"),
            Infix::LessOrEqual => write!(f, "<="),
            Infix::GratherThan => write!(f, ">"),
            Infix::GratherOrEqual => write!(f, ">="),
        }
    }
}
