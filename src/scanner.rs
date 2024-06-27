
mod scanner {
    use crate::token::token::Token;
    use crate::token::token::TokenType;

    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: i32,
        current: i32,
        line: i32,
    }

    impl Scanner {
        pub fn scan_tokens(&mut self) -> () {
            while !self.is_at_end() {
                self.start = self.current
            }

            self.tokens.push(Token {
                        token_type: TokenType::EOF,
                        lexeme: "".to_string(),
                        literal: "".to_string(),
                        line: 0
            });
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.len().try_into().unwrap()
        }


        fn scan_token(&mut self) {
            let character = self.advance();
            match character {
                '(' => self.add_token(TokenType::LeftParenthesis, "".to_string()),
                ')' => self.add_token(TokenType::RightParenthesis, "".to_string()),
                '{' => self.add_token(TokenType::LeftBrace, "".to_string()),
                '}' => self.add_token(TokenType::RightBrace, "".to_string()),
                ',' => self.add_token(TokenType::COMMA, "".to_string()),
                '.' => self.add_token(TokenType::DOT, "".to_string()),
                '-' => self.add_token(TokenType::MINUS, "".to_string()),
                '+' => self.add_token(TokenType::PLUS, "".to_string()),
                ';' => self.add_token(TokenType::SEMICOLON, "".to_string()),
                '*' => self.add_token(TokenType::STAR, "".to_string()),
                _ => print!("test"),
            }
        }

        fn advance(&self) -> char {
            let index : usize = usize::try_from(self.current + 1).expect("can't change i32 to usize");
            let result = self.source.chars().nth(index).expect("index out of bound");
            result
        }

        fn add_token(&mut self, token_type: TokenType, literal: String) -> () {
            let text: String = self.source[self.start as usize..self.current as usize].to_string();

            self.tokens.push(Token {
                        token_type,
                        lexeme: text.to_string(),
                        literal: literal.to_string(),
                        line: 0
            });
        }
    }

}
