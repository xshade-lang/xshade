use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;

pub type TypeCheckResult<T> = Result<T, TypeError>;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    TypeNotFound(String),
    SymbolNameAlreadyUsed(String),
    SyntaxOnlyValidInCoreModule,
    CastAlreadyDeclared(String, String),
    VariableNotFound(String),
    CannotInfer(String),
    IncompatibleTypes(String, String),
    CannotMakeCallable,
    NotCallable,
    IncompatibleArguments,
    TypeHasNoMember,
    MemberNotFound,
    CannotInstantiateStructWithArguments,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypeError {
    span: Span,
    kind: ErrorKind,
}

impl TypeError {
    pub fn new(span: Span, kind: ErrorKind) -> TypeError {
        TypeError {
            span: span,
            kind: kind,
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::TypeNotFound(ref type_name) => {
                write!(f, "Unknown type \"{}\".", type_name)
            },
            ErrorKind::SymbolNameAlreadyUsed(ref symbol_name) => {
                write!(f, "Symbol \"{}\" already declared.", symbol_name)
            },
            ErrorKind::SyntaxOnlyValidInCoreModule => {
                write!(f, "Syntax only valid in core module.")
            },
            ErrorKind::CastAlreadyDeclared(ref from, ref to) => {
                write!(f, "Cast from \"{}\" to \"{}\" already declared.", from, to)
            },
            ErrorKind::VariableNotFound(ref variable_name) => {
                write!(f, "Unknown variable \"{}\".", variable_name)
            },
            ErrorKind::CannotInfer(ref variable_name) => {
                write!(f, "Cannot infer type for variable \"{}\".", variable_name)
            },
            ErrorKind::IncompatibleTypes(ref left, ref right) => {
                write!(f, "Incompatible types \"{}\" and \"{}\".", left, right)
            },
            ErrorKind::CannotMakeCallable => {
                write!(f, "Cannot make type callable.")
            },
            ErrorKind::NotCallable => {
                write!(f, "Not callable.")
            },
            ErrorKind::IncompatibleArguments => {
                write!(f, "Arguments incompatible.")
            },
            ErrorKind::TypeHasNoMember => {
                write!(f, "Type has no member.")
            },
            ErrorKind::MemberNotFound => {
                write!(f, "Member not found.")
            },
            ErrorKind::CannotInstantiateStructWithArguments => {
                write!(f, "Cannot instantiate structure with given arguments.")
            },
        }
    }
}

impl Error for TypeError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::TypeNotFound(_) => "Unknown type.",
            ErrorKind::SymbolNameAlreadyUsed(_) => "Symbol name already declared.",
            ErrorKind::SyntaxOnlyValidInCoreModule => "Syntax only valid in core module.",
            ErrorKind::CastAlreadyDeclared(_, _) => "Cast already declared.",
            ErrorKind::VariableNotFound(_) => "Unknown type.",
            ErrorKind::CannotInfer(_) => "Cannot infer type.",
            ErrorKind::IncompatibleTypes(_, _) => "Incompatible types.",
            ErrorKind::CannotMakeCallable => "Cannot make type callable.",
            ErrorKind::NotCallable => "Not callable.",
            ErrorKind::IncompatibleArguments => "Arguments incompatible.",
            ErrorKind::TypeHasNoMember => "Type has no member.",
            ErrorKind::MemberNotFound => "Member not found.",
            ErrorKind::CannotInstantiateStructWithArguments => "Cannot instantiate structure with given arguments.",
        }
    }
}
