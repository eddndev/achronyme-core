#pragma once

#include <vector>
#include <string>
#include <stdexcept>

namespace achronyme {
namespace core {

/**
 * Mathematical matrix representation (row-major order)
 *
 * Supports matrix arithmetic, transpose, determinant, and inverse.
 */
class Matrix {
public:
    // Constructors
    Matrix() : rows_(0), cols_(0) {}
    Matrix(size_t rows, size_t cols, std::vector<double> data);
    Matrix(size_t rows, size_t cols, double value = 0.0);

    // Accessors
    size_t rows() const { return rows_; }
    size_t cols() const { return cols_; }
    size_t size() const { return rows_ * cols_; }

    double at(size_t row, size_t col) const;
    double& at(size_t row, size_t col);

    const std::vector<double>& data() const { return data_; }

    // Matrix operations
    Matrix transpose() const;
    double determinant() const;
    Matrix inverse() const;
    double trace() const;  // Sum of diagonal elements

    // Arithmetic operators
    Matrix operator+(const Matrix& other) const;
    Matrix operator-(const Matrix& other) const;
    Matrix operator*(const Matrix& other) const;  // Matrix multiplication
    Matrix operator*(double scalar) const;         // Scalar multiplication
    Matrix operator/(double scalar) const;
    Matrix operator-() const;  // Unary minus

    // Comparison
    bool operator==(const Matrix& other) const;
    bool operator!=(const Matrix& other) const;

    // String representation
    std::string toString() const;

    // Static factory methods
    static Matrix zeros(size_t rows, size_t cols);
    static Matrix ones(size_t rows, size_t cols);
    static Matrix identity(size_t size);

    // Matrix properties
    bool isSquare() const { return rows_ == cols_; }
    bool isEmpty() const { return rows_ == 0 || cols_ == 0; }

private:
    size_t rows_;
    size_t cols_;
    std::vector<double> data_;  // Row-major order

    size_t index(size_t row, size_t col) const {
        return row * cols_ + col;
    }

    void checkSameSize(const Matrix& other) const;
    void checkMultipliable(const Matrix& other) const;

    // Helper for determinant calculation
    double determinant2x2() const;
    double determinant3x3() const;
    double determinantNxN() const;  // General case (Gaussian elimination)
};

// Scalar multiplication (scalar * matrix)
Matrix operator*(double scalar, const Matrix& mat);

}  // namespace core
}  // namespace achronyme
