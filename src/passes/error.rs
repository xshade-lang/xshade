use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;

#[derive(Debug)]
pub struct PassError {
    span: Span,
}

impl PassError {
    pub fn new(span: Span) -> PassError {
        PassError {
            span: span,
        }
    }
}

impl Error for PassError {
    fn description(&self) -> &str {
        "An error occured during a pass"
    }
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "An error occured during a pass")
    }
}
