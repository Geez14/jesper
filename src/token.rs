pub type Literal = String;

pub struct Position {
    pub filename: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[allow(non_camel_case_types)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT,

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

pub fn lookup(ident: &str) -> Token {
    match ident {
        "auto" => Token::AUTO,
        "break" => Token::BREAK,
        "case" => Token::CASE,
        "char" => Token::CHAR,
        "const" => Token::CONST,
        "continue" => Token::CONTINUE,
        "default" => Token::DEFAULT,
        "do" => Token::DO,
        "double" => Token::DOUBLE,
        "else" => Token::ELSE,
        "enum" => Token::ENUM,
        "extern" => Token::EXTERN,
        "float" => Token::FLOAT,
        "for" => Token::FOR,
        "goto" => Token::GOTO,
        "if" => Token::IF,
        "inline" => Token::INLINE,
        "int" => Token::INT,
        "long" => Token::LONG,
        "register" => Token::REGISTER,
        "restrict" => Token::RESTRICT,
        "return" => Token::RETURN,
        "short" => Token::SHORT,
        "signed" => Token::SIGNED,
        "sizeof" => Token::SIZEOF,
        "static" => Token::STATIC,
        "struct" => Token::STRUCT,
        "switch" => Token::SWITCH,
        "typedef" => Token::TYPEDEF,
        "union" => Token::UNION,
        "unsigned" => Token::UNSIGNED,
        "void" => Token::VOID,
        "volatile" => Token::VOLATILE,
        "while" => Token::WHILE,
        _ => Token::IDENT,
    }
}
