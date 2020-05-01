
use std::fs;
mod tokenizer;
mod parser;
mod ast;
mod interpreter;
mod ntable;
#[macro_use]
extern crate lazy_static;

pub fn run(filename: &String) -> Result<(),String> {
    
    let contents = fs::read_to_string(filename)
        .expect(&format!("Read content from {} failed", filename));
    let tokenizer = tokenizer::Tokenizer::new(&contents);
    let mut parser = parser::Parser::new(tokenizer);
    let tree = match parser.parse() {
        Ok(tree) => tree,
        Err(msg) => return Err(msg),
    };
    let mut intp = interpreter::Interpreter::new();
    return match intp.execute(&tree) {
        Ok(_) => Ok(()),
        Err(msg) => Err(msg),
    };
}
