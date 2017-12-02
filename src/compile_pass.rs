use ::module::Module;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;
use ::compile_error::CompileError;

#[derive(Debug)]
pub struct CompilePass {
    uid: i32,
    pub modules: Vec<Module>,
    pub symbol_table: SymbolTable,
}

impl CompilePass {
    pub fn new(uid: i32, modules: Vec<Module>, symbol_table: SymbolTable) -> CompilePass {
        CompilePass {
            uid: uid,
            modules: modules,
            symbol_table: symbol_table, // Symbol-Table is prefilled with core modules
        }
    }

    pub fn get_uid(&self) -> i32 { 
        self.uid
    }

    pub fn get_modules(&self) -> Vec<&Module> {
         self.modules.iter().map(|x| x).collect() // Somehow transforms Module->&Module @Vengarioth? Halp to understand please!
    }
 
    pub fn get_modules_mut(&mut self) -> Vec<&mut Module> {
        self.modules.iter_mut().map(|x| x).collect() // Somehow transforms mut Module->&mut Module @Vengarioth? Halp to understand please!
    }

    pub fn get_symbols(&self) -> &SymbolTable {
        &self.symbol_table // Somehow transforms Module->&Module @Vengarioth? Halp to understand please!
    }
 
    pub fn get_symbols_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table // Somehow transforms mut Module->&mut Module @Vengarioth? Halp to understand please!
    }
}