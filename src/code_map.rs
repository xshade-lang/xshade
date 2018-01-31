use ::ast::Span;

pub struct CodeMap {
    path: String,
    source: String,
}

impl CodeMap {
    pub fn new(path: String, source: String) -> CodeMap {
        CodeMap {
            path: path,
            source: source,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_span_contents(&self, span: Span) -> &str {
        let from = span.offset;
        let to = span.offset + span.length;
        &self.source[from..to]
    }

    pub fn get_span_line(&self, span: Span) -> &str {
        let lines: Vec<&str> = self.source.lines().collect();
        lines[span.line - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &'static str = "
fn main(in: f32) -> f32 {
    return in;
}
    ";

    #[test]
    pub fn test_get_span_contents() {
        let code_map = CodeMap::new("some/path/to/file.xs".to_string(), CODE.to_string());
        assert_eq!(code_map.get_span_contents(Span::new(31, 10, 3, 4)), "return in;");
    }

    #[test]
    pub fn test_get_span_line() {
        let code_map = CodeMap::new("some/path/to/file.xs".to_string(), CODE.to_string());
        assert_eq!(code_map.get_span_line(Span::new(31, 10, 3, 4)), "    return in;");
    }
}
