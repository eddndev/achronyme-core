#ifndef ACHRONYME_PARSER_LEXER_HPP
#define ACHRONYME_PARSER_LEXER_HPP

#include <string>
#include <vector>
#include <optional>

namespace achronyme {
namespace parser {

/**
 * Token types for the lexer
 *
 * Phase 1: Arithmetic operators and numbers
 * Phase 2: Will add IDENTIFIER for functions
 */
enum class TokenType {
    // Literals
    NUMBER,         // 123, 3.14, .5, 2e-3

    // Operators
    PLUS,           // +
    MINUS,          // -
    STAR,           // *
    SLASH,          // /
    CARET,          // ^ (power)

    // Delimiters
    LPAREN,         // (
    RPAREN,         // )

    // End of input
    END
};

/**
 * Token structure
 */
struct Token {
    TokenType type;
    std::string lexeme;     // Original text
    double value;           // For NUMBER tokens
    size_t position;        // Position in source

    Token(TokenType type, std::string lexeme, size_t pos)
        : type(type), lexeme(lexeme), value(0.0), position(pos) {}

    Token(TokenType type, std::string lexeme, double value, size_t pos)
        : type(type), lexeme(lexeme), value(value), position(pos) {}
};

/**
 * Lexer (Tokenizer)
 *
 * Converts source string into a sequence of tokens.
 *
 * Example:
 *   "2 + 3 * 4" → [NUMBER(2), PLUS, NUMBER(3), STAR, NUMBER(4), END]
 */
class Lexer {
public:
    explicit Lexer(const std::string& source);

    // Tokenize the entire source
    std::vector<Token> tokenize();

private:
    std::string source_;
    size_t current_;

    // Helper methods
    bool isAtEnd() const;
    char peek() const;
    char peekNext() const;
    char advance();
    void skipWhitespace();

    // Token scanners
    Token scanNumber();
    Token scanOperator();
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_LEXER_HPP
