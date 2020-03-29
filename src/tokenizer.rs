use std::vec;
use std::collections::VecDeque;

pub struct Tokenizer {
    current_pos: usize,
    chars: vec::Vec<char>,
    tokens: VecDeque<Token>,
    current_line: u32,
    current_column: u32,
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
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::EOF,
            literal: "".to_string(),
            row: 0,
            col: 0,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            token_type: self.token_type,
            literal: self.literal.clone(),
            row: self.row,
            col: self.col,
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
                self.current_column += 1;
                continue;
            }
            return;
        }
    }

    fn eof(&self) -> Result<Token, String> {
        Ok(Token {
            token_type: TokenType::EOF,
            literal: String::from(""),
            row: self.current_line,
            col: self.current_column,
        })
    }

    //TODO: support _ and digit in name
    //FIXME: if the last line like 'print a' without <newline>, it will crash because of OUT OF INDEX
    fn name_or_print(&mut self) -> Result<Token, String> {
        let mut string = String::from("");
        let old_column = self.current_column;
        loop {
            let c = self.chars[self.current_pos];
            if c.is_alphabetic() {
                string.push(c);
                self.current_pos += 1;
                self.current_column += 1;
                continue;
            }
            if c.is_whitespace() {
                let result: Result<Token, String>;
                self.current_pos += 1;
                if string == "print".to_string() {
                    result = Ok(Token {
                        literal: "print".to_string(),
                        token_type: TokenType::Print,
                        row: self.current_line,
                        col: old_column,
                    });
                } else {
                    result =  Ok(Token {
                        literal: string.clone(),
                        token_type: TokenType::Name,
                        row: self.current_line,
                        col: old_column,
                    })
                }
                if c == '\n' {
                    self.current_line += 1;
                    self.current_pos = 1;
                } else {
                    self.current_column += 1;
                }
                return result;
            }
            return Err(format!("unexpected character {}", c).to_string());
        }
    }

    fn integer(&mut self) -> Result<Token, String> {
        let mut string = String::from("");
        let old_column = self.current_column;
        loop {
            let c = self.chars[self.current_pos];
            if c.is_digit(10) {
                string.push(c);
                self.current_pos += 1;
                self.current_column += 1;
                continue;
            }
            if c.is_whitespace() || c.is_ascii_punctuation() {
                self.current_pos += 1;
                let result = Ok(Token {
                    literal: string.clone(),
                    token_type: TokenType::Integer,
                    row: self.current_line,
                    col: old_column,
                });
                if c == '\n' {
                    self.current_line += 1;
                    self.current_column = 1;
                } else {
                    self.current_column += 1;
                }
                return result;
            }
            return Err(format!("unexpected character {}", c).to_string());
        }
    }

    fn assign(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn add(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Add,
            literal: "+".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn sub(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Sub,
            literal: "-".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn mul(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Mul,
            literal: "*".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn div(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Div,
            literal: "/".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn pow(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Pow,
            literal: "^".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }

    fn remainder(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        self.current_column += 1;
        Ok(Token {
            token_type: TokenType::Mod,
            literal: "%".to_string(),
            row: self.current_line,
            col: self.current_column - 1,
        })
    }


    fn new_line(&mut self) -> Result<Token, String> {
        self.current_pos += 1;
        let result = Ok(Token {
            token_type: TokenType::Newline,
            literal: "\n".to_string(),
            row: self.current_line,
            col: self.current_column,
        });
        self.current_line += 1;
        self.current_column = 1;
        return result;
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

    pub fn look_ahead(&mut self, n: usize) -> Result<Token, String> {
        assert_ne!(n, 0);
        let mut has_read = self.tokens.len();
        while has_read < n {
            let next_token = match self.next() {
                Ok(token) => token,
                Err(msg) => return Err(msg),
            };
            self.tokens.push_back(next_token.clone());
            has_read = self.tokens.len();
            match next_token.token_type {
                TokenType::EOF => return match self.tokens.back() {
                    Some(token) => Ok(token.clone()),
                    None => panic!(format!("line:{}, column:{}, fatal error, it should be a TokenType::EOF here",
                        next_token.row, next_token.col)),
                },
                _ => (),
            }
        }
        return Ok(self.tokens[n-1].clone());
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
            current_column: 1,
            current_line: 1,
        };
        for c in s.chars() {
            t.chars.push(c);
        }
        t
    }
}
