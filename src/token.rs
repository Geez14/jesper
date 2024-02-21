use std::collections::HashMap;

use lazy_static::lazy_static;

pub type Literal = String;

#[derive(Debug, Clone)]
pub struct Position {
    pub filename: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT,
    OCTAL,
    HEX,
    BINARY,
    INTEGER,
    FLOAT_LIT,
    DOUBLE_LIT,

    ASSIGN,       // =
    PLUS_ASSIGN,  // +=
    MINUS_ASSIGN, // -=
    MUL_ASSIGN,   // *=
    DIV_ASSIGN,   // /=
    REM_ASSIGN,   // %=

    SHL_ASSIGN, // >>=
    SHR_ASSIGN, // <<=
    XOR_ASSIGN, // ^=

    AND_ASSIGN, // &=
    OR_ASSIGN,  // |=

    AND,   // &
    OR,    // |
    XOR,   // ^
    SHL,   // <<
    SHR,   // >>
    TILDE, // ~

    PLUS,     // +
    MINUS,    // -
    ASTERISK, // *
    SLASH,    // /
    REM,      // %

    INC, // ++
    DEC, // --

    NOT, // !
    EQL, // ==
    NEQ, // !=
    GT,  // >
    LT,  // <
    GEQ, // <=
    LEQ, // >=

    TERNERY, // ?
    ARROW,   // ->
    DOT,     // .
    ELIPSE,  // ...

    LPAREN, // (
    LBRACE, // {
    LBRACK, // [
    COMMA,  // ,
    COLON,  // :

    RPAREN,    // )
    RBRACE,    // }
    RBRACK,    // ]
    SEMICOLON, // ;

    AUTO,
    BREAK,
    CASE,
    CHAR,
    CONST,
    CONTINUE,
    DEFAULT,
    DO,
    DOUBLE,
    ELSE,
    ENUM,
    EXTERN,
    FLOAT,
    FOR,
    GOTO,
    IF,
    INLINE,
    INT,
    LONG,
    REGISTER,
    RESTRICT,
    RETURN,
    SHORT,
    SIGNED,
    SIZEOF,
    STATIC,
    STRUCT,
    SWITCH,
    TYPEDEF,
    UNION,
    UNSIGNED,
    VOID,
    VOLATILE,
    WHILE,
}

lazy_static! {
    static ref TOKENS: HashMap<Token, &'static str> = HashMap::from([
        (Token::ILLEGAL, "ILLEGAL"),
        (Token::EOF, "EOF"),
        (Token::IDENT, "IDENT"),
        (Token::OCTAL, "OCTAL"),
        (Token::HEX, "HEX"),
        (Token::BINARY, "BINARY"),
        (Token::INTEGER, "INTEGER"),
        (Token::FLOAT_LIT, "FLOAT"),
        (Token::DOUBLE_LIT, "DOUBLE"),
        (Token::ASSIGN, "="),
        (Token::PLUS_ASSIGN, "+="),
        (Token::MINUS_ASSIGN, "-="),
        (Token::MUL_ASSIGN, "*="),
        (Token::DIV_ASSIGN, "/="),
        (Token::REM_ASSIGN, "%="),
        (Token::SHL_ASSIGN, ">>="),
        (Token::SHR_ASSIGN, "<<="),
        (Token::XOR_ASSIGN, "^="),
        (Token::AND_ASSIGN, "&="),
        (Token::OR_ASSIGN, "|="),
        (Token::AND, "&"),
        (Token::OR, "|"),
        (Token::XOR, "^"),
        (Token::SHL, "<<"),
        (Token::SHR, ">>"),
        (Token::TILDE, "~"),
        (Token::PLUS, "+"),
        (Token::MINUS, "-"),
        (Token::ASTERISK, "*"),
        (Token::SLASH, "/"),
        (Token::REM, "%"),
        (Token::INC, "++"),
        (Token::DEC, "--"),
        (Token::NOT, "!"),
        (Token::EQL, "=="),
        (Token::NEQ, "!="),
        (Token::GT, ">"),
        (Token::LT, "<"),
        (Token::GEQ, "<="),
        (Token::LEQ, ">="),
        (Token::TERNERY, "?"),
        (Token::ARROW, "->"),
        (Token::DOT, "."),
        (Token::ELIPSE, "..."),
        (Token::LPAREN, "("),
        (Token::LBRACE, "{"),
        (Token::LBRACK, "["),
        (Token::COMMA, ","),
        (Token::COLON, ":"),
        (Token::RPAREN, ")"),
        (Token::RBRACE, "}"),
        (Token::RBRACK, "]"),
        (Token::SEMICOLON, ";"),
        (Token::AUTO, "auto"),
        (Token::BREAK, "break"),
        (Token::CASE, "case"),
        (Token::CHAR, "char"),
        (Token::CONST, "const"),
        (Token::CONTINUE, "continue"),
        (Token::DEFAULT, "default"),
        (Token::DO, "do"),
        (Token::DOUBLE, "double"),
        (Token::ELSE, "else"),
        (Token::ENUM, "enum"),
        (Token::EXTERN, "extern"),
        (Token::FLOAT, "float"),
        (Token::FOR, "for"),
        (Token::GOTO, "goto"),
        (Token::IF, "if"),
        (Token::INLINE, "inline"),
        (Token::INT, "int"),
        (Token::LONG, "long"),
        (Token::REGISTER, "register"),
        (Token::RESTRICT, "restrict"),
        (Token::RETURN, "return"),
        (Token::SHORT, "short"),
        (Token::SIGNED, "signed"),
        (Token::SIZEOF, "sizeof"),
        (Token::STATIC, "static"),
        (Token::STRUCT, "struct"),
        (Token::SWITCH, "switch"),
        (Token::TYPEDEF, "typedef"),
        (Token::UNION, "union"),
        (Token::UNSIGNED, "unsigned"),
        (Token::VOID, "void"),
        (Token::VOLATILE, "volatile"),
        (Token::WHILE, "while"),
    ]);
}

impl Token {
    pub fn to_str(&self) -> &'static str {
        TOKENS[self]
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = HashMap::from([
        ("auto", Token::AUTO),
        ("break", Token::BREAK),
        ("case", Token::CASE),
        ("char", Token::CHAR),
        ("const", Token::CONST),
        ("continue", Token::CONTINUE),
        ("default", Token::DEFAULT),
        ("do", Token::DO),
        ("double", Token::DOUBLE),
        ("else", Token::ELSE),
        ("enum", Token::ENUM),
        ("extern", Token::EXTERN),
        ("float", Token::FLOAT),
        ("for", Token::FOR),
        ("goto", Token::GOTO),
        ("if", Token::IF),
        ("inline", Token::INLINE),
        ("int", Token::INT),
        ("long", Token::LONG),
        ("register", Token::REGISTER),
        ("restrict", Token::RESTRICT),
        ("return", Token::RETURN),
        ("short", Token::SHORT),
        ("signed", Token::SIGNED),
        ("sizeof", Token::SIZEOF),
        ("static", Token::STATIC),
        ("struct", Token::STRUCT),
        ("switch", Token::SWITCH),
        ("typedef", Token::TYPEDEF),
        ("union", Token::UNION),
        ("unsigned", Token::UNSIGNED),
        ("void", Token::VOID),
        ("volatile", Token::VOLATILE),
        ("while", Token::WHILE),
    ]);
}

pub fn lookup(ident: &str) -> Token {
    *KEYWORDS.get(ident).unwrap_or(&Token::IDENT)
}
