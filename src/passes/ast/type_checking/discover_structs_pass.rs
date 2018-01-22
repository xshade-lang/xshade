use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;

ast_pass!(DiscoverStructsPass, {
    fn visit_struct(&mut self, struct_definition: &mut StructDefinition) {
        let type_ref = pass_try!(self, symbol_table_mut!(self).create_type(&struct_definition.struct_name.name));
        pass_try!(self, symbol_table_mut!(self).add_symbol_with_type(&struct_definition.struct_name.name, type_ref));
        struct_definition.declaring_type = Some(type_ref);
        self.walk_struct(struct_definition);
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
        let mut ast = compile_ast("struct Foo {}");
        let symbol_table = SymbolTableReference::new(SymbolTable::new(TypeEnvironment::new()));
        let result = PassResultReference::new(PassResult::new());
        let mut pass = DiscoverStructsPass::new(symbol_table.clone(), result.clone());

        pass.execute(&mut ast);

        assert!(symbol_table.borrow().find_type_by_name("Foo").is_some());
    }
}
