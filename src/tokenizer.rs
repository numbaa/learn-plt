use std::vec;
use std::collections::VecDeque;

pub struct Tokenizer {
    current_pos: usize,
    chars: vec::Vec<char>,
    tokens: VecDeque<Token>,
}

#[derive(Copy, Clone)]
pub enum TokenType {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Name,
    Integer,
    Print,
    Newline,
    EOF,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::EOF,
            literal: "".to_string(),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            token_type: self.token_type,
            literal: self.literal.clone(),
        }
    }
}

impl Tokenizer {

    fn skip_whitespace(&mut self) {
        loop {
            if self.chars.len() > self.current_pos &&
                self.chars[self.current_pos].is_whitespace() &&
                self.chars[self.current_pos] != '\n' {
                self.current_pos += 1;
                continue;
            }
            return;
        }
    }

    fn eof(&self) -> Result<Token, String> {
        Ok(Token {
            token_type: TokenType::EOF,
            literal: String::from(""),
        })
    }

    //TODO: support _ and digit in name
    //FIXME: if the last line like 'print a' without <newline>, it will crash because of OUT OF INDEX
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
                        literal: "print".to_string(),
                        token_type: TokenType::Print,
                    })
                } else {
                    return Ok(Token {
                        literal: string.clone(),
                        token_type: TokenType::Name,
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
                    literal: string.clone(),
                    token_type: TokenType::Integer,
                })
            }
            return Err(format!("unexpected character {}", c).to_string());
        }
    }

    fn assign(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        })
    }

    fn add(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Add,
            literal: "+".to_string(),
        })
    }

    fn sub(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Sub,
            literal: "-".to_string(),
        })
    }

    fn mul(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Mul,
            literal: "*".to_string(),
        })
    }

    fn div(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Div,
            literal: "/".to_string(),
        })
    }

    fn pow(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Pow,
            literal: "^".to_string(),
        })
    }

    fn remainder(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Mod,
            literal: "%".to_string(),
        })
    }


    fn new_line(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        Ok(Token {
            token_type: TokenType::Newline,
            literal: "\n".to_string(),
        })
    }
    fn next(&mut self) -> Result<Token, String> {
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
            '\n' => return self.new_line(),
            '+' => return self.add(),
            '-' => return self.sub(),
            '*' => return self.mul(),
            '/' => return self.div(),
            '%' => return self.remainder(),
            '^' => return self.pow(),
            _ => return Err(format!("unexpected characters {}", next_char).to_string()),
        }
    }

    pub fn look_ahead(&mut self, n: usize) -> Token {
        assert_ne!(n, 0);
        let mut has_read = self.tokens.len();
        while has_read < n {
            let next_token = self.next().unwrap();
            let token_type = next_token.token_type;
            self.tokens.push_back(next_token);
            has_read = self.tokens.len();
            match token_type {
                TokenType::EOF => return self.tokens.back().unwrap().clone(),
                _ => (),
            }
        }
        return self.tokens[n-1].clone();
    }

    pub fn eat(&mut self, n: usize) {
        assert_ne!(n, 0);
        for _i in 0..n {
            self.tokens.pop_front();
        }
    }

    pub fn new(s: &String) -> Tokenizer {
        let mut t = Tokenizer {
            current_pos: 0,
            chars: vec::Vec::<char>::new(),
            tokens: VecDeque::<Token>::new(),
        };
        for c in s.chars() {
            t.chars.push(c);
        }
        t
    }
}
