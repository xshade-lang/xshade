use ::ast::*;
use ::passes::*;

pub mod discover_structs;

impl<T: AstWalker> Pass<Vec<ItemKind>> for T {
    fn execute(&mut self, items: &mut Vec<ItemKind>) {
        self.walk(items);
    }
}

pub trait AstWalker {
    fn walk(&mut self, items: &mut Vec<ItemKind>) {
        for item in items.iter_mut() {
            match *item {
                ItemKind::Struct(ref mut item) => self.walk_struct(item),
                _ => (),
            }
        }
    }

    fn walk_struct(&mut self, struct_definition: &mut StructDefinition) {
        self.on_struct(struct_definition);

        for member in struct_definition.struct_member.iter_mut() {
            self.walk_struct_member(member);
        }
    }
    fn on_struct(&mut self, struct_definition: &mut StructDefinition);

    fn walk_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition) {
        self.on_struct_member(struct_member_definition);
    }
    fn on_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition);
}
