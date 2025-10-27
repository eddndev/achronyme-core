#include <emscripten/bind.h>
#include <string>
#include "../parser/lexer.hpp"
#include "../parser/parser.hpp"
#include "../parser/evaluator.hpp"

using namespace emscripten;
using namespace achronyme;

/**
 * SOLUCIÓN 1: Evaluador Global Persistente
 *
 * El evaluador vive durante toda la sesión, manteniendo
 * las variables entre llamadas.
 */

// Global evaluator instance - persists across calls
static parser::Evaluator globalEvaluator;

/**
 * Main evaluation function with persistent state
 *
 * Variables defined with 'let' persist across calls:
 *   Module.eval("let x = 5");  // → "5"
 *   Module.eval("x + 10");     // → "15" ✓
 */
std::string eval(const std::string& expression) {
    try {
        // 1. Lexer: string → tokens
        parser::Lexer lexer(expression);
        auto tokens = lexer.tokenize();

        // 2. Parser: tokens → AST
        parser::Parser parser(tokens);
        auto ast = parser.parse();

        // 3. Evaluator: AST → result
        // ✨ Uses GLOBAL evaluator - variables persist!
        auto result = globalEvaluator.evaluate(ast.get());

        // Return string representation
        return result.toString();
    }
    catch (const std::exception& e) {
        // Return error message
        return std::string("Error: ") + e.what();
    }
}

/**
 * Reset the global evaluator state
 *
 * Clears all defined variables. Useful for:
 * - Starting a new session
 * - Running isolated tests
 * - Recovering from errors
 */
std::string reset() {
    globalEvaluator.environment().clear();
    return "Environment cleared";
}

/**
 * Get list of defined variables (for debugging)
 *
 * Returns a JSON-like string with all variables
 */
std::string listVariables() {
    // TODO: Implement if needed
    return "Variables listing not implemented yet";
}

// Emscripten bindings
EMSCRIPTEN_BINDINGS(achronyme_core) {
    function("eval", &eval);
    function("reset", &reset);
    function("listVariables", &listVariables);
}
