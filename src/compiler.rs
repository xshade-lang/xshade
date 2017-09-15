use ::compile_error::{ CompileError, CompileResult };

pub struct Compiler;

impl Compiler {
    
    pub fn new() -> Compiler {
        Compiler
    }

    pub fn compile_module(&mut self, module_location: &str) -> CompileResult<()> {
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_compiler() {
        let compiler = Compiler::new();
    }

    #[test]
    fn test_compile_module() {
        let mut compiler = Compiler::new();

        compiler.compile_module("");
    }

}
