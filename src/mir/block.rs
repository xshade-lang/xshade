use ::std::collections::HashMap;
use ::mir::mir::*;

pub struct Block {
    items: HashMap<usize, Op>,
    entry: Link,
    next_id: usize,
}

impl Block {
    pub fn new() -> Block {
        let mut items = HashMap::new();
        let entry = Link::Address(0);
        items.insert(0, Op::Entry{ next: Link::Void });
        Block {
            items: items,
            entry: entry,
            next_id: 1,
        }
    }

    pub fn get(&self, link: Link) -> Option<&Op> {
        match link {
            Link::Address(address) => match self.items.get(&address) {
                Some(op) => Some(op),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_mut(&mut self, link: Link) -> Option<&mut Op> {
        match link {
            Link::Address(address) => match self.items.get_mut(&address) {
                Some(mut op) => Some(op),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_entry(&self) -> Option<&Op> {
        self.get(self.entry)
    }

    pub fn get_entry_mut(&mut self) -> Option<&mut Op> {
        let entry = { self.entry };
        self.get_mut(entry)
    }

    pub fn add(&mut self, op: Op) -> Link {
        self.next_id += 1;
        self.items.insert(self.next_id, op);
        Link::Address(self.next_id)
    }

    pub fn remove(&mut self, link: Link) {
        match link {
            Link::Address(address) => { self.items.remove(&address); },
            _ => {},
        };
    }
}
