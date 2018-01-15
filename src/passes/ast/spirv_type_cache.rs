use ::std::collections::HashMap;
use ::spirv::Word;
use ::type_system::type_environment::TypeReference;

pub struct SpirvTypeCache {
    cache: HashMap<TypeReference, Word>,
    void: Word,
}

impl SpirvTypeCache {
    pub fn new(void: Word) -> SpirvTypeCache {
        SpirvTypeCache {
            cache: HashMap::new(),
            void: void,
        }
    }

    pub fn get_void(&self) -> Word {
        self.void
    }

    pub fn find(&self, type_reference: TypeReference) -> Option<&Word> {
        self.cache.get(&type_reference)
    }

    pub fn insert(&mut self, type_reference: TypeReference, spirv_type: Word) {
        self.cache.insert(type_reference, spirv_type);
    }
}
