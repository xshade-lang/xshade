use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;
use ::type_system::structure_members::{ StructureMembers, StructureMember };

pub struct CheckStructMemberPass {
    symbol_table: SymbolTableReference,
    result: PassResultReference,
    member_list: Option<Vec<StructureMember>>,
}

impl CheckStructMemberPass {
    pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> CheckStructMemberPass {
        CheckStructMemberPass {
            symbol_table: symbol_table,
            result: result,
            member_list: None,
        }
    }
}

ast_pass_impl!(CheckStructMemberPass, {
    fn visit_struct(&mut self, struct_definition: &mut StructDefinition) {
        self.member_list = Some(Vec::new());

        self.walk_struct(struct_definition);

        let member_list = self.member_list.take().unwrap();
        match symbol_table_mut!(self).find_type_mut(struct_definition.declaring_type.unwrap()) {
            Some(ref mut x) => pass_try!(self, x.set_members(StructureMembers::new(member_list))),
            None => panic!("test"),
        }
    }

    fn visit_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition) {
        let mut list = self.member_list.take().unwrap();
        let struct_member_type = pass_try!(self, symbol_table!(self).find_type_ref_or_err(&struct_member_definition.struct_member_type_name.name));
        struct_member_definition.struct_member_type = Some(struct_member_type);
        list.push(StructureMember::new(struct_member_definition.struct_member_name.name.clone(), struct_member_type));
        self.member_list = Some(list);
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
    fn it_works() {
        let mut ast = compile_ast("struct Foo { bar: bool, }");
        let mut symbol_table = SymbolTable::new(TypeEnvironment::new());
        symbol_table.create_global_type("bool").unwrap();
        let symbol_table = SymbolTableReference::new(symbol_table);
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), result.clone())),
            Box::new(CheckStructMemberPass::new(symbol_table.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        assert_eq!(symbol_table.borrow().find_type_by_name("Foo").unwrap().get_member().unwrap(), &StructureMembers::new(vec![
            StructureMember::new("bar".to_string(), TypeReference::new(0)),
        ]));
    }
}
