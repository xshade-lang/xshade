use ::mir::mir::*;
use ::mir::container::*;

pub trait MirWalker {

    fn visit(&mut self, reference: MirReference, container: &MirContainer) {
        if let Some(wrapper) = container.find_mut(reference) {
            match *wrapper {
                Mir::EntryPoint(ref mut node) => self.visit_entry_point(node, container),
                Mir::ExitPoint(ref mut node) => self.visit_exit_point(node, container),
                Mir::Loop(ref mut node) => self.visit_loop(node, container),
                Mir::LoopMerge(ref mut node) => self.visit_loop_merge(node, container),
                Mir::Constant(ref mut node) => self.visit_constant(node, container),
                Mir::BinOp(ref mut node) => self.visit_bin_op(node, container),
            }
        }
    }

    fn visit_entry_point(&mut self, node: &MirEntryPoint, container: &MirContainer);
    fn visit_exit_point(&mut self, node: &MirExitPoint, container: &MirContainer);
    fn visit_loop(&mut self, node: &MirLoop, container: &MirContainer);
    fn visit_loop_merge(&mut self, node: &MirLoopMerge, container: &MirContainer);
    fn visit_constant(&mut self, node: &MirConstant, container: &MirContainer);
    fn visit_bin_op(&mut self, node: &MirBinOp, container: &MirContainer);
}

pub trait TypedMirWalker {
    type T;

    fn visit(&mut self, reference: MirReference, container: &MirContainer) -> Option<Self::T> {
        if let Some(wrapper) = container.find_mut(reference) {
            match *wrapper {
                Mir::EntryPoint(ref mut node) => self.visit_entry_point(node, container),
                Mir::ExitPoint(ref mut node) => self.visit_exit_point(node, container),
                Mir::Loop(ref mut node) => self.visit_loop(node, container),
                Mir::LoopMerge(ref mut node) => self.visit_loop_merge(node, container),
                Mir::Constant(ref mut node) => self.visit_constant(node, container),
                Mir::BinOp(ref mut node) => self.visit_bin_op(node, container),
            }
        } else {
            None
        }
    }

    fn visit_entry_point(&mut self, node: &MirEntryPoint, container: &MirContainer) -> Option<Self::T>;
    fn visit_exit_point(&mut self, node: &MirExitPoint, container: &MirContainer) -> Option<Self::T>;
    fn visit_loop(&mut self, node: &MirLoop, container: &MirContainer) -> Option<Self::T>;
    fn visit_loop_merge(&mut self, node: &MirLoopMerge, container: &MirContainer) -> Option<Self::T>;
    fn visit_constant(&mut self, node: &MirConstant, container: &MirContainer) -> Option<Self::T>;
    fn visit_bin_op(&mut self, node: &MirBinOp, container: &MirContainer) -> Option<Self::T>;
}

pub trait MirReplacer {
    fn visit(&mut self, node: &Mir) -> Mir;
    fn visit_entry_point(&mut self, node: &MirEntryPoint) -> Mir;
    fn visit_constant(&mut self, node: &MirConstant) -> Mir;
    fn visit_bin_op(&mut self, node: &MirBinOp) -> Mir;
}
