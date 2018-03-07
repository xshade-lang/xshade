use ::std::error::Error;
use ::std::fmt;

#[derive(Debug)]
pub struct TypeConstructionError;

impl Error for TypeConstructionError {
    fn description(&self) -> &str {
        "Error constructing a type"
    }
}

impl fmt::Display for TypeConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Error constructing a type")
    }
}
