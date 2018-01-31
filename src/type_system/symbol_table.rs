use ::std::collections::HashMap;
use ::std::rc::Rc;
use ::std::cell::{ RefCell, Ref, RefMut };
use ::ast::Span;
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::type_system::type_definition::TypeDefinition;
use ::type_system::type_environment::{ TypeEnvironment, TypeReference };
use ::data_structures::shared::Shared;

pub type SymbolTableReference = Shared<SymbolTable>;

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

    pub fn get_type(&self) -> Option<TypeReference> {
        match self.state {
            SymbolState::Typed(t) => Some(t.clone()),
            _ => None
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
    types: TypeEnvironment,
}

impl SymbolTable {
    pub fn new(types: TypeEnvironment) -> SymbolTable {
        SymbolTable {
            scopes: vec![Scope::new()],
            types: types,
        }
    }

    pub fn add_global_type(&mut self, name: &str, type_reference: TypeReference) -> TypeCheckResult<()> {
        let root = self.scopes.len() - 1;
        if self.scopes[root].types.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_string())));
        }

        self.scopes[root].types.insert(name.to_string(), type_reference);
        Ok(())
    }

    pub fn add_type(&mut self, name: &str, type_reference: TypeReference) -> TypeCheckResult<()> {
        if self.scopes[0].types.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_owned())));
        }

        self.scopes[0].types.insert(name.to_string(), type_reference);
        Ok(())
    }

    pub fn create_type(&mut self, name: &str) -> TypeCheckResult<TypeReference> {
        if self.scopes[0].types.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_owned())));
        }

        let type_ref = try!(self.types.create_type(name));

        self.scopes[0].types.insert(name.to_string(), type_ref);
        Ok(type_ref)
    }

    pub fn create_global_type(&mut self, name: &str) -> TypeCheckResult<TypeReference> {
        let root = self.scopes.len() - 1;
        if self.scopes[root].types.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_owned())));
        }

        let type_ref = try!(self.types.create_type(name));

        self.scopes[root].types.insert(name.to_string(), type_ref);
        Ok(type_ref)
    }

    pub fn find_type(&self, type_ref: TypeReference) -> Option<&TypeDefinition> {
        self.types.find_type(type_ref)
    }

    pub fn find_type_by_name(&self, type_name: &str) -> Option<&TypeDefinition> {
        self.types.find_type_by_name(type_name)
    }

    pub fn find_type_or_err(&self, type_ref: TypeReference) -> TypeCheckResult<&TypeDefinition> {
        self.types.find_type_or_err(type_ref)
    }

    pub fn find_type_mut(&mut self, type_ref: TypeReference) -> Option<&mut TypeDefinition> {
        self.types.find_type_mut(type_ref)
    }

    pub fn find_type_mut_or_err(&mut self, type_ref: TypeReference) -> TypeCheckResult<&mut TypeDefinition> {
        self.types.find_type_mut_or_err(type_ref)
    }

    pub fn find_type_ref(&self, name: &str) -> Option<TypeReference> {
        for scope in &self.scopes {
            if scope.types.contains_key(name) {
                match scope.types.get(name) {
                    Some(t) => return Some(t.clone()),
                    None => return None,
                }
            }
        }

        None
    }

    pub fn find_type_ref_or_err(&self, name: &str) -> TypeCheckResult<TypeReference> {
        for scope in &self.scopes {
            if scope.types.contains_key(name) {
                match scope.types.get(name) {
                    Some(t) => return Ok(t.clone()),
                    None => return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::TypeNotFound(name.to_owned()))),
                }
            }
        }

        Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::TypeNotFound(name.to_owned())))
    }

    pub fn add_symbol(&mut self, name: &str) -> TypeCheckResult<()> {
        if self.scopes[0].symbols.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_owned())));
        }

        self.scopes[0].symbols.insert(name.to_string(), Symbol::new(name, SymbolState::Free));
        Ok(())
    }

    pub fn add_symbol_with_type(&mut self, name: &str, symbol_type: TypeReference) -> TypeCheckResult<()> {
        if self.scopes[0].symbols.contains_key(name) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SymbolNameAlreadyUsed(name.to_owned())));
        }

        self.scopes[0].symbols.insert(name.to_string(), Symbol::new(name, SymbolState::Typed(symbol_type)));
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

    pub fn resolve_symbol_type(&mut self, name: &str, symbol_type: TypeReference) -> TypeCheckResult<()> {
        for scope in &mut self.scopes {
            if scope.symbols.contains_key(name) {
                match scope.symbols.get_mut(name) {
                    Some(ref mut s) => s.resolve_type(symbol_type),
                    None => return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::VariableNotFound(name.to_owned()))),
                }
                return Ok(());
            }
        }

        Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::VariableNotFound(name.to_owned())))
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
        let mut symbols = SymbolTable::new(TypeEnvironment::new());

        symbols.add_symbol("test_symbol").unwrap();

        assert_eq!(symbols.find_symbol("test_symbol"), Some(&Symbol::new("test_symbol", SymbolState::Free)));
    }

    #[test]
    fn enter_and_leave_scope() {
        let mut symbols = SymbolTable::new(TypeEnvironment::new());

        symbols.enter_scope();
        symbols.leave_scope();
    }

    #[test]
    fn add_enter_then_find_symbol() {
        let mut symbols = SymbolTable::new(TypeEnvironment::new());
        symbols.add_symbol("test_symbol").unwrap();
        symbols.enter_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), Some(&Symbol::new("test_symbol", SymbolState::Free)));
    }

    #[test]
    fn enter_add_leave_then_dont_find_symbol() {
        let mut symbols = SymbolTable::new(TypeEnvironment::new());
        symbols.enter_scope();
        symbols.add_symbol("test_symbol").unwrap();
        symbols.leave_scope();

        assert_eq!(symbols.find_symbol("test_symbol"), None);
    }

    #[test]
    fn cannot_leave_root_scope() {
        let mut symbols = SymbolTable::new(TypeEnvironment::new());
        symbols.leave_scope();
    }

    #[test]
    fn add_type() {
        let reference = TypeReference::new(0);
        let mut symbols = SymbolTable::new(TypeEnvironment::new());
        symbols.add_type("f32", reference).unwrap();
    }
}
