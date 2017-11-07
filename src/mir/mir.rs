use ::mir::container::{ MirVariable, MirReference };

#[derive(Debug, Eq, PartialEq)]
pub enum Mir {
    EntryPoint(MirEntryPoint),
    ExitPoint(MirExitPoint),
    Loop(MirLoop),
    LoopMerge(MirLoop),
    Constant(MirConstant),
    BinOp(MirBinOp)
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirEntryPoint {
    pub next: MirReference,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirExitPoint;

#[derive(Debug, Eq, PartialEq)]
pub struct MirLoop {
    pub condition: MirVariable,
    pub loop_body: MirReference,
    pub next: MirReference,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirLoopMerge {
    next: MirReference,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirConstant {
    pub value: Vec<u8>,
    pub variable: MirVariable,
    next: MirReference,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MirBinOpType {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MirBinOp {
    pub op_type: MirBinOpType,
    pub left: MirVariable,
    pub right: MirVariable,
    pub result: MirVariable,
    pub next: MirReference,
}
