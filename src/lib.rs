#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;

extern crate rspirv;
extern crate spirv_headers as spirv;

mod data_structures;
mod type_system;
mod types;
mod passes;

mod api;
mod compiler;
mod module;
mod compile_error;
mod ast;
mod parser;
mod string_builder;

#[cfg(test)]
mod testing;

pub use ::ast::*;
pub use api::parse_module;

pub use compile_error::{ CompileError, ErrorKind as CompileErrorKind };
pub use type_system::error::{ TypeError, ErrorKind as TypeErrorKind };
pub use compiler::{ Compiler, ModuleResolver };
