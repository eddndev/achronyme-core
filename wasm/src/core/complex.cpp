#include "complex.hpp"
#include <sstream>
#include <iomanip>
#include <stdexcept>

namespace achronyme {
namespace core {

// Magnitude: |z| = sqrt(a² + b²)
double Complex::magnitude() const {
    return std::sqrt(real_ * real_ + imag_ * imag_);
}

// Argument: arg(z) = atan2(b, a)
double Complex::argument() const {
    return std::atan2(imag_, real_);
}

// Conjugate: conj(a + bi) = a - bi
Complex Complex::conjugate() const {
    return Complex(real_, -imag_);
}

// Addition: (a + bi) + (c + di) = (a + c) + (b + d)i
Complex Complex::operator+(const Complex& other) const {
    return Complex(real_ + other.real_, imag_ + other.imag_);
}

// Subtraction: (a + bi) - (c + di) = (a - c) + (b - d)i
Complex Complex::operator-(const Complex& other) const {
    return Complex(real_ - other.real_, imag_ - other.imag_);
}

// Multiplication: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
Complex Complex::operator*(const Complex& other) const {
    double real_part = real_ * other.real_ - imag_ * other.imag_;
    double imag_part = real_ * other.imag_ + imag_ * other.real_;
    return Complex(real_part, imag_part);
}

// Division: (a + bi) / (c + di) = [(ac + bd) + (bc - ad)i] / (c² + d²)
Complex Complex::operator/(const Complex& other) const {
    double denominator = other.real_ * other.real_ + other.imag_ * other.imag_;

    if (denominator == 0.0) {
        throw std::runtime_error("Division by zero in complex division");
    }

    double real_part = (real_ * other.real_ + imag_ * other.imag_) / denominator;
    double imag_part = (imag_ * other.real_ - real_ * other.imag_) / denominator;

    return Complex(real_part, imag_part);
}

// Unary minus: -(a + bi) = -a - bi
Complex Complex::operator-() const {
    return Complex(-real_, -imag_);
}

// Power: z^w using exponential form
// z^w = exp(w * log(z))
Complex Complex::pow(const Complex& exponent) const {
    // Handle special case: 0^0 = 1
    if (magnitude() == 0.0 && exponent.magnitude() == 0.0) {
        return Complex(1.0, 0.0);
    }

    // 0^w = 0 for w != 0
    if (magnitude() == 0.0) {
        return Complex(0.0, 0.0);
    }

    // z^w = exp(w * log(z))
    return complex_math::exp(exponent * complex_math::log(*this));
}

Complex Complex::pow(double exponent) const {
    return pow(Complex(exponent, 0.0));
}

// Comparison (by magnitude and then by argument)
bool Complex::operator==(const Complex& other) const {
    const double epsilon = 1e-10;
    return std::abs(real_ - other.real_) < epsilon &&
           std::abs(imag_ - other.imag_) < epsilon;
}

bool Complex::operator!=(const Complex& other) const {
    return !(*this == other);
}

// String representation
std::string Complex::toString() const {
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(6);

    if (imag_ >= 0) {
        oss << real_ << " + " << imag_ << "i";
    } else {
        oss << real_ << " - " << (-imag_) << "i";
    }

    return oss.str();
}

// Create from polar coordinates: z = r * e^(iθ) = r(cos θ + i sin θ)
Complex Complex::fromPolar(double magnitude, double argument) {
    return Complex(
        magnitude * std::cos(argument),
        magnitude * std::sin(argument)
    );
}

// ============================================================================
// Complex Mathematical Functions
// ============================================================================

namespace complex_math {

// Square root: sqrt(z) = sqrt(r) * e^(i*θ/2)
Complex sqrt(const Complex& z) {
    double r = z.magnitude();
    double theta = z.argument();
    return Complex::fromPolar(std::sqrt(r), theta / 2.0);
}

// Exponential: exp(a + bi) = e^a * (cos b + i sin b)
Complex exp(const Complex& z) {
    double exp_real = std::exp(z.real());
    return Complex(
        exp_real * std::cos(z.imag()),
        exp_real * std::sin(z.imag())
    );
}

// Natural logarithm: log(z) = log|z| + i*arg(z)
Complex log(const Complex& z) {
    if (z.magnitude() == 0.0) {
        throw std::runtime_error("Logarithm of zero is undefined");
    }
    return Complex(std::log(z.magnitude()), z.argument());
}

// Sine: sin(z) = (e^(iz) - e^(-iz)) / (2i)
Complex sin(const Complex& z) {
    Complex iz = Complex(-z.imag(), z.real());  // i * z
    Complex exp_iz = exp(iz);
    Complex exp_neg_iz = exp(-iz);

    Complex result = (exp_iz - exp_neg_iz) * Complex(0.0, -0.5);  // Divide by 2i
    return result;
}

// Cosine: cos(z) = (e^(iz) + e^(-iz)) / 2
Complex cos(const Complex& z) {
    Complex iz = Complex(-z.imag(), z.real());  // i * z
    Complex exp_iz = exp(iz);
    Complex exp_neg_iz = exp(-iz);

    return (exp_iz + exp_neg_iz) * Complex(0.5, 0.0);
}

// Tangent: tan(z) = sin(z) / cos(z)
Complex tan(const Complex& z) {
    return sin(z) / cos(z);
}

}  // namespace complex_math

}  // namespace core
}  // namespace achronyme
