use ::std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SymbolState {
    Bound,
    Free,
    Typed(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub state: SymbolState,
}

impl Symbol {
    pub fn new(name: &str, state: SymbolState) -> Symbol {
        Symbol {
            name: name.to_string(),
            state: state,
        }
    }
}

#[derive(Debug)]
struct Scope {
    symbols: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scopes: vec![Scope{ symbols: HashMap::new(), }],
        }
    }

    pub fn find_symbol(&mut self, name: &str) -> Option<&Symbol> {
        for scope in &self.scopes {
            if scope.symbols.contains_key(name) {
                return scope.symbols.get(name);
            }
        }

        None
    }

    pub fn add_symbol(&mut self, name: &str) -> Result<(), ()> {
        if self.scopes[0].symbols.contains_key(name) {
            return Err(());
        }

        self.scopes[0].symbols.insert(name.to_string(), Symbol::new(name, SymbolState::Free));
        Ok(())
    }

    pub fn enter_scope(&mut self) {
        self.scopes.insert(0, Scope {
            symbols: HashMap::new(),
        });
    }

    pub fn leave_scope(&mut self) {
        self.scopes.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_find_symbol() {
        let mut symbols = SymbolTable::new();

        symbols.add_symbol("test_symbol");

        assert_eq!(symbols.find_symbol("test_symbol"), Some(&Symbol::new("test_symbol", SymbolState::Free)));
    }

    #[test]
    fn enter_and_leave_scope() {
        let mut symbols = SymbolTable::new();

        symbols.enter_scope();
        symbols.leave_scope();
    }

    #[test]
    fn add_enter_then_find_symbol() {
        let mut symbols = SymbolTable::new();
        symbols.add_symbol("test_symbol");
        symbols.enter_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), Some(&Symbol::new("test_symbol", SymbolState::Free)));
    }

    #[test]
    fn enter_add_leave_then_dont_find_symbol() {
        let mut symbols = SymbolTable::new();
        symbols.enter_scope();
        symbols.add_symbol("test_symbol");
        symbols.leave_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), None);
    }

    #[test]
    fn cannot_leave_root_scope() {
        let mut symbols = SymbolTable::new();
        symbols.leave_scope();
    }
}
