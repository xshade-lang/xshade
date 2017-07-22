#[derive(Debug, Eq, PartialEq)]
pub struct TypeDefinition {
    name: String,
}

impl TypeDefinition {
    pub fn new(name: &str) -> TypeDefinition {
        TypeDefinition {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn same_types_are_equal() {
        let mut a = TypeDefinition::new("TypeA");
        let mut b = TypeDefinition::new("TypeA");

        assert_eq!(a, b);
    }
}
