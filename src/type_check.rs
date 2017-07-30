use ::ast::*;
use ::module::Module;
use ::symbol_table::SymbolTable;

pub fn type_check(type_environment: SymbolTable, module: &mut Module) {

}

#[cfg(test)]
mod tests {
    use super::*;
    use ::api::parse_module;

    #[test]
    fn test() {
        let code = r#"
            struct SomeStruct {
                x: f32,
                y: f32,
            };

            fn main() -> SomeStruct {
                let someStruct = SomeStruct {
                    x: 0.0,
                    y: 0,0
                };
                return someStruct;
            }
        "#;

        let mut module = parse_module(code).unwrap();

        println!("{:?}", module);

        // panic!("");
    }
}
