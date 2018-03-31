/*
    fn multiply<T; N, M>(vec: Vector<T; N>, matrix: Matrix<T; N, M>) -> Vector<T; M>
    has type
    (Vector<T; N>, Matrix<T; N, M>) -> Vector<T; M>
    with Type Parameter `T`
    and Value Parameter `N` and `M`
*/

/* 
    fn add<T>(a: T, b: T) -> T
    has type
    (T, T) -> T
    and
    fn add<f32>(a: f32, b: f32) -> f32
    is a subtype
*/

pub struct FunctionType {
    name: String,
    type_parameter: TypeParameterList,
    value_parameter: ValueParameterList,
    arguments: Vec<TypeReference>,
    return_type: TypeReference,
}

impl FunctionType {
    pub fn new(name: String, type_parameter: TypeParameterList, value_parameter: ValueParameterList, arguments: Vec<TypeReference>, return_type: TypeReference) -> FunctionType {
        FunctionType {
            name: name,
            type_parameter: type_parameter,
            value_parameter: value_parameter,
            arguments: arguments,
            return_type: return_type,
        }
    }
}
