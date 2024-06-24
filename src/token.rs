enum TokenType {
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

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32
}



