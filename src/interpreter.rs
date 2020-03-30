use super::ast;
use super::ntable;
use super::tokenizer;

pub struct Interpreter {
    name_table: ntable::NameTable,
}

impl Interpreter {
    fn lookup_variable(&self, node: &ast::AstNode) -> Result<i64, String> {
        return match self.name_table.get(&node.token.literal) {
            Ok(variable) => Ok(variable.value),
            Err(msg) => Err(msg),
        };
    }

    fn exec_pow_expression(&self, node: &ast::AstNode) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        match node.token.token_type {
            tokenizer::TokenType::Pow => {
                let mut result: i64 = 1;
                for _ in 0..right {
                    result *= left;
                }
                return Ok(result);
            },
            _ => panic!("logic error"),
        }
    }

    fn exec_mul_expression(&self, node: &ast::AstNode) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        match node.token.token_type {
            tokenizer::TokenType::Mul => return Ok(left * right),
            tokenizer::TokenType::Div => return Ok(left / right),
            _ => panic!("logic error"),
        }
    }

    fn exec_add_expression(&self, node: &ast::AstNode) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1]) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        match node.token.token_type {
            tokenizer::TokenType::Add => return Ok(left + right),
            tokenizer::TokenType::Sub => return Ok(left - right),
            tokenizer::TokenType::Mod => return Ok(left % right),
            _ => panic!("logic error"),
        }
    }

    fn exec_expression(&self, node: &ast::AstNode) -> Result<i64, String> {
        match node.node_type {
            ast::NodeType::Add => return self.exec_add_expression(node),
            ast::NodeType::Mul => return self.exec_mul_expression(node),
            ast::NodeType::Pow => return self.exec_pow_expression(node),
            ast::NodeType::Integer => return match node.token.literal.parse::<i64>() {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("lines:{}, column:{}, parse int failed", node.token.row, node.token.col)),
            },
            ast::NodeType::Name => return self.lookup_variable(node),
            _ => panic!("logic error"),
        }
    }

    fn exec_print(&self, node: &ast::AstNode) -> Result<(), String> {
        let expr = &node.childs[0];
        let value = match self.exec_expression(expr) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        println!("{}", value);
        Ok(())
    }

    fn exec_assign(&mut self, node: &ast::AstNode) -> Result<(), String> {
        let name = &node.childs[0];
        let expr = &node.childs[1];
        let value = match self.exec_expression(expr) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        self.name_table.set(name.token.literal.clone(), ntable::Variable::new(&name.token.literal, value));
        Ok(())
    }

    pub fn execute(&mut self, tree: &ast::AstNode) -> Result<(), String> {
        for node in &tree.childs {
            match node.node_type {
                ast::NodeType::Print => match self.exec_print(node) {
                    Ok(()) => (),
                    Err(msg) => return Err(msg),
                },
                ast::NodeType::Assign => match self.exec_assign(node) {
                    Ok(()) => (),
                    Err(msg) => return Err(msg),
                },
                _ => return Err(format!("line:{}, column:{}, syntax error, expect 'print' or variable",
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