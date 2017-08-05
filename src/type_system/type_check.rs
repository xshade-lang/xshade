use ::std::error::Error;
use ::ast::*;
use ::module::Module;
use ::type_system::error::{ TypeError, TypeCheckResult };
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;

pub fn type_check(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, module: &mut Module) -> TypeCheckResult<()> {
    symbol_table.enter_scope();
    try!(check_primitives(type_environment, symbol_table, &mut module.find_primitives_mut()));
    try!(check_structs(type_environment, symbol_table, &mut module.find_structs_mut()));
    try!(check_samler(type_environment, symbol_table, &mut module.find_sampler_mut()));
    symbol_table.leave_scope();
    Ok(())
}

fn check_primitives(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, primitives: &mut Vec<&mut PrimitiveDeclaration>) -> TypeCheckResult<()> {
    for p in primitives.iter_mut() {
        let reference = try!(type_environment.create_type(&p.type_name.name));
        try!(symbol_table.add_type(&p.type_name.name, reference));
        p.declaring_type = Type::Typed(reference);
    }
    Ok(())
}

fn check_structs(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, structs: &mut Vec<&mut StructDefinition>) -> TypeCheckResult<()> {

    for s in structs.iter_mut() {
        try!(symbol_table.add_symbol(&s.struct_name.name));
        let reference = try!(type_environment.create_type(&s.struct_name.name));
        symbol_table.add_type(&s.struct_name.name, reference);
        s.declaring_type = Type::Typed(reference);
    }

    for s in structs.iter_mut() {
        for member in s.struct_member.iter_mut() {
            match symbol_table.find_type(&member.struct_member_type_name.name) {
                Some(type_ref) => {
                    member.struct_member_type = Type::Typed(type_ref.clone());
                },
                None => {
                    return Err(TypeError::TypeNotFound(member.struct_member_type_name.name.clone()));
                },
            }
        }
    }
    
    Ok(())
}

fn check_samler(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, sampler: &mut Vec<&mut SamplerDefinition>) -> TypeCheckResult<()> {
    for s in sampler.iter_mut() {
        try!(symbol_table.add_symbol(&s.sampler_name.name));
        match symbol_table.find_type(&s.sampler_type.name) {
            Some(type_ref) => {

            },
            None => {
                return Err(TypeError::TypeNotFound(s.sampler_type.name.clone()));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::api::parse_module;

    #[test]
    fn test_check_primitive() {
        let code = r#"
            primitive type f32;
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        type_check(&mut type_environment, &mut symbol_table, &mut module).unwrap();

        let type_ref = type_environment.find_reference_by_name("f32").unwrap();

        assert_eq!(module.find_primitives(), vec![&PrimitiveDeclaration{
            type_name: Identifier::from_str("f32"),
            declaring_type: Type::Typed(type_ref),
        }]);
    }

    #[test]
    fn test_check_struct() {
        let code = r#"
            primitive type f32;
            struct Test {
                x: f32,
                y: f32,
            }
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        assert!(type_check(&mut type_environment, &mut symbol_table, &mut module).is_ok());
    }

    #[test]
    fn test_check_struct_with_unknown_type() {
        let code = r#"
            struct Test {
                x: f32,
            }
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        assert!(type_check(&mut type_environment, &mut symbol_table, &mut module).is_err());
    }

    #[test]
    fn test_forward_declarations() {
        let code = r#"
            struct Test {
                x: Test2,
            }

            struct Test2 {
                y: f32,
            }
            
            primitive type f32;
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        assert!(type_check(&mut type_environment, &mut symbol_table, &mut module).is_ok());
    }

    #[test]
    fn test_check_sampler_unknown() {
        let code = r#"
            sampler Albedo: Sampler2d;
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        assert!(type_check(&mut type_environment, &mut symbol_table, &mut module).is_err());
    }

    #[test]
    fn test_check_sampler() {
        let code = r#"
            primitive type Sampler2d;
            sampler Albedo: Sampler2d;
        "#;

        let mut module = parse_module(code).unwrap();
        let mut type_environment = TypeEnvironment::new();
        let mut symbol_table = SymbolTable::new();

        assert!(type_check(&mut type_environment, &mut symbol_table, &mut module).is_ok());
    }
}
