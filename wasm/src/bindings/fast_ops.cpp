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
// Mathematical Functions (Vectorized Fast Path)
// ============================================================================

Handle sin_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("sin")(args);
    return globalHandleManager.create(std::move(result));
}

Handle cos_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("cos")(args);
    return globalHandleManager.create(std::move(result));
}

Handle tan_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("tan")(args);
    return globalHandleManager.create(std::move(result));
}

Handle exp_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("exp")(args);
    return globalHandleManager.create(std::move(result));
}

Handle ln_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("ln")(args);
    return globalHandleManager.create(std::move(result));
}

Handle abs_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("abs")(args);
    return globalHandleManager.create(std::move(result));
}

Handle sqrt_fast(Handle h) {
    const Value& v = globalHandleManager.get(h);
    std::vector<Value> args = { v };
    Value result = FunctionRegistry::instance().getFunction("sqrt")(args);
    return globalHandleManager.create(std::move(result));
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
