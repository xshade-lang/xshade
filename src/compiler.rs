use ::std::error::Error;
use ::compile_error::{ CompileError, CompileResult, ErrorKind };
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_check::type_check;
use ::type_system::type_environment::TypeEnvironment;
use ::ast::Span;

fn parse_core_modules(symbols: &mut SymbolTable) -> Result<Module, Box<Error>> {
    let primitives = include_str!("../libcore/primitives.xs");
    let ast = parse_str(primitives)?;
    let mut module = Module::new("".to_owned(), primitives.to_owned(), ast, true);
    type_check(symbols, &mut module)?;
    Ok(module)
}

pub trait ModuleResolver {
    fn resolve(&mut self, module_path: &str) -> Result<String, Box<Error>>;
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

    pub fn compile_module(&mut self, module_path: &str) -> CompileResult<Module> {
        let source = match self.resolver.resolve(module_path) {
            Ok(source) => source,
            Err(_) => return Err(CompileError::unknown()),
        };

        let ast = parse_str(&source)?;

        let mut symbols = SymbolTable::new(TypeEnvironment::new());
        parse_core_modules(&mut symbols).unwrap();

        let mut module = Module::new(module_path.to_owned(), source, ast, false);

        match type_check(&mut symbols, &mut module) {
            Ok(_) => Ok(module),
            Err(e) => {
                let span = e.get_span();
                module.set_error(CompileError::new(ErrorKind::TypeError(e), span));
                Ok(module)
            },
        }
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

        let module = compiler.compile_module("test").unwrap();

        assert!(compiler.compile_module("test").is_ok());
    }

}
