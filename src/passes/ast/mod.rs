use ::ast::*;
use ::passes::*;

pub mod spirv_symbol_table;
pub mod generate_spirv;

/// Visitor pattern over the AST
/// calls override visit_* functions to process the given items
/// some may require you to call the given walk_* function to continue
/// you may choose to not call walk_* if you don't need further processing
pub trait AstWalker {
    fn visit(&mut self, items: &mut Vec<ItemKind>) -> PassResult {
        for item in items.iter_mut() {
            match *item {
                ItemKind::Struct(ref mut item) => self.visit_struct(item),
                ItemKind::Function(ref mut item) => self.visit_function(item),
                _ => Ok(()),
            };
        }

        Ok(())
    }

    fn visit_block(&mut self, block: &mut BlockDeclaration) -> PassResult {
        self.walk_block(block)
    }

    fn walk_block(&mut self, block: &mut BlockDeclaration) -> PassResult {
        for statement in block.statements.iter_mut() {
            match *statement {
                BlockStatement::Local(ref mut s) => self.visit_local_statement(s),
                BlockStatement::Return(ref mut s) => self.visit_return_statement(s),
                BlockStatement::Expression(ref mut s) => self.visit_expression_statement(s),
            };
        }

        Ok(())
    }

    fn visit_local_statement(&mut self, local_statement: &mut LocalDeclaration) -> PassResult {
        self.walk_local_statement(local_statement)
    }

    fn walk_local_statement(&mut self, local_statement: &mut LocalDeclaration) -> PassResult {
        self.visit_expression_statement(&mut local_statement.expression)
    }

    fn visit_return_statement(&mut self, return_statement: &mut ReturnDeclaration) -> PassResult {
        self.walk_return_statement(return_statement)
    }

    fn walk_return_statement(&mut self, return_statement: &mut ReturnDeclaration) -> PassResult {
        self.visit_expression_statement(&mut return_statement.expression)
    }

    fn visit_expression_statement(&mut self, expression_statement: &mut ExpressionStatement) -> PassResult {
        self.walk_expression_statement(expression_statement)
    }

    fn walk_expression_statement(&mut self, expression_statement: &mut ExpressionStatement) -> PassResult {
        self.visit_expression(expression_statement)
    }

    fn visit_expression(&mut self, expression_statement: &mut ExpressionStatement) -> PassResult {
        match *expression_statement {
            ExpressionStatement::Infix(ref mut e) => self.visit_infix_expression(e),
            ExpressionStatement::Literal(ref mut e) => self.visit_literal_expression(e),
            ExpressionStatement::Call(ref mut e) => self.visit_call_expression(e),
            ExpressionStatement::StructInstantiation(ref mut e) => self.visit_struct_instantiation_expression(e),
            ExpressionStatement::FieldAccessor(ref mut e) => self.visit_field_accessor_expression(e),
            ExpressionStatement::IndexAccessor(ref mut e) => self.visit_index_accessor_expression(e),
            ExpressionStatement::Variable(ref mut e) => self.visit_variable_expression(e),
        }
    }

    fn visit_infix_expression(&mut self, infix_expression: &mut InfixExpression) -> PassResult {
        self.walk_infix_expression(infix_expression)
    }

    fn walk_infix_expression(&mut self, infix_expression: &mut InfixExpression) -> PassResult {
        self.visit_expression(&mut infix_expression.left_hand);
        self.visit_expression(&mut infix_expression.right_hand);
        Ok(())
    }

    fn walk_infix_expression_left(&mut self, infix_expression: &mut InfixExpression) -> PassResult {
        self.visit_expression(&mut infix_expression.left_hand)
    }

    fn walk_infix_expression_right(&mut self, infix_expression: &mut InfixExpression) -> PassResult {
        self.visit_expression(&mut infix_expression.right_hand)
    }

    fn visit_literal_expression(&mut self, literal_expression: &mut LiteralExpression) -> PassResult {
        Ok(())
    }

    fn visit_call_expression(&mut self, call_expression: &mut CallExpression) -> PassResult {
        self.walk_call_expression(call_expression)
    }

    fn walk_call_expression(&mut self, call_expression: &mut CallExpression) -> PassResult {
        for e in call_expression.arguments.iter_mut() {
            self.visit_expression(e);
        }

        Ok(())
    }

    fn visit_struct_instantiation_expression(&mut self, struct_instantiation_expression: &mut StructInstantiationExpression) -> PassResult {
        self.walk_struct_instantiation_expression(struct_instantiation_expression)
    }

    fn walk_struct_instantiation_expression(&mut self, struct_instantiation_expression: &mut StructInstantiationExpression) -> PassResult {
        for e in struct_instantiation_expression.struct_field_initializer.iter_mut() {
            self.visit_struct_field_initializer(e);
        }

        Ok(())
    }

    fn visit_struct_field_initializer(&mut self, struct_field_initializer: &mut StructFieldInitializerExpression) -> PassResult {
        self.walk_struct_field_initializer(struct_field_initializer)
    }

    fn walk_struct_field_initializer(&mut self, struct_field_initializer: &mut StructFieldInitializerExpression) -> PassResult {
        self.visit_expression(&mut struct_field_initializer.initializer)
    }

    fn visit_field_accessor_expression(&mut self, field_accessor_expression: &mut FieldAccessorExpression) -> PassResult {
        Ok(())
    }

    fn visit_index_accessor_expression(&mut self, index_accessor_expression: &mut IndexAccesorExpression) -> PassResult {
       self.walk_index_accessor_expression(index_accessor_expression)
    }

    fn walk_index_accessor_expression(&mut self, index_accessor_expression: &mut IndexAccesorExpression) -> PassResult {
        self.visit_expression(&mut index_accessor_expression.access_expression)
    }

    fn visit_variable_expression(&mut self, variable_expression: &mut VariableExpression) -> PassResult {
        Ok(())
    }

    fn visit_function_argument(&mut self, function_argument: &mut FunctionArgumentDeclaration) -> PassResult {
        Ok(())
    }

    fn walk_function(&mut self, function_definition: &mut FunctionDeclaration) -> PassResult {
        for argument in function_definition.arguments.iter_mut() {
            self.visit_function_argument(argument);
        }

        self.visit_block(&mut function_definition.block);

        Ok(())
    }

    fn visit_function(&mut self, function_definition: &mut FunctionDeclaration) -> PassResult {
        self.walk_function(function_definition)
    }

    fn walk_struct(&mut self, struct_definition: &mut StructDefinition) -> PassResult {
        for member in struct_definition.struct_member.iter_mut() {
            self.visit_struct_member(member);
        }

        Ok(())
    }

    fn visit_struct(&mut self, struct_definition: &mut StructDefinition) -> PassResult {
        self.walk_struct(struct_definition)
    }

    fn visit_struct_member(&mut self, struct_member_definition: &mut StructMemberDefinition) -> PassResult {
        Ok(())
    }
}
