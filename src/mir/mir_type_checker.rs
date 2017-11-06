use ::mir::mir::*;
use ::mir::visitor::TypedMirWalker;

pub struct MirTypeChecker;

impl TypedMirWalker for MirTypeChecker {
    type T = usize;

    fn visit(&mut self, node: &Mir) -> Self::T {
        match *node {
            Mir::EntryPoint(ref n) => self.visit_entry_point(n),
            Mir::Constant(ref n) => self.visit_constant(n),
            Mir::BinOp(ref n) => self.visit_bin_op(n),
        }
    }

    fn visit_entry_point(&mut self, node: &MirEntryPoint) -> Self::T {
        self.visit(&(*node.next))
    }

    fn visit_constant(&mut self, node: &MirConstant) -> Self::T {
        0
    }

    fn visit_bin_op(&mut self, node: &MirBinOp) -> Self::T {
        // TODO
        self.visit(&(node.left))
    }
}
