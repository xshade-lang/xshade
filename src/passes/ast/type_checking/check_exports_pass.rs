use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::passes::ast::type_checking::error;

ast_pass!(CheckExportsPass, {
    fn visit_export(&mut self, export_definition: &mut ExportDefinition) {
        pass_err!(self, Box::new(error::ExperimentalSyntaxWarning::new(export_definition.span, "export")));

        let mut symbol_table_ref = symbol_table_mut!(self); 

        for i in &export_definition.items {
            let type_name = match i {
                &ImportItem::Named(ref identifier) => &*identifier.name,
                _ => continue,
            };

            let type_ref = match symbol_table_ref.find_type_ref(type_name) {
                Some(t) => t,
                None => pass_try!(self, Err(TypeError::new(Span::empty(), ErrorKind::TypeNotFound(type_name.to_owned())))),
            };
            match symbol_table_ref.find_type_mut(type_ref) {
                Some(t) => {
                    if !(t.is_struct() || t.is_callable()) { 
                        pass_try!(self, Err(TypeError::new(Span::empty(), ErrorKind::InvalidExport(type_name.to_owned()))))
                    } else {
                        continue;
                    }
                },
                None => pass_try!(self, Err(TypeError::new(Span::empty(), ErrorKind::TypeNotFound(type_name.to_owned())))),
            };
            
        }
    }
});

#[cfg(test)]
mod tests {
    use super::*;
    use ::testing::compile_ast;
    use ::passes::results::PassResult;
    use ::type_system::symbol_table::SymbolTable;
    use ::type_system::type_environment::TypeEnvironment;
    use ::passes::ast::type_checking::check_primitives_pass;
    use ::passes::ast::type_checking::discover_structs_pass;
    use ::passes::ast::type_checking::check_function_signatures_pass;
    use ::passes::ast::type_checking::check_struct_member_pass;

    #[test]
    fn check_all_exports() {
        let mut ast = compile_ast("struct Test { position: vec4, } export *;");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("vec4").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());
        
        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckExportsPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        // assert!(!result.borrow().has_errors());
    }

    #[test]
    fn check_single_export_struct() {
        let mut ast = compile_ast("struct Test { position: vec4, } export Test;");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("vec4").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckExportsPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        // assert!(!result.borrow().has_errors());
    }

    #[test]
    fn check_single_export_fn() {
        let mut ast = compile_ast("fn TestFn() -> i32 {  return 0; } export TestFn;");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("i32").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckExportsPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        // assert!(!result.borrow().has_errors());
    }
    
    #[test]
    fn check_multiple_export_item() {
        let mut ast = compile_ast("struct Test { position: vec4, } struct Another { position: vec4, size: i32, } export { Test, Another };");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("vec4").unwrap();
        symbol_table.create_global_type("i32").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());
        
        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckExportsPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        // assert!(!result.borrow().has_errors());
    }

    #[test]
    fn check_multiple_export_item_with_fn() {
        let mut ast = compile_ast("struct Test { position: vec4, } fn TestFn() -> i32 { return 0; } export { Test, TestFn };");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("vec4").unwrap();
        symbol_table.create_global_type("i32").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());
        
        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_struct_member_pass::CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
            Box::new(check_function_signatures_pass::CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckExportsPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        // assert!(!result.borrow().has_errors());
    }
}
