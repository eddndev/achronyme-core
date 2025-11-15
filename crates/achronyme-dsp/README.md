# achronyme-dsp

Digital Signal Processing (DSP) module for Achronyme.

## 🎯 Responsibility

The `achronyme-dsp` crate provides comprehensive Digital Signal Processing functionality for the Achronyme language, including:

- **Fast Fourier Transform (FFT)**: Efficient frequency-domain analysis using the Cooley-Tukey algorithm
- **Inverse FFT**: Time-domain reconstruction from frequency spectra
- **Convolution**: Both time-domain (direct) and frequency-domain (FFT-based) implementations
- **Window Functions**: Hanning, Hamming, Blackman, and Rectangular windows for spectral analysis

This crate serves as the DSP backend for Achronyme's signal processing capabilities, exposing all operations to the SOC language through the evaluator.

## 📦 Dependencies

### External Crates
- **rustfft** (v6.1): High-performance FFT library using the Cooley-Tukey algorithm
- **num-complex** (v0.4): Complex number arithmetic
- **approx** (v0.5, dev): Floating-point comparison for tests

### Internal Crates
- **achronyme-types**: Provides `Complex`, `RealTensor`, and `ComplexTensor` types

## 🔌 Used By

- **achronyme-eval**: Exposes DSP functions to the SOC language (`fft()`, `ifft()`, `conv()`, etc.)

## 🏗️ High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   achronyme-dsp                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   FFT    │  │  Convolution │  │   Windows    │     │
│  │  Module  │  │    Module    │  │    Module    │     │
│  └─────┬────┘  └──────┬───────┘  └──────┬───────┘     │
│        │              │                  │             │
│        │              │                  │             │
│  ┌─────▼──────────────▼──────────────────▼───────┐     │
│  │          achronyme-types (Complex)            │     │
│  └───────────────────────────────────────────────┘     │
│                                                         │
│  ┌───────────────────────────────────────────────┐     │
│  │          rustfft (FFT Engine)                 │     │
│  └───────────────────────────────────────────────┘     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Module Organization

1. **fft.rs**: FFT/IFFT transformations with real and complex signal support
2. **convolution.rs**: Direct and FFT-based convolution algorithms
3. **windows.rs**: Window function generation and application
4. **lib.rs**: Public API exports and module declarations

## 🚀 Usage Examples

### SOC Language Examples

#### Basic FFT Analysis
```soc
// Analyze frequency content of a signal
let signal = [1, 2, 3, 4, 3, 2, 1, 0]
let spectrum = fft(signal)          // Returns ComplexTensor
let magnitudes = fft_mag(signal)    // Returns magnitude spectrum
let phases = fft_phase(signal)      // Returns phase spectrum
```

#### Complete DSP Pipeline
```soc
// Full signal processing workflow
let num_samples = 1024
let sample_rate = 1024
let freq = 50

// Generate signal
let t = linspace(0, 1, num_samples)
let signal = map(x => sin(2 * PI * freq * x), t)

// Apply windowing
let windowed_signal = signal * hanning(num_samples)

// Analyze spectrum
let spectrum = fft_mag(windowed_signal)
```

#### Convolution for Filtering
```soc
// Apply a moving average filter
let signal = [1, 2, 3, 4, 5, 6, 7, 8]
let kernel = [0.25, 0.5, 0.25]  // Gaussian-like kernel

let filtered = conv(signal, kernel)
// Or use FFT-based for large signals:
let filtered_fast = conv_fft(signal, kernel)
```

#### Window Functions
```soc
// Compare different window functions
let n = 128
let hann = hanning(n)
let hamm = hamming(n)
let black = blackman(n)
let rect = rectangular(n)

// Apply window to signal
let signal = linspace(0, 10, n)
let windowed = signal * hann
```

### Rust API Examples

#### FFT Transform
```rust
use achronyme_dsp::{fft_transform, ifft_transform};
use achronyme_types::complex::Complex;

// Complex signal FFT
let signal = vec![
    Complex::new(1.0, 0.0),
    Complex::new(2.0, 0.0),
    Complex::new(3.0, 0.0),
    Complex::new(4.0, 0.0),
];

let spectrum = fft_transform(&signal);
let reconstructed = ifft_transform(&spectrum);

// Real signal FFT (convenience function)
let real_signal = vec![1.0, 2.0, 3.0, 4.0];
let spectrum = fft_real(&real_signal);
```

#### Convolution
```rust
use achronyme_dsp::{convolve, convolve_fft};

let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let kernel = vec![0.25, 0.5, 0.25];

// Direct convolution - O(n*m)
let result_direct = convolve(&signal, &kernel);

// FFT-based convolution - O(n log n)
let result_fft = convolve_fft(&signal, &kernel);
```

#### Window Functions
```rust
use achronyme_dsp::{hanning_window, apply_window};

let window = hanning_window(256);
let signal = vec![1.0; 256];
let windowed = apply_window(&signal, &window).unwrap();
```

## 📊 Key Algorithms

### Fast Fourier Transform (FFT)

The FFT computes the Discrete Fourier Transform efficiently using the Cooley-Tukey algorithm.

**Mathematical Definition (DFT):**
```
X[k] = Σ(n=0 to N-1) x[n] · e^(-j2πkn/N)
```

**Complexity:**
- DFT (naive): O(N²)
- FFT (Cooley-Tukey): O(N log N)

**Implementation:**
- Uses `rustfft` library for optimal performance
- Supports both complex and real-valued signals
- Automatic scaling (1/N) in IFFT

### Convolution

#### Direct Convolution
**Formula:**
```
(f * g)[n] = Σ(m=0 to M-1) f[m] · g[n - m]
```

**Complexity:** O(N · M) where N and M are signal lengths

**Use when:**
- Small kernel size (< 100 elements)
- Memory is limited
- Exact results needed

#### FFT-Based Convolution

Uses the **Convolution Theorem**:
```
conv(f, g) = IFFT(FFT(f) · FFT(g))
```

**Complexity:** O((N+M) log(N+M))

**Use when:**
- Large kernel size (> 100 elements)
- Processing long signals
- Speed is critical

### Window Functions

Window functions reduce spectral leakage in FFT analysis by tapering signal edges.

#### Hanning (Hann) Window
```
w[n] = 0.5 · (1 - cos(2πn/(N-1)))
```
- **Main lobe width:** 8π/N
- **Sidelobe attenuation:** ~31 dB
- **Use:** General-purpose spectral analysis

#### Hamming Window
```
w[n] = 0.54 - 0.46 · cos(2πn/(N-1))
```
- **Main lobe width:** 8π/N
- **Sidelobe attenuation:** ~43 dB
- **Use:** Better sidelobe suppression than Hanning

#### Blackman Window
```
w[n] = 0.42 - 0.5·cos(2πn/(N-1)) + 0.08·cos(4πn/(N-1))
```
- **Main lobe width:** 12π/N
- **Sidelobe attenuation:** ~58 dB
- **Use:** Excellent sidelobe suppression (wider main lobe)

#### Rectangular (Boxcar) Window
```
w[n] = 1.0  for all n
```
- **No windowing** (equivalent to no window)
- **Use:** When signal is already periodic in the window

## 🔬 Mathematical Foundations

### Discrete Fourier Transform (DFT)

The DFT transforms a discrete time-domain signal into its frequency-domain representation.

**Forward Transform:**
```
X[k] = Σ(n=0 to N-1) x[n] · e^(-j2πkn/N)
```

**Inverse Transform:**
```
x[n] = (1/N) · Σ(k=0 to N-1) X[k] · e^(j2πkn/N)
```

**Properties:**
- **Linearity:** DFT(ax + by) = a·DFT(x) + b·DFT(y)
- **Time shifting:** DFT(x[n-m]) = e^(-j2πkm/N) · X[k]
- **Frequency shifting:** DFT(e^(j2πmn/N)·x[n]) = X[k-m]
- **Convolution theorem:** DFT(x * y) = DFT(x) · DFT(y)
- **Parseval's theorem:** Σ|x[n]|² = (1/N)·Σ|X[k]|²

### Z-Transform

The Z-transform is the discrete-time analog of the Laplace transform.

**Definition:**
```
X(z) = Σ(n=-∞ to ∞) x[n] · z^(-n)
```

**Relationship to DFT:**
The DFT is the Z-transform evaluated on the unit circle (z = e^(j2πk/N)).

### Filter Theory

Convolution in the time domain corresponds to multiplication in the frequency domain:
```
y[n] = x[n] * h[n]  ⟺  Y(ω) = X(ω) · H(ω)
```

Where:
- `x[n]`: Input signal
- `h[n]`: Impulse response (filter kernel)
- `y[n]`: Output signal
- `H(ω)`: Frequency response

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_fft_ifft_roundtrip

# Run benchmarks (if available)
cargo bench
```

### Test Coverage

The crate includes comprehensive tests for:
- **FFT/IFFT roundtrip accuracy** (floating-point precision)
- **DC component detection** (constant signals)
- **Sine wave frequency detection** (single-frequency signals)
- **Convolution correctness** (direct vs FFT methods)
- **Window function properties** (endpoint values, symmetry)
- **Edge cases** (zero-length, single-element windows)

## 📈 Performance Characteristics

### FFT Performance

| N (samples) | DFT (naive) | FFT (Cooley-Tukey) | Speedup |
|-------------|-------------|-------------------|---------|
| 64          | 4,096 ops   | 192 ops           | 21x     |
| 256         | 65,536 ops  | 1,024 ops         | 64x     |
| 1024        | 1,048,576   | 5,120 ops         | 205x    |
| 4096        | 16,777,216  | 24,576 ops        | 683x    |

**Notes:**
- Operations counted as complex multiplications
- FFT is most efficient for power-of-2 lengths
- `rustfft` uses mixed-radix algorithms for non-power-of-2 lengths

### Convolution Performance Crossover

For typical signals, FFT-based convolution becomes faster when:
```
N + M > 100  (approximately)
```

Where N is signal length and M is kernel length.

**Example timings** (approximate, depends on hardware):
- Direct convolution (N=1000, M=10): ~10 µs
- FFT convolution (N=1000, M=10): ~50 µs
- Direct convolution (N=1000, M=100): ~100 µs
- FFT convolution (N=1000, M=100): ~60 µs ✓ faster

### Memory Usage

- **FFT**: O(N) working memory (in-place with rustfft)
- **Direct convolution**: O(N+M) output buffer
- **FFT convolution**: O(2·next_power_of_2(N+M)) for padded buffers

## 🔗 Related Crates

### Dependencies
- [`achronyme-types`](../achronyme-types/README.md): Complex numbers, tensors
- [`rustfft`](https://docs.rs/rustfft): FFT implementation

### Users
- [`achronyme-eval`](../achronyme-eval/README.md): Evaluator exposing DSP functions to SOC

### Peers
- [`achronyme-numerical`](../achronyme-numerical/README.md): Numerical analysis (integration, differentiation, root-finding)
- [`achronyme-linalg`](../achronyme-linalg/README.md): Linear algebra (matrices, vectors)
- [`achronyme-solver`](../achronyme-solver/README.md): Optimization (LP/ILP)

## 📚 References

### Books
1. **Oppenheim, A. V., & Schafer, R. W.** (2009). *Discrete-Time Signal Processing* (3rd ed.). Prentice Hall.
2. **Proakis, J. G., & Manolakis, D. G.** (2006). *Digital Signal Processing* (4th ed.). Pearson.
3. **Smith, S. W.** (1997). *The Scientist and Engineer's Guide to Digital Signal Processing*. California Technical Publishing.

### Papers
- **Cooley, J. W., & Tukey, J. W.** (1965). "An algorithm for the machine calculation of complex Fourier series." *Mathematics of Computation*, 19(90), 297-301.
- **Harris, F. J.** (1978). "On the use of windows for harmonic analysis with the discrete Fourier transform." *Proceedings of the IEEE*, 66(1), 51-83.

### Online Resources
- [FFT Algorithm Visualization](https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm)
- [Window Function Comparison](https://en.wikipedia.org/wiki/Window_function)
- [rustfft Documentation](https://docs.rs/rustfft)

## 🎯 Design Philosophy

1. **Separation of Concerns**: DSP algorithms are independent of the SOC language
2. **Type Safety**: Strong typing with `Complex`, `RealTensor`, `ComplexTensor`
3. **Performance**: Leverage optimized libraries (`rustfft`) for critical paths
4. **Flexibility**: Support both direct and optimized (FFT) algorithms
5. **Accuracy**: Extensive testing for numerical precision
6. **Usability**: Convenience functions for common cases (e.g., `fft_real()`, `fft_mag()`)

## 🔮 Future Enhancements

Potential additions to the DSP crate:

1. **Filter Design**
   - IIR filter design (Butterworth, Chebyshev, Elliptic)
   - FIR filter design (Parks-McClellan, windowed-sinc)
   - Filter coefficient generation

2. **Advanced Transforms**
   - Discrete Cosine Transform (DCT)
   - Discrete Wavelet Transform (DWT)
   - Short-Time Fourier Transform (STFT)

3. **Spectral Analysis**
   - Power Spectral Density (PSD) estimation
   - Spectrogram generation
   - Cepstral analysis

4. **Time-Domain Analysis**
   - Autocorrelation and cross-correlation
   - Peak detection
   - Zero-crossing rate

5. **Adaptive Filtering**
   - LMS (Least Mean Squares) algorithm
   - RLS (Recursive Least Squares) algorithm

## 📝 Version History

- **v0.1.0**: Initial implementation with FFT, IFFT, convolution, and window functions
  - FFT using rustfft (Cooley-Tukey algorithm)
  - Direct and FFT-based convolution
  - Hanning, Hamming, Blackman, Rectangular windows
  - Integration with achronyme-types tensors
