#ifndef ACHRONYME_PARSER_EVALUATOR_HPP
#define ACHRONYME_PARSER_EVALUATOR_HPP

#include "ast.hpp"
#include "../core/value.hpp"

namespace achronyme {
namespace parser {

/**
 * Evaluator
 *
 * Walks the AST and computes the result.
 * Uses a post-order traversal (visit children before parent).
 *
 * Example:
 *       +
 *      / \
 *     2   *
 *        / \
 *       3   4
 *
 * Evaluation order:
 *   1. eval(2) → 2
 *   2. eval(3) → 3
 *   3. eval(4) → 4
 *   4. eval(3*4) → 12
 *   5. eval(2+12) → 14
 */
class Evaluator {
public:
    Evaluator() = default;

    // Evaluate an AST and return the result
    core::Value evaluate(const ASTNode* node);

private:
    // Helper methods for each node type
    core::Value evaluateNumber(const NumberNode* node);
    core::Value evaluateBinaryOp(const BinaryOpNode* node);
    core::Value evaluateUnaryOp(const UnaryOpNode* node);
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_EVALUATOR_HPP
