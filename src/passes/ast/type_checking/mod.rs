use ::ast::Ast;
use ::passes::{ Pass, PassCollection };
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::SymbolTableReference;
use ::type_system::type_environment::TypeEnvironmentReference;

mod error;
mod check_primitives_pass;
mod discover_structs_pass;
// mod check_struct_member_pass;
// mod check_exports_pass;
// mod check_function_signatures_pass;

pub struct TypeChecker {
    passes: PassCollection<Ast>,
}

impl TypeChecker {
    pub fn new(symbol_table: SymbolTableReference, type_environment: TypeEnvironmentReference, result: PassResultReference) -> TypeChecker {
        TypeChecker {
            passes: PassCollection::from_passes(vec![
                Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), type_environment.clone(), result.clone())),
                Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), type_environment.clone(), result.clone())),
                // Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
                // Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
                // Box::new(check_exports_pass::CheckExportsPass::new(symbol_table.clone(), result.clone())),
            ]),
        }
    }
}

impl Pass<Ast> for TypeChecker {
    fn execute(&mut self, items: &mut Ast) {
        self.passes.execute(items);
    }
}
