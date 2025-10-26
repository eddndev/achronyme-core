#ifndef ACHRONYME_CORE_FUNCTIONS_HPP
#define ACHRONYME_CORE_FUNCTIONS_HPP

#include <functional>
#include <vector>
#include <string>
#include <unordered_map>
#include "value.hpp"

namespace achronyme {
namespace core {

/**
 * Mathematical function type
 *
 * Takes a vector of Values and returns a Value.
 * Variable arity supported (-1 for variadic functions).
 */
using MathFunction = std::function<Value(const std::vector<Value>&)>;

/**
 * Function Registry
 *
 * Singleton registry for mathematical functions.
 * Supports case-insensitive lookup (sin = SIN = Sin).
 */
class FunctionRegistry {
public:
    static FunctionRegistry& instance();

    // Register a function with a specific arity
    // arity = -1 for variadic functions (min, max, etc.)
    void registerFunction(const std::string& name, MathFunction func, int arity);

    // Check if a function exists
    bool hasFunction(const std::string& name) const;

    // Get function
    MathFunction getFunction(const std::string& name) const;

    // Get expected arity (-1 for variadic)
    int getArity(const std::string& name) const;

private:
    FunctionRegistry();

    struct FunctionInfo {
        MathFunction func;
        int arity;
    };

    std::unordered_map<std::string, FunctionInfo> functions_;

    // Helper to convert to lowercase for case-insensitive lookup
    static std::string toLower(const std::string& str);

    // Register all built-in functions
    void registerBuiltInFunctions();
};

} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_FUNCTIONS_HPP
