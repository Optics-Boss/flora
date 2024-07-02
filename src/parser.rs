pub mod parser {
    use crate::token::token::{Token, TokenType};

    struct expression {
        left: String,
        operator: Token,
        right: String
    }

    struct parser {
        tokens: Vec<Token>,
        current: i32
    }

    impl parser {
        fn get_expression(&self) {
            self.equality()
        }

        fn equality(&self) {
            let exp : expression = self.comparison();

            while self.match_expression(TokenType::BangEqual, TokenType::EqualEqual) {

            }
        }

    }

}
