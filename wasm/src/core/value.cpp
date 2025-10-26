#include "value.hpp"
#include <cmath>
#include <sstream>

namespace achronyme {
namespace core {

// Type determination
Value::Type Value::type() const {
    if (isNumber()) return Type::NUMBER;
    if (isComplex()) return Type::COMPLEX;
    if (isVector()) return Type::VECTOR;
    if (isMatrix()) return Type::MATRIX;
    if (isFunction()) return Type::FUNCTION;
    throw std::runtime_error("Unknown value type");
}

// Type access methods
double Value::asNumber() const {
    if (!isNumber()) {
        throw std::runtime_error("Value is not a number");
    }
    return std::get<double>(data_);
}

Complex Value::asComplex() const {
    if (isComplex()) {
        return std::get<Complex>(data_);
    }
    if (isNumber()) {
        return Complex(std::get<double>(data_), 0.0);
    }
    throw std::runtime_error("Value cannot be converted to complex");
}

Vector Value::asVector() const {
    if (!isVector()) {
        throw std::runtime_error("Value is not a vector");
    }
    return std::get<Vector>(data_);
}

Matrix Value::asMatrix() const {
    if (!isMatrix()) {
        throw std::runtime_error("Value is not a matrix");
    }
    return std::get<Matrix>(data_);
}

Function Value::asFunction() const {
    if (!isFunction()) {
        throw std::runtime_error("Value is not a function");
    }
    return std::get<Function>(data_);
}

// Type coercion
Complex Value::toComplex() const {
    if (isComplex()) {
        return std::get<Complex>(data_);
    }
    if (isNumber()) {
        return Complex(std::get<double>(data_), 0.0);
    }
    throw std::runtime_error("Cannot convert to complex number");
}

// Addition
Value Value::operator+(const Value& other) const {
    // Number + Number
    if (isNumber() && other.isNumber()) {
        return Value(asNumber() + other.asNumber());
    }

    // Complex + Complex (or Number promoted to Complex)
    if ((isNumber() || isComplex()) && (other.isNumber() || other.isComplex())) {
        return Value(toComplex() + other.toComplex());
    }

    // Vector + Vector
    if (isVector() && other.isVector()) {
        return Value(asVector() + other.asVector());
    }

    // Matrix + Matrix
    if (isMatrix() && other.isMatrix()) {
        return Value(asMatrix() + other.asMatrix());
    }

    // Scalar + Vector (broadcast)
    if (isNumber() && other.isVector()) {
        Vector vec = other.asVector();
        std::vector<double> elements;
        for (size_t i = 0; i < vec.size(); ++i) {
            elements.push_back(vec[i] + asNumber());
        }
        return Value(Vector(elements));
    }

    if (isVector() && other.isNumber()) {
        Vector vec = asVector();
        std::vector<double> elements;
        for (size_t i = 0; i < vec.size(); ++i) {
            elements.push_back(vec[i] + other.asNumber());
        }
        return Value(Vector(elements));
    }

    // Scalar + Matrix (broadcast)
    if (isNumber() && other.isMatrix()) {
        Matrix mat = other.asMatrix();
        std::vector<double> data;
        for (size_t i = 0; i < mat.size(); ++i) {
            data.push_back(mat.data()[i] + asNumber());
        }
        return Value(Matrix(mat.rows(), mat.cols(), data));
    }

    if (isMatrix() && other.isNumber()) {
        Matrix mat = asMatrix();
        std::vector<double> data;
        for (size_t i = 0; i < mat.size(); ++i) {
            data.push_back(mat.data()[i] + other.asNumber());
        }
        return Value(Matrix(mat.rows(), mat.cols(), data));
    }

    throw std::runtime_error("Incompatible types for addition");
}

// Subtraction
Value Value::operator-(const Value& other) const {
    // Number - Number
    if (isNumber() && other.isNumber()) {
        return Value(asNumber() - other.asNumber());
    }

    // Complex - Complex (or Number promoted to Complex)
    if ((isNumber() || isComplex()) && (other.isNumber() || other.isComplex())) {
        return Value(toComplex() - other.toComplex());
    }

    // Vector - Vector
    if (isVector() && other.isVector()) {
        return Value(asVector() - other.asVector());
    }

    // Matrix - Matrix
    if (isMatrix() && other.isMatrix()) {
        return Value(asMatrix() - other.asMatrix());
    }

    // Scalar - Vector
    if (isNumber() && other.isVector()) {
        Vector vec = other.asVector();
        std::vector<double> elements;
        for (size_t i = 0; i < vec.size(); ++i) {
            elements.push_back(asNumber() - vec[i]);
        }
        return Value(Vector(elements));
    }

    if (isVector() && other.isNumber()) {
        Vector vec = asVector();
        std::vector<double> elements;
        for (size_t i = 0; i < vec.size(); ++i) {
            elements.push_back(vec[i] - other.asNumber());
        }
        return Value(Vector(elements));
    }

    throw std::runtime_error("Incompatible types for subtraction");
}

// Multiplication
Value Value::operator*(const Value& other) const {
    // Number * Number
    if (isNumber() && other.isNumber()) {
        return Value(asNumber() * other.asNumber());
    }

    // Complex * Complex (or Number promoted to Complex)
    if ((isNumber() || isComplex()) && (other.isNumber() || other.isComplex())) {
        return Value(toComplex() * other.toComplex());
    }

    // Vector * Scalar (or Scalar * Vector)
    if (isVector() && other.isNumber()) {
        return Value(asVector() * other.asNumber());
    }

    if (isNumber() && other.isVector()) {
        return Value(other.asVector() * asNumber());
    }

    // Matrix * Scalar (or Scalar * Matrix)
    if (isMatrix() && other.isNumber()) {
        return Value(asMatrix() * other.asNumber());
    }

    if (isNumber() && other.isMatrix()) {
        return Value(other.asMatrix() * asNumber());
    }

    // Matrix * Matrix
    if (isMatrix() && other.isMatrix()) {
        return Value(asMatrix() * other.asMatrix());
    }

    throw std::runtime_error("Incompatible types for multiplication");
}

// Division
Value Value::operator/(const Value& other) const {
    // Number / Number
    if (isNumber() && other.isNumber()) {
        if (other.asNumber() == 0.0) {
            throw std::runtime_error("Division by zero");
        }
        return Value(asNumber() / other.asNumber());
    }

    // Complex / Complex (or Number promoted to Complex)
    if ((isNumber() || isComplex()) && (other.isNumber() || other.isComplex())) {
        return Value(toComplex() / other.toComplex());
    }

    // Vector / Scalar
    if (isVector() && other.isNumber()) {
        return Value(asVector() / other.asNumber());
    }

    // Matrix / Scalar
    if (isMatrix() && other.isNumber()) {
        return Value(asMatrix() / other.asNumber());
    }

    throw std::runtime_error("Incompatible types for division");
}

// Unary minus
Value Value::operator-() const {
    if (isNumber()) {
        return Value(-asNumber());
    }
    if (isComplex()) {
        return Value(-asComplex());
    }
    if (isVector()) {
        return Value(-asVector());
    }
    if (isMatrix()) {
        return Value(-asMatrix());
    }
    throw std::runtime_error("Unary minus not supported for this type");
}

// Power
Value Value::pow(const Value& exponent) const {
    // Number ^ Number
    if (isNumber() && exponent.isNumber()) {
        return Value(std::pow(asNumber(), exponent.asNumber()));
    }

    // Complex ^ Complex (or Number promoted to Complex)
    if ((isNumber() || isComplex()) && (exponent.isNumber() || exponent.isComplex())) {
        return Value(toComplex().pow(exponent.toComplex()));
    }

    throw std::runtime_error("Incompatible types for power operation");
}

// String representation
std::string Value::toString() const {
    if (isNumber()) {
        std::ostringstream oss;
        oss << asNumber();
        return oss.str();
    }
    if (isComplex()) {
        return asComplex().toString();
    }
    if (isVector()) {
        return asVector().toString();
    }
    if (isMatrix()) {
        return asMatrix().toString();
    }
    if (isFunction()) {
        return asFunction().toString();
    }
    return "<unknown>";
}

} // namespace core
} // namespace achronyme
