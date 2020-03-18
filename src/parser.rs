use super::tokenizer;

pub struct Parser {
    tokenizer: tokenizer::Tokenizer
}

//only for syntax check now
impl Parser {
    fn name_or_noe(&mut self, prev_token: tokenizer::Token) -> i64 {
        let token = self.tokenizer.next().unwrap();
        match token.token_type {
            tokenizer::TokenType::Operator => return self.expression(),
            tokenizer::TokenType::Newline | tokenizer::TokenType::EOF => return prev_token.value,
            _ => panic!("syntax error, expect operator or newline"),
        }
    }

    fn integer_or_ioe(&mut self, prev_token: tokenizer::Token) -> i64 {
        let token = self.tokenizer.next().unwrap();
        match token.token_type {
            tokenizer::TokenType::Operator => return self.expression(),
            tokenizer::TokenType::Newline | tokenizer::TokenType::EOF => return prev_token.value,
            _ => panic!("syntax error, expect operator or newline, found {}", token.string),
        }
    }
    fn expression(&mut self) -> i64 {
        let token = self.tokenizer.next().unwrap();
        match token.token_type {
            tokenizer::TokenType::Name => return self.name_or_noe(token),
            tokenizer::TokenType::Integer => return self.integer_or_ioe(token),
            _ => panic!("syntax error, expect variable or integer, found {}", token.string),
        }
    }
    
    fn statement_print(&mut self) {
        let value = self.expression();
        println!("print statement, value: {}", value);
    }

    fn statement_assign(&mut self, token: tokenizer::Token) {
        let name = token;
        let assign = self.tokenizer.next().unwrap();
        match assign.token_type {
            tokenizer::TokenType::Assign => println!(""),
            _ => panic!("syntax error, expect '=', found {}", assign.string),
        }
        let value = self.expression();
        println!("assign statement, value: {}", value);
    }

    pub fn parse(&mut self) {
        loop {
            let token = self.tokenizer.next().unwrap();
            match token.token_type {
                tokenizer::TokenType::Print => self.statement_print(),
                tokenizer::TokenType::Name => self.statement_assign(token),
                tokenizer::TokenType::Newline => continue,
                tokenizer::TokenType::EOF => return,
                _ => { println!("syntax error"); return; }
            }
        }
    }

    pub fn new(tokenizer: tokenizer::Tokenizer) -> Parser {
        return Parser {
            tokenizer: tokenizer
        }
    }
}