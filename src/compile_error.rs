use std::error::Error;
use std::fmt;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError;

impl CompileError {
    pub fn new() -> CompileError {
        CompileError
    }

    pub fn unknown() -> CompileError {
        CompileError
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
        None
    }
}
