use ::passes::{ Pass, PassCollection, PassResult };
use ::ast::ItemKind;
use ::type_system::symbol_table::SymbolTableReference;

pub struct TypeChecker {
    passes: PassCollection<Vec<ItemKind>>,
    symbol_table: SymbolTableReference,
}

impl TypeChecker {
    pub fn new(symbol_table: SymbolTableReference) -> TypeChecker {
        TypeChecker {
            passes: PassCollection::new(),
            symbol_table: symbol_table,
        }
    }
}

impl Pass<Vec<ItemKind>> for TypeChecker {
    fn execute(&mut self, items: &mut Vec<ItemKind>) -> PassResult {
        self.passes.execute(items)
    }
}
