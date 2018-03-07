pub mod builder;
pub mod error;

pub mod function_type;
pub mod value_type;
pub mod structure_type;
pub mod type_constructor;
pub mod type_environment;

pub use self::function_type::FunctionType;
pub use self::structure_type::StructureType;
pub use self::value_type::ValueType;
pub use self::type_environment::TypeEnvironment;

pub trait Type {
    fn get_name(&self) -> &str;
    fn is_function_type(&self) -> bool;
    fn is_structure_type(&self) -> bool;
    fn is_value_type(&self) -> bool;
}
