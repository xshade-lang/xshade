use ::std;
use ::nom::*;
use ::ast::*;

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
        name: ws!(parse_identifier) >>
        ws!(tag!(":")) >>
        type_name: ws!(parse_identifier) >>
        ws!(tag!(";")) >>
        (ItemKind::Sampler(SamplerDefinition{ sampler_name: Identifier::from_u8_slice(name), sampler_type: Identifier::from_u8_slice(type_name) }))
    )
);

named!(parse_struct_member<&[u8], StructMemberDefinition>,
    do_parse!(
        name: ws!(parse_identifier) >>
        ws!(tag!(":")) >>
        type_name: ws!(parse_identifier) >>
        ws!(tag!(",")) >>
        (StructMemberDefinition{ struct_member_name: Identifier::from_u8_slice(name), struct_member_type: Identifier::from_u8_slice(type_name) })
    )
);

named!(parse_struct<&[u8], ItemKind>,
    do_parse!(
        ws!(tag!("struct")) >>
        name: ws!(parse_identifier) >>
        ws!(tag!("{")) >>
        member: ws!(many0!(parse_struct_member)) >>
        ws!(tag!("}")) >>
        (ItemKind::Struct(StructDefinition{ struct_name: Identifier::from_u8_slice(name), struct_member: member }))
    )
);

named!(parse<&[u8], Vec<ItemKind>>,
    many0!(
        ws!(
            alt!(
                parse_sampler |
                parse_struct
            )
        )
    )
);

pub fn parse_str(program: &str) {
    parse(program.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sampler() {
        let sampler_code = "sampler albedo: Sampler2d;";
        assert_eq!(
            parse(sampler_code.as_bytes()),
            IResult::Done(&b""[..], vec![
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
            parse(sampler_code.as_bytes()),
            IResult::Done(&b""[..], vec![
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
