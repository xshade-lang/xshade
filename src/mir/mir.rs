#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Link {
    Address(usize),
    Void,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Entry{ next: Link },
}
