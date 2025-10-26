#ifndef ACHRONYME_CORE_VALUE_HPP
#define ACHRONYME_CORE_VALUE_HPP

#include <variant>
#include <string>
#include <stdexcept>
#include "complex.hpp"
#include "vector.hpp"
#include "matrix.hpp"

namespace achronyme {
namespace core {

/**
 * Value type for the SOC (Superior Order Calculator)
 *
 * Phase 1-2: Supported double (Number)
 * Phase 3: Now supports Complex, Vector, Matrix
 * Phase 4+: Will support Function, Symbolic expressions
 */
class Value {
public:
    // Type enumeration
    enum class Type {
        NUMBER,
        COMPLEX,
        VECTOR,
        MATRIX
    };

    // Variant type holding all possible values
    using ValueType = std::variant<double, Complex, Vector, Matrix>;

    // Constructors
    Value() : data_(0.0) {}
    Value(double value) : data_(value) {}
    Value(const Complex& value) : data_(value) {}
    Value(const Vector& value) : data_(value) {}
    Value(const Matrix& value) : data_(value) {}

    // Type checking
    Type type() const;
    bool isNumber() const { return std::holds_alternative<double>(data_); }
    bool isComplex() const { return std::holds_alternative<Complex>(data_); }
    bool isVector() const { return std::holds_alternative<Vector>(data_); }
    bool isMatrix() const { return std::holds_alternative<Matrix>(data_); }

    // Type access (throws if wrong type)
    double asNumber() const;
    Complex asComplex() const;
    Vector asVector() const;
    Matrix asMatrix() const;

    // Type coercion (automatic promotion)
    Complex toComplex() const;  // Number → Complex
    Value promoteToCommon(const Value& other) const;  // Find common type

    // Operators for arithmetic (type-dependent dispatch)
    Value operator+(const Value& other) const;
    Value operator-(const Value& other) const;
    Value operator*(const Value& other) const;
    Value operator/(const Value& other) const;
    Value operator-() const;  // Unary minus

    // Power operator
    Value pow(const Value& exponent) const;

    // String representation
    std::string toString() const;

    // Get raw variant (for pattern matching)
    const ValueType& data() const { return data_; }

private:
    ValueType data_;

    // Helper methods for arithmetic operations
    Value numberOp(const Value& other, char op) const;
    Value complexOp(const Value& other, char op) const;
    Value vectorOp(const Value& other, char op) const;
    Value matrixOp(const Value& other, char op) const;
};

} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_VALUE_HPP
