use ::types::Type;
use ::types::FunctionType;

#[derive(Debug)]
pub struct FunctionTypeBuilder {
    name: Option<String>,
}

impl FunctionTypeBuilder {
    pub fn new() -> FunctionTypeBuilder {
        FunctionTypeBuilder {
            name: None,
        }
    }

    pub fn with_name_str(mut self, name: &str) -> FunctionTypeBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_name(mut self, name: String) -> FunctionTypeBuilder {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<Box<Type>, ()> {
        let name = match self.name {
            Some(n) => n.to_string(),
            None => return Err(()),
        };

        Ok(Box::new(FunctionType::new(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_type_with_name() {
        let mut builder = FunctionTypeBuilder::new();
        let built_type = builder.with_name_str("SomeType").build().unwrap();

        assert_eq!(built_type.get_name(), "SomeType");
    }
}
