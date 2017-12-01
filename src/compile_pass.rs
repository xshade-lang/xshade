use ::module::Module;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;
use ::compile_error::CompileError;

#[derive(Debug)]
pub struct CompilePass {
    uid: i32,
    modules: Vec<Module>,
    symbol_table: SymbolTable,
}

impl CompilePass {
    pub fn new(uid: i32, modules: Vec<Module>) -> CompilePass {
        CompilePass {
            uid: uid,
            modules: modules,
            symbol_table: SymbolTable::new(TypeEnvironment::new()),
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
}