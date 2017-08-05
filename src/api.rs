use ::std::error::Error;
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_check::type_check;
use ::type_system::type_environment::TypeEnvironment;

pub fn parse_module(program: &str) -> Result<Module, Box<Error>> {
    let ast = parse_str(program)?;
    let mut module = Module::from_ast(ast);
    let mut environment = TypeEnvironment::new();
    let mut symbols = SymbolTable::new();
    type_check(&mut environment, &mut symbols, &mut module)?;
    Ok(module)
}
