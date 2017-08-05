use ::ast::*;

#[derive(Debug)]
pub struct Module {
    ast: Vec<ItemKind>,
}

impl Module {
    pub fn from_ast(ast: Vec<ItemKind>) -> Module {
        Module {
            ast: ast,
        }
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
}
