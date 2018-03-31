use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::SymbolTableReference;
use ::type_system::type_environment::TypeEnvironmentReference;
use ::type_system::type_environment::TypeReference;
use ::passes::ast::type_checking::error::TypeWithSameNameAlreadyDefinedError;

ast_pass!(CheckPrimitivesPass, {
    fn visit_primitive(&mut self, primitive_declaration: &mut PrimitiveDeclaration) {
        let type_ref = pass_try!(self, match type_environment_mut!(self).create_type(&primitive_declaration.type_name.name) {
            Some(t) => Ok(t),
            None => Err(TypeWithSameNameAlreadyDefinedError::new(primitive_declaration.span, primitive_declaration.type_name.name.to_string())),
        });

        // TODO handle result, add test
        symbol_table_mut!(self).add_type(&primitive_declaration.type_name.name, type_ref);

        primitive_declaration.declaring_type = Some(type_ref);
    }
});

#[cfg(test)]
mod tests {
    use super::*;
    use ::testing::compile_ast;
    use ::passes::results::PassResult;
    use ::type_system::symbol_table::SymbolTable;
    use ::type_system::type_environment::TypeEnvironment;

    #[test]
    fn it_works() {
        let mut ast = compile_ast("primitive type bool;");
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let type_environment = TypeEnvironmentReference::new(TypeEnvironment::new());
        let result = PassResultReference::new(PassResult::new());
        let mut pass = CheckPrimitivesPass::new(symbol_table.clone(), type_environment.clone(), result.clone());

        pass.execute(&mut ast);

        assert!(type_environment.borrow().find_reference_by_name("bool").is_some());
    }

    #[test]
    fn duplicate_declarations_produce_an_error() {
        let mut ast = compile_ast("primitive type bool; primitive type bool;");
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let type_environment = TypeEnvironmentReference::new(TypeEnvironment::new());
        let result = PassResultReference::new(PassResult::new());
        let mut pass = CheckPrimitivesPass::new(symbol_table.clone(), type_environment.clone(), result.clone());

        pass.execute(&mut ast);

        assert!(result.borrow().has_errors());
    }
}
