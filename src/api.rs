use ::std::error::Error;
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_check::type_check;
use ::type_system::type_environment::TypeEnvironment;

fn parse_core_modules(environment: &mut TypeEnvironment, symbols: &mut SymbolTable) -> Result<Module, Box<Error>> {
    let primitives = include_str!("../libcore/primitives.xs");
    let ast = parse_str(primitives)?;
    let mut module = Module::from_ast(ast, true);
    type_check(environment, symbols, &mut module)?;
    Ok(module)
}

pub fn parse_module(program: &str) -> Result<Module, Box<Error>> {
    let mut environment = TypeEnvironment::new();
    let mut symbols = SymbolTable::new();

    parse_core_modules(&mut environment, &mut symbols)?;

    let ast = parse_str(program)?;
    let mut module = Module::from_ast(ast, false);
    type_check(&mut environment, &mut symbols, &mut module)?;
    Ok(module)
}
