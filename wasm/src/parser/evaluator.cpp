#include "evaluator.hpp"
#include "../core/constants.hpp"
#include "../core/functions.hpp"
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

        case ASTNodeType::FUNCTION_CALL:
            return evaluateFunctionCall(static_cast<const FunctionCallNode*>(node));

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

core::Value Evaluator::evaluateFunctionCall(const FunctionCallNode* node) {
    const std::string& name = node->name();
    const auto& argNodes = node->args();

    // Check if it's a constant (zero arguments)
    if (argNodes.empty()) {
        auto& constRegistry = core::constants::ConstantsRegistry::instance();
        if (constRegistry.hasConstant(name)) {
            return core::Value(constRegistry.getConstant(name));
        }
    }

    // Otherwise, it's a function call
    auto& funcRegistry = core::FunctionRegistry::instance();

    if (!funcRegistry.hasFunction(name)) {
        throw std::runtime_error("Unknown function or constant: " + name);
    }

    // Evaluate all arguments
    std::vector<core::Value> args;
    for (const auto& argNode : argNodes) {
        args.push_back(evaluate(argNode.get()));
    }

    // Check arity (if not variadic)
    int expectedArity = funcRegistry.getArity(name);
    if (expectedArity >= 0 && static_cast<int>(args.size()) != expectedArity) {
        throw std::runtime_error("Function " + name +
                               " expects " + std::to_string(expectedArity) +
                               " arguments, got " + std::to_string(args.size()));
    }

    // Call the function
    auto func = funcRegistry.getFunction(name);
    return func(args);
}

} // namespace parser
} // namespace achronyme
