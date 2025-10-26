#include "constants.hpp"
#include <algorithm>
#include <stdexcept>
#include <cctype>

namespace achronyme {
namespace core {
namespace constants {

ConstantsRegistry::ConstantsRegistry() {
    // Register all constants (lowercase keys for case-insensitive lookup)
    constants_["pi"] = PI;
    constants_["e"] = E;
    constants_["phi"] = PHI;
    constants_["sqrt2"] = SQRT2;
    constants_["sqrt3"] = SQRT3;
    constants_["ln2"] = LN2;
    constants_["ln10"] = LN10;

    // Common aliases
    constants_["goldenratio"] = PHI;
}

ConstantsRegistry& ConstantsRegistry::instance() {
    static ConstantsRegistry registry;
    return registry;
}

bool ConstantsRegistry::hasConstant(const std::string& name) const {
    std::string lowerName = toLower(name);
    return constants_.find(lowerName) != constants_.end();
}

double ConstantsRegistry::getConstant(const std::string& name) const {
    std::string lowerName = toLower(name);
    auto it = constants_.find(lowerName);

    if (it == constants_.end()) {
        throw std::runtime_error("Unknown constant: " + name);
    }

    return it->second;
}

std::string ConstantsRegistry::toLower(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(),
                   [](unsigned char c) { return std::tolower(c); });
    return result;
}

} // namespace constants
} // namespace core
} // namespace achronyme
