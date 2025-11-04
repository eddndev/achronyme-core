# API Reference - Achronyme SDK v2.0

This document provides a detailed reference for all classes, methods, and functions in the Achronyme SDK.

## Table of Contenidos

- [Achronyme (Main Class)](#achronyme-main-class)
- [Value Classes](#value-classes)
  - [Value (Base Class)](#value-base-class)
  - [Vector](#vector)
  - [Matrix](#matrix)
  - [Scalar](#scalar)
  - [Complex](#complex)
- [Operation Modules](#operation-modules)
  - [MathOps (`ach.math`)](#mathops-achmath)
  - [DSPOps (`ach.dsp`)](#dspops-achdsp)
  - [LinalgOps (`ach.linalg`)](#linalgops-achlinalg)
  - [VectorOps (`ach.vecOps`)](#vectorops-achvecops)
  - [StatsOps (`ach.stats`)](#statsops-achstats)
  - [HOFOps (`ach.hof`)](#hofops-achhof)

---

## Achronyme (Main Class)

The main facade for all SDK functionality.

### `constructor()`

Creates a new Achronyme instance.

```typescript
const ach = new Achronyme();
```

### `init(): Promise<void>`

Initializes the underlying WASM module. **Must be called before any other operation.**

```typescript
await ach.init();
```

### `use<T>(fn: () => Promise<T> | T): Promise<T>`

Executes a function within a managed session, ensuring all created `Value` objects are automatically cleaned up. This is the recommended way to use the SDK.

```typescript
await ach.use(async () => {
    const v = ach.vector([1, 2, 3]);
    // ... operations ...
    // v is auto-disposed here
});
```

### `cleanup(): void`

Manually cleans up all values created by this instance. Only needed if not using `use()`.

### Value Constructors

- `vector(data: number[]): Vector`: Creates a new `Vector`.
- `matrix(data: number[][]): Matrix`: Creates a new `Matrix`.
- `scalar(value: number): Scalar`: Creates a new `Scalar`.
- `complex(re: number, im: number): Complex`: Creates a new `Complex` number.
- `vec(data: number[]): Vector`: Alias for `vector()`.
- `mat(data: number[][]): Matrix`: Alias for `matrix()`.

### Utility Functions

- `linspace(start: number, stop: number, num: number): Vector`: Creates a vector with `num` linearly spaced points from `start` to `stop`.
- `identity(n: number): Matrix`: Creates an `n x n` identity matrix.
- `zeros(n: number): Vector`: Creates a vector of `n` zeros.
- `ones(n: number): Vector`: Creates a vector of `n` ones.

### Expression Evaluation

- `eval(expr: string): string`: Evaluates a SOC (String-Oriented Calculation) language expression and returns the result as a string. This is a powerful feature for executing complex logic in a single WASM call.
- `resetEvaluator(): void`: Resets the state of the SOC evaluator, clearing any declared variables.

### Memory Debugging

- `getMemoryStats()`: Returns statistics about allocated, freed, and active memory handles.
- `gc(): number`: Forces garbage collection of any dead handles.
- `getActiveValuesCount(): number`: Returns the number of currently active (non-disposed) values.

### Mathematical Constants

- `PI`, `E`, `SQRT2`, `LN2`, `LN10`: Read-only properties for common mathematical constants.

---

## Value Classes

### Value (Base Class)

Abstract base class for all data types.

- `handle: Handle`: The internal WASM memory handle.
- `isDisposed: boolean`: Returns `true` if the value has been disposed.
- `dispose(): void`: Releases the WASM memory associated with this value.
- `metadata`: Provides metadata about the value's creation.

### Vector

Represents a 1D array of numbers.

- `length: number`: The number of elements in the vector.
- `data: Float64Array`: A **zero-copy** view of the underlying WASM memory.
- `get(index: number): number`: Returns the element at a specific index.
- `set(index: number, value: number): void`: Sets the element at a specific index.
- `toArray(): number[]`: Returns a **copy** of the vector data as a standard JavaScript array.
- `map(fn): Vector`: Creates a new vector by applying a function to each element.
- `filter(fn): Vector`: Creates a new vector with elements that pass the predicate.
- `reduce(fn, initialValue): T`: Reduces the vector to a single value.
- `[Symbol.iterator]()`: Allows for `for...of` iteration.

### Matrix

Represents a 2D array of numbers (row-major).

- `rows: number`: The number of rows.
- `cols: number`: The number of columns.
- `data: Float64Array`: A **zero-copy** view of the underlying flattened (row-major) WASM memory.
- `get(row: number, col: number): number`: Returns the element at a specific `(row, col)`.
- `set(row: number, col: number, value: number): void`: Sets the element at a specific `(row, col)`.
- `row(index: number): Vector`: Returns a specific row as a new `Vector` (copy).
- `col(index: number): Vector`: Returns a specific column as a new `Vector` (copy).
- `toArray(): number[][]`: Returns a **copy** of the matrix data as a 2D JavaScript array.
- `toFlatArray(): number[]`: Returns a **copy** of the flattened matrix data.
- `[Symbol.iterator]()`: Iterates over the rows of the matrix, yielding a `Vector` for each.

### Scalar

Represents a single number value.

- `value: number`: The numeric value.
- `toNumber(): number`: Returns the numeric value.
- `valueOf(): number`: Allows for numeric coercion (e.g., `+scalar`).

### Complex

Represents a complex number `z = a + bi`.

- `re: number`: The real part.
- `im: number`: The imaginary part.
- `magnitude: number`: The magnitude `|z|`.
- `phase: number`: The phase (angle) in radians.
- `conjugate(): Complex`: Returns the complex conjugate.
- `toArray(): number[]`: Returns `[re, im]`.

---

## Operation Modules

### MathOps (`ach.math`)

Basic mathematical functions. All functions accept `number` or `Value` types.

- `sin(x)`, `cos(x)`, `tan(x)`
- `asin(x)`, `acos(x)`, `atan(x)`, `atan2(y, x)`
- `sinh(x)`, `cosh(x)`, `tanh(x)`
- `exp(x)` (e^x)
- `ln(x)`, `log(x)` (natural log), `log10(x)`, `log2(x)`
- `sqrt(x)`, `cbrt(x)` (cube root)
- `abs(x)`
- `pow(base, exponent)`
- `floor(x)`, `ceil(x)`, `round(x)`, `trunc(x)`
- `sign(x)`

### DSPOps (`ach.dsp`)

Digital Signal Processing functions.

- `fft(signal: Vector): Matrix`: Computes the FFT, returning a complex spectrum as an `[N x 2]` matrix.
- `ifft(spectrum: Matrix): Vector`: Computes the Inverse FFT.
- `fftMag(signal: Vector | Matrix): Vector`: Computes the magnitude of the FFT spectrum.
- `fftPhase(signal: Vector | Matrix): Vector`: Computes the phase of the FFT spectrum.
- `conv(signal1: Vector, signal2: Vector): Vector`: Linear convolution.
- `hanning(n: number): Vector`: Creates a Hanning window.
- `hamming(n: number): Vector`: Creates a Hamming window.
- `blackman(n: number): Vector`: Creates a Blackman window.
- `fftshift(spectrum: Vector): Vector`: Shifts the zero-frequency component to the center.
- `ifftshift(spectrum: Vector): Vector`: Reverses `fftshift`.

### LinalgOps (`ach.linalg`)

Linear Algebra functions.

- `lu(matrix: Matrix): LUResult`: LU decomposition. Returns `{ L, U, P }`.
- `qr(matrix: Matrix): QRResult`: QR decomposition. Returns `{ Q, R }`.
- `svd(matrix: Matrix): SVDResult`: Singular Value Decomposition. Returns `{ U, S, V }`.
- `det(matrix: Matrix): number`: Computes the determinant.
- `inverse(matrix: Matrix): Matrix`: Computes the matrix inverse.
- `transpose(matrix: Matrix): Matrix`: Transposes the matrix.
- `identity(n: number): Matrix`: Creates an `n x n` identity matrix.

### VectorOps (`ach.vecOps`)

Element-wise and other vector-specific operations.

- `vadd(v1: Vector, v2: Vector): Vector`: Element-wise addition.
- `vsub(v1: Vector, v2: Vector): Vector`: Element-wise subtraction.
- `vmul(v1: Vector, v2: Vector): Vector`: Element-wise multiplication (Hadamard product).
- `vdiv(v1: Vector, v2: Vector): Vector`: Element-wise division.
- `vscale(vector: Vector, scalar: number): Vector`: Multiplies a vector by a scalar.
- `dot(v1: Vector, v2: Vector): number`: Dot product.
- `cross(v1: Vector, v2: Vector): Vector`: Cross product (for 3D vectors).
- `norm(vector: Vector): number`: L2 (Euclidean) norm.
- `normL1(vector: Vector): number`: L1 (Manhattan) norm.
- `normalize(vector: Vector): Vector`: Normalizes the vector to unit length.

### StatsOps (`ach.stats`)

Statistical functions.

- `sum(vector: Vector): number`: Sum of all elements.
- `mean(vector: Vector): number`: Mean (average).
- `std(vector: Vector, ddof?: number): number`: Standard deviation (ddof=0 for population, 1 for sample).
- `variance(vector: Vector, ddof?: number): number`: Variance.
- `min(vector: Vector): number`: Minimum value.
- `max(vector: Vector): number`: Maximum value.
- `argmin(vector: Vector): number`: Index of the minimum value.
- `argmax(vector: Vector): number`: Index of the maximum value.
- `median(vector: Vector): number`: Median value.
- `percentile(vector: Vector, p: number): number`: Computes the p-th percentile.
- `cov(v1: Vector, v2: Vector): number`: Covariance.
- `corr(v1: Vector, v2: Vector): number`: Correlation coefficient.

### HOFOps (`ach.hof`)

Higher-Order Functions.

- `map(fn, vector: Vector): Vector`: Applies a function to each element.
- `filter(fn, vector: Vector): Vector`: Filters elements based on a predicate.
- `reduce(fn, initialValue, vector: Vector): T`: Reduces a vector to a single value.
- `pipe(fns: Function[]): Function`: Left-to-right function composition.
- `compose(fns: Function[]): Function`: Right-to-left function composition.