
mod scanner {
    use crate::token::token::Token;
    use crate::token::token::TokenType;
    use crate::error;
    use std::collections::HashMap;

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

            let match_equals = self.match_char('=');
            let match_slash = self.match_char('/');
            let match_r = self.match_char('r');

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

                '!' => self.add_token(
                    if match_equals
                      { TokenType::BangEqual } 
                    else 
                      { TokenType::BANG }, 
                "".to_string()
                ),
                '=' => self.add_token( 
                    if match_equals
                      { TokenType::EqualEqual } 
                    else 
                      { TokenType::EQUAL }, 
                "".to_string()
                ),
                '<' => self.add_token( 
                    if match_equals
                      { TokenType::LessEqual } 
                    else 
                      { TokenType::LESS }, 
                "".to_string()
                ),
                '>' => self.add_token( 
                    if match_equals 
                      { TokenType::LessEqual } 
                    else 
                      { TokenType::LESS }, 
                "".to_string()
                ),
                '/' =>  
                    if match_slash { 
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                         self.add_token(TokenType::SLASH, "".to_string())
                    }
                ' ' => println!(""),
                '\r' => println!(""),
                '\t' => println!(""),
                '\n' => self.line += 1,
                '"' => self.its_string(),
                'o' => if match_r {
                    self.add_token(TokenType::OR, "".to_string())
                },
                value => if self.is_digit(value) {
                    self.number()
                } else if self.is_alpha(value){
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character".to_string());
                }
            }
        }

        fn identifier(&mut self) -> () {
            let keyword = HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::CLASS),
                ("for", TokenType::FOR),
                ("fn", TokenType::FN),
                ("if", TokenType::IF),
                ("no", TokenType::NO),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]);

            while self.is_alpha_numeric(self.peek()) { self.advance(); }

            let text: String = self.source[(self.start) as usize..(self.current) as usize].to_string();


            let token_type : Option<&TokenType> = keyword.get(&*text);

            match token_type {
                Some(token) => {
                    let token : TokenType = token.clone();
                    self.add_token(token, String::from("")) 
                }
                None => self.add_token(TokenType::IDENTIFIER, String::from(""))
            }
        }

        fn is_digit(&self, character: char) -> bool {
            character >= '0' && character <= '9'
        }

        fn is_alpha(&self, character: char) -> bool {
            (character >= 'a' && character <= 'z') ||
            (character >= 'A' && character <= 'Z') ||
            character == '_'
        }

        fn is_alpha_numeric(&self, character: char) -> bool {
            self.is_alpha(character) || self.is_digit(character)
        }

        fn number(&mut self) -> () {
            while self.is_digit(self.peek()) { self.advance(); }

            if self.peek() == '.' && self.is_digit(self.peek_next()) {
                self.advance();
            }

            while self.is_digit(self.peek()) { self.advance(); }

            let double_source: f32 = self.source[self.start as usize..self.current as usize].to_string().parse().expect("this is not a double");

            self.add_token(TokenType::NUMBER, double_source.to_string())

        }

        fn advance(&self) -> char {
            let index : usize = usize::try_from(self.current + 1).expect("can't change i32 to usize");
            let result = self.source.chars().nth(index).expect("index out of bound");
            result
        }

        fn match_char(&mut self, expected: char) -> bool {
            let index : usize = usize::try_from(self.current).
                expect("can't change i32 to usize");
            let result = self.source.chars().nth(index).expect("index out of bound");

            if self.is_at_end() { return false };
            if result != expected { return false };

            self.current += 1;

            true
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

        fn peek(&self) -> char {
            if self.is_at_end() { return '\0' };

            let index : usize = usize::try_from(self.current).
                expect("can't change i32 to usize");
            let result = self.source.chars().nth(index).expect("index out of bound");

            result
        }

        fn peek_next(&self) -> char {
            if self.current + 1 >= self.source.len().try_into().unwrap() { return '\0' }
            let index : usize = usize::try_from(self.current).expect("can't change i32 to usize");

            self.source.chars().nth(index).expect("index out of bound")

        }

        fn its_string(&mut self) -> () {
            while self.peek() != '"' && !self.is_at_end() {
                if self.peek() != '\n' { self.line += 1 }
                self.advance();
            }

            if self.is_at_end() {
                error(self.line, "Unexpected character".to_string());
            }

            self.advance();

            let value: String = self.source[(self.start + 1) as usize..(self.current - 1) as usize].to_string();
            self.add_token(TokenType::STRING, value)
        }
    }
}
