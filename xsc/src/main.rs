extern crate xshade;
extern crate getopts;

use getopts::Options;
use std::env;
use xshade::*;

mod file_resolver;

fn get_span_line(lines: &Vec<&str>, span: Span) -> String {
    lines[span.line - 1].to_owned()
}

fn create_span_marker(span: Span) -> String {
    let mut marker = String::new();
    for i in 0..span.column - 1 {
        marker.push_str(" ");
    }
    for i in 0..span.length {
        marker.push_str("^");
    }
    marker
}

fn create_path_with_span(path: &str, span: Span) -> String {
    let mut p = String::new();
    p.push_str(path);
    p.push_str(":");
    p.push_str(&span.line.to_string());
    p.push_str(":");
    p.push_str(&span.column.to_string());
    p
}

fn prefix(line: &str, indent: usize, content: &str) -> String {
    let mut l = String::new();
    l.push_str(" ");
    for i in 0..indent - content.len() {
        l.push_str(" ");
    }
    l.push_str(content);
    l.push_str(" | ");
    l.push_str(line);
    l
}

fn single_span_error(line: &str, span: Span, path: &str, top: String, bottom: String) {
    let marker = create_span_marker(span);

    let indent = span.line.to_string().len();

    println!("{}", top);
    println!("error:");
    println!("{}", path);
    println!("{}", prefix(&line, indent, &span.line.to_string()));
    println!("{}", prefix(&marker, indent, ""));
    println!("{}", bottom);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let module_path = args[1].to_string();

    let file_resolver = file_resolver::FileResolver::new();
    let mut compiler = Compiler::new(Box::new(file_resolver));

    match compiler.compile_module(&module_path) {
        Ok(module) => {

            if let Some(error) = module.get_error() {
                let source = module.get_source();
                let lines: Vec<&str> = source.lines().collect();

                match error.get_kind() {
                    &CompileErrorKind::TypeError(ref type_error) => {
                        let path = create_path_with_span(module.get_path(), type_error.get_span());
                        match type_error.get_kind() {
                            &TypeErrorKind::TypeNotFound(ref type_name) => {
                                let span = type_error.get_span();
                                let line = get_span_line(&lines, span);
                                
                                single_span_error(&line, span, &path,
                                format!("error: Type not found:"),
                                format!("Type `{}` not found.", type_name));
                            }
                            &TypeErrorKind::TypeHasNoMember => {
                                let span = type_error.get_span();
                                let line = get_span_line(&lines, span);


                                single_span_error(&line, span, &path,
                                format!("error: Type has no members:"),
                                format!("Type `{{}}` has no members."));
                            }
                            &TypeErrorKind::IncompatibleTypes(left, right) => {
                                let left_line = get_span_line(&lines, left);
                                let right_line = get_span_line(&lines, right);

                                let left_marker = create_span_marker(left);
                                let right_marker = create_span_marker(right);

                                let indent = right.line.to_string().len();

                                println!("");
                                println!("error: Incompatible Types:");
                                println!("{}", path);
                                println!("{}", prefix(&left_line, indent, &left.line.to_string()));
                                println!("{}", prefix(&left_marker, indent, ""));
                                println!("{}", prefix(&right_line, indent, &right.line.to_string()));
                                println!("{}", prefix(&right_marker, indent, ""));
                                println!("Cannot use operator `{{}}` on type `{{}}` and `{{}}`.");
                            }
                            _ => println!("{:#?}", error),
                        }
                    },
                    _ => println!("{:#?}", error),
                }
            } else {
                println!("{:#?}", module);
            }
        },
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}