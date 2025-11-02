#include "fast_ops.hpp"
#include "../parser/evaluator.hpp"
#include <cmath>
#include <algorithm>

namespace achronyme {
namespace bindings {

using namespace core;

// Referencia al evaluador global (declarado en main.cpp)
extern parser::Evaluator globalEvaluator;

// ============================================================================
// Vector Creation
// ============================================================================

Handle createVectorFromBuffer(uintptr_t dataPtr, size_t length) {
    double* data = reinterpret_cast<double*>(dataPtr);
    std::vector<double> vec(data, data + length);

    Value value{Vector(vec)};
    return globalHandleManager.create(std::move(value));
}

Handle createMatrixFromBuffer(uintptr_t dataPtr, size_t rows, size_t cols) {
    double* data = reinterpret_cast<double*>(dataPtr);
    std::vector<double> matData(data, data + (rows * cols));

    Value value(Matrix(rows, cols, matData));
    return globalHandleManager.create(std::move(value));
}

// ============================================================================
// Data Extraction
// ============================================================================

uintptr_t getVectorData(Handle handle, size_t* outLength) {
    const Value& value = globalHandleManager.get(handle);

    if (!value.isVector()) {
        throw std::runtime_error("Handle does not contain a vector");
    }

    const Vector& vec = value.asVector();
    if (outLength != nullptr) {
        *outLength = vec.size();
    }

    // Retornar puntero a los datos internos
    // NOTA: Este puntero es válido mientras el handle exista
    return reinterpret_cast<uintptr_t>(vec.elements().data());
}

uintptr_t getMatrixData(Handle handle, size_t* outRows, size_t* outCols) {
    const Value& value = globalHandleManager.get(handle);

    if (!value.isMatrix()) {
        throw std::runtime_error("Handle does not contain a matrix");
    }

    const Matrix& mat = value.asMatrix();
    if (outRows != nullptr) {
        *outRows = mat.rows();
    }
    if (outCols != nullptr) {
        *outCols = mat.cols();
    }

    return reinterpret_cast<uintptr_t>(mat.data().data());
}

size_t copyVectorToBuffer(Handle handle, uintptr_t destPtr, size_t maxLength) {
    const Value& value = globalHandleManager.get(handle);

    if (!value.isVector()) {
        throw std::runtime_error("Handle does not contain a vector");
    }

    const Vector& vec = value.asVector();
    size_t copyLength = std::min(vec.size(), maxLength);

    double* dest = reinterpret_cast<double*>(destPtr);
    const double* src = vec.elements().data();
    std::copy(src, src + copyLength, dest);

    return copyLength;
}

size_t getVectorLength(Handle handle) {
    const Value& value = globalHandleManager.get(handle);

    if (!value.isVector()) {
        throw std::runtime_error("Handle does not contain a vector");
    }

    return value.asVector().size();
}

uintptr_t getVectorDataPtr(Handle handle) {
    const Value& value = globalHandleManager.get(handle);

    if (!value.isVector()) {
        throw std::runtime_error("Handle does not contain a vector");
    }

    const Vector& vec = value.asVector();
    return reinterpret_cast<uintptr_t>(vec.elements().data());
}

// ============================================================================
// DSP Operations (Fast Path)
// ============================================================================

Handle fft_fast(Handle inputHandle) {
    const Value& input = globalHandleManager.get(inputHandle);

    if (!input.isVector()) {
        throw std::runtime_error("fft_fast requires a vector input");
    }

    // Llamar a la función FFT existente
    std::vector<Value> args = { input };
    Value result = FunctionRegistry::instance().getFunction("fft")(args);

    return globalHandleManager.create(std::move(result));
}

Handle fft_mag_fast(Handle inputHandle) {
    const Value& input = globalHandleManager.get(inputHandle);

    if (!input.isVector()) {
        throw std::runtime_error("fft_mag_fast requires a vector input");
    }

    std::vector<Value> args = { input };
    Value result = FunctionRegistry::instance().getFunction("fft_mag")(args);

    return globalHandleManager.create(std::move(result));
}

Handle fft_phase_fast(Handle inputHandle) {
    const Value& input = globalHandleManager.get(inputHandle);

    if (!input.isVector()) {
        throw std::runtime_error("fft_phase_fast requires a vector input");
    }

    std::vector<Value> args = { input };
    Value result = FunctionRegistry::instance().getFunction("fft_phase")(args);

    return globalHandleManager.create(std::move(result));
}

Handle ifft_fast(Handle inputHandle) {
    const Value& input = globalHandleManager.get(inputHandle);

    std::vector<Value> args = { input };
    Value result = FunctionRegistry::instance().getFunction("ifft")(args);

    return globalHandleManager.create(std::move(result));
}

Handle conv_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    std::vector<Value> args = { v1, v2 };
    Value result = FunctionRegistry::instance().getFunction("conv")(args);

    return globalHandleManager.create(std::move(result));
}

Handle conv_fft_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    std::vector<Value> args = { v1, v2 };
    Value result = FunctionRegistry::instance().getFunction("conv_fft")(args);

    return globalHandleManager.create(std::move(result));
}

// ============================================================================
// Vector Operations (Fast Path)
// ============================================================================

Handle vadd_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    Value result = v1 + v2;
    return globalHandleManager.create(std::move(result));
}

Handle vsub_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    Value result = v1 - v2;
    return globalHandleManager.create(std::move(result));
}

Handle vmul_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    Value result = v1 * v2;
    return globalHandleManager.create(std::move(result));
}

Handle vdiv_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    Value result = v1 / v2;
    return globalHandleManager.create(std::move(result));
}

Handle vscale_fast(Handle h, double scalar) {
    const Value& v = globalHandleManager.get(h);

    Value scalarValue(scalar);
    Value result = v * scalarValue;

    return globalHandleManager.create(std::move(result));
}

Handle dot_fast(Handle h1, Handle h2) {
    const Value& v1 = globalHandleManager.get(h1);
    const Value& v2 = globalHandleManager.get(h2);

    std::vector<Value> args = { v1, v2 };
    Value result = FunctionRegistry::instance().getFunction("dot")(args);

    return globalHandleManager.create(std::move(result));
}

Handle norm_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("norm")(args);

    return globalHandleManager.create(std::move(result));
}

// ============================================================================
// Mathematical Functions (Vectorized Fast Path - OPTIMIZED)
// ============================================================================

Handle sin_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::sin(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference (asVector returns by value)
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::sin(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("sin_fast: Value must be number or vector");
}

Handle cos_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::cos(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::cos(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("cos_fast: Value must be number or vector");
}

Handle tan_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::tan(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::tan(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("tan_fast: Value must be number or vector");
}

Handle exp_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::exp(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::exp(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("exp_fast: Value must be number or vector");
}

Handle ln_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::log(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::log(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("ln_fast: Value must be number or vector");
}

Handle abs_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::abs(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::abs(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("abs_fast: Value must be number or vector");
}

Handle sqrt_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);

    if (v.isNumber()) {
        return globalHandleManager.create(Value(std::sqrt(v.asNumber())));
    }

    if (v.isVector()) {
        Vector vec = v.asVector();  // Copy, not reference
        const size_t n = vec.size();
        std::vector<double> result;
        result.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            result.push_back(std::sqrt(vec[i]));
        }
        return globalHandleManager.create(Value(Vector(result)));
    }

    throw std::runtime_error("sqrt_fast: Value must be number or vector");
}

// ============================================================================
// Optimization Functions (Fast Path)
// ============================================================================

Handle linspace_fast(double start, double end, size_t n) {
    std::vector<Value> args = { Value(start), Value(end), Value(static_cast<double>(n)) };
    Value result = FunctionRegistry::instance().getFunction("linspace")(args);
    return globalHandleManager.create(std::move(result));
}

Handle fftshift_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("fftshift")(args);
    return globalHandleManager.create(std::move(result));
}

Handle ifftshift_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("ifftshift")(args);
    return globalHandleManager.create(std::move(result));
}

Handle fft_spectrum_fast(Handle signalHandle, double fs, bool shift, bool angular, double omegaRange) {
    const Value& signal = globalHandleManager.get(signalHandle);

    std::vector<Value> args = {
        signal,
        Value(fs),
        Value(shift ? 1.0 : 0.0),
        Value(angular ? 1.0 : 0.0),
        Value(omegaRange)
    };

    Value result = FunctionRegistry::instance().getFunction("fft_spectrum")(args);
    return globalHandleManager.create(std::move(result));
}

// ============================================================================
// Handle Management
// ============================================================================

void releaseHandle(Handle handle) {
    globalHandleManager.release(handle);
}

bool isValidHandle(Handle handle) {
    return globalHandleManager.isValid(handle);
}

int getHandleType(Handle handle) {
    const Value& value = globalHandleManager.get(handle);

    if (value.isNumber()) return 0;
    if (value.isComplex()) return 1;
    if (value.isVector()) return 2;
    if (value.isMatrix()) return 3;
    if (value.isFunction()) return 4;

    return -1; // Unknown
}

Handle cloneHandle(Handle handle) {
    return globalHandleManager.clone(handle);
}

// ============================================================================
// Integration with Evaluator
// ============================================================================

void bindVariableToHandle(const std::string& varName, Handle handle) {
    const Value& value = globalHandleManager.get(handle);

    // Definir la variable en el environment del evaluador
    globalEvaluator.environment().define(varName, value);
}

Handle createHandleFromVariable(const std::string& varName) {
    // Obtener valor del environment
    if (!globalEvaluator.environment().has(varName)) {
        throw std::runtime_error("Variable not found: " + varName);
    }

    const Value& value = globalEvaluator.environment().get(varName);
    return globalHandleManager.create(value);
}

} // namespace bindings
} // namespace achronyme
