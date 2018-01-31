use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;
use ::code_map::CodeMap;

#[derive(Debug, Copy, Clone)]
pub enum Severity {
    Warning,
    Error,
    Fatal,
}

pub trait PassError {
    fn format(&self, code_map: &CodeMap) -> String;
    fn get_severity(&self) -> Severity;

    fn is_warning(&self) -> bool {
        match self.get_severity() {
            Severity::Warning => true,
            _ => false,
        }
    }

    fn is_error(&self) -> bool {
        match self.get_severity() {
            Severity::Error => true,
            _ => false,
        }
    }

    fn is_fatal(&self) -> bool {
        match self.get_severity() {
            Severity::Fatal => true,
            _ => false,
        }
    }
}
