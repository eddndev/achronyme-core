#include "decompositions.hpp"
#include <cmath>
#include <algorithm>
#include <limits>

namespace achronyme {
namespace linalg {

// ============================================================================
// Helper Functions
// ============================================================================

bool is_symmetric(const Matrix& A, double tol) {
    if (A.rows() != A.cols()) return false;

    for (size_t i = 0; i < A.rows(); ++i) {
        for (size_t j = i + 1; j < A.cols(); ++j) {
            if (std::abs(A(i, j) - A(j, i)) > tol) {
                return false;
            }
        }
    }
    return true;
}

bool is_positive_definite(const Matrix& A) {
    if (!is_symmetric(A)) return false;

    // Try Cholesky - if it succeeds, matrix is positive definite
    try {
        cholesky_decomposition(A);
        return true;
    } catch (...) {
        return false;
    }
}

Matrix identity(size_t n) {
    std::vector<std::vector<double>> data(n, std::vector<double>(n, 0.0));
    for (size_t i = 0; i < n; ++i) {
        data[i][i] = 1.0;
    }
    return Matrix(data);
}

Matrix permutation_matrix(const std::vector<size_t>& pivots, size_t n) {
    Matrix P = identity(n);
    for (size_t i = 0; i < pivots.size(); ++i) {
        if (pivots[i] != i) {
            // Swap rows i and pivots[i]
            for (size_t j = 0; j < n; ++j) {
                std::swap(P(i, j), P(pivots[i], j));
            }
        }
    }
    return P;
}

// ============================================================================
// LU Decomposition
// ============================================================================

std::tuple<Matrix, Matrix, Matrix> lu_decomposition(const Matrix& A) {
    const size_t n = A.rows();

    // Check if square
    if (A.rows() != A.cols()) {
        throw std::runtime_error("LU decomposition requires square matrix");
    }

    // Initialize L, U, and pivot tracking
    Matrix L = identity(n);
    Matrix U = A;  // Will be transformed in-place
    std::vector<size_t> pivots(n);
    for (size_t i = 0; i < n; ++i) pivots[i] = i;

    // Gaussian elimination with partial pivoting
    for (size_t k = 0; k < n - 1; ++k) {
        // Find pivot (largest element in column k, from row k downwards)
        size_t pivot_row = k;
        double max_val = std::abs(U(k, k));

        for (size_t i = k + 1; i < n; ++i) {
            double val = std::abs(U(i, k));
            if (val > max_val) {
                max_val = val;
                pivot_row = i;
            }
        }

        // Check for singularity
        if (max_val < std::numeric_limits<double>::epsilon() * 100) {
            throw std::runtime_error("Matrix is singular or nearly singular");
        }

        // Swap rows if necessary
        if (pivot_row != k) {
            pivots[k] = pivot_row;

            // Swap rows in U
            for (size_t j = 0; j < n; ++j) {
                std::swap(U(k, j), U(pivot_row, j));
            }

            // Swap rows in L (only the already-computed part)
            for (size_t j = 0; j < k; ++j) {
                std::swap(L(k, j), L(pivot_row, j));
            }
        }

        // Elimination
        for (size_t i = k + 1; i < n; ++i) {
            double factor = U(i, k) / U(k, k);
            L(i, k) = factor;

            // Update U
            for (size_t j = k; j < n; ++j) {
                U(i, j) -= factor * U(k, j);
            }
        }
    }

    // Create permutation matrix from pivots
    Matrix P = permutation_matrix(pivots, n);

    return {L, U, P};
}

std::tuple<Matrix, Matrix> lu_no_pivot(const Matrix& A) {
    const size_t n = A.rows();

    if (A.rows() != A.cols()) {
        throw std::runtime_error("LU decomposition requires square matrix");
    }

    Matrix L = identity(n);
    Matrix U = A;

    for (size_t k = 0; k < n - 1; ++k) {
        // Check for zero pivot
        if (std::abs(U(k, k)) < std::numeric_limits<double>::epsilon() * 100) {
            throw std::runtime_error("Zero pivot encountered - matrix requires pivoting");
        }

        // Elimination
        for (size_t i = k + 1; i < n; ++i) {
            double factor = U(i, k) / U(k, k);
            L(i, k) = factor;

            for (size_t j = k; j < n; ++j) {
                U(i, j) -= factor * U(k, j);
            }
        }
    }

    return {L, U};
}

// ============================================================================
// QR Decomposition
// ============================================================================

std::tuple<Matrix, Matrix> qr_gram_schmidt(const Matrix& A) {
    const size_t m = A.rows();
    const size_t n = A.cols();

    if (m < n) {
        throw std::runtime_error("QR requires m >= n");
    }

    // Initialize Q and R
    std::vector<std::vector<double>> Q_data(m, std::vector<double>(n, 0.0));
    std::vector<std::vector<double>> R_data(n, std::vector<double>(n, 0.0));

    // Gram-Schmidt orthogonalization
    for (size_t j = 0; j < n; ++j) {
        // Start with column j of A
        std::vector<double> v(m);
        for (size_t i = 0; i < m; ++i) {
            v[i] = A(i, j);
        }

        // Subtract projections onto previous Q columns
        for (size_t k = 0; k < j; ++k) {
            // R(k,j) = Q_k^T * A_j (dot product)
            double dot = 0.0;
            for (size_t i = 0; i < m; ++i) {
                dot += Q_data[i][k] * A(i, j);
            }
            R_data[k][j] = dot;

            // v = v - R(k,j) * Q_k
            for (size_t i = 0; i < m; ++i) {
                v[i] -= dot * Q_data[i][k];
            }
        }

        // R(j,j) = ||v||
        double norm = 0.0;
        for (double val : v) norm += val * val;
        norm = std::sqrt(norm);

        if (norm < std::numeric_limits<double>::epsilon() * 100) {
            throw std::runtime_error("Matrix columns are linearly dependent");
        }

        R_data[j][j] = norm;

        // Q_j = v / ||v||
        for (size_t i = 0; i < m; ++i) {
            Q_data[i][j] = v[i] / norm;
        }
    }

    return {Matrix(Q_data), Matrix(R_data)};
}

std::tuple<Matrix, Matrix> qr_decomposition(const Matrix& A) {
    // TODO: Implement Householder reflections (more stable than Gram-Schmidt)
    // For now, use Gram-Schmidt
    return qr_gram_schmidt(A);
}

// ============================================================================
// Cholesky Decomposition
// ============================================================================

Matrix cholesky_decomposition(const Matrix& A) {
    const size_t n = A.rows();

    if (A.rows() != A.cols()) {
        throw std::runtime_error("Cholesky requires square matrix");
    }

    if (!is_symmetric(A)) {
        throw std::runtime_error("Cholesky requires symmetric matrix");
    }

    // Initialize L
    std::vector<std::vector<double>> L_data(n, std::vector<double>(n, 0.0));

    // Cholesky-Banachiewicz algorithm
    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j <= i; ++j) {
            double sum = 0.0;

            if (j == i) {
                // Diagonal element
                for (size_t k = 0; k < j; ++k) {
                    sum += L_data[j][k] * L_data[j][k];
                }

                double val = A(j, j) - sum;
                if (val <= 0.0) {
                    throw std::runtime_error("Matrix is not positive definite");
                }

                L_data[j][j] = std::sqrt(val);
            } else {
                // Off-diagonal element
                for (size_t k = 0; k < j; ++k) {
                    sum += L_data[i][k] * L_data[j][k];
                }

                L_data[i][j] = (A(i, j) - sum) / L_data[j][j];
            }
        }
    }

    return Matrix(L_data);
}

// ============================================================================
// SVD Decomposition
// ============================================================================

std::tuple<Matrix, Vector, Matrix> svd_decomposition(const Matrix& A) {
    // TODO: Implement Golub-Reinsch SVD algorithm
    // This is complex and will be implemented in next iteration
    throw std::runtime_error("SVD not yet implemented - coming in next update");
}

}  // namespace linalg
}  // namespace achronyme
