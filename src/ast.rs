use std::str;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String
}

impl Identifier {
    pub fn from_u8_slice(v: &[u8]) -> Identifier {
        Identifier {
            name: str::from_utf8(v).unwrap().to_string(),
        }
    }

    pub fn from_str(v: &str) -> Identifier {
        Identifier {
            name: v.to_string(),
        }
    }
}

type TypeIdentifier = Identifier;

#[derive(Debug, Eq, PartialEq)]
pub struct SamplerDefinition {
    pub sampler_name: Identifier,
    pub sampler_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructDefinition {
    pub struct_name: Identifier,
    pub struct_member: Vec<StructMemberDefinition>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructMemberDefinition {
    pub struct_member_name: Identifier,
    pub struct_member_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionArgumentDeclaration {
    name: Identifier,
    type_name: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDeclaration {
    name: Identifier,
    arguments: Vec<FunctionArgumentDeclaration>,
    return_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ItemKind {
    None,
    Struct(StructDefinition),
    Sampler(SamplerDefinition),
    Function(FunctionDeclaration),
}
