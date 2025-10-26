#include "parser.hpp"
#include <stdexcept>

namespace achronyme {
namespace parser {

Parser::Parser(const std::vector<Token>& tokens)
    : tokens_(tokens), current_(0) {}

std::unique_ptr<ASTNode> Parser::parse() {
    return expression();
}

// Helper methods
const Token& Parser::peek() const {
    return tokens_[current_];
}

const Token& Parser::previous() const {
    return tokens_[current_ - 1];
}

bool Parser::isAtEnd() const {
    return peek().type == TokenType::END;
}

const Token& Parser::advance() {
    if (!isAtEnd()) current_++;
    return previous();
}

bool Parser::check(TokenType type) const {
    if (isAtEnd()) return false;
    return peek().type == type;
}

bool Parser::match(TokenType type) {
    if (check(type)) {
        advance();
        return true;
    }
    return false;
}

void Parser::consume(TokenType type, const std::string& message) {
    if (check(type)) {
        advance();
        return;
    }
    throw std::runtime_error(message);
}

// Grammar rules

// expression → term (('+' | '-') term)*
std::unique_ptr<ASTNode> Parser::expression() {
    auto node = term();

    while (match(TokenType::PLUS) || match(TokenType::MINUS)) {
        TokenType op = previous().type;
        auto right = term();

        if (op == TokenType::PLUS) {
            node = std::make_unique<BinaryOpNode>(BinaryOp::ADD, std::move(node), std::move(right));
        } else {
            node = std::make_unique<BinaryOpNode>(BinaryOp::SUBTRACT, std::move(node), std::move(right));
        }
    }

    return node;
}

// term → factor (('*' | '/') factor)*
std::unique_ptr<ASTNode> Parser::term() {
    auto node = factor();

    while (match(TokenType::STAR) || match(TokenType::SLASH)) {
        TokenType op = previous().type;
        auto right = factor();

        if (op == TokenType::STAR) {
            node = std::make_unique<BinaryOpNode>(BinaryOp::MULTIPLY, std::move(node), std::move(right));
        } else {
            node = std::make_unique<BinaryOpNode>(BinaryOp::DIVIDE, std::move(node), std::move(right));
        }
    }

    return node;
}

// factor → exponent ('^' exponent)*
// Note: Right-associative! We need to handle this carefully
std::unique_ptr<ASTNode> Parser::factor() {
    auto node = exponent();

    if (match(TokenType::CARET)) {
        // Right-associative: parse the rest as a factor
        auto right = factor(); // Recursive call for right-associativity
        node = std::make_unique<BinaryOpNode>(BinaryOp::POWER, std::move(node), std::move(right));
    }

    return node;
}

// exponent → '-' exponent | primary
std::unique_ptr<ASTNode> Parser::exponent() {
    if (match(TokenType::MINUS)) {
        auto operand = exponent(); // Recursive for multiple negations: --5
        return std::make_unique<UnaryOpNode>(UnaryOp::NEGATE, std::move(operand));
    }

    return primary();
}

// primary → NUMBER | '(' expression ')'
std::unique_ptr<ASTNode> Parser::primary() {
    if (match(TokenType::NUMBER)) {
        return std::make_unique<NumberNode>(previous().value);
    }

    if (match(TokenType::LPAREN)) {
        auto node = expression();
        consume(TokenType::RPAREN, "Expected ')' after expression");
        return node;
    }

    throw std::runtime_error("Expected expression");
}

} // namespace parser
} // namespace achronyme
