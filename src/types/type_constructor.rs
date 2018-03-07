pub enum TypeName {
    Any,
    Fixed,
}

pub enum TypeParameterList {
    Any,
    Fixed(Vec<TypeParameter>),
    None,
}

pub enum TypeParameter {
    Any,
    Constrained,
    Fixed,
    None,
}

pub enum ValueParameterList {
    Any,
    Fixed(Vec<ValueParameter>),
    None,
}

pub enum ValueParameter {
    Any,
    Constrained,
    Fixed,
    None,
}

pub enum ConstructionResult {
    Constructor(TypeConstructor),
    Type,
    None,
}

pub struct TypeConstructor {
    type_name: TypeName,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
}

impl TypeConstructor {
    pub fn new_root() -> TypeConstructor {
        TypeConstructor {
            type_name: TypeName::Any,
            type_parameter: TypeParameterList::Any,
            value_parameter: ValueParameterList::Any,
        }
    }

    pub fn new(type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> TypeConstructor {
        TypeConstructor {
            type_name: type_name,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
        }
    }

    pub fn build(type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Result<ConstructionResult, ()> {
        // TODO check constraints

        // TODO if any parameters are open, create new constructor

        // TODO if all parameters are fixed, create new type
        unimplemented!()
    }
}
