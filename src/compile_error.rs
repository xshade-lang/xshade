use std::error::Error;
use std::fmt;
use ::type_system::error::TypeError;
use ::ast::Span;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Unknown,
    ParseError,
    TypeError(TypeError),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError {
    kind: ErrorKind,
    span: Span,
}

impl CompileError {
    pub fn new(kind: ErrorKind, span: Span) -> CompileError {
        CompileError {
            kind: kind,
            span: span,
        }
    }

    pub fn unknown() -> CompileError {
        CompileError {
            kind: ErrorKind::Unknown,
            span: Span::new(0, 0, 1, 1),
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compile Error")
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        "Compile Error"
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ErrorKind::Unknown => None,
            ErrorKind::TypeError(ref t) => Some(t),
            ErrorKind::ParseError => None,
        }
    }
}
