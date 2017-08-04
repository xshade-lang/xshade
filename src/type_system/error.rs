use ::std::error::Error;
use ::std::fmt;

pub type TypeCheckResult = Result<(), TypeError>;


#[derive(Debug)]
pub enum TypeError {
    TypeNotFound(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeError::TypeNotFound(ref type_name) => {
                write!(f, "Unknown Type \"{}\"", type_name)
            },
        }
    }
}

impl Error for TypeError {
    fn description(&self) -> &str {
        match *self {
            TypeError::TypeNotFound(ref e) => "Unknown Type",
        }
    }
}
