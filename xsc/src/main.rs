extern crate xshade;
extern crate getopts;

use getopts::Options;
use std::env;
use xshade::*;

mod file_resolver;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let module_path = args[1].to_string();

    let file_resolver = file_resolver::FileResolver::new();
    let mut compiler = Compiler::new(Box::new(file_resolver));

    let module = compiler.compile_module(&module_path);

    println!("{:#?}", module);
}
