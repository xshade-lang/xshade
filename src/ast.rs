use ::std::str;
use ::nom_locate::LocatedSpan;
use ::type_system::type_environment::TypeReference;

type NomSpan<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Eq, PartialEq)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize, line: usize) -> Span {
        Span {
            offset: offset,
            length: length,
            line: line,
        }
    }

    pub fn from_to(from: NomSpan, to: NomSpan) -> Span {
        Span {
            offset: from.offset,
            length: to.offset - from.offset,
            line: from.line as usize,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: &str, span: Span) -> Identifier {
        Identifier {
            name: name.to_string(),
            span: span,
        }
    }

    pub fn from_span(span: NomSpan) -> Identifier {
        Identifier {
            name: span.fragment.to_string(),
            span: Span::new(span.offset, span.fragment.len(), span.line as usize),
        }
    }
}

type TypeIdentifier = Identifier;

#[derive(Debug, Eq, PartialEq)]
pub enum ConstantVariant {
    Constant,
    Sampler,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConstantDefinition {
    pub constant_name: Identifier,
    pub constant_variant: ConstantVariant,
    pub constant_type_name: TypeIdentifier,
    pub constant_type: Option<TypeReference>,
    pub span: Span,
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
    pub declaring_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructMemberDefinition {
    pub struct_member_name: Identifier,
    pub struct_member_type_name: TypeIdentifier,
    pub struct_member_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionArgumentDeclaration {
    pub argument_name: Identifier,
    pub argument_type_name: TypeIdentifier,
    pub argument_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDeclaration {
    pub function_name: Identifier,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub block: BlockDeclaration,
    pub return_type_name: TypeIdentifier,
    pub return_type: Option<TypeReference>,
    pub declaring_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructFieldInitializerExpression {
    pub struct_field_name: Identifier,
    pub initializer: Box<ExpressionStatement>,
    pub struct_field_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructInstantiationExpression {
    pub struct_type_name: TypeIdentifier,
    pub struct_field_initializer: Vec<StructFieldInitializerExpression>,
    pub struct_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LiteralType {
    Int,
    Float,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LiteralExpression {
    pub value: String,
    pub literal_expression_type: LiteralType,
    pub literal_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct InfixExpression {
    pub operator: Operator,
    pub left_hand: Box<ExpressionStatement>,
    pub right_hand: Box<ExpressionStatement>,
    pub infix_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct VariableExpression {
    pub variable_name: Identifier,
    pub variable_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FieldAccessorExpression {
    pub variable_name: Identifier,
    pub field_name: Identifier,
    pub field_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IndexAccesorExpression {
    pub variable_name: Identifier,
    pub access_expression: Box<ExpressionStatement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExpressionStatement {
    Infix(InfixExpression),
    Literal(LiteralExpression),
    Call(CallExpression),
    StructInstantiation(StructInstantiationExpression),
    FieldAccessor(FieldAccessorExpression),
    IndexAccessor(IndexAccesorExpression),
    Variable(VariableExpression),
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalDeclaration {
    pub symbol_name: Identifier,
    pub expression: ExpressionStatement,
    pub local_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ReturnDeclaration {
    pub expression: ExpressionStatement,
    pub return_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CallExpression {
    pub function_name: Identifier,
    pub arguments: Vec<ExpressionStatement>,
    pub function_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BlockStatement {
    /// e.g. a `let` statement
    Local(LocalDeclaration),

    /// return statement
    Return(ReturnDeclaration),

    /// statement with only expressions e.g. `my_fn();`
    Expression(ExpressionStatement),
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockDeclaration {
    pub statements: Vec<BlockStatement>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PrimitiveDeclaration {
    pub type_name: Identifier,
    pub declaring_type: Option<TypeReference>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Eq, PartialEq)]
pub struct OperatorDeclaration {
    pub operator: Operator,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub return_type: TypeIdentifier, 
}

#[derive(Debug, Eq, PartialEq)]
pub enum CastType {
    Implicit,
    Explicit
}

#[derive(Debug, Eq, PartialEq)]
pub struct CastDeclaration {
    pub cast_type: CastType,
    pub source_type: TypeIdentifier,
    pub target_type: TypeIdentifier,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ItemKind {
    None,
    Struct(StructDefinition),
    Program(ProgramDefinition),
    Constant(ConstantDefinition),
    Function(FunctionDeclaration),
    Block(BlockDeclaration),
    Primitive(PrimitiveDeclaration),
    Operator(OperatorDeclaration),
    Cast(CastDeclaration),
}
