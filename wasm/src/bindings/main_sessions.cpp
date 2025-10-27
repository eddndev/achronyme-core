#include <emscripten/bind.h>
#include <string>
#include <memory>
#include <unordered_map>
#include "../parser/lexer.hpp"
#include "../parser/parser.hpp"
#include "../parser/evaluator.hpp"

using namespace emscripten;
using namespace achronyme;

/**
 * SOLUCIÓN 2: Sistema de Sesiones
 *
 * Permite múltiples evaluadores independientes.
 * Útil para:
 * - Tests paralelos
 * - Múltiples notebooks/worksheets
 * - Aislamiento de contextos
 */

// Session storage: sessionId → Evaluator
static std::unordered_map<std::string, std::shared_ptr<parser::Evaluator>> sessions;
static std::string currentSessionId = "default";

/**
 * Create a new session
 *
 * @param sessionId Unique identifier for the session
 * @return Success message
 */
std::string createSession(const std::string& sessionId) {
    if (sessions.count(sessionId)) {
        return "Error: Session '" + sessionId + "' already exists";
    }

    sessions[sessionId] = std::make_shared<parser::Evaluator>();
    return "Session '" + sessionId + "' created";
}

/**
 * Switch to a different session
 *
 * @param sessionId Session to activate
 * @return Success message
 */
std::string useSession(const std::string& sessionId) {
    if (!sessions.count(sessionId)) {
        // Auto-create if doesn't exist
        sessions[sessionId] = std::make_shared<parser::Evaluator>();
    }

    currentSessionId = sessionId;
    return "Switched to session '" + sessionId + "'";
}

/**
 * Delete a session and free its memory
 *
 * @param sessionId Session to delete
 * @return Success message
 */
std::string deleteSession(const std::string& sessionId) {
    if (!sessions.count(sessionId)) {
        return "Error: Session '" + sessionId + "' not found";
    }

    sessions.erase(sessionId);

    if (currentSessionId == sessionId) {
        currentSessionId = "default";
        if (!sessions.count("default")) {
            sessions["default"] = std::make_shared<parser::Evaluator>();
        }
    }

    return "Session '" + sessionId + "' deleted";
}

/**
 * Evaluate expression in current session
 */
std::string eval(const std::string& expression) {
    try {
        // Ensure current session exists
        if (!sessions.count(currentSessionId)) {
            sessions[currentSessionId] = std::make_shared<parser::Evaluator>();
        }

        // Get current session's evaluator
        auto& evaluator = *sessions[currentSessionId];

        // 1. Lexer: string → tokens
        parser::Lexer lexer(expression);
        auto tokens = lexer.tokenize();

        // 2. Parser: tokens → AST
        parser::Parser parser(tokens);
        auto ast = parser.parse();

        // 3. Evaluator: AST → result
        auto result = evaluator.evaluate(ast.get());

        return result.toString();
    }
    catch (const std::exception& e) {
        return std::string("Error: ") + e.what();
    }
}

// Emscripten bindings
EMSCRIPTEN_BINDINGS(achronyme_core) {
    function("eval", &eval);
    function("createSession", &createSession);
    function("useSession", &useSession);
    function("deleteSession", &deleteSession);
}
