use ::ast::Span;
use ::type_system::call_signature::CallSignature;
use ::type_system::structure_members::StructureMembers;
use ::type_system::error::{ TypeError, ErrorKind, TypeCheckResult };
use ::type_system::type_environment::TypeReference;

#[derive(Debug, Eq)]
pub struct TypeDefinition {
    id: usize,
    name: String,
    implicit_casts: Vec<TypeReference>,
    explicit_casts: Vec<TypeReference>,

    call_signature: Option<CallSignature>,
    // member: Option<Vec<TypeReference>>,
    member: Option<StructureMembers>,
}

impl TypeDefinition {
    pub fn new(id: usize, name: &str) -> TypeDefinition {
        TypeDefinition {
            id: id,
            name: name.to_string(),
            implicit_casts: Vec::new(),
            explicit_casts: Vec::new(),
            call_signature: None,
            member: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn find_member_type(&self, field_name: &str) -> Option<TypeReference> {
        if let Some(ref member) = self.member {
            member.find_member_type(field_name)
        } else {
            None
        }
    }

    pub fn has_member(&self) -> bool {
        match self.member {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_member(&self) -> Option<&StructureMembers> {
        match self.member {
            Some(ref m) => Some(&m),
            None => None,
        }
    }

    pub fn set_members(&mut self, members: StructureMembers) -> TypeCheckResult<()> {
        if self.has_member() {
            // TODO error if already set
        }

        self.member = Some(members);

        Ok(())
    }

    pub fn is_struct(&self) -> bool {
        match self.get_member() {
            Some(_) => true,
            None    => false
        }
    }

    pub fn make_callable(&mut self, signature: CallSignature) -> TypeCheckResult<()> {
        if self.is_callable() {
            return Err(TypeError::new(Span::empty(), ErrorKind::CannotMakeCallable));
        }

        self.call_signature = Some(signature);
        Ok(())
    }

    pub fn get_call_signature(&self) -> Option<&CallSignature> {
        match self.call_signature {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn get_call_signature_or_err(&self) -> TypeCheckResult<&CallSignature> {
        match self.call_signature {
            Some(ref s) => Ok(s),
            None => Err(TypeError::new(Span::empty(), ErrorKind::NotCallable)),
        }
    }

    pub fn is_callable(&self) -> bool {
        match self.call_signature {
            Some(_) => true,
            None => false,
        }
    }

    pub fn does_cast_exist(&self, other: TypeReference) -> bool {
        self.implicit_casts.iter().any(|&t| t == other) ||
        self.explicit_casts.iter().any(|&t| t == other)
    }

    pub fn does_implicit_cast_exist(&self, other: TypeReference) -> bool {
        self.implicit_casts.iter().any(|&t| t == other)
    }

    pub fn does_explicit_cast_exist(&self, other: TypeReference) -> bool {
        self.explicit_casts.iter().any(|&t| t == other)
    }

    pub fn add_implicit_cast(&mut self, other: TypeReference) {
        self.implicit_casts.push(other);
    }

    pub fn add_explicit_cast(&mut self, other: TypeReference) {
        self.explicit_casts.push(other);
    }
}

impl PartialEq for TypeDefinition {
    fn eq(&self, other: &TypeDefinition) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    use super::*;

    #[test]
    fn same_types_are_equal() {
        let a = TypeDefinition::new(0, "f32");
        let b = TypeDefinition::new(0, "f32");

        assert_eq!(a, b);
    }
}
