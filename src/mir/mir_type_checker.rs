use ::mir::mir::*;
use ::mir::container::MirContainer;
use ::mir::visitor::TypedMirWalker;
use ::type_system::type_environment::TypeReference;

pub struct MirTypeChecker;

impl TypedMirWalker for MirTypeChecker {
    type T = TypeReference;

    fn visit_entry_point(&mut self, node: &MirEntryPoint, container: &MirContainer) -> Option<Self::T> {
        self.visit(node.next, container)
    }

    fn visit_exit_point(&mut self, node: &MirExitPoint, container: &MirContainer) -> Option<Self::T> {
        None
    }
    
    fn visit_loop(&mut self, node: &MirLoop, container: &MirContainer) -> Option<Self::T> {
        self.visit(node.next, container)
    }
    
    fn visit_loop_merge(&mut self, node: &MirLoopMerge, container: &MirContainer) -> Option<Self::T> {
        self.visit(node.next, container)
    }
    
    fn visit_constant(&mut self, node: &MirConstant, container: &MirContainer) -> Option<Self::T> {
        self.visit(node.next, container)
    }
    
    fn visit_bin_op(&mut self, node: &MirBinOp, container: &MirContainer) -> Option<Self::T> {
        self.visit(node.next, container)
    }
}
