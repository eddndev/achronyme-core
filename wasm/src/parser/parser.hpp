#ifndef ACHRONYME_PARSER_PARSER_HPP
#define ACHRONYME_PARSER_PARSER_HPP

#include <memory>
#include <vector>
#include "lexer.hpp"
#include "ast.hpp"

namespace achronyme {
namespace parser {

/**
 * Recursive Descent Parser
 *
 * Grammar (with precedence):
 *   expression  → term (('+' | '-') term)*
 *   term        → factor (('*' | '/') factor)*
 *   factor      → exponent ('^' exponent)*   [right-associative]
 *   exponent    → '-' exponent | primary
 *   primary     → NUMBER | IDENTIFIER ('(' args ')')? | '(' expression ')'
 *   args        → expression (',' expression)*
 *
 * Precedence (highest to lowest):
 *   1. Parentheses ()
 *   2. Unary minus -
 *   3. Exponentiation ^ (right-associative: 2^3^2 = 2^(3^2) = 512)
 *   4. Multiplication *, Division /
 *   5. Addition +, Subtraction -
 */
class Parser {
public:
    explicit Parser(const std::vector<Token>& tokens);

    // Parse and return AST
    std::unique_ptr<ASTNode> parse();

private:
    std::vector<Token> tokens_;
    size_t current_;

    // Helper methods
    const Token& peek() const;
    const Token& previous() const;
    bool isAtEnd() const;
    const Token& advance();
    bool check(TokenType type) const;
    bool match(TokenType type);
    void consume(TokenType type, const std::string& message);

    // Grammar rules (in order of precedence)
    std::unique_ptr<ASTNode> expression();
    std::unique_ptr<ASTNode> term();
    std::unique_ptr<ASTNode> factor();
    std::unique_ptr<ASTNode> exponent();
    std::unique_ptr<ASTNode> primary();

    // Helper methods for Phase 2
    std::unique_ptr<ASTNode> parseFunctionCall(const std::string& name);
    std::unique_ptr<ASTNode> parseConstant(const std::string& name);
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_PARSER_HPP
