use core::fmt;

#[derive(Clone)]
pub enum RuntimeErrorKind {
    NameError,
    InvalidOp,
    TypeError,
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameError => write!(f, "[Name error]"),
            Self::InvalidOp => write!(f, "[Invalid Operation]"),
            Self::TypeError => write!(f, "[Type Error]"),
        }
    }
}

#[derive(Clone)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub msg: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.msg)
    }
}
