#pragma once

#include "../core/matrix.hpp"
#include "../core/vector.hpp"
#include <tuple>
#include <stdexcept>

namespace achronyme {
namespace linalg {

using namespace core;

/**
 * Eigenvalue and Eigenvector Solvers
 *
 * Provides algorithms for computing eigenvalues and eigenvectors of matrices.
 */

// ============================================================================
// Power Iteration Method
// ============================================================================

/**
 * Power Iteration - Find dominant eigenvalue and eigenvector
 *
 * Iteratively computes the largest eigenvalue (by magnitude) and its
 * corresponding eigenvector.
 *
 * Algorithm: Power iteration with normalization
 * Complexity: O(n² × iterations)
 * Convergence: Linear (depends on eigenvalue ratio)
 *
 * @param A Square matrix
 * @param maxIterations Maximum number of iterations (default: 1000)
 * @param tolerance Convergence tolerance (default: 1e-10)
 * @return Tuple (eigenvalue, eigenvector)
 * @throws std::runtime_error if doesn't converge
 *
 * Example:
 *   Matrix A = {{4, 1}, {2, 3}};
 *   auto [lambda, v] = power_iteration(A);
 */
std::tuple<double, Vector> power_iteration(
    const Matrix& A,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

// ============================================================================
// QR Algorithm for Eigenvalues
// ============================================================================

/**
 * QR Algorithm - Compute all eigenvalues
 *
 * Uses iterative QR decomposition to find all eigenvalues of a matrix.
 * For symmetric matrices, this is very effective.
 *
 * Algorithm: QR iteration with shifts
 * Complexity: O(n³ × iterations)
 * Best for: Symmetric or nearly symmetric matrices
 *
 * @param A Square matrix
 * @param maxIterations Maximum iterations (default: 1000)
 * @param tolerance Convergence tolerance (default: 1e-10)
 * @return Vector of eigenvalues
 *
 * Note: For non-symmetric matrices, may return real parts of complex eigenvalues
 */
Vector qr_algorithm_eigenvalues(
    Matrix A,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

/**
 * Compute eigenvalues and eigenvectors for symmetric matrices
 *
 * Uses QR algorithm with accumulation of Q matrices to get eigenvectors.
 * Only works reliably for symmetric matrices.
 *
 * @param A Symmetric square matrix
 * @param maxIterations Maximum iterations
 * @param tolerance Convergence tolerance
 * @return Tuple (eigenvalues as Vector, eigenvectors as Matrix columns)
 *
 * Example:
 *   Matrix A = {{2, 1}, {1, 2}};  // Symmetric
 *   auto [eigenvalues, eigenvectors] = eigen_symmetric(A);
 */
std::tuple<Vector, Matrix> eigen_symmetric(
    const Matrix& A,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Check convergence of off-diagonal elements
 * Used internally by QR algorithm
 */
bool is_diagonal(const Matrix& A, double tolerance);

/**
 * Extract diagonal elements as vector
 */
Vector diagonal(const Matrix& A);

}  // namespace linalg
}  // namespace achronyme
