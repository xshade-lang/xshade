use ::std::error::Error;
use ::std::mem;
use ::std::collections::HashMap;
use ::compile_error::{ CompileError, CompileResult, ErrorKind };
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;
use ::ast::{ ItemKind, Span };
use ::passes::ast::AstWalker;

fn parse_core_modules(symbols: &mut SymbolTable) -> Result<Module, Box<Error>> {
    let primitives = include_str!("../libcore/primitives.xs");
    let ast = parse_str(primitives)?;
    let mut module = Module::new("".to_owned(), primitives.to_owned(), ast, true);
    Ok(module)
}

pub trait ModuleResolver {
    fn resolve(&mut self, module_path: &str) -> Result<String, Box<Error>>;
}

pub struct Compilation {
    symbol_table: Option<SymbolTable>,
    module: Module
}

impl Compilation {
    fn new(symbol_table: SymbolTable, module: Module) -> Compilation {
        Compilation {
            symbol_table: Some(symbol_table),
            module: module,
        }
    }

    pub fn get_ast_mut(&mut self) -> &mut Vec<ItemKind> {
        self.module.get_ast_mut()
    }
}

pub struct Compiler {
    resolver: Box<ModuleResolver>,
}

impl Compiler {
    
    pub fn new(resolver: Box<ModuleResolver>) -> Compiler {
        Compiler {
            resolver: resolver,
        }
    }

    pub fn compile_module(&mut self, module_path: &str) -> CompileResult<Compilation> {
        let source = match self.resolver.resolve(module_path) {
            Ok(source) => source,
            Err(_) => return Err(CompileError::unknown()),
        };

        let ast = parse_str(&source)?;

        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        parse_core_modules(&mut symbol_table).unwrap();

        let mut modules = HashMap::new();
        self.load_modules(module_path, &mut modules)?;

        let mut module = Module::new(module_path.to_owned(), source, ast, false);

        // TODO insert pass system here

        Ok(Compilation::new(symbol_table, module))
    }

    fn load_modules(&mut self, module_path: &str, modules: &mut HashMap<String, Module>) -> CompileResult<()> {
        let source = match self.resolver.resolve(module_path) {
            Ok(source) => source,
            Err(_) => return Err(CompileError::unknown()),
        };

        let ast = parse_str(&source)?;
        let module = Module::new(module_path.to_owned(), source, ast, false);

        let imports: Vec<String> = module.find_imports().iter().map(|&i| i.module_id.to_owned()).collect();
        modules.insert(module_path.to_owned(), module);

        for import in imports {
            if modules.contains_key(&import) {
                continue;
            }

            self.load_modules(&import, modules)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    struct TestResolver {
        map: HashMap<String, String>,
    }
    impl TestResolver {
        pub fn new(map: HashMap<String, String>) -> TestResolver {
            TestResolver {
                map: map,
            }
        }
    }
    impl ModuleResolver for TestResolver {
        fn resolve(&mut self, module_path: &str) -> Result<String, Box<Error>> {
            match self.map.get(module_path) {
                Some(s) => Ok(s.to_string()),
                None => Err(Box::new(CompileError::unknown())),
            }
        }
    }

    #[test]
    fn test_create_compiler() {
        let resolver = Box::new(TestResolver::new(HashMap::new()));
        let compiler = Compiler::new(resolver);
    }

    #[test]
    fn test_compile_module() {
        let mut map = HashMap::new();
        map.insert("test".to_string(), "struct Test {}".to_string());
        let resolver = Box::new(TestResolver::new(map));
        let mut compiler = Compiler::new(resolver);

        // TODO assert that two modules were loaded
        assert!(compiler.compile_module("test").is_ok());
    }

    #[test]
    fn test_compile_multiple_modules() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), "import Test from 'b';".to_string());
        map.insert("b".to_string(), "struct Test {}".to_string());
        let resolver = Box::new(TestResolver::new(map));
        let mut compiler = Compiler::new(resolver);

        assert!(compiler.compile_module("a").is_ok());
    }
}
