use ::std::error::Error;
use ::std::fmt;

pub type TypeCheckResult<T> = Result<T, TypeError>;

#[derive(Debug)]
pub enum TypeError {
    TypeNotFound(String),
    SymbolNameAlreadyUsed(String),
    SyntaxOnlyValidInCoreModule,
    CastAlreadyDeclared(String, String),
    VariableNotFound(String),
    CannotInfer(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeError::TypeNotFound(ref type_name) => {
                write!(f, "Unknown type \"{}\".", type_name)
            },
            TypeError::SymbolNameAlreadyUsed(ref symbol_name) => {
                write!(f, "Symbol \"{}\" already declared.", symbol_name)
            },
            TypeError::SyntaxOnlyValidInCoreModule => {
                write!(f, "Syntax only valid in core module.")
            },
            TypeError::CastAlreadyDeclared(ref from, ref to) => {
                write!(f, "Cast from \"{}\" to \"{}\" already declared.", from, to)
            },
            TypeError::VariableNotFound(ref variable_name) => {
                write!(f, "Unknown variable \"{}\".", variable_name)
            },
            TypeError::CannotInfer(ref variable_name) => {
                write!(f, "Cannot infer type for variable \"{}\".", variable_name)
            },
        }
    }
}

impl Error for TypeError {
    fn description(&self) -> &str {
        match *self {
            TypeError::TypeNotFound(_) => "Unknown type.",
            TypeError::SymbolNameAlreadyUsed(_) => "Symbol name already declared.",
            TypeError::SyntaxOnlyValidInCoreModule => "Syntax only valid in core module.",
            TypeError::CastAlreadyDeclared(_, _) => "Cast already declared.",
            TypeError::VariableNotFound(_) => "Unknown type.",
            TypeError::CannotInfer(_) => "Cannot infer type.",
        }
    }
}
