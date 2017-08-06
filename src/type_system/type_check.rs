use ::ast::*;
use ::module::Module;
use ::type_system::error::{ TypeError, TypeCheckResult };
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;

pub fn type_check(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, module: &mut Module) -> TypeCheckResult<()> {
    symbol_table.enter_scope();
    try!(check_primitives(module.is_core(), type_environment, symbol_table, &mut module.find_primitives_mut()));
    try!(check_structs(type_environment, symbol_table, &mut module.find_structs_mut()));
    try!(check_constant(type_environment, symbol_table, &mut module.find_constants_mut()));
    symbol_table.leave_scope();
    Ok(())
}

fn check_primitives(is_core_module: bool, type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, primitives: &mut Vec<&mut PrimitiveDeclaration>) -> TypeCheckResult<()> {
    if !is_core_module && primitives.len() > 0 {
        return Err(TypeError::SyntaxOnlyValidInCoreModule)
    }
    for p in primitives.iter_mut() {
        let reference = try!(type_environment.create_type(&p.type_name.name));
        try!(symbol_table.add_global_type(&p.type_name.name, reference));
        p.declaring_type = Type::Typed(reference);
    }
    Ok(())
}

fn check_structs(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, structs: &mut Vec<&mut StructDefinition>) -> TypeCheckResult<()> {
    for s in structs.iter_mut() {
        try!(symbol_table.add_symbol(&s.struct_name.name));
        let reference = try!(type_environment.create_type(&s.struct_name.name));
        try!(symbol_table.add_type(&s.struct_name.name, reference));
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

fn check_constant(type_environment: &mut TypeEnvironment, symbol_table: &mut SymbolTable, constants: &mut Vec<&mut ConstantDefinition>) -> TypeCheckResult<()> {
    for s in constants.iter_mut() {
        try!(symbol_table.add_symbol(&s.constant_name.name));
        match symbol_table.find_type(&s.constant_type_name.name) {
            Some(type_ref) => {
                s.constant_type = Type::Typed(type_ref.clone());
            },
            None => {
                return Err(TypeError::TypeNotFound(s.constant_type_name.name.clone()));
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
    fn test_check_struct() {
        let code = r#"
            struct Test {
                x: f32,
                y: f32,
            }
        "#;

        assert!(parse_module(code).is_ok());
    }

    #[test]
    fn test_check_struct_with_unknown_type() {
        let code = r#"
            struct Test {
                x: ThisTypeShouldNotExist,
            }
        "#;

        assert!(parse_module(code).is_err());
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
        "#;

        parse_module(code).unwrap();

        assert!(parse_module(code).is_ok());
    }

    #[test]
    fn test_check_constant_unknown() {
        let code = r#"
            const test: ThisTypeShouldNotExist;
        "#;

        assert!(parse_module(code).is_err());
    }

    #[test]
    fn test_check_constant() {
        let code = r#"
            const test: f32;
        "#;

        assert!(parse_module(code).is_ok());
    }

    #[test]
    fn test_check_sampler_unknown() {
        let code = r#"
            sampler test: ThisTypeShouldNotExist;
        "#;

        assert!(parse_module(code).is_err());
    }

    #[test]
    fn test_check_sampler() {
        let code = r#"
            sampler test: f32;
        "#;

        assert!(parse_module(code).is_ok());
    }
}
