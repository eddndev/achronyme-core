# Achronyme SDK v2.0

Modern TypeScript SDK for the Achronyme mathematical engine, built on Rust WASM with zero-copy views and automatic memory management.

## ✨ Features

- **Session-Based Management**: RAII-style `use()` blocks for automatic, guaranteed memory cleanup.
- **Zero-Copy Views**: Instant access to WASM memory via `Float64Array` views, eliminating slow data copies.
- **Type-Safe API**: Fully typed with TypeScript for robust, predictable code.
- **Modular Design**: Operations are grouped into logical modules (`math`, `dsp`, `linalg`, etc.), allowing for tree-shaking.
- **High-Performance Core**: Powered by a Rust-based WASM engine for near-native speed.
- **Powerful Expression Evaluator**: Includes a built-in evaluator for the SOC language, enabling complex operations in a single call.

## 🚀 Quick Start

### Installation

```bash
npm install @achronyme/core
```

### Basic Usage (Recommended)

The recommended approach is to use `ach.use()` to ensure all memory is automatically cleaned up.

```typescript
import { Achronyme } from '@achronyme/core';

// Initialize the engine
const ach = new Achronyme();
await ach.init();

// Use a session for automatic resource management
await ach.use(async () => {
    // Create values (vectors, matrices, etc.)
    const signal = ach.vector([...Array(1024).keys()]);

    // Perform operations using the modular API
    const sinSignal = ach.math.sin(signal);

    // Run DSP functions
    const spectrum = ach.dsp.fftMag(sinSignal);

    // Access data with a zero-copy view (instant!)
    const spectrumData = spectrum.data;
    console.log('Spectrum preview:', spectrumData.slice(0, 5));

    // ✅ All values (signal, sinSignal, spectrum) are
    // automatically disposed when this block exits!
});

console.log('Active values after use() block:', ach.getActiveValuesCount()); // 0
```

## 🏛️ Architecture

The SDK is designed with a clear, modular structure:

```
src/sdk/
├── Achronyme.ts        # Main facade and entry point
├── core/               # Core infrastructure
│   ├── Session.ts         # RAII-style session management (`use` block)
│   ├── HandleManager.ts   # Tracks WASM memory handles
│   ├── RustBindings.ts    # Type-safe interface to the WASM module
│   └── MemoryPool.ts      # (Optional) Buffer pooling for performance
├── operations/         # Modules for different domains
│   ├── MathOps.ts         # (ach.math) sin, cos, exp, etc.
│   ├── DSPOps.ts          # (ach.dsp) fft, conv, windowing
│   ├── LinalgOps.ts       # (ach.linalg) lu, qr, svd, det
│   ├── VectorOps.ts       # (ach.vecOps) dot, cross, norm
│   ├── StatsOps.ts        # (ach.stats) sum, mean, std
│   └── HOFOps.ts          # (ach.hof) map, filter, reduce
└── values/             # Data structure classes
    ├── Value.ts           # Abstract base class for all values
    ├── Vector.ts          # 1D data with zero-copy .data view
    ├── Matrix.ts          # 2D data with zero-copy .data view
    ├── Scalar.ts          # Single number wrapper
    └── Complex.ts         # Complex number type
```

## 🧠 Memory Management

The SDK provides two primary ways to manage memory:

1.  **Session-based (Recommended)**: The `ach.use()` method creates a scope. Any Achronyme value created inside is automatically disposed of when the scope is exited, even if an error occurs. This is the safest and easiest way to prevent memory leaks.

2.  **Manual Cleanup**: For advanced use cases, you can manage memory manually. Any value you create must be explicitly freed using its `.dispose()` method.

See the [Memory Management Guide](./memory-management.md) for detailed patterns and best practices.

## ⚡ Performance

The SDK is built for high performance. Key features include:

-   **Zero-Copy Views**: Accessing the `.data` property of a `Vector` or `Matrix` returns a `Float64Array` that points directly to the WASM memory, avoiding any data duplication.
-   **WASM-Native Functions**: All operations in the `ach.*Ops` modules are executed natively in Rust/WASM.
-   **Expression Evaluator**: The `ach.eval()` method can execute complex chains of operations in a single call, minimizing the JS-WASM communication overhead.

See the [Performance Guide](./optimization-functions.md) for more details.