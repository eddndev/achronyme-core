#ifndef ACHRONYME_CORE_VALUE_HPP
#define ACHRONYME_CORE_VALUE_HPP

#include <variant>
#include <string>
#include <stdexcept>

namespace achronyme {
namespace core {

/**
 * Value type for the SOC (Superior Order Calculator)
 *
 * Phase 1: Only supports double (Number)
 * Phase 3: Will support Complex, Vector, Matrix, Function
 */
class Value {
public:
    // Phase 1: Simple double wrapper
    using ValueType = double;

    // Constructors
    Value() : data_(0.0) {}
    Value(double value) : data_(value) {}

    // Get the value as double
    double asNumber() const { return data_; }

    // Operators for arithmetic
    Value operator+(const Value& other) const {
        return Value(data_ + other.data_);
    }

    Value operator-(const Value& other) const {
        return Value(data_ - other.data_);
    }

    Value operator*(const Value& other) const {
        return Value(data_ * other.data_);
    }

    Value operator/(const Value& other) const {
        if (other.data_ == 0.0) {
            throw std::runtime_error("Division by zero");
        }
        return Value(data_ / other.data_);
    }

    // Unary minus
    Value operator-() const {
        return Value(-data_);
    }

    // Power operator (will implement with std::pow)
    Value pow(const Value& exponent) const;

private:
    ValueType data_;
};

} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_VALUE_HPP
