use ::std::str;
use ::nom_locate::LocatedSpan;
use ::type_system::type_environment::TypeReference;

type NomSpan<'a> = LocatedSpan<&'a str>;

pub trait Spanned {
    fn get_span(&self) -> Span;
}

macro_rules! impl_spanned {
    ($t:ty) => (
        impl Spanned for $t {
            fn get_span(&self) -> Span {
                self.span
            }
        }
    )
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize, line: usize, column: usize) -> Span {
        Span {
            offset: offset,
            length: length,
            line: line,
            column: column,
        }
    }

    pub fn from_nom_span(span: &NomSpan) -> Span {
        Span {
            offset: span.offset,
            length: span.fragment.len(),
            line: span.line as usize,
            column: span.get_column(), // TODO get_column_utf8 ?
        }
    }

    pub fn from_to(from: Span, to: Span) -> Span {
        Span {
            offset: from.offset,
            length: to.offset - from.offset + to.length,
            line: from.line,
            column: from.column,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Identifier {
    pub span: Span,
    pub name: String,
}

impl_spanned!(Identifier);

impl Identifier {
    pub fn new(name: &str, span: Span) -> Identifier {
        Identifier {
            span: span,
            name: name.to_string(),
        }
    }

    pub fn from_nom_span(span: NomSpan) -> Identifier {
        Identifier {
            span: Span::new(span.offset, span.fragment.len(), span.line as usize, span.get_column() as usize),
            name: span.fragment.to_string(),
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
    pub span: Span,
    pub constant_name: Identifier,
    pub constant_variant: ConstantVariant,
    pub constant_type_name: TypeIdentifier,
    pub constant_type: Option<TypeReference>,
}

impl_spanned!(ConstantDefinition);

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramDefinition {
    pub span: Span,
    pub program_name: Identifier,
    pub program_bindings: Vec<ProgramBindingDefinition>,
}

impl_spanned!(ProgramDefinition);

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramBindingDefinition {
    pub span: Span,
    pub program_binding_point: Identifier,
    pub bound_function_name: Identifier,
}

impl_spanned!(ProgramBindingDefinition);

#[derive(Debug, Eq, PartialEq)]
pub struct StructDefinition {
    pub span: Span,
    pub struct_name: Identifier,
    pub struct_member: Vec<StructMemberDefinition>,
    pub declaring_type: Option<TypeReference>,
}

impl_spanned!(StructDefinition);

#[derive(Debug, Eq, PartialEq)]
pub struct StructMemberDefinition {
    pub span: Span,
    pub struct_member_name: Identifier,
    pub struct_member_type_name: TypeIdentifier,
    pub struct_member_type: Option<TypeReference>,
}

impl_spanned!(StructMemberDefinition);

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionArgumentDeclaration {
    pub span: Span,
    pub argument_name: Identifier,
    pub argument_type_name: TypeIdentifier,
    pub argument_type: Option<TypeReference>,
}

impl_spanned!(FunctionArgumentDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub function_name: Identifier,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub block: BlockDeclaration,
    pub return_type_name: TypeIdentifier,
    pub return_type: Option<TypeReference>,
    pub declaring_type: Option<TypeReference>,
}

impl_spanned!(FunctionDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub struct StructFieldInitializerExpression {
    pub span: Span,
    pub struct_field_name: Identifier,
    pub initializer: Box<ExpressionStatement>,
    pub struct_field_type: Option<TypeReference>,
}

impl_spanned!(StructFieldInitializerExpression);

#[derive(Debug, Eq, PartialEq)]
pub struct StructInstantiationExpression {
    pub span: Span,
    pub struct_type_name: TypeIdentifier,
    pub struct_field_initializer: Vec<StructFieldInitializerExpression>,
    pub struct_type: Option<TypeReference>,
}

impl_spanned!(StructInstantiationExpression);

#[derive(Debug, Eq, PartialEq)]
pub enum LiteralType {
    Int,
    Float,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LiteralExpression {
    pub span: Span,
    pub value: String,
    pub literal_expression_type: LiteralType,
    pub literal_type: Option<TypeReference>,
}

impl_spanned!(LiteralExpression);

#[derive(Debug, Eq, PartialEq)]
pub struct InfixExpression {
    pub span: Span,
    pub operator: Operator,
    pub left_hand: Box<ExpressionStatement>,
    pub right_hand: Box<ExpressionStatement>,
    pub infix_type: Option<TypeReference>,
}

impl_spanned!(InfixExpression);

#[derive(Debug, Eq, PartialEq)]
pub struct VariableExpression {
    pub span: Span,
    pub variable_name: Identifier,
    pub variable_type: Option<TypeReference>,
}

impl_spanned!(VariableExpression);

#[derive(Debug, Eq, PartialEq)]
pub struct FieldAccessorExpression {
    pub span: Span,
    pub variable_name: Identifier,
    pub field_name: Identifier,
    pub field_type: Option<TypeReference>,
}

impl_spanned!(FieldAccessorExpression);

#[derive(Debug, Eq, PartialEq)]
pub struct IndexAccesorExpression {
    pub span: Span,
    pub variable_name: Identifier,
    pub access_expression: Box<ExpressionStatement>,
}

impl_spanned!(IndexAccesorExpression);

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

impl Spanned for ExpressionStatement {
    fn get_span(&self) -> Span {
        match *self {
            ExpressionStatement::Infix(ref expression) => expression.span,
            ExpressionStatement::Literal(ref expression) => expression.span,
            ExpressionStatement::Call(ref expression) => expression.span,
            ExpressionStatement::StructInstantiation(ref expression) => expression.span,
            ExpressionStatement::FieldAccessor(ref expression) => expression.span,
            ExpressionStatement::IndexAccessor(ref expression) => expression.span,
            ExpressionStatement::Variable(ref expression) => expression.span,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalDeclaration {
    pub span: Span,
    pub symbol_name: Identifier,
    pub expression: ExpressionStatement,
    pub local_type: Option<TypeReference>,
}

impl_spanned!(LocalDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub struct ReturnDeclaration {
    pub span: Span,
    pub expression: ExpressionStatement,
    pub return_type: Option<TypeReference>,
}

impl_spanned!(ReturnDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub struct CallExpression {
    pub span: Span,
    pub function_name: Identifier,
    pub arguments: Vec<ExpressionStatement>,
    pub function_type: Option<TypeReference>,
}

impl_spanned!(CallExpression);

#[derive(Debug, Eq, PartialEq)]
pub enum BlockStatement {
    /// e.g. a `let` statement
    Local(LocalDeclaration),

    /// return statement
    Return(ReturnDeclaration),

    /// statement with only expressions e.g. `my_fn();`
    Expression(ExpressionStatement),
}

impl Spanned for BlockStatement {
    fn get_span(&self) -> Span {
        match *self {
            BlockStatement::Local(ref statement) => statement.span,
            BlockStatement::Return(ref statement) => statement.span,
            BlockStatement::Expression(ref statement) => statement.get_span(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockDeclaration {
    pub span: Span,
    pub statements: Vec<BlockStatement>,
}

impl_spanned!(BlockDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub struct PrimitiveDeclaration {
    pub span: Span,
    pub type_name: Identifier,
    pub declaring_type: Option<TypeReference>,
}

impl_spanned!(PrimitiveDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

// TODO type check
#[derive(Debug, Eq, PartialEq)]
pub struct OperatorDeclaration {
    pub span: Span,
    pub operator: Operator,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub return_type: TypeIdentifier, 
}

impl_spanned!(OperatorDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub enum CastType {
    Implicit,
    Explicit
}

// TODO type check
#[derive(Debug, Eq, PartialEq)]
pub struct CastDeclaration {
    pub span: Span,
    pub cast_type: CastType,
    pub source_type: TypeIdentifier,
    pub target_type: TypeIdentifier,
}

impl_spanned!(CastDeclaration);

#[derive(Debug, Eq, PartialEq)]
pub enum ItemKind {
    Struct(StructDefinition),
    Program(ProgramDefinition),
    Constant(ConstantDefinition),
    Function(FunctionDeclaration),
    Block(BlockDeclaration),
    Primitive(PrimitiveDeclaration),
    Operator(OperatorDeclaration),
    Cast(CastDeclaration),
}

impl Spanned for ItemKind {
    fn get_span(&self) -> Span {
        match *self {
            ItemKind::Struct(ref item) => item.span,
            ItemKind::Program(ref item) => item.span,
            ItemKind::Constant(ref item) => item.span,
            ItemKind::Function(ref item) => item.span,
            ItemKind::Block(ref item) => item.span,
            ItemKind::Primitive(ref item) => item.span,
            ItemKind::Operator(ref item) => item.span,
            ItemKind::Cast(ref item) => item.span,
        }
    }
}
