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
    FUNCTION_CALL,  // Function call (sin(x), max(a,b,c), etc.)
    COMPLEX_LITERAL,// Complex number literal (3+4i)
    VECTOR_LITERAL, // Vector literal ([1, 2, 3])
    MATRIX_LITERAL  // Matrix literal ([[1, 2], [3, 4]])
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

/**
 * Complex literal node (Phase 3)
 *
 * Examples:
 *   3i
 *   3 + 4i (parsed as BinaryOp with ComplexLiteralNode)
 *   (2+3)i (parsed as ComplexLiteralNode with expression)
 */
class ComplexLiteralNode : public ASTNode {
public:
    // For pure imaginary: 3i
    explicit ComplexLiteralNode(double imag) : real_(0.0), imag_(imag) {}

    // For complex with both parts: used internally
    ComplexLiteralNode(double real, double imag) : real_(real), imag_(imag) {}

    ASTNodeType type() const override { return ASTNodeType::COMPLEX_LITERAL; }
    double real() const { return real_; }
    double imag() const { return imag_; }

private:
    double real_;
    double imag_;
};

/**
 * Vector literal node (Phase 3)
 *
 * Examples:
 *   [1, 2, 3]
 *   [sin(0), cos(0), tan(0)]
 */
class VectorLiteralNode : public ASTNode {
public:
    explicit VectorLiteralNode(std::vector<std::unique_ptr<ASTNode>> elements)
        : elements_(std::move(elements)) {}

    ASTNodeType type() const override { return ASTNodeType::VECTOR_LITERAL; }
    const std::vector<std::unique_ptr<ASTNode>>& elements() const { return elements_; }

private:
    std::vector<std::unique_ptr<ASTNode>> elements_;
};

/**
 * Matrix literal node (Phase 3)
 *
 * Examples:
 *   [[1, 2], [3, 4]]
 *   [[sin(0), cos(0)], [1, 2]]
 */
class MatrixLiteralNode : public ASTNode {
public:
    explicit MatrixLiteralNode(std::vector<std::vector<std::unique_ptr<ASTNode>>> rows)
        : rows_(std::move(rows)) {}

    ASTNodeType type() const override { return ASTNodeType::MATRIX_LITERAL; }
    const std::vector<std::vector<std::unique_ptr<ASTNode>>>& rows() const { return rows_; }

private:
    std::vector<std::vector<std::unique_ptr<ASTNode>>> rows_;
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_AST_HPP
