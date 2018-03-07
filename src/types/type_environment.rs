use ::std::collections::HashMap;
use ::data_structures::shared::Shared;
use ::types::error::TypeConstructionError;

pub type TypeEnvironmentReference = Shared<TypeEnvironment>;

#[derive(Debug, Copy, Clone)]
pub struct Type;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct TypeReference {
    id: usize,
}

impl TypeReference {
    fn new(id: usize) -> TypeReference {
        TypeReference {
            id: id,
        }
    }
}

pub struct TypeEnvironment {
    names_lookup: HashMap<String, TypeReference>,
    types: Vec<Type>,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            names_lookup: HashMap::new(),
            types: Vec::new(),
        }
    }

    pub fn find_type(&self, reference: TypeReference) -> Option<&Type> {
        let id = reference.id;
        if id >= self.types.len() {
            return None;
        }

        Some(&self.types[id])
    }

    pub fn find_type_mut(&mut self, reference: TypeReference) -> Option<&mut Type> {
        let id = reference.id;
        if id >= self.types.len() {
            return None;
        }

        Some(&mut self.types[id])
    }

    fn create_structure_type(&mut self, name: String) -> Result<TypeReference, TypeConstructionError> {
        let id = self.types.len();
        let type_ref = TypeReference::new(id);

        let new_type = Type{};

        self.types.push(new_type);
        self.names_lookup.insert(name.to_string(), type_ref);

        Ok(type_ref)
    }
}
