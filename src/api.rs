use ::std::error::Error;
use ::module::Module;
use ::parser::parse_str;

pub fn parse_module(program: &str) -> Result<Module, Box<Error>> {
    let ast = parse_str(program)?;
    Ok(Module::from_ast(ast))
}
