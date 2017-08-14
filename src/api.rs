use ::std::error::Error;
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_check::type_check;
use ::type_system::type_environment::TypeEnvironment;

fn parse_core_modules(symbols: &mut SymbolTable) -> Result<Module, Box<Error>> {
    let primitives = include_str!("../libcore/primitives.xs");
    let ast = parse_str(primitives)?;
    let mut module = Module::from_ast(ast, true);
    type_check(symbols, &mut module)?;
    Ok(module)
}

pub fn parse_module(program: &str) -> Result<Module, Box<Error>> {
    let environment = TypeEnvironment::new();
    let mut symbols = SymbolTable::new(environment);

    parse_core_modules(&mut symbols)?;

    let ast = parse_str(program)?;
    let mut module = Module::from_ast(ast, false);
    type_check(&mut symbols, &mut module)?;
    Ok(module)
}
