#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    Identifier(String),

    // Arithmetic Operators
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Caret,      // ^ (power)
    Modulo,     // %

    // Comparison Operators
    Gt,         // >
    Lt,         // <
    Gte,        // >=
    Lte,        // <=
    Eq,         // ==
    Neq,        // !=

    // Assignment and Arrow
    Assign,     // =
    Arrow,      // =>

    // Delimiters
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Semicolon,  // ;

    // Keywords
    Let,        // let

    // End of file
    Eof,
}
