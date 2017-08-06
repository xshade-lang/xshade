use ::ast::*;
use ::type_system::type_environment::TypeReference;

#[derive(Debug, Eq)]
pub struct TypeDefinition {
    id: usize,
    name: String,
    implicit_casts: Vec<TypeReference>,
    explicit_casts: Vec<TypeReference>,
}

impl TypeDefinition {
    pub fn new(id: usize, name: &str) -> TypeDefinition {
        TypeDefinition {
            id: id,
            name: name.to_string(),
            implicit_casts: Vec::new(),
            explicit_casts: Vec::new(),
        }
    }

    pub fn does_cast_exist(&self, other: TypeReference) -> bool {
        self.implicit_casts.iter().any(|&t| t == other) ||
        self.explicit_casts.iter().any(|&t| t == other)
    }

    pub fn does_implicit_cast_exist(&self, other: TypeReference) -> bool {
        self.implicit_casts.iter().any(|&t| t == other)
    }

    pub fn does_explicit_cast_exist(&self, other: TypeReference) -> bool {
        self.explicit_casts.iter().any(|&t| t == other)
    }

    pub fn add_implicit_cast(&mut self, other: &TypeReference) {
        self.implicit_casts.push(other.clone());
    }

    pub fn add_explicit_cast(&mut self, other: &TypeReference) {
        self.explicit_casts.push(other.clone());
    }
}

impl PartialEq for TypeDefinition {
    fn eq(&self, other: &TypeDefinition) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn same_types_are_equal() {
        let a = TypeDefinition::new(0, "f32");
        let b = TypeDefinition::new(0, "f32");

        assert_eq!(a, b);
    }
}
