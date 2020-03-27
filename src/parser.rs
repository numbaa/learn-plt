use super::tokenizer::*;
use super::ast;

pub struct Parser {
    tokenizer: Tokenizer
}

//only for syntax check now
impl Parser {
    fn expression_integer_or_name(&mut self) -> ast::AstNode {
        let token = self.tokenizer.look_ahead(1);
        self.tokenizer.eat(1);
        match token.token_type {
            TokenType::Integer => return ast::AstNode::new(ast::NodeType::Integer, token),
            TokenType::Name => return ast::AstNode::new(ast::NodeType::Name, token),
            _ => panic!("syntax error, line:{}, column:{} expect integer or name", token.row, token.col),
        }
    }
    fn expression_pow(&mut self) -> ast::AstNode {
        let mut left = self.expression_integer_or_name();
        loop {
            let op = self.tokenizer.look_ahead(1);
            match op.token_type {
                TokenType::Pow => {
                    let mut pow_expr = ast::AstNode::new(ast::NodeType::Pow, op);
                    self.tokenizer.eat(1);
                    pow_expr.add_node(left);
                    pow_expr.add_node(self.expression_integer_or_name());
                    left = pow_expr;
                },
                _ => return left,
            }
        }
    }

    fn expression_mul(&mut self) -> ast::AstNode {
        let mut left = self.expression_pow();
        loop {
            let op = self.tokenizer.look_ahead(1);
            match op.token_type {
                TokenType::Mul | TokenType::Div => {
                    let mut mul_expr = ast::AstNode::new(ast::NodeType::Mul, op);
                    self.tokenizer.eat(1);
                    mul_expr.add_node(left);
                    mul_expr.add_node(self.expression_pow());
                    left = mul_expr;
                },
                _ => return left,
            }
        }
    }
    fn expression_add(&mut self) -> ast::AstNode {
        let mut left = self.expression_mul();
        loop {
            let op = self.tokenizer.look_ahead(1);
            match op.token_type {
                TokenType::Add | TokenType::Sub | TokenType::Mod => {
                    let mut add_expr = ast::AstNode::new(ast::NodeType::Add, op);
                    self.tokenizer.eat(1);
                    add_expr.add_node(left);
                    add_expr.add_node(self.expression_mul());
                    left = add_expr;
                },
                _ => return left,
            }
        }
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
            _ => panic!("syntax error, line:{}, column:{}, expect '=', found '{}'", assign.row, assign.col, assign.literal),
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
                TokenType::Newline => { self.tokenizer.eat(1); continue; },
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
