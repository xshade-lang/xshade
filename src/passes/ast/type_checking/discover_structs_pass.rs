use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::SymbolTableReference;
use ::type_system::type_environment::TypeEnvironmentReference;
use ::type_system::type_environment::TypeReference;
use ::passes::ast::type_checking::error::TypeWithSameNameAlreadyDefinedError;

ast_pass!(DiscoverStructsPass, {
    fn visit_struct(&mut self, struct_definition: &mut StructDefinition) {
        // TODO use new TypeWithSameNameAlreadyDefined error
        // let type_ref = pass_try!(self, symbol_table_mut!(self).create_type(&struct_definition.struct_name.name));
        // pass_try!(self, symbol_table_mut!(self).add_symbol_with_type(&struct_definition.struct_name.name, type_ref));
        // struct_definition.declaring_type = Some(type_ref);
        // self.walk_struct(struct_definition);

        let type_ref = pass_try!(self, match type_environment_mut!(self).create_type(&struct_definition.struct_name.name) {
            Some(t) => Ok(t),
            None => Err(TypeWithSameNameAlreadyDefinedError::new(struct_definition.span, struct_definition.struct_name.name.to_string())),
        });

        pass_try!(self, match symbol_table_mut!(self).add_symbol_with_type(&struct_definition.struct_name.name, type_ref) {
            None => Err(TypeWithSameNameAlreadyDefinedError::new(struct_definition.span, struct_definition.struct_name.name.to_string())),
            Some(()) => Ok(()),
        });

        struct_definition.declaring_type = Some(type_ref);
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
    fn it_type_checks_a_struct() {
        let mut ast = compile_ast("struct Foo {}");
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let type_environment = TypeEnvironmentReference::new(TypeEnvironment::new());
        let result = PassResultReference::new(PassResult::new());
        let mut pass = DiscoverStructsPass::new(symbol_table.clone(), type_environment.clone(), result.clone());

        pass.execute(&mut ast);

        assert!(type_environment.borrow().find_reference_by_name("Foo").is_some());
    }

    #[test]
    fn it_errors_on_duplicate_structs() {
        let mut ast = compile_ast("struct Foo {} struct Foo {}");
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let type_environment = TypeEnvironmentReference::new(TypeEnvironment::new());
        let result = PassResultReference::new(PassResult::new());
        let mut pass = DiscoverStructsPass::new(symbol_table.clone(), type_environment.clone(), result.clone());

        pass.execute(&mut ast);

        assert!(result.borrow().has_errors());
    }
}
