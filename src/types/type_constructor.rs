pub enum ConstructionType {
    Any,
    Product,
    Function,
    Value,
}

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
    construction_type: ConstructionType,
    type_name: TypeName,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
}

impl TypeConstructor {
    pub fn new_root() -> TypeConstructor {
        TypeConstructor {
            construction_type: ConstructionType::Any,
            type_name: TypeName::Any,
            type_parameter: TypeParameterList::Any,
            value_parameter: ValueParameterList::Any,
        }
    }

    pub fn new(construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> TypeConstructor {
        TypeConstructor {
            construction_type: construction_type,
            type_name: type_name,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
        }
    }

    pub fn build(&mut self, construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Result<ConstructionResult, ()> {
        // TODO check constraints

        // if self.construction_type == ConstructionType::Any
            // create or delegate to child TypeConstructor(construction_type, Any, Any, Any)
        
        // if self.type_name == TypeName::Any
            // create or delegate to child TypeConstructor(construction_type, Fixed(type_name), Any, Any)
        
        // if self.type_parameter == TypeParameterList::Any
            // create or delegate to child TypeConstructor(construction_type, Fixed(type_name), ..., Any)

        // if self.value_parameter == ValueParameterList::Any
            // create or delegate to child TypeConstructor(construction_type, Fixed(type_name), ..., ...)

        unimplemented!()
    }

    fn get_or_create_child_type_constructor(construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> &TypeConstructor {

    }

    fn get_or_create_child_type_constructor_mut(construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> &mut TypeConstructor {

    }
}
