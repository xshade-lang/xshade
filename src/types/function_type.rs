use ::types::Type;

// TODO arguments
// TODO return type
pub struct FunctionType {
    name: String,
}

impl FunctionType {
    pub fn new(name: String) -> FunctionType {
        FunctionType {
            name: name,
        }
    }
}

impl Type for FunctionType {
    fn get_name(&self) -> &str { &self.name }
    fn is_function_type(&self) -> bool { true }
    fn is_structure_type(&self) -> bool { false }
    fn is_value_type(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_queries() {
        let function_type = FunctionType::new("Foo".to_string());
        assert_eq!(function_type.is_function_type(), true);
        assert_eq!(function_type.is_structure_type(), false);
        assert_eq!(function_type.is_value_type(), false);
    }
}
