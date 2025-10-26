#include "vector.hpp"
#include <cmath>
#include <sstream>
#include <iomanip>
#include <algorithm>

namespace achronyme {
namespace core {

Vector::Vector(std::vector<double> elements) : elements_(std::move(elements)) {}

Vector::Vector(size_t size, double value) : elements_(size, value) {}

double Vector::operator[](size_t index) const {
    if (index >= elements_.size()) {
        throw std::out_of_range("Vector index out of range");
    }
    return elements_[index];
}

double& Vector::operator[](size_t index) {
    if (index >= elements_.size()) {
        throw std::out_of_range("Vector index out of range");
    }
    return elements_[index];
}

// Euclidean norm: ||v|| = sqrt(x₁² + x₂² + ... + xₙ²)
double Vector::norm() const {
    return std::sqrt(normSquared());
}

double Vector::normSquared() const {
    double sum = 0.0;
    for (double element : elements_) {
        sum += element * element;
    }
    return sum;
}

// Normalize: v / ||v||
Vector Vector::normalize() const {
    double n = norm();
    if (n == 0.0) {
        throw std::runtime_error("Cannot normalize zero vector");
    }
    return (*this) / n;
}

// Dot product: v · w = v₁w₁ + v₂w₂ + ... + vₙwₙ
double Vector::dot(const Vector& other) const {
    checkSameSize(other);

    double result = 0.0;
    for (size_t i = 0; i < elements_.size(); ++i) {
        result += elements_[i] * other.elements_[i];
    }
    return result;
}

// Cross product (3D only): v × w
Vector Vector::cross(const Vector& other) const {
    if (elements_.size() != 3 || other.elements_.size() != 3) {
        throw std::runtime_error("Cross product is only defined for 3D vectors");
    }

    return Vector({
        elements_[1] * other.elements_[2] - elements_[2] * other.elements_[1],
        elements_[2] * other.elements_[0] - elements_[0] * other.elements_[2],
        elements_[0] * other.elements_[1] - elements_[1] * other.elements_[0]
    });
}

// Addition: v + w
Vector Vector::operator+(const Vector& other) const {
    checkSameSize(other);

    std::vector<double> result(elements_.size());
    for (size_t i = 0; i < elements_.size(); ++i) {
        result[i] = elements_[i] + other.elements_[i];
    }
    return Vector(result);
}

// Subtraction: v - w
Vector Vector::operator-(const Vector& other) const {
    checkSameSize(other);

    std::vector<double> result(elements_.size());
    for (size_t i = 0; i < elements_.size(); ++i) {
        result[i] = elements_[i] - other.elements_[i];
    }
    return Vector(result);
}

// Scalar multiplication: v * scalar
Vector Vector::operator*(double scalar) const {
    std::vector<double> result(elements_.size());
    for (size_t i = 0; i < elements_.size(); ++i) {
        result[i] = elements_[i] * scalar;
    }
    return Vector(result);
}

// Scalar division: v / scalar
Vector Vector::operator/(double scalar) const {
    if (scalar == 0.0) {
        throw std::runtime_error("Division by zero in vector division");
    }
    return (*this) * (1.0 / scalar);
}

// Unary minus: -v
Vector Vector::operator-() const {
    return (*this) * (-1.0);
}

// Comparison
bool Vector::operator==(const Vector& other) const {
    if (elements_.size() != other.elements_.size()) {
        return false;
    }

    const double epsilon = 1e-10;
    for (size_t i = 0; i < elements_.size(); ++i) {
        if (std::abs(elements_[i] - other.elements_[i]) >= epsilon) {
            return false;
        }
    }
    return true;
}

bool Vector::operator!=(const Vector& other) const {
    return !(*this == other);
}

// String representation: [x, y, z]
std::string Vector::toString() const {
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(6);
    oss << "[";

    for (size_t i = 0; i < elements_.size(); ++i) {
        if (i > 0) oss << ", ";
        oss << elements_[i];
    }

    oss << "]";
    return oss.str();
}

// Static factory methods
Vector Vector::zeros(size_t size) {
    return Vector(size, 0.0);
}

Vector Vector::ones(size_t size) {
    return Vector(size, 1.0);
}

Vector Vector::unit(size_t size, size_t index) {
    if (index >= size) {
        throw std::out_of_range("Unit vector index out of range");
    }
    std::vector<double> elements(size, 0.0);
    elements[index] = 1.0;
    return Vector(elements);
}

// Helper: Check that two vectors have the same size
void Vector::checkSameSize(const Vector& other) const {
    if (elements_.size() != other.elements_.size()) {
        throw std::runtime_error(
            "Vector dimension mismatch: " +
            std::to_string(elements_.size()) + " vs " +
            std::to_string(other.elements_.size())
        );
    }
}

// Scalar multiplication (scalar * vector)
Vector operator*(double scalar, const Vector& vec) {
    return vec * scalar;
}

}  // namespace core
}  // namespace achronyme
