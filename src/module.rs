use ::ast::*;
use ::compile_error::CompileError;

#[derive(Debug)]
pub struct Module {
    path: String,
    source: String,
    ast: Vec<ItemKind>,
    is_core_module: bool,
    error: Option<CompileError>,
}

impl Module {

    pub fn new(path: String, source: String, ast: Vec<ItemKind>, is_core_module: bool) -> Module {
        Module {
            path: path,
            source: source,
            ast: ast,
            is_core_module: is_core_module,
            error: None,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn set_error(&mut self, error: CompileError) {
        self.error = Some(error);
    }

    pub fn get_error(&self) -> Option<&CompileError> {
        match self.error {
            Some(ref e) => Some(&e),
            None => None,
        }
    }

    pub fn is_core(&self) -> bool {
        self.is_core_module
    }

    pub fn find_programs(&self) -> Vec<&ProgramDefinition> {
        let mut programs = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Program(ref p) => programs.push(p),
                _ => (),
            }
        }
        programs
    }

    pub fn find_programs_mut(&mut self) -> Vec<&mut ProgramDefinition> {
        let mut programs = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Program(ref mut p) => programs.push(p),
                _ => (),
            }
        }
        programs
    }

    pub fn find_structs(&self) -> Vec<&StructDefinition> {
        let mut structs = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Struct(ref s) => structs.push(s),
                _ => (),
            }
        }
        structs
    }

    pub fn find_structs_mut(&mut self) -> Vec<&mut StructDefinition> {
        let mut structs = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Struct(ref mut s) => structs.push(s),
                _ => (),
            }
        }
        structs
    }

    pub fn find_functions(&self) -> Vec<&FunctionDeclaration> {
        let mut functions = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Function(ref f) => functions.push(f),
                _ => (),
            }
        }
        functions
    }

    pub fn find_functions_mut(&mut self) -> Vec<&mut FunctionDeclaration> {
        let mut functions = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Function(ref mut f) => functions.push(f),
                _ => (),
            }
        }
        functions
    }

    pub fn find_primitives(&self) -> Vec<&PrimitiveDeclaration> {
        let mut primitives = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Primitive(ref p) => primitives.push(p),
                _ => (),
            }
        }
        primitives
    }

    pub fn find_primitives_mut(&mut self) -> Vec<&mut PrimitiveDeclaration> {
        let mut primitives = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Primitive(ref mut p) => primitives.push(p),
                _ => (),
            }
        }
        primitives
    }

    pub fn find_constants(&self) -> Vec<&ConstantDefinition> {
        let mut constants = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Constant(ref c) => constants.push(c),
                _ => (),
            }
        }
        constants
    }

    pub fn find_constants_mut(&mut self) -> Vec<&mut ConstantDefinition> {
        let mut constants = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Constant(ref mut c) => constants.push(c),
                _ => (),
            }
        }
        constants
    }

    pub fn find_casts(&self) -> Vec<&CastDeclaration> {
        let mut casts = Vec::new();
        for item in &self.ast {
            match item {
                &ItemKind::Cast(ref c) => casts.push(c),
                _ => (),
            }
        }
        casts
    }

    pub fn find_casts_mut(&mut self) -> Vec<&mut CastDeclaration> {
        let mut casts = Vec::new();
        for item in &mut self.ast {
            match item {
                &mut ItemKind::Cast(ref mut c) => casts.push(c),
                _ => (),
            }
        }
        casts
    }
}
