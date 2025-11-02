#include "decompositions.hpp"
#include "eigensolvers.hpp"
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
            if (std::abs(A.at(i, j) - A.at(j, i)) > tol) {
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
    std::vector<double> data(n * n, 0.0);
    for (size_t i = 0; i < n; ++i) {
        data[i * n + i] = 1.0;
    }
    return Matrix(n, n, data);
}

Matrix permutation_matrix(const std::vector<size_t>& pivots, size_t n) {
    Matrix P = identity(n);
    for (size_t i = 0; i < pivots.size(); ++i) {
        if (pivots[i] != i) {
            // Swap rows i and pivots[i]
            for (size_t j = 0; j < n; ++j) {
                std::swap(P.at(i, j), P.at(pivots[i], j));
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
        double max_val = std::abs(U.at(k, k));

        for (size_t i = k + 1; i < n; ++i) {
            double val = std::abs(U.at(i, k));
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
                std::swap(U.at(k, j), U.at(pivot_row, j));
            }

            // Swap rows in L (only the already-computed part)
            for (size_t j = 0; j < k; ++j) {
                std::swap(L.at(k, j), L.at(pivot_row, j));
            }
        }

        // Elimination
        for (size_t i = k + 1; i < n; ++i) {
            double factor = U.at(i, k) / U.at(k, k);
            L.at(i, k) = factor;

            // Update U
            for (size_t j = k; j < n; ++j) {
                U.at(i, j) -= factor * U.at(k, j);
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
        if (std::abs(U.at(k, k)) < std::numeric_limits<double>::epsilon() * 100) {
            throw std::runtime_error("Zero pivot encountered - matrix requires pivoting");
        }

        // Elimination
        for (size_t i = k + 1; i < n; ++i) {
            double factor = U.at(i, k) / U.at(k, k);
            L.at(i, k) = factor;

            for (size_t j = k; j < n; ++j) {
                U.at(i, j) -= factor * U.at(k, j);
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

    // Initialize Q and R (flattened row-major)
    std::vector<double> Q_data(m * n, 0.0);
    std::vector<double> R_data(n * n, 0.0);

    // Gram-Schmidt orthogonalization
    for (size_t j = 0; j < n; ++j) {
        // Start with column j of A
        std::vector<double> v(m);
        for (size_t i = 0; i < m; ++i) {
            v[i] = A.at(i, j);
        }

        // Subtract projections onto previous Q columns
        for (size_t k = 0; k < j; ++k) {
            // R(k,j) = Q_k^T * A_j (dot product)
            double dot = 0.0;
            for (size_t i = 0; i < m; ++i) {
                dot += Q_data[i * n + k] * A.at(i, j);
            }
            R_data[k * n + j] = dot;

            // v = v - R(k,j) * Q_k
            for (size_t i = 0; i < m; ++i) {
                v[i] -= dot * Q_data[i * n + k];
            }
        }

        // R(j,j) = ||v||
        double norm = 0.0;
        for (double val : v) norm += val * val;
        norm = std::sqrt(norm);

        if (norm < std::numeric_limits<double>::epsilon() * 100) {
            throw std::runtime_error("Matrix columns are linearly dependent");
        }

        R_data[j * n + j] = norm;

        // Q_j = v / ||v||
        for (size_t i = 0; i < m; ++i) {
            Q_data[i * n + j] = v[i] / norm;
        }
    }

    return {Matrix(m, n, Q_data), Matrix(n, n, R_data)};
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

    // Initialize L (flattened row-major)
    std::vector<double> L_data(n * n, 0.0);

    // Cholesky-Banachiewicz algorithm
    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j <= i; ++j) {
            double sum = 0.0;

            if (j == i) {
                // Diagonal element
                for (size_t k = 0; k < j; ++k) {
                    sum += L_data[j * n + k] * L_data[j * n + k];
                }

                double val = A.at(j, j) - sum;
                if (val <= 0.0) {
                    throw std::runtime_error("Matrix is not positive definite");
                }

                L_data[j * n + j] = std::sqrt(val);
            } else {
                // Off-diagonal element
                for (size_t k = 0; k < j; ++k) {
                    sum += L_data[i * n + k] * L_data[j * n + k];
                }

                L_data[i * n + j] = (A.at(i, j) - sum) / L_data[j * n + j];
            }
        }
    }

    return Matrix(n, n, L_data);
}

// ============================================================================
// SVD Decomposition
// ============================================================================

std::tuple<Matrix, Vector, Matrix> svd_decomposition(const Matrix& A) {
    const size_t m = A.rows();
    const size_t n = A.cols();

    // Compute A^T * A (n x n matrix)
    Matrix At = A.transpose();
    Matrix AtA = At * A;

    // Compute eigenvalues and eigenvectors of A^T * A
    // Eigenvalues are squares of singular values
    // Eigenvectors are right singular vectors (V)
    auto [eigenvalues_vec, V] = eigen_symmetric(AtA);

    // Sort eigenvalues and eigenvectors in descending order
    std::vector<double> eigenvalues = eigenvalues_vec.elements();
    std::vector<size_t> indices(n);
    for (size_t i = 0; i < n; ++i) indices[i] = i;

    std::sort(indices.begin(), indices.end(), [&eigenvalues](size_t i, size_t j) {
        return eigenvalues[i] > eigenvalues[j];
    });

    // Reorder eigenvalues and build singular values
    std::vector<double> singular_values(n);
    std::vector<double> V_sorted_data(n * n);

    for (size_t i = 0; i < n; ++i) {
        size_t idx = indices[i];
        // Singular value = sqrt(eigenvalue)
        singular_values[i] = std::sqrt(std::max(0.0, eigenvalues[idx]));

        // Copy column idx to column i
        for (size_t row = 0; row < n; ++row) {
            V_sorted_data[row * n + i] = V.at(row, idx);
        }
    }

    Matrix V_sorted(n, n, V_sorted_data);

    // Compute U = A * V * Σ^(-1)
    // For each column i of V, compute u_i = (1/σ_i) * A * v_i
    std::vector<double> U_data(m * n, 0.0);

    for (size_t i = 0; i < n; ++i) {
        if (singular_values[i] > 1e-10) {  // Skip near-zero singular values
            // Extract column i from V
            std::vector<double> v_col(n);
            for (size_t j = 0; j < n; ++j) {
                v_col[j] = V_sorted.at(j, i);
            }

            // Compute A * v
            std::vector<double> Av(m, 0.0);
            for (size_t row = 0; row < m; ++row) {
                for (size_t col = 0; col < n; ++col) {
                    Av[row] += A.at(row, col) * v_col[col];
                }
            }

            // u_i = (1/σ_i) * A * v_i
            for (size_t row = 0; row < m; ++row) {
                U_data[row * n + i] = Av[row] / singular_values[i];
            }
        }
    }

    Matrix U(m, n, U_data);
    Vector S(singular_values);

    return {U, S, V_sorted};
}

}  // namespace linalg
}  // namespace achronyme
