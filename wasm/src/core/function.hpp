#ifndef ACHRONYME_CORE_FUNCTION_HPP
#define ACHRONYME_CORE_FUNCTION_HPP

#include <string>
#include <vector>
#include <memory>
#include <functional>

namespace achronyme {

// Forward declarations
namespace parser {
    class ASTNode;
    class Environment;
}

namespace core {

// Forward declaration
class Value;

/**
 * Function type for lambda expressions and higher-order functions (Phase 4A)
 *
 * Supports two types of functions:
 * 1. Native functions: C++ functions (already handled by FunctionRegistry)
 * 2. Lambda functions: User-defined functions with closures
 *
 * Example lambdas:
 *   x => x * 2
 *   (x, y) => x + y
 *   x => x > 0       (predicate for filter)
 */
class Function {
public:
    /**
     * Create a lambda function
     *
     * @param params Parameter names (can be single or multiple)
     * @param body AST node representing the function body
     * @param closure Captured environment (for closures)
     */
    Function(std::vector<std::string> params,
             std::shared_ptr<parser::ASTNode> body,
             std::shared_ptr<parser::Environment> closure);

    // Getters
    const std::vector<std::string>& params() const { return params_; }
    size_t arity() const { return params_.size(); }
    const parser::ASTNode* body() const { return body_.get(); }
    const parser::Environment* closure() const { return closure_.get(); }

    // Check if valid
    bool isValid() const { return body_ != nullptr; }

    // String representation
    std::string toString() const;

private:
    std::vector<std::string> params_;              // Parameter names (e.g., ["x", "y"])
    std::shared_ptr<parser::ASTNode> body_;        // Function body AST
    std::shared_ptr<parser::Environment> closure_; // Captured environment
};

} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_FUNCTION_HPP
