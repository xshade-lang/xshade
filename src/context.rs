use ::data_structures::{ StringId, StringInterner };
use ::type_system::type_definition::TypeDefinition;

pub struct Context {
    string_interner: StringInterner,
}

impl Context {
    pub fn new() -> Context {
        Context {
            string_interner: StringInterner::new(),
        }
    }

    pub fn intern_str(&mut self, s: &str) -> StringId {
        self.string_interner.intern(s)
    }

    pub fn find_str(&self, id: StringId) -> Option<String> {
        self.string_interner.find(id)
    }

    pub fn find_type(&self) -> Option<&TypeDefinition> {
        None
    }

    pub fn find_type_mut(&mut self) -> Option<&mut TypeDefinition> {
        None
    }

    pub fn find_type_by_name(&self, id: StringId) -> Option<TypeDefinition> {
        None
    }

    pub fn find_type_by_name_mut(&mut self, id: StringId) -> Option<&mut TypeDefinition> {
        None
    }
}
