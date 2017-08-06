use ::std::error::Error;
use ::std::fmt;

pub type TypeCheckResult<T> = Result<T, TypeError>;

#[derive(Debug)]
pub enum TypeError {
    TypeNotFound(String),
    SymbolNameAlreadyUsed(String),
    SyntaxOnlyValidInCoreModule,
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
            }
        }
    }
}

impl Error for TypeError {
    fn description(&self) -> &str {
        match *self {
            TypeError::TypeNotFound(_) => "Unknown type.",
            TypeError::SymbolNameAlreadyUsed(_) => "Symbol name already declared.",
            TypeError::SyntaxOnlyValidInCoreModule => "Syntax only valid in core module.",
        }
    }
}
