use std::collections::HashMap;
use ::mir::mir::Mir;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MirVariable {
    id: usize,
}

impl MirVariable {
    fn new(id: usize) -> MirVariable {
        MirVariable {
            id: id,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MirReference {
    id: usize,
}

impl MirReference {
    fn new(id: usize) -> MirReference {
        MirReference {
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct MirContainer {
    map: HashMap<MirReference, Mir>,
    id_incrementor: usize,
}

impl MirContainer {
    pub fn new() -> MirContainer {
        MirContainer {
            map: HashMap::new(),
            id_incrementor: 0,
        }
    }

    pub fn insert(&mut self, node: Mir) -> MirReference {
        let id = MirReference::new(self.id_incrementor);
        self.id_incrementor += 1;
        self.map.insert(id, node);
        id
    }

    pub fn find(&mut self, reference: MirReference) -> Option<&Mir> {
        self.map.get(&reference)
    }

    pub fn find_mut(&mut self, reference: MirReference) -> Option<&mut Mir> {
        self.map.get_mut(&reference)
    }

    pub fn replace(&mut self, reference: MirReference, node: Mir) -> Option<Mir> {
        self.map.insert(reference, node)
    }

    pub fn remove(&mut self, reference: MirReference) -> Option<Mir> {
        self.map.remove(&reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::mir::mir::*;

    #[test]
    fn it_creates_a_simple_graph() {
        let mut container = MirContainer::new();
        let exit_id = container.insert(Mir::ExitPoint(MirExitPoint{}));
        let entry_id = container.insert(Mir::EntryPoint(MirEntryPoint {
            next: exit_id,
        }));

        assert_eq!(Some(&Mir::EntryPoint(MirEntryPoint{ next: exit_id })), container.find(entry_id));
    }
}
