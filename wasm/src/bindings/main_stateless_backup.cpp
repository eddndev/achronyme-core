#include <emscripten/bind.h>
#include <string>
#include "../parser/lexer.hpp"
#include "../parser/parser.hpp"
#include "../parser/evaluator.hpp"

using namespace emscripten;
using namespace achronyme;

/**
 * Main evaluation function exposed to JavaScript
 *
 * Phase 1-2: Returns double
 * Phase 3: Returns string representation (for Complex, Vector, Matrix support)
 *
 * Usage from JS:
 *   const result = Module.eval("2 + 3 * 4");
 *   console.log(result); // "14" or "3 + 4i" or "[1, 2, 3]" or "[[1, 2], [3, 4]]"
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
        parser::Evaluator evaluator;
        auto result = evaluator.evaluate(ast.get());

        // Return string representation
        return result.toString();
    }
    catch (const std::exception& e) {
        // Return error message
        return std::string("Error: ") + e.what();
    }
}

// Emscripten bindings
EMSCRIPTEN_BINDINGS(achronyme_core) {
    function("eval", &eval);
}
