use ::parse_tree::*;
use ::string_builder::StringBuilder;

fn render_struct(structure_definition: &StructDefinition, sb: &mut StringBuilder) {
    sb.append("struct ");
    sb.append(&structure_definition.struct_name.name);
    sb.append(" {\n");
    for member in &structure_definition.struct_member {
        sb.append("    ");
        sb.append(&member.struct_member_type.name);
        sb.append(" ");
        sb.append(&member.struct_member_name.name);
        sb.append(";\n");
    }
    sb.append("}\n");
}

pub fn render_glsl(ast: &Vec<ItemKind>) -> String {
    let mut sb = StringBuilder::new(1024);
    sb.append("#version 330\n");
    for item in ast {
        match item {
            &ItemKind::Struct(ref structure_definition) => {
                render_struct(structure_definition, &mut sb);
            },
            _ => {},
        }
    }

    sb.to_string().unwrap_or("".to_string())
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn render_struct() {
        let ast = vec![ItemKind::Struct(StructDefinition {
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
        })];

        assert_eq!("#version 330\nstruct VS_IN {\n    vec3 position;\n    vec2 uv;\n}\n", render_glsl(&ast));
    }
}
