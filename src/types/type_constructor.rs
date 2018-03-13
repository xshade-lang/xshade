use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TypeRelation {
    Subtype,
    Supertype,
    Equivalent,
    None,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ConstructionType {
    Any,
    Product,
    Function,
    Value,
}

impl ConstructionType {
    pub fn get_type_relation(&self, other: ConstructionType) -> TypeRelation {
        match *self {
            ConstructionType::Any => match other {
                ConstructionType::Any => TypeRelation::Equivalent,
                ConstructionType::Product => TypeRelation::Subtype,
                ConstructionType::Function => TypeRelation::Subtype,
                ConstructionType::Value => TypeRelation::Subtype,
            },
            ConstructionType::Product => match other {
                ConstructionType::Any => TypeRelation::Supertype,
                ConstructionType::Product => TypeRelation::Equivalent,
                ConstructionType::Function => TypeRelation::None,
                ConstructionType::Value => TypeRelation::None,
            },
            ConstructionType::Function => match other {
                ConstructionType::Any => TypeRelation::Supertype,
                ConstructionType::Product => TypeRelation::None,
                ConstructionType::Function => TypeRelation::Equivalent,
                ConstructionType::Value => TypeRelation::None,
            },
            ConstructionType::Value => match other {
                ConstructionType::Any => TypeRelation::Supertype,
                ConstructionType::Product => TypeRelation::None,
                ConstructionType::Function => TypeRelation::None,
                ConstructionType::Value => TypeRelation::Equivalent,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    Any,
    Fixed(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeParameterList {
    Any,
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
    Constrained,
    Fixed(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValueParameterList {
    Any,
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
    Constrained,
    Fixed(u32),
}

pub struct Type {
    type_name: TypeName,
    construction_type: ConstructionType,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
    subtypes: Vec<Type>,
}


impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.subtypes.len() == 0 {
            write!(f, "Type({:?}, {:?}, {:?}, {:?})", 
                self.construction_type,
                self.type_name,
                self.type_parameter,
                self.value_parameter)
        } else {
            write!(f, "Type({:?}, {:?}, {:?}, {:?}, {:#?})", 
                self.construction_type,
                self.type_name,
                self.type_parameter,
                self.value_parameter,
                self.subtypes)
        }
        
    }
}

impl Type {
    pub fn new(type_name: TypeName, construction_type: ConstructionType, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Type {
        Type {
            type_name: type_name,
            construction_type: construction_type,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
            subtypes: Vec::new(),
        }
    }

    pub fn new_any() -> Type {
        Type {
            type_name: TypeName::Any,
            construction_type: ConstructionType::Any,
            type_parameter: TypeParameterList::Any,
            value_parameter: ValueParameterList::Any,
            subtypes: Vec::new(),
        }
    }

    pub fn create_subtype(&mut self, type_name: TypeName, construction_type: ConstructionType, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Option<&mut Type> {
        if self.construction_type == ConstructionType::Any {
            return self.get_or_create_direct_subtype(TypeName::Any, construction_type.clone(), TypeParameterList::Any, ValueParameterList::Any)
                .create_subtype(type_name, construction_type, type_parameter, value_parameter);
        }

        if self.type_name == TypeName::Any {
            if type_parameter == TypeParameterList::None && value_parameter == ValueParameterList::None {
                return self.construct_subtype(type_name, construction_type, type_parameter, value_parameter);
            }

            let direct_type_parameter = match type_parameter {
                TypeParameterList::Any => return None, // TODO error: cannot create "any" parameter types
                TypeParameterList::Fixed(ref list) => {
                    let mut direct_list = Vec::new();
                    for tp in list {
                        let direct_tp = match *tp {
                            TypeParameter::Any => TypeParameter::Any,
                            TypeParameter::Constrained => TypeParameter::Constrained,
                            TypeParameter::Fixed(_) => TypeParameter::Any,
                        };
                        direct_list.push(direct_tp);
                    }

                    TypeParameterList::Fixed(direct_list)
                }
                TypeParameterList::None => TypeParameterList::None,
            };

            let direct_value_parameter = match value_parameter {
                ValueParameterList::Any => return None, // TODO error: cannot create "any" parameter types
                ValueParameterList::Fixed(ref list) => {
                    let mut direct_list = Vec::new();
                    for tp in list {
                        let direct_tp = match *tp {
                            ValueParameter::Any => ValueParameter::Any,
                            ValueParameter::Constrained => ValueParameter::Constrained,
                            ValueParameter::Fixed(_) => ValueParameter::Any,
                        };
                        direct_list.push(direct_tp);
                    }

                    ValueParameterList::Fixed(direct_list)
                }
                ValueParameterList::None => ValueParameterList::None,
            };

            return self.get_or_create_direct_subtype(type_name.clone(), construction_type.clone(), direct_type_parameter, direct_value_parameter)
                .create_subtype(type_name, construction_type, type_parameter, value_parameter);
        }
        
        if self.type_parameter == TypeParameterList::Any {
            return self.get_or_create_direct_subtype(type_name.clone(), construction_type.clone(), type_parameter.clone(), ValueParameterList::Any)
                .create_subtype(type_name, construction_type, type_parameter, value_parameter);
        }

        self.construct_subtype(type_name, construction_type, type_parameter, value_parameter)
    }

    fn construct_subtype(&mut self, type_name: TypeName, construction_type: ConstructionType, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Option<&mut Type> {
        self.subtypes.push(Type::new(type_name, construction_type, type_parameter, value_parameter));
        let index = self.subtypes.len() -1;
        Some(&mut self.subtypes[index])
    }

    fn get_or_create_direct_subtype(&mut self, type_name: TypeName, construction_type: ConstructionType, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> &mut Type {
        let index = {
            self.subtypes.iter().position(|ref x| construction_type == x.construction_type && type_name == x.type_name && type_parameter == x.type_parameter && value_parameter == x.value_parameter)
        };

        match index {
            Some(i) => return &mut self.subtypes[i],
            None => {
                let new_child_constructor = Type::new(type_name, construction_type, type_parameter, value_parameter);
                self.subtypes.push(new_child_constructor);
                let index = self.subtypes.len() -1;
                return &mut self.subtypes[index];
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let mut any_type = Type::new_any();

        // `primitive f32`
        any_type.create_subtype(
            TypeName::Fixed("f32".to_string()),
            ConstructionType::Value,
            TypeParameterList::None,
            ValueParameterList::None);

        // Vector<T; N>
        any_type.create_subtype(
            TypeName::Fixed("Vector".to_string()),
            ConstructionType::Product,
            TypeParameterList::Fixed(vec![TypeParameter::Any]),
            ValueParameterList::Fixed(vec![ValueParameter::Any]));

        // Matrix<T; N, M>
        any_type.create_subtype(
            TypeName::Fixed("Matrix".to_string()),
            ConstructionType::Product,
            TypeParameterList::Fixed(vec![TypeParameter::Any]),
            ValueParameterList::Fixed(vec![ValueParameter::Any, ValueParameter::Any]));

        // Matrix<T; N, M> where N >= 1
        any_type.create_subtype(
            TypeName::Fixed("Matrix".to_string()),
            ConstructionType::Product,
            TypeParameterList::Fixed(vec![TypeParameter::Any]),
            ValueParameterList::Fixed(vec![ValueParameter::Constrained, ValueParameter::Any]));

        println!("");
        println!("{:#?}", any_type);
        panic!("");
    }
}
