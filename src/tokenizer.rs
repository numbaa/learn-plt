use std::vec;

pub struct Tokenizer {
    current_pos: usize,
    chars: vec::Vec<char>
}

enum TokenType {
    Assign,
    Operator,
    Name,
    Integer,
    Print,
    EOF,
}

pub struct Token {
    token_type: TokenType,
    string: String,
    value: i64,
}

impl Tokenizer {

    fn skip_whitespace(&mut self) {
        loop {
            if self.chars.len() < self.current_pos &&
                self.chars[self.current_pos].is_whitespace() {
                self.current_pos += 1;
                continue;
            }
            return;
        }
    }

    fn eof(&self) -> Result<Token, String> {
        Ok(Token {
            token_type: TokenType::EOF,
            string: String::from(""),
            value: 0,
        })
    }

    //TODO: support _ and digit in name
    fn name_or_print(&mut self) -> Result<Token, String> {
        let mut string = String::from("");
        loop {
            let c = self.chars[self.current_pos];
            if c.is_alphabetic() {
                string.push(c);
                self.current_pos += 1;
                continue;
            }
            if c.is_whitespace() {
                self.current_pos += 1;
                if string == "print".to_string() {
                    return Ok(Token {
                        string: "print".to_string(),
                        token_type: TokenType::Print,
                        value: 0,
                    })
                } else {
                    return Ok(Token {
                        string: string.clone(),
                        token_type: TokenType::Name,
                        value: 0,
                    })
                }
            }
            return Err(format!("unexpected character {}", c).to_string());
        }
    }

    fn integer(&mut self) -> Result<Token, String> {
        let mut string = String::from("");
        loop {
            let c = self.chars[self.current_pos];
            if c.is_digit(10) {
                string.push(c);
                self.current_pos += 1;
                continue;
            }
            if c.is_whitespace() {
                self.current_pos += 1;
                return Ok(Token {
                    string: string.clone(),
                    token_type: TokenType::Integer,
                    value: string.parse::<i64>().unwrap(),
                })
            }
            return Err(format!("unexpected character {}", c).to_string());
        }
    }

    fn assign(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Assign,
            string: "=".to_string(),
            value: 0,
        })
    }

    fn operator(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Operator,
            string: self.chars[self.current_pos-1].to_string(),
            value: 0,
        })
    }

    pub fn next(&mut self) -> Result<Token, String> {
        if self.current_pos == self.chars.len() {
            return self.eof();
        }
        self.skip_whitespace();
        let next_char = self.chars[self.current_pos];
        if next_char.is_alphabetic() || next_char == '_' {
            return self.name_or_print();
        }
        if next_char.is_digit(10) {
            return self.integer();
        }
        match next_char {
            '=' => return self.assign(),
            '+' | '-' | '*' | '/' | '^' | '%' => return self.operator(),
            _ => return Err(format!("unexpected characters {}", next_char).to_string()),
        }
    }

    pub fn new(s: &String) -> Tokenizer {
        let mut t = Tokenizer {
            current_pos: 0,
            chars: vec::Vec::<char>::new(),
        };
        for c in s.chars() {
            t.chars.push(c);
        }
        t
    }
}
