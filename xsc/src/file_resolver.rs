use ::std::env;
use ::std::fs::File;
use ::std::io::prelude::*;
use ::std::error::Error;
use ::xshade::ModuleResolver;

pub struct FileResolver;

impl FileResolver {
    pub fn new() -> FileResolver {
        FileResolver
    }
}

impl ModuleResolver for FileResolver {
    fn resolve(&mut self, module_path: &str) -> Result<String, Box<Error>> {

        let mut f = File::open(module_path).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Ok(contents)
    }
}
