use ::std::collections::HashMap;
use ::spirv::Word;
use ::type_system::type_environment::TypeReference;

struct Scope {
    symbols: HashMap<String, Word>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            symbols: HashMap::new(),
        }
    }
}

pub struct SpirvSymbolTable {
    scopes: Vec<Scope>,
}

impl SpirvSymbolTable {
    pub fn new() -> SpirvSymbolTable {
        SpirvSymbolTable {
            scopes: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol_name: &str, symbol_type: Word) {
        self.scopes[0].symbols.insert(symbol_name.to_string(), symbol_type);
    }

    pub fn find_symbol(&mut self, name: &str) -> Option<&Word> {
        for scope in &self.scopes {
            if scope.symbols.contains_key(name) {
                return scope.symbols.get(name);
            }
        }

        None
    }

    pub fn enter_scope(&mut self) {
        self.scopes.insert(0, Scope::new());
    }

    pub fn leave_scope(&mut self) {
        self.scopes.remove(0);
    }
}
