use ::nom::*;
use ::ast::*;
use ::compile_error::CompileError;

named!(parse_identifier<&[u8], &[u8]>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
            ()
        )
    )
);

named!(parse_constant<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("const")) >>
        constant_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        constant_type: parse_type_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Constant(ConstantDefinition{
            constant_name: constant_name,
            constant_type: constant_type,
        }))
    )
);

named!(parse_sampler<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("sampler")) >>
        sampler_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        sampler_type: parse_type_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Sampler(SamplerDefinition{
            sampler_name: sampler_name,
            sampler_type: sampler_type,
        }))
    )
);

named!(parse_program_binding<&[u8], ProgramBindingDefinition>,
    do_parse!(
        program_binding_point: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        bound_function_name: parse_symbol_declaration >>
        (ProgramBindingDefinition{
            program_binding_point: program_binding_point,
            bound_function_name: bound_function_name,
        })
    )
);

named!(parse_program<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("program")) >>
        program_name: parse_symbol_declaration >>
        ws!(tag!("{")) >>
        program_bindings: ws!(separated_list!(tag!(","), parse_program_binding)) >>
        opt!(ws!(tag!(","))) >>
        ws!(tag!("}")) >>
        (ItemKind::Program(ProgramDefinition{
            program_name: program_name,
            program_bindings: program_bindings
        }))
    )
);

named!(parse_struct_member<&[u8], StructMemberDefinition>,
    do_parse!(
        struct_member_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        struct_member_type: parse_type_declaration >>
        (StructMemberDefinition{
            struct_member_name: struct_member_name,
            struct_member_type: struct_member_type,
        })
    )
);

named!(parse_struct<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("struct")) >>
        struct_name: parse_symbol_declaration >>
        ws!(tag!("{")) >>
        member: ws!(separated_list!(tag!(","), parse_struct_member)) >>
        opt!(ws!(tag!(","))) >>
        ws!(tag!("}")) >>
        (ItemKind::Struct(StructDefinition{
            struct_name: struct_name,
            struct_member: member
        }))
    )
);

named!(parse_function_argument<&[u8], FunctionArgumentDeclaration>,
    do_parse!(
        argument_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        argument_type: parse_type_declaration >>
        (FunctionArgumentDeclaration{
            argument_name: argument_name,
            argument_type: argument_type,
        })
    )
);

named!(parse_symbol_declaration<&[u8], Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_u8_slice(name))
    )
);

named!(parse_type_declaration<&[u8], Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_u8_slice(name))
    )
);

named!(parse_struct_instantiation_field_initializer<&[u8], StructFieldInitializerExpression>,
    do_parse!(
        struct_field_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        initializer: parse_expression >>
        (StructFieldInitializerExpression{
            struct_field_name: struct_field_name,
            initializer: Box::new(initializer),
        })
    )
);

named!(parse_struct_instantiation<&[u8], ExpressionStatement>,
    do_parse!(
        struct_type_name: parse_type_declaration >>
        ws!(tag!("{")) >>
        struct_field_initializer: ws!(separated_list!(tag!(","), parse_struct_instantiation_field_initializer)) >>
        opt!(ws!(tag!(","))) >>
        ws!(tag!("}")) >>
        (ExpressionStatement::StructInstantiation(StructInstantiationExpression{
            struct_type_name: struct_type_name,
            struct_field_initializer: struct_field_initializer,
        }))
    )
);

fn parse_int_literal(parts: Vec<char>) -> ExpressionStatement {
    let string: String = parts.into_iter().collect();
    ExpressionStatement::Literal(LiteralExpression::Int(string))
}

fn parse_float_literal(before: Vec<char>, after: Vec<char>) -> ExpressionStatement {
    let mut before: String = before.into_iter().collect();
    let after: String = after.into_iter().collect();
    before.push_str(".");
    before.push_str(&after);
    ExpressionStatement::Literal(LiteralExpression::Float(before))
}

named!(parse_float_literal_expression<&[u8], ExpressionStatement>,
    do_parse!(
        before: ws!(many0!(
            one_of!("0123456789")
        )) >>
        ws!(tag!(".")) >>
        after: ws!(many0!(
            one_of!("0123456789")
        )) >>
        (parse_float_literal(before, after))
    )
);

named!(parse_int_literal_expression<&[u8], ExpressionStatement>,
    do_parse!(
        numbers: ws!(many1!(
            one_of!("0123456789")
        )) >>
        (parse_int_literal(numbers))
    )
);

// TODO more literals
named!(parse_literal_expression<&[u8], ExpressionStatement>,
    alt!(
        parse_float_literal_expression |
        parse_int_literal_expression
    )
);

pub fn infix(operator: char, left: ExpressionStatement, right: ExpressionStatement) -> ExpressionStatement {
    match operator {
        '+' => ExpressionStatement::Infix(InfixExpression::Plus(Box::new(left), Box::new(right))),
        '-' => ExpressionStatement::Infix(InfixExpression::Minus(Box::new(left), Box::new(right))),
        '*' => ExpressionStatement::Infix(InfixExpression::Multiply(Box::new(left), Box::new(right))),
        '/' => ExpressionStatement::Infix(InfixExpression::Divide(Box::new(left), Box::new(right))),
        _ => panic!("")
    }
}

named!(parse_infix_expression<&[u8], ExpressionStatement>,
    do_parse!(
        left: parse_expression_no_left_recursion >>
        operator: ws!(one_of!("+-*/")) >>
        right: parse_expression >>
        (infix(operator, left, right))
    )
);

named!(parse_variable_expression<&[u8], ExpressionStatement>,
    do_parse!(
        variable_name: parse_symbol_declaration >>
        (ExpressionStatement::Variable(VariableExpression{
            variable_name: variable_name,
        }))
    )
);

named!(parse_call_statement<&[u8], BlockStatement>,
    do_parse!(
        call: parse_call >>
        (BlockStatement::Call(call))
    )
);

named!(parse_call_expression<&[u8], ExpressionStatement>,
    do_parse!(
        call: parse_call >>
        (ExpressionStatement::Call(call))
    )
);

named!(parse_call<&[u8], CallDeclaration>,
    do_parse!(
        function_name: parse_symbol_declaration >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_expression)) >>
        ws!(tag!(")")) >>
        (CallDeclaration {
            function_name: function_name,
            arguments: arguments,
        })
    )
);

// TODO more accessor types like `a.b.c`
named!(parse_accessor_expression<&[u8], ExpressionStatement>,
    do_parse!(
        variable_name: parse_symbol_declaration >>
        ws!(tag!(".")) >>
        accesse: parse_symbol_declaration >>
        (ExpressionStatement::Accessor(AccessorExpression{
            variable_name: variable_name,
            accesse: accesse,
        }))
    )
);

named!(parse_expression_no_left_recursion<&[u8], ExpressionStatement>,
    alt!(
        parse_struct_instantiation |
        parse_literal_expression | 
        parse_call_expression |
        parse_variable_expression
    )
);

// TODO precedence
// TODO parentheses
named!(parse_expression<&[u8], ExpressionStatement>,
    alt!(
        parse_infix_expression |
        parse_struct_instantiation |
        parse_literal_expression |
        parse_accessor_expression |
        parse_call_expression |
        parse_variable_expression |
        parse_accessor_expression
    )
);

named!(parse_local_declaration<&[u8], BlockStatement>,
    do_parse!(
        ws!(tag!("let")) >>
        symbol_name: parse_symbol_declaration >>
        ws!(tag!("=")) >>
        expression: parse_expression >>
        ws!(tag!(";")) >>
        (BlockStatement::Local(
            LocalDeclaration{
                symbol_name: symbol_name,
                expression: expression,
            }
        ))
    )
);

named!(parse_return_declaration<&[u8], BlockStatement>,
    do_parse!(
        ws!(tag!("return")) >>
        expression: parse_expression >>
        ws!(tag!(";")) >>
        (BlockStatement::Return(
            expression
        ))
    )
);

named!(parse_block_statements<&[u8], Vec<BlockStatement>>,
    many0!(
        ws!(
            alt!(
                parse_local_declaration |
                parse_return_declaration |
                parse_call_statement
            )
        )
    )
);

named!(parse_block_declaration<&[u8], BlockDeclaration>,
    do_parse!(
        statements: parse_block_statements >>
        (BlockDeclaration{
            statements: statements,
        })
    )
);

// TODO make return type optional
named!(parse_function<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("fn")) >>
        function_name: parse_symbol_declaration >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type: parse_type_declaration >>
        ws!(tag!("{")) >>
        block: parse_block_declaration >>
        ws!(tag!("}")) >>
        (ItemKind::Function(FunctionDeclaration{
            function_name: function_name,
            arguments: arguments,
            block: block,
            return_type: return_type,
        }))
    )
);

named!(parse<&[u8], Vec<ItemKind>>,
    many0!(
        ws!(
            alt!(
                parse_sampler |
                parse_constant |
                parse_struct |
                parse_program |
                parse_function
            )
        )
    )
);

pub fn parse_bytes(program: &[u8]) -> Result<Vec<ItemKind>, CompileError> {
    match parse(program) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(CompileError::new())
    }
}

pub fn parse_block(program: &str) -> Result<Vec<BlockStatement>, CompileError> {
    match parse_block_statements(program.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(CompileError::new())
    }
}

pub fn parse_str(program: &str) -> Result<Vec<ItemKind>, CompileError> {
    match parse(program.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(CompileError::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_let_statement() {
        let code = "let x = 42;";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::Literal(LiteralExpression::Int("42".to_string()))
                })
            ])
        );
    }

    #[test]
    fn parse_float_literal_statement() {
        let code = "let x = 42.0;";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::Literal(LiteralExpression::Float("42.0".to_string()))
                })
            ])
        );
    }

    #[test]
    fn parse_empty_call_statement() {
        let code = "test();";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Call(CallDeclaration{
                    function_name: Identifier::from_str("test"),
                    arguments: Vec::new(),
                })
            ])
        );
    }

    #[test]
    fn parse_call_statement() {
        let code = "test(a, b);";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Call(CallDeclaration{
                    function_name: Identifier::from_str("test"),
                    arguments: vec![
                        ExpressionStatement::Variable(VariableExpression{
                            variable_name: Identifier::from_str("a"),
                        }),
                        ExpressionStatement::Variable(VariableExpression{
                            variable_name: Identifier::from_str("b"),
                        })
                    ],
                })
            ])
        );
    }

    #[test]
    fn parse_empty_call_expression() {
        let code = "let x = test();";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::Call(CallDeclaration{
                        function_name: Identifier::from_str("test"),
                        arguments: Vec::new(),
                    })
                })
            ])
        );
    }

    #[test]
    fn parse_infix_expression_statement() {
        let code = "let x = a + b;";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::Infix(
                        InfixExpression::Plus(
                            Box::new(ExpressionStatement::Variable(VariableExpression{
                                variable_name: Identifier::from_str("a"),
                            })),
                            Box::new(ExpressionStatement::Variable(VariableExpression{
                                variable_name: Identifier::from_str("b"),
                            })),
                        )
                    ),
                })
            ])
        );
    }

    #[test]
    fn parse_nested_infix_expression_statement() {
        let code = "let x = a + b + c;";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::Infix(
                        InfixExpression::Plus(
                            Box::new(ExpressionStatement::Variable(VariableExpression{
                                variable_name: Identifier::from_str("a"),
                            })),
                            Box::new(ExpressionStatement::Infix(
                                InfixExpression::Plus(
                                    Box::new(ExpressionStatement::Variable(VariableExpression{
                                        variable_name: Identifier::from_str("b"),
                                    })),
                                    Box::new(ExpressionStatement::Variable(VariableExpression{
                                        variable_name: Identifier::from_str("c"),
                                    })),
                                )
                            )),
                        )
                    ),
                })
            ])
        );
    }

    #[test]
    fn parse_struct_instantiation_expression() {
        let code = r#"
            let x = SomeStruct {
                a: 24,
                b: 42,
            };
        "#;
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::StructInstantiation(StructInstantiationExpression{
                        struct_type_name: Identifier::from_str("SomeStruct"),
                        struct_field_initializer: vec![
                            StructFieldInitializerExpression{
                                struct_field_name: Identifier::from_str("a"),
                                initializer: Box::new(ExpressionStatement::Literal(LiteralExpression::Int("24".to_string()))),
                            },
                            StructFieldInitializerExpression{
                                struct_field_name: Identifier::from_str("b"),
                                initializer: Box::new(ExpressionStatement::Literal(LiteralExpression::Int("42".to_string()))),
                            },
                        ],
                    })
                })
            ])
        );
    }

    #[test]
    fn parse_nested_struct_instantiation_expression() {
        let code = r#"
            let x = SomeStruct {
                a: 24,
                b: SomeOtherStruct {
                    c: 42,
                },
            };
        "#;
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Local(LocalDeclaration{
                    symbol_name: Identifier::from_str("x"),
                    expression: ExpressionStatement::StructInstantiation(StructInstantiationExpression{
                        struct_type_name: Identifier::from_str("SomeStruct"),
                        struct_field_initializer: vec![
                            StructFieldInitializerExpression{
                                struct_field_name: Identifier::from_str("a"),
                                initializer: Box::new(ExpressionStatement::Literal(LiteralExpression::Int("24".to_string()))),
                            },
                            StructFieldInitializerExpression{
                                struct_field_name: Identifier::from_str("b"),
                                initializer: Box::new(ExpressionStatement::StructInstantiation(StructInstantiationExpression{
                                    struct_type_name: Identifier::from_str("SomeOtherStruct"),
                                    struct_field_initializer: vec![
                                        StructFieldInitializerExpression{
                                            struct_field_name: Identifier::from_str("c"),
                                            initializer: Box::new(ExpressionStatement::Literal(LiteralExpression::Int("42".to_string()))),
                                        },
                                    ],
                                })),
                            },
                        ],
                    })
                })
            ])
        );
    }

    #[test]
    fn parse_return_statement() {
        let code = "return 0;";
        assert_eq!(
            parse_block(code),
            Ok(vec![
                BlockStatement::Return(
                    ExpressionStatement::Literal(LiteralExpression::Int("0".to_string()))
                )
            ])
        );
    }

    #[test]
    fn parse_empty_function() {
        let code = "fn main() -> void {}";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Function(FunctionDeclaration{
                    function_name: Identifier::from_str("main"),
                    arguments: Vec::new(),
                    block: BlockDeclaration {
                        statements: Vec::new(),
                    },
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_empty_function_with_arguments() {
        let code = "fn main(a: B, c: D) -> void {}";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Function(FunctionDeclaration{
                    function_name: Identifier::from_str("main"),
                    arguments: vec![
                        FunctionArgumentDeclaration {
                            argument_name: Identifier::from_str("a"),
                            argument_type: Identifier::from_str("B"),
                        },
                        FunctionArgumentDeclaration {
                            argument_name: Identifier::from_str("c"),
                            argument_type: Identifier::from_str("D"),
                        }
                    ],
                    block: BlockDeclaration {
                        statements: Vec::new(),
                    },
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_function_with_expression() {
        let code = "fn main(a: B) -> void { return a; }";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Function(FunctionDeclaration{
                    function_name: Identifier::from_str("main"),
                    arguments: vec![
                        FunctionArgumentDeclaration {
                            argument_name: Identifier::from_str("a"),
                            argument_type: Identifier::from_str("B"),
                        }
                    ],
                    block: BlockDeclaration {
                        statements: vec![
                            BlockStatement::Return(ExpressionStatement::Variable(VariableExpression{ variable_name: Identifier::from_str("a") }))
                        ],
                    },
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_empty_function_with_expression() {
        let code = "fn main() -> void { return 0.0; }";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Function(FunctionDeclaration{
                    function_name: Identifier::from_str("main"),
                    arguments: Vec::new(),
                    block: BlockDeclaration {
                        statements: vec![
                            BlockStatement::Return(ExpressionStatement::Literal(LiteralExpression::Float("0.0".to_string())))
                        ],
                    },
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_sampler() {
        let code = "sampler albedo: Sampler2d;";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Sampler(SamplerDefinition {
                    sampler_name: Identifier::from_str("albedo"),
                    sampler_type: Identifier::from_str("Sampler2d"),
                })
            ])
        );
    }

    #[test]
    fn parse_const() {
        let code = "const mvp: mat4x4;";
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Constant(ConstantDefinition {
                    constant_name: Identifier::from_str("mvp"),
                    constant_type: Identifier::from_str("mat4x4"),
                })
            ])
        );
    }

    #[test]
    fn parse_struct() {
        let code = r#"
            struct VS_IN {
                position: vec3,
                uv: vec2,
            }
        "#;
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Struct(StructDefinition {
                    struct_name: Identifier::from_str("VS_IN"),
                    struct_member: vec![
                        StructMemberDefinition {
                            struct_member_name: Identifier::from_str("position"),
                            struct_member_type: Identifier::from_str("vec3"),
                        },
                        StructMemberDefinition {
                            struct_member_name: Identifier::from_str("uv"),
                            struct_member_type: Identifier::from_str("vec2"),
                        },
                    ],
                })
            ])
        );
    }

    #[test]
    fn parse_program() {
        let code = r#"
            program Flat {
                vertex: vertexShader,
                fragment: fragmentShader,
            }
        "#;
        assert_eq!(
            parse_str(code),
            Ok(vec![
                ItemKind::Program(ProgramDefinition {
                    program_name: Identifier::from_str("Flat"),
                    program_bindings: vec![
                        ProgramBindingDefinition {
                            program_binding_point: Identifier::from_str("vertex"),
                            bound_function_name: Identifier::from_str("vertexShader"),
                        },
                        ProgramBindingDefinition {
                            program_binding_point: Identifier::from_str("fragment"),
                            bound_function_name: Identifier::from_str("fragmentShader"),
                        },
                    ],
                })
            ])
        );
    }
}
