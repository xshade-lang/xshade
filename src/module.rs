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
}
