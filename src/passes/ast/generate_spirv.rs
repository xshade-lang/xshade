use ::rspirv::mr;
use ::rspirv::binary::Assemble;
use ::spirv;
use ::ast::*;
use ::passes::*;
use ::passes::ast::*;

pub struct GenerateSpirvPass {
    builder: mr::Builder,
}

impl GenerateSpirvPass {
    pub fn new() -> GenerateSpirvPass {
        let mut builder = mr::Builder::new();

        builder.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

        GenerateSpirvPass {
            builder: builder,
        }
    }

    pub fn assemble(self) -> Vec<u32> {
        let module = self.builder.module();
        let code = module.assemble();
        code
    }
}

impl AstWalker for GenerateSpirvPass {
    fn visit_function(&mut self, function_definition: &mut FunctionDeclaration) -> PassResult {
        let void = self.builder.type_void();
        let voidf = self.builder.type_function(void, vec![void]);

        self.builder.begin_function(
            void,
            None,
            (spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST),
            voidf);
        self.walk_function(function_definition);
        self.builder.end_function().unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::parser::parse_str;
    use ::rspirv;
    use ::rspirv::binary::Disassemble;

    #[test]
    pub fn it_works() {
        let mut ast = parse_str("fn main() -> void {  }").unwrap();
        let mut pass = GenerateSpirvPass::new();

        pass.visit(&mut ast).unwrap();
        
        let mut code = pass.assemble();
        let mut loader = mr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).unwrap();
        let module = loader.module();
        println!("");
        print!("{}", module.disassemble());
        println!("");
        panic!("");
    }    
}
