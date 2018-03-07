use ::types::Type;

// TODO member types
pub struct StructureType {
    name: String,
}

impl StructureType {
    pub fn new(name: String) -> StructureType {
        StructureType {
            name: name,
        }
    }
}

impl Type for StructureType {
    fn get_name(&self) -> &str { &self.name }
    fn is_function_type(&self) -> bool { false }
    fn is_structure_type(&self) -> bool { true }
    fn is_value_type(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_queries() {
        let function_type = StructureType::new("Foo".to_string());
        assert_eq!(function_type.is_function_type(), false);
        assert_eq!(function_type.is_structure_type(), true);
        assert_eq!(function_type.is_value_type(), false);
    }
}
