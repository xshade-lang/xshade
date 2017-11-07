use ::mir::mir::*;
use ::mir::container::*;

pub struct Visitor {
    visitor_impl: Box<MirVisitor>,
}

impl Visitor {
    pub fn new(visitor_impl: Box<MirVisitor>) -> Visitor {
        Visitor {
            visitor_impl: visitor_impl,
        }
    }

    pub fn visit(&mut self, reference: MirReference, container: &mut MirContainer) {
        let next: Vec<MirReference> = if let Some(node) = container.find(reference) {
            match *node {
                Mir::EntryPoint(ref n) => {
                    self.visitor_impl.visit_entry_point(n);
                    vec![n.next]
                },
                Mir::ExitPoint(ref n) => {
                    self.visitor_impl.visit_exit_point(n);
                    vec![]
                },
                _ => unimplemented!(),
            }
        } else {
            vec![]
        };

        for n in next {
            self.visit(n, container);
        }
    }
}

pub trait MirVisitor {
    fn visit_entry_point(&mut self, node: &MirEntryPoint);
    fn visit_exit_point(&mut self, node: &MirExitPoint);
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
