use ::ast::*;
use ::passes::*;
use ::passes::ast::*;
use ::passes::results::PassResultReference;
use ::type_system::symbol_table::{ SymbolTableReference };
use ::type_system::type_environment::TypeReference;

pub struct CheckExpressionPass {
    symbol_table: SymbolTableReference,
    result: PassResultReference,
    last_type: Option<TypeReference>,
}

impl CheckExpressionPass {
    pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> CheckExpressionPass {
        CheckExpressionPass {
            symbol_table: symbol_table,
            result: result,
            last_type: None,
        }
    }
}

ast_pass_impl!(CheckExpressionPass, {
    fn visit_infix_expression(&mut self, infix_expression: &mut InfixExpression) {
        self.walk_infix_expression_left(infix_expression);
        let left_type = self.last_type.take().unwrap();
        self.walk_infix_expression_right(infix_expression);
        let right_type = self.last_type.take().unwrap();

        if left_type == right_type {
            infix_expression.infix_type = Some(left_type);
            return;
        }
    }
});

#[cfg(test)]
mod tests {
}
