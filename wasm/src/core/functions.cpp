#include "functions.hpp"
#include <cmath>
#include <algorithm>
#include <stdexcept>
#include <limits>
#include <cctype>

namespace achronyme {
namespace core {

FunctionRegistry::FunctionRegistry() {
    registerBuiltInFunctions();
}

FunctionRegistry& FunctionRegistry::instance() {
    static FunctionRegistry registry;
    return registry;
}

void FunctionRegistry::registerFunction(const std::string& name, MathFunction func, int arity) {
    std::string lowerName = toLower(name);
    functions_[lowerName] = {func, arity};
}

bool FunctionRegistry::hasFunction(const std::string& name) const {
    std::string lowerName = toLower(name);
    return functions_.find(lowerName) != functions_.end();
}

MathFunction FunctionRegistry::getFunction(const std::string& name) const {
    std::string lowerName = toLower(name);
    auto it = functions_.find(lowerName);

    if (it == functions_.end()) {
        throw std::runtime_error("Unknown function: " + name);
    }

    return it->second.func;
}

int FunctionRegistry::getArity(const std::string& name) const {
    std::string lowerName = toLower(name);
    auto it = functions_.find(lowerName);

    if (it == functions_.end()) {
        throw std::runtime_error("Unknown function: " + name);
    }

    return it->second.arity;
}

std::string FunctionRegistry::toLower(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(),
                   [](unsigned char c) { return std::tolower(c); });
    return result;
}

void FunctionRegistry::registerBuiltInFunctions() {
    // ========================================================================
    // Trigonometric Functions (radians)
    // ========================================================================

    registerFunction("sin", [](const std::vector<Value>& args) {
        return Value(std::sin(args[0].asNumber()));
    }, 1);

    registerFunction("cos", [](const std::vector<Value>& args) {
        return Value(std::cos(args[0].asNumber()));
    }, 1);

    registerFunction("tan", [](const std::vector<Value>& args) {
        return Value(std::tan(args[0].asNumber()));
    }, 1);

    registerFunction("asin", [](const std::vector<Value>& args) {
        return Value(std::asin(args[0].asNumber()));
    }, 1);

    registerFunction("acos", [](const std::vector<Value>& args) {
        return Value(std::acos(args[0].asNumber()));
    }, 1);

    registerFunction("atan", [](const std::vector<Value>& args) {
        return Value(std::atan(args[0].asNumber()));
    }, 1);

    registerFunction("atan2", [](const std::vector<Value>& args) {
        return Value(std::atan2(args[0].asNumber(), args[1].asNumber()));
    }, 2);

    // Hyperbolic functions
    registerFunction("sinh", [](const std::vector<Value>& args) {
        return Value(std::sinh(args[0].asNumber()));
    }, 1);

    registerFunction("cosh", [](const std::vector<Value>& args) {
        return Value(std::cosh(args[0].asNumber()));
    }, 1);

    registerFunction("tanh", [](const std::vector<Value>& args) {
        return Value(std::tanh(args[0].asNumber()));
    }, 1);

    // ========================================================================
    // Exponential and Logarithmic Functions
    // ========================================================================

    registerFunction("exp", [](const std::vector<Value>& args) {
        return Value(std::exp(args[0].asNumber()));
    }, 1);

    registerFunction("log", [](const std::vector<Value>& args) {
        return Value(std::log(args[0].asNumber()));
    }, 1);

    // Aliases for natural log
    registerFunction("ln", [](const std::vector<Value>& args) {
        return Value(std::log(args[0].asNumber()));
    }, 1);

    registerFunction("log10", [](const std::vector<Value>& args) {
        return Value(std::log10(args[0].asNumber()));
    }, 1);

    registerFunction("log2", [](const std::vector<Value>& args) {
        return Value(std::log2(args[0].asNumber()));
    }, 1);

    // ========================================================================
    // Power and Root Functions
    // ========================================================================

    registerFunction("sqrt", [](const std::vector<Value>& args) {
        return Value(std::sqrt(args[0].asNumber()));
    }, 1);

    registerFunction("cbrt", [](const std::vector<Value>& args) {
        return Value(std::cbrt(args[0].asNumber()));
    }, 1);

    registerFunction("pow", [](const std::vector<Value>& args) {
        return Value(std::pow(args[0].asNumber(), args[1].asNumber()));
    }, 2);

    // ========================================================================
    // Rounding Functions
    // ========================================================================

    registerFunction("floor", [](const std::vector<Value>& args) {
        return Value(std::floor(args[0].asNumber()));
    }, 1);

    registerFunction("ceil", [](const std::vector<Value>& args) {
        return Value(std::ceil(args[0].asNumber()));
    }, 1);

    registerFunction("round", [](const std::vector<Value>& args) {
        return Value(std::round(args[0].asNumber()));
    }, 1);

    registerFunction("trunc", [](const std::vector<Value>& args) {
        return Value(std::trunc(args[0].asNumber()));
    }, 1);

    // ========================================================================
    // Other Mathematical Functions
    // ========================================================================

    registerFunction("abs", [](const std::vector<Value>& args) {
        // abs() works for both numbers and complex numbers
        if (args[0].isComplex()) {
            Complex z = args[0].asComplex();
            return Value(z.magnitude());
        } else {
            return Value(std::abs(args[0].asNumber()));
        }
    }, 1);

    registerFunction("sign", [](const std::vector<Value>& args) {
        double x = args[0].asNumber();
        if (x > 0) return Value(1.0);
        if (x < 0) return Value(-1.0);
        return Value(0.0);
    }, 1);

    // ========================================================================
    // Variadic Functions (min, max)
    // ========================================================================

    registerFunction("min", [](const std::vector<Value>& args) {
        if (args.empty()) {
            throw std::runtime_error("min() requires at least one argument");
        }
        double minVal = args[0].asNumber();
        for (size_t i = 1; i < args.size(); ++i) {
            minVal = std::min(minVal, args[i].asNumber());
        }
        return Value(minVal);
    }, -1);  // -1 = variadic

    registerFunction("max", [](const std::vector<Value>& args) {
        if (args.empty()) {
            throw std::runtime_error("max() requires at least one argument");
        }
        double maxVal = args[0].asNumber();
        for (size_t i = 1; i < args.size(); ++i) {
            maxVal = std::max(maxVal, args[i].asNumber());
        }
        return Value(maxVal);
    }, -1);  // -1 = variadic

    // ========================================================================
    // Additional Useful Functions
    // ========================================================================

    registerFunction("deg", [](const std::vector<Value>& args) {
        // Convert radians to degrees
        return Value(args[0].asNumber() * 180.0 / 3.141592653589793);
    }, 1);

    registerFunction("rad", [](const std::vector<Value>& args) {
        // Convert degrees to radians
        return Value(args[0].asNumber() * 3.141592653589793 / 180.0);
    }, 1);

    // ========================================================================
    // Phase 3: Complex Number Functions
    // ========================================================================

    registerFunction("complex", [](const std::vector<Value>& args) {
        // complex(real, imag) → Complex number
        double real = args[0].asNumber();
        double imag = args[1].asNumber();
        return Value(Complex(real, imag));
    }, 2);

    registerFunction("real", [](const std::vector<Value>& args) {
        // real(z) → real part of complex number
        Complex z = args[0].asComplex();
        return Value(z.real());
    }, 1);

    registerFunction("imag", [](const std::vector<Value>& args) {
        // imag(z) → imaginary part of complex number
        Complex z = args[0].asComplex();
        return Value(z.imag());
    }, 1);

    registerFunction("conj", [](const std::vector<Value>& args) {
        // conj(z) → complex conjugate
        Complex z = args[0].asComplex();
        return Value(z.conjugate());
    }, 1);

    registerFunction("arg", [](const std::vector<Value>& args) {
        // arg(z) → argument/phase of complex number
        Complex z = args[0].asComplex();
        return Value(z.argument());
    }, 1);

    // ========================================================================
    // Phase 3: Vector Functions
    // ========================================================================

    registerFunction("dot", [](const std::vector<Value>& args) {
        // dot(v1, v2) → dot product of two vectors
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();
        return Value(v1.dot(v2));
    }, 2);

    registerFunction("cross", [](const std::vector<Value>& args) {
        // cross(v1, v2) → cross product (3D only)
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();
        return Value(v1.cross(v2));
    }, 2);

    registerFunction("norm", [](const std::vector<Value>& args) {
        // norm(v) → magnitude of vector
        Vector v = args[0].asVector();
        return Value(v.norm());
    }, 1);

    registerFunction("normalize", [](const std::vector<Value>& args) {
        // normalize(v) → unit vector
        Vector v = args[0].asVector();
        return Value(v.normalize());
    }, 1);

    // ========================================================================
    // Phase 3: Matrix Functions
    // ========================================================================

    registerFunction("transpose", [](const std::vector<Value>& args) {
        // transpose(M) → matrix transpose
        Matrix M = args[0].asMatrix();
        return Value(M.transpose());
    }, 1);

    registerFunction("det", [](const std::vector<Value>& args) {
        // det(M) → determinant of matrix
        Matrix M = args[0].asMatrix();
        return Value(M.determinant());
    }, 1);

    registerFunction("inverse", [](const std::vector<Value>& args) {
        // inverse(M) → matrix inverse
        Matrix M = args[0].asMatrix();
        return Value(M.inverse());
    }, 1);

    registerFunction("trace", [](const std::vector<Value>& args) {
        // trace(M) → sum of diagonal elements
        Matrix M = args[0].asMatrix();
        return Value(M.trace());
    }, 1);
}

} // namespace core
} // namespace achronyme
