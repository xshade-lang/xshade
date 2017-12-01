#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;

mod data_structures;
mod type_system;

mod api;
mod compile_pass;
mod compiler;
mod module;
mod compile_error;
mod ast;
mod parser;
mod string_builder;

pub use ::ast::*;
pub use api::parse_module;

pub use compile_error::{ CompileError, ErrorKind as CompileErrorKind };
pub use type_system::error::{ TypeError, ErrorKind as TypeErrorKind };
pub use compiler::{ Compiler, ModuleResolver };
