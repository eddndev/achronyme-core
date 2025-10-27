#include "functions.hpp"
#include "value.hpp"
#include "../parser/evaluator.hpp"
#include <algorithm>
#include <stdexcept>

namespace achronyme {
namespace core {

/**
 * map(f, coll1, coll2, ...) - Apply function to elements
 *
 * Multi-collection support:
 *   map(f, [1,2,3]) → applies f(x) to each element
 *   map(f, [1,2], [3,4]) → applies f(x,y) to pairs
 *
 * Truncates to shortest collection.
 */
Value mapFunction(const std::vector<Value>& args) {
    if (args.size() < 2) {
        throw std::runtime_error("map requires at least 2 arguments: function and collection(s)");
    }

    // First argument must be a function
    if (!args[0].isFunction()) {
        throw std::runtime_error("First argument to map must be a function");
    }

    const Function& func = args[0].asFunction();

    // All other arguments must be vectors
    std::vector<Vector> collections;
    size_t minLength = SIZE_MAX;

    for (size_t i = 1; i < args.size(); ++i) {
        if (!args[i].isVector()) {
            throw std::runtime_error("map arguments must be vectors");
        }
        Vector vec = args[i].asVector();
        collections.push_back(vec);
        minLength = std::min(minLength, vec.size());
    }

    // Check arity matches number of collections
    if (func.arity() != collections.size()) {
        throw std::runtime_error("Function arity (" + std::to_string(func.arity()) +
                               ") must match number of collections (" + std::to_string(collections.size()) + ")");
    }

    // Get current evaluator
    auto* evaluator = parser::Evaluator::getCurrentEvaluator();
    if (!evaluator) {
        throw std::runtime_error("No evaluator available for map");
    }

    // Apply function to each element
    std::vector<double> results;
    for (size_t i = 0; i < minLength; ++i) {
        // Gather arguments for this iteration
        std::vector<Value> funcArgs;
        for (const auto& coll : collections) {
            funcArgs.push_back(Value(coll[i]));
        }

        // Apply function
        Value result = evaluator->applyFunction(func, funcArgs);

        // Result must be a number (for now)
        if (!result.isNumber()) {
            throw std::runtime_error("map function must return numbers");
        }

        results.push_back(result.asNumber());
    }

    return Value(Vector(results));
}

/**
 * filter(predicate, collection) - Filter elements
 *
 * Returns elements where predicate returns true (non-zero).
 */
Value filterFunction(const std::vector<Value>& args) {
    if (args.size() != 2) {
        throw std::runtime_error("filter requires 2 arguments: predicate and collection");
    }

    // First argument must be a function
    if (!args[0].isFunction()) {
        throw std::runtime_error("First argument to filter must be a function");
    }

    const Function& predicate = args[0].asFunction();

    // Second argument must be a vector
    if (!args[1].isVector()) {
        throw std::runtime_error("Second argument to filter must be a vector");
    }

    Vector collection = args[1].asVector();

    // Predicate must be unary
    if (predicate.arity() != 1) {
        throw std::runtime_error("filter predicate must take exactly 1 argument");
    }

    // Get current evaluator
    auto* evaluator = parser::Evaluator::getCurrentEvaluator();
    if (!evaluator) {
        throw std::runtime_error("No evaluator available for filter");
    }

    // Filter elements
    std::vector<double> results;
    for (size_t i = 0; i < collection.size(); ++i) {
        Value elem(collection[i]);

        // Apply predicate
        Value result = evaluator->applyFunction(predicate, {elem});

        // Check result (non-zero = true)
        if (!result.isNumber()) {
            throw std::runtime_error("filter predicate must return a number");
        }

        if (result.asNumber() != 0.0) {
            results.push_back(collection[i]);
        }
    }

    return Value(Vector(results));
}

/**
 * reduce(f, init, collection) - Reduce collection to single value
 *
 * Applies f(accumulator, element) repeatedly.
 */
Value reduceFunction(const std::vector<Value>& args) {
    if (args.size() != 3) {
        throw std::runtime_error("reduce requires 3 arguments: function, initial value, and collection");
    }

    // First argument must be a function
    if (!args[0].isFunction()) {
        throw std::runtime_error("First argument to reduce must be a function");
    }

    const Function& func = args[0].asFunction();

    // Second argument is initial value (must be number for now)
    if (!args[1].isNumber()) {
        throw std::runtime_error("reduce initial value must be a number");
    }

    double accumulator = args[1].asNumber();

    // Third argument must be a vector
    if (!args[2].isVector()) {
        throw std::runtime_error("Third argument to reduce must be a vector");
    }

    Vector collection = args[2].asVector();

    // Function must be binary
    if (func.arity() != 2) {
        throw std::runtime_error("reduce function must take exactly 2 arguments");
    }

    // Get current evaluator
    auto* evaluator = parser::Evaluator::getCurrentEvaluator();
    if (!evaluator) {
        throw std::runtime_error("No evaluator available for reduce");
    }

    // Reduce elements
    for (size_t i = 0; i < collection.size(); ++i) {
        Value accVal(accumulator);
        Value elemVal(collection[i]);

        // Apply function
        Value result = evaluator->applyFunction(func, {accVal, elemVal});

        // Result must be number
        if (!result.isNumber()) {
            throw std::runtime_error("reduce function must return a number");
        }

        accumulator = result.asNumber();
    }

    return Value(accumulator);
}

/**
 * compose(f, g, ...) - Function composition
 *
 * Returns a new function that applies functions right-to-left:
 * compose(f, g, h)(x) = f(g(h(x)))
 *
 * All arguments must be unary functions.
 */
Value composeFunction(const std::vector<Value>& args) {
    if (args.size() < 2) {
        throw std::runtime_error("compose requires at least 2 functions");
    }

    // Verify all arguments are unary functions
    for (size_t i = 0; i < args.size(); ++i) {
        if (!args[i].isFunction()) {
            throw std::runtime_error("All arguments to compose must be functions");
        }

        const Function& func = args[i].asFunction();
        if (func.arity() != 1) {
            throw std::runtime_error("compose only supports unary functions");
        }
    }

    // Get current evaluator
    auto* evaluator = parser::Evaluator::getCurrentEvaluator();
    if (!evaluator) {
        throw std::runtime_error("No evaluator available for compose");
    }

    // Create a lambda that captures the functions and applies them right-to-left
    // We'll create a lambda AST that represents: x => f(g(h(x)))
    // For simplicity, we'll create a composed function object that stores the functions

    // Store the functions in a vector (we'll apply them in reverse order)
    std::vector<Function> functions;
    for (const auto& arg : args) {
        functions.push_back(arg.asFunction());
    }

    // Create a lambda that applies all functions in sequence
    // The lambda will be: x => (apply functions right-to-left)
    // We need to create this dynamically...

    // For now, let's throw an error indicating this needs the parser
    throw std::runtime_error("compose is not yet fully implemented - requires dynamic lambda creation");
}

/**
 * pipe(value, f1, f2, ...) - Apply functions left-to-right
 *
 * pipe(x, f, g, h) = h(g(f(x)))
 *
 * First argument is the initial value, rest are unary functions.
 */
Value pipeFunction(const std::vector<Value>& args) {
    if (args.size() < 2) {
        throw std::runtime_error("pipe requires at least 2 arguments: value and function(s)");
    }

    // First argument is the initial value
    Value result = args[0];

    // Get current evaluator
    auto* evaluator = parser::Evaluator::getCurrentEvaluator();
    if (!evaluator) {
        throw std::runtime_error("No evaluator available for pipe");
    }

    // Apply each function left-to-right
    for (size_t i = 1; i < args.size(); ++i) {
        if (!args[i].isFunction()) {
            throw std::runtime_error("pipe arguments after the first must be functions");
        }

        const Function& func = args[i].asFunction();

        // Check arity
        if (func.arity() != 1) {
            throw std::runtime_error("pipe only supports unary functions");
        }

        // Apply function to current result
        result = evaluator->applyFunction(func, {result});
    }

    return result;
}

} // namespace core
} // namespace achronyme
