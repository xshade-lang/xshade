mod generate_spirv_pass;
mod spirv_symbol_table;
mod spirv_type_cache;

use ::passes::{ Pass, PassCollection };
use ::passes::results::PassResultReference;
use ::ast::ItemKind;
use ::type_system::symbol_table::SymbolTableReference;
use ::passes::ast::code_generation::generate_spirv_pass::GenerateSpirvPass;

pub struct CodeGenerator {
    passes: PassCollection<Vec<ItemKind>>,
}

impl CodeGenerator {
    pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> CodeGenerator {
        CodeGenerator {
            passes: PassCollection::from_passes(vec![
                Box::new(GenerateSpirvPass::new(symbol_table.clone(), result.clone())),
            ]),
        }
    }
}

impl Pass<Vec<ItemKind>> for CodeGenerator {
    fn execute(&mut self, items: &mut Vec<ItemKind>) {
        self.passes.execute(items);
    }
}
