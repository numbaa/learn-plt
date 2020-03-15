
use std::fs;
mod tokenizer;

pub fn run(filename: &String) -> Result<(),()> {
    
    let contents = fs::read_to_string(filename)
        .expect(&format!("Read content from {} failed", filename));
    let mut tokenizer = tokenizer::Tokenizer::new(&contents);
    tokenizer.next();
    return Ok(())
}
