use super::ast;
use super::ntable;
use super::tokenizer;
use std::collections::HashMap;
use std::vec;

pub struct Interpreter {
    name_table: ntable::NameTable,
}

impl Interpreter {
    fn exec_func_call(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<i64, String> {
        let mut local_name_table = vec::Vec::<ntable::Variable>::new();
        for param in &node.childs[0].childs {
            let variable = match self.name_table.get(&param.token.literal) {
                Ok(variable) => variable,
                Err(msg) => return Err(msg),
            };
            local_name_table.push(variable);
        }
        let func_node = match func_table.get(&node.token.literal) {
            Some(node) => node,
            None => return Err("function not defined".to_string()),
        };
        for i in 0..func_node.childs[0].childs.len() {
            local_name_table[i].name = func_node.childs[0].childs[i].token.literal.clone();
        }
        //Seems not possible to finish this without scope system
        Ok(0)
    }
    fn lookup_variable(&self, node: &ast::AstNode) -> Result<i64, String> {
        return match self.name_table.get(&node.token.literal) {
            Ok(variable) => Ok(variable.value),
            Err(msg) => Err(msg),
        };
    }

    fn exec_pow_expression(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0], func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1], func_table) {
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

    fn exec_mul_expression(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0], func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1], func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        match node.token.token_type {
            tokenizer::TokenType::Mul => return Ok(left * right),
            tokenizer::TokenType::Div => return Ok(left / right),
            _ => panic!("logic error"),
        }
    }

    fn exec_add_expression(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<i64, String> {
        let left = match self.exec_expression(&node.childs[0], func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let right = match self.exec_expression(&node.childs[1], func_table) {
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

    fn exec_expression(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<i64, String> {
        match node.node_type {
            ast::NodeType::Add => return self.exec_add_expression(node, func_table),
            ast::NodeType::Mul => return self.exec_mul_expression(node, func_table),
            ast::NodeType::Pow => return self.exec_pow_expression(node, func_table),
            ast::NodeType::Integer => return match node.token.literal.parse::<i64>() {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("lines:{}, column:{}, parse int failed", node.token.row, node.token.col)),
            },
            ast::NodeType::Name => return self.lookup_variable(node),
            ast::NodeType::FuncCall => return self.exec_func_call(node, func_table),
            _ => panic!("logic error"),
        }
    }

    fn exec_print(&self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<(), String> {
        let expr = &node.childs[0];
        let value = match self.exec_expression(expr, func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        println!("{}", value);
        Ok(())
    }

    fn exec_assign(&mut self, node: &ast::AstNode, func_table: &HashMap<String, &ast::AstNode>) -> Result<(), String> {
        let name = &node.childs[0];
        let expr = &node.childs[1];
        let value = match self.exec_expression(expr, func_table) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        self.name_table.set(name.token.literal.clone(), ntable::Variable::new(&name.token.literal, value));
        Ok(())
    }

    fn exec_func_decl<'a>(&mut self, node: &'a ast::AstNode, func_table: &mut HashMap<String, &'a ast::AstNode>) -> Result<(), String> {
        func_table.insert(node.token.literal.clone(), node);
        Ok(())
    }

    pub fn execute(&mut self, tree: &ast::AstNode) -> Result<(), String> {
        let mut func_table = HashMap::<String, &ast::AstNode>::new();
        for node in &tree.childs {
            match node.node_type {
                ast::NodeType::Print => match self.exec_print(node, &func_table) {
                    Ok(()) => (),
                    Err(msg) => return Err(msg),
                },
                ast::NodeType::Assign => match self.exec_assign(node, &func_table) {
                    Ok(()) => (),
                    Err(msg) => return Err(msg),
                },
                ast::NodeType::FuncDecl => match self.exec_func_decl(node, &mut func_table) {
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