use ::std::rc::Rc;
use ::std::cell::{ RefCell, Ref, RefMut };
use ::std::error::Error;
use ::data_structures::shared::Shared;

pub type PassResultReference = Shared<PassResult>;

#[derive(Debug)]
pub struct PassResult {
    errors: Vec<Box<Error>>,
    warnings: Vec<Box<Error>>,
}

impl PassResult {
    pub fn new() -> PassResult {
        PassResult {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn add_error(&mut self, error: Box<Error>) {
        self.errors.push(error);
    }
}
