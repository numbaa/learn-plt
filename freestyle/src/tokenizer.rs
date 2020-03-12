//pub mod tokenizer {

pub struct Tokenizer {
    pub content: String,
    pub current_pos: i32
}

enum TokenType {
    Assign,
    Operator,
    Name,
    Number,
}

struct Token {
    token_type: TokenType,
    string: String,
    value: f64,
}

impl Tokenizer {
    pub fn next() -> String {
        String::from("")
    }
}
//}