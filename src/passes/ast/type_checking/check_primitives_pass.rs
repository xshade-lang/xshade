use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;

ast_pass!(CheckPrimitivesPass, {
    fn visit_primitive(&mut self, primitive_declaration: &mut PrimitiveDeclaration) {
        // pass_warning!(self, "'primitive' is experimental syntax and might get changed or removed in the future.");

        // let type_ref = pass_try!(self, symbol_table_mut!(self).create_global_type(&primitive_declaration.type_name.name));

        // primitive_declaration.declaring_type = Some(type_ref);
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
        let result = PassResultReference::new(PassResult::new());
        let mut pass = CheckPrimitivesPass::new(symbol_table.clone(), result.clone());

        pass.execute(&mut ast);

        // TODO assert via type environment
        // assert!(symbol_table.borrow().find_type_by_name("bool").is_some());
    }

    #[test]
    fn duplicate_declarations_produce_an_error() {
        let mut ast = compile_ast("primitive type bool; primitive type bool;");
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let result = PassResultReference::new(PassResult::new());
        let mut pass = CheckPrimitivesPass::new(symbol_table.clone(), result.clone());

        pass.execute(&mut ast);

        // TODO assert via type environment
        // assert!(result.borrow().has_errors());
    }
}
