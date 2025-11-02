#pragma once

#include "../core/value.hpp"
#include "../core/handle_manager.hpp"
#include <emscripten/val.h>

namespace achronyme {
namespace bindings {

using Handle = core::HandleManager::Handle;

// ============================================================================
// LU Decomposition
// ============================================================================

/**
 * LU Decomposition with partial pivoting: PA = LU
 *
 * @param matrixHandle Handle to input matrix
 * @return JavaScript object with handles: { L, U, P }
 */
emscripten::val lu_decomposition_js(Handle matrixHandle);

/**
 * LU Decomposition without pivoting: A = LU
 *
 * @param matrixHandle Handle to input matrix
 * @return JavaScript object with handles: { L, U }
 */
emscripten::val lu_no_pivot_js(Handle matrixHandle);

// ============================================================================
// QR Decomposition
// ============================================================================

/**
 * QR Decomposition using Householder reflections: A = QR
 *
 * @param matrixHandle Handle to input matrix
 * @return JavaScript object with handles: { Q, R }
 */
emscripten::val qr_decomposition_js(Handle matrixHandle);

/**
 * QR Decomposition using Gram-Schmidt: A = QR
 *
 * @param matrixHandle Handle to input matrix
 * @return JavaScript object with handles: { Q, R }
 */
emscripten::val qr_gram_schmidt_js(Handle matrixHandle);

// ============================================================================
// Cholesky Decomposition
// ============================================================================

/**
 * Cholesky Decomposition: A = LL^T
 *
 * @param matrixHandle Handle to symmetric positive definite matrix
 * @return Handle to lower triangular matrix L
 */
Handle cholesky_decomposition_js(Handle matrixHandle);

// ============================================================================
// SVD Decomposition
// ============================================================================

/**
 * Singular Value Decomposition: A = UΣV^T
 *
 * @param matrixHandle Handle to input matrix
 * @return JavaScript object with handles: { U, S, V }
 */
emscripten::val svd_decomposition_js(Handle matrixHandle);

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Check if matrix is symmetric
 *
 * @param matrixHandle Handle to matrix
 * @param tol Tolerance for symmetry check
 * @return true if symmetric within tolerance
 */
bool is_symmetric_js(Handle matrixHandle, double tol = 1e-12);

/**
 * Check if matrix is positive definite
 *
 * @param matrixHandle Handle to matrix
 * @return true if positive definite
 */
bool is_positive_definite_js(Handle matrixHandle);

/**
 * Create identity matrix of size n×n
 *
 * @param n Size of identity matrix
 * @return Handle to identity matrix
 */
Handle identity_js(size_t n);

// ============================================================================
// Eigenvalue Solvers
// ============================================================================

/**
 * Power Iteration - Find dominant eigenvalue and eigenvector
 *
 * @param matrixHandle Handle to square matrix
 * @param maxIterations Maximum iterations (default: 1000)
 * @param tolerance Convergence tolerance (default: 1e-10)
 * @return JavaScript object with { eigenvalue: number, eigenvector: Handle }
 */
emscripten::val power_iteration_js(
    Handle matrixHandle,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

/**
 * Compute all eigenvalues using QR algorithm
 *
 * @param matrixHandle Handle to square matrix
 * @param maxIterations Maximum iterations
 * @param tolerance Convergence tolerance
 * @return Handle to vector of eigenvalues
 */
Handle qr_eigenvalues_js(
    Handle matrixHandle,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

/**
 * Compute eigenvalues and eigenvectors for symmetric matrix
 *
 * @param matrixHandle Handle to symmetric square matrix
 * @param maxIterations Maximum iterations
 * @param tolerance Convergence tolerance
 * @return JavaScript object with { eigenvalues: Handle, eigenvectors: Handle }
 */
emscripten::val eigen_symmetric_js(
    Handle matrixHandle,
    size_t maxIterations = 1000,
    double tolerance = 1e-10
);

}  // namespace bindings
}  // namespace achronyme
