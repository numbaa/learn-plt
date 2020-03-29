use super::ast;
use super::ntable;
use super::tokenizer;

pub struct Interpreter {
    name_table: ntable::NameTable,
}

impl Interpreter {
    fn lookup_name(&self, node: &ast::AstNode) -> ntable::Variable {
        return self.name_table.get(&node.token.literal).unwrap();
    }

    fn exec_pow_expression(&self, node: &ast::AstNode) -> i64 {
        let left = self.exec_expression(&node.childs[0]);
        let right = self.exec_expression(&node.childs[1]);
        match node.token.token_type {
            tokenizer::TokenType::Pow => {
                let mut result: i64 = 1;
                for _ in 0..right {
                    result *= left;
                }
                return result;
            },
            _ => panic!("logic error"),
        }
    }

    fn exec_mul_expression(&self, node: &ast::AstNode) -> i64 {
        let left = self.exec_expression(&node.childs[0]);
        let right = self.exec_expression(&node.childs[1]);
        match node.token.token_type {
            tokenizer::TokenType::Mul => return left * right,
            tokenizer::TokenType::Div => return left / right,
            _ => panic!("logic error"),
        }
    }

    fn exec_add_expression(&self, node: &ast::AstNode) -> i64 {
        let left = self.exec_expression(&node.childs[0]);
        let right = self.exec_expression(&node.childs[1]);
        match node.token.token_type {
            tokenizer::TokenType::Add => return left + right,
            tokenizer::TokenType::Sub => return left - right,
            tokenizer::TokenType::Mod => return left % right,
            _ => panic!("logic error"),
        }
    }

    fn exec_expression(&self, node: &ast::AstNode) -> i64 {
        match node.node_type {
            ast::NodeType::Add => return self.exec_add_expression(node),
            ast::NodeType::Mul => return self.exec_mul_expression(node),
            ast::NodeType::Pow => return self.exec_pow_expression(node),
            ast::NodeType::Integer => return node.token.literal.parse::<i64>().unwrap(),
            ast::NodeType::Name => return self.lookup_name(node).value,
            _ => panic!("logic error"),
        }
    }

    fn exec_print(&self, node: &ast::AstNode) {
        let expr = &node.childs[0];
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