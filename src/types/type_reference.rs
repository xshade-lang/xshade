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
