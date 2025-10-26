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
 * Usage from JS:
 *   const result = Module.eval("2 + 3 * 4");
 *   console.log(result); // 14
 */
double eval(const std::string& expression) {
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

        return result.asNumber();
    }
    catch (const std::exception& e) {
        // In Phase 2, we'll add proper error handling
        // For now, return NaN on error
        emscripten::val::global("console").call<void>("error",
            std::string("Evaluation error: ") + e.what());
        return std::numeric_limits<double>::quiet_NaN();
    }
}

// Emscripten bindings
EMSCRIPTEN_BINDINGS(achronyme_core) {
    function("eval", &eval);
}
