#ifndef ACHRONYME_PARSER_AST_HPP
#define ACHRONYME_PARSER_AST_HPP

#include <memory>
#include <vector>
#include "../core/value.hpp"

namespace achronyme {
namespace parser {

/**
 * AST Node types
 */
enum class ASTNodeType {
    NUMBER,         // Literal number
    BINARY_OP,      // Binary operation (+, -, *, /, ^)
    UNARY_OP,       // Unary operation (- for negation)
    FUNCTION_CALL   // Function call (sin(x), max(a,b,c), etc.)
};

/**
 * Binary operators
 */
enum class BinaryOp {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POWER
};

/**
 * Unary operators
 */
enum class UnaryOp {
    NEGATE
};

/**
 * Abstract base class for AST nodes
 */
class ASTNode {
public:
    virtual ~ASTNode() = default;
    virtual ASTNodeType type() const = 0;
};

/**
 * Number literal node
 */
class NumberNode : public ASTNode {
public:
    explicit NumberNode(double value) : value_(value) {}

    ASTNodeType type() const override { return ASTNodeType::NUMBER; }
    double value() const { return value_; }

private:
    double value_;
};

/**
 * Binary operation node
 */
class BinaryOpNode : public ASTNode {
public:
    BinaryOpNode(BinaryOp op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right)
        : op_(op), left_(std::move(left)), right_(std::move(right)) {}

    ASTNodeType type() const override { return ASTNodeType::BINARY_OP; }
    BinaryOp op() const { return op_; }
    const ASTNode* left() const { return left_.get(); }
    const ASTNode* right() const { return right_.get(); }

private:
    BinaryOp op_;
    std::unique_ptr<ASTNode> left_;
    std::unique_ptr<ASTNode> right_;
};

/**
 * Unary operation node
 */
class UnaryOpNode : public ASTNode {
public:
    UnaryOpNode(UnaryOp op, std::unique_ptr<ASTNode> operand)
        : op_(op), operand_(std::move(operand)) {}

    ASTNodeType type() const override { return ASTNodeType::UNARY_OP; }
    UnaryOp op() const { return op_; }
    const ASTNode* operand() const { return operand_.get(); }

private:
    UnaryOp op_;
    std::unique_ptr<ASTNode> operand_;
};

/**
 * Function call node
 *
 * Examples:
 *   sin(PI/2)
 *   max(1, 2, 3, 4)
 *   sqrt(16)
 */
class FunctionCallNode : public ASTNode {
public:
    FunctionCallNode(std::string name, std::vector<std::unique_ptr<ASTNode>> args)
        : name_(std::move(name)), args_(std::move(args)) {}

    ASTNodeType type() const override { return ASTNodeType::FUNCTION_CALL; }
    const std::string& name() const { return name_; }
    const std::vector<std::unique_ptr<ASTNode>>& args() const { return args_; }

private:
    std::string name_;
    std::vector<std::unique_ptr<ASTNode>> args_;
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_AST_HPP
