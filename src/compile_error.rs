use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError;

impl CompileError {
    pub fn new() -> CompileError {
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
