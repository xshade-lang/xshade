use ::std::collections::HashMap;
use ::ast::Span;

#[derive(Debug)]
pub struct Type {
    id: usize,
    declaration: Span,
}

impl Type {
    pub fn new(id: usize, declaration: Span) -> Type {
        Type {
            id: id,
            declaration: declaration,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeReference {
    id: usize,
}

impl TypeReference {
    pub fn new(id: usize) -> TypeReference {
        TypeReference {
            id: id,
        }
    }
}

#[derive(Debug)]
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

    pub fn create_struct_type(&mut self) -> TypeReference {
        panic!("");
    }

    pub fn create_primitive_type(&mut self) -> TypeReference {
        panic!("");
    }

    pub fn create_function_type(&mut self) -> TypeReference {
        panic!("");
    }

    pub fn find_type_by_name(&self) -> &Type {
        panic!("");
    }

    pub fn find_type_by_name_mut(&mut self) -> &mut Type {
        panic!("");
    }

    pub fn find_type_by_reference(&self) -> &Type {
        panic!("");
    }

    pub fn find_type_by_reference_mut(&mut self) -> &mut Type {
        panic!("");
    }
}
