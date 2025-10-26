#ifndef ACHRONYME_CORE_CONSTANTS_HPP
#define ACHRONYME_CORE_CONSTANTS_HPP

#include <string>
#include <unordered_map>

namespace achronyme {
namespace core {
namespace constants {

// Mathematical constants
constexpr double PI = 3.141592653589793238462643383279502884;
constexpr double E = 2.718281828459045235360287471352662498;
constexpr double PHI = 1.618033988749894848204586834365638118;  // Golden ratio
constexpr double SQRT2 = 1.414213562373095048801688724209698079;
constexpr double SQRT3 = 1.732050807568877293527446341505872367;
constexpr double LN2 = 0.693147180559945309417232121458176568;
constexpr double LN10 = 2.302585092994045684017991454684364208;

/**
 * Constants Registry
 *
 * Maps constant names to their values.
 * Supports case-insensitive lookup (PI = pi = Pi).
 */
class ConstantsRegistry {
public:
    static ConstantsRegistry& instance();

    // Check if a constant exists
    bool hasConstant(const std::string& name) const;

    // Get constant value
    double getConstant(const std::string& name) const;

private:
    ConstantsRegistry();

    std::unordered_map<std::string, double> constants_;

    // Helper to convert to lowercase for case-insensitive lookup
    static std::string toLower(const std::string& str);
};

} // namespace constants
} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_CONSTANTS_HPP
