use core::fmt;

type BuiltInFunc = fn(Vec<Object>) -> Object;

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Builtin(usize, BuiltInFunc),
    Null,
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "\"{}\"", val),
            Self::Number(val) => write!(f, "{}", val),
            Self::Builtin(_, _) => write!(f, "[Builtin Function]"),
            Self::Null => write!(f, "null"),
            Self::Error(msg) => write!(f, "{}", msg),
        }
    }
}
