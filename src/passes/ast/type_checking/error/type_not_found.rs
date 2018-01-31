use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;
use ::code_map::CodeMap;
use ::error_formatter::ErrorFormatter;
use ::passes::error::{ PassError, Severity };

#[derive(Debug)]
pub struct TypeNotFoundError {
    span: Span,
    type_name: String,
}

impl TypeNotFoundError {
    pub fn new(span: Span, type_name: String) -> TypeNotFoundError {
        TypeNotFoundError {
            span: span,
            type_name: type_name,
        }
    }
}

impl PassError for TypeNotFoundError {
    fn get_severity(&self) -> Severity {
        Severity::Error
    }

    fn format(&self, code_map: &CodeMap) -> String {
        let mut formatter = ErrorFormatter::new();

        formatter.add_line(&format!("error: type \"{}\" not found", self.type_name));
        formatter.add_line(&format!("> {}:{}:{}", code_map.get_path(), self.span.line, self.span.column));
        formatter.add_line(code_map.get_span_line(self.span));
        formatter.add_underlines(self.span);

        formatter.finish()
    }
}

impl Error for TypeNotFoundError {
    fn description(&self) -> &str {
        "A Type was not found"
    }
}

impl fmt::Display for TypeNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Type \"{}\" not found.", self.type_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &'static str = "

pub fn main(in: T) -> T {
    return in;
}

    ";

    #[test]
    pub fn test() {
        let code_map = CodeMap::new("src\\lib.xs".to_string(), CODE.to_string());
        let error = TypeNotFoundError::new(Span::new(18, 1, 3, 17), "T".to_string());
        assert_eq!(error.format(&code_map), "error: type \"T\" not found\n> src\\lib.xs:3:17\npub fn main(in: T) -> T {\n                ^\n");
    }
}
