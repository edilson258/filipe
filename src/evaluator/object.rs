use core::fmt;

pub enum Object {
    Number(f64),
    String(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "{}", val),
            Self::Number(val) => write!(f, "{}", val),
        }
    }
}
