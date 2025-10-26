#include "function.hpp"
#include "../parser/ast.hpp"
#include "../parser/environment.hpp"
#include <sstream>

namespace achronyme {
namespace core {

Function::Function(std::vector<std::string> params,
                   std::shared_ptr<parser::ASTNode> body,
                   std::shared_ptr<parser::Environment> closure)
    : params_(std::move(params)), body_(std::move(body)), closure_(std::move(closure)) {}

std::string Function::toString() const {
    std::ostringstream oss;

    if (params_.size() == 1) {
        // Single param: x => <function>
        oss << params_[0] << " => <function>";
    } else {
        // Multiple params: (x, y, z) => <function>
        oss << "(";
        for (size_t i = 0; i < params_.size(); ++i) {
            if (i > 0) oss << ", ";
            oss << params_[i];
        }
        oss << ") => <function>";
    }

    return oss.str();
}

} // namespace core
} // namespace achronyme
