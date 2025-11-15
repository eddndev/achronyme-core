# achronyme-dsp/src - Implementation Guide

Internal architecture and implementation details for the `achronyme-dsp` crate.

## 🏛️ Internal Architecture

The DSP crate is organized into three main modules, each responsible for a specific aspect of digital signal processing:

```
src/
├── lib.rs              # Public API, module declarations, re-exports
├── fft.rs              # FFT/IFFT implementations
├── convolution.rs      # Convolution algorithms
└── windows.rs          # Window function generation
```

### Architectural Principles

1. **Thin Wrapper Pattern**: Wraps `rustfft` with Achronyme-native types
2. **Type Conversion Layer**: Converts between `achronyme_types::Complex` and `num_complex::Complex64`
3. **Algorithm Variants**: Provides both direct and optimized (FFT) versions where applicable
4. **Zero-Copy Where Possible**: Minimizes allocations in hot paths
5. **Error Handling**: Uses `Result<T, String>` for user-facing errors

## 📁 Module Structure

### lib.rs - Public API

**Responsibilities:**
- Module declarations (`pub mod fft`, `convolution`, `windows`)
- Public API re-exports for convenience
- Crate-level documentation

**Exports:**
```rust
// FFT functions
pub use fft::{fft_transform, ifft_transform, fft_real, ifft_real};

// Convolution functions
pub use convolution::{convolve, convolve_fft};

// Window functions
pub use windows::{
    hanning_window, hamming_window, blackman_window,
    rectangular_window, apply_window
};
```

**Design Pattern:**
- **Facade Pattern**: Provides a simplified interface to the DSP subsystems
- Users can import from `achronyme_dsp::` directly without module paths

### fft.rs - Fourier Transforms

**Responsibilities:**
- FFT and IFFT for complex signals
- Convenience functions for real-valued signals
- Type conversions between Achronyme and rustfft types

**Key Components:**

#### Type Conversion Functions
```rust
fn to_complex64(input: &[Complex]) -> Vec<Complex64>
fn from_complex64(input: &[Complex64]) -> Vec<Complex>
```

**Purpose:** Bridge between Achronyme's `Complex` type and rustfft's `Complex64`

**Complexity:** O(N) - unavoidable copy for type safety

**Design Decision:**
- Could use `unsafe` transmute for zero-copy, but chose safety over performance
- Modern compilers optimize these copies well
- The FFT itself dominates runtime (O(N log N))

#### Core FFT Functions

##### `fft_transform(input: &[Complex]) -> Vec<Complex>`

**Algorithm:** Cooley-Tukey FFT via `rustfft::FftPlanner`

**Implementation Details:**
```rust
pub fn fft_transform(input: &[Complex]) -> Vec<Complex> {
    // 1. Convert to rustfft's Complex64
    let mut buffer = to_complex64(input);

    // 2. Create FFT planner (cached internally by rustfft)
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer.len());

    // 3. In-place FFT
    fft.process(&mut buffer);

    // 4. Convert back to Achronyme Complex
    from_complex64(&buffer)
}
```

**Optimizations:**
- `FftPlanner` caches twiddle factors for repeated calls with same size
- `process()` is in-place (no extra allocation)
- Optimal for power-of-2 lengths, uses mixed-radix for others

**Complexity:**
- Time: O(N log N)
- Space: O(N) for conversion + O(log N) for rustfft internals

##### `ifft_transform(input: &[Complex]) -> Vec<Complex>`

**Differences from FFT:**
1. Uses `plan_fft_inverse()` instead of `plan_fft_forward()`
2. Applies scaling factor `1/N` after transform

**Scaling Factor:**
```rust
// IFFT must scale by 1/N to satisfy:
// IFFT(FFT(x)) = x
let scale = 1.0 / (n as f64);
for x in &mut buffer {
    *x *= scale;
}
```

**Convention:** Scaling in IFFT (not FFT) matches NumPy/MATLAB convention

##### `fft_real(input: &[f64]) -> Vec<Complex>`

**Purpose:** Convenience for real-valued signals (most common case)

**Implementation:**
```rust
pub fn fft_real(input: &[f64]) -> Vec<Complex> {
    let complex_input: Vec<Complex> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    fft_transform(&complex_input)
}
```

**Optimization Opportunity:**
- Could use `rustfft::num_complex::Complex64::from_real()` more efficiently
- Current implementation prioritizes clarity

**Output Symmetry:**
For real input, output has **conjugate symmetry**:
```
X[k] = conj(X[N-k])  for k = 1, 2, ..., N-1
```

This property is not exploited (could halve storage), but kept for simplicity.

##### `ifft_real(input: &[Complex]) -> Vec<f64>`

**Assumption:** Input has conjugate symmetry (from FFT of real signal)

**Returns:** Only real parts (imaginary parts should be near-zero)

**Use Case:** Reconstruct real signal after frequency-domain processing

### convolution.rs - Convolution Algorithms

**Responsibilities:**
- Direct (time-domain) convolution
- FFT-based (frequency-domain) convolution
- Algorithm selection guidance

**Key Components:**

#### `convolve(signal: &[f64], kernel: &[f64]) -> Vec<f64>`

**Algorithm:** Direct convolution (textbook definition)

**Mathematical Formula:**
```
y[n] = Σ(m=0 to M-1) signal[n-m] · kernel[m]
```

**Implementation:**
```rust
pub fn convolve(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let m = kernel.len();
    let output_len = n + m - 1;  // Standard linear convolution length

    let mut result = vec![0.0; output_len];

    for i in 0..output_len {
        let mut sum = 0.0;
        for j in 0..m {
            if i >= j && i - j < n {
                sum += signal[i - j] * kernel[j];
            }
        }
        result[i] = sum;
    }

    result
}
```

**Complexity Analysis:**
- Time: O(N · M) where N = signal length, M = kernel length
- Space: O(N + M - 1) for output

**Boundary Handling:**
```rust
if i >= j && i - j < n {
    // Only accumulate when indices are valid
    // Implicitly zero-pads signal
}
```

**When to Use:**
- Small kernels (M < 50)
- Simplicity over speed
- Exact arithmetic (no FFT rounding)

#### `convolve_fft(signal: &[f64], kernel: &[f64]) -> Vec<f64>`

**Algorithm:** FFT-based convolution using the Convolution Theorem

**Convolution Theorem:**
```
conv(f, g) = IFFT(FFT(f) · FFT(g))
```

**Implementation Steps:**
```rust
pub fn convolve_fft(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let m = kernel.len();
    let output_len = n + m - 1;

    // 1. Pad to next power of 2 for FFT efficiency
    let fft_len = output_len.next_power_of_two();

    // 2. Zero-pad both signals to fft_len
    let mut signal_padded = signal.to_vec();
    signal_padded.resize(fft_len, 0.0);

    let mut kernel_padded = kernel.to_vec();
    kernel_padded.resize(fft_len, 0.0);

    // 3. FFT both signals
    let signal_fft = fft_real(&signal_padded);
    let kernel_fft = fft_real(&kernel_padded);

    // 4. Element-wise multiplication in frequency domain
    let product: Vec<Complex> = signal_fft
        .iter()
        .zip(kernel_fft.iter())
        .map(|(a, b)| *a * *b)  // Complex multiplication
        .collect();

    // 5. IFFT to get result
    let result_full = ifft_real(&product);

    // 6. Trim to actual output length
    result_full.into_iter().take(output_len).collect()
}
```

**Complexity Analysis:**
- Padding: O(N + M)
- Two FFTs: 2 × O(P log P) where P = next power of 2 after (N+M-1)
- Multiplication: O(P)
- One IFFT: O(P log P)
- Trimming: O(N + M)
- **Total: O(P log P) ≈ O((N+M) log(N+M))**

**Zero-Padding Strategy:**

Why pad to power of 2?
```rust
let fft_len = output_len.next_power_of_two();
```

- FFT is fastest for power-of-2 lengths (radix-2 algorithm)
- For 500-element output, pads to 512 (2⁹)
- Small overhead, significant speed gain

**Frequency-Domain Multiplication:**
```rust
.map(|(a, b)| *a * *b)  // Uses Complex::mul()
```

Each complex multiplication:
```
(a + bi) × (c + di) = (ac - bd) + (ad + bc)i
```

**When to Use:**
- Large kernels (M > 100)
- Very long signals (N > 1000)
- Real-time processing with fixed kernel

**Performance Crossover:**

Empirical testing shows crossover at approximately:
```
N · M ≈ 10,000 operations
```

Examples:
- N=100, M=10: Use direct (1,000 ops)
- N=100, M=100: Use FFT (10,000 ops, borderline)
- N=1000, M=100: Use FFT (100,000 ops → FFT much faster)

### windows.rs - Window Functions

**Responsibilities:**
- Generate standard window functions
- Apply windows to signals
- Provide mathematical correctness for edge cases

**Key Components:**

#### Window Function Theory

Windows reduce **spectral leakage** caused by finite-length signals.

**The Problem:**
```
FFT assumes signal is periodic with period N.
Non-periodic signals have discontinuities at boundaries.
Discontinuities → high-frequency artifacts (spectral leakage).
```

**The Solution:**
```
Multiply signal by window that tapers to zero at edges.
Smooth transition → reduced leakage.
Trade-off: Wider main lobe, better sidelobe attenuation.
```

#### `hanning_window(n: usize) -> Vec<f64>`

**Mathematical Formula:**
```
w[i] = 0.5 · (1 - cos(2πi/(N-1)))    for i = 0, 1, ..., N-1
```

**Implementation:**
```rust
pub fn hanning_window(n: usize) -> Vec<f64> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![1.0]; }  // Edge case

    let mut window = Vec::with_capacity(n);
    let n_minus_1 = (n - 1) as f64;

    for i in 0..n {
        let value = 0.5 * (1.0 - (2.0 * PI * i as f64 / n_minus_1).cos());
        window.push(value);
    }

    window
}
```

**Properties:**
- `w[0] = w[N-1] = 0.0` (zero at endpoints)
- `w[N/2] = 1.0` (peak at center)
- Symmetric: `w[i] = w[N-1-i]`

**Numerical Stability:**
- Division by `N-1` (not `N`) for correct endpoint behavior
- Avoids `cos(2π) ≠ 1.0` due to floating-point error

**Spectral Characteristics:**
- Main lobe width: 8π/N (radians)
- First sidelobe: -31.5 dB
- Sidelobe rolloff: -18 dB/octave

#### `hamming_window(n: usize) -> Vec<f64>`

**Mathematical Formula:**
```
w[i] = 0.54 - 0.46 · cos(2πi/(N-1))
```

**Alternative form:**
```
w[i] = α - (1-α) · cos(2πi/(N-1))    where α = 0.54
```

**Key Difference from Hanning:**
- Non-zero endpoints: `w[0] = w[N-1] = 0.08`
- Optimized for sidelobe attenuation

**Implementation:**
```rust
pub fn hamming_window(n: usize) -> Vec<f64> {
    // ... edge cases ...
    for i in 0..n {
        let value = 0.54 - 0.46 * (2.0 * PI * i as f64 / n_minus_1).cos();
        window.push(value);
    }
    window
}
```

**Spectral Characteristics:**
- Main lobe width: 8π/N (same as Hanning)
- First sidelobe: -42.7 dB (much better than Hanning)
- Sidelobe rolloff: -6 dB/octave (slower than Hanning)

**When to Use:**
- Need better sidelobe suppression than Hanning
- Can tolerate slightly worse distant sidelobe behavior
- Classic choice for speech processing

#### `blackman_window(n: usize) -> Vec<f64>`

**Mathematical Formula:**
```
w[i] = 0.42 - 0.5·cos(2πi/(N-1)) + 0.08·cos(4πi/(N-1))
```

**Three-term cosine series** (generalized form):
```
w[i] = a₀ - a₁·cos(2πi/(N-1)) + a₂·cos(4πi/(N-1))
where: a₀ = 0.42, a₁ = 0.5, a₂ = 0.08
```

**Implementation:**
```rust
pub fn blackman_window(n: usize) -> Vec<f64> {
    // ... edge cases ...
    for i in 0..n {
        let t = 2.0 * PI * i as f64 / n_minus_1;
        let value = 0.42 - 0.5 * t.cos() + 0.08 * (2.0 * t).cos();
        window.push(value);
    }
    window
}
```

**Optimization:**
- Precompute `t = 2πi/(N-1)` to avoid redundant calculation
- `cos(2t)` computed from `t` (not `4πi/(N-1)`)

**Spectral Characteristics:**
- Main lobe width: 12π/N (wider than Hanning/Hamming)
- First sidelobe: -58.1 dB (excellent)
- Sidelobe rolloff: -18 dB/octave

**When to Use:**
- Need excellent sidelobe suppression
- Can afford wider main lobe (less frequency resolution)
- High-precision spectral analysis

#### `rectangular_window(n: usize) -> Vec<f64>`

**Mathematical Formula:**
```
w[i] = 1.0    for all i
```

**Implementation:**
```rust
pub fn rectangular_window(n: usize) -> Vec<f64> {
    vec![1.0; n]
}
```

**Simplest possible implementation** - no windowing.

**Spectral Characteristics:**
- Main lobe width: 4π/N (narrowest)
- First sidelobe: -13.3 dB (worst)
- Sidelobe rolloff: -6 dB/octave

**When to Use:**
- Signal is already periodic in window (no discontinuities)
- Maximum frequency resolution needed
- Spectral leakage is not a concern

#### `apply_window(signal: &[f64], window: &[f64]) -> Result<Vec<f64>, String>`

**Purpose:** Element-wise multiplication of signal and window

**Implementation:**
```rust
pub fn apply_window(signal: &[f64], window: &[f64]) -> Result<Vec<f64>, String> {
    if signal.len() != window.len() {
        return Err(format!(
            "Signal length ({}) must match window length ({})",
            signal.len(), window.len()
        ));
    }

    let result: Vec<f64> = signal
        .iter()
        .zip(window.iter())
        .map(|(s, w)| s * w)
        .collect();

    Ok(result)
}
```

**Error Handling:**
- Validates length match before processing
- Returns descriptive error message

**Complexity:** O(N)

**Alternative in SOC:**
```soc
// Can also use element-wise multiplication:
let windowed = signal * window
```

## 🔄 Module Interactions

### Type Flow Diagram

```
User (SOC) → achronyme-eval → achronyme-dsp → rustfft
    ↓             ↓                ↓              ↓
  String      Value           Complex      Complex64
              Tensor         Vec<f64>
           RealTensor
         ComplexTensor
```

### Function Call Flow (FFT Example)

```
1. SOC:  fft([1, 2, 3, 4])
         ↓
2. eval: function_modules/dsp.rs::fft()
         - Convert Value::Vector to RealTensor
         - Extract &[f64] from tensor
         ↓
3. dsp:  fft.rs::fft_real(&[f64])
         - Convert f64 to Complex (imag = 0)
         ↓
4. dsp:  fft.rs::fft_transform(&[Complex])
         - Convert Complex to Complex64
         - Call rustfft
         - Convert back to Complex
         ↓
5. eval: Wrap Vec<Complex> in ComplexTensor
         ↓
6. SOC:  Returns ComplexTensor value
```

### Error Propagation

```
rustfft → (no errors, panics on invalid input)
         ↓
  dsp → validates before calling rustfft
         returns Result<T, String>
         ↓
 eval → catches errors, adds context
         returns Result<Value, String>
         ↓
  SOC → displays error to user
```

## 🎯 Design Patterns

### 1. Adapter Pattern

**Problem:** rustfft uses `num_complex::Complex64`, Achronyme uses custom `Complex`

**Solution:** Conversion functions
```rust
fn to_complex64(input: &[Complex]) -> Vec<Complex64>
fn from_complex64(input: &[Complex64]) -> Vec<Complex>
```

### 2. Facade Pattern

**Problem:** Multiple modules with complex APIs

**Solution:** lib.rs re-exports for simple imports
```rust
pub use fft::fft_transform;
pub use convolution::convolve;
pub use windows::hanning_window;
```

### 3. Strategy Pattern

**Problem:** Multiple convolution algorithms

**Solution:** Separate functions for each strategy
```rust
convolve()      // Direct algorithm
convolve_fft()  // FFT-based algorithm
```

User selects based on performance needs.

### 4. Template Method Pattern (implicit)

**Window functions** follow common pattern:
```rust
1. Handle edge cases (n=0, n=1)
2. Allocate result vector
3. Compute denominator (N-1)
4. Loop: compute window value
5. Return vector
```

Could be refactored to:
```rust
fn generate_window<F>(n: usize, formula: F) -> Vec<f64>
where F: Fn(usize, f64) -> f64
```

## 🧪 Testing Strategy

### Unit Tests

Each module has comprehensive unit tests:

#### FFT Tests (fft.rs)
```rust
#[test]
fn test_fft_ifft_roundtrip()
    // Verifies: IFFT(FFT(x)) ≈ x
    // Uses approx::assert_relative_eq! for floating-point comparison

#[test]
fn test_fft_dc_component()
    // Verifies: FFT([1,1,1,1]) = [4, 0, 0, 0]
    // Tests DC (zero frequency) component

#[test]
fn test_fft_sine_wave()
    // Verifies: Peak at correct frequency bin
    // Tests frequency detection
```

#### Convolution Tests (convolution.rs)
```rust
#[test]
fn test_convolve_simple()
    // Verifies: Known input → known output

#[test]
fn test_convolve_identity()
    // Verifies: conv(x, [1]) = x

#[test]
fn test_convolve_fft_matches_direct()
    // Verifies: Both algorithms produce same results
    // Critical for correctness
```

#### Window Tests (windows.rs)
```rust
#[test]
fn test_hanning_window()
    // Verifies: Endpoints are zero, center is 1

#[test]
fn test_window_edge_cases()
    // Verifies: n=0 and n=1 handled correctly
```

### Integration Tests

Located in `achronyme-eval/tests/test_dsp_functions.rs`:
- Tests SOC language integration
- Tests Value type conversions
- Tests error messages

### Property-Based Testing (Future)

Could add `proptest` or `quickcheck` for:
```rust
// Property: Parseval's theorem
∀ signal. sum(|signal[i]|²) ≈ sum(|FFT(signal)[k]|²) / N

// Property: Convolution commutativity
∀ a, b. conv(a, b) = conv(b, a)

// Property: FFT linearity
∀ a, b, α, β. FFT(α·a + β·b) = α·FFT(a) + β·FFT(b)
```

## 🔧 Extension Guide

### Adding a New Window Function

Example: Adding Kaiser window

**Step 1:** Implement in `windows.rs`
```rust
/// Generate a Kaiser window
pub fn kaiser_window(n: usize, beta: f64) -> Vec<f64> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![1.0]; }

    let mut window = Vec::with_capacity(n);
    let n_minus_1 = (n - 1) as f64;

    for i in 0..n {
        let alpha = (i as f64 - n_minus_1 / 2.0) / (n_minus_1 / 2.0);
        let value = bessel_i0(beta * (1.0 - alpha * alpha).sqrt())
                  / bessel_i0(beta);
        window.push(value);
    }

    window
}

// Helper: Modified Bessel function of first kind, order 0
fn bessel_i0(x: f64) -> f64 {
    // Implementation using series expansion
    // ...
}
```

**Step 2:** Export in `lib.rs`
```rust
pub use windows::{
    // ... existing ...
    kaiser_window,
};
```

**Step 3:** Add tests in `windows.rs`
```rust
#[test]
fn test_kaiser_window() {
    let window = kaiser_window(10, 5.0);
    assert_eq!(window.len(), 10);
    // Check symmetry
    for i in 0..5 {
        assert_relative_eq!(window[i], window[9-i], epsilon = 1e-10);
    }
}
```

**Step 4:** Expose to SOC in `achronyme-eval/src/function_modules/dsp.rs`
```rust
pub fn register_functions(registry: &mut FunctionRegistry) {
    // ... existing ...
    registry.register("kaiser", kaiser, 2);  // n and beta
}

fn kaiser(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(n), Value::Number(beta)) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err("kaiser() requires non-negative integer".to_string());
            }
            let window = achronyme_dsp::windows::kaiser_window(*n as usize, *beta);
            let tensor = achronyme_types::tensor::RealTensor::vector(window);
            Ok(Value::Tensor(tensor))
        }
        _ => Err("kaiser() requires (window_size, beta)".to_string()),
    }
}
```

### Adding a New Transform

Example: Adding Discrete Cosine Transform (DCT)

**Step 1:** Create `src/dct.rs`
```rust
use achronyme_types::complex::Complex;

/// Discrete Cosine Transform (DCT-II)
pub fn dct(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let mut result = vec![0.0; n];

    for k in 0..n {
        let mut sum = 0.0;
        for i in 0..n {
            let angle = PI * k as f64 * (2.0 * i as f64 + 1.0) / (2.0 * n as f64);
            sum += input[i] * angle.cos();
        }
        // Normalization
        let scale = if k == 0 {
            (1.0 / n as f64).sqrt()
        } else {
            (2.0 / n as f64).sqrt()
        };
        result[k] = sum * scale;
    }

    result
}

/// Inverse DCT (DCT-III)
pub fn idct(input: &[f64]) -> Vec<f64> {
    // Similar implementation
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_dct_idct_roundtrip() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let transformed = dct(&signal);
        let reconstructed = idct(&transformed);

        for i in 0..signal.len() {
            assert_relative_eq!(signal[i], reconstructed[i], epsilon = 1e-10);
        }
    }
}
```

**Step 2:** Declare module in `lib.rs`
```rust
pub mod dct;
pub use dct::{dct, idct};
```

**Step 3:** Register in evaluator

### Adding Filter Design Functions

Example: Butterworth filter

**Step 1:** Create `src/filters.rs`
```rust
pub struct ButterworthFilter {
    pub b: Vec<f64>,  // Numerator coefficients
    pub a: Vec<f64>,  // Denominator coefficients
}

pub fn butterworth_lowpass(order: usize, cutoff: f64, sample_rate: f64)
    -> Result<ButterworthFilter, String>
{
    // Implement using bilinear transform
    // 1. Design analog prototype
    // 2. Apply bilinear transform s → z
    // 3. Return digital filter coefficients
}

pub fn apply_filter(signal: &[f64], filter: &ButterworthFilter) -> Vec<f64> {
    // Implement Direct Form II filtering
    // ...
}
```

## 📐 Numerical Considerations

### Floating-Point Precision

**Issue:** FFT accumulates rounding errors

**Mitigation:**
- Use `f64` (not `f32`) for all intermediate calculations
- Test with `approx::assert_relative_eq!` with `epsilon = 1e-10`
- Document expected precision in function comments

**Example:**
```rust
// FFT roundtrip: expect ~1e-10 relative error
let reconstructed = ifft_transform(&fft_transform(&signal));
assert_relative_eq!(signal[i], reconstructed[i], epsilon = 1e-10);
```

### Numerical Stability in Convolution

**Direct convolution** is numerically stable (no FFT rounding).

**FFT convolution** can have issues:
- Zero-padding introduces artificial precision
- IFFT accumulates rounding in imaginary parts
- Solution: Use `ifft_real()` to discard imaginary parts

### Window Function Precision

**Edge case:** N=1
```rust
if n == 1 { return vec![1.0]; }
```
Without this, division by `(n-1) = 0` causes NaN.

**Endpoint values:**
```rust
// Hanning: w[0] should be exactly 0.0
// But cos(0) = 1.0, so 0.5 * (1 - 1) = 0.0 ✓

// Hamming: w[0] should be 0.08
// 0.54 - 0.46 * 1 = 0.08 ✓
```

## 📊 Performance Optimization

### Current Optimizations

1. **In-place FFT**: `rustfft::process()` modifies buffer in-place
2. **Power-of-2 padding**: Ensures fast radix-2 FFT
3. **Iterator chains**: Zero-allocation where possible
4. **Preallocation**: `Vec::with_capacity(n)` before loops

### Future Optimizations

#### Real FFT Optimization
Currently:
```rust
// Convert real → complex → FFT
let complex_input: Vec<Complex> = input.iter()
    .map(|&x| Complex::new(x, 0.0))
    .collect();
fft_transform(&complex_input)
```

Could use:
```rust
// Use rustfft's RealFft for 2x speedup
use rustfft::FftPlannerRealInverse;
```

Benefit: Exploits conjugate symmetry, halves computation.

#### SIMD for Convolution

Direct convolution could use SIMD:
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Vectorize inner loop using AVX2
// Process 4 f64 values at once
```

Benefit: ~4x speedup for direct convolution.

#### Multithreading for Large Signals

```rust
use rayon::prelude::*;

// Parallel FFT for very large signals
chunks.par_iter()
    .map(|chunk| fft_transform(chunk))
    .collect()
```

Benefit: Scale to multiple cores for N > 10⁶.

## 📚 Implementation References

### FFT Implementation
- **Cooley-Tukey Algorithm**: Original 1965 paper
- **rustfft documentation**: https://docs.rs/rustfft
- **Numerical Recipes in C**: Chapter 12 (FFT)

### Convolution
- **Oppenheim & Schafer**: Section 3.3 (Convolution)
- **Smith's DSP Guide**: Chapter 6 (Convolution)

### Window Functions
- **Harris, 1978**: "On the use of windows..." (definitive reference)
- **MATLAB Signal Processing Toolbox**: Window function reference

## 🎓 Educational Notes

### Why FFT is Fast

**DFT (naive):**
```
X[k] = Σ(n=0 to N-1) x[n] · e^(-j2πkn/N)
```
For each k: N multiplications
Total: N × N = O(N²)

**FFT (Cooley-Tukey):**
Key insight - divide-and-conquer:
```
Split N-point DFT into:
- N/2-point DFT of even-indexed samples
- N/2-point DFT of odd-indexed samples
- Combine with N twiddle factors
```

Recursion:
```
T(N) = 2·T(N/2) + O(N)
```

By Master Theorem: **T(N) = O(N log N)**

### Convolution Theorem Proof (Sketch)

```
Given: conv(f, g)[n] = Σ f[m]·g[n-m]

1. Take DFT of both sides:
   DFT(conv(f,g))[k] = Σₙ (Σₘ f[m]·g[n-m]) · e^(-j2πkn/N)

2. Rearrange summations:
   = Σₘ f[m] · (Σₙ g[n-m]·e^(-j2πkn/N))

3. Substitute p = n-m:
   = Σₘ f[m]·e^(-j2πkm/N) · (Σₚ g[p]·e^(-j2πkp/N))

4. Recognize DFTs:
   = DFT(f)[k] · DFT(g)[k]

∴ DFT(conv(f,g)) = DFT(f) · DFT(g)
```

### Why Window Functions Work

**Problem:** Finite signal → discontinuity at boundary → high frequencies

**Mathematical view:**
```
Observed signal = True signal × Rectangular window
                            (implicit)
```

**Frequency domain (Convolution Theorem):**
```
FFT(observed) = FFT(true signal) * FFT(rectangular window)
                                   (sinc function - has lobes)
```

Sinc sidelobes "leak" into other frequency bins.

**Solution:**
```
Windowed signal = True signal × Smooth window
```

Smooth window → narrower frequency response → less leakage.

## 🔍 Debugging Tips

### FFT Results Look Wrong

**Check 1:** Signal length
```rust
println!("Signal length: {}", signal.len());
// FFT works for any length, but powers of 2 are fastest
```

**Check 2:** Scaling
```rust
// FFT magnitude scales with N
// Normalize if needed:
let normalized: Vec<_> = spectrum.iter()
    .map(|x| x / signal.len() as f64)
    .collect();
```

**Check 3:** Frequency bins
```rust
// Bin k corresponds to frequency:
// f_k = k · (sample_rate / N)
let freq = k as f64 * sample_rate / signal.len() as f64;
```

### Convolution Results Don't Match

**Check 1:** Output length
```rust
// Should be N + M - 1, not N or M
assert_eq!(result.len(), signal.len() + kernel.len() - 1);
```

**Check 2:** Direct vs FFT differ
```rust
// Check for numerical precision issues
for i in 0..result.len() {
    assert_relative_eq!(direct[i], fft[i], epsilon = 1e-8);
    // Use 1e-8, not 1e-10 (FFT has more rounding)
}
```

**Check 3:** Kernel orientation
```rust
// Convolution vs correlation
// conv(f, g) ≠ corr(f, g)
// For symmetric kernels, they're equal
```

### Window Not Working

**Check 1:** Window length matches signal
```rust
assert_eq!(signal.len(), window.len());
```

**Check 2:** Applied correctly
```rust
// Element-wise multiplication
let windowed: Vec<_> = signal.iter()
    .zip(window.iter())
    .map(|(s, w)| s * w)
    .collect();

// NOT convolution!
```

## 🗺️ Roadmap

### Short-term (Next Release)
- [ ] Real FFT optimization using `rustfft::RealFft`
- [ ] Add DCT/IDCT (Discrete Cosine Transform)
- [ ] Add autocorrelation function

### Medium-term
- [ ] IIR filter design (Butterworth, Chebyshev)
- [ ] FIR filter design (windowed-sinc, Parks-McClellan)
- [ ] Spectrogram generation (STFT)

### Long-term
- [ ] Wavelet transforms
- [ ] Adaptive filters (LMS, RLS)
- [ ] Multirate DSP (upsampling, downsampling)

## 📝 Contribution Guidelines

When adding new DSP functionality:

1. **Follow existing patterns**
   - Type conversions for rustfft interop
   - Edge case handling (n=0, n=1)
   - Comprehensive tests

2. **Document mathematical foundations**
   - Include formulas in doc comments
   - Reference standard textbooks
   - Explain algorithm complexity

3. **Test thoroughly**
   - Unit tests for correctness
   - Numerical precision tests (`approx`)
   - Edge cases

4. **Consider performance**
   - Profile before optimizing
   - Document complexity
   - Provide algorithm variants (direct vs optimized)

5. **Maintain SOC integration**
   - Add functions to `achronyme-eval/function_modules/dsp.rs`
   - Write integration tests in `achronyme-eval/tests/`
   - Update examples in `examples/soc/`
