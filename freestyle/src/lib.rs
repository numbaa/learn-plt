
use std::fs;
mod tokenizer;

pub fn run(filename: &String) -> Result<(),()> {
    
    let contents = fs::read_to_string(filename)
        .expect(&format!("Read content from {} failed", filename));
    let tokenizer = tokenizer::Tokenizer{
        content: contents,
        current_pos: 0,
    };
    println!("Read text:\n\n{}", tokenizer.content);
    return Ok(())
}
