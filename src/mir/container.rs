use ::std::cell::{ Cell, UnsafeCell };
use ::std::collections::HashMap;
use ::mir::mir::Mir;
use ::type_system::type_environment::TypeReference;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MirVariable {
    id: usize,
    variable_type: TypeReference,
}

impl MirVariable {
    fn new(id: usize, variable_type: TypeReference) -> MirVariable {
        MirVariable {
            id: id,
            variable_type: variable_type,
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
    map: UnsafeCell<HashMap<MirReference, UnsafeCell<Mir>>>,
    id_incrementor: Cell<usize>,
}

impl MirContainer {
    pub fn new() -> MirContainer {
        MirContainer {
            map: UnsafeCell::new(HashMap::new()),
            id_incrementor: Cell::new(0),
        }
    }

    pub fn insert(&self, node: Mir) -> MirReference {
        let mut map = unsafe { &mut *self.map.get() };
        let id = self.id_incrementor.get();
        self.id_incrementor.set(id + 1);

        let id = MirReference::new(id);
        map.insert(id, UnsafeCell::new(node));
        id
    }

    pub fn find(&self, reference: MirReference) -> Option<&Mir> {
        let mut map = unsafe { &mut *self.map.get() };
        if let Some(cell) = map.get(&reference) {
            Some(unsafe { &*cell.get() })
        } else {
            None
        }
    }

    pub fn find_mut(&self, reference: MirReference) -> Option<&mut Mir> {
        let mut map = unsafe { &mut *self.map.get() };
        if let Some(cell) = map.get(&reference) {
            Some(unsafe { &mut *cell.get() })
        } else {
            None
        }
    }

    pub fn replace(&self, reference: MirReference, node: Mir) -> Option<Mir> {
        let mut map = unsafe { &mut *self.map.get() };
        if let Some(cell) = map.insert(reference, UnsafeCell::new(node)) {
            Some(unsafe { cell.into_inner() })
        } else {
            None
        }
    }

    pub fn remove(&self, reference: MirReference) -> Option<Mir> {
        let mut map = unsafe { &mut *self.map.get() };
        if let Some(cell) = map.remove(&reference) {
            Some(unsafe { cell.into_inner() })
        } else {
            None
        }
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
