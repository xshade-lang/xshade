use std::str;

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
pub struct ConstantDefinition {
    pub constant_name: Identifier,
    pub constant_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramDefinition {
    pub program_name: Identifier,
    pub program_bindings: Vec<ProgramBindingDefinition>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramBindingDefinition {
    pub program_binding_point: Identifier,
    pub bound_function_name: Identifier,
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
    pub argument_name: Identifier,
    pub argument_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDeclaration {
    pub function_name: Identifier,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub block: BlockDeclaration,
    pub return_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructFieldInitializerExpression {
    pub struct_field_name: Identifier,
    pub initializer: Box<ExpressionStatement>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructInstantiationExpression {
    pub struct_type_name: TypeIdentifier,
    pub struct_field_initializer: Vec<StructFieldInitializerExpression>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LiteralExpression {
    Int(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExpressionStatement {
    Literal(LiteralExpression),
    StructInstantiation(StructInstantiationExpression),
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalDeclaration {
    pub symbol_name: Identifier,
    pub expression: ExpressionStatement,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BlockStatement {
    None,
    /// e.g. a `let` statement
    Local(LocalDeclaration),
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockDeclaration {
    pub statements: Vec<BlockStatement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ItemKind {
    None,
    Struct(StructDefinition),
    Program(ProgramDefinition),
    Sampler(SamplerDefinition),
    Constant(ConstantDefinition),
    Function(FunctionDeclaration),
    Block(BlockDeclaration),
}
