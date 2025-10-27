#include "functions.hpp"
#include "value.hpp"
#include <cmath>
#include <stdexcept>
#include <algorithm>
#include <complex>

namespace achronyme {
namespace core {

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Check if n is a power of 2
 */
static bool isPowerOfTwo(size_t n) {
    return n > 0 && (n & (n - 1)) == 0;
}

/**
 * Find next power of 2 >= n
 */
static size_t nextPowerOfTwo(size_t n) {
    if (n == 0) return 1;
    size_t power = 1;
    while (power < n) {
        power *= 2;
    }
    return power;
}

/**
 * Recursive FFT implementation (Cooley-Tukey algorithm)
 * Input: complex vector of size 2^k
 * Returns: FFT of input
 */
static std::vector<std::complex<double>> fft_recursive(const std::vector<std::complex<double>>& x) {
    size_t N = x.size();

    // Base case
    if (N == 1) {
        return x;
    }

    // Divide: separate even and odd indices
    std::vector<std::complex<double>> even(N/2);
    std::vector<std::complex<double>> odd(N/2);

    for (size_t i = 0; i < N/2; ++i) {
        even[i] = x[2*i];
        odd[i] = x[2*i + 1];
    }

    // Conquer: recursively compute FFT of even and odd parts
    auto fft_even = fft_recursive(even);
    auto fft_odd = fft_recursive(odd);

    // Combine: use twiddle factors
    std::vector<std::complex<double>> result(N);
    const double PI = 3.141592653589793;

    for (size_t k = 0; k < N/2; ++k) {
        // Twiddle factor: e^(-2πik/N)
        double angle = -2.0 * PI * k / N;
        std::complex<double> twiddle(std::cos(angle), std::sin(angle));
        std::complex<double> t = twiddle * fft_odd[k];

        result[k] = fft_even[k] + t;
        result[k + N/2] = fft_even[k] - t;
    }

    return result;
}

/**
 * dft(signal) - Discrete Fourier Transform
 *
 * Returns a matrix [N x 2] where each row contains [real, imaginary]
 * Formula: X[k] = Σ(n=0 to N-1) x[n] * e^(-2πikn/N)
 * Complexity: O(N²)
 */
Value dftFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("dft requires 1 argument: signal vector");
    }

    if (!args[0].isVector()) {
        throw std::runtime_error("dft argument must be a vector");
    }

    Vector signal = args[0].asVector();
    size_t N = signal.size();

    if (N == 0) {
        throw std::runtime_error("dft requires non-empty vector");
    }

    // Compute DFT: X[k] = sum over n of x[n] * exp(-2πikn/N)
    std::vector<double> matrixData;
    matrixData.reserve(N * 2);

    const double TWO_PI = 2.0 * 3.141592653589793;

    for (size_t k = 0; k < N; ++k) {
        double sumReal = 0.0;
        double sumImag = 0.0;

        for (size_t n = 0; n < N; ++n) {
            double angle = -TWO_PI * k * n / N;
            double cosAngle = std::cos(angle);
            double sinAngle = std::sin(angle);

            // x[n] * exp(-2πikn/N) = x[n] * (cos(angle) + i*sin(angle))
            sumReal += signal[n] * cosAngle;
            sumImag += signal[n] * sinAngle;
        }

        matrixData.push_back(sumReal);
        matrixData.push_back(sumImag);
    }

    // Return as Matrix [N x 2]
    return Value(Matrix(N, 2, matrixData));
}

/**
 * dft_mag(signal) - DFT magnitude spectrum
 *
 * Returns vector of magnitudes: |X[k]|
 */
Value dftMagFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("dft_mag requires 1 argument: signal vector");
    }

    if (!args[0].isVector()) {
        throw std::runtime_error("dft_mag argument must be a vector");
    }

    Vector signal = args[0].asVector();
    size_t N = signal.size();

    if (N == 0) {
        throw std::runtime_error("dft_mag requires non-empty vector");
    }

    std::vector<double> magnitudes;
    magnitudes.reserve(N);

    const double TWO_PI = 2.0 * 3.141592653589793;

    for (size_t k = 0; k < N; ++k) {
        double sumReal = 0.0;
        double sumImag = 0.0;

        for (size_t n = 0; n < N; ++n) {
            double angle = -TWO_PI * k * n / N;
            double cosAngle = std::cos(angle);
            double sinAngle = std::sin(angle);

            sumReal += signal[n] * cosAngle;
            sumImag += signal[n] * sinAngle;
        }

        double magnitude = std::sqrt(sumReal * sumReal + sumImag * sumImag);
        magnitudes.push_back(magnitude);
    }

    return Value(Vector(magnitudes));
}

/**
 * dft_phase(signal) - DFT phase spectrum
 *
 * Returns vector of phases: arg(X[k])
 */
Value dftPhaseFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("dft_phase requires 1 argument: signal vector");
    }

    if (!args[0].isVector()) {
        throw std::runtime_error("dft_phase argument must be a vector");
    }

    Vector signal = args[0].asVector();
    size_t N = signal.size();

    if (N == 0) {
        throw std::runtime_error("dft_phase requires non-empty vector");
    }

    std::vector<double> phases;
    phases.reserve(N);

    const double TWO_PI = 2.0 * 3.141592653589793;

    for (size_t k = 0; k < N; ++k) {
        double sumReal = 0.0;
        double sumImag = 0.0;

        for (size_t n = 0; n < N; ++n) {
            double angle = -TWO_PI * k * n / N;
            double cosAngle = std::cos(angle);
            double sinAngle = std::sin(angle);

            sumReal += signal[n] * cosAngle;
            sumImag += signal[n] * sinAngle;
        }

        double phase = std::atan2(sumImag, sumReal);
        phases.push_back(phase);
    }

    return Value(Vector(phases));
}

/**
 * fft(signal) - Fast Fourier Transform (Cooley-Tukey)
 *
 * Returns a matrix [N x 2] where each row contains [real, imaginary]
 * Automatically zero-pads to next power of 2 if needed.
 * Complexity: O(N log N)
 */
Value fftFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("fft requires 1 argument: signal vector");
    }

    if (!args[0].isVector()) {
        throw std::runtime_error("fft argument must be a vector");
    }

    Vector signal = args[0].asVector();
    size_t originalSize = signal.size();

    if (originalSize == 0) {
        throw std::runtime_error("fft requires non-empty vector");
    }

    // Zero-pad to next power of 2
    size_t N = nextPowerOfTwo(originalSize);
    std::vector<std::complex<double>> x(N);

    for (size_t i = 0; i < originalSize; ++i) {
        x[i] = std::complex<double>(signal[i], 0.0);
    }
    for (size_t i = originalSize; i < N; ++i) {
        x[i] = std::complex<double>(0.0, 0.0);
    }

    // Compute FFT
    auto result = fft_recursive(x);

    // Convert to Matrix [N x 2]
    std::vector<double> matrixData;
    matrixData.reserve(N * 2);

    for (size_t i = 0; i < N; ++i) {
        matrixData.push_back(result[i].real());
        matrixData.push_back(result[i].imag());
    }

    return Value(Matrix(N, 2, matrixData));
}

/**
 * fft_mag(signal) - FFT magnitude spectrum
 *
 * Returns vector of magnitudes: |X[k]|
 */
Value fftMagFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("fft_mag requires 1 argument: signal vector");
    }

    if (!args[0].isVector()) {
        throw std::runtime_error("fft_mag argument must be a vector");
    }

    Vector signal = args[0].asVector();
    size_t originalSize = signal.size();

    if (originalSize == 0) {
        throw std::runtime_error("fft_mag requires non-empty vector");
    }

    // Zero-pad to next power of 2
    size_t N = nextPowerOfTwo(originalSize);
    std::vector<std::complex<double>> x(N);

    for (size_t i = 0; i < originalSize; ++i) {
        x[i] = std::complex<double>(signal[i], 0.0);
    }
    for (size_t i = originalSize; i < N; ++i) {
        x[i] = std::complex<double>(0.0, 0.0);
    }

    // Compute FFT
    auto result = fft_recursive(x);

    // Convert to magnitude vector
    std::vector<double> magnitudes;
    magnitudes.reserve(N);

    for (size_t i = 0; i < N; ++i) {
        magnitudes.push_back(std::abs(result[i]));
    }

    return Value(Vector(magnitudes));
}

/**
 * ifft(spectrum) - Inverse Fast Fourier Transform
 *
 * Takes a matrix [N x 2] with [real, imaginary] and returns time-domain signal.
 * Formula: IFFT(X) = conj(FFT(conj(X))) / N
 * Complexity: O(N log N)
 */
Value ifftFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("ifft requires 1 argument: spectrum matrix [N x 2]");
    }

    if (!args[0].isMatrix()) {
        throw std::runtime_error("ifft argument must be a matrix [N x 2]");
    }

    Matrix spectrum = args[0].asMatrix();
    size_t N = spectrum.rows();

    if (spectrum.cols() != 2) {
        throw std::runtime_error("ifft requires matrix with 2 columns [real, imag]");
    }

    if (!isPowerOfTwo(N)) {
        throw std::runtime_error("ifft requires matrix with power-of-2 rows (use zero-padding)");
    }

    // Convert matrix to complex vector and conjugate
    std::vector<std::complex<double>> x(N);
    for (size_t i = 0; i < N; ++i) {
        double real = spectrum.at(i, 0);
        double imag = spectrum.at(i, 1);
        x[i] = std::complex<double>(real, -imag);  // Conjugate
    }

    // Apply FFT to conjugated input
    auto result = fft_recursive(x);

    // Conjugate and scale by 1/N
    std::vector<double> timeSignal;
    timeSignal.reserve(N);

    for (size_t i = 0; i < N; ++i) {
        timeSignal.push_back(result[i].real() / N);  // Take real part and scale
    }

    return Value(Vector(timeSignal));
}

} // namespace core
} // namespace achronyme
