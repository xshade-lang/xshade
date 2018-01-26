use ::ast::Ast;
use ::passes::{ Pass, PassCollection };
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::SymbolTableReference;

mod check_primitives_pass;
mod discover_structs_pass;
mod check_struct_member_pass;
mod check_expression_pass;

pub struct TypeChecker {
    passes: PassCollection<Ast>,
}

impl TypeChecker {
    pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> TypeChecker {
        TypeChecker {
            passes: PassCollection::from_passes(vec![
                Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
                Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
                Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            ]),
        }
    }
}

impl Pass<Ast> for TypeChecker {
    fn execute(&mut self, items: &mut Ast) {
        self.passes.execute(items);
    }
}
