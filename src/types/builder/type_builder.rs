use ::types::type_definition::TypeDefinition;

#[derive(Debug)]
pub struct TypeBuilder {
    name: Option<String>,
}

impl TypeBuilder {
    pub fn new() -> TypeBuilder {
        TypeBuilder {
            name: None,
        }
    }

    pub fn with_name_str(mut self, name: &str) -> TypeBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_name(mut self, name: String) -> TypeBuilder {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<TypeDefinition, ()> {
        let name = match self.name {
            Some(n) => n.to_string(),
            None => return Err(()),
        };

        Ok(TypeDefinition::new(name))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_type_with_name() {
        let mut builder = TypeBuilder::new();

        let built_type = builder.with_name_str("SomeType").build().unwrap();

        assert_eq!(built_type.get_name(), "SomeType");
    }
}
