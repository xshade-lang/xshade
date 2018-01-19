use ::ast::Ast;
use ::std::collections::HashMap;
use ::std::error::Error;
use ::compile_error::CompileError;
use ::compiler::{ Compiler, Compilation, ModuleResolver };

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

pub fn compile(code_to_compile: &str) -> Compilation {
    let mut map = HashMap::new();
    map.insert("test".to_string(), code_to_compile.to_string());
    let resolver = Box::new(TestResolver::new(map));
    let mut compiler = Compiler::new(resolver);
    compiler.compile_module("test").unwrap()
}

pub fn compile_ast(code_to_compile: &str) -> Ast {
    ::parser::parse_str(code_to_compile).unwrap()
}
