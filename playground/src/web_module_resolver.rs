use ::std::env;
use ::std::fs::File;
use ::std::io::prelude::*;
use ::std::error::Error;
use ::xshade::ModuleResolver;

pub struct WebModuleResolver;

impl WebModuleResolver {
    pub fn new() -> WebModuleResolver {
        WebModuleResolver
    }
}

impl ModuleResolver for WebModuleResolver {
    fn resolve(&mut self, module_path: &str) -> Result<String, Box<Error>> {
		// Just redirect, ifnempty
		// assert_eq!(module_path.is_empty(), true);
		
        let mut contents = module_path.to_owned();
		
        Ok(contents)
    }
}
