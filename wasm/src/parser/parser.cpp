#include "parser.hpp"
#include <stdexcept>

namespace achronyme {
namespace parser {

Parser::Parser(const std::vector<Token>& tokens)
    : tokens_(tokens), current_(0) {}

std::unique_ptr<ASTNode> Parser::parse() {
    return statement();
}

// statement → let IDENTIFIER = expression | expression
std::unique_ptr<ASTNode> Parser::statement() {
    // Check for let declaration
    if (match(TokenType::LET)) {
        // let IDENTIFIER = expression
        if (!check(TokenType::IDENTIFIER)) {
            throw std::runtime_error("Expected variable name after 'let'");
        }

        std::string name = advance().lexeme;

        consume(TokenType::ASSIGN, "Expected '=' after variable name");

        auto initializer = expression();

        return std::make_unique<VariableDeclarationNode>(name, std::move(initializer));
    }

    // Otherwise, it's just an expression
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

// expression → comparison
std::unique_ptr<ASTNode> Parser::expression() {
    return comparison();
}

// comparison → additive (('>' | '<' | '>=' | '<=' | '==' | '!=') additive)*
std::unique_ptr<ASTNode> Parser::comparison() {
    auto node = additive();

    while (match(TokenType::GT) || match(TokenType::LT) ||
           match(TokenType::GTE) || match(TokenType::LTE) ||
           match(TokenType::EQ) || match(TokenType::NEQ)) {

        TokenType op = previous().type;
        auto right = additive();

        BinaryOp binOp;
        switch (op) {
            case TokenType::GT:  binOp = BinaryOp::GT;  break;
            case TokenType::LT:  binOp = BinaryOp::LT;  break;
            case TokenType::GTE: binOp = BinaryOp::GTE; break;
            case TokenType::LTE: binOp = BinaryOp::LTE; break;
            case TokenType::EQ:  binOp = BinaryOp::EQ;  break;
            case TokenType::NEQ: binOp = BinaryOp::NEQ; break;
            default:
                throw std::runtime_error("Unknown comparison operator");
        }

        node = std::make_unique<BinaryOpNode>(binOp, std::move(node), std::move(right));
    }

    return node;
}

// additive → term (('+' | '-') term)*
std::unique_ptr<ASTNode> Parser::additive() {
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

// primary → NUMBER 'i'? | IDENTIFIER ('(' args ')')? | '(' expression ')' 'i'? | '[' vector_or_matrix ']'
std::unique_ptr<ASTNode> Parser::primary() {
    // NUMBER with optional 'i' suffix
    if (match(TokenType::NUMBER)) {
        double value = previous().value;

        // Check for imaginary unit: 3i
        if (match(TokenType::IDENTIFIER) && previous().lexeme == "i") {
            return std::make_unique<ComplexLiteralNode>(value);  // 0 + value*i
        }

        return std::make_unique<NumberNode>(value);
    }

    // Handle identifiers (constants, function calls, lambdas, or 'i')
    if (match(TokenType::IDENTIFIER)) {
        std::string name = previous().lexeme;

        // Special case: standalone 'i' (imaginary unit)
        if (name == "i") {
            return std::make_unique<ComplexLiteralNode>(1.0);  // 0 + 1i
        }

        // Check for single-param lambda: param => expr
        if (check(TokenType::ARROW)) {
            advance(); // consume =>
            auto body = expression();
            return std::make_unique<LambdaNode>(std::vector<std::string>{name}, std::move(body));
        }

        // Check if it's a function call
        if (check(TokenType::LPAREN)) {
            return parseFunctionCall(name);
        }

        // Otherwise, it's a constant or variable reference
        return parseConstant(name);
    }

    // Parenthesized expression or multi-param lambda
    if (match(TokenType::LPAREN)) {
        // Lookahead to determine if this is a lambda or expression
        // Lambda: (x, y) => expr
        // Expression: (2 + 3)

        size_t savedPos = current_;

        // Try to parse as lambda parameters
        std::vector<std::string> params;
        bool isLambda = false;

        if (check(TokenType::IDENTIFIER)) {
            // Could be lambda
            params.push_back(advance().lexeme);

            // Check for more parameters
            while (match(TokenType::COMMA)) {
                if (!check(TokenType::IDENTIFIER)) {
                    // Not a lambda, restore position
                    current_ = savedPos;
                    goto parse_as_expression;
                }
                params.push_back(advance().lexeme);
            }

            // Must have RPAREN followed by ARROW
            if (match(TokenType::RPAREN) && check(TokenType::ARROW)) {
                isLambda = true;
                advance(); // consume =>
                auto body = expression();
                return std::make_unique<LambdaNode>(params, std::move(body));
            }
        }

        // Restore position and parse as expression
        current_ = savedPos;

parse_as_expression:
        auto node = expression();
        consume(TokenType::RPAREN, "Expected ')' after expression");

        // Check for imaginary unit: (2+3)i
        // In this case, the entire expression becomes the imaginary part
        if (match(TokenType::IDENTIFIER) && previous().lexeme == "i") {
            // We need to evaluate this at parse time if it's a simple number
            // For now, we'll handle this in a simplified way:
            // (expr)i is treated as a special case to be handled in evaluator
            // For simplicity, we'll return the node as-is and handle complex
            // construction via multiplication by 'i' in evaluator
            // Actually, let's create a UnaryOp or handle this differently

            // For now, let's just throw an error and implement the simple cases first
            throw std::runtime_error("Complex syntax (expr)i not yet fully supported. Use expr * i instead.");
        }

        return node;
    }

    // Vector or Matrix literal
    if (match(TokenType::LBRACKET)) {
        return parseVectorOrMatrix();
    }

    throw std::runtime_error("Expected expression");
}

// Parse function call: name '(' args ')'
std::unique_ptr<ASTNode> Parser::parseFunctionCall(const std::string& name) {
    consume(TokenType::LPAREN, "Expected '(' after function name");

    std::vector<std::unique_ptr<ASTNode>> args;

    // Parse arguments (if any)
    if (!check(TokenType::RPAREN)) {
        do {
            args.push_back(expression());
        } while (match(TokenType::COMMA));
    }

    consume(TokenType::RPAREN, "Expected ')' after arguments");

    return std::make_unique<FunctionCallNode>(name, std::move(args));
}

// Parse constant or variable reference
std::unique_ptr<ASTNode> Parser::parseConstant(const std::string& name) {
    // Phase 4A: Could be a constant (PI, E) or a variable (x, y)
    // Return a VariableReferenceNode and let the evaluator decide
    // (it will check variables first, then constants)
    return std::make_unique<VariableReferenceNode>(name);
}

// Parse vector or matrix starting with '['
// After consuming '[', we need to determine if this is:
//   - Vector: [expr, expr, expr]
//   - Matrix: [[expr, expr], [expr, expr]]
std::unique_ptr<ASTNode> Parser::parseVectorOrMatrix() {
    // We've already consumed the first '['

    // Check if this is a matrix (next token is '[')
    if (check(TokenType::LBRACKET)) {
        // This is a matrix: [[...], [...], ...]
        std::vector<std::vector<std::unique_ptr<ASTNode>>> rows;

        do {
            consume(TokenType::LBRACKET, "Expected '[' for matrix row");

            std::vector<std::unique_ptr<ASTNode>> row;
            if (!check(TokenType::RBRACKET)) {
                do {
                    row.push_back(expression());
                } while (match(TokenType::COMMA));
            }

            consume(TokenType::RBRACKET, "Expected ']' after matrix row");
            rows.push_back(std::move(row));

        } while (match(TokenType::COMMA));

        consume(TokenType::RBRACKET, "Expected ']' after matrix");

        // Validate that all rows have the same length
        if (!rows.empty()) {
            size_t expectedCols = rows[0].size();
            for (size_t i = 1; i < rows.size(); ++i) {
                if (rows[i].size() != expectedCols) {
                    throw std::runtime_error(
                        "Matrix rows must have the same number of elements. "
                        "Row 0 has " + std::to_string(expectedCols) +
                        " elements, but row " + std::to_string(i) +
                        " has " + std::to_string(rows[i].size()) + " elements."
                    );
                }
            }
        }

        return std::make_unique<MatrixLiteralNode>(std::move(rows));
    }

    // This is a vector: [expr, expr, expr]
    std::vector<std::unique_ptr<ASTNode>> elements;

    if (!check(TokenType::RBRACKET)) {
        do {
            elements.push_back(expression());
        } while (match(TokenType::COMMA));
    }

    consume(TokenType::RBRACKET, "Expected ']' after vector");

    return std::make_unique<VectorLiteralNode>(std::move(elements));
}

// Parse vector (called when we know it's a vector)
std::unique_ptr<ASTNode> Parser::parseVector(std::vector<std::unique_ptr<ASTNode>> firstRow) {
    return std::make_unique<VectorLiteralNode>(std::move(firstRow));
}

// Parse matrix (called when we know it's a matrix)
std::unique_ptr<ASTNode> Parser::parseMatrix(std::vector<std::unique_ptr<ASTNode>> firstRow) {
    std::vector<std::vector<std::unique_ptr<ASTNode>>> rows;
    rows.push_back(std::move(firstRow));

    // Continue parsing remaining rows
    while (match(TokenType::COMMA)) {
        consume(TokenType::LBRACKET, "Expected '[' for matrix row");

        std::vector<std::unique_ptr<ASTNode>> row;
        do {
            row.push_back(expression());
        } while (match(TokenType::COMMA));

        consume(TokenType::RBRACKET, "Expected ']' after matrix row");
        rows.push_back(std::move(row));
    }

    return std::make_unique<MatrixLiteralNode>(std::move(rows));
}

} // namespace parser
} // namespace achronyme
