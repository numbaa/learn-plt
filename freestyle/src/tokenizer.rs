struct Tokenizer {
    content: String
    current_pos: i32
}

enum TokenType {
    Assign,
    Operator,
    Name,
    Number,
}

struct Token {
    type: TokenType,
    string: String,
    value: f64,
}

impl Tokenizer {
    pub fn next() -> &String {

    }
}