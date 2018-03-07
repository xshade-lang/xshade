use std::fmt;

#[derive(Eq, PartialEq)]
pub struct Type {
    construction_type: ConstructionType,
    type_name: TypeName,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type({:?}, {:?}, {:?}, {:?})", 
            self.construction_type,
            self.type_name,
            self.type_parameter,
            self.value_parameter)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ConstructionType {
    Any,
    Product,
    Function,
    Value,
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
    None,
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
    None,
}

pub struct TypeConstructor {
    construction_type: ConstructionType,
    type_name: TypeName,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
    child_constructors: Vec<TypeConstructor>,
    types: Vec<Type>,
}

impl fmt::Debug for TypeConstructor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TypeConstructor({:?}, {:?}, {:?}, {:?}) {:#?} {:#?}", 
            self.construction_type,
            self.type_name,
            self.type_parameter,
            self.value_parameter,
            self.child_constructors,
            self.types)
    }
}

impl TypeConstructor {
    pub fn new_root() -> TypeConstructor {
        TypeConstructor {
            construction_type: ConstructionType::Any,
            type_name: TypeName::Any,
            type_parameter: TypeParameterList::Any,
            value_parameter: ValueParameterList::Any,
            child_constructors: Vec::new(),
            types: Vec::new(),
        }
    }

    pub fn new(construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> TypeConstructor {
        TypeConstructor {
            construction_type: construction_type,
            type_name: type_name,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
            child_constructors: Vec::new(),
            types: Vec::new(),
        }
    }

    pub fn build(&mut self, construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> Result<(), ()> {
        if self.construction_type == ConstructionType::Any {
            return self.get_or_create_child_type_constructor_mut(construction_type.clone(), TypeName::Any, TypeParameterList::Any, ValueParameterList::Any)
                .build(construction_type, type_name, type_parameter, value_parameter);
        }

        if self.type_name == TypeName::Any {
            return self.get_or_create_child_type_constructor_mut(construction_type.clone(), type_name.clone(), TypeParameterList::Any, ValueParameterList::Any)
                .build(construction_type, type_name, type_parameter, value_parameter);
        }
        
        if self.type_parameter == TypeParameterList::Any {
            return self.get_or_create_child_type_constructor_mut(construction_type.clone(), type_name.clone(), type_parameter.clone(), ValueParameterList::Any)
                .build(construction_type, type_name, type_parameter, value_parameter);
        }

        self.types.push(Type{
            construction_type: construction_type,
            type_name: type_name,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
        });

        Ok(())
    }

    fn get_or_create_child_type_constructor_mut(&mut self, construction_type: ConstructionType, type_name: TypeName, type_parameter: TypeParameterList, value_parameter: ValueParameterList) -> &mut TypeConstructor {
        let index = {
            self.child_constructors.iter().position(|ref x| construction_type == x.construction_type && type_name == x.type_name && type_parameter == x.type_parameter && value_parameter == x.value_parameter)
        };

        match index {
            Some(i) => return &mut self.child_constructors[i],
            None => {
                let new_child_constructor = TypeConstructor::new(construction_type, type_name, type_parameter, value_parameter);
                self.child_constructors.push(new_child_constructor);
                let index = self.child_constructors.len() -1;
                return &mut self.child_constructors[index];
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let mut tc = TypeConstructor::new_root();

        tc.build(ConstructionType::Value, TypeName::Fixed("f32".to_string()), TypeParameterList::None, ValueParameterList::None);
        tc.build(ConstructionType::Value, TypeName::Fixed("f64".to_string()), TypeParameterList::None, ValueParameterList::None);
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f32".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(2)]));
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f32".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(3)]));
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f32".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(4)]));
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f64".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(2)]));
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f64".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(3)]));
        tc.build(ConstructionType::Product, TypeName::Fixed("Vector".to_string()), TypeParameterList::Fixed(vec![TypeParameter::Fixed("f64".to_string())]), ValueParameterList::Fixed(vec![ValueParameter::Fixed(4)]));

        println!("");
        println!("{:#?}", tc);
        panic!("");
    }
}
