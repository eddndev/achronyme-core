#ifndef ACHRONYME_PARSER_ENVIRONMENT_HPP
#define ACHRONYME_PARSER_ENVIRONMENT_HPP

#include <string>
#include <unordered_map>
#include <stdexcept>
#include "../core/value.hpp"

namespace achronyme {
namespace parser {

/**
 * Environment for variable storage (Phase 4A)
 *
 * Stores variable bindings for let declarations.
 * Supports nested scopes for lambda expressions (Phase 4A+).
 *
 * Example:
 *   Environment env;
 *   env.define("x", Value(5.0));
 *   Value val = env.get("x");  // 5.0
 */
class Environment {
public:
    Environment() = default;

    // Copy constructor for closures
    Environment(const Environment& other) = default;
    Environment& operator=(const Environment& other) = default;

    /**
     * Define a new variable in this environment
     *
     * @param name Variable name
     * @param value Initial value
     * @throws std::runtime_error if variable already exists
     */
    void define(const std::string& name, const core::Value& value) {
        if (variables_.count(name)) {
            throw std::runtime_error("Variable '" + name + "' already declared");
        }
        variables_[name] = value;
    }

    /**
     * Get a variable value
     *
     * @param name Variable name
     * @return Variable value
     * @throws std::runtime_error if variable not found
     */
    core::Value get(const std::string& name) const {
        auto it = variables_.find(name);
        if (it == variables_.end()) {
            throw std::runtime_error("Undefined variable '" + name + "'");
        }
        return it->second;
    }

    /**
     * Check if a variable is defined
     *
     * @param name Variable name
     * @return true if variable exists
     */
    bool has(const std::string& name) const {
        return variables_.count(name) > 0;
    }

    /**
     * Update an existing variable
     *
     * @param name Variable name
     * @param value New value
     * @throws std::runtime_error if variable not found
     */
    void set(const std::string& name, const core::Value& value) {
        auto it = variables_.find(name);
        if (it == variables_.end()) {
            throw std::runtime_error("Cannot assign to undefined variable '" + name + "'");
        }
        it->second = value;
    }

    /**
     * Clear all variables
     */
    void clear() {
        variables_.clear();
    }

private:
    std::unordered_map<std::string, core::Value> variables_;
};

} // namespace parser
} // namespace achronyme

#endif // ACHRONYME_PARSER_ENVIRONMENT_HPP
