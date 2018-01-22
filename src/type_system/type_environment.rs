use ::std::collections::HashMap;
use ::ast::Span;
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::type_system::type_definition::TypeDefinition;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypeReference {
    id: usize,
}

impl TypeReference {
    pub fn new(id: usize) -> TypeReference {
        TypeReference {
            id: id,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
pub struct TypeEnvironment {
    names_lookup: HashMap<String, TypeReference>,
    types: Vec<TypeDefinition>,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            names_lookup: HashMap::new(),
            types: Vec::new(),
        }
    }

    pub fn create_type(&mut self, name: &str) -> TypeCheckResult<TypeReference> {
        let id = self.types.len();
        let type_definition = TypeDefinition::new(id, name);
        self.types.push(type_definition);
        let type_ref = TypeReference::new(id);
        self.names_lookup.insert(name.to_string(), type_ref);
        Ok(type_ref)
    }

    pub fn find_type(&self, reference: TypeReference) -> Option<&TypeDefinition> {
        let id = reference.get_id();
        if id >= self.types.len() {
            return None;
        }

        Some(&self.types[id])
    }

    pub fn find_type_or_err(&self, reference: TypeReference) -> TypeCheckResult<&TypeDefinition> {
        let id = reference.get_id();
        if id >= self.types.len() {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::TypeNotFound("".to_owned())));
        }

        Ok(&self.types[id])
    }

    pub fn find_type_mut(&mut self, reference: TypeReference) -> Option<&mut TypeDefinition> {
        let id = reference.get_id();
        if id >= self.types.len() {
            return None;
        }

        Some(&mut self.types[id])
    }

    pub fn find_type_mut_or_err(&mut self, reference: TypeReference) -> TypeCheckResult<&mut TypeDefinition> {
        let id = reference.get_id();
        if id >= self.types.len() {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::TypeNotFound("".to_owned())));
        }

        Ok(&mut self.types[id])
    }

    pub fn find_reference_by_name(&self, name: &str) -> Option<TypeReference> {
        if !self.names_lookup.contains_key(name) {
            return None;
        }

        match self.names_lookup.get(name) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn find_type_by_name(&self, name: &str) -> Option<&TypeDefinition> {
        if !self.names_lookup.contains_key(name) {
            return None;
        }

        match self.names_lookup.get(name) {
            Some(r) => self.find_type(r.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_type() {
        let mut type_environment = TypeEnvironment::new();

        let reference = type_environment.create_type("f32").unwrap();

        assert_eq!(type_environment.find_type(reference), Some(&TypeDefinition::new(reference.get_id(), "f32")));
    }
}
