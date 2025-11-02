#include "linalg_bindings.hpp"
#include "../linalg/decompositions.hpp"
#include "../linalg/eigensolvers.hpp"
#include "../core/handle_manager.hpp"
#include <emscripten/bind.h>

namespace achronyme {
namespace bindings {

using namespace core;
using namespace linalg;

// ============================================================================
// LU Decomposition
// ============================================================================

emscripten::val lu_decomposition_js(Handle matrixHandle) {
    try {
        // Get matrix from handle
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("LU decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Perform LU decomposition
        auto [L, U, P] = lu_decomposition(A);

        // Create handles for results
        Handle L_handle = globalHandleManager.create(Value(L));
        Handle U_handle = globalHandleManager.create(Value(U));
        Handle P_handle = globalHandleManager.create(Value(P));

        // Return JavaScript object with handles
        emscripten::val result = emscripten::val::object();
        result.set("L", L_handle);
        result.set("U", U_handle);
        result.set("P", P_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("LU decomposition failed: ") + e.what());
    }
}

emscripten::val lu_no_pivot_js(Handle matrixHandle) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("LU decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();
        auto [L, U] = lu_no_pivot(A);

        Handle L_handle = globalHandleManager.create(Value(L));
        Handle U_handle = globalHandleManager.create(Value(U));

        emscripten::val result = emscripten::val::object();
        result.set("L", L_handle);
        result.set("U", U_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("LU decomposition (no pivot) failed: ") + e.what());
    }
}

// ============================================================================
// QR Decomposition
// ============================================================================

emscripten::val qr_decomposition_js(Handle matrixHandle) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("QR decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Perform QR decomposition
        auto [Q, R] = qr_decomposition(A);

        // Create handles for results
        Handle Q_handle = globalHandleManager.create(Value(Q));
        Handle R_handle = globalHandleManager.create(Value(R));

        // Return JavaScript object
        emscripten::val result = emscripten::val::object();
        result.set("Q", Q_handle);
        result.set("R", R_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("QR decomposition failed: ") + e.what());
    }
}

emscripten::val qr_gram_schmidt_js(Handle matrixHandle) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("QR decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();
        auto [Q, R] = qr_gram_schmidt(A);

        Handle Q_handle = globalHandleManager.create(Value(Q));
        Handle R_handle = globalHandleManager.create(Value(R));

        emscripten::val result = emscripten::val::object();
        result.set("Q", Q_handle);
        result.set("R", R_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("QR (Gram-Schmidt) failed: ") + e.what());
    }
}

// ============================================================================
// Cholesky Decomposition
// ============================================================================

Handle cholesky_decomposition_js(Handle matrixHandle) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("Cholesky decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Perform Cholesky decomposition
        Matrix L = cholesky_decomposition(A);

        // Create handle for result
        return globalHandleManager.create(Value(L));

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("Cholesky decomposition failed: ") + e.what());
    }
}

// ============================================================================
// SVD Decomposition
// ============================================================================

emscripten::val svd_decomposition_js(Handle matrixHandle) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("SVD requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Perform SVD
        auto [U, S, V] = svd_decomposition(A);

        // Create handles for results
        Handle U_handle = globalHandleManager.create(Value(U));
        Handle S_handle = globalHandleManager.create(Value(S));
        Handle V_handle = globalHandleManager.create(Value(V));

        // Return JavaScript object
        emscripten::val result = emscripten::val::object();
        result.set("U", U_handle);
        result.set("S", S_handle);
        result.set("V", V_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("SVD failed: ") + e.what());
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

bool is_symmetric_js(Handle matrixHandle, double tol) {
    const Value& value = globalHandleManager.get(matrixHandle);
    if (!value.isMatrix()) {
        throw std::runtime_error("is_symmetric requires a matrix");
    }

    return is_symmetric(value.asMatrix(), tol);
}

bool is_positive_definite_js(Handle matrixHandle) {
    const Value& value = globalHandleManager.get(matrixHandle);
    if (!value.isMatrix()) {
        throw std::runtime_error("is_positive_definite requires a matrix");
    }

    return is_positive_definite(value.asMatrix());
}

Handle identity_js(size_t n) {
    Matrix I = identity(n);
    return globalHandleManager.create(Value(I));
}

// ============================================================================
// Eigenvalue Solvers
// ============================================================================

emscripten::val power_iteration_js(
    Handle matrixHandle,
    size_t maxIterations,
    double tolerance
) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("Power iteration requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Perform power iteration
        auto [eigenvalue, eigenvector] = power_iteration(A, maxIterations, tolerance);

        // Create handle for eigenvector
        Handle eigenvector_handle = globalHandleManager.create(Value(eigenvector));

        // Return JavaScript object
        emscripten::val result = emscripten::val::object();
        result.set("eigenvalue", eigenvalue);
        result.set("eigenvector", eigenvector_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("Power iteration failed: ") + e.what());
    }
}

Handle qr_eigenvalues_js(
    Handle matrixHandle,
    size_t maxIterations,
    double tolerance
) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("QR eigenvalues requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Compute eigenvalues
        Vector eigenvalues = qr_algorithm_eigenvalues(A, maxIterations, tolerance);

        return globalHandleManager.create(Value(eigenvalues));

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("QR eigenvalues failed: ") + e.what());
    }
}

emscripten::val eigen_symmetric_js(
    Handle matrixHandle,
    size_t maxIterations,
    double tolerance
) {
    try {
        const Value& value = globalHandleManager.get(matrixHandle);
        if (!value.isMatrix()) {
            throw std::runtime_error("Eigen decomposition requires a matrix");
        }

        const Matrix& A = value.asMatrix();

        // Compute eigenvalues and eigenvectors
        auto [eigenvalues, eigenvectors] = eigen_symmetric(A, maxIterations, tolerance);

        // Create handles
        Handle eigenvalues_handle = globalHandleManager.create(Value(eigenvalues));
        Handle eigenvectors_handle = globalHandleManager.create(Value(eigenvectors));

        // Return JavaScript object
        emscripten::val result = emscripten::val::object();
        result.set("eigenvalues", eigenvalues_handle);
        result.set("eigenvectors", eigenvectors_handle);

        return result;

    } catch (const std::exception& e) {
        throw std::runtime_error(std::string("Eigen decomposition failed: ") + e.what());
    }
}

}  // namespace bindings
}  // namespace achronyme
