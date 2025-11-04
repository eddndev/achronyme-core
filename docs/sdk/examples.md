# Examples - Achronyme SDK v2.0

This page provides practical examples for using the Achronyme SDK v2.0. All examples assume you have initialized the SDK:

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();
```

For simplicity, most examples omit the recommended `ach.use()` block, but you should always use it in production code to prevent memory leaks.

---

## Basic Operations

### Creating and Using Vectors

```typescript
// Create a vector
const v = ach.vector([1, 2, 3, 4, 5]);

// Access data with a zero-copy view
const dataView = v.data; // Instant Float64Array view
console.log('Zero-copy view:', dataView);

// Get a copy as a standard JS array
const dataCopy = v.toArray();
console.log('Copied array:', dataCopy);

// Perform math operations
const sinV = ach.math.sin(v);
console.log('sin(v):', sinV.toString());

// Clean up memory
v.dispose();
sinV.dispose();
```

### Creating and Using Matrices

```typescript
// Create a matrix
const m = ach.matrix([
    [1, 2, 3],
    [4, 5, 6]
]);

console.log(`Matrix is ${m.rows}x${m.cols}`);

// Get a specific element
console.log('Element at (1, 2):', m.get(1, 2)); // 6

// Get a row as a new vector
const row1 = m.row(1); // Vector [4, 5, 6]
console.log('Row 1:', row1.toArray());

// Clean up
m.dispose();
row1.dispose();
```

---

## Linear Algebra

### Matrix Decomposition (LU)

```typescript
await ach.use(async () => {
    const A = ach.matrix([[4, 3], [6, 3]]);

    // Perform LU decomposition
    const { L, U, P } = ach.linalg.lu(A);

    console.log('L matrix:\n', L.toArray());
    console.log('U matrix:\n', U.toArray());
    console.log('P matrix:\n', P.toArray());

    // All matrices (A, L, U, P) are auto-disposed
});
```

### Matrix Inverse and Determinant

```typescript
await ach.use(async () => {
    const A = ach.matrix([[1, 2], [3, 4]]);

    // Calculate the determinant
    const det = ach.linalg.det(A);
    console.log('Determinant:', det); // -2

    // Calculate the inverse
    const A_inv = ach.linalg.inverse(A);
    console.log('Inverse:\n', A_inv.toArray()); // [[-2, 1], [1.5, -0.5]]
});
```

---

## Digital Signal Processing (DSP)

### Generating a Signal and Applying FFT

This example generates a sine wave, applies a window, and computes its FFT magnitude spectrum.

```typescript
await ach.use(async () => {
    const numSamples = 1024;
    const sampleRate = 1024;

    // 1. Generate a time vector
    const t = ach.linspace(0, 1, numSamples);

    // 2. Generate a 50 Hz sine wave
    const signal = ach.map(x => Math.sin(2 * Math.PI * 50 * x), t);

    // 3. Apply a Hanning window
    const window = ach.dsp.hanning(numSamples);
    const windowedSignal = ach.vecOps.vmul(signal, window); // Element-wise multiplication

    // 4. Compute the FFT magnitude spectrum
    const spectrum = ach.dsp.fftMag(windowedSignal);

    // 5. Access the data (zero-copy) and find the peak
    const spectrumData = spectrum.data;
    let maxMag = -1;
    let peakFreq = -1;
    for (let i = 0; i < spectrumData.length / 2; i++) {
        if (spectrumData[i] > maxMag) {
            maxMag = spectrumData[i];
            peakFreq = i * (sampleRate / numSamples);
        }
    }

    console.log(`Found peak frequency at: ${peakFreq.toFixed(2)} Hz`); // ~50.00 Hz
});
```

---

## Statistics

### Calculating Basic Statistics

```typescript
await ach.use(async () => {
    const data = ach.vector([1, 5, 2, 8, 7, 3, 4, 6, 9, 5]);

    const mean = ach.stats.mean(data);
    const std = ach.stats.std(data, 1); // Sample std dev
    const median = ach.stats.median(data);
    const p95 = ach.stats.percentile(data, 95);

    console.log(`Mean: ${mean.toFixed(2)}`);
    console.log(`Std Dev: ${std.toFixed(2)}`);
    console.log(`Median: ${median}`);
    console.log(`95th Percentile: ${p95}`);
});
```

---

## Higher-Order Functions

### Map, Filter, Reduce

```typescript
await ach.use(async () => {
    const v = ach.vector([1, 2, 3, 4, 5, 6]);

    // Map: square each element
    const squared = ach.hof.map(x => x * x, v);
    console.log('Squared:', squared.toArray()); // [1, 4, 9, 16, 25, 36]

    // Filter: keep only even numbers
    const evens = ach.hof.filter(x => x % 2 === 0, squared);
    console.log('Evens:', evens.toArray()); // [4, 16, 36]

    // Reduce: sum the results
    const sum = ach.hof.reduce((acc, x) => acc + x, 0, evens);
    console.log('Sum:', sum); // 56
});
```

---

## Expression Evaluator (`eval`)

The `eval` engine allows you to run complex sequences of operations defined in a string, minimizing JS-WASM overhead.

```typescript
await ach.use(async () => {
    // Simple math
    console.log(ach.eval("2 + 3 * 4")); // "14"

    // Using functions
    console.log(ach.eval("sqrt(16) + sin(PI/2)")); // "5"

    // Declare a variable and a function
    ach.eval("let f = x => x * x + 1");
    // Use them
    console.log(ach.eval("f(5)")); // "26"

    // Process a vector
    console.log(ach.eval("map(x => x * 2, [1, 2, 3, 4])")); // "[2, 4, 6, 8]"

    // Reset the evaluator's state
    ach.resetEvaluator();
});
```