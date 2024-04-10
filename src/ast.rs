use core::fmt;

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Call { func: Box<Expr>, args: Vec<Expr> },
    Identifier(Identifier),
    Infix(Box<Expr>, Infix, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Sum,         // +
    Product,     // *
    Call,        // myFunction(x)
}

#[derive(Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Devide,
    Multiply,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Devide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
        }
    }
}
