pub mod token {

    #[derive(Clone)]
    pub enum TokenType {
        LeftParenthesis, 
        RightParenthesis,
        LeftBrace, 
        RightBrace,
        COMMA,
        DOT,
        MINUS,
        PLUS,
        SEMICOLON,
        SLASH,
        STAR,

        BANG,
        BangEqual,
        EQUAL,
        EqualEqual,
        GREATER,
        GreaterEqual,
        LESS,
        LessEqual,

        IDENTIFIER,
        STRING,
        NUMBER,

        AND,
        CLASS,
        ELSE,
        FALSE,
        FN,
        FOR,
        IF,
        NO,
        OR,
        PRINT,
        RETURN,
        SUPER,
        THIS,
        TRUE,
        VAR,
        WHILE,

        EOF,
    }

    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub literal: String,
        pub line: i32
    }
}

