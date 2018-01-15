use ::rspirv::mr;
use ::rspirv::binary::Assemble;
use ::spirv;
use ::spirv::Word;
use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeReference;
use ::passes::ast::spirv_symbol_table::SpirvSymbolTable;

/// AST pass to generate spir-v bytecode
pub struct GenerateSpirvPass {
    builder: mr::Builder,
    symbol_table: SymbolTable,
    spirv_symbol_table: SpirvSymbolTable,

    last_expression: Option<Word>,
}

impl GenerateSpirvPass {
    pub fn new(symbol_table: SymbolTable) -> GenerateSpirvPass {
        let mut builder = mr::Builder::new();

        builder.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

        GenerateSpirvPass {
            builder: builder,
            symbol_table: symbol_table,
            spirv_symbol_table: SpirvSymbolTable::new(),
            last_expression: None,
        }
    }

    pub fn assemble(self) -> Vec<u32> {
        let module = self.builder.module();
        let code = module.assemble();
        code
    }

    fn get_spirv_type(&mut self, type_ref: Option<TypeReference>) -> spirv::Word {
        match type_ref {
            Some(t) => match self.symbol_table.find_type(t) {
                Some(type_definition) => match type_definition.get_name() {
                    "bool" => self.builder.type_bool(),
                    "f32" => self.builder.type_float(32),
                    "f64" => self.builder.type_float(64),
                    "i32" => self.builder.type_int(32, 0),
                    "i64" => self.builder.type_int(64, 0),
                    _ => self.builder.type_void(),
                }
                None => self.builder.type_void(),
            }
            None => self.builder.type_void(),
        }
    }
}

impl AstWalker for GenerateSpirvPass {
    fn visit_function(&mut self, function_definition: &mut FunctionDeclaration) -> PassResult {
        self.spirv_symbol_table.enter_scope();
        let return_type = self.get_spirv_type(function_definition.return_type);
        let argument_types = if function_definition.arguments.len() > 0 {
            function_definition.arguments.iter().map(|ref a| self.get_spirv_type(a.argument_type)).collect()
        } else {
            vec![self.get_spirv_type(None)]
        };

        let function_type = self.builder.type_function(return_type, argument_types);

        self.builder.begin_function(
            return_type,
            None,
            (spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST),
            function_type);
        
        self.walk_function(function_definition);
        self.builder.end_function().unwrap();
        self.spirv_symbol_table.leave_scope();
        Ok(())
    }

    fn visit_function_argument(&mut self, function_argument: &mut FunctionArgumentDeclaration) -> PassResult {
        let argument_type = self.get_spirv_type(function_argument.argument_type);
        let argument = self.builder.function_parameter(argument_type).unwrap();
        self.spirv_symbol_table.add_symbol(&function_argument.argument_name.name, argument);
        Ok(())
    }

    fn visit_block(&mut self, block: &mut BlockDeclaration) -> PassResult {
        self.builder.begin_basic_block(None).unwrap();
        self.walk_block(block)
    }

    fn visit_variable_expression(&mut self, variable_expression: &mut VariableExpression) -> PassResult {
        self.last_expression = Some(self.spirv_symbol_table.find_symbol(&variable_expression.variable_name.name).unwrap().clone());
        Ok(())
    }

    fn visit_infix_expression(&mut self, infix_expression: &mut InfixExpression) -> PassResult {
        self.walk_infix_expression_left(infix_expression);
        let left = self.last_expression.take().unwrap();

        self.walk_infix_expression_right(infix_expression);
        let right = self.last_expression.take().unwrap();

        let result_type = self.get_spirv_type(infix_expression.infix_type);

        self.last_expression = Some(self.builder.fmul(result_type, None, left, right).unwrap());
        Ok(())
    }

    fn visit_return_statement(&mut self, return_statement: &mut ReturnDeclaration) -> PassResult {
        self.walk_return_statement(return_statement);
        let ret_value = self.last_expression.take().unwrap();
        self.builder.ret_value(ret_value).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::testing::compile;
    use ::parser::parse_str;
    use ::rspirv;
    use ::rspirv::binary::Disassemble;

    #[test]
    pub fn it_works() {
        let mut compilation = compile("fn main(a: f32, b: f32) -> f32 { return a * b; }");
        let mut pass = GenerateSpirvPass::new(compilation.get_symbol_table());

        compilation.run_ast_pass(&mut pass);
        
        let mut code = pass.assemble();
        let mut loader = mr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).unwrap();
        let module = loader.module();

        println!("");
        println!("{:#?}", compilation.get_ast_mut());
        println!("");
        println!("{}", module.disassemble());
        println!("");
        panic!("");
    }    
}
