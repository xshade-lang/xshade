use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;
use ::code_map::CodeMap;
use ::error_formatter::ErrorFormatter;
use ::passes::error::{ PassError, Severity };

#[derive(Debug)]
pub struct ExperimentalSyntaxWarning {
    span: Span,
    syntax_name: String,
}

impl ExperimentalSyntaxWarning {
    pub fn new(span: Span, syntax_name: &str) -> ExperimentalSyntaxWarning {
        ExperimentalSyntaxWarning {
            span: span,
            syntax_name: syntax_name.to_string(),
        }
    }
}

impl PassError for ExperimentalSyntaxWarning {
    fn get_severity(&self) -> Severity {
        Severity::Warning
    }

    fn format(&self, code_map: &CodeMap) -> String {
        let mut formatter = ErrorFormatter::new();

        formatter.add_line(&format!("warning: use of experimental syntax \"{}\"", self.syntax_name));
        formatter.add_line(&format!("> {}:{}:{}", code_map.get_path(), self.span.line, self.span.column));
        formatter.add_line(code_map.get_span_line(self.span));
        formatter.add_underlines(self.span);

        formatter.finish()
    }
}

impl Error for ExperimentalSyntaxWarning {
    fn description(&self) -> &str {
        "Use of experimental syntax"
    }
}

impl fmt::Display for ExperimentalSyntaxWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Experimental syntax \"{}\" used.", self.syntax_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &'static str = "export foo;";

    #[test]
    pub fn test() {
        let code_map = CodeMap::new("src\\lib.xs".to_string(), CODE.to_string());
        let error = ExperimentalSyntaxWarning::new(Span::new(0, 11, 1, 1), "export");
        assert_eq!(error.format(&code_map), "warning: use of experimental syntax \"export\"\n> src\\lib.xs:1:1\nexport foo;\n^^^^^^^^^^^\n");
    }
}
