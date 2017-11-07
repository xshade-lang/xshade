use ::mir::mir::*;
use ::mir::visitor::TypedMirWalker;
use ::type_system::type_environment::TypeReference;

pub struct MirTypeChecker;

impl TypedMirWalker for MirTypeChecker {
    type T = TypeReference;

    fn visit(&mut self, node: &Mir) -> Self::T {
        match *node {
            Mir::EntryPoint(ref n) => self.visit_entry_point(n),
            Mir::Constant(ref n) => self.visit_constant(n),
            Mir::BinOp(ref n) => self.visit_bin_op(n),
            _ => unimplemented!(),
        }
    }

    fn visit_entry_point(&mut self, node: &MirEntryPoint) -> Self::T {
        unimplemented!()
    }

    fn visit_constant(&mut self, node: &MirConstant) -> Self::T {
        unimplemented!()
    }

    fn visit_bin_op(&mut self, node: &MirBinOp) -> Self::T {
        unimplemented!()
    }
}
