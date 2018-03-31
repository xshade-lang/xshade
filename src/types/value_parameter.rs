#[derive(Debug, Eq, PartialEq)]
pub enum ValueParameterList {
    Fixed(Vec<ValueParameter>),
    None,
}

impl Clone for ValueParameterList {
    fn clone(&self) -> ValueParameterList {
        match *self {
            ValueParameterList::Any => ValueParameterList::Any,
            ValueParameterList::Fixed(ref l) => ValueParameterList::Fixed(l.to_vec()),
            ValueParameterList::None => ValueParameterList::None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValueParameter {
    Any,
    Fixed(u32),
    Constrained(ValueConstraint),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValueConstraint {
    Many(Vec<ValueConstraint>),
    LessThan(u32),
    EqualTo(u32),
    GreaterThan(u32),
    LessOrEqualTo(u32),
    GreaterOrEqualTo(u32),
}
