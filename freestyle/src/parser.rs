use super::tokenizer::*;
use super::ast;

pub struct Parser {
    tokenizer: Tokenizer
}

impl Parser {
    fn function_call(&mut self, func: Token) -> Result<ast::AstNode, String> {
        self.tokenizer.eat(1);
        let mut func_node = ast::AstNode::new(ast::NodeType::FuncCall, func);
        loop {
            let arg = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            self.tokenizer.eat(1);
            match arg.token_type {
                TokenType::Symbol => (),
                _ => return Err(format!("line:{}, column:{}, syntax error, expect argument, found '{}'",
                    arg.row, arg.col, arg.literal)),
            }
            func_node.add_node(ast::AstNode::new(ast::NodeType::Arg, arg));
            let next_token = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match next_token.token_type {
                TokenType::Comma => (),
                TokenType::RP => break,
                _ => return Err(format!("line:{}, column:{}, syntax error, expect ',' or ')'), found '{}'",
                    next_token.row, next_token.col, next_token.literal)),
            }
        }
        Ok(func_node)
    }
    fn expression_integer_or_name(&mut self) -> Result<ast::AstNode, String> {
        let token = match self.tokenizer.look_ahead(1) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        self.tokenizer.eat(1);
        match token.token_type {
            TokenType::Integer => return Ok(ast::AstNode::new(ast::NodeType::Integer, token)),
            TokenType::Symbol => {
                let next_token = match self.tokenizer.look_ahead(1) {
                    Ok(token) => token,
                    Err(msg) => return Err(msg),
                };
                match next_token.token_type {
                    TokenType::LP => return self.function_call(token),
                    _ => return Ok(ast::AstNode::new(ast::NodeType::Name, token))
                }
            },
            _ => return Err(format!("line:{}, column:{}, syntax error, expect integer or variable or function",
                    token.row, token.col)),
        }
    }
    fn expression_pow(&mut self) -> Result<ast::AstNode, String> {
        let mut left = match self.expression_integer_or_name() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        };
        loop {
            let op = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match op.token_type {
                TokenType::Pow => {
                    let mut pow_expr = ast::AstNode::new(ast::NodeType::Pow, op);
                    self.tokenizer.eat(1);
                    pow_expr.add_node(left);
                    pow_expr.add_node(match self.expression_integer_or_name() {
                        Ok(node) => node,
                        Err(msg) => return Err(msg),
                    });
                    left = pow_expr;
                },
                _ => return Ok(left),
            }
        }
    }

    fn expression_mul(&mut self) -> Result<ast::AstNode, String> {
        let mut left = match self.expression_pow() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        };
        loop {
            let op = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match op.token_type {
                TokenType::Mul | TokenType::Div => {
                    let mut mul_expr = ast::AstNode::new(ast::NodeType::Mul, op);
                    self.tokenizer.eat(1);
                    mul_expr.add_node(left);
                    mul_expr.add_node(match self.expression_pow() {
                        Ok(node) => node,
                        Err(msg) => return Err(msg),
                    });
                    left = mul_expr;
                },
                _ => return Ok(left),
            }
        }
    }
    fn expression_add(&mut self) -> Result<ast::AstNode, String> {
        let mut left = match self.expression_mul() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        };
        loop {
            let op = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match op.token_type {
                TokenType::Add | TokenType::Sub | TokenType::Mod => {
                    let mut add_expr = ast::AstNode::new(ast::NodeType::Add, op);
                    self.tokenizer.eat(1);
                    add_expr.add_node(left);
                    add_expr.add_node(match self.expression_mul() {
                        Ok(node) => node,
                        Err(msg) => return Err(msg),
                    });
                    left = add_expr;
                },
                _ => return Ok(left),
            }
        }
    }

    fn expression(&mut self) -> Result<ast::AstNode, String> {
        return self.expression_add();
    }

    fn statement_print(&mut self, parent: &mut ast::AstNode) -> Result<(), String> {
        let mut print_node = ast::AstNode::new(ast::NodeType::Print, match self.tokenizer.look_ahead(1) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        });
        self.tokenizer.eat(1);
        print_node.add_node(match self.expression() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        });
        parent.add_node(print_node);
        return Ok(())
    }

    fn statement_assign(&mut self, parent: &mut ast::AstNode) -> Result<(), String> {
        let name = match self.tokenizer.look_ahead(1) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        let assign = match self.tokenizer.look_ahead(2) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        self.tokenizer.eat(2);
        match assign.token_type {
            TokenType::Assign => (),
            _ => return Err(format!("line:{}, column:{}, syntax error, expect '=', found '{}'",
                    assign.row, assign.col, assign.literal)),
        }
        let mut assign_node = ast::AstNode::new(ast::NodeType::Assign, assign);
        let name_node = ast::AstNode::new(ast::NodeType::Name, name);
        assign_node.add_node(name_node);
        assign_node.add_node(match self.expression() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        });
        parent.add_node(assign_node);
        return Ok(())
    }

    fn parameters_node(&mut self) -> Result<ast::AstNode, String> {
        let lp = match self.tokenizer.look_ahead(1) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        self.tokenizer.eat(1);
        match lp.token_type {
            TokenType::LP => (),
            _ => return Err(format!("line:{}, column:{}, syntax error, expect '('), found '{}'",
                lp.row, lp.col, lp.literal)),
        }
        let mut param_node = ast::AstNode::new(ast::NodeType::ParamList, lp);
        loop {
            let param = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            self.tokenizer.eat(1);
            match param.token_type {
                TokenType::Symbol => (),
                _ => return Err(format!("line:{}, column:{}, syntax error, expect parameter, found '{}'",
                    param.row, param.col, param.literal)),
            }
            param_node.add_node(ast::AstNode::new(ast::NodeType::Param, param));
            let next_token = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match next_token.token_type {
                TokenType::Comma => (),
                TokenType::RP => break,
                _ => return Err(format!("line:{}, column:{}, syntax error, expect ',' or ')'), found '{}'",
                    next_token.row, next_token.col, next_token.literal)),
            }
        }
        Ok(param_node)
    }

    fn function_body(&mut self) -> Result<ast::AstNode, String> {
        let lbraceket = match self.tokenizer.look_ahead(1) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        self.tokenizer.eat(1);
        match lbraceket.token_type {
            TokenType::LBraceket => (),
            _ => return Err(format!("line:{}, column:{}, syntax error, expect '('), found '{}'",
                lbraceket.row, lbraceket.col, lbraceket.literal)),
        }
        let mut body = ast::AstNode::new(ast::NodeType::FuncBody, lbraceket);
        loop {
            let token = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            } ;
            match token.token_type {
                TokenType::Print => {
                    let result = self.statement_print(&mut body);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                },
                TokenType::Symbol => {
                    let result = self.statement_assign(&mut body);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                },
                TokenType::Return => {
                    let mut ret = ast::AstNode::new(ast::NodeType::Return, token);
                    self.tokenizer.eat(1);
                    let expr = match self.expression() {
                        Ok(token) => token,
                        Err(msg) => return Err(msg),
                    };
                    ret.add_node(expr);
                    body.add_node(ret);
                    return Ok(body);
                }
                TokenType::Newline => { self.tokenizer.eat(1); continue; },
                _ => return Err(format!("line:{}, column:{}, syntax error, expect 'print' or 'variable'",
                        token.row, token.col)),
            }
        }
    }

    fn statement_func_decl(&mut self, parent: &mut ast::AstNode) -> Result<(), String> {
        let func_name = match self.tokenizer.look_ahead(2) {
            Ok(token) => token,
            Err(msg) => return Err(msg),
        };
        self.tokenizer.eat(2);
        match func_name.token_type {
            TokenType::Symbol => (),
            _ => return Err(format!("line:{}, column:{}, syntax error, expect function name, found '{}'",
            func_name.row, func_name.col, func_name.literal)),
        }
        let mut func_decl_node = ast::AstNode::new(ast::NodeType::FuncDecl, func_name);
        let parameters_node = match self.parameters_node() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        };
        let func_body = match self.function_body() {
            Ok(node) => node,
            Err(msg) => return Err(msg),
        };
        func_decl_node.add_node(parameters_node);
        func_decl_node.add_node(func_body);
        parent.add_node(func_decl_node);
        Ok(())
    }

    pub fn parse(&mut self) -> Result<ast::AstNode, String> {
        let mut ast_root = ast::AstNode::new(ast::NodeType::Root, Token::new());
        loop {
            let token = match self.tokenizer.look_ahead(1) {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            match token.token_type {
                TokenType::Print => {
                    let result = self.statement_print(&mut ast_root);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                },
                TokenType::Symbol => {
                    let result = self.statement_assign(&mut ast_root);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                },
                TokenType::FuncDecl => {
                    let result = self.statement_func_decl(&mut ast_root);
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                },
                TokenType::Newline => { self.tokenizer.eat(1); continue; },
                TokenType::EOF => return Ok(ast_root),
                _ => return Err(format!("line:{}, column:{}, syntax error, expect 'print' or 'variable'",
                        token.row, token.col)),
            }
        }
    }

    pub fn new(tokenizer: Tokenizer) -> Parser {
        return Parser {
            tokenizer: tokenizer
        }
    }
}
