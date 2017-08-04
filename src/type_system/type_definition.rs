#[derive(Debug, Eq, PartialEq)]
pub struct TypeDefinition {
    id: usize,
    name: String,
}

impl TypeDefinition {
    pub fn new(id: usize, name: &str) -> TypeDefinition {
        TypeDefinition {
            id: id,
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
        let mut a = TypeDefinition::new(0, "f32");
        let mut b = TypeDefinition::new(0, "f32");

        assert_eq!(a, b);
    }
}
