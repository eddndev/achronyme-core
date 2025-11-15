# achronyme-linalg

Advanced Linear Algebra module for Achronyme.

## 🎯 Responsibility

The `achronyme-linalg` crate provides comprehensive linear algebra operations for the Achronyme language, including:

- **Matrix Decompositions**: LU, QR, Cholesky, and SVD (Singular Value Decomposition)
- **Eigenvalue Computations**: Eigenvalues, eigenvectors, and specialized algorithms (Power Iteration, QR Algorithm)
- **Linear System Solvers**: Direct solution methods, matrix inversion, determinant computation
- **Matrix Analysis**: Symmetry checking, positive-definiteness testing

This crate serves as the numerical linear algebra backend for Achronyme, powered by the high-performance **faer** library (100% Rust, WASM-compatible). It bridges the gap between Achronyme's tensor types and advanced matrix operations required for scientific computing, optimization, and machine learning applications.

## 📦 Dependencies

### External Crates
- **faer** (v0.19): Modern, high-performance linear algebra library written in 100% Rust
  - WASM-compatible (no C/Fortran dependencies)
  - Excellent performance rivaling LAPACK/BLAS
  - Safe, memory-efficient implementations
- **num-traits** (v0.2): Generic numeric traits for abstraction
- **num-complex** (v0.4): Complex number arithmetic
- **approx** (v0.5, dev): Floating-point comparison utilities for tests

### Internal Crates
- **achronyme-types**: Provides `RealTensor`, `ComplexTensor`, and `Complex` types

### Migration from nalgebra to faer

This crate was migrated from `nalgebra` to `faer` to achieve:
1. **WASM Compatibility**: Pure Rust implementation with no native dependencies
2. **Better Performance**: faer's modern algorithms match or exceed LAPACK performance
3. **Type Safety**: Stronger compile-time guarantees and better error handling
4. **Future-Proof**: Active development and excellent SIMD optimization

## 🔌 Used By

- **achronyme-eval**: Exposes linear algebra functions to the SOC language through the matrix function module
- **achronyme-solver**: May use matrix operations for advanced optimization algorithms
- **achronyme-numerical**: Could leverage decompositions for numerical stability

## 🏗️ High-Level Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    achronyme-linalg                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │Decompositions│  │ Eigenvalues  │  │   Solvers    │      │
│  │              │  │              │  │              │      │
│  │ • LU         │  │ • Standard   │  │ • Inverse    │      │
│  │ • QR         │  │ • Power Iter │  │ • Solve Ax=b │      │
│  │ • Cholesky   │  │ • QR Algo    │  │ • Determinant│      │
│  │ • SVD        │  │ • Symmetric  │  │ • Checks     │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │              │
│         └──────────────────┼──────────────────┘              │
│                            │                                 │
│  ┌─────────────────────────▼──────────────────────────────┐  │
│  │         Tensor Conversion Layer (RealTensor ↔ faer)   │  │
│  └─────────────────────────┬──────────────────────────────┘  │
│                            │                                 │
│  ┌─────────────────────────▼──────────────────────────────┐  │
│  │            achronyme-types (RealTensor)               │  │
│  └─────────────────────────┬──────────────────────────────┘  │
│                            │                                 │
│  ┌─────────────────────────▼──────────────────────────────┐  │
│  │            faer (Linear Algebra Engine)               │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Module Organization

1. **decompositions.rs**: Matrix factorization algorithms (LU, QR, Cholesky, SVD)
2. **eigenvalues.rs**: Eigenvalue and eigenvector computations
3. **solvers.rs**: Linear system solving, inversion, determinants, matrix properties
4. **lib.rs**: Public API exports and module declarations

## 🚀 Usage Examples

### SOC Language Examples

#### Matrix Decompositions
```soc
// LU Decomposition with partial pivoting
let A = [[2, 1, 1],
         [4, 3, 3],
         [8, 7, 9]]

let result = lu(A)           // Returns {L, U, P}
let L = result.L             // Lower triangular
let U = result.U             // Upper triangular
let P = result.P             // Permutation vector
```

#### QR Decomposition
```soc
// Orthogonal-triangular factorization
let A = [[1, 1],
         [1, 2],
         [1, 3]]

let result = qr(A)           // Returns {Q, R}
let Q = result.Q             // Orthogonal matrix
let R = result.R             // Upper triangular

// Verify: A ≈ Q * R
let reconstructed = Q @ R
```

#### Cholesky Decomposition
```soc
// For symmetric positive-definite matrices
let A = [[4, 2, 1],
         [2, 3, 1],
         [1, 1, 2]]

let L = cholesky(A)          // A = L * L^T
let LT = transpose(L)
let reconstructed = L @ LT    // Should equal A
```

#### Singular Value Decomposition
```soc
// SVD: A = U * Σ * V^T
let A = [[1, 2],
         [3, 4],
         [5, 6]]

let result = svd(A)
let U = result.U              // Left singular vectors (3x2)
let S = result.S              // Singular values (vector)
let VT = result.VT            // Right singular vectors transposed (2x2)

// Reconstruct matrix
let Sigma = diag(S)           // Convert to diagonal matrix
let reconstructed = U @ Sigma @ VT
```

#### Eigenvalue Computation
```soc
// Compute eigenvalues and eigenvectors
let A = [[4, 1],
         [2, 3]]

let eigenvals = eigenvalues(A)         // Returns complex eigenvalues
let result = eigenvectors(A)           // Returns {eigenvalues, eigenvectors}

// Each column of eigenvectors is an eigenvector
let lambda1 = result.eigenvalues[0]
let v1 = result.eigenvectors[:, 0]

// Verify: A * v1 ≈ lambda1 * v1
```

#### Solving Linear Systems
```soc
// Solve Ax = b
let A = [[3, 1],
         [1, 2]]
let b = [9, 8]

let x = solve(A, b)           // x = [2, 3]

// Verify solution
let b_check = A @ x           // Should equal b
```

#### Matrix Inversion
```soc
// Compute multiplicative inverse
let A = [[4, 7],
         [2, 6]]

let A_inv = inv(A)

// Verify: A * A_inv = I
let I_check = A @ A_inv       // Should be identity matrix
let det_A = det(A)            // Determinant: 10
```

#### Matrix Properties
```soc
// Check matrix characteristics
let A = [[4, 2, 1],
         [2, 3, 1],
         [1, 1, 2]]

let is_sym = is_symmetric(A, 1e-10)         // true
let is_pos_def = is_positive_definite(A)    // true (can use Cholesky)

// For non-symmetric matrix
let B = [[1, 2],
         [3, 4]]

let is_sym_B = is_symmetric(B, 1e-10)       // false
```

### Rust API Examples

#### Using Decompositions
```rust
use achronyme_linalg::{lu_decomposition, qr_decomposition, cholesky_decomposition};
use achronyme_types::tensor::RealTensor;

// LU Decomposition
let a = RealTensor::matrix(3, 3, vec![
    2.0, 1.0, 1.0,
    4.0, 3.0, 3.0,
    8.0, 7.0, 9.0
]).unwrap();

let (l, u, p) = lu_decomposition(&a).unwrap();
// l: lower triangular, u: upper triangular, p: permutation

// QR Decomposition
let a = RealTensor::matrix(3, 2, vec![
    1.0, 1.0,
    1.0, 2.0,
    1.0, 3.0
]).unwrap();

let (q, r) = qr_decomposition(&a).unwrap();
// q: orthogonal, r: upper triangular

// Cholesky Decomposition (symmetric positive-definite)
let a = RealTensor::matrix(3, 3, vec![
    4.0, 2.0, 1.0,
    2.0, 3.0, 1.0,
    1.0, 1.0, 2.0
]).unwrap();

let l = cholesky_decomposition(&a).unwrap();
// a = l * l^T
```

#### Eigenvalue Computations
```rust
use achronyme_linalg::{eigenvalues, eigenvectors, power_iteration};
use achronyme_types::tensor::RealTensor;

// Standard eigenvalue decomposition
let a = RealTensor::matrix(2, 2, vec![
    4.0, 1.0,
    2.0, 3.0
]).unwrap();

let eigs = eigenvalues(&a).unwrap();
// Returns Vec<Complex> with potentially complex eigenvalues

let (eigs, vecs) = eigenvectors(&a).unwrap();
// vecs is a matrix where each column is an eigenvector

// Power iteration for dominant eigenvalue
let (lambda, v) = power_iteration(&a, 1000, 1e-10).unwrap();
// lambda: largest eigenvalue by magnitude
// v: corresponding eigenvector
```

#### Solving Systems and Matrix Analysis
```rust
use achronyme_linalg::{solve_system, inverse, determinant_nd, is_symmetric};
use achronyme_types::tensor::RealTensor;

// Solve Ax = b
let a = RealTensor::matrix(2, 2, vec![
    3.0, 1.0,
    1.0, 2.0
]).unwrap();
let b = RealTensor::vector(vec![9.0, 8.0]);

let x = solve_system(&a, &b).unwrap();
// x = [2.0, 3.0]

// Matrix inverse
let a_inv = inverse(&a).unwrap();

// Determinant
let det = determinant_nd(&a).unwrap();

// Check properties
let is_sym = is_symmetric(&a, 1e-10);
```

## 📊 Key Algorithms Provided

### Matrix Decompositions

#### LU Decomposition with Partial Pivoting
- **Purpose**: Factorize A into lower and upper triangular matrices
- **Formula**: P·A = L·U
- **Complexity**: O(n³)
- **Use Cases**:
  - Solving linear systems efficiently
  - Computing determinants
  - Matrix inversion
- **Algorithm**: Gaussian elimination with row pivoting
- **Stability**: Numerically stable with partial pivoting

#### QR Decomposition
- **Purpose**: Factorize A into orthogonal and upper triangular matrices
- **Formula**: A = Q·R
- **Complexity**: O(mn²) for m×n matrix (m ≥ n)
- **Use Cases**:
  - Least squares problems
  - Eigenvalue computation (QR algorithm)
  - Orthogonalization
- **Algorithm**: Householder reflections (in faer)
- **Stability**: Excellent numerical stability

#### Cholesky Decomposition
- **Purpose**: Factorize symmetric positive-definite matrix
- **Formula**: A = L·L^T
- **Complexity**: O(n³/3) - faster than LU
- **Use Cases**:
  - Efficient linear system solving for SPD matrices
  - Monte Carlo simulations
  - Optimization (Hessian matrices)
- **Algorithm**: Modified Gaussian elimination exploiting symmetry
- **Stability**: Stable for well-conditioned SPD matrices
- **Note**: Fails if matrix is not positive-definite

#### SVD (Singular Value Decomposition)
- **Purpose**: Factorize any matrix into singular values and vectors
- **Formula**: A = U·Σ·V^T
- **Complexity**: O(min(m²n, mn²))
- **Use Cases**:
  - Principal Component Analysis (PCA)
  - Low-rank approximations
  - Pseudoinverse computation
  - Condition number estimation
- **Algorithm**: Two-phase approach (bidiagonalization + QR iteration)
- **Stability**: Most stable decomposition available

### Eigenvalue Algorithms

#### Standard Eigenvalue Decomposition
- **Purpose**: Find eigenvalues and eigenvectors
- **Formula**: A·v = λ·v
- **Complexity**: O(n³)
- **Method**: QR algorithm with implicit shifts (faer implementation)
- **Output**: Complex eigenvalues (real or conjugate pairs)

#### Power Iteration
- **Purpose**: Find dominant (largest magnitude) eigenvalue
- **Complexity**: O(n²) per iteration, typically converges in O(log(1/ε)) iterations
- **Use Cases**:
  - PageRank algorithm
  - Finding spectral radius
  - When only largest eigenvalue needed
- **Convergence**: Linear, rate depends on eigenvalue gap |λ₁/λ₂|

#### QR Algorithm
- **Purpose**: Compute all eigenvalues iteratively
- **Complexity**: O(n³) total
- **Method**: Iterative QR decomposition: A_{k+1} = R_k·Q_k
- **Convergence**: Diagonal elements converge to eigenvalues

#### Symmetric Eigenvalue Decomposition
- **Purpose**: Specialized algorithm for symmetric matrices
- **Output**: Real eigenvalues, orthogonal eigenvectors
- **Benefits**: Faster and more stable than general case
- **Future**: Will be optimized with specialized symmetric algorithms

### Linear System Solvers

#### Direct Solution (LU-based)
- **Purpose**: Solve Ax = b exactly (within numerical precision)
- **Method**: LU decomposition + forward/backward substitution
- **Complexity**: O(n³) for decomposition, O(n²) for substitution
- **Accuracy**: Limited by machine precision and condition number

#### Matrix Inverse
- **Purpose**: Compute A^(-1)
- **Method**: Solve A·X = I using LU decomposition
- **Complexity**: O(n³)
- **Note**: Direct solving Ax = b is often more efficient than computing A^(-1)·b

#### Determinant Computation
- **Purpose**: Compute det(A)
- **Method**: LU decomposition + product of diagonal elements
- **Complexity**: O(n³)
- **Sign**: Adjusted for permutation parity

## 🔬 Mathematical Foundations

### Matrix Theory Concepts

#### Vector Spaces and Linear Independence
- **Span**: Set of all linear combinations of vectors
- **Basis**: Linearly independent set that spans the space
- **Dimension**: Number of vectors in a basis
- **Rank**: Dimension of column space = dimension of row space

#### Matrix Norms and Conditioning
- **Frobenius Norm**: ||A||_F = √(Σ a_{ij}²)
- **Spectral Norm**: ||A||_2 = largest singular value
- **Condition Number**: κ(A) = ||A|| · ||A^(-1)||
  - Well-conditioned: κ(A) ≈ 1
  - Ill-conditioned: κ(A) >> 1
  - Singular: κ(A) = ∞

#### Matrix Properties
- **Symmetric**: A = A^T
- **Orthogonal**: Q^T·Q = I (preserves norms and angles)
- **Positive Definite**: x^T·A·x > 0 for all x ≠ 0
  - All eigenvalues positive
  - Admits Cholesky decomposition
- **Hermitian**: A = Ā^T (complex analog of symmetric)

### Numerical Stability Considerations

#### Sources of Numerical Error
1. **Rounding Error**: Limited precision (machine epsilon ≈ 2.22×10^(-16) for f64)
2. **Cancellation**: Subtraction of nearly equal numbers
3. **Overflow/Underflow**: Values outside representable range
4. **Accumulation**: Error growth through iterative algorithms

#### Stability Strategies
1. **Pivoting**: Swap rows/columns to avoid small pivot elements
   - Partial pivoting: O(n²) overhead, good practical stability
   - Full pivoting: O(n³) overhead, theoretical stability
2. **Orthogonal Transformations**: Preserve norms (QR, Householder)
3. **Scaling**: Normalize matrices to improve conditioning
4. **Iterative Refinement**: Improve solution accuracy post-computation

#### Algorithm Stability Rankings
1. **Most Stable**: SVD, QR decomposition
2. **Stable with Pivoting**: LU with partial pivoting
3. **Conditionally Stable**: Cholesky (requires SPD matrix)
4. **Potentially Unstable**: Direct Gaussian elimination (no pivoting)

### Complexity Analysis

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Matrix Multiplication (n×n) | O(n³) | Can be reduced to ~O(n^2.37) with Strassen |
| LU Decomposition | O(n³) | 2n³/3 flops |
| QR Decomposition | O(n³) | 2n³ flops for square matrices |
| Cholesky Decomposition | O(n³/3) | n³/3 flops (half of LU) |
| SVD | O(n³) | ~11n³ flops |
| Eigenvalue (general) | O(n³) | Iterative, depends on spectrum |
| Forward/Backward Substitution | O(n²) | |
| Matrix Inverse | O(n³) | Same as LU decomposition |
| Determinant | O(n³) | Via LU decomposition |

### When to Use Each Decomposition

```
Decomposition Selection Guide
═══════════════════════════════════════════════════════════

Problem: Solve Ax = b
├─ A is symmetric positive definite → Cholesky
├─ A is symmetric → LU with symmetric pivoting
├─ Need to solve for multiple b → LU (reuse decomposition)
└─ General case → LU with partial pivoting

Problem: Least Squares (minimize ||Ax - b||)
└─ QR decomposition or SVD

Problem: Eigenvalues/Eigenvectors
├─ Only dominant eigenvalue → Power Iteration
├─ Symmetric matrix → Symmetric eigendecomposition
└─ General matrix → Standard eigendecomposition

Problem: Matrix Rank/Nullspace
└─ SVD (most reliable)

Problem: Condition Number
└─ SVD (σ_max / σ_min)

Problem: Low-rank Approximation
└─ SVD truncation

Problem: Orthogonalization
└─ QR decomposition
```

## 🧪 Testing

### Run All Tests
```bash
cargo test -p achronyme-linalg
```

### Run with Output
```bash
cargo test -p achronyme-linalg -- --nocapture
```

### Test Specific Module
```bash
cargo test -p achronyme-linalg --lib decompositions
cargo test -p achronyme-linalg --lib eigenvalues
cargo test -p achronyme-linalg --lib solvers
```

### Test Coverage
The test suite includes:
- Basic functionality tests for each algorithm
- Edge cases (singular matrices, non-square matrices, etc.)
- Numerical accuracy verification using `approx` crate
- Reconstruction tests (verifying A = L·U, A = Q·R, etc.)
- Error handling (invalid inputs, dimension mismatches)

## 📈 Performance Characteristics

### faer vs nalgebra vs LAPACK
The `faer` library provides:
- **Competitive Performance**: Matches or exceeds LAPACK for many operations
- **Better SIMD Utilization**: Modern vectorization strategies
- **Zero-Cost Abstractions**: No runtime overhead from Rust abstractions
- **WASM Compatibility**: Works in browser environments (unlike LAPACK bindings)

### Optimization Tips
1. **Reuse Decompositions**: If solving Ax = b for multiple b, compute LU once
2. **Choose Right Algorithm**: Cholesky is 2× faster than LU for SPD matrices
3. **Avoid Unnecessary Inversions**: Solve Ax = b directly instead of x = A^(-1)·b
4. **Consider Sparsity**: faer has sparse matrix support (not yet exposed in achronyme-linalg)
5. **Batch Operations**: Process multiple matrices together when possible

### Memory Usage
| Operation | Memory Overhead | Notes |
|-----------|----------------|-------|
| LU Decomposition | O(n²) | Stores L and U |
| QR Decomposition | O(n²) | Stores Q and R |
| SVD | O(n²) | Stores U, Σ, V^T |
| Eigendecomposition | O(n²) | Stores eigenvalues + eigenvectors |
| In-place Operations | O(1) | faer supports some in-place ops |

## 🔗 Related Crates

- **achronyme-types**: Core tensor and complex number types
- **achronyme-eval**: Evaluator that exposes linalg functions to SOC
- **achronyme-solver**: Optimization algorithms (may use linalg internally)
- **achronyme-numerical**: Numerical methods (differentiation, integration)
- **achronyme-dsp**: Digital signal processing (FFT, convolution)

## 📚 References

### Textbooks
1. **Golub & Van Loan**: "Matrix Computations" (4th ed.) - The definitive reference
2. **Trefethen & Bau**: "Numerical Linear Algebra" - Excellent for understanding stability
3. **Demmel**: "Applied Numerical Linear Algebra" - Comprehensive modern treatment
4. **Horn & Johnson**: "Matrix Analysis" - Deep theoretical treatment

### Papers and Resources
- Higham, N. J.: "Accuracy and Stability of Numerical Algorithms"
- Wilkinson, J. H.: "The Algebraic Eigenvalue Problem"
- LAPACK documentation: https://netlib.org/lapack/
- faer documentation: https://docs.rs/faer/

### Online Resources
- Matrix Cookbook: https://www.math.uwaterloo.ca/~hwolkowi/matrixcookbook.pdf
- Numerical Linear Algebra course notes (Trefethen): https://people.maths.ox.ac.uk/trefethen/

## 🎓 Educational Notes

### Understanding Matrix Decompositions

**Why decompose matrices?**
1. **Numerical Stability**: Well-conditioned subproblems
2. **Efficiency**: Solve related problems faster
3. **Insight**: Reveal structure (rank, conditioning, eigenvalues)
4. **Generality**: Handle edge cases (singular, rectangular matrices)

**Visual Intuition**:
```
LU Decomposition:
    ┌     ┐       ┌     ┐   ┌     ┐
    │ a b │   =   │ 1 0 │ × │ u v │
    │ c d │       │ l 1 │   │ 0 w │
    └     ┘       └     ┘   └     ┘
     Original      Lower     Upper

QR Decomposition:
    ┌     ┐       ┌     ┐   ┌     ┐
    │ →  │   =   │ ↑ ↑ │ × │ ╲   │
    │ →  │       │ ↑ ↑ │   │   ╲ │
    │ →  │       │ ↑ ↑ │   └     ┘
    └     ┘       └     ┘   Triangular
     Original    Orthogonal

SVD:
    ┌     ┐       ┌     ┐   ┌   ┐   ┌     ┐
    │ →  │   =   │ ↑ ↑ │ × │ σ │ × │ ↔ ↔ │
    │ →  │       │ ↑ ↑ │   └   ┘   └     ┘
    │ →  │       │ ↑ ↑ │   Singular   Right
    └     ┘       └     ┘   Values    Vectors
     Original     Left
                 Vectors
```

### Common Pitfalls and Solutions

1. **Computing A^(-1) explicitly**
   - ❌ Bad: `x = inv(A) @ b`
   - ✅ Good: `x = solve(A, b)`
   - Why: Solving directly is faster and more accurate

2. **Using wrong decomposition**
   - ❌ Bad: SVD for solving well-conditioned square systems
   - ✅ Good: LU for general, Cholesky for SPD matrices
   - Why: SVD is slower, use it when you need its special properties

3. **Ignoring condition number**
   - ❌ Bad: Trusting results without checking κ(A)
   - ✅ Good: Estimate condition number, use regularization if needed
   - Why: Ill-conditioned matrices amplify errors exponentially

4. **Not checking matrix properties**
   - ❌ Bad: Assuming matrix is invertible
   - ✅ Good: Check determinant ≠ 0 or handle errors gracefully
   - Why: Singular matrices cause division by zero

## 🚧 Future Enhancements

### Planned Features
- [ ] Sparse matrix support (CSR/CSC formats)
- [ ] Iterative solvers (Conjugate Gradient, GMRES)
- [ ] Parallel matrix operations
- [ ] Complex matrix eigendecomposition
- [ ] Generalized eigenvalue problems (Ax = λBx)
- [ ] Matrix exponential and logarithm
- [ ] Schur decomposition
- [ ] Hessenberg reduction

### Performance Improvements
- [ ] Cache-friendly memory layouts
- [ ] Explicit SIMD optimizations
- [ ] GPU acceleration (via wgpu)
- [ ] Multi-threaded decompositions

### API Enhancements
- [ ] Builder patterns for fine-tuned algorithms
- [ ] Streaming/incremental decompositions
- [ ] Better error messages with recovery suggestions
- [ ] Integration with automatic differentiation

---

**Version**: 0.1.0
**License**: Same as Achronyme project
**Maintainer**: Achronyme Project Team
