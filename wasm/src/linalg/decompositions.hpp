#pragma once

#include "../core/matrix.hpp"
#include <tuple>
#include <stdexcept>

namespace achronyme {
namespace linalg {

using namespace core;

/**
 * Matrix Decompositions for Advanced Linear Algebra
 *
 * Implements standard matrix factorizations used in numerical computing:
 * - LU Decomposition (with partial pivoting)
 * - QR Decomposition (Householder reflections)
 * - Cholesky Decomposition (for positive definite matrices)
 * - SVD Decomposition (Singular Value Decomposition)
 */

// ============================================================================
// LU Decomposition: PA = LU
// ============================================================================

/**
 * LU Decomposition with partial pivoting
 *
 * Factorizes matrix A into:
 *   P × A = L × U
 * where:
 *   - P is a permutation matrix
 *   - L is lower triangular with 1s on diagonal
 *   - U is upper triangular
 *
 * Algorithm: Gaussian elimination with partial pivoting
 * Complexity: O(n³)
 * Stability: Numerically stable with pivoting
 *
 * @param A Square matrix to decompose (n×n)
 * @return Tuple (L, U, P)
 * @throws std::runtime_error if matrix is singular or not square
 *
 * Example:
 *   Matrix A = {{4, 3}, {6, 3}};
 *   auto [L, U, P] = lu_decomposition(A);
 *   // Verify: P * A == L * U
 */
std::tuple<Matrix, Matrix, Matrix> lu_decomposition(const Matrix& A);

/**
 * LU Decomposition without pivoting (faster but less stable)
 *
 * Use only when you know the matrix is well-conditioned
 * and won't encounter zero pivots.
 *
 * @param A Square matrix to decompose
 * @return Tuple (L, U)
 */
std::tuple<Matrix, Matrix> lu_no_pivot(const Matrix& A);

// ============================================================================
// QR Decomposition: A = QR
// ============================================================================

/**
 * QR Decomposition using Householder reflections
 *
 * Factorizes matrix A into:
 *   A = Q × R
 * where:
 *   - Q is orthogonal (Q^T × Q = I)
 *   - R is upper triangular
 *
 * Algorithm: Householder reflections
 * Complexity: O(mn²) for m×n matrix
 * Stability: Numerically stable
 *
 * @param A Matrix to decompose (m×n, m >= n)
 * @return Tuple (Q, R)
 *
 * Applications:
 *   - Least squares problems
 *   - Eigenvalue computation (QR algorithm)
 *   - Orthogonalization
 */
std::tuple<Matrix, Matrix> qr_decomposition(const Matrix& A);

/**
 * QR Decomposition using Gram-Schmidt (less stable, educational)
 *
 * Simpler but less numerically stable than Householder.
 * Included for educational purposes and simple cases.
 */
std::tuple<Matrix, Matrix> qr_gram_schmidt(const Matrix& A);

// ============================================================================
// Cholesky Decomposition: A = L×L^T
// ============================================================================

/**
 * Cholesky Decomposition for symmetric positive definite matrices
 *
 * Factorizes matrix A into:
 *   A = L × L^T
 * where:
 *   - L is lower triangular
 *
 * Algorithm: Cholesky-Banachiewicz
 * Complexity: O(n³/3) - Faster than LU
 * Requirements: A must be symmetric and positive definite
 *
 * @param A Symmetric positive definite matrix
 * @return L (lower triangular matrix)
 * @throws std::runtime_error if not positive definite
 *
 * Applications:
 *   - Solving systems with symmetric matrices
 *   - Monte Carlo simulations
 *   - Optimization problems
 */
Matrix cholesky_decomposition(const Matrix& A);

// ============================================================================
// SVD: A = UΣV^T
// ============================================================================

/**
 * Singular Value Decomposition
 *
 * Factorizes matrix A into:
 *   A = U × Σ × V^T
 * where:
 *   - U: m×m orthogonal (left singular vectors)
 *   - Σ: m×n diagonal (singular values, non-negative)
 *   - V: n×n orthogonal (right singular vectors)
 *
 * Algorithm: Golub-Reinsch with bidiagonalization
 * Complexity: O(min(m,n)² × max(m,n))
 *
 * @param A Matrix to decompose (m×n)
 * @return Tuple (U, S, V) where S is vector of singular values
 *
 * Applications:
 *   - Principal Component Analysis (PCA)
 *   - Data compression
 *   - Pseudoinverse computation
 *   - Low-rank approximation
 */
std::tuple<Matrix, Vector, Matrix> svd_decomposition(const Matrix& A);

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Check if matrix is positive definite
 * Uses Sylvester's criterion (all leading principal minors > 0)
 */
bool is_positive_definite(const Matrix& A);

/**
 * Check if matrix is symmetric
 */
bool is_symmetric(const Matrix& A, double tol = 1e-12);

/**
 * Create identity matrix of size n×n
 */
Matrix identity(size_t n);

/**
 * Create permutation matrix from pivot vector
 */
Matrix permutation_matrix(const std::vector<size_t>& pivots, size_t n);

}  // namespace linalg
}  // namespace achronyme
