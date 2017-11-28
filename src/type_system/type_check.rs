use std::collections::HashMap;
use ::ast::*;
use ::module::Module;
use ::type_system::call_signature::CallSignature;
use ::type_system::structure_members::{ StructureMember, StructureMembers };
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::{ TypeReference };

pub fn type_check(symbol_table: &mut SymbolTable, module: &mut Module) -> TypeCheckResult<()> {
    println!("Checking Types!");

    symbol_table.enter_scope();
    try!(check_primitives(module.is_core(), symbol_table, &mut module.find_primitives_mut()));
    try!(check_structs(symbol_table, &mut module.find_structs_mut()));
    try!(check_casts(module.is_core(), symbol_table, &mut module.find_casts_mut()));
    try!(check_constant(symbol_table, &mut module.find_constants_mut()));
    try!(check_functions(symbol_table, &mut module.find_functions_mut()));
    try!(check_programs(symbol_table, &mut module.find_programs_mut()));
    symbol_table.leave_scope();
    
    println!("Done Checking Types!");
    println!("Current symbol_table: {:#?}", symbol_table);

    Ok(())
}

fn check_primitives(is_core_module: bool, symbol_table: &mut SymbolTable, primitives: &mut Vec<&mut PrimitiveDeclaration>) -> TypeCheckResult<()> {
    if !is_core_module && primitives.len() > 0 {
        return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SyntaxOnlyValidInCoreModule))
    }
    for p in primitives.iter_mut() {
        let type_ref = try!(symbol_table.create_global_type(&p.type_name.name));
        p.declaring_type = Some(type_ref);
    }
    Ok(())
}

fn check_structs(symbol_table: &mut SymbolTable, structs: &mut Vec<&mut StructDefinition>) -> TypeCheckResult<()> {
    for s in structs.iter_mut() {
        let type_ref = try!(symbol_table.create_type(&s.struct_name.name));
        try!(symbol_table.add_symbol_with_type(&s.struct_name.name, type_ref));
        s.declaring_type = Some(type_ref);
    }

    for s in structs.iter_mut() {
        let mut member_list = Vec::new();
        for member in s.struct_member.iter_mut() {
            let struct_member_type = try!(symbol_table.find_type_ref_or_err(&member.struct_member_type_name.name));
            member.struct_member_type = Some(struct_member_type);
            member_list.push(StructureMember::new(member.struct_member_name.name.clone(), struct_member_type));
        }
        let mut t = try!(symbol_table.find_type_mut_or_err(s.declaring_type.unwrap())); //TODO ugly unwrap
        try!(t.set_members(StructureMembers::new(member_list)));
    }

    
    Ok(())
}

fn check_casts(is_core_module: bool, symbol_table: &mut SymbolTable, casts: &mut Vec<&mut CastDeclaration>) -> TypeCheckResult<()> {
    if !is_core_module && casts.len() > 0 {
        return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::SyntaxOnlyValidInCoreModule));
    }
    for c in casts.iter_mut() {
        let source_type = try!(symbol_table.find_type_ref_or_err(&c.source_type.name));
        let target_type = try!(symbol_table.find_type_ref_or_err(&c.target_type.name));

        let mut source_type = match symbol_table.find_type_mut(source_type) {
            Some(t) => t,
            None => return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::TypeNotFound(c.source_type.name.clone())))
        };

        if source_type.does_cast_exist(target_type.clone()) {
            return Err(TypeError::new(Span::new(0, 0, 1, 1), ErrorKind::CastAlreadyDeclared(c.source_type.name.clone(), c.target_type.name.clone())));
        }

        match c.cast_type {
            CastType::Implicit => source_type.add_implicit_cast(target_type),
            CastType::Explicit => source_type.add_explicit_cast(target_type),
        }
    }
    Ok(())
}

fn check_constant(symbol_table: &mut SymbolTable, constants: &mut Vec<&mut ConstantDefinition>) -> TypeCheckResult<()> {
    for s in constants.iter_mut() {
        try!(symbol_table.add_symbol(&s.constant_name.name));

        let type_ref = try!(symbol_table.find_type_ref_or_err(&s.constant_type_name.name));
        
        s.constant_type = Some(type_ref.clone());
        try!(symbol_table.resolve_symbol_type(&s.constant_name.name, type_ref.clone()))
    }
    Ok(())
}

fn check_functions(symbol_table: &mut SymbolTable, functions: &mut Vec<&mut FunctionDeclaration>) -> TypeCheckResult<()> {
    for f in functions.iter_mut() {
        let function_type = try!(symbol_table.create_type(&f.function_name.name));
        try!(symbol_table.add_symbol_with_type(&f.function_name.name, function_type));

        let mut arguments = Vec::new();
        symbol_table.enter_scope();
        for argument in f.arguments.iter_mut() {

            let type_ref = match symbol_table.find_type_ref(&argument.argument_type_name.name) {
                Some(t) => t,
                None => return Err(TypeError::new(argument.argument_type_name.get_span(), ErrorKind::TypeNotFound(argument.argument_type_name.name.to_owned()))),
            };
            
            argument.argument_type = Some(type_ref);
            try!(symbol_table.add_symbol_with_type(&argument.argument_name.name, type_ref));
            arguments.push(type_ref);
        }

        let type_ref = try!(symbol_table.find_type_ref_or_err(&f.return_type_name.name));
        f.return_type = Some(type_ref.clone());
        f.declaring_type = Some(function_type);

        let signature = CallSignature::new(arguments, Some(type_ref));
        try!(try!(symbol_table.find_type_mut_or_err(function_type)).make_callable(signature));

        try!(check_block(symbol_table, &mut f.block));
        symbol_table.leave_scope();
    }
    Ok(())
}

fn check_programs(symbol_table: &mut SymbolTable, programs: &mut Vec<&mut ProgramDefinition>) -> TypeCheckResult<()> {
    for p in programs.iter_mut() {
        let mut stageTrace = HashMap::new();
        
        {
            for s in p.program_stages.iter_mut() {
                let stage_type = try!(symbol_table.create_type(&s.stage_name.name));
                try!(symbol_table.add_symbol_with_type(&s.stage_name.name, stage_type));

                let mut arguments = Vec::new();
                symbol_table.enter_scope();
                for argument in s.arguments.iter_mut() {

                    let type_ref = match symbol_table.find_type_ref(&argument.argument_type_name.name) {
                        Some(t) => t,
                        None => return Err(TypeError::new(argument.argument_type_name.get_span(), ErrorKind::TypeNotFound(argument.argument_type_name.name.to_owned()))),
                    };
                    
                    argument.argument_type = Some(type_ref);
                    try!(symbol_table.add_symbol_with_type(&argument.argument_name.name, type_ref));
                    arguments.push(type_ref);
                }

                let type_ref = try!(symbol_table.find_type_ref_or_err(&s.return_type_name.name));
                s.return_type = Some(type_ref.clone());
                s.declaring_type = Some(stage_type);

                let signature = CallSignature::new(arguments, Some(type_ref));
                try!(try!(symbol_table.find_type_mut_or_err(stage_type)).make_callable(signature));

                try!(check_block(symbol_table, &mut s.block));
                symbol_table.leave_scope();
                
                *stageTrace.entry(&*s.stage_name.name).or_insert(0) += 1;
            }
        }

        // 
        // Perform program analysis and verification:
        // 1. Is at least a vertex and fragment stage available
        // 2. Are there duplicate stages?
        // 3. Is the input signature to vertex stage valid?
        // 4. Are the signatures between vertex and fragment stage compatible?
        // 
        if !stageTrace.contains_key("vertex") {
            return Err(TypeError::new((&p).get_span(), ErrorKind::TypeNotFound("vertex".to_owned())))
        }

        if !stageTrace.contains_key("fragment") {
            return Err(TypeError::new((&p).get_span(), ErrorKind::TypeNotFound("fragment".to_owned())))
        }
    }
    Ok(())
}

fn check_block(symbol_table: &mut SymbolTable, block: &mut BlockDeclaration) -> TypeCheckResult<()> {
    symbol_table.enter_scope();
    for s in block.statements.iter_mut() {
        match *s {
            BlockStatement::Local(ref mut local_declaration) => {
                let local_type = try!(check_expression(symbol_table, &mut local_declaration.expression));
                local_declaration.local_type = Some(local_type.clone());
                try!(symbol_table.add_symbol_with_type(&local_declaration.symbol_name.name, local_type));
            },
            BlockStatement::Return(ref mut return_declaration) => {
                let return_type = try!(check_expression(symbol_table, &mut return_declaration.expression));
                return_declaration.return_type = Some(return_type);
            },
            BlockStatement::Expression(ref mut expression_statement) => {
                try!(check_expression(symbol_table, expression_statement));
            }
        }
    }
    symbol_table.leave_scope();
    Ok(())
}

fn check_expression(symbol_table: &mut SymbolTable, expression: &mut ExpressionStatement) -> TypeCheckResult<TypeReference> {
    match *expression {
        ExpressionStatement::Literal(ref mut literal_expression) => check_literal_expression(symbol_table, literal_expression),
        ExpressionStatement::Variable(ref mut variable_expression) => check_variable_expression(symbol_table, variable_expression),
        ExpressionStatement::Infix(ref mut infix_expression) => check_infix_expression(symbol_table, infix_expression),
        ExpressionStatement::StructInstantiation(ref mut struct_instantiation_expression) => check_struct_instatiation_expression(symbol_table, struct_instantiation_expression),
        ExpressionStatement::Call(ref mut call_expression) => check_call_expression(symbol_table, call_expression),
        ExpressionStatement::FieldAccessor(ref mut field_accessor_expression) => check_field_accessor_expression(symbol_table, field_accessor_expression),
        _ => Ok(TypeReference::new(0)), // TODO implement remaining expressions
    }
}

fn check_field_accessor_expression(symbol_table: &mut SymbolTable, field_accessor_expression: &mut FieldAccessorExpression) -> TypeCheckResult<TypeReference> {
    let type_ref = match symbol_table.find_symbol(&field_accessor_expression.variable_name.name) {
        Some(symbol) => match symbol.get_type() {
            Some(t) => t,
            None => return Err(TypeError::new(Span::empty(), ErrorKind::CannotInfer(field_accessor_expression.variable_name.name.clone()))),
        },
        None => return Err(TypeError::new(Span::empty(), ErrorKind::VariableNotFound(field_accessor_expression.variable_name.name.clone()))),
    };

    let field_type_ref = match symbol_table.find_type(type_ref) {
        Some(t) => {
            if !t.has_member() {
                return Err(TypeError::new(field_accessor_expression.get_span(), ErrorKind::TypeHasNoMember));
            }

            match t.find_member_type(&field_accessor_expression.field_name.name) {
                Some(t) => t,
                None => return Err(TypeError::new(Span::empty(), ErrorKind::MemberNotFound)),
            }
        },
        None => return Err(TypeError::new(Span::empty(), ErrorKind::CannotInfer(field_accessor_expression.variable_name.name.clone()))),
    };

    field_accessor_expression.field_type = Some(field_type_ref);

    Ok(field_type_ref)
}

fn check_call_expression(symbol_table: &mut SymbolTable, call_expression: &mut CallExpression) -> TypeCheckResult<TypeReference> {
    let function_type_ref = try!(symbol_table.find_type_ref_or_err(&call_expression.function_name.name));

    let mut argument_types = Vec::new();
    for argument in call_expression.arguments.iter_mut() {
        argument_types.push(try!(check_expression(symbol_table, argument)));
    }

    let function_type = try!(symbol_table.find_type_or_err(function_type_ref));
    let signature = try!(function_type.get_call_signature_or_err());
    try!(signature.match_arguments_or_err(argument_types));

    let return_type = signature.get_return_type().unwrap(); // TODO void types

    Ok(return_type)
}

fn check_struct_instatiation_expression(symbol_table: &mut SymbolTable, struct_instantiation_expression: &mut StructInstantiationExpression) -> TypeCheckResult<TypeReference> {
    let struct_type = try!(symbol_table.find_type_ref_or_err(&struct_instantiation_expression.struct_type_name.name));
    struct_instantiation_expression.struct_type = Some(struct_type);

    let mut x = Vec::new();
    for initializer in struct_instantiation_expression.struct_field_initializer.iter_mut() {
        // TODO check if all struct fields are assigned
        // TODO check if initializer is same as or convertible to struct field type
        let initializer_type = try!(check_expression(symbol_table, &mut *initializer.initializer));
        initializer.struct_field_type = Some(initializer_type);
        x.push(StructureMember::new(initializer.struct_field_name.name.clone(), initializer_type));
    }
    match symbol_table.find_type(struct_type) {
        Some(t) => match t.get_member() {
            Some(m) => {
                if !m.is_assignable_with(&x) {
                    return Err(TypeError::new(Span::empty(), ErrorKind::CannotInstantiateStructWithArguments));
                }
            },
            None => {
                return Err(TypeError::new(Span::empty(), ErrorKind::TypeHasNoMember));
            }
        },
        None => return Err(TypeError::new(Span::empty(), ErrorKind::TypeNotFound(struct_instantiation_expression.struct_type_name.name.clone()))),
    }

    Ok(struct_type)
}

fn check_infix_expression(symbol_table: &mut SymbolTable, infix_expression: &mut InfixExpression) -> TypeCheckResult<TypeReference> {
    let left_hand_type = try!(check_expression(symbol_table, &mut *infix_expression.left_hand));
    let right_hand_type = try!(check_expression(symbol_table, &mut *infix_expression.right_hand));

    // TODO associativity

    if left_hand_type != right_hand_type {
        // TODO implicit casts

        let left_span = infix_expression.left_hand.get_span();
        let right_span = infix_expression.right_hand.get_span();
        return Err(TypeError::new(Span::from_to(left_span, right_span), ErrorKind::IncompatibleTypes(left_span, right_span)));
    }

    // TODO check if operator is available

    infix_expression.infix_type = Some(left_hand_type);

    Ok(left_hand_type)
}

fn check_variable_expression(symbol_table: &mut SymbolTable, variable_expression: &mut VariableExpression) -> TypeCheckResult<TypeReference> {
    match symbol_table.find_symbol(&variable_expression.variable_name.name) {
        Some(ref symbol) => match symbol.get_type() {
            Some(t) => {
                variable_expression.variable_type = Some(t);
                return Ok(t);
            },
            None => return Err(TypeError::new(Span::empty(), ErrorKind::CannotInfer(variable_expression.variable_name.name.clone()))),
        },
        None => return Err(TypeError::new(Span::empty(), ErrorKind::VariableNotFound(variable_expression.variable_name.name.clone()))),
    }
}

fn check_literal_expression(symbol_table: &mut SymbolTable, literal_expression: &mut LiteralExpression) -> TypeCheckResult<TypeReference> {
    let type_ref = match literal_expression.literal_expression_type {
        LiteralType::Float => try!(symbol_table.find_type_ref_or_err("f32")),
        LiteralType::Int => try!(symbol_table.find_type_ref_or_err("i32")),
    };
    literal_expression.literal_type = Some(type_ref.clone());
    Ok(type_ref.clone())
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

    #[test]
    fn test_check_function() {
        let code = r#"
            fn add(x: f32, y: f32) -> f32 {
                return x + y;
            }
        "#;

        assert!(parse_module(code).is_ok());
    }

    #[test]
    fn test_create_struct() {
        let code = r#"
            struct Vec4 {
                x: f32,
                y: f32,
                z: f32,
                w: f32,
            }

            fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
                return Vec4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                };
            }

            fn main() -> Vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        "#;

        assert!(parse_module(code).is_ok());
    }

    #[test]
    fn test_create_struct_wrong_number_of_arguments() {
        let code = r#"
            struct Vec4 {
                x: f32,
                y: f32,
                z: f32,
                w: f32,
            }

            fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
                return Vec4 {
                    x: x,
                    y: y,
                    z: z,
                };
            }

            fn main() -> Vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        "#;

        assert!(parse_module(code).is_err());
    }

    #[test]
    fn test_create_struct_wrong_types() {
        let code = r#"
            struct Vec4 {
                x: f32,
                y: f32,
                z: f32,
                w: f32,
            }

            fn vec4(x: f32, y: f32, z: f32, w: f64) -> Vec4 {
                return Vec4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                };
            }

            fn main() -> Vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        "#;

        assert!(parse_module(code).is_err());
    }
}
