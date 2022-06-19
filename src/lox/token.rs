use std::fmt::Write;

#[derive(Debug)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u64,
}

impl Token {
    pub fn new(tok_type: TokenType, lexeme: String, literal: Literal, line: u64) -> Self {
        Token {
            tok_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        write!(
            &mut str,
            "{:?} {} {:?}",
            self.tok_type, self.lexeme, self.literal
        )
        .unwrap();
        str
    }
}

#[derive(Debug)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

#[derive(Debug)]
pub enum Literal {
    NONE,
    STRING(String),
    INT(i64),
}
