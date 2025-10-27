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

    // Note: min/max are implemented below as unified functions that handle both
    // variadic scalars and single vector arguments

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
    // Native Vector Operations (Optimized for Performance)
    // ========================================================================

    registerFunction("vadd", [](const std::vector<Value>& args) {
        // vadd(v1, v2) → element-wise addition of vectors
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();

        if (v1.size() != v2.size()) {
            throw std::runtime_error("vadd() requires vectors of same size");
        }

        Vector result(v1.size());
        for (size_t i = 0; i < v1.size(); ++i) {
            result[i] = v1[i] + v2[i];
        }
        return Value(result);
    }, 2);

    registerFunction("vsub", [](const std::vector<Value>& args) {
        // vsub(v1, v2) → element-wise subtraction of vectors
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();

        if (v1.size() != v2.size()) {
            throw std::runtime_error("vsub() requires vectors of same size");
        }

        Vector result(v1.size());
        for (size_t i = 0; i < v1.size(); ++i) {
            result[i] = v1[i] - v2[i];
        }
        return Value(result);
    }, 2);

    registerFunction("vmul", [](const std::vector<Value>& args) {
        // vmul(v1, v2) → element-wise multiplication of vectors
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();

        if (v1.size() != v2.size()) {
            throw std::runtime_error("vmul() requires vectors of same size");
        }

        Vector result(v1.size());
        for (size_t i = 0; i < v1.size(); ++i) {
            result[i] = v1[i] * v2[i];
        }
        return Value(result);
    }, 2);

    registerFunction("vdiv", [](const std::vector<Value>& args) {
        // vdiv(v1, v2) → element-wise division of vectors
        Vector v1 = args[0].asVector();
        Vector v2 = args[1].asVector();

        if (v1.size() != v2.size()) {
            throw std::runtime_error("vdiv() requires vectors of same size");
        }

        Vector result(v1.size());
        for (size_t i = 0; i < v1.size(); ++i) {
            if (v2[i] == 0.0) {
                throw std::runtime_error("vdiv() division by zero");
            }
            result[i] = v1[i] / v2[i];
        }
        return Value(result);
    }, 2);

    registerFunction("vscale", [](const std::vector<Value>& args) {
        // vscale(v, scalar) → scale vector by scalar
        Vector v = args[0].asVector();
        double scalar = args[1].asNumber();

        Vector result(v.size());
        for (size_t i = 0; i < v.size(); ++i) {
            result[i] = v[i] * scalar;
        }
        return Value(result);
    }, 2);

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

    // ========================================================================
    // Phase 4A: Higher-Order Functions
    // ========================================================================

    registerFunction("map", mapFunction, -1);  // variadic: map(f, coll1, coll2, ...)
    registerFunction("filter", filterFunction, 2);  // filter(predicate, collection)
    registerFunction("reduce", reduceFunction, 3);  // reduce(f, init, collection)
    registerFunction("pipe", pipeFunction, -1);  // variadic: pipe(value, f1, f2, ...)
    // compose requires returning a function (deferred for now)
    // registerFunction("compose", composeFunction, -1);

    // ========================================================================
    // Native Statistical Functions (Optimized)
    // ========================================================================

    registerFunction("sum", [](const std::vector<Value>& args) {
        // sum(vector) → scalar - optimized native sum
        if (args[0].isVector()) {
            const Vector& v = args[0].asVector();
            double total = 0.0;
            for (size_t i = 0; i < v.size(); ++i) {
                total += v[i];
            }
            return Value(total);
        }
        throw std::runtime_error("sum() requires a vector argument");
    }, 1);

    registerFunction("mean", [](const std::vector<Value>& args) {
        // mean(vector) → scalar - optimized native mean
        if (args[0].isVector()) {
            const Vector& v = args[0].asVector();
            if (v.size() == 0) return Value(0.0);
            double total = 0.0;
            for (size_t i = 0; i < v.size(); ++i) {
                total += v[i];
            }
            return Value(total / v.size());
        }
        throw std::runtime_error("mean() requires a vector argument");
    }, 1);

    registerFunction("max", [](const std::vector<Value>& args) {
        // max can handle both: max(v1, v2, v3, ...) or max(vector)
        if (args.empty()) {
            throw std::runtime_error("max() requires at least one argument");
        }

        // If single argument and it's a vector, find max of vector elements
        if (args.size() == 1 && args[0].isVector()) {
            const Vector& v = args[0].asVector();
            if (v.size() == 0) throw std::runtime_error("max() requires non-empty vector");
            double maxVal = v[0];
            for (size_t i = 1; i < v.size(); ++i) {
                if (v[i] > maxVal) maxVal = v[i];
            }
            return Value(maxVal);
        }

        // Otherwise, treat as variadic scalars: max(a, b, c, ...)
        double maxVal = args[0].asNumber();
        for (size_t i = 1; i < args.size(); ++i) {
            maxVal = std::max(maxVal, args[i].asNumber());
        }
        return Value(maxVal);
    }, -1);  // variadic

    registerFunction("min", [](const std::vector<Value>& args) {
        // min can handle both: min(v1, v2, v3, ...) or min(vector)
        if (args.empty()) {
            throw std::runtime_error("min() requires at least one argument");
        }

        // If single argument and it's a vector, find min of vector elements
        if (args.size() == 1 && args[0].isVector()) {
            const Vector& v = args[0].asVector();
            if (v.size() == 0) throw std::runtime_error("min() requires non-empty vector");
            double minVal = v[0];
            for (size_t i = 1; i < v.size(); ++i) {
                if (v[i] < minVal) minVal = v[i];
            }
            return Value(minVal);
        }

        // Otherwise, treat as variadic scalars: min(a, b, c, ...)
        double minVal = args[0].asNumber();
        for (size_t i = 1; i < args.size(); ++i) {
            minVal = std::min(minVal, args[i].asNumber());
        }
        return Value(minVal);
    }, -1);  // variadic

    registerFunction("std", [](const std::vector<Value>& args) {
        // std(vector) → scalar - standard deviation
        if (args[0].isVector()) {
            const Vector& v = args[0].asVector();
            if (v.size() == 0) return Value(0.0);

            // Calculate mean
            double mean = 0.0;
            for (size_t i = 0; i < v.size(); ++i) {
                mean += v[i];
            }
            mean /= v.size();

            // Calculate variance
            double variance = 0.0;
            for (size_t i = 0; i < v.size(); ++i) {
                double diff = v[i] - mean;
                variance += diff * diff;
            }
            variance /= v.size();

            return Value(std::sqrt(variance));
        }
        throw std::runtime_error("std() requires a vector argument");
    }, 1);

    // ========================================================================
    // Phase 4B: DSP Functions
    // ========================================================================

    registerFunction("dft", dftFunction, 1);  // dft(signal) -> Matrix [N x 2]
    registerFunction("dft_mag", dftMagFunction, 1);  // dft_mag(signal) -> Vector
    registerFunction("dft_phase", dftPhaseFunction, 1);  // dft_phase(signal) -> Vector

    registerFunction("fft", fftFunction, 1);  // fft(signal) -> Matrix [N x 2]
    registerFunction("fft_mag", fftMagFunction, 1);  // fft_mag(signal) -> Vector
    registerFunction("fft_phase", fftPhaseFunction, 1);  // fft_phase(signal) -> Vector
    registerFunction("ifft", ifftFunction, 1);  // ifft(spectrum) -> Vector

    // Convolution
    registerFunction("conv", convFunction, 2);  // conv(sig1, sig2) -> Vector
    registerFunction("conv_fft", convFFTFunction, 2);  // conv_fft(sig1, sig2) -> Vector (faster)

    // Window functions
    registerFunction("hanning", hanningFunction, 1);  // hanning(N) -> Vector
    registerFunction("hamming", hammingFunction, 1);  // hamming(N) -> Vector
    registerFunction("blackman", blackmanFunction, 1);  // blackman(N) -> Vector

    // ========================================================================
    // Optimization Functions (Reduce JS-WASM overhead)
    // ========================================================================

    registerFunction("linspace", linspaceFunction, 3);  // linspace(start, end, N) -> Vector
    registerFunction("fftshift", fftshiftFunction, 1);  // fftshift(vector) -> Vector
    registerFunction("ifftshift", ifftshiftFunction, 1);  // ifftshift(vector) -> Vector
    registerFunction("fft_spectrum", fftSpectrumFunction, -1);  // fft_spectrum(signal, fs, shift?, angular?, omegaRange?) -> Matrix [N x 3]
}

} // namespace core
} // namespace achronyme
