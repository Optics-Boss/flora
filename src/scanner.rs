
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
        pub fn scan_tokens(&mut self) -> Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current
            }

            self.tokens.push(Token {
                        token_type: TokenType::EOF,
                        lexeme: "".to_string(),
                        literal: "".to_string(),
                        line: 0
            });

            self.tokens
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.len().try_into().unwrap()
        }


        fn scan_token(&self) {
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
            }
        }

        fn advance(&self) -> char {
            let result = self.source.chars().nth(self.current += 1).unwrap_or("");
        }

        fn add_token(&self, token_type: token::TokenType, literal: String) -> char {
            let text: String = self.source[self.start..self.current];

            self.tokens.add(token::Token(
                    token::TokenType::EOF,
                    text, 
                    literal,
                    self.line
                    ));
        }
    }

}
