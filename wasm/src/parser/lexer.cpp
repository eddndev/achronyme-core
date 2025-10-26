#include "lexer.hpp"
#include <cctype>
#include <stdexcept>

namespace achronyme {
namespace parser {

Lexer::Lexer(const std::string& source)
    : source_(source), current_(0) {}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;

    while (!isAtEnd()) {
        skipWhitespace();

        if (isAtEnd()) break;

        char c = peek();

        // Numbers
        if (std::isdigit(c) || c == '.') {
            tokens.push_back(scanNumber());
        }
        // Identifiers (functions, constants)
        else if (std::isalpha(c) || c == '_') {
            tokens.push_back(scanIdentifier());
        }
        // Operators and delimiters
        else if (c == '+' || c == '-' || c == '*' || c == '/' || c == '^' ||
                 c == '(' || c == ')' || c == ',') {
            tokens.push_back(scanOperator());
        }
        else {
            throw std::runtime_error(std::string("Unexpected character: ") + c);
        }
    }

    tokens.push_back(Token(TokenType::END, "", current_));
    return tokens;
}

bool Lexer::isAtEnd() const {
    return current_ >= source_.length();
}

char Lexer::peek() const {
    if (isAtEnd()) return '\0';
    return source_[current_];
}

char Lexer::peekNext() const {
    if (current_ + 1 >= source_.length()) return '\0';
    return source_[current_ + 1];
}

char Lexer::advance() {
    return source_[current_++];
}

void Lexer::skipWhitespace() {
    while (!isAtEnd() && std::isspace(peek())) {
        advance();
    }
}

Token Lexer::scanNumber() {
    size_t start = current_;
    std::string numStr;

    // Integer part
    while (!isAtEnd() && std::isdigit(peek())) {
        numStr += advance();
    }

    // Decimal part
    if (peek() == '.' && std::isdigit(peekNext())) {
        numStr += advance(); // consume '.'
        while (!isAtEnd() && std::isdigit(peek())) {
            numStr += advance();
        }
    }

    // Scientific notation (e.g., 1e-3, 2.5e10)
    if (peek() == 'e' || peek() == 'E') {
        numStr += advance(); // consume 'e' or 'E'
        if (peek() == '+' || peek() == '-') {
            numStr += advance();
        }
        while (!isAtEnd() && std::isdigit(peek())) {
            numStr += advance();
        }
    }

    double value = std::stod(numStr);
    return Token(TokenType::NUMBER, numStr, value, start);
}

Token Lexer::scanOperator() {
    size_t pos = current_;
    char c = advance();

    switch (c) {
        case '+': return Token(TokenType::PLUS, "+", pos);
        case '-': return Token(TokenType::MINUS, "-", pos);
        case '*': return Token(TokenType::STAR, "*", pos);
        case '/': return Token(TokenType::SLASH, "/", pos);
        case '^': return Token(TokenType::CARET, "^", pos);
        case '(': return Token(TokenType::LPAREN, "(", pos);
        case ')': return Token(TokenType::RPAREN, ")", pos);
        case ',': return Token(TokenType::COMMA, ",", pos);
        default:
            throw std::runtime_error(std::string("Unknown operator: ") + c);
    }
}

Token Lexer::scanIdentifier() {
    size_t start = current_;
    std::string identifier;

    // First character: letter or underscore
    while (!isAtEnd() && (std::isalnum(peek()) || peek() == '_')) {
        identifier += advance();
    }

    return Token(TokenType::IDENTIFIER, identifier, start);
}

} // namespace parser
} // namespace achronyme
