#include "value.hpp"
#include <cmath>

namespace achronyme {
namespace core {

Value Value::pow(const Value& exponent) const {
    return Value(std::pow(data_, exponent.data_));
}

} // namespace core
} // namespace achronyme
