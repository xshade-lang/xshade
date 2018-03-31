#[derive(Debug, Eq, PartialEq)]
pub enum TypeParameterList {
    Fixed(Vec<TypeParameter>),
    None,
}

impl Clone for TypeParameterList {
    fn clone(&self) -> TypeParameterList {
        match *self {
            TypeParameterList::Any => TypeParameterList::Any,
            TypeParameterList::Fixed(ref l) => TypeParameterList::Fixed(l.to_vec()),
            TypeParameterList::None => TypeParameterList::None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeParameter {
    Any,
    Fixed(String),
    Constrained(TypeConstraint),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeConstraint {
    Many(Vec<TypeConstraint>),
    SubtypeOf(String),
    ImplementsTrait(String),
}
