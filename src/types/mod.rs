pub mod function_type;
pub mod value_type;
pub mod structure_type;
pub mod type_environment;
pub mod type_reference;

pub struct Type;

impl Type {
    pub fn new(name: String) -> Type {
        Type
    }

    pub fn is_function_type(&self) -> bool { unimplemented!() }
    pub fn is_structure_type(&self) -> bool { unimplemented!() }
    pub fn is_value_type(&self) -> bool { unimplemented!() }
}
