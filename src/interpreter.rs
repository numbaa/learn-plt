use super::ast;
use super::ntable;

pub struct Interpreter {
    name_table: ntable::NameTable,
}

impl Interpreter {
    fn exec_expression(&self, node: &ast::AstNode) -> i64 {
        0
    }

    fn exec_print(&self, node: &ast::AstNode) {
        let expr = &node.childs[1];
        let value = self.exec_expression(expr);
        println!("{}", value);
    }

    fn exec_assign(&mut self, node: &ast::AstNode) {
        let name = &node.childs[0];
        let expr = &node.childs[1];
        let value = self.exec_expression(expr);
        self.name_table.set(name.token.literal.clone(), ntable::Variable::new(&name.token.literal, value));
    }

    pub fn execute(&mut self, tree: &ast::AstNode) -> Result<(), String> {
        for node in &tree.childs {
            match node.node_type {
                ast::NodeType::Print => self.exec_print(node),
                ast::NodeType::Assign => self.exec_assign(node),
                _ => return Err(format!("syntax error, expect 'print' or variable, line:{}, column:{}",
                            node.token.row, node.token.col)),
            }
        }
        Ok(())
    }
    
    pub fn new() -> Interpreter {
        Interpreter {
            name_table: ntable::NameTable::new(),
        }
    }
}