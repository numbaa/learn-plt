
use std::fs;
mod tokenizer;
mod parser;
mod ast;
mod interpreter;
mod ntable;

pub fn run(filename: &String) -> Result<(),()> {
    
    let contents = fs::read_to_string(filename)
        .expect(&format!("Read content from {} failed", filename));
    let tokenizer = tokenizer::Tokenizer::new(&contents);
    let mut parser = parser::Parser::new(tokenizer);
    let tree = parser.parse();
    let mut intp = interpreter::Interpreter::new();
    intp.execute(&tree);
    return Ok(())
}
