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

        case ASTNodeType::COMPLEX_LITERAL:
            return evaluateComplexLiteral(static_cast<const ComplexLiteralNode*>(node));

        case ASTNodeType::VECTOR_LITERAL:
            return evaluateVectorLiteral(static_cast<const VectorLiteralNode*>(node));

        case ASTNodeType::MATRIX_LITERAL:
            return evaluateMatrixLiteral(static_cast<const MatrixLiteralNode*>(node));

        case ASTNodeType::VARIABLE_DECLARATION:
            return evaluateVariableDeclaration(static_cast<const VariableDeclarationNode*>(node));

        case ASTNodeType::VARIABLE_REFERENCE:
            return evaluateVariableReference(static_cast<const VariableReferenceNode*>(node));

        case ASTNodeType::LAMBDA:
            return evaluateLambda(static_cast<const LambdaNode*>(node));

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

        // Comparison operators (Phase 4A)
        // Returns 1.0 for true, 0.0 for false
        case BinaryOp::GT: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() > right.asNumber() ? 1.0 : 0.0);
        }

        case BinaryOp::LT: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() < right.asNumber() ? 1.0 : 0.0);
        }

        case BinaryOp::GTE: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() >= right.asNumber() ? 1.0 : 0.0);
        }

        case BinaryOp::LTE: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() <= right.asNumber() ? 1.0 : 0.0);
        }

        case BinaryOp::EQ: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() == right.asNumber() ? 1.0 : 0.0);
        }

        case BinaryOp::NEQ: {
            if (!left.isNumber() || !right.isNumber()) {
                throw std::runtime_error("Comparison operators currently only support numbers");
            }
            return core::Value(left.asNumber() != right.asNumber() ? 1.0 : 0.0);
        }

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

// Evaluate complex literal: 3i, (2+3)i
core::Value Evaluator::evaluateComplexLiteral(const ComplexLiteralNode* node) {
    return core::Value(core::Complex(node->real(), node->imag()));
}

// Evaluate vector literal: [1, 2, 3]
core::Value Evaluator::evaluateVectorLiteral(const VectorLiteralNode* node) {
    std::vector<double> elements;

    for (const auto& elementNode : node->elements()) {
        auto value = evaluate(elementNode.get());

        // For now, only support numbers in vectors
        if (!value.isNumber()) {
            throw std::runtime_error("Vector elements must be numbers");
        }

        elements.push_back(value.asNumber());
    }

    return core::Value(core::Vector(elements));
}

// Evaluate matrix literal: [[1, 2], [3, 4]]
core::Value Evaluator::evaluateMatrixLiteral(const MatrixLiteralNode* node) {
    const auto& rows = node->rows();

    if (rows.empty()) {
        throw std::runtime_error("Matrix cannot be empty");
    }

    size_t numRows = rows.size();
    size_t numCols = rows[0].size();

    // Flatten matrix data (row-major)
    std::vector<double> data;
    data.reserve(numRows * numCols);

    for (const auto& row : rows) {
        for (const auto& elementNode : row) {
            auto value = evaluate(elementNode.get());

            // For now, only support numbers in matrices
            if (!value.isNumber()) {
                throw std::runtime_error("Matrix elements must be numbers");
            }

            data.push_back(value.asNumber());
        }
    }

    return core::Value(core::Matrix(numRows, numCols, data));
}

// Evaluate variable declaration: let x = expr
core::Value Evaluator::evaluateVariableDeclaration(const VariableDeclarationNode* node) {
    const std::string& name = node->name();

    // Evaluate the initializer
    auto value = evaluate(node->initializer());

    // Define the variable in the environment
    env_.define(name, value);

    // Return the value (so "let x = 5" evaluates to 5)
    return value;
}

// Evaluate variable reference: x, PI, E, etc.
core::Value Evaluator::evaluateVariableReference(const VariableReferenceNode* node) {
    const std::string& name = node->name();

    // Check if it's a variable first
    if (env_.has(name)) {
        return env_.get(name);
    }

    // Otherwise, check if it's a constant
    auto& constRegistry = core::constants::ConstantsRegistry::instance();
    if (constRegistry.hasConstant(name)) {
        return core::Value(constRegistry.getConstant(name));
    }

    // Not found
    throw std::runtime_error("Undefined variable or constant: " + name);
}

// Evaluate lambda: x => expr or (x, y) => expr
core::Value Evaluator::evaluateLambda(const LambdaNode* node) {
    // Get parameter names
    std::vector<std::string> params = node->params();

    // Create shared pointer to the body (non-owning for now)
    // The body is owned by the LambdaNode which is part of the AST tree
    // We're assuming the AST lives as long as we need it
    std::shared_ptr<ASTNode> bodyPtr(const_cast<ASTNode*>(node->body()), [](ASTNode*){});

    // Capture current environment as closure
    auto closure = std::make_shared<Environment>(env_);

    // Create Function object
    core::Function func(params, bodyPtr, closure);

    // Return as Value
    return core::Value(func);
}

} // namespace parser
} // namespace achronyme
