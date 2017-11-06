use ::mir::mir::*;

pub trait MirWalker {
    fn visit(&mut self, node: &Mir);
    fn visit_entry_point(&mut self, node: &MirEntryPoint);
    fn visit_constant(&mut self, node: &MirConstant);
    fn visit_bin_op(&mut self, node: &MirBinOp);
}

pub trait TypedMirWalker {
    type T;
    fn visit(&mut self, node: &Mir) -> Self::T;
    fn visit_entry_point(&mut self, node: &MirEntryPoint) -> Self::T;
    fn visit_constant(&mut self, node: &MirConstant) -> Self::T;
    fn visit_bin_op(&mut self, node: &MirBinOp) -> Self::T;
}

pub trait MirReplacer {
    fn visit(&mut self, node: &Mir) -> Mir;
    fn visit_entry_point(&mut self, node: &MirEntryPoint) -> Mir;
    fn visit_constant(&mut self, node: &MirConstant) -> Mir;
    fn visit_bin_op(&mut self, node: &MirBinOp) -> Mir;
}
