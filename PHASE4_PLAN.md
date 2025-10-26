# Phase 4 Implementation Plan: Higher-Order Functions & DSP

**Status**: Planning
**Target Date**: TBD
**Objective**: Add functional programming paradigms and Digital Signal Processing capabilities

---

## Overview

Phase 4 expands Achronyme Core with:
1. **Higher-order functions** (map, reduce, filter, compose)
2. **Lambda expressions** (anonymous functions)
3. **DSP functions** (DFT, FFT, convolution, filtering)
4. **Function composition** and pipelining

This transforms the calculator into a **functional programming environment** for mathematical computing.

---

## Part A: Higher-Order Functions (Functional Programming)

### 1. Lambda Expressions

**Syntax Options**:
```javascript
// Option 1: Arrow syntax (like JS/Python)
x => x^2 + 2*x + 1
(x, y) => x^2 + y^2

// Option 2: Lambda keyword (like Python)
lambda x: x^2 + 2*x + 1
lambda x, y: x^2 + y^2

// Option 3: Function literal (like Haskell)
\x -> x^2 + 2*x + 1
\x y -> x^2 + y^2
```

**Recommendation**: Use **arrow syntax** (x => expr) - most familiar to modern developers

**Implementation**:
```cpp
// New AST node
class LambdaNode : public ASTNode {
public:
    LambdaNode(std::vector<std::string> params,
               std::unique_ptr<ASTNode> body);

    const std::vector<std::string>& params() const;
    const ASTNode* body() const;
};

// New Value type: Function
class Function {
public:
    Function(std::vector<std::string> params,
             std::unique_ptr<ASTNode> body,
             Environment* closure);  // Captured variables

    Value call(const std::vector<Value>& args) const;
};
```

**Examples**:
```javascript
// Define lambda
square = x => x^2

// Use it
square(5)                    // → 25

// Inline usage
map(x => x^2, [1, 2, 3])    // → [1, 4, 9]
```

### 2. Higher-Order Functions

**Core Functions**:

#### map(f, collection)
Apply function to each element
```javascript
map(x => x^2, [1, 2, 3, 4])           // → [1, 4, 9, 16]
map(x => abs(x), [-1, -2, 3, -4])     // → [1, 2, 3, 4]
map(x => x + 1, [[1,2],[3,4]])        // → [[2,3],[4,5]]
```

#### filter(predicate, collection)
Keep elements where predicate is true
```javascript
filter(x => x > 0, [-2, -1, 0, 1, 2])   // → [1, 2]
filter(x => x % 2 == 0, [1,2,3,4,5,6])  // → [2, 4, 6]
```

#### reduce(f, initial, collection)
Accumulate values
```javascript
reduce((acc, x) => acc + x, 0, [1,2,3,4])     // → 10
reduce((acc, x) => acc * x, 1, [1,2,3,4])     // → 24
reduce((acc, x) => max(acc, x), 0, [3,1,4,1]) // → 4
```

#### compose(f, g, ...)
Function composition: (f ∘ g)(x) = f(g(x))
```javascript
f = x => x + 1
g = x => x * 2
h = compose(f, g)    // h(x) = f(g(x)) = (x*2) + 1

h(5)                 // → 11  (5*2 + 1)
```

#### pipe(x, f1, f2, ...)
Pipeline: x |> f1 |> f2 |> f3
```javascript
// Traditional
sqrt(abs(sin(PI/4)))

// Pipeline (more readable for long chains)
pipe(PI/4, sin, abs, sqrt)   // Same result, left-to-right
```

### 3. Environment/Closure Support

**Need variable binding**:
```javascript
// Variables
let x = 5
let y = 10

// Functions can close over variables
let adder = y => x + y
adder(3)              // → 8

// Update x
x = 10
adder(3)              // → 13  (closure sees new x)
```

**Implementation**:
```cpp
class Environment {
public:
    void define(const std::string& name, Value value);
    Value get(const std::string& name) const;
    void set(const std::string& name, Value value);

    std::shared_ptr<Environment> parent;  // For lexical scoping

private:
    std::unordered_map<std::string, Value> variables_;
};

class Evaluator {
    std::shared_ptr<Environment> environment_;

    // Evaluate with environment
    Value evaluate(const ASTNode* node, Environment* env);
};
```

### 4. Comparison Operators (for filter)

**New operators needed**:
```javascript
x > y     // Greater than
x < y     // Less than
x >= y    // Greater or equal
x <= y    // Less or equal
x == y    // Equal
x != y    // Not equal
```

**Implementation**:
```cpp
enum class BinaryOp {
    // Existing
    ADD, SUBTRACT, MULTIPLY, DIVIDE, POWER,

    // New (Phase 4)
    GT, LT, GTE, LTE, EQ, NEQ
};
```

---

## Part B: Digital Signal Processing (DSP)

### 1. DFT (Discrete Fourier Transform)

**Naive implementation** (for understanding):
```javascript
dft([1, 2, 3, 4])  // → Complex frequency spectrum
```

**Formula**:
```
X[k] = Σ(n=0 to N-1) x[n] * e^(-2πikn/N)
```

**Implementation**:
```cpp
Vector dft(const Vector& signal) {
    size_t N = signal.size();
    std::vector<Complex> spectrum(N);

    for (size_t k = 0; k < N; k++) {
        Complex sum(0, 0);
        for (size_t n = 0; n < N; n++) {
            double angle = -2.0 * M_PI * k * n / N;
            Complex twiddle = Complex::fromPolar(1.0, angle);
            sum = sum + signal[n] * twiddle;
        }
        spectrum[k] = sum;
    }

    return spectrum;  // Returns complex vector
}
```

**Usage**:
```javascript
// Time domain signal
signal = [1, 2, 3, 4, 3, 2, 1, 0]

// Frequency domain
spectrum = dft(signal)

// Get magnitude spectrum
magnitudes = map(z => abs(z), spectrum)
```

**Complexity**: O(N²) - slow but simple

### 2. FFT (Fast Fourier Transform)

**Cooley-Tukey algorithm** (radix-2):
```javascript
fft([1, 2, 3, 4, 5, 6, 7, 8])  // → Same as DFT but MUCH faster
```

**Requirements**:
- Input size must be power of 2
- If not, zero-pad to next power of 2

**Implementation** (Recursive):
```cpp
std::vector<Complex> fft(const std::vector<Complex>& x) {
    size_t N = x.size();

    // Base case
    if (N == 1) return x;

    // Divide
    std::vector<Complex> even(N/2), odd(N/2);
    for (size_t i = 0; i < N/2; i++) {
        even[i] = x[2*i];
        odd[i] = x[2*i + 1];
    }

    // Conquer
    auto fft_even = fft(even);
    auto fft_odd = fft(odd);

    // Combine
    std::vector<Complex> result(N);
    for (size_t k = 0; k < N/2; k++) {
        double angle = -2.0 * M_PI * k / N;
        Complex twiddle = Complex::fromPolar(1.0, angle);
        Complex t = twiddle * fft_odd[k];

        result[k] = fft_even[k] + t;
        result[k + N/2] = fft_even[k] - t;
    }

    return result;
}
```

**Complexity**: O(N log N) - MUCH faster than DFT

**Comparison**:
| N | DFT Time | FFT Time | Speedup |
|---|----------|----------|---------|
| 64 | ~4,096 ops | ~384 ops | 10.6x |
| 256 | ~65,536 ops | ~2,048 ops | 32x |
| 1024 | ~1,048,576 ops | ~10,240 ops | 102x |
| 4096 | ~16,777,216 ops | ~49,152 ops | 341x |

### 3. IFFT (Inverse FFT)

**Formula**: Almost identical to FFT, just conjugate and scale
```cpp
std::vector<Complex> ifft(const std::vector<Complex>& X) {
    size_t N = X.size();

    // Conjugate input
    std::vector<Complex> X_conj(N);
    for (size_t i = 0; i < N; i++) {
        X_conj[i] = X[i].conjugate();
    }

    // Apply FFT
    auto result = fft(X_conj);

    // Conjugate and scale
    for (size_t i = 0; i < N; i++) {
        result[i] = result[i].conjugate() / N;
    }

    return result;
}
```

**Usage**:
```javascript
signal = [1, 2, 3, 4, 5, 6, 7, 8]
spectrum = fft(signal)
recovered = ifft(spectrum)  // → [1, 2, 3, 4, 5, 6, 7, 8]
```

### 4. Convolution

**Linear convolution**:
```javascript
convolve([1, 2, 3], [0.5, 0.5])  // → [0.5, 1.5, 2.5, 1.5]
```

**Direct implementation** (O(N*M)):
```cpp
Vector convolve(const Vector& x, const Vector& h) {
    size_t N = x.size();
    size_t M = h.size();
    size_t L = N + M - 1;  // Output length

    std::vector<double> y(L, 0.0);

    for (size_t n = 0; n < L; n++) {
        for (size_t m = 0; m < M; m++) {
            if (n >= m && (n - m) < N) {
                y[n] += x[n - m] * h[m];
            }
        }
    }

    return Vector(y);
}
```

**FFT-based convolution** (O(N log N)) - faster for large inputs:
```cpp
Vector fftConvolve(const Vector& x, const Vector& h) {
    // 1. Zero-pad to next power of 2
    size_t L = nextPowerOf2(x.size() + h.size() - 1);

    // 2. FFT of both signals
    auto X = fft(padded(x, L));
    auto H = fft(padded(h, L));

    // 3. Multiply in frequency domain
    std::vector<Complex> Y(L);
    for (size_t i = 0; i < L; i++) {
        Y[i] = X[i] * H[i];
    }

    // 4. IFFT to get result
    auto y = ifft(Y);

    // 5. Take real parts and trim
    return extractReal(y, x.size() + h.size() - 1);
}
```

**Use cases**:
- Audio filtering
- Image processing
- Feature detection
- System response analysis

### 5. Filtering Functions

#### Low-pass filter
```javascript
// Simple moving average filter
lpf = [0.2, 0.2, 0.2, 0.2, 0.2]  // 5-point MA
filtered = convolve(signal, lpf)
```

#### High-pass filter
```javascript
hpf = [-0.25, -0.25, 1, -0.25, -0.25]
filtered = convolve(signal, hpf)
```

### 6. Windowing Functions

**For reducing spectral leakage**:
```javascript
// Hanning window
hanning(N)  // → [0, ..., 1 (center), ..., 0]

// Hamming window
hamming(N)

// Blackman window
blackman(N)

// Apply window
windowed = [1, 2, 3, 4] * hanning(4)
```

**Implementation**:
```cpp
Vector hanning(size_t N) {
    std::vector<double> window(N);
    for (size_t n = 0; n < N; n++) {
        window[n] = 0.5 * (1.0 - std::cos(2.0 * M_PI * n / (N - 1)));
    }
    return Vector(window);
}
```

---

## Part C: Implementation Strategy

### Step 1: Comparison Operators (Week 1)
- Add `>`, `<`, `>=`, `<=`, `==`, `!=` to lexer
- Update parser for comparison expressions
- Implement in evaluator
- Returns boolean (represented as 0/1)

### Step 2: Lambda Syntax (Week 1-2)
- Add `=>` token to lexer
- Add `LambdaNode` to AST
- Implement lambda parsing
- Add Function type to Value
- Implement lambda evaluation with closures

### Step 3: Environment/Variables (Week 2)
- Create `Environment` class
- Add variable binding: `let x = 5`
- Implement lexical scoping
- Update evaluator to use environment

### Step 4: Higher-Order Functions (Week 2-3)
- Implement `map(f, vec)`
- Implement `filter(pred, vec)`
- Implement `reduce(f, init, vec)`
- Implement `compose(f, g, ...)`
- Implement `pipe(x, f1, f2, ...)`

### Step 5: DFT (Naive) (Week 3)
- Implement O(N²) DFT
- Add tests with known signals
- Verify with MATLAB/NumPy/SciPy

### Step 6: FFT (Cooley-Tukey) (Week 4)
- Implement recursive FFT
- Implement IFFT
- Zero-padding utilities
- Performance benchmarks

### Step 7: Convolution (Week 4)
- Direct convolution (O(NM))
- FFT-based convolution (O(N log N))
- Auto-select based on size

### Step 8: Windowing & Filters (Week 5)
- Hanning, Hamming, Blackman windows
- Common filter designs
- Helper functions

---

## Part D: Syntax Examples

### Complete Phase 4 Capabilities

```javascript
// ============================================================
// HIGHER-ORDER FUNCTIONS
// ============================================================

// Lambda expressions
square = x => x^2
add = (x, y) => x + y
quadratic = (a, b, c) => x => a*x^2 + b*x + c

// Map
squares = map(x => x^2, [1, 2, 3, 4, 5])
// → [1, 4, 9, 16, 25]

magnitudes = map(z => abs(z), [3+4i, 1+2i, 5+12i])
// → [5, 2.236, 13]

// Filter
positives = filter(x => x > 0, [-2, -1, 0, 1, 2, 3])
// → [1, 2, 3]

evens = filter(x => x % 2 == 0, [1, 2, 3, 4, 5, 6])
// → [2, 4, 6]

// Reduce
sum = reduce((acc, x) => acc + x, 0, [1, 2, 3, 4, 5])
// → 15

product = reduce((acc, x) => acc * x, 1, [1, 2, 3, 4, 5])
// → 120

// Compose
f = x => x + 1
g = x => x * 2
h = x => x^2
composed = compose(h, g, f)  // h(g(f(x)))

composed(3)  // → 64  (((3+1)*2)^2 = 64)

// Pipeline (more readable)
result = pipe(
    3,
    x => x + 1,    // 4
    x => x * 2,    // 8
    x => x^2       // 64
)

// ============================================================
// DSP FUNCTIONS
// ============================================================

// Time domain signal
signal = [1, 2, 3, 4, 3, 2, 1, 0]

// DFT (slow but simple)
spectrum = dft(signal)

// FFT (fast for large signals)
spectrum = fft(signal)

// Inverse FFT
recovered = ifft(spectrum)

// Get magnitude spectrum
magnitudes = map(z => abs(z), spectrum)

// Get phase spectrum
phases = map(z => arg(z), spectrum)

// Convolution (filtering)
impulse_response = [0.25, 0.5, 0.25]  // Simple low-pass
filtered = convolve(signal, impulse_response)

// Windowing (reduce spectral leakage)
windowed = signal * hanning(8)
spectrum = fft(windowed)

// ============================================================
// REAL-WORLD EXAMPLES
// ============================================================

// Audio: Frequency analysis
audio = loadAudio("signal.wav")
spectrum = fft(audio * hanning(length(audio)))
dominant_freq = findPeak(map(z => abs(z), spectrum))

// Signal processing: Noise reduction
noisy_signal = [/* ... */]
lpf = [0.2, 0.2, 0.2, 0.2, 0.2]  // Moving average
clean = convolve(noisy_signal, lpf)

// Data science: Moving average
prices = [100, 102, 101, 105, 110, 108, 112]
ma = (window, data) => (
    let kernel = map(x => 1/window, range(window))
    convolve(data, kernel)
)
smoothed = ma(3, prices)

// Physics: Wave interference
wave1 = map(t => sin(2*PI*5*t), linspace(0, 1, 100))
wave2 = map(t => sin(2*PI*7*t), linspace(0, 1, 100))
interference = map((w1, w2) => w1 + w2, wave1, wave2)
```

---

## Part E: Testing Strategy

### Higher-Order Function Tests
```javascript
// map
map(x => x^2, [1, 2, 3])             // → [1, 4, 9]
map(x => abs(x), [-1, 2, -3])        // → [1, 2, 3]

// filter
filter(x => x > 0, [-1, 0, 1, 2])    // → [1, 2]
filter(x => x % 2 == 0, [1,2,3,4])   // → [2, 4]

// reduce
reduce((a,x) => a+x, 0, [1,2,3,4])   // → 10

// compose
f = x => x+1
g = x => x*2
compose(g, f)(3)                      // → 8

// pipe
pipe(3, x => x+1, x => x*2)          // → 8
```

### DSP Tests
```javascript
// DFT of impulse
dft([1, 0, 0, 0])                    // → [1, 1, 1, 1]

// DFT of constant
dft([1, 1, 1, 1])                    // → [4, 0, 0, 0]

// FFT = DFT
signal = [1, 2, 3, 4, 5, 6, 7, 8]
abs(dft(signal) - fft(signal)) < 1e-10  // → true

// IFFT recovers signal
ifft(fft(signal)) == signal          // → true

// Convolution identity
convolve([1], x) == x                // → true

// Convolution with impulse
convolve([1,2,3], [1]) == [1,2,3]   // → true
```

---

## Part F: Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Lambda call | <10μs | Closure overhead |
| map(f, 1000 elements) | <1ms | Including lambda calls |
| DFT (N=64) | <500μs | Naive O(N²) |
| FFT (N=1024) | <100μs | O(N log N) |
| FFT (N=8192) | <1ms | Typical audio frame |
| Convolution (N=1000, M=10) | <100μs | Direct |
| Convolution (N=8192, M=512) | <2ms | FFT-based |

---

## Part G: Success Criteria

- [ ] Lambda expressions work: `x => x^2`
- [ ] Variables and closures work
- [ ] map, filter, reduce implemented
- [ ] compose and pipe work
- [ ] Comparison operators (>, <, ==, etc.)
- [ ] DFT produces correct results
- [ ] FFT produces correct results (matches DFT)
- [ ] IFFT recovers original signal
- [ ] Convolution works correctly
- [ ] FFT-convolution matches direct convolution
- [ ] Windowing functions implemented
- [ ] 100+ tests passing
- [ ] Performance targets met
- [ ] Documentation complete

---

## Part H: Size Estimate

**Current**: 234 KB
**Estimated Phase 4**: 350-400 KB
**Increase**: ~150 KB

**Breakdown**:
- Lambda/closure support: +30 KB
- Environment/variables: +20 KB
- Higher-order functions: +30 KB
- DFT/FFT implementation: +50 KB
- Convolution & filters: +30 KB
- Windowing functions: +10 KB

---

## Part I: Breaking Changes

**None expected** - all additions are backwards compatible

Phase 1-3 code continues to work:
```javascript
2 + 3           // Still works
[1,2] + [3,4]   // Still works
det([[1,2],[3,4]])  // Still works
```

---

## Next: Phase 5+ (Future)

- Symbolic computation (derivatives, integrals)
- Units and dimensions (5 m/s, 10 kg)
- Statistical functions (mean, std, correlation)
- Optimization (gradient descent, Newton's method)
- Linear algebra (eigenvalues, SVD, QR)
- Plotting/visualization
- File I/O

---

**Ready to start Phase 4!** 🚀
