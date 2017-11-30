use ::nom::*;
use ::nom_locate::LocatedSpan;
use ::ast::*;
use ::compile_error::{ CompileError, CompileResult };
use ::compile_error::ErrorKind as CompileErrorKind;

type NomSpan<'a> = LocatedSpan<&'a str>;

named!(parse_identifier<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
            ()
        )
    )
);

named!(parse_number<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            many1!(one_of!("0123456789")) >>
            ()
        )
    )
);

named!(parse_path<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(
                one_of!("\\/_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
            ) >>
            ()
        )
    )
);

named!(parse_path_declaration<NomSpan, String>,
    do_parse!(
        name: ws!(parse_path) >>
        (Identifier::from_nom_span(name).name.to_owned())
    )
);

named!(parse_symbol_or_wildcard_declaration<NomSpan, Identifier>,
    do_parse!(
        name: ws!(alt!(tag!("*") | parse_identifier)) >>
        (Identifier::from_nom_span(name))
    )
);

named!(parse_constant<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("const")) >>
        constant_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        constant_type_name: parse_type_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Constant(ConstantDefinition{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            constant_name: constant_name,
            constant_variant: ConstantVariant::Constant,
            constant_type_name: constant_type_name,
            constant_type: None,
        }))
    )
);

named!(parse_module_exports<NomSpan, Vec<Identifier>>,
    do_parse!(
        exports: 
            alt!(
                do_parse!(
                    single_export: ws!(parse_symbol_or_wildcard_declaration) >>
                    (vec![single_export])
                ) |
                do_parse!(
                    ws!(tag!("{")) >>
                    list: ws!(separated_list!(tag!(","), parse_symbol_declaration)) >>
                    ws!(tag!("}")) >>
                    (list)
                )
            ) >>
        (exports)
    )
);

named!(parse_module_id<NomSpan, String>,
    do_parse!(
        ws!(opt!(tag!("'"))) >>
        module_id: parse_path_declaration >> 
        ws!(opt!(tag!("'"))) >>
        (
            module_id
        )
    )
);

named!(parse_import<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("import")) >>
        exports: parse_module_exports >> 
        ws!(tag!("from")) >>
        module_id: parse_module_id >> 
        to: ws!(tag!(";")) >> 
        (ItemKind::Import(ImportDefinition{ 
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            export_selection: exports,
            module_id: module_id,
        }))        
    )
);

named!(parse_sampler<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("sampler")) >>
        sampler_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        sampler_type_name: parse_type_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Constant(ConstantDefinition{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            constant_name: sampler_name,
            constant_variant: ConstantVariant::Sampler,
            constant_type_name: sampler_type_name,
            constant_type: None,
        }))
    )
);

named!(parse_program<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("program")) >>
        program_name: parse_symbol_declaration >>
        ws!(tag!("{")) >>
        program_stages: many0!(ws!(parse_stage)) >> 
        opt!(ws!(tag!(","))) >>
        to: ws!(tag!("}")) >>
        (ItemKind::Program(ProgramDefinition{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            program_name: program_name,
            program_stages: program_stages,
        }))
    )
);

named!(parse_stage<NomSpan, ProgramStageDefinition>,    
    do_parse!(
        from: ws!(tag!("stage")) >>
        stage_name: ws!(alt!(tag!("vertex") | tag!("fragment"))) >> 
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type_name: parse_type_declaration >>
        ws!(tag!("{")) >>
        block: parse_block_declaration >>
        to: ws!(tag!("}")) >>        
         (ProgramStageDefinition {
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            stage_name: Identifier::from_nom_span(stage_name),
            function: FunctionDeclaration {
                span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
                function_name: Identifier::from_nom_span(stage_name),
                arguments: arguments,
                block: block,
                return_type_name: return_type_name,
                return_type: None,
                declaring_type: None,
            },
            declaring_type: None,
        })       
    )
);

named!(parse_struct_member<NomSpan, StructMemberDefinition>,
    do_parse!(
        struct_member_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        struct_member_type_name: parse_type_declaration >>
        (StructMemberDefinition{
            span: Span::from_to(struct_member_name.span, struct_member_type_name.span),
            struct_member_name: struct_member_name,
            struct_member_type_name: struct_member_type_name,
            struct_member_type: None,
        })
    )
);

named!(parse_struct<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("struct")) >>
        struct_name: parse_symbol_declaration >>
        ws!(tag!("{")) >>
        member: ws!(separated_list!(tag!(","), parse_struct_member)) >>
        opt!(ws!(tag!(","))) >>
        to: ws!(tag!("}")) >>
        (ItemKind::Struct(StructDefinition{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            struct_name: struct_name,
            struct_member: member,
            declaring_type: None,
        }))
    )
);

named!(parse_function_argument<NomSpan, FunctionArgumentDeclaration>,
    do_parse!(
        argument_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        argument_type_name: parse_type_declaration >>
        (FunctionArgumentDeclaration{
            span: Span::from_to(argument_name.span, argument_type_name.span),
            argument_name: argument_name,
            argument_type_name: argument_type_name,
            argument_type: None,
        })
    )
);

named!(parse_symbol_declaration<NomSpan, Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_nom_span(name))
    )
);

named!(parse_type_declaration<NomSpan, Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_nom_span(name))
    )
);

named!(parse_struct_instantiation_field_initializer<NomSpan, StructFieldInitializerExpression>,
    do_parse!(
        struct_field_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        initializer: parse_expression >>
        (StructFieldInitializerExpression{
            span: Span::from_to(struct_field_name.span, initializer.get_span()),
            struct_field_name: struct_field_name,
            initializer: Box::new(initializer),
            struct_field_type: None,
        })
    )
);

named!(parse_struct_instantiation<NomSpan, ExpressionStatement>,
    do_parse!(
        struct_type_name: parse_type_declaration >>
        ws!(tag!("{")) >>
        struct_field_initializer: ws!(separated_list!(tag!(","), parse_struct_instantiation_field_initializer)) >>
        opt!(ws!(tag!(","))) >>
        to: ws!(tag!("}")) >>
        (ExpressionStatement::StructInstantiation(StructInstantiationExpression{
            span: Span::from_to(struct_type_name.span, Span::from_nom_span(&to)),
            struct_type_name: struct_type_name,
            struct_field_initializer: struct_field_initializer,
            struct_type: None,
        }))
    )
);

fn parse_int_literal(parts: NomSpan) -> ExpressionStatement {
    let string: String = parts.fragment.to_string();
    ExpressionStatement::Literal(LiteralExpression {
        span: Span::from_nom_span(&parts),
        value: string,
        literal_expression_type: LiteralType::Int,
        literal_type: None,
    })
}

fn parse_float_literal(before: NomSpan, after: NomSpan) -> ExpressionStatement {
    let mut a: String = before.fragment.to_string();
    let b: String = after.fragment.to_string();
    a.push_str(".");
    a.push_str(&b);
    ExpressionStatement::Literal(LiteralExpression {
        span: Span::from_to(Span::from_nom_span(&before), Span::from_nom_span(&after)),
        value: a,
        literal_expression_type: LiteralType::Float,
        literal_type: None,
    })
}

named!(parse_float_literal_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        before: ws!(parse_number) >>
        ws!(tag!(".")) >>
        after: ws!(parse_number) >>
        (parse_float_literal(before, after))
    )
);

named!(parse_int_literal_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        numbers: ws!(parse_number) >>
        (parse_int_literal(numbers))
    )
);

// TODO more literals
named!(parse_literal_expression<NomSpan, ExpressionStatement>,
    alt!(
        parse_float_literal_expression |
        parse_int_literal_expression
    )
);

named!(parse_infix_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        left: parse_expression_no_left_recursion >>
        operator: ws!(one_of!("+-*/")) >>
        right: parse_expression >>
        (ExpressionStatement::Infix(InfixExpression{
            span: Span::from_to(left.get_span(), right.get_span()),
            operator: char_to_operator(operator),
            left_hand: Box::new(left),
            right_hand: Box::new(right),
            infix_type: None,
        }))
    )
);

named!(parse_variable_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        variable_name: parse_symbol_declaration >>
        (ExpressionStatement::Variable(VariableExpression{
            span: variable_name.span.clone(),
            variable_name: variable_name,
            variable_type: None,
        }))
    )
);

named!(parse_call_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        call: parse_call >>
        (ExpressionStatement::Call(call))
    )
);

named!(parse_call<NomSpan, CallExpression>,
    do_parse!(
        function_name: parse_symbol_declaration >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_expression)) >>
        to: ws!(tag!(")")) >>
        (CallExpression {
            span: Span::from_to(function_name.span, Span::from_nom_span(&to)),
            function_name: function_name,
            arguments: arguments,
            function_type: None,
        })
    )
);

// TODO nested accessor expressions like `a.b.c`
named!(parse_field_accessor_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        variable_name: parse_symbol_declaration >>
        ws!(tag!(".")) >>
        field_name: parse_symbol_declaration >>
        (ExpressionStatement::FieldAccessor(FieldAccessorExpression{
            span: Span::from_to(variable_name.span, field_name.span),
            variable_name: variable_name,
            field_name: field_name,
            field_type: None,
        }))
    )
);

named!(parse_expression_no_left_recursion<NomSpan, ExpressionStatement>,
    alt!(
        parse_struct_instantiation |
        parse_literal_expression |
        parse_field_accessor_expression |
        parse_call_expression |
        parse_variable_expression
    )
);

// TODO precedence
// TODO parentheses
named!(parse_expression<NomSpan, ExpressionStatement>,
    alt!(
        parse_infix_expression |
        parse_struct_instantiation |
        parse_literal_expression |
        parse_field_accessor_expression |
        parse_call_expression |
        parse_variable_expression
    )
);

named!(parse_local_declaration<NomSpan, BlockStatement>,
    do_parse!(
        from: ws!(tag!("let")) >>
        symbol_name: parse_symbol_declaration >>
        ws!(tag!("=")) >>
        expression: parse_expression >>
        to: ws!(tag!(";")) >>
        (BlockStatement::Local(
            LocalDeclaration{
                span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
                symbol_name: symbol_name,
                expression: expression,
                local_type: None,
            }
        ))
    )
);

named!(parse_return_declaration<NomSpan, BlockStatement>,
    do_parse!(
        from: ws!(tag!("return")) >>
        expression: parse_expression >>
        to: ws!(tag!(";")) >>
        (BlockStatement::Return(ReturnDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            expression: expression,
            return_type: None,
        }))
    )
);

named!(parse_expression_declaration<NomSpan, BlockStatement>,
    do_parse!(
        expression: parse_expression >>
        ws!(tag!(";")) >>
        (BlockStatement::Expression(
            expression
        ))
    )
);

named!(parse_block_statements<NomSpan, Vec<BlockStatement>>,
    many0!(
        ws!(
            alt!(
                parse_local_declaration |
                parse_return_declaration |
                parse_expression_declaration
            )
        )
    )
);

named!(parse_block_declaration<NomSpan, BlockDeclaration>,
    do_parse!(
        statements: parse_block_statements >>
        (BlockDeclaration{
            span: Span::new(0, 0, 1, 1), // TODO complicated with empty blocks
            statements: statements,
        })
    )
);

// TODO make return type optional
named!(parse_function<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("fn")) >>
        function_name: parse_symbol_declaration >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type_name: parse_type_declaration >>
        ws!(tag!("{")) >>
        block: parse_block_declaration >>
        to: ws!(tag!("}")) >>
        (ItemKind::Function(FunctionDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            function_name: function_name,
            arguments: arguments,
            block: block,
            return_type_name: return_type_name,
            return_type: None,
            declaring_type: None,
        }))
    )
);

named!(parse_primitive<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("primitive")) >>
        ws!(tag!("type")) >>
        type_name: parse_symbol_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Primitive(PrimitiveDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            type_name: type_name,
            declaring_type: None,
        }))
    )
);

named!(parse_operator_type<NomSpan, Operator>,
    do_parse!(
        operator: ws!(one_of!("+-*/")) >>
        (char_to_operator(operator))
    )
);

fn char_to_operator(operator: char) -> Operator {
    match operator {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
        _ => panic!(""),
    }
}

named!(parse_operator<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("operator")) >>
        operator: parse_operator_type >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type: parse_type_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Operator(OperatorDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            operator: operator,
            arguments: arguments,
            return_type: return_type,
        }))
    )
);

named!(parse_implicit_cast<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("implicit")) >>
        ws!(tag!("cast")) >>
        source_type: parse_type_declaration >>
        ws!(tag!("->")) >>
        target_type: parse_type_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Cast(CastDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            cast_type: CastType::Implicit,
            source_type: source_type,
            target_type: target_type,
        }))
    )
);

named!(parse_explicit_cast<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(tag!("explicit")) >>
        ws!(tag!("cast")) >>
        source_type: parse_type_declaration >>
        ws!(tag!("->")) >>
        target_type: parse_type_declaration >>
        to: ws!(tag!(";")) >>
        (ItemKind::Cast(CastDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            cast_type: CastType::Explicit,
            source_type: source_type,
            target_type: target_type,
        }))
    )
);

named!(parse<NomSpan, Vec<ItemKind>>,
    many0!(
        ws!(
            alt!(
                parse_import |
                parse_sampler |
                parse_constant |
                parse_struct |
                parse_program |
                parse_function |
                parse_primitive |
                parse_operator |
                parse_implicit_cast |
                parse_explicit_cast
            )
        )
    )
);

pub fn parse_block(program: &str) -> CompileResult<Vec<BlockStatement>> {
    let input = NomSpan::new(program);
    match parse_block_statements(input) {
        IResult::Done(remaining, result) => {
            if remaining.fragment.len() > 0 {
                return Err(CompileError::new(CompileErrorKind::ParseError, Span::new(0, 0, 1, 1)));
            }
            Ok(result)
        },
        _ => Err(CompileError::new(CompileErrorKind::ParseError, Span::new(0, 0, 1, 1))),
    }
}

pub fn parse_str(program: &str) -> CompileResult<Vec<ItemKind>> {
    let input = NomSpan::new(program);
    match parse(input) {
        IResult::Done(remaining, result) => {
            if remaining.fragment.len() > 0 {
                return Err(CompileError::new(CompileErrorKind::ParseError, Span::new(0, 0, 1, 1)));
            }
            Ok(result)
        },
        _ => Err(CompileError::new(CompileErrorKind::ParseError, Span::new(0, 0, 1, 1))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_const_statement() {
        let code = "const mvp: mat4x4;";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Constant(
                    ConstantDefinition {
                        span: Span::new(0, 18, 1, 1),
                        constant_name: Identifier::new("mvp", Span::new(6, 3, 1, 7)),
                        constant_variant: ConstantVariant::Constant,
                        constant_type_name: Identifier::new("mat4x4", Span::new(11, 6, 1, 12)),
                        constant_type: None,
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_program() {
        let code = "struct VertexInput {
    position: vec4,
    color: vec4,
}

struct VertexOutput {
    position: vec4,
    color: vec4,
}

program VertexColored {
    stage vertex(in: VertexInput) -> VertexOutput {
        return VertexOutput {
            position: in.position,
            color:    in.color,
        };
    }

    stage fragment(in: VertexOutput) -> vec4 {
        return in.color;
    }
}";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Struct(
                    StructDefinition {                     
                        span: Span::new(0, 59, 1, 1),
                        struct_name: Identifier::new("VertexInput", Span::new(7, 11, 1, 8)),
                        struct_member: vec![
                            StructMemberDefinition {
                                span: Span::new(25, 14, 2, 5),
                                struct_member_name: Identifier::new("position", Span::new(25, 8, 2, 5)),                   
                                struct_member_type_name: Identifier::new("vec4", Span::new(35, 4, 2, 15)),
                                struct_member_type: None,
                            },
                            StructMemberDefinition {
                                span: Span::new(45, 11, 3, 5),
                                struct_member_name: Identifier::new("color", Span::new(45, 5, 3, 5)),
                                struct_member_type_name: Identifier::new("vec4", Span::new(52, 4, 3, 12)),
                                struct_member_type: None,
                            }
                        ],
                        declaring_type: None,
                    }
                ),
                ItemKind::Struct(
                    StructDefinition {
                        span: Span::new(61, 60, 6, 1),
                        struct_name: Identifier::new("VertexOutput", Span::new(68, 12, 6, 8)),
                        struct_member: vec![
                            StructMemberDefinition {
                                span: Span::new(87, 14, 7, 5),
                                struct_member_name: Identifier::new("position", Span::new(87, 8, 7, 5)),
                                struct_member_type_name: Identifier::new("vec4", Span::new(97, 4, 7, 15)),
                                struct_member_type: None,
                            },
                            StructMemberDefinition {
                                span: Span::new(107, 11, 8, 5),
                                struct_member_name: Identifier::new("color", Span::new(107, 5, 8, 5)),
                                struct_member_type_name: Identifier::new("vec4", Span::new(114, 4, 8, 12)),
                                struct_member_type: None,
                            }
                        ],
                        declaring_type: None,
                    }                    
                ),
                ItemKind::Program(
                    ProgramDefinition {
                        span: Span::new(123, 270, 11, 1),
                        program_name: Identifier::new("VertexColored", Span::new(131, 13, 11, 9)),
                        program_stages: vec![
                            ProgramStageDefinition {
                                span: Span::new(151, 161, 12, 5),
                                stage_name: Identifier::new("vertex", Span::new(157, 6, 12, 11)),
                                function: FunctionDeclaration {
                                    span: Span::new(151, 161, 12, 5),
                                    function_name: Identifier::new("vertex", Span::new(157, 6, 12, 11)),
                                    arguments: vec![
                                        FunctionArgumentDeclaration {
                                            span: Span::new(164, 15, 12, 18), 
                                            argument_name: Identifier::new("in", Span::new(164, 2, 12, 18)),
                                            argument_type_name: Identifier::new("VertexInput", Span::new(168, 11, 12, 22)),
                                            argument_type: None,
                                        }
                                    ],
                                    block: BlockDeclaration {
                                        span: Span::new(0, 0, 1, 1),
                                        statements: vec![
                                            BlockStatement::Return(
                                                ReturnDeclaration {
                                                    span: Span::new(207, 99, 13, 9),
                                                    expression: ExpressionStatement::StructInstantiation(
                                                        StructInstantiationExpression {
                                                            span: Span::new(214, 91, 13, 16),
                                                            struct_type_name: Identifier::new("VertexOutput", Span::new(214, 12, 13, 16)),
                                                            struct_field_initializer: vec![
                                                                StructFieldInitializerExpression {
                                                                    span: Span::new(241, 21, 14, 13),
                                                                    struct_field_name: Identifier::new("position", Span::new(241, 8, 14, 13)),
                                                                    initializer: Box::new(ExpressionStatement::FieldAccessor(
                                                                        FieldAccessorExpression {
                                                                            span: Span::new(251, 11, 14, 23),
                                                                            variable_name: Identifier::new("in", Span::new(251, 2, 14, 23)),
                                                                            field_name: Identifier::new("position", Span::new(254, 8, 14, 26)),
                                                                            field_type: None
                                                                        }
                                                                    )),
                                                                    struct_field_type: None
                                                                },
                                                                StructFieldInitializerExpression {
                                                                    span: Span::new(276, 18, 15, 13),
                                                                    struct_field_name: Identifier::new("color", Span::new(276, 5, 15, 13)),
                                                                    initializer: Box::new(ExpressionStatement::FieldAccessor(
                                                                        FieldAccessorExpression {
                                                                            span: Span::new(286, 8, 15, 23),
                                                                            variable_name: Identifier::new("in", Span::new(286, 2, 15, 23)),
                                                                            field_name: Identifier::new("color", Span::new(289, 5, 15, 26)),
                                                                            field_type: None
                                                                        }
                                                                    )),
                                                                    struct_field_type: None
                                                                }
                                                            ],
                                                            struct_type: None
                                                        }
                                                    ),
                                                    return_type: None
                                                }
                                            )
                                        ]
                                    },
                                    return_type_name: Identifier::new("VertexOutput", Span::new(184, 12, 12, 38)),
                                    return_type: None,
                                    declaring_type: None,
                                },
                                declaring_type: None,
                            },
                            ProgramStageDefinition {
                                span: Span::new(318, 73, 19, 5),
                                stage_name: Identifier::new("fragment", Span::new(324, 8, 19, 11)),
                                function: FunctionDeclaration {
                                    span: Span::new(318, 73, 19, 5),
                                    function_name: Identifier::new("fragment", Span::new(324, 8, 19, 11)),
                                    arguments: vec![
                                        FunctionArgumentDeclaration {
                                            span: Span::new(333, 16, 19, 20),
                                            argument_name: Identifier::new("in", Span::new(333, 2, 19, 20)),
                                            argument_type_name: Identifier::new("VertexOutput", Span::new(337, 12, 19, 24)),
                                            argument_type: None,
                                        }
                                    ],
                                    block: BlockDeclaration {
                                        span: Span::new(0, 0, 1, 1),
                                        statements: vec![
                                            BlockStatement::Return(
                                                ReturnDeclaration {
                                                    span: Span::new(369, 16, 20, 9),
                                                    expression: ExpressionStatement::FieldAccessor(
                                                        FieldAccessorExpression {
                                                            span: Span::new(376, 8, 20, 16),
                                                            variable_name: Identifier::new("in", Span::new(376, 2, 20, 16)),
                                                            field_name: Identifier::new("color", Span::new(379, 5, 20, 19)),
                                                            field_type: None,
                                                        }
                                                    ),
                                                    return_type: None,
                                                }
                                            )
                                        ]
                                    },
                                    return_type_name: Identifier::new("vec4", Span::new(354, 4, 19, 41)),
                                    return_type: None,
                                    declaring_type: None,
                                },
                                declaring_type: None,
                            }
                        ]
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_struct() {
        let code = "struct VertexInput { position: vec3, uv: vec2, }";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Struct(
                    StructDefinition {
                        span: Span::new(0, 48, 1, 1),
                        struct_name: Identifier::new("VertexInput", Span::new(7, 11, 1, 8)),
                        struct_member: vec![
                            StructMemberDefinition {
                                span: Span::new(21, 14, 1, 22),
                                struct_member_name: Identifier::new("position", Span::new(21, 8, 1, 22)),
                                struct_member_type_name: Identifier::new("vec3", Span::new(31, 4, 1, 32)),
                                struct_member_type: None,
                            },
                            StructMemberDefinition {
                                span: Span::new(37, 8, 1, 38),
                                struct_member_name: Identifier::new("uv", Span::new(37, 2, 1, 38)),
                                struct_member_type_name: Identifier::new("vec2", Span::new(41, 4, 1, 42)),
                                struct_member_type: None,
                            },
                        ],
                        declaring_type: None,
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_function() {
        let code = "fn main() -> f32 { return 0.0; }";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Function(
                    FunctionDeclaration {
                        span: Span::new(0, 32, 1, 1),
                        function_name: Identifier::new("main", Span::new(3, 4, 1, 4)),
                        arguments: vec![],
                        block: BlockDeclaration {
                            span: Span::empty(),
                            statements: vec![
                                BlockStatement::Return(
                                    ReturnDeclaration {
                                        span: Span::new(19, 11, 1, 20),
                                        expression: ExpressionStatement::Literal(
                                            LiteralExpression {
                                                span: Span::new(26, 3, 1, 27),
                                                value: "0.0".to_string(),
                                                literal_expression_type: LiteralType::Float,
                                                literal_type: None,
                                            }
                                        ),
                                        return_type: None,
                                    }
                                )
                            ],
                        },
                        return_type_name: Identifier::new("f32", Span::new(13, 3, 1, 14)),
                        return_type: None,
                        declaring_type: None,
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_primitive() {
        let code = "primitive type f32;";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Primitive(
                    PrimitiveDeclaration {
                        span: Span::new(0, 19, 1, 1),
                        type_name: Identifier::new("f32", Span::new(15, 3, 1, 16)),
                        declaring_type: None,
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_operator() {
        let code = "operator + (lhs: f32, rhs: f32) -> f32;";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Operator(
                    OperatorDeclaration {
                        span: Span::new(0, 39, 1, 1),
                        operator: Operator::Plus,
                        arguments: vec![
                            FunctionArgumentDeclaration {
                                span: Span::new(12, 8, 1, 13),
                                argument_name: Identifier::new("lhs", Span::new(12, 3, 1, 13)),
                                argument_type_name: Identifier::new("f32", Span::new(17, 3, 1, 18)),
                                argument_type: None,
                            },
                            FunctionArgumentDeclaration {
                                span: Span::new(22, 8, 1, 23),
                                argument_name: Identifier::new("rhs", Span::new(22, 3, 1, 23)),
                                argument_type_name: Identifier::new("f32", Span::new(27, 3, 1, 28)),
                                argument_type: None,
                            }
                        ],
                        return_type: Identifier::new("f32", Span::new(35, 3, 1, 36)),
                    }
                )
            ]
        ));
    }

    #[test]
    fn test_parse_implicit_cast() {
        let code = "implicit cast f32 -> f64;";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Cast(
                    CastDeclaration {
                        span: Span::new(0, 25, 1, 1),
                        cast_type: CastType::Implicit,
                        source_type: Identifier::new("f32", Span::new(14, 3, 1, 15)),
                        target_type: Identifier::new("f64", Span::new(21, 3, 1, 22)),
                    }
                )
            ]
        ));
    }
}
