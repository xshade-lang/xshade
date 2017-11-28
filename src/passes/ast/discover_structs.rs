use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::type_system::symbol_table::SymbolTable;

pub struct DiscoverStructsPass {
    symbol_table: SymbolTable,
}

impl DiscoverStructsPass {
    pub fn new(symbol_table: SymbolTable) -> DiscoverStructsPass {
        DiscoverStructsPass {
            symbol_table: symbol_table,
        }
    }
}

impl AstWalker for DiscoverStructsPass {
    fn on_struct(&mut self, struct_definition: &mut StructDefinition) {
        //let type_ref = try!(self.symbol_table.create_type(&struct_definition.struct_name.name));
        //try!(self.symbol_table.add_symbol_with_type(&struct_definition.struct_name.name, type_ref));
        //struct_definition.declaring_type = Some(type_ref);
    }

    fn on_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition) {

    }
}
