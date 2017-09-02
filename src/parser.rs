use ::nom::*;
use ::nom_locate::LocatedSpan;
use ::ast::*;
use ::compile_error::CompileError;

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

named!(parse_constant<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(position!()) >>
        ws!(tag!("const")) >>
        constant_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        constant_type_name: parse_type_declaration >>
        ws!(tag!(";")) >>
        to: ws!(position!()) >>
        (ItemKind::Constant(ConstantDefinition{
            constant_name: constant_name,
            constant_variant: ConstantVariant::Constant,
            constant_type_name: constant_type_name,
            constant_type: None,
            span: Span::from_to(from, to),
        }))
    )
);

named!(parse_sampler<NomSpan, ItemKind>,
    do_parse!(
        from: ws!(position!()) >>
        ws!(tag!("sampler")) >>
        sampler_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        sampler_type_name: parse_type_declaration >>
        ws!(tag!(";")) >>
        to: ws!(position!()) >>
        (ItemKind::Constant(ConstantDefinition{
            constant_name: sampler_name,
            constant_variant: ConstantVariant::Sampler,
            constant_type_name: sampler_type_name,
            constant_type: None,
            span: Span::from_to(from, to),
        }))
    )
);

named!(parse_program_binding<NomSpan, ProgramBindingDefinition>,
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

named!(parse_program<NomSpan, ItemKind>,
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

named!(parse_struct_member<NomSpan, StructMemberDefinition>,
    do_parse!(
        struct_member_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        struct_member_type_name: parse_type_declaration >>
        (StructMemberDefinition{
            struct_member_name: struct_member_name,
            struct_member_type_name: struct_member_type_name,
            struct_member_type: None,
        })
    )
);

named!(parse_struct<NomSpan, ItemKind>,
    do_parse!(
        ws!(tag!("struct")) >>
        struct_name: parse_symbol_declaration >>
        ws!(tag!("{")) >>
        member: ws!(separated_list!(tag!(","), parse_struct_member)) >>
        opt!(ws!(tag!(","))) >>
        ws!(tag!("}")) >>
        (ItemKind::Struct(StructDefinition{
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
            argument_name: argument_name,
            argument_type_name: argument_type_name,
            argument_type: None,
        })
    )
);

named!(parse_symbol_declaration<NomSpan, Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_span(name))
    )
);

named!(parse_type_declaration<NomSpan, Identifier>,
    do_parse!(
        name: ws!(parse_identifier) >>
        (Identifier::from_span(name))
    )
);

named!(parse_struct_instantiation_field_initializer<NomSpan, StructFieldInitializerExpression>,
    do_parse!(
        struct_field_name: parse_symbol_declaration >>
        ws!(tag!(":")) >>
        initializer: parse_expression >>
        (StructFieldInitializerExpression{
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
        ws!(tag!("}")) >>
        (ExpressionStatement::StructInstantiation(StructInstantiationExpression{
            struct_type_name: struct_type_name,
            struct_field_initializer: struct_field_initializer,
            struct_type: None,
        }))
    )
);

fn parse_int_literal(parts: Vec<char>) -> ExpressionStatement {
    let string: String = parts.into_iter().collect();
    ExpressionStatement::Literal(LiteralExpression {
                                     value: string,
                                     literal_expression_type: LiteralType::Int,
                                     literal_type: None,
                                 })
}

fn parse_float_literal(before: Vec<char>, after: Vec<char>) -> ExpressionStatement {
    let mut before: String = before.into_iter().collect();
    let after: String = after.into_iter().collect();
    before.push_str(".");
    before.push_str(&after);
    ExpressionStatement::Literal(LiteralExpression {
                                     value: before,
                                     literal_expression_type: LiteralType::Float,
                                     literal_type: None,
                                 })
}

named!(parse_float_literal_expression<NomSpan, ExpressionStatement>,
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

named!(parse_int_literal_expression<NomSpan, ExpressionStatement>,
    do_parse!(
        numbers: ws!(many1!(
            one_of!("0123456789")
        )) >>
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
        ws!(tag!(")")) >>
        (CallExpression {
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
        ws!(tag!("let")) >>
        symbol_name: parse_symbol_declaration >>
        ws!(tag!("=")) >>
        expression: parse_expression >>
        ws!(tag!(";")) >>
        (BlockStatement::Local(
            LocalDeclaration{
                symbol_name: symbol_name,
                expression: expression,
                local_type: None,
            }
        ))
    )
);

named!(parse_return_declaration<NomSpan, BlockStatement>,
    do_parse!(
        ws!(tag!("return")) >>
        expression: parse_expression >>
        ws!(tag!(";")) >>
        (BlockStatement::Return(ReturnDeclaration{
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
            statements: statements,
        })
    )
);

// TODO make return type optional
named!(parse_function<NomSpan, ItemKind>,
    do_parse!(
        ws!(tag!("fn")) >>
        function_name: parse_symbol_declaration >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type_name: parse_type_declaration >>
        ws!(tag!("{")) >>
        block: parse_block_declaration >>
        ws!(tag!("}")) >>
        (ItemKind::Function(FunctionDeclaration{
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
        ws!(tag!("primitive")) >>
        ws!(tag!("type")) >>
        type_name: parse_symbol_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Primitive(PrimitiveDeclaration{
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
        ws!(tag!("operator")) >>
        operator: parse_operator_type >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("->")) >>
        return_type: parse_type_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Operator(OperatorDeclaration{
            operator: operator,
            arguments: arguments,
            return_type: return_type,
        }))
    )
);

named!(parse_implicit_cast<NomSpan, ItemKind>,
    do_parse!(
        ws!(tag!("implicit")) >>
        ws!(tag!("cast")) >>
        source_type: parse_type_declaration >>
        ws!(tag!("->")) >>
        target_type: parse_type_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Cast(CastDeclaration{
            cast_type: CastType::Implicit,
            source_type: source_type,
            target_type: target_type,
        }))
    )
);

named!(parse_explicit_cast<NomSpan, ItemKind>,
    do_parse!(
        ws!(tag!("explicit")) >>
        ws!(tag!("cast")) >>
        source_type: parse_type_declaration >>
        ws!(tag!("->")) >>
        target_type: parse_type_declaration >>
        ws!(tag!(";")) >>
        (ItemKind::Cast(CastDeclaration{
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

pub fn parse_block(program: &str) -> Result<Vec<BlockStatement>, CompileError> {
    let input = NomSpan::new(program);
    match parse_block_statements(input) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(CompileError::new()),
    }
}

pub fn parse_str(program: &str) -> Result<Vec<ItemKind>, CompileError> {
    let input = NomSpan::new(program);
    match parse(input) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(CompileError::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_primitive_statement() {
        let code = "const mvp: mat4x4;";

        assert_eq!(parse_str(code), Ok(
            vec![
                ItemKind::Constant(
                    ConstantDefinition {
                        constant_name: Identifier::new("mvp", Span::new(6, 3, 1)),
                        constant_variant: ConstantVariant::Constant,
                        constant_type_name: Identifier::new("mat4x4", Span::new(11, 6, 1)),
                        constant_type: None,
                        span: Span::new(0, 18, 1)
                    }
                )
            ]
        ));
    }
}
