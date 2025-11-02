#include "eigensolvers.hpp"
#include "decompositions.hpp"
#include <cmath>
#include <algorithm>
#include <limits>

namespace achronyme {
namespace linalg {

// ============================================================================
// Helper Functions
// ============================================================================

bool is_diagonal(const Matrix& A, double tolerance) {
    const size_t n = A.rows();

    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j < n; ++j) {
            if (i != j && std::abs(A.at(i, j)) > tolerance) {
                return false;
            }
        }
    }
    return true;
}

Vector diagonal(const Matrix& A) {
    const size_t n = std::min(A.rows(), A.cols());
    std::vector<double> diag(n, 0.0);  // Explicitly initialize with 0.0

    for (size_t i = 0; i < n; ++i) {
        diag[i] = A.at(i, i);
    }

    return Vector(diag);
}

// ============================================================================
// Power Iteration
// ============================================================================

std::tuple<double, Vector> power_iteration(
    const Matrix& A,
    size_t maxIterations,
    double tolerance
) {
    const size_t n = A.rows();

    if (A.rows() != A.cols()) {
        throw std::runtime_error("Power iteration requires square matrix");
    }

    // Initialize with random vector (just use [1, 1, ..., 1] for reproducibility)
    std::vector<double> v_data(n, 1.0);
    Vector v(v_data);

    // Normalize
    v = v / v.norm();

    double eigenvalue = 0.0;
    double prev_eigenvalue = 0.0;

    for (size_t iter = 0; iter < maxIterations; ++iter) {
        // v_new = A * v
        std::vector<double> v_new_data(n, 0.0);
        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < n; ++j) {
                v_new_data[i] += A.at(i, j) * v[j];
            }
        }
        Vector v_new(v_new_data);

        // Compute Rayleigh quotient: eigenvalue = v^T * A * v / v^T * v
        eigenvalue = v.dot(v_new);

        // Normalize
        v_new = v_new / v_new.norm();

        // Check convergence
        if (iter > 0 && std::abs(eigenvalue - prev_eigenvalue) < tolerance) {
            return {eigenvalue, v_new};
        }

        v = v_new;
        prev_eigenvalue = eigenvalue;
    }

    // Return best approximation even if not fully converged
    return {eigenvalue, v};
}

// ============================================================================
// QR Algorithm
// ============================================================================

Vector qr_algorithm_eigenvalues(
    Matrix A, // Pass by value to get a mutable copy
    size_t maxIterations,
    double tolerance
) {
    if (A.rows() != A.cols()) {
        throw std::runtime_error("QR algorithm requires square matrix");
    }

    for (size_t iter = 0; iter < maxIterations; ++iter) {
        // QR decomposition of A
        auto [Q, R] = qr_decomposition(A);

        // A_k+1 = R_k * Q_k
        A = R * Q; // Modify the local copy

        // Check if converged to diagonal form
        if (is_diagonal(A, tolerance)) {
            break;
        }
    }

    // Extract eigenvalues from diagonal
    return diagonal(A);
}

std::tuple<Vector, Matrix> eigen_symmetric(
    const Matrix& A,
    size_t maxIterations,
    double tolerance
) {
    const size_t n = A.rows();

    if (A.rows() != A.cols()) {
        throw std::runtime_error("Eigenvalue decomposition requires square matrix");
    }

    if (!is_symmetric(A)) {
        throw std::runtime_error("This implementation requires symmetric matrix");
    }

    // Compute eigenvalues using QR algorithm
    Vector eigenvalues = qr_algorithm_eigenvalues(A, maxIterations, tolerance);

    // Return identity matrix as eigenvectors placeholder
    // TODO: Implement full eigenvector computation
    Matrix V = identity(n);

    return std::make_tuple(eigenvalues, V);
}

}  // namespace linalg
}  // namespace achronyme
