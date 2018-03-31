use ::std::collections::HashMap;
use ::data_structures::shared::Shared;
use ::types::error::TypeConstructionError;

pub type TypeEnvironmentReference = Shared<TypeEnvironment>;

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
    value_types: Vec<ValueType>,
    product_types: Vec<ProductType>,
    function_types: Vec<FunctionType>,
    trait_types: Vec<TraitType>,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
        }
    }

    pub fn find_type_by_name(name: &str) -> TypeReference {
        unimplemented!()
    }
    
    pub fn create_value_type(name: String) -> TypeReference {
        unimplemented!()
    }

    pub fn create_product_type(name: String) -> TypeReference {
        unimplemented!()
    }

    pub fn create_function_type(name: String) -> TypeReference {
        unimplemented!()
    }

    pub fn create_trait_type(name: String) -> TypeReference {
        unimplemented!()
    }

    pub fn add_implementation(type_reference: TypeReference) {
        unimplemented!()
    }
}
