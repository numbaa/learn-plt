use std::vec;
use super::tokenizer;

pub enum NodeType {
    Root,
    Print,
    Assign,
    Add,
    Mul,
    Pow,
    Name,
    Integer,
    FuncDecl,
    FuncBody,
    FuncCall,
    Return,
    Param,
    Arg,
    ParamList,
    ArgList,
}

// pub trait NodeTrait {
//     pub token(&self) -> tokenizer::TokenType;
//     pub type(&self) -> NodeType;
//     pub childs(&self) -> std::vec::Vec<NodeTrait>;
// }

pub struct AstNode {
    pub token: tokenizer::Token,
    pub node_type: NodeType,
    pub childs: vec::Vec<AstNode>,
}

impl AstNode {
    pub fn new(node_type: NodeType, token: tokenizer::Token) -> AstNode {
        AstNode {
            token: token,
            node_type: node_type,
            childs: vec::Vec::<AstNode>::new(),
        }
    }

    pub fn add_node(&mut self, node: AstNode) {
        self.childs.push(node);
    }
}

