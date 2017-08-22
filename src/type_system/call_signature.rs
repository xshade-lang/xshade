use ::type_system::type_environment::TypeReference;
use ::type_system::error::{ TypeError, TypeCheckResult };

#[derive(Debug, Eq, PartialEq)]
pub struct CallSignature {
    arguments: Vec<TypeReference>,
    return_type: Option<TypeReference>,
}

impl CallSignature {
    pub fn new(arguments: Vec<TypeReference>, return_type: Option<TypeReference>) -> CallSignature {
        CallSignature {
            arguments: arguments,
            return_type: return_type,
        }
    }

    pub fn match_arguments(&self, arguments: Vec<TypeReference>) -> bool {
        self.arguments == arguments
    }

    pub fn match_arguments_or_err(&self, arguments: Vec<TypeReference>) -> TypeCheckResult<()> {
        if self.arguments == arguments {
            Ok(())
        }else{
            Err(TypeError::IncompatibleArguments)
        }
    }

    pub fn match_return_type(&self, return_type: Option<TypeReference>) -> bool {
        self.return_type == return_type
    }

    pub fn get_return_type(&self) -> Option<TypeReference> {
        match self.return_type {
            Some(t) => Some(t),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn same_call_signatures_are_equal() {
        let a = CallSignature::new(vec![TypeReference::new(0), TypeReference::new(1)], Some(TypeReference::new(2)));
        let b = CallSignature::new(vec![TypeReference::new(0), TypeReference::new(1)], Some(TypeReference::new(2)));

        assert_eq!(a == b, true);
    }

    #[test]
    fn different_call_signatures_are_unequal() {
        let a = CallSignature::new(vec![TypeReference::new(1), TypeReference::new(0)], Some(TypeReference::new(2)));
        let b = CallSignature::new(vec![TypeReference::new(0), TypeReference::new(1)], Some(TypeReference::new(2)));

        assert_eq!(a == b, false);
    }

    #[test]
    fn test_match_arguments() {
        let signature = CallSignature::new(vec![TypeReference::new(1), TypeReference::new(0)], Some(TypeReference::new(2)));

        assert!(signature.match_arguments(vec![TypeReference::new(1), TypeReference::new(0)]));
    }

    #[test]
    fn test_match_return_type() {
        let signature = CallSignature::new(vec![TypeReference::new(1), TypeReference::new(0)], Some(TypeReference::new(2)));

        assert!(signature.match_return_type(Some(TypeReference::new(2))));
    }
}
