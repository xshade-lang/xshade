use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;
use ::code_map::CodeMap;
use ::error_formatter::ErrorFormatter;
use ::passes::error::{ PassError, Severity };

#[derive(Debug)]
pub struct TypeWithSameNameAlreadyDefinedError {
    span: Span,
    type_name: String,
}

impl TypeWithSameNameAlreadyDefinedError {
    pub fn new(span: Span, type_name: String) -> TypeWithSameNameAlreadyDefinedError {
        TypeWithSameNameAlreadyDefinedError {
            span: span,
            type_name: type_name,
        }
    }
}

impl PassError for TypeWithSameNameAlreadyDefinedError {
    fn get_severity(&self) -> Severity {
        Severity::Error
    }

    fn format(&self, code_map: &CodeMap) -> String {
        let mut formatter = ErrorFormatter::new();

        formatter.add_line(&format!("error: type with name \"{}\" already defined", self.type_name));
        formatter.add_line(&format!("> {}:{}:{}", code_map.get_path(), self.span.line, self.span.column));
        formatter.add_line(code_map.get_span_line(self.span));
        formatter.add_underlines(self.span);

        formatter.finish()
    }
}

impl Error for TypeWithSameNameAlreadyDefinedError {
    fn description(&self) -> &str {
        "A with the same name was already defined"
    }
}

impl fmt::Display for TypeWithSameNameAlreadyDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Type \"{}\" already defined.", self.type_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &'static str = "
struct AlreadyDefined {

}
    ";

    #[test]
    pub fn test() {
        let code_map = CodeMap::new("src\\lib.xs".to_string(), CODE.to_string());
        let error = TypeWithSameNameAlreadyDefinedError::new(Span::new(8, 14, 2, 8), "AlreadyDefined".to_string());
        assert_eq!(error.format(&code_map), "error: type with name \"AlreadyDefined\" already defined\n> src\\lib.xs:2:8\nstruct AlreadyDefined {\n       ^^^^^^^^^^^^^^\n");
    }
}
