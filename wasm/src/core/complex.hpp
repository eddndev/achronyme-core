#pragma once

#include <cmath>
#include <string>

namespace achronyme {
namespace core {

/**
 * Complex number representation: a + bi
 *
 * Supports full arithmetic operations and common complex functions.
 */
class Complex {
public:
    // Constructors
    Complex() : real_(0.0), imag_(0.0) {}
    Complex(double real, double imag) : real_(real), imag_(imag) {}
    explicit Complex(double real) : real_(real), imag_(0.0) {}

    // Accessors
    double real() const { return real_; }
    double imag() const { return imag_; }

    // Complex-specific operations
    double magnitude() const;  // |z| = sqrt(a² + b²)
    double argument() const;    // arg(z) = atan2(b, a)
    Complex conjugate() const;  // conj(z) = a - bi

    // Arithmetic operators
    Complex operator+(const Complex& other) const;
    Complex operator-(const Complex& other) const;
    Complex operator*(const Complex& other) const;
    Complex operator/(const Complex& other) const;
    Complex operator-() const;  // Unary minus

    // Power operation
    Complex pow(const Complex& exponent) const;
    Complex pow(double exponent) const;

    // Comparison (magnitude comparison)
    bool operator==(const Complex& other) const;
    bool operator!=(const Complex& other) const;

    // String representation
    std::string toString() const;

    // Static factory methods
    static Complex fromPolar(double magnitude, double argument);
    static Complex I() { return Complex(0.0, 1.0); }  // Imaginary unit

private:
    double real_;
    double imag_;
};

// Mathematical functions for Complex
namespace complex_math {
    Complex sqrt(const Complex& z);
    Complex exp(const Complex& z);
    Complex log(const Complex& z);
    Complex sin(const Complex& z);
    Complex cos(const Complex& z);
    Complex tan(const Complex& z);
}

}  // namespace core
}  // namespace achronyme
