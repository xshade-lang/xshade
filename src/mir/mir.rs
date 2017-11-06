#[derive(Debug, Eq, PartialEq)]
pub enum Mir {
    EntryPoint(MirEntryPoint),
    Constant(MirConstant),
    BinOp(MirBinOp)
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirEntryPoint {
    pub next: Box<Mir>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirConstant {
    pub value: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirBinOp {
    pub op_type: MirBinOpType,
    pub left: Box<Mir>,
    pub right: Box<Mir>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MirBinOpType {
    Add,
    Subtract,
    Divide,
    Multiply,
}
