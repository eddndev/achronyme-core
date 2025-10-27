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

// ============================================================================
// Phase 4B: Convolution and Window Functions
// ============================================================================

/**
 * conv(signal1, signal2) - Linear Convolution (Direct Method)
 *
 * Computes the discrete convolution: y[n] = Σ x[k] * h[n-k]
 * Output length: N + M - 1 (where N = len(signal1), M = len(signal2))
 * Complexity: O(N*M)
 *
 * Use cases:
 * - FIR filtering
 * - System response
 * - Cross-correlation
 *
 * Example:
 *   conv([1,2,3], [1,1]) → [1, 3, 5, 3]
 */
Value convFunction(const std::vector<Value>& args) {
    if (args.size() != 2) {
        throw std::runtime_error("conv requires 2 arguments: signal1, signal2");
    }

    if (!args[0].isVector() || !args[1].isVector()) {
        throw std::runtime_error("conv arguments must be vectors");
    }

    Vector signal1 = args[0].asVector();
    Vector signal2 = args[1].asVector();

    size_t N = signal1.size();
    size_t M = signal2.size();

    if (N == 0 || M == 0) {
        throw std::runtime_error("conv requires non-empty vectors");
    }

    // Output length: N + M - 1
    size_t outputLen = N + M - 1;
    std::vector<double> result(outputLen, 0.0);

    // Compute convolution: y[n] = sum over k of x[k] * h[n-k]
    for (size_t n = 0; n < outputLen; ++n) {
        for (size_t k = 0; k < N; ++k) {
            // Check if index is valid: n-k must be in range [0, M-1]
            if (n >= k && (n - k) < M) {
                result[n] += signal1[k] * signal2[n - k];
            }
        }
    }

    return Value(Vector(result));
}

/**
 * conv_fft(signal1, signal2) - Fast Convolution using FFT
 *
 * Computes convolution using the convolution theorem:
 *   conv(x, h) = IFFT(FFT(x) * FFT(h))
 *
 * Complexity: O((N+M) log(N+M)) vs O(N*M) for direct method
 * Much faster for large signals!
 *
 * Example:
 *   conv_fft([1,2,3,4,5], [1,1,1]) → same as conv but faster
 */
Value convFFTFunction(const std::vector<Value>& args) {
    if (args.size() != 2) {
        throw std::runtime_error("conv_fft requires 2 arguments: signal1, signal2");
    }

    if (!args[0].isVector() || !args[1].isVector()) {
        throw std::runtime_error("conv_fft arguments must be vectors");
    }

    Vector signal1 = args[0].asVector();
    Vector signal2 = args[1].asVector();

    size_t N = signal1.size();
    size_t M = signal2.size();

    if (N == 0 || M == 0) {
        throw std::runtime_error("conv_fft requires non-empty vectors");
    }

    // Output length for linear convolution
    size_t outputLen = N + M - 1;

    // FFT size: next power of 2 >= outputLen
    size_t fftSize = nextPowerOfTwo(outputLen);

    // Zero-pad both signals to FFT size
    std::vector<std::complex<double>> x1(fftSize, 0.0);
    std::vector<std::complex<double>> x2(fftSize, 0.0);

    for (size_t i = 0; i < N; ++i) {
        x1[i] = std::complex<double>(signal1[i], 0.0);
    }
    for (size_t i = 0; i < M; ++i) {
        x2[i] = std::complex<double>(signal2[i], 0.0);
    }

    // Compute FFT of both signals
    auto fft1 = fft_recursive(x1);
    auto fft2 = fft_recursive(x2);

    // Multiply in frequency domain (element-wise)
    std::vector<std::complex<double>> product(fftSize);
    for (size_t i = 0; i < fftSize; ++i) {
        product[i] = fft1[i] * fft2[i];
    }

    // Compute IFFT using conjugate trick
    for (size_t i = 0; i < fftSize; ++i) {
        product[i] = std::conj(product[i]);
    }
    auto ifft_result = fft_recursive(product);

    // Extract real part, conjugate, and scale
    std::vector<double> result;
    result.reserve(outputLen);

    for (size_t i = 0; i < outputLen; ++i) {
        result.push_back(ifft_result[i].real() / fftSize);
    }

    return Value(Vector(result));
}

/**
 * hanning(N) - Hanning (Hann) Window
 *
 * Formula: w[n] = 0.5 * (1 - cos(2πn/(N-1)))
 *
 * Properties:
 * - Smooth transitions at edges (goes to 0)
 * - Good frequency resolution
 * - Moderate sidelobe suppression (-31 dB)
 *
 * Use case: General purpose spectral analysis
 *
 * Example:
 *   hanning(8) → [0, 0.188, 0.611, 0.950, 0.950, 0.611, 0.188, 0]
 */
Value hanningFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("hanning requires 1 argument: window size N");
    }

    if (!args[0].isNumber()) {
        throw std::runtime_error("hanning argument must be a number");
    }

    int N = static_cast<int>(args[0].asNumber());

    if (N <= 0) {
        throw std::runtime_error("hanning requires positive window size");
    }

    std::vector<double> window;
    window.reserve(N);

    const double PI = 3.141592653589793;

    if (N == 1) {
        window.push_back(1.0);
    } else {
        for (int n = 0; n < N; ++n) {
            // w[n] = 0.5 * (1 - cos(2πn/(N-1)))
            double value = 0.5 * (1.0 - std::cos(2.0 * PI * n / (N - 1)));
            window.push_back(value);
        }
    }

    return Value(Vector(window));
}

/**
 * hamming(N) - Hamming Window
 *
 * Formula: w[n] = 0.54 - 0.46 * cos(2πn/(N-1))
 *
 * Properties:
 * - Similar to Hanning but doesn't go to zero at edges
 * - Better sidelobe suppression than Hanning (-43 dB)
 * - Slightly worse frequency resolution
 *
 * Use case: When sidelobe suppression is important
 *
 * Example:
 *   hamming(8) → [0.08, 0.253, 0.642, 0.954, 0.954, 0.642, 0.253, 0.08]
 */
Value hammingFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("hamming requires 1 argument: window size N");
    }

    if (!args[0].isNumber()) {
        throw std::runtime_error("hamming argument must be a number");
    }

    int N = static_cast<int>(args[0].asNumber());

    if (N <= 0) {
        throw std::runtime_error("hamming requires positive window size");
    }

    std::vector<double> window;
    window.reserve(N);

    const double PI = 3.141592653589793;
    const double alpha = 0.54;
    const double beta = 0.46;

    if (N == 1) {
        window.push_back(1.0);
    } else {
        for (int n = 0; n < N; ++n) {
            // w[n] = α - β * cos(2πn/(N-1))
            double value = alpha - beta * std::cos(2.0 * PI * n / (N - 1));
            window.push_back(value);
        }
    }

    return Value(Vector(window));
}

/**
 * blackman(N) - Blackman Window
 *
 * Formula: w[n] = 0.42 - 0.5*cos(2πn/(N-1)) + 0.08*cos(4πn/(N-1))
 *
 * Properties:
 * - Excellent sidelobe suppression (-58 dB)
 * - Wider main lobe (worse frequency resolution)
 * - Best for reducing spectral leakage
 *
 * Use case: When maximum sidelobe suppression is needed
 *
 * Example:
 *   blackman(8) → [0, 0.091, 0.459, 0.920, 0.920, 0.459, 0.091, 0]
 */
Value blackmanFunction(const std::vector<Value>& args) {
    if (args.size() != 1) {
        throw std::runtime_error("blackman requires 1 argument: window size N");
    }

    if (!args[0].isNumber()) {
        throw std::runtime_error("blackman argument must be a number");
    }

    int N = static_cast<int>(args[0].asNumber());

    if (N <= 0) {
        throw std::runtime_error("blackman requires positive window size");
    }

    std::vector<double> window;
    window.reserve(N);

    const double PI = 3.141592653589793;
    const double a0 = 0.42;
    const double a1 = 0.5;
    const double a2 = 0.08;

    if (N == 1) {
        window.push_back(1.0);
    } else {
        for (int n = 0; n < N; ++n) {
            // w[n] = a0 - a1*cos(2πn/(N-1)) + a2*cos(4πn/(N-1))
            double angle = 2.0 * PI * n / (N - 1);
            double value = a0 - a1 * std::cos(angle) + a2 * std::cos(2.0 * angle);
            window.push_back(value);
        }
    }

    return Value(Vector(window));
}

} // namespace core
} // namespace achronyme
