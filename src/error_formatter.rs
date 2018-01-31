use ::ast::Span;

pub struct ErrorFormatter {
    result: String,
}

impl ErrorFormatter {
    pub fn new() -> ErrorFormatter {
        ErrorFormatter {
            result: String::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.result.push_str(line);
        self.result.push('\n');
    }

    pub fn add_underlines(&mut self, span: Span) {
        let mut marker = String::new();
        for i in 0..span.column - 1 {
            marker.push_str(" ");
        }
        for i in 0..span.length {
            marker.push_str("^");
        }
        self.result.push_str(&marker);
        self.result.push('\n');
    }

    pub fn finish(self) -> String {
        self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_line() {
        let mut error_formatter = ErrorFormatter::new();
        error_formatter.add_line("foo");
        error_formatter.add_line("bar");
        assert_eq!(&error_formatter.finish(), "foo\nbar\n");
    }

    #[test]
    pub fn test_add_underlines() {
        let mut error_formatter = ErrorFormatter::new();
        error_formatter.add_line("     foo");
        error_formatter.add_underlines(Span::new(5, 3, 1, 6));
        assert_eq!(&error_formatter.finish(), "     foo\n     ^^^\n");
    }
}
