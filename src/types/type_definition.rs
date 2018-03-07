pub struct TypeDefinition {
    name: String,
}

impl TypeDefinition {
    pub fn new(name: String) -> TypeDefinition {
        TypeDefinition {
            name: name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn refine(&mut self) {
        // todo refine this type into a fully qualified type (e.g. StructureType, FunctionType, etc)
    }
}
