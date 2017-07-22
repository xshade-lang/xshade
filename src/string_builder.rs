pub struct StringBuilder {
    buffer: Vec<u8>,
}

impl StringBuilder {
    pub fn new(capacity: usize) -> StringBuilder {
        StringBuilder {
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.buffer.extend(s.as_bytes().iter().cloned());
    }

    pub fn to_string(self) -> Option<String> {
        match String::from_utf8(self.buffer) {
            Ok(string) => Some(string),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn build_string() {
        let mut sb = StringBuilder::new(64);
        sb.append("foo");
        sb.append("bar");
        assert_eq!(Some("foobar".to_string()), sb.to_string());
    }
}
