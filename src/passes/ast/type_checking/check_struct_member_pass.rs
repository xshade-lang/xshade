use ::std::mem;
use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::ast::type_checking::error::type_not_found::TypeNotFoundError;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::{ TypeReference, TypeEnvironmentReference };
use ::type_system::structure_members::{ StructureMembers, StructureMember };

pub struct CheckStructMemberPass {
    symbol_table: SymbolTableReference,
    type_environment: TypeEnvironmentReference,
    result: PassResultReference,
    member_list: Vec<StructureMember>,
}

impl CheckStructMemberPass {
    pub fn new(symbol_table: SymbolTableReference, type_environment: TypeEnvironmentReference, result: PassResultReference) -> CheckStructMemberPass {
        CheckStructMemberPass {
            symbol_table: symbol_table,
            type_environment: type_environment,
            result: result,
            member_list: Vec::new(),
        }
    }
}

ast_pass_impl!(CheckStructMemberPass, {
    fn visit_struct(&mut self, struct_definition: &mut StructDefinition) {
        self.member_list.clear();

        self.walk_struct(struct_definition);

        let member_list = mem::replace(&mut self.member_list, Vec::new());

        if member_list.len() != struct_definition.struct_member.len() {
            return;
        }

        match type_environment_mut!(self).find_type_mut(struct_definition.declaring_type.unwrap()) {
            Some(ref mut x) => pass_try!(self, x.set_members(StructureMembers::new(member_list))),
            None => panic!("test"),
        }
    }

    fn visit_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition) {

        let struct_member_type = pass_try!(self, match symbol_table!(self).find_type_ref(&struct_member_definition.struct_member_type_name.name) {
            Some(t) => Ok(t),
            None => Err(TypeNotFoundError::new(struct_member_definition.span, struct_member_definition.struct_member_type_name.name.to_string())),
        });

        struct_member_definition.struct_member_type = Some(struct_member_type);
        self.member_list.push(StructureMember::new(struct_member_definition.struct_member_name.name.clone(), struct_member_type));
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
        let mut ast = compile_ast("primitive type bool; struct Foo { bar: bool, }");
        let type_environment = TypeEnvironmentReference::new(TypeEnvironment::new());
        let symbol_table = SymbolTableReference::new(SymbolTable::new());
        let result = PassResultReference::new(PassResult::new());

        let mut passes = PassCollection::from_passes(vec![
            Box::new(check_primitives_pass::CheckPrimitivesPass::new(symbol_table.clone(), type_environment.clone(), result.clone())),
            Box::new(discover_structs_pass::DiscoverStructsPass::new(symbol_table.clone(), type_environment.clone(), result.clone())),
            Box::new(CheckStructMemberPass::new(symbol_table.clone(), type_environment.clone(), result.clone())),
        ]);

        passes.execute(&mut ast);

        assert_eq!(type_environment.borrow().find_type_by_name("Foo").unwrap().get_member().unwrap(), &StructureMembers::new(vec![
            StructureMember::new("bar".to_string(), TypeReference::new(0)), 
        ]));
    }
}
