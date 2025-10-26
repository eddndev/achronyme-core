#include "matrix.hpp"
#include <cmath>
#include <sstream>
#include <iomanip>
#include <algorithm>

namespace achronyme {
namespace core {

Matrix::Matrix(size_t rows, size_t cols, std::vector<double> data)
    : rows_(rows), cols_(cols), data_(std::move(data)) {
    if (data_.size() != rows * cols) {
        throw std::invalid_argument(
            "Matrix data size mismatch: expected " +
            std::to_string(rows * cols) + " elements, got " +
            std::to_string(data_.size())
        );
    }
}

Matrix::Matrix(size_t rows, size_t cols, double value)
    : rows_(rows), cols_(cols), data_(rows * cols, value) {}

double Matrix::at(size_t row, size_t col) const {
    if (row >= rows_ || col >= cols_) {
        throw std::out_of_range("Matrix index out of range");
    }
    return data_[index(row, col)];
}

double& Matrix::at(size_t row, size_t col) {
    if (row >= rows_ || col >= cols_) {
        throw std::out_of_range("Matrix index out of range");
    }
    return data_[index(row, col)];
}

// Transpose: A^T
Matrix Matrix::transpose() const {
    std::vector<double> result(rows_ * cols_);

    for (size_t i = 0; i < rows_; ++i) {
        for (size_t j = 0; j < cols_; ++j) {
            result[j * rows_ + i] = at(i, j);
        }
    }

    return Matrix(cols_, rows_, result);
}

// Determinant
double Matrix::determinant() const {
    if (!isSquare()) {
        throw std::runtime_error("Determinant is only defined for square matrices");
    }

    if (rows_ == 0) return 1.0;
    if (rows_ == 1) return data_[0];
    if (rows_ == 2) return determinant2x2();
    if (rows_ == 3) return determinant3x3();

    return determinantNxN();
}

// 2x2 determinant: |a b| = ad - bc
//                  |c d|
double Matrix::determinant2x2() const {
    return at(0, 0) * at(1, 1) - at(0, 1) * at(1, 0);
}

// 3x3 determinant using rule of Sarrus
double Matrix::determinant3x3() const {
    double a = at(0, 0), b = at(0, 1), c = at(0, 2);
    double d = at(1, 0), e = at(1, 1), f = at(1, 2);
    double g = at(2, 0), h = at(2, 1), i = at(2, 2);

    return a * e * i + b * f * g + c * d * h
         - c * e * g - b * d * i - a * f * h;
}

// General NxN determinant using Gaussian elimination
double Matrix::determinantNxN() const {
    // Create a copy for Gaussian elimination
    std::vector<double> mat = data_;
    size_t n = rows_;
    double det = 1.0;

    // Forward elimination
    for (size_t i = 0; i < n; ++i) {
        // Find pivot
        size_t pivot = i;
        double max_val = std::abs(mat[i * n + i]);

        for (size_t k = i + 1; k < n; ++k) {
            double val = std::abs(mat[k * n + i]);
            if (val > max_val) {
                max_val = val;
                pivot = k;
            }
        }

        // Swap rows if needed
        if (pivot != i) {
            for (size_t j = 0; j < n; ++j) {
                std::swap(mat[i * n + j], mat[pivot * n + j]);
            }
            det = -det;  // Row swap changes sign
        }

        // Check for singular matrix
        if (std::abs(mat[i * n + i]) < 1e-10) {
            return 0.0;
        }

        // Eliminate column
        for (size_t k = i + 1; k < n; ++k) {
            double factor = mat[k * n + i] / mat[i * n + i];
            for (size_t j = i; j < n; ++j) {
                mat[k * n + j] -= factor * mat[i * n + j];
            }
        }
    }

    // Product of diagonal elements
    for (size_t i = 0; i < n; ++i) {
        det *= mat[i * n + i];
    }

    return det;
}

// Matrix inverse (using Gauss-Jordan elimination)
Matrix Matrix::inverse() const {
    if (!isSquare()) {
        throw std::runtime_error("Inverse is only defined for square matrices");
    }

    size_t n = rows_;

    // Create augmented matrix [A | I]
    std::vector<double> aug(n * 2 * n);
    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j < n; ++j) {
            aug[i * 2 * n + j] = at(i, j);
            aug[i * 2 * n + n + j] = (i == j) ? 1.0 : 0.0;
        }
    }

    // Gauss-Jordan elimination
    for (size_t i = 0; i < n; ++i) {
        // Find pivot
        size_t pivot = i;
        double max_val = std::abs(aug[i * 2 * n + i]);

        for (size_t k = i + 1; k < n; ++k) {
            double val = std::abs(aug[k * 2 * n + i]);
            if (val > max_val) {
                max_val = val;
                pivot = k;
            }
        }

        // Swap rows
        if (pivot != i) {
            for (size_t j = 0; j < 2 * n; ++j) {
                std::swap(aug[i * 2 * n + j], aug[pivot * 2 * n + j]);
            }
        }

        // Check for singular matrix
        if (std::abs(aug[i * 2 * n + i]) < 1e-10) {
            throw std::runtime_error("Matrix is singular (non-invertible)");
        }

        // Scale pivot row
        double pivot_val = aug[i * 2 * n + i];
        for (size_t j = 0; j < 2 * n; ++j) {
            aug[i * 2 * n + j] /= pivot_val;
        }

        // Eliminate column
        for (size_t k = 0; k < n; ++k) {
            if (k != i) {
                double factor = aug[k * 2 * n + i];
                for (size_t j = 0; j < 2 * n; ++j) {
                    aug[k * 2 * n + j] -= factor * aug[i * 2 * n + j];
                }
            }
        }
    }

    // Extract inverse from right half
    std::vector<double> inv_data(n * n);
    for (size_t i = 0; i < n; ++i) {
        for (size_t j = 0; j < n; ++j) {
            inv_data[i * n + j] = aug[i * 2 * n + n + j];
        }
    }

    return Matrix(n, n, inv_data);
}

// Trace: sum of diagonal elements
double Matrix::trace() const {
    if (!isSquare()) {
        throw std::runtime_error("Trace is only defined for square matrices");
    }

    double sum = 0.0;
    for (size_t i = 0; i < rows_; ++i) {
        sum += at(i, i);
    }
    return sum;
}

// Addition: A + B
Matrix Matrix::operator+(const Matrix& other) const {
    checkSameSize(other);

    std::vector<double> result(data_.size());
    for (size_t i = 0; i < data_.size(); ++i) {
        result[i] = data_[i] + other.data_[i];
    }
    return Matrix(rows_, cols_, result);
}

// Subtraction: A - B
Matrix Matrix::operator-(const Matrix& other) const {
    checkSameSize(other);

    std::vector<double> result(data_.size());
    for (size_t i = 0; i < data_.size(); ++i) {
        result[i] = data_[i] - other.data_[i];
    }
    return Matrix(rows_, cols_, result);
}

// Matrix multiplication: A * B
Matrix Matrix::operator*(const Matrix& other) const {
    checkMultipliable(other);

    std::vector<double> result(rows_ * other.cols_, 0.0);

    for (size_t i = 0; i < rows_; ++i) {
        for (size_t j = 0; j < other.cols_; ++j) {
            double sum = 0.0;
            for (size_t k = 0; k < cols_; ++k) {
                sum += at(i, k) * other.at(k, j);
            }
            result[i * other.cols_ + j] = sum;
        }
    }

    return Matrix(rows_, other.cols_, result);
}

// Scalar multiplication: A * scalar
Matrix Matrix::operator*(double scalar) const {
    std::vector<double> result(data_.size());
    for (size_t i = 0; i < data_.size(); ++i) {
        result[i] = data_[i] * scalar;
    }
    return Matrix(rows_, cols_, result);
}

// Scalar division: A / scalar
Matrix Matrix::operator/(double scalar) const {
    if (scalar == 0.0) {
        throw std::runtime_error("Division by zero in matrix division");
    }
    return (*this) * (1.0 / scalar);
}

// Unary minus: -A
Matrix Matrix::operator-() const {
    return (*this) * (-1.0);
}

// Comparison
bool Matrix::operator==(const Matrix& other) const {
    if (rows_ != other.rows_ || cols_ != other.cols_) {
        return false;
    }

    const double epsilon = 1e-10;
    for (size_t i = 0; i < data_.size(); ++i) {
        if (std::abs(data_[i] - other.data_[i]) >= epsilon) {
            return false;
        }
    }
    return true;
}

bool Matrix::operator!=(const Matrix& other) const {
    return !(*this == other);
}

// String representation
std::string Matrix::toString() const {
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(6);
    oss << "[";

    for (size_t i = 0; i < rows_; ++i) {
        if (i > 0) oss << ", ";
        oss << "[";
        for (size_t j = 0; j < cols_; ++j) {
            if (j > 0) oss << ", ";
            oss << at(i, j);
        }
        oss << "]";
    }

    oss << "]";
    return oss.str();
}

// Static factory methods
Matrix Matrix::zeros(size_t rows, size_t cols) {
    return Matrix(rows, cols, 0.0);
}

Matrix Matrix::ones(size_t rows, size_t cols) {
    return Matrix(rows, cols, 1.0);
}

Matrix Matrix::identity(size_t size) {
    std::vector<double> data(size * size, 0.0);
    for (size_t i = 0; i < size; ++i) {
        data[i * size + i] = 1.0;
    }
    return Matrix(size, size, data);
}

// Helper methods
void Matrix::checkSameSize(const Matrix& other) const {
    if (rows_ != other.rows_ || cols_ != other.cols_) {
        throw std::runtime_error(
            "Matrix dimension mismatch: (" +
            std::to_string(rows_) + "x" + std::to_string(cols_) + ") vs (" +
            std::to_string(other.rows_) + "x" + std::to_string(other.cols_) + ")"
        );
    }
}

void Matrix::checkMultipliable(const Matrix& other) const {
    if (cols_ != other.rows_) {
        throw std::runtime_error(
            "Cannot multiply matrices: columns of first (" +
            std::to_string(cols_) + ") != rows of second (" +
            std::to_string(other.rows_) + ")"
        );
    }
}

// Scalar multiplication (scalar * matrix)
Matrix operator*(double scalar, const Matrix& mat) {
    return mat * scalar;
}

}  // namespace core
}  // namespace achronyme
