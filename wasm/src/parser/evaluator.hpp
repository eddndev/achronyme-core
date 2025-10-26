#ifndef ACHRONYME_PARSER_EVALUATOR_HPP
#define ACHRONYME_PARSER_EVALUATOR_HPP

#include "ast.hpp"
#include "environment.hpp"
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

    // Get the environment (for testing/debugging)
    Environment& environment() { return env_; }
    const Environment& environment() const { return env_; }

    // Apply a lambda function with arguments (Phase 4A: Higher-order functions)
    core::Value applyFunction(const core::Function& func, const std::vector<core::Value>& args);

private:
    // Helper methods for each node type
    core::Value evaluateNumber(const NumberNode* node);
    core::Value evaluateBinaryOp(const BinaryOpNode* node);
    core::Value evaluateUnaryOp(const UnaryOpNode* node);
    core::Value evaluateFunctionCall(const FunctionCallNode* node);

    // Phase 3: Complex types
    core::Value evaluateComplexLiteral(const ComplexLiteralNode* node);
    core::Value evaluateVectorLiteral(const VectorLiteralNode* node);
    core::Value evaluateMatrixLiteral(const MatrixLiteralNode* node);

    // Phase 4A: Variables
    core::Value evaluateVariableDeclaration(const VariableDeclarationNode* node);
    core::Value evaluateVariableReference(const VariableReferenceNode* node);

    // Phase 4A: Lambdas
    core::Value evaluateLambda(const LambdaNode* node);

    // Environment for variable storage
    Environment env_;
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_EVALUATOR_HPP
