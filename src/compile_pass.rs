use std::fmt;
use ::std::error::Error;
use ::compile_error::{ CompileError, CompileResult, ErrorKind };
use ::module::Module;
use ::parser::parse_str;
use ::type_system::symbol_table::SymbolTable;
use ::type_system::type_environment::TypeEnvironment;
use ::type_system::type_check::type_check;


// TODO: This is a copy of the current pass system implementations core constructs.
//       Once an initial version was published, move to the official integration
pub enum PassError {
    Warnings,
    Errors,
    Fatal,
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compile Error")
    }
}


pub type PassResult = Result<(), PassError>;

pub trait Pass<T> {
    fn execute(&mut self, items: &mut T) -> PassResult;
}

#[derive(Debug)]
pub struct CompilePass {
    uid: i32,
    source: String,
    source_path: String,
    pub modules: Vec<Module>,
    pub symbol_table: SymbolTable,
    // Pass-System requirement
    pub executed: bool,
}

impl CompilePass {
    pub fn new(uid: i32, source: String, source_path: String, symbol_table: SymbolTable) -> CompilePass {
        CompilePass {
            uid: uid,
            source: source,
            source_path: source_path,
            modules: Vec::new(),
            symbol_table: symbol_table, // Symbol-Table is prefilled with core modules
            executed: false,
        }
    }

    pub fn get_uid(&self) -> i32 { 
        self.uid
    }
}

impl Pass<Vec<Module>> for CompilePass {
    fn execute(&mut self, items: &mut Vec<Module>) -> PassResult {
        self.executed = true;
        
        let ast = match parse_str(&self.source) { 
            Ok(a) => a,
            Err(e) => return Err(PassError::Errors),
        };

        // println!("{:#?}", ast);

        let module = Module::new(self.source_path.to_owned(), self.source.clone(), ast, false);
        self.modules.push(module);

        {
            for m in &mut self.modules {
                match type_check(&mut self.symbol_table, m) {
                    Ok(_) => {},
                    Err(e) => {
                        let span = e.get_span();
                        m.set_error(CompileError::new(ErrorKind::TypeError(e), span));
                    },
                }
            }
        }

        // items = self.modules.to_vec();

        Ok(())
    }
}