#include <emscripten/bind.h>
#include <string>
#include "../parser/lexer.hpp"
#include "../parser/parser.hpp"
#include "../parser/evaluator.hpp"
#include "../core/handle_manager.hpp"
#include "fast_ops.hpp"

using namespace emscripten;
using namespace achronyme;

/**
 * SOLUCIÓN 1: Evaluador Global Persistente
 *
 * El evaluador vive durante toda la sesión, manteniendo
 * las variables entre llamadas.
 */

namespace achronyme {
namespace bindings {
    // Global evaluator instance - persists across calls
    // Note: Shared with fast_ops.cpp
    parser::Evaluator globalEvaluator;
} // namespace bindings
} // namespace achronyme

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
        // ✨ Saves AST so lambda bodies remain valid!
        auto result = achronyme::bindings::globalEvaluator.evaluateAndSave(std::move(ast));

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
    achronyme::bindings::globalEvaluator.environment().clear();
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
    // ========================================================================
    // Expression API (Original - Parsing-based)
    // ========================================================================
    function("eval", &eval);
    function("reset", &reset);
    function("listVariables", &listVariables);

    // ========================================================================
    // Fast API (NEW - Handle-based, Zero-parsing)
    // ========================================================================

    // Vector/Matrix Creation
    function("createVectorFromBuffer", &bindings::createVectorFromBuffer);
    function("createMatrixFromBuffer", &bindings::createMatrixFromBuffer);

    // Data Extraction
    function("getVectorData", &bindings::getVectorData, allow_raw_pointers());
    function("getVectorLength", &bindings::getVectorLength);
    function("getVectorDataPtr", &bindings::getVectorDataPtr);
    function("getMatrixData", &bindings::getMatrixData, allow_raw_pointers());
    function("copyVectorToBuffer", &bindings::copyVectorToBuffer);

    // DSP Operations
    function("fft_fast", &bindings::fft_fast);
    function("fft_mag_fast", &bindings::fft_mag_fast);
    function("fft_phase_fast", &bindings::fft_phase_fast);
    function("ifft_fast", &bindings::ifft_fast);
    function("conv_fast", &bindings::conv_fast);
    function("conv_fft_fast", &bindings::conv_fft_fast);

    // Vector Operations
    function("vadd_fast", &bindings::vadd_fast);
    function("vsub_fast", &bindings::vsub_fast);
    function("vmul_fast", &bindings::vmul_fast);
    function("vdiv_fast", &bindings::vdiv_fast);
    function("vscale_fast", &bindings::vscale_fast);
    function("dot_fast", &bindings::dot_fast);
    function("norm_fast", &bindings::norm_fast);

    // Mathematical Functions
    function("sin_fast", &bindings::sin_fast);
    function("cos_fast", &bindings::cos_fast);
    function("tan_fast", &bindings::tan_fast);
    function("exp_fast", &bindings::exp_fast);
    function("ln_fast", &bindings::ln_fast);
    function("abs_fast", &bindings::abs_fast);
    function("sqrt_fast", &bindings::sqrt_fast);

    // Optimization Functions
    function("linspace_fast", &bindings::linspace_fast);
    function("fftshift_fast", &bindings::fftshift_fast);
    function("ifftshift_fast", &bindings::ifftshift_fast);
    function("fft_spectrum_fast", &bindings::fft_spectrum_fast);

    // Handle Management
    function("releaseHandle", &bindings::releaseHandle);
    function("isValidHandle", &bindings::isValidHandle);
    function("getHandleType", &bindings::getHandleType);
    function("cloneHandle", &bindings::cloneHandle);

    // Integration with Evaluator
    function("bindVariableToHandle", &bindings::bindVariableToHandle);
    function("createHandleFromVariable", &bindings::createHandleFromVariable);
}
