#![feature(link_args)] // Experimental, likely to cause issues without nightly build!

extern crate xshade;
extern crate getopts;

use std::env;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use getopts::Options;
use xshade::*;

mod lib;
mod web_module_resolver;

#[link_args = "-s EXPORTED_FUNCTIONS=['_xsc_call_w_code']"]
#[link_args = "-s DEMANGLE_SUPPORT=1"]
extern {}

// Converts an unsafe JS input character array into a safe RUST String
fn safe_str(s: *mut c_char) -> String { 
	unsafe { 
	  CStr::from_ptr(s).to_string_lossy().into_owned()
	}
}

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

#[no_mangle] // Make it detectable for the WebAssembly mechanism
pub fn xsc_call_w_code(in_program: *mut c_char) -> *mut c_char {
    let program = safe_str(in_program);

	println!("Received program {}!", program);
	
    let     web_module_resolver = web_module_resolver::WebModuleResolver::new();
    let mut compiler            = Compiler::new(Box::new(web_module_resolver));

    let mut output : String = String::new();

    match compiler.compile_module(&program) {
        Ok(module) => {
            if let Some(error) = module.get_error() {
                let source           = module.get_source();
                let lines: Vec<&str> = source.lines().collect();

                // Test for errors
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
								  
								  
				                fmt::write(&mut output, format_args!("{:#?}", "Type Error"))
					              .expect("Error occurred while trying to write in String");
                            }
                            &TypeErrorKind::TypeHasNoMember => {
                                let span = type_error.get_span();
                                let line = get_span_line(&lines, span);

                                single_span_error(&line, span, &path,
                                  format!("error: Type has no members:"),
                                  format!("Type `{{}}` has no members."));								
								
				                fmt::write(&mut output, format_args!("{:#?}", "TypeHasNoMember"))
					              .expect("Error occurred while trying to write in String");
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
								
				                fmt::write(&mut output, format_args!("{:#?}", "IncompatibleTypes"))
					              .expect("Error occurred while trying to write in String");
                            }
                            _ => {
  							    println!("{:#?}", error);

				                fmt::write(&mut output, format_args!("{:#?}", error))
					              .expect("Error occurred while trying to write in String");
							},
                        }
                    },
                    _ => {
                         println!("{:#?}", error);
				
				        fmt::write(&mut output, format_args!("{:#?}", error))
					      .expect("Error occurred while trying to write in String");					
					},
                }
            } else {
                println!("{:#?}", module);	

				fmt::write(&mut output, format_args!("{:#?}", module))
                  .expect("Error occurred while trying to write in String");			
            }
        },
        Err(error) => {
            println!("{:#?}", error);
			
            fmt::write(&mut output, format_args!("{:#?}", error))
              .expect("Error occurred while trying to write in String");
        }
    }

    // Wrap output and return
    CString::new(output.as_str())
      .unwrap()
      .into_raw()
}

pub fn main() {
	// Intentionally left blank
}
