use ::types::function_type::FunctionType;
use ::types::structure_type::StructureType;
use ::types::primitive_type::PrimitiveType;

pub enum RefinedType {
    None,
    Function(FunctionType),
    Structure(StructureType),
    Primitive(PrimitiveType),
}
