#pragma once

#include <vector>
#include <string>
#include <stdexcept>

namespace achronyme {
namespace core {

/**
 * Mathematical vector representation
 *
 * Supports vector arithmetic, dot product, cross product, and norms.
 */
class Vector {
public:
    // Constructors
    Vector() = default;
    explicit Vector(std::vector<double> elements);
    explicit Vector(size_t size, double value = 0.0);

    // Accessors
    size_t size() const { return elements_.size(); }
    double operator[](size_t index) const;
    double& operator[](size_t index);

    const std::vector<double>& elements() const { return elements_; }

    // Vector operations
    double norm() const;           // ||v|| = sqrt(sum(xi²))
    double normSquared() const;    // sum(xi²)
    Vector normalize() const;      // v / ||v||
    double dot(const Vector& other) const;
    Vector cross(const Vector& other) const;  // 3D only

    // Arithmetic operators
    Vector operator+(const Vector& other) const;
    Vector operator-(const Vector& other) const;
    Vector operator*(double scalar) const;
    Vector operator/(double scalar) const;
    Vector operator-() const;  // Unary minus

    // Comparison
    bool operator==(const Vector& other) const;
    bool operator!=(const Vector& other) const;

    // String representation
    std::string toString() const;

    // Static factory methods
    static Vector zeros(size_t size);
    static Vector ones(size_t size);
    static Vector unit(size_t size, size_t index);  // Unit vector (0...1...0)

private:
    std::vector<double> elements_;

    void checkSameSize(const Vector& other) const;
};

// Scalar multiplication (scalar * vector)
Vector operator*(double scalar, const Vector& vec);

}  // namespace core
}  // namespace achronyme
