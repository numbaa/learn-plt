use super::tokenizer::*;
use super::ast;

pub struct Parser {
    tokenizer: Tokenizer
}

//only for syntax check now
impl Parser {

    fn expression_pow(&mut self) -> ast::AstNode {
        let token = self.tokenizer.look_ahead(1);
        self.tokenizer.eat(1);
        let integer_or_name: ast::AstNode;
        match token.token_type {
            TokenType::Integer => integer_or_name = ast::AstNode::new(ast::NodeType::Integer, token),
            TokenType::Name => integer_or_name = ast::AstNode::new(ast::NodeType::Name, token),
            _ => panic!("syntax error, expect integer or name"),
        }
        let op = self.tokenizer.look_ahead(1);
        let mut expr_pow: ast::AstNode;
        match op.token_type {
            TokenType::Pow => expr_pow = ast::AstNode::new(ast::NodeType::Pow, op),
            _ => return integer_or_name,
        }
        self.tokenizer.eat(1);
        expr_pow.add_node(integer_or_name);
        expr_pow.add_node(self.expression_pow());
        return expr_pow;
    }

    fn expression_mul(&mut self) -> ast::AstNode {
        let expr_pow = self.expression_pow();
        let op = self.tokenizer.look_ahead(1);
        let mut expr_mul: ast::AstNode;
        match op.token_type {
            TokenType::Mul | TokenType::Div => expr_mul = ast::AstNode::new(ast::NodeType::Mul, op),
            _ => return expr_pow,
        }
        self.tokenizer.eat(1);
        expr_mul.add_node(expr_pow);
        expr_mul.add_node(self.expression_mul());
        return expr_mul;
    }
    fn expression_add(&mut self) -> ast::AstNode {
        let expr_mul = self.expression_mul();
        let op = self.tokenizer.look_ahead(1);
        let mut expr_add: ast::AstNode;
        match op.token_type {
            TokenType::Add | TokenType::Sub | TokenType::Mod => expr_add = ast::AstNode::new(ast::NodeType::Add, op),
            _ => return expr_mul,
        }
        self.tokenizer.eat(1);
        expr_add.add_node(expr_mul);
        expr_add.add_node(self.expression_add());
        return expr_add;
    }

    fn expression(&mut self) -> ast::AstNode {
        return self.expression_add();
    }
    
    fn statement_print(&mut self, parent: &mut ast::AstNode) {
        let mut print_node = ast::AstNode::new(ast::NodeType::Print, self.tokenizer.look_ahead(1));
        self.tokenizer.eat(1);
        print_node.add_node(self.expression());
        parent.add_node(print_node);
    }

    fn statement_assign(&mut self, parent: &mut ast::AstNode) {
        let name = self.tokenizer.look_ahead(1);
        let assign = self.tokenizer.look_ahead(2);
        self.tokenizer.eat(2);
        match assign.token_type {
            TokenType::Assign => (),
            _ => panic!("syntax error, expect '=', found {}", assign.literal),
        }
        let mut assign_node = ast::AstNode::new(ast::NodeType::Assign, assign);
        let name_node = ast::AstNode::new(ast::NodeType::Name, name);
        assign_node.add_node(name_node);
        assign_node.add_node(self.expression());
        parent.add_node(assign_node);
    }

    pub fn parse(&mut self) -> ast::AstNode {
        let mut ast_root = ast::AstNode::new(ast::NodeType::Root, Token::new());
        loop {
            let token = self.tokenizer.look_ahead(1);
            match token.token_type {
                TokenType::Print => self.statement_print(&mut ast_root),
                TokenType::Name => self.statement_assign(&mut ast_root),
                TokenType::Newline => continue,
                TokenType::EOF => return ast_root,
                _ => { println!("syntax error"); return ast_root; }
            }
        }
    }

    pub fn new(tokenizer: Tokenizer) -> Parser {
        return Parser {
            tokenizer: tokenizer
        }
    }
}