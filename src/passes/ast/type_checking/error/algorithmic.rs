use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;
use ::code_map::CodeMap;
use ::error_formatter::ErrorFormatter;
use ::passes::error::{ PassError, Severity };

#[derive(Debug)]
pub struct AlgorithmicError {
    span: Span,
    reason: String,
}

impl AlgorithmicError {
    pub fn new(span: Span, reason: &str) -> AlgorithmicError {
        AlgorithmicError {
            span: span,
            reason: reason.to_string(),
        }
    }
}

impl PassError for AlgorithmicError {
    fn get_severity(&self) -> Severity {
        Severity::Fatal
    }

    fn format(&self, code_map: &CodeMap) -> String {
        self.reason.to_string()
    }
}

impl Error for AlgorithmicError {
    fn description(&self) -> &str {
        "An algorithmic error occured"
    }
}

impl fmt::Display for AlgorithmicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Algorithmic error: \"{}\"", self.reason)
    }
}
