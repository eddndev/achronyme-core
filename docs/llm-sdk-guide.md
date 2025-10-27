# Achronyme SDK - LLM Instructions

**Version:** 0.3.0
**Package:** `@achronyme/core`
**Type:** WebAssembly-based mathematical computation engine with TypeScript SDK

---

## Quick Installation

```bash
npm install @achronyme/core
```

For Vite projects, install the WASM plugin:

```bash
npm install vite-plugin-wasm vite-plugin-top-level-await --save-dev
```

---

## Vite Configuration (REQUIRED for WebAssembly)

Create or update `vite.config.js`:

```javascript
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  optimizeDeps: {
    exclude: ['@achronyme/core']
  }
});
```

**Why this is needed:**
- `vite-plugin-wasm`: Serves `.wasm` files correctly
- `vite-plugin-top-level-await`: Enables top-level await for WASM initialization
- `exclude: ['@achronyme/core']`: Prevents Vite from pre-bundling the WASM module

---

## Basic Usage Pattern

```typescript
import { Achronyme } from '@achronyme/core';

// 1. Initialize
const ach = new Achronyme();
await ach.init();

// 2. Create values
const x = ach.number(42);
const y = ach.number(10);

// 3. Perform operations
const result = x.add(y);

// 4. Extract values
const value = await result.toNumber();
console.log(value); // 52

// 5. CRITICAL: Dispose all values
x.dispose();
y.dispose();
result.dispose();
```

---

## 🚨 CRITICAL: Memory Management Rules

**EVERY AchronymeValue MUST be disposed:**

### ✅ CORRECT Pattern
```typescript
const sumValue = ach.sum(vector);
const sum = await sumValue.toNumber();
sumValue.dispose();  // ← MUST dispose
```

### ❌ WRONG Pattern (Memory Leak)
```typescript
const sum = await ach.sum(vector).toNumber();
// ← No reference to dispose! MEMORY LEAK
```

### ❌ WRONG Pattern in reduce()
```typescript
const result = ach.reduce('(a,b) => a+b', v, 0);
// ← 0 should be ach.number(0) which needs dispose
```

### ✅ CORRECT Pattern in reduce()
```typescript
const initialValue = ach.number(0);
const result = ach.reduce('(a,b) => a+b', v, initialValue);
// Cleanup:
v.dispose();
initialValue.dispose();
result.dispose();
```

---

## Type Constructors

```typescript
ach.number(42)                      // Number
ach.vector([1, 2, 3, 4])           // Vector
ach.matrix([[1, 2], [3, 4]])       // Matrix
ach.complex(2, 3)                   // Complex: 2+3i
```

---

## Value Extraction

```typescript
await value.toNumber()      // → number
await value.toVector()      // → number[]
await value.toMatrix()      // → number[][]
await value.toComplex()     // → {re: number, im: number}
```

---

## Common Operations

### Arithmetic (Fluent API)
```typescript
const x = ach.number(5);
const result = x.mul(2).add(10).div(4);  // (5 * 2 + 10) / 4 = 5
const val = await result.toNumber();

x.dispose();
result.dispose();
```

### Vector Operations (Native - Fast)
```typescript
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.vector([4, 5, 6]);

// Optimized native functions
const sum = ach.vadd(v1, v2);      // Vector addition
const prod = ach.vmul(v1, v2);     // Element-wise multiplication
const scaled = ach.vscale(v1, 2);  // Scalar multiplication

// Cleanup
v1.dispose();
v2.dispose();
sum.dispose();
prod.dispose();
scaled.dispose();
```

### Statistics (Native - Fast)
```typescript
const data = ach.vector([1, 2, 3, 4, 5]);

const sumValue = ach.sum(data);
const meanValue = ach.mean(data);
const stdValue = ach.std(data);
const maxValue = ach.max(data);
const minValue = ach.min(data);

// Extract values
const sum = await sumValue.toNumber();
const mean = await meanValue.toNumber();
const std = await stdValue.toNumber();
const max = await maxValue.toNumber();
const min = await minValue.toNumber();

// MUST dispose all
data.dispose();
sumValue.dispose();
meanValue.dispose();
stdValue.dispose();
maxValue.dispose();
minValue.dispose();
```

### DSP Functions
```typescript
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);

// FFT (requires power of 2)
const spectrum = ach.fft_mag(signal);
const phase = ach.fft_phase(signal);

// Windows
const hanning = ach.hanning(8);
const hamming = ach.hamming(8);
const blackman = ach.blackman(8);

// Convolution
const s1 = ach.vector([1, 2, 3]);
const s2 = ach.vector([1, 1]);
const conv = ach.conv(s1, s2);

// Cleanup
signal.dispose();
spectrum.dispose();
phase.dispose();
hanning.dispose();
hamming.dispose();
blackman.dispose();
s1.dispose();
s2.dispose();
conv.dispose();
```

### Functional Programming
```typescript
const data = ach.vector([1, 2, 3, 4, 5, 6]);

// Map
const squared = ach.map('x => x^2', data);

// Filter
const evens = ach.filter('x => x % 2 == 0', data);

// Reduce (CORRECT pattern)
const initialValue = ach.number(0);
const sum = ach.reduce('(a,b) => a+b', data, initialValue);

// Cleanup
data.dispose();
squared.dispose();
evens.dispose();
initialValue.dispose();
sum.dispose();
```

---

## Math Functions

```typescript
// Trigonometric
ach.sin(x), ach.cos(x), ach.tan(x)
ach.asin(x), ach.acos(x), ach.atan(x), ach.atan2(y, x)

// Exponential & Logarithmic
ach.exp(x), ach.ln(x), ach.log(x), ach.log10(x), ach.log2(x)

// Roots & Powers
ach.sqrt(x), ach.cbrt(x), ach.pow(base, exp)

// Rounding
ach.floor(x), ach.ceil(x), ach.round(x), ach.trunc(x)

// Utilities
ach.abs(x), ach.sign(x)
ach.min(...values), ach.max(...values)
```

---

## Matrix Operations

```typescript
const A = ach.matrix([[1, 2], [3, 4]]);
const B = ach.matrix([[5, 6], [7, 8]]);

// Operations
const sum = A.add(B);
const product = A.mul(B);
const transpose = A.transpose();
const det = A.det();

// Cleanup
A.dispose();
B.dispose();
sum.dispose();
product.dispose();
transpose.dispose();
det.dispose();
```

---

## Memory Management Best Practices

### Pattern 1: Dispose Immediately After Use
```typescript
const x = ach.number(10);
const result = await x.toNumber();
x.dispose();
```

### Pattern 2: Array for Multiple Variables
```typescript
const toDispose = [];

const signal = ach.vector(data);
toDispose.push(signal);

const window = ach.hanning(1024);
toDispose.push(window);

const windowed = signal.mul(window);
toDispose.push(windowed);

// Cleanup all at once
toDispose.forEach(v => v.dispose());
```

### Pattern 3: Try-Finally for Error Safety
```typescript
const values = [];
try {
  const x = ach.number(10);
  values.push(x);

  const y = x.mul(2);
  values.push(y);

  const result = await y.toNumber();
  return result;
} finally {
  values.forEach(v => v.dispose());
}
```

### Check Memory Stats
```typescript
const stats = ach.getMemoryStats();
console.log('Active Variables:', stats.activeVariables);
console.log('Disposed Variables:', stats.disposedVariables);
```

---

## Complete Example: DSP Pipeline

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

const toDispose = [];

// Generate signal
const N = 1024;
const signalData = Array.from({length: N}, (_, i) =>
  Math.sin(2 * Math.PI * 50 * i / 1000)
);
const signal = ach.vector(signalData);
toDispose.push(signal);

// Apply window
const window = ach.hanning(N);
toDispose.push(window);

const windowed = ach.vmul(signal, window);  // Native vmul
toDispose.push(windowed);

// FFT
const spectrum = ach.fft_mag(windowed);
toDispose.push(spectrum);

// Statistics
const maxValue = ach.max(spectrum);
toDispose.push(maxValue);

const meanValue = ach.mean(spectrum);
toDispose.push(meanValue);

// Extract results
const spectrumVals = await spectrum.toVector();
const max = await maxValue.toNumber();
const mean = await meanValue.toNumber();

console.log('Spectrum peak:', max);
console.log('Spectrum mean:', mean);

// Cleanup
toDispose.forEach(v => v.dispose());
```

---

## Constants

```typescript
ach.PI      // 3.14159265358979...
ach.E       // 2.71828182845905...
ach.PHI     // 1.61803398874989...
ach.TAU     // 6.28318530717959...
```

---

## Error Handling

```typescript
import {
  AchronymeError,
  AchronymeSyntaxError,
  AchronymeRuntimeError,
  AchronymeTypeError
} from '@achronyme/core';

try {
  const x = ach.number(5);
  const result = x.div(0);
} catch (e) {
  if (e instanceof AchronymeRuntimeError) {
    console.error('Runtime error:', e.message);
  }
}
```

---

## Common Pitfalls to Avoid

### ❌ Don't do this
```typescript
// 1. Chaining without storing references (can't dispose intermediates)
const result = ach.sum(ach.vector([1, 2, 3]));
await result.toNumber();
result.dispose();
// ← The vector is never disposed!

// 2. Using literals in reduce
ach.reduce('(a,b) => a+b', vector, 0);  // ← 0 should be ach.number(0)

// 3. Calling toNumber() immediately
const sum = await ach.sum(vector).toNumber();  // ← Can't dispose!
```

### ✅ Do this instead
```typescript
// 1. Store all intermediate values
const vector = ach.vector([1, 2, 3]);
const result = ach.sum(vector);
await result.toNumber();
vector.dispose();
result.dispose();

// 2. Use ach.number() for reduce initial value
const initialValue = ach.number(0);
const result = ach.reduce('(a,b) => a+b', vector, initialValue);
initialValue.dispose();

// 3. Store reference before extraction
const sumValue = ach.sum(vector);
const sum = await sumValue.toNumber();
sumValue.dispose();
```

---

## Performance Tips

1. **Use native functions when available:**
   - `ach.vadd()`, `ach.vmul()`, `ach.vscale()` instead of operations through parser
   - `ach.sum()`, `ach.mean()`, `ach.std()` are optimized C++ implementations

2. **For simple reductions on large datasets:**
   - Consider extracting to JS and using native JS reduce for very simple operations
   - WASM functions shine in complex operations and when avoiding data transfer

3. **Minimize WASM ↔ JS boundary crossings:**
   - Chain operations in WASM before extracting
   - Use `toDispose` array pattern for pipelines

---

## Browser vs Node.js

The SDK works identically in both environments. The only difference is the module system:

```typescript
// Browser (ES modules)
import { Achronyme } from '@achronyme/core';

// Node.js (ES modules, .mjs files)
import { Achronyme } from '@achronyme/core';
```

---

## Package Information

- **Name:** `@achronyme/core`
- **Current Version:** `0.3.0-beta-6`
- **License:** MIT
- **Repository:** achronyme-core
- **Bundle Size:** ~500KB (WASM module included)
- **Browser Support:** Modern browsers with WebAssembly support
- **Node.js:** v16+ recommended

---

## Testing Installation

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

const x = ach.number(42);
console.log(await x.toNumber()); // Should print: 42
x.dispose();

const stats = ach.getMemoryStats();
console.log('Memory stats:', stats);
// Should show: { totalVariables: 1, activeVariables: 0, disposedVariables: 1 }
```

---

## Related Documentation

- Full SDK Guide: `docs/sdk-guide.md`
- Language Specification: `docs/language-spec.md`
- Roadmap: `docs/roadmap.md`

---

**Last Updated:** 2025-10-27