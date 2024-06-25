use token;

pub mod scanner {

    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: i32,
        current: i32,
        line: i32,
    }

    pub impl Scanner {
        pub fn scanTokens(&self) -> Vec<Token> {
            while !is_at_end() {
                self.start = self.current
            }

            self.tokens.add(token::Token(
                    token::TokenType::EOF,
                    "", 
                    "",
                    0
                    ));

            self.tokens
        }

        fn is_at_end(&self) {
            self.current >= source.len();
        }

        fn scanToken() {
            character = '[';
            match character {
                '(' => add_token(token::TokenType::LeftParenthesis),
                ')' => add_token(token::TokenType::RightParenthesis),
                '{' => add_token(token::TokenType::LeftBrace),
                '}' => add_token(token::TokenType::RightBrace),
                ',' => add_token(token::TokenType::COMMA),
                '.' => add_token(token::TokenType::DOT),
                '-' => add_token(token::TokenType::MINUS),
                '+' => add_token(token::TokenType::PLUS),
                ';' => add_token(token::TokenType::SEMICOLON),
                '*' => add_token(token::TokenType::STAR),
            }
        }

        fn advance(&self) -> char {
            self.source.chars().nth(self.current++)
        }

        fn add_token(&self, token_type: token::TokenType) -> char {
            self.tokens.add(token::Token(
                    token::TokenType::EOF,
                    "", 
                    "",
                    0
                    ));
        }

    }

}
