#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;

mod type_system;

mod api;
mod module;
mod compile_error;
mod ast;
mod parser;
mod string_builder;

pub use ::ast::*;
pub use api::parse_module;
