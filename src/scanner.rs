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

        }
    }

}
