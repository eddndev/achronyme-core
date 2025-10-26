#include "evaluator.hpp"
#include <stdexcept>

namespace achronyme {
namespace parser {

core::Value Evaluator::evaluate(const ASTNode* node) {
    switch (node->type()) {
        case ASTNodeType::NUMBER:
            return evaluateNumber(static_cast<const NumberNode*>(node));

        case ASTNodeType::BINARY_OP:
            return evaluateBinaryOp(static_cast<const BinaryOpNode*>(node));

        case ASTNodeType::UNARY_OP:
            return evaluateUnaryOp(static_cast<const UnaryOpNode*>(node));

        default:
            throw std::runtime_error("Unknown AST node type");
    }
}

core::Value Evaluator::evaluateNumber(const NumberNode* node) {
    return core::Value(node->value());
}

core::Value Evaluator::evaluateBinaryOp(const BinaryOpNode* node) {
    // Post-order: evaluate children first
    auto left = evaluate(node->left());
    auto right = evaluate(node->right());

    // Apply operator
    switch (node->op()) {
        case BinaryOp::ADD:
            return left + right;

        case BinaryOp::SUBTRACT:
            return left - right;

        case BinaryOp::MULTIPLY:
            return left * right;

        case BinaryOp::DIVIDE:
            return left / right;

        case BinaryOp::POWER:
            return left.pow(right);

        default:
            throw std::runtime_error("Unknown binary operator");
    }
}

core::Value Evaluator::evaluateUnaryOp(const UnaryOpNode* node) {
    auto operand = evaluate(node->operand());

    switch (node->op()) {
        case UnaryOp::NEGATE:
            return -operand;

        default:
            throw std::runtime_error("Unknown unary operator");
    }
}

} // namespace parser
} // namespace achronyme
