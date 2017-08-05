use ::std::collections::HashMap;
use ::type_system::error::{ TypeError, TypeCheckResult };
use ::type_system::type_environment::TypeReference;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SymbolState {
    Bound,
    Free,
    Typed(TypeReference),
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

    pub fn resolve_type(&mut self, type_reference: TypeReference) {
        match self.state {
            SymbolState::Bound => self.state = SymbolState::Typed(type_reference),
            SymbolState::Free => self.state = SymbolState::Typed(type_reference),
            SymbolState::Typed(_) => panic!("Symbol already has a type!"),
        }
    }
}

#[derive(Debug)]
struct Scope {
    symbols: HashMap<String, Symbol>,
    types: HashMap<String, TypeReference>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            symbols: HashMap::new(),
            types: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scopes: vec![Scope::new()],
        }
    }

    pub fn add_type(&mut self, name: &str, type_reference: TypeReference) -> TypeCheckResult<()> {
        if self.scopes[0].types.contains_key(name) {
            return Err(TypeError::SymbolNameAlreadyUsed(name.to_string()));
        }

        self.scopes[0].types.insert(name.to_string(), type_reference);
        Ok(())
    }

    pub fn find_type(&self, name: &str) -> Option<&TypeReference> {
        for scope in &self.scopes {
            if scope.types.contains_key(name) {
                return scope.types.get(name);
            }
        }

        None
    }

    pub fn add_symbol(&mut self, name: &str) -> TypeCheckResult<()> {
        if self.scopes[0].symbols.contains_key(name) {
            return Err(TypeError::SymbolNameAlreadyUsed(name.to_string()));
        }

        self.scopes[0].symbols.insert(name.to_string(), Symbol::new(name, SymbolState::Free));
        Ok(())
    }

    pub fn find_symbol(&mut self, name: &str) -> Option<&Symbol> {
        for scope in &self.scopes {
            if scope.symbols.contains_key(name) {
                return scope.symbols.get(name);
            }
        }

        None
    }

    pub fn find_symbol_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for scope in &mut self.scopes {
            if scope.symbols.contains_key(name) {
                return scope.symbols.get_mut(name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_find_symbol() {
        let mut symbols = SymbolTable::new();

        symbols.add_symbol("test_symbol").unwrap();

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
        symbols.add_symbol("test_symbol").unwrap();
        symbols.enter_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), Some(&Symbol::new("test_symbol", SymbolState::Free)));
    }

    #[test]
    fn enter_add_leave_then_dont_find_symbol() {
        let mut symbols = SymbolTable::new();
        symbols.enter_scope();
        symbols.add_symbol("test_symbol").unwrap();
        symbols.leave_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), None);
    }

    #[test]
    fn cannot_leave_root_scope() {
        let mut symbols = SymbolTable::new();
        symbols.leave_scope();
    }

    #[test]
    fn add_type() {
        let reference = TypeReference::new(0);
        let mut symbols = SymbolTable::new();
        symbols.add_type("f32", reference);
    }
}
