
use std::fs;

pub fn run(filename: &String) -> Result<(),()> {
    let contents = fs::read_to_string(filename)
        .expect(&format!("Read content from {} failed", filename));
    println!("Read text:\n\n{}", contents);
    return Ok(())
}
