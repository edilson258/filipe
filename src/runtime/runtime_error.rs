use core::fmt;

#[derive(Clone)]
pub enum ErrorKind {
    NameError,
    TypeError,
    ArgumentError,
    ValueError,
}

#[derive(Clone)]
pub struct RuntimeError {
    pub kind: ErrorKind,
    pub msg: String,
}

#[derive(Clone)]
pub struct RuntimeErrorHandler {
    error: Option<RuntimeError>,
}

impl RuntimeErrorHandler {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn has_error(&mut self) -> bool {
        self.error.is_some()
    }

    pub fn get_error(&mut self) -> Option<RuntimeError> {
        self.error.clone()
    }

    pub fn set_name_error(&mut self, msg: String) {
        self.set_error(ErrorKind::NameError, msg)
    }

    pub fn set_type_error(&mut self, msg: String) {
        self.set_error(ErrorKind::TypeError, msg)

    }

    pub fn set_error(&mut self, kind: ErrorKind, msg: String) {
        self.error = Some(RuntimeError {
            kind,
            msg,
        });
    }

}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.msg)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameError => write!(f, "[Name error]"),
            Self::TypeError => write!(f, "[Type Error]"),
            Self::ArgumentError => write!(f, "[Argument Error]"),
            Self::ValueError => write!(f, "[Value Error]"),
        }
    }
}
