# Type Definitions - Achronyme SDK v2.0

This document outlines the shared TypeScript types, interfaces, and custom error classes used throughout the SDK.

---

## Core Types

### `Handle`

A `Handle` is a numeric identifier (a `number`) that acts as a pointer to a value stored in the WASM memory. This is an internal concept, but it's useful to understand that every `Value` object is managed via its handle.

```typescript
export type Handle = number;
```

### `HandleStats`

This interface defines the structure of the object returned by `ach.getMemoryStats()`, used for debugging memory usage.

```typescript
export interface HandleStats {
    /** Total handles allocated since initialization. */
    allocated: number;

    /** Total handles freed via dispose() or use() blocks. */
    freed: number;

    /** Currently active (non-disposed) handles. */
    active: number;

    /** Should be 0 in this memory model. */
    leaked: number;
}
```

---

## Result Types for Decompositions

These interfaces define the return types for linear algebra decomposition methods in `LinalgOps`.

### `LUResult`

Result of LU decomposition (`linalg.lu`).

```typescript
import type { Matrix } from '../values/Matrix';

export interface LUResult {
    L: Matrix; // Lower triangular matrix
    U: Matrix; // Upper triangular matrix
    P: Matrix; // Permutation matrix
}
```

### `QRResult`

Result of QR decomposition (`linalg.qr`).

```typescript
import type { Matrix } from '../values/Matrix';

export interface QRResult {
    Q: Matrix; // Orthogonal matrix
    R: Matrix; // Upper triangular matrix
}
```

### `SVDResult`

Result of Singular Value Decomposition (`linalg.svd`).

```typescript
import type { Matrix } from '../values/Matrix';
import type { Vector } from '../values/Vector';

export interface SVDResult {
    U: Matrix; // Left singular vectors
    S: Vector; // Singular values as a vector
    V: Matrix; // Right singular vectors
}
```

### `CholeskyResult`

Result of Cholesky decomposition (`linalg.cholesky`).

```typescript
import type { Matrix } from '../values/Matrix';

export interface CholeskyResult {
    L: Matrix; // Lower triangular matrix
}
```

### `EigenResult`

Result of eigenvalue decomposition (`linalg.eig`).

```typescript
import type { Matrix } from '../values/Matrix';
import type { Vector } from '../values/Vector';

export interface EigenResult {
    values: Vector;  // Eigenvalues
    vectors: Matrix; // Eigenvectors as columns
}
```

---

## Custom Error Types

The SDK throws custom, typed errors to allow for specific error handling. All custom errors inherit from `AchronymeError`.

### `AchronymeError`

The base class for all SDK-specific errors.

```typescript
export class AchronymeError extends Error {
  constructor(message: string, public readonly code?: string);
}
```

### `AchronymeNotInitializedError`

Thrown when the SDK is used before `ach.init()` has been called.

-   **code**: `'NOT_INITIALIZED'`

### `AchronymeDisposedError`

Thrown when an operation is attempted on a `Value` that has already been disposed.

-   **code**: `'DISPOSED_ERROR'`

### `AchronymeArgumentError`

Thrown when a function is called with invalid arguments (e.g., wrong type, incorrect dimensions).

-   **code**: `'ARGUMENT_ERROR'`

### `AchronymeSyntaxError`

Thrown by the `eval` engine for expressions with incorrect syntax.

-   **code**: `'SYNTAX_ERROR'`
-   **expression**: The expression string that caused the error.

### `AchronymeRuntimeError`

Thrown for errors that occur during evaluation (e.g., division by zero).

-   **code**: `'RUNTIME_ERROR'`
-   **expression**: The expression string being evaluated.

### `AchronymeTypeError`

Thrown for type mismatches during an operation.

-   **code**: `'TYPE_ERROR'`

### Error Handling Example

```typescript
try {
    await ach.use(() => {
        const v = ach.vector([-1, 4]);
        const result = ach.math.sqrt(v); // Fails on -1
    });
} catch (error) {
    if (error instanceof AchronymeRuntimeError) {
        console.error('A runtime error occurred:', error.message);
    } else if (error instanceof AchronymeError) {
        console.error(`An Achronyme error occurred [${error.code}]:`, error.message);
    } else {
        console.error('An unexpected error occurred:', error);
    }
}
```