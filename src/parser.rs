use ::nom::*;
use ::parse_tree::*;

named!(parse_identifier<&[u8], &[u8]>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
            ()
        )
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

named!(parse_function<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("fn")) >>
        name: ws!(parse_identifier) >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_function_argument)) >>
        ws!(tag!(")")) >>
        ws!(tag!("{")) >>
        ws!(tag!("}")) >>
        (ItemKind::Function(FunctionDeclaration{ function_name: Identifier::from_u8_slice(name), arguments: arguments, return_type: Identifier::from_str("void"), }))
    )
);

named!(parse<&[u8], Vec<ItemKind>>,
    many0!(
        ws!(
            alt!(
                parse_sampler |
                parse_struct |
                parse_function
            )
        )
    )
);

pub fn parse_bytes(program: &[u8]) -> Result<Vec<ItemKind>, ()> {
    match parse(program) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(())
    }
}

pub fn parse_str(program: &str) -> Result<Vec<ItemKind>, ()> {
    match parse(program.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        _ => Err(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn parse_empty_function() {
        let empty_function = "fn main() {}";
        assert_eq!(
            parse_str(empty_function),
            Ok(vec![
                ItemKind::Function(FunctionDeclaration{
                    function_name: Identifier::from_str("main"),
                    arguments: Vec::new(),
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_empty_function_with_arguments() {
        let empty_function = "fn main(a: B, c: D) {}";
        assert_eq!(
            parse_str(empty_function),
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
                    return_type: Identifier::from_str("void"),
                })
            ])
        );
    }

    #[test]
    fn parse_sampler() {
        let sampler_code = "sampler albedo: Sampler2d;";
        assert_eq!(
            parse_str(sampler_code),
            Ok(vec![
                ItemKind::Sampler(SamplerDefinition {
                    sampler_name: Identifier::from_str("albedo"),
                    sampler_type: Identifier::from_str("Sampler2d"),
                })
            ])
        );
    }

    #[test]
    fn parse_struct() {
        let sampler_code = r#"
            struct VS_IN {
                position: vec3,
                uv: vec2,
            }
        "#;
        assert_eq!(
            parse_str(sampler_code),
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
}
