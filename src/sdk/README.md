# Achronyme SDK v2.0

Modern TypeScript SDK for the Achronyme mathematical engine, built on Rust WASM with zero-copy views and automatic memory management.

## ✨ Features

- **Zero Memory Leaks**: Automatic cleanup with FinalizationRegistry + WeakRef
- **Zero-Copy Views**: 80x+ faster data access for large datasets
- **Type-Safe**: Full TypeScript support, no `any` types
- **Modular**: Tree-shakeable, <300 LOC per file
- **Session-Based**: RAII-style resource management
- **High Performance**: Rust WASM backend, 1.43x faster than JS native

## 🚀 Quick Start

```typescript
import { Achronyme } from '@achronyme/core';

// Initialize
const ach = new Achronyme();
await ach.init();

// Session-based usage (recommended)
await ach.use(async () => {
    // Create values
    const signal = ach.vector([...Array(1000).keys()]);

    // Math operations
    const sinSignal = ach.math.sin(signal);

    // DSP operations
    const spectrum = ach.dsp.fftMag(signal);

    // Zero-copy view (instant!)
    const view = spectrum.data; // Float64Array

    // ✅ NO dispose() needed - auto-cleanup!
});
```

## 📚 Architecture

```
src/sdk-v2/
├── core/               # Core infrastructure
│   ├── RustBindings.ts    # Rust WASM bindings
│   ├── HandleManager.ts   # Auto-cleanup with GC
│   ├── Session.ts         # RAII resource management
│   └── MemoryPool.ts      # Buffer pooling
├── values/             # Value types
│   ├── Vector.ts          # 1D arrays (zero-copy)
│   ├── Matrix.ts          # 2D arrays (zero-copy)
│   ├── Scalar.ts          # Single numbers
│   └── Complex.ts         # Complex numbers
├── operations/         # Operation modules
│   ├── MathOps.ts         # sin, cos, exp, etc.
│   ├── DSPOps.ts          # fft, conv, windows
│   ├── LinalgOps.ts       # lu, qr, svd
│   ├── VectorOps.ts       # dot, cross, norm
│   ├── HOFOps.ts          # map, filter, reduce
│   └── StatsOps.ts        # sum, mean, std
├── Achronyme.ts        # Main facade
└── index.ts            # Public API
```

## 🎯 Usage Examples

### Basic Operations

```typescript
// Create vectors
const v = ach.vector([1, 2, 3, 4, 5]);

// Zero-copy view
const view = v.data; // Float64Array (instant)

// Explicit copy (when needed)
const arr = v.toArray(); // number[] (copy)

// Math operations
const sinV = ach.sin(v);
const expV = ach.exp(v);
const sqrtV = ach.sqrt(v);
```

### DSP Operations

```typescript
// Generate signal
const t = ach.linspace(0, 1, 1000);
const signal = ach.map(x => Math.sin(2 * Math.PI * 10 * x), t);

// Apply window
const window = ach.dsp.hanning(1000);
const windowed = ach.vector.vmul(signal, window);

// FFT
const spectrum = ach.dsp.fftMag(windowed);

// Process
for (const mag of spectrum) {
    console.log(mag);
}
```

### Linear Algebra

```typescript
// Create matrix
const A = ach.matrix([[4, 3], [6, 3]]);

// LU decomposition
const { L, U, P } = ach.lu(A);

// QR decomposition
const { Q, R } = ach.qr(A);

// SVD
const { U: U_svd, S, V } = ach.svd(A);

// Matrix operations
const det = ach.det(A);
const At = ach.transpose(A);
```

### Statistics

```typescript
const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

// Basic stats
const sum = ach.sum(data);     // 55
const mean = ach.mean(data);   // 5.5
const std = ach.std(data);     // ~2.87

// Min/max
const min = ach.min(data);     // 1
const max = ach.max(data);     // 10

// Advanced
const median = ach.stats.median(data);
const p95 = ach.stats.percentile(data, 95);
```

### Higher-Order Functions

```typescript
const v = ach.vector([1, 2, 3, 4, 5]);

// Map
const squared = ach.map(x => x * x, v); // [1, 4, 9, 16, 25]

// Filter
const evens = ach.filter(x => x % 2 === 0, v); // [2, 4]

// Reduce
const sum = ach.reduce((acc, x) => acc + x, 0, v); // 15

// Pipeline
const result = ach.hof.pipe([
    (x) => ach.sin(x),
    (x) => ach.abs(x),
    (x) => ach.dsp.fftMag(x)
])(v);
```

## 🔧 API Reference

### Main Facade

```typescript
class Achronyme {
    // Initialization
    async init(): Promise<void>

    // Session management
    async use<T>(fn: () => Promise<T> | T): Promise<T>
    cleanup(): void

    // Value constructors
    vector(data: number[]): Vector
    matrix(data: number[][]): Matrix
    scalar(value: number): Scalar
    complex(re: number, im: number): Complex

    // Operation modules
    readonly math: MathOps
    readonly dsp: DSPOps
    readonly linalg: LinalgOps
    readonly vector: VectorOps
    readonly hof: HOFOps
    readonly stats: StatsOps

    // Convenience methods
    sin, cos, tan, exp, ln, log, sqrt, abs, pow
    fft, fftMag, ifft, conv
    lu, qr, svd, det, transpose
    dot, cross, norm
    sum, mean, std, min, max
    map, filter, reduce

    // Utilities
    linspace(start, stop, num): Vector
    identity(n): Matrix
    zeros(n): Vector
    ones(n): Vector

    // Constants
    PI, E, SQRT2, LN2, LN10

    // Memory debugging
    getMemoryStats(): HandleStats
    gc(): number
    getActiveValuesCount(): number
}
```

### Value Types

```typescript
class Vector extends Value {
    get data(): Float64Array    // Zero-copy view
    get length(): number
    get(index: number): number
    set(index: number, value: number): void
    toArray(): number[]         // Explicit copy
    map(fn): Vector
    filter(fn): Vector
    reduce<T>(fn, init): T
    [Symbol.iterator]()         // for...of support
}

class Matrix extends Value {
    get data(): Float64Array    // Flattened (row-major)
    get rows(): number
    get cols(): number
    get(row: number, col: number): number
    set(row: number, col: number, value: number): void
    row(index: number): Vector
    col(index: number): Vector
    toArray(): number[][]       // 2D array
    toFlatArray(): number[]     // Flattened
    map(fn): Matrix
    [Symbol.iterator]()         // Iterates over rows
}
```

## ⚡ Performance

Benchmarks (10M elements):

| Operation | SDK v1.0 | SDK v2.0 | Speedup |
|-----------|----------|----------|---------|
| Vector creation | 350ms | 350ms | 1.0x |
| toArray() copy | 80ms | <1ms | **80x+** ⚡ |
| FFT (16M) | 2957ms | 2957ms | 1.0x |
| Memory cleanup | Manual | Auto | ♾️ |
| Memory leaks | Possible | **Zero** | ✅ |

## 🧪 Memory Management

### Session-Based (Recommended)

```typescript
await ach.use(async () => {
    const v = ach.vector([...10_000_000]);
    // ... operations ...
    // ✅ Auto-cleanup when scope exits
});
```

### Manual (Advanced)

```typescript
const v = ach.vector([1, 2, 3]);
// ... use v ...
v.dispose(); // Manual cleanup
```

### Debugging Memory

```typescript
// Get statistics
const stats = ach.getMemoryStats();
console.log(stats);
// {
//   allocated: 100,
//   freed: 95,
//   active: 5,
//   leaked: 0  // ✅ Should be 0
// }

// Force GC
const cleaned = ach.gc();
console.log(`Cleaned ${cleaned} handles`);
```

## 📦 Module Exports

```typescript
// Main facade
import { Achronyme } from '@achronyme/core';

// Core infrastructure
import {
    AchronymeSession,
    HandleManager,
    MemoryPool,
    RustWASM
} from '@achronyme/core';

// Value types
import {
    Vector,
    Matrix,
    Scalar,
    Complex
} from '@achronyme/core';

// Operations modules
import {
    MathOps,
    DSPOps,
    LinalgOps,
    VectorOps,
    HOFOps,
    StatsOps
} from '@achronyme/core';
```

## 🎯 Migration from v1.0

### Breaking Changes

1. **Session-based API** (optional but recommended)
2. **toVector() → .data** (zero-copy view)
3. **Modular operations** (ach.math.sin vs ach.sin)

### Migration Example

```typescript
// v1.0
const ach = new Achronyme();
await ach.init();
const x = ach.vector([1, 2, 3]);
const result = await x.sin().toVector();
x.dispose(); // Must remember!

// v2.0 (option 1: session-based)
await ach.use(async () => {
    const x = ach.vector([1, 2, 3]);
    const result = ach.sin(x);
    const data = result.data; // Zero-copy
    // Auto-cleanup!
});

// v2.0 (option 2: direct with auto-GC)
const x = ach.vector([1, 2, 3]);
const result = ach.sin(x);
const data = result.data;
// Auto-cleanup when GC'd
```

## 🔍 Troubleshooting

### Memory Leaks

```typescript
// Check for leaks
const stats = ach.getMemoryStats();
if (stats.leaked > 0) {
    console.warn(`Detected ${stats.leaked} leaked handles`);
    ach.gc(); // Force cleanup
}
```

### Performance Issues

```typescript
// Use zero-copy views
const data = vector.data; // ✅ Fast

// Avoid unnecessary copies
const arr = vector.toArray(); // ❌ Slow for large data
```

## 📄 License

MIT

## 🤝 Contributing

Contributions welcome! Please see CONTRIBUTING.md

---

**Built with ❤️ using Rust + WASM + TypeScript**
