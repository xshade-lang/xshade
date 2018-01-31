use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::type_system::call_signature::CallSignature;

pub struct CheckFunctionSignaturePass {
    symbol_table: SymbolTableReference,
    result: PassResultReference,
    argument_type_list: Option<Vec<TypeReference>>,
}

impl CheckFunctionSignaturePass {
    pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> CheckFunctionSignaturePass {
        CheckFunctionSignaturePass {
            symbol_table: symbol_table,
            result: result,
            argument_type_list: None,
        }
    }
}

ast_pass_impl!(CheckFunctionSignaturePass, {
    fn visit_function(&mut self, function_declaration: &mut FunctionDeclaration) {
        // let mut symbol_table = symbol_table_mut!(self);

        let function_type = pass_try!(self, symbol_table_mut!(self).create_type(&function_declaration.function_name.name));
        pass_try!(self, symbol_table_mut!(self).add_symbol_with_type(&function_declaration.function_name.name, function_type));

        symbol_table_mut!(self).enter_scope();

        let type_ref = pass_try!(self, symbol_table!(self).find_type_ref_or_err(&function_declaration.return_type_name.name));
        function_declaration.return_type    = Some(type_ref.clone());
        function_declaration.declaring_type = Some(function_type);

        self.argument_type_list = Some(Vec::new());
        for argument in function_declaration.arguments.iter_mut() {
            self.visit_function_argument(argument);
        }
        
        let argument_list = self.argument_type_list.take().unwrap();
        
        let signature = CallSignature::new(argument_list, Some(type_ref));
        pass_try!(self, pass_try!(self, symbol_table_mut!(self).find_type_mut_or_err(function_type)).make_callable(signature));

        symbol_table_mut!(self).leave_scope();
    }

    fn visit_function_argument(&mut self, function_argument_declaration: &mut FunctionArgumentDeclaration) {
        let mut list = self.argument_type_list.take().unwrap();

        let type_ref = match symbol_table!(self).find_type_ref(&function_argument_declaration.argument_type_name.name) {
            Some(t) => t,
            None => pass_try!(self, Err(TypeError::new(function_argument_declaration.argument_type_name.get_span(), ErrorKind::TypeNotFound(function_argument_declaration.argument_type_name.name.to_owned())))),
        };
        function_argument_declaration.argument_type = Some(type_ref);
        pass_try!(self, symbol_table_mut!(self).add_symbol_with_type(&function_argument_declaration.argument_name.name, type_ref));
        list.push(type_ref);

        self.argument_type_list = Some(list);
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

    #[test]
    fn function_no_param_empty_body() {
        let mut ast = compile_ast("fn test() -> void {}");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("void").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        assert!(!result.borrow().has_errors());
        assert!(is_some(symbol_table.borrow().find_type_by_name("test")));
    }

    fn is_some<T>(optional: Option<T>) -> bool {
        return match optional {
            Some(_) => true,
            None    => false
        };
    }

    #[test]
    fn function_one_param_return_body() {
        let mut ast = compile_ast("struct Test { position: i32, } fn test(p0: i32) -> Test { return Test { position: 0, }; }");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("void").unwrap();
        symbol_table.create_global_type("i32").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckFunctionSignaturePass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        assert!(!result.borrow().has_errors());
        assert!(is_some(symbol_table.borrow().find_type_by_name("test")));
        assert!(is_some(symbol_table.borrow().find_type_by_name("i32")));
        assert!(is_some(symbol_table.borrow().find_type_by_name("Test")));
        assert!(symbol_table.borrow().find_type_by_name("test").unwrap().is_callable());
        assert!(is_some(symbol_table.borrow().find_type_by_name("test").unwrap().get_call_signature()));
        
        let i32TypeRef  = symbol_table.borrow().find_type_ref("i32").unwrap();
        let TestTypeRef = symbol_table.borrow().find_type_ref("Test").unwrap();

        assert!(symbol_table.borrow().find_type_by_name("test").unwrap().get_call_signature().unwrap().match_arguments(vec![i32TypeRef]));
        assert!(symbol_table.borrow().find_type_by_name("test").unwrap().get_call_signature().unwrap().match_return_type(Some(TestTypeRef)));
    }
}
