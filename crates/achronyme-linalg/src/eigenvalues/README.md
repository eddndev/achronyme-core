# Eigenvalue Computation Module

Comprehensive eigenvalue and eigenvector algorithms for the `achronyme-linalg` crate.

**Location**: `crates/achronyme-linalg/src/eigenvalues.rs`
**Lines of Code**: 312
**Target Audience**: Contributors implementing or extending eigenvalue algorithms

---

## 📋 Overview

This module provides five eigenvalue computation methods:

1. **Standard Eigenvalue Decomposition** - General matrices, complex eigenvalues
2. **Eigenvectors** - Complete eigenvalue/eigenvector decomposition
3. **Power Iteration** - Dominant eigenvalue (largest magnitude)
4. **QR Algorithm** - Iterative computation of all eigenvalues
5. **Symmetric Eigendecomposition** - Specialized for symmetric matrices (real eigenvalues)

All algorithms leverage the `faer` library's eigendecomposition capabilities, with some custom iterative implementations for educational and special-case purposes.

---

## 🏗️ Module Structure

```rust
eigenvalues.rs
│
├── Conversion Functions (Internal)
│   └── tensor_to_faer_mat(&RealTensor) -> Mat<f64>
│
├── Public Eigenvalue Functions
│   ├── eigenvalues(&RealTensor) -> Result<Vec<Complex>, String>
│   ├── eigenvectors(&RealTensor) -> Result<(Vec<Complex>, RealTensor), String>
│   ├── power_iteration(&RealTensor, usize, f64) -> Result<(f64, RealTensor), String>
│   ├── qr_eigenvalues(&RealTensor, usize, f64) -> Result<Vec<f64>, String>
│   └── eigen_symmetric(&RealTensor, usize, f64) -> Result<(Vec<f64>, RealTensor), String>
│
└── Tests
    ├── test_eigenvalues_2x2()
    ├── test_eigenvalues_3x3()
    ├── test_eigenvectors()
    └── test_eigenvalues_nonsquare_fails()
```

---

## 🔧 Conversion Functions

### `tensor_to_faer_mat`

**Purpose**: Convert Achronyme `RealTensor` to faer `Mat<f64>`

**Signature**:
```rust
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
```

**Implementation**: Same as in `decompositions.rs` (duplicated for module independence)

**Preconditions**: `tensor.is_matrix()` must be true (asserted)

---

## 🎯 Eigenvalue Algorithms

### 1. Standard Eigenvalue Decomposition

#### Mathematical Background

**Eigenvalue Equation**:
```
A·v = λ·v
```
where:
- **A**: n×n matrix
- **λ**: Eigenvalue (scalar, possibly complex)
- **v**: Eigenvector (non-zero vector)

**Characteristic Polynomial**:
```
det(A - λI) = 0
```
- Polynomial of degree n → n eigenvalues (counting multiplicities)
- Real matrices can have complex eigenvalues (conjugate pairs)

**Geometric Interpretation**:
- Eigenvectors are special directions that remain unchanged (except for scaling) under transformation A
- Eigenvalue is the scaling factor

#### Function Signature

```rust
pub fn eigenvalues(tensor: &RealTensor) -> Result<Vec<Complex>, String>
```

**Returns**: Vector of complex eigenvalues, length n

**Note**: Returns `Complex` (Achronyme type) even for real eigenvalues (imaginary part = 0)

#### Algorithm (faer implementation)

Uses `faer::eigendecomposition::<c64>()`:

1. **Hessenberg Reduction**: Reduce A to upper Hessenberg form (QR factorization)
2. **QR Algorithm with Shifts**: Iteratively diagonalize
   - Implicit double-shift QR (Francis algorithm)
   - Converges to Schur form (upper triangular or quasi-upper triangular)
3. **Extract Eigenvalues**: Diagonal of Schur form

**Complexity**:
- Time: O(n³) total (~10n³ flops)
- Space: O(n²) for Schur vectors

#### Numerical Properties

- **Stability**: Backward stable (computes exact eigenvalues of A + E where ||E|| ≈ ε||A||)
- **Accuracy**: Small eigenvalues computed with relative accuracy
- **Complex Eigenvalues**: Conjugate pairs for real matrices
- **Condition Number**: Eigenvalue sensitivity depends on eigenvector condition number

#### Use Cases

1. **Spectral Analysis**: Understand matrix behavior through eigenvalue spectrum
2. **Stability Analysis**: System stable iff all eigenvalues have negative real parts (for dynamical systems)
3. **Matrix Powers**: `A^k = V·Λ^k·V^(-1)` (if diagonalizable)
4. **Spectral Radius**: `ρ(A) = max|λ_i|`

#### Code Example

```rust
use achronyme_linalg::eigenvalues;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(2, 2, vec![
    4.0, 1.0,
    2.0, 3.0
]).unwrap();

let eigs = eigenvalues(&a).unwrap();

// Should have 2 eigenvalues
assert_eq!(eigs.len(), 2);

// For this matrix, eigenvalues are 5 and 2 (both real)
for eig in &eigs {
    println!("λ = {} + {}i", eig.re, eig.im);
}

// Check if eigenvalue is real
if eigs[0].im.abs() < 1e-10 {
    println!("Eigenvalue is real: {}", eigs[0].re);
}
```

#### Error Conditions

- **Non-square matrix**: `Err("Eigenvalue computation requires square matrix")`

---

### 2. Eigenvectors

#### Overview

Computes both eigenvalues and eigenvectors simultaneously.

**Mathematical Definition**:
```
A = V·Λ·V^(-1)
```
where:
- **V**: Matrix of eigenvectors (column i is eigenvector for λ_i)
- **Λ**: Diagonal matrix of eigenvalues

**Note**: Not all matrices are diagonalizable (defective matrices exist)

#### Function Signature

```rust
pub fn eigenvectors(tensor: &RealTensor)
    -> Result<(Vec<Complex>, RealTensor), String>
```

**Returns**: `(eigenvalues, eigenvector_matrix)` where:
- `eigenvalues`: Vec<Complex> of length n
- `eigenvector_matrix`: RealTensor n×n, column i is eigenvector for eigenvalue i

**Current Limitation**: Eigenvector matrix contains only real parts (complex eigenvectors truncated)

#### Algorithm

Same as `eigenvalues()` but also extracts eigenvectors:

1. Compute eigendecomposition: `A·V = V·Λ`
2. Extract eigenvalues from diagonal Λ
3. Extract eigenvectors from columns of V
4. Convert complex eigenvectors to real (take real part only)

**TODO**: Full complex eigenvector support

#### Properties of Eigenvectors

1. **Normalization**: faer returns normalized eigenvectors (||v_i|| = 1)
2. **Orthogonality**: Only guaranteed for symmetric matrices
3. **Sign Ambiguity**: If v is eigenvector, so is -v (arbitrary sign)

#### Code Example

```rust
use achronyme_linalg::eigenvectors;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(2, 2, vec![
    4.0, 1.0,
    2.0, 3.0
]).unwrap();

let (eigs, vecs) = eigenvectors(&a).unwrap();

// Verify: A·v_i ≈ λ_i·v_i
for i in 0..2 {
    let lambda = eigs[i].re;  // Assume real eigenvalue
    let v = extract_column(&vecs, i);  // Helper function

    let av = a.matmul(&v).unwrap();
    let lambda_v = v.scalar_mul(lambda);

    // Check if av ≈ lambda_v (within tolerance)
}
```

#### Error Conditions

- **Non-square matrix**: `Err("Eigenvector computation requires square matrix")`

---

### 3. Power Iteration

#### Overview

Iterative method to find the **dominant eigenvalue** (largest absolute value) and its eigenvector.

**Algorithm Principle**:
```
v_{k+1} = A·v_k / ||A·v_k||
```
Converges to eigenvector of largest eigenvalue (by magnitude).

#### Function Signature

```rust
pub fn power_iteration(
    tensor: &RealTensor,
    max_iterations: usize,
    tolerance: f64
) -> Result<(f64, RealTensor), String>
```

**Parameters**:
- `max_iterations`: Maximum number of iterations (e.g., 1000)
- `tolerance`: Convergence threshold for eigenvalue (e.g., 1e-10)

**Returns**: `(dominant_eigenvalue, corresponding_eigenvector)`

**Note**: Returns real eigenvalue only (magnitude of dominant eigenvalue)

#### Detailed Algorithm

**Initialization**:
```rust
v = [1, 0, 0, ..., 0]  // Start with simple vector
λ = 0
```

**Iteration Loop**:
```rust
for k = 0 to max_iterations:
    w = A·v                    // Matrix-vector multiplication
    λ_new = ||w||_2            // Norm as eigenvalue approximation
    v = w / ||w||_2            // Normalize

    if |λ_new - λ| < tolerance:
        return (λ_new, v)      // Converged

    λ = λ_new
```

#### Convergence Analysis

**Convergence Rate**:
```
error_k ≈ (|λ_2| / |λ_1|)^k · error_0
```
where λ_1 is dominant, λ_2 is second-largest eigenvalue.

**Implications**:
- **Fast Convergence**: When |λ_1| >> |λ_2| (well-separated eigenvalues)
- **Slow Convergence**: When |λ_1| ≈ |λ_2| (clustered eigenvalues)
- **No Convergence**: When |λ_1| = |λ_2| (multiple dominant eigenvalues)

**Typical Iterations**: O(log(1/ε)) for tolerance ε, if eigenvalue gap is good

#### Complexity

- **Per Iteration**: O(n²) for dense matrix-vector multiplication
- **Total**: O(k·n²) where k is number of iterations
- **Space**: O(n) for vectors

#### Use Cases

1. **PageRank Algorithm**: Google's original ranking (dominant eigenvector of web graph)
2. **Spectral Radius**: `ρ(A) = max|λ_i|`
3. **Perron-Frobenius**: Find dominant eigenvalue of non-negative matrices
4. **When Only Largest Needed**: Much faster than full eigendecomposition

#### Code Example

```rust
use achronyme_linalg::power_iteration;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(3, 3, vec![
    4.0, 1.0, 0.0,
    1.0, 3.0, 1.0,
    0.0, 1.0, 2.0
]).unwrap();

let (lambda, v) = power_iteration(&a, 1000, 1e-10).unwrap();

println!("Dominant eigenvalue: {}", lambda);
println!("Eigenvector: {:?}", v.data());

// lambda should be ≈ 5.0 (largest eigenvalue)
```

#### Limitations

1. **Only Largest**: Can't find other eigenvalues directly
2. **Sign Ambiguity**: Eigenvector sign arbitrary
3. **Convergence Issues**: Fails for multiple dominant eigenvalues
4. **Real Only**: Doesn't handle complex dominant eigenvalues well

**Variants** (not implemented):
- **Inverse Power Iteration**: Find smallest eigenvalue (use A^(-1))
- **Shifted Power Iteration**: Find eigenvalue near σ (use (A - σI)^(-1))
- **Rayleigh Quotient Iteration**: Cubic convergence (adaptive shift)

---

### 4. QR Algorithm (Iterative)

#### Overview

Custom iterative implementation of the QR algorithm for finding all eigenvalues.

**Algorithm Principle**:
```
A_0 = A
for k = 0, 1, 2, ...:
    A_k = Q_k·R_k          // QR decomposition
    A_{k+1} = R_k·Q_k      // Reverse multiplication
```
Converges to Schur form (diagonal or quasi-diagonal), revealing eigenvalues.

#### Function Signature

```rust
pub fn qr_eigenvalues(
    tensor: &RealTensor,
    max_iterations: usize,
    tolerance: f64
) -> Result<Vec<f64>, String>
```

**Parameters**:
- `max_iterations`: Maximum QR iterations (e.g., 1000)
- `tolerance`: Convergence threshold for off-diagonal elements (e.g., 1e-10)

**Returns**: Vec<f64> of real eigenvalues (imaginary parts discarded)

**Limitation**: Complex eigenvalues handled poorly (only real parts returned)

#### Detailed Algorithm

**Initialization**:
```rust
A = tensor.clone()
```

**Iteration Loop**:
```rust
for iter = 0 to max_iterations:
    (Q, R) = qr_decomposition(A)  // Use existing QR
    A = R·Q                        // Similarity transformation

    // Check convergence: off-diagonal elements → 0
    max_off_diag = max(|A[i,j]| for i ≠ j)
    if max_off_diag < tolerance:
        break                       // Converged

// Extract diagonal as eigenvalues
eigenvalues = [A[0,0], A[1,1], ..., A[n-1,n-1]]
```

#### Mathematical Properties

**Similarity Transformation**:
```
A_{k+1} = R_k·Q_k = Q_k^(-1)·A_k·Q_k = Q_k^T·A_k·Q_k
```
- Preserves eigenvalues (similar matrices have same eigenvalues)
- Orthogonal transformation (numerically stable)

**Convergence**:
- Diagonal elements → eigenvalues
- Off-diagonal elements → 0
- Rate depends on eigenvalue separation

#### Complexity

- **Per Iteration**: O(n³) for QR decomposition + O(n³) for R·Q multiplication
- **Total**: O(k·n³) where k is number of iterations
- **Typical k**: 5-30 iterations for most matrices

#### Use Cases

1. **Educational**: Understanding eigenvalue algorithms
2. **Small Matrices**: When full eigendecomposition is overkill
3. **Custom Convergence**: Control iteration parameters

**Note**: For production, use standard `eigenvalues()` (faer's optimized QR with shifts)

#### Code Example

```rust
use achronyme_linalg::qr_eigenvalues;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(3, 3, vec![
    4.0, 1.0, 0.0,
    1.0, 3.0, 1.0,
    0.0, 1.0, 2.0
]).unwrap();

let eigs = qr_eigenvalues(&a, 100, 1e-10).unwrap();

println!("Eigenvalues: {:?}", eigs);

// For symmetric matrix, eigenvalues are real
// eigs should be approximately [5.0, 3.0, 1.0]
```

#### Limitations

1. **No Shifts**: Plain QR without shifts (slow convergence)
2. **Complex Eigenvalues**: 2×2 blocks not handled (only diagonal extracted)
3. **Convergence**: May not converge for some matrices

**Improvements** (not implemented):
- **Hessenberg Preprocessing**: Reduce to Hessenberg first (cheaper iterations)
- **Shifts**: Use Wilkinson shift or Francis double-shift
- **Deflation**: Remove converged eigenvalues to accelerate

---

### 5. Symmetric Eigendecomposition

#### Overview

Specialized eigendecomposition for symmetric matrices.

**Mathematical Properties**:
- Symmetric matrix `A = A^T`
- **All eigenvalues are real**
- **Eigenvectors are orthogonal**: `V^T·V = I`
- Spectral decomposition: `A = V·Λ·V^T` (not V^(-1), just V^T)

#### Function Signature

```rust
pub fn eigen_symmetric(
    tensor: &RealTensor,
    max_iterations: usize,
    tolerance: f64
) -> Result<(Vec<f64>, RealTensor), String>
```

**Parameters**: Currently ignored (delegates to standard eigendecomposition)

**Returns**: `(eigenvalues, eigenvector_matrix)` where:
- `eigenvalues`: Vec<f64> (real only)
- `eigenvector_matrix`: Orthogonal matrix (V^T·V = I)

#### Current Implementation

**TODO**: Currently delegates to general `eigenvectors()` and extracts real parts.

**Planned**: Specialized symmetric algorithm (Jacobi, Divide-and-Conquer, or MRRR)

```rust
pub fn eigen_symmetric(...) -> Result<(Vec<f64>, RealTensor), String> {
    // Current implementation
    let (eigs, vecs) = eigenvectors(tensor)?;
    let real_eigs: Vec<f64> = eigs.iter().map(|c| c.re).collect();
    Ok((real_eigs, vecs))
}
```

#### Specialized Symmetric Algorithms (Future)

**1. Jacobi Algorithm**:
- Iteratively apply Givens rotations to diagonalize
- Very accurate, good for small dense matrices
- O(n³) per sweep, typically 5-10 sweeps

**2. Divide-and-Conquer**:
- Recursively divide matrix, solve subproblems, merge
- Faster than QR for large matrices
- O(n³) but with smaller constant

**3. MRRR (Multiple Relatively Robust Representations)**:
- State-of-the-art, O(n²) in many cases
- Used in LAPACK (dsyevr)

#### Why Specialize for Symmetric?

**Advantages**:
1. **Guaranteed Real**: No complex arithmetic needed
2. **Orthogonal Eigenvectors**: Numerical stability
3. **Faster Algorithms**: ~2× speedup over general case
4. **Better Accuracy**: Relative error guarantees

**Complexity Comparison**:
| Method | General QR | Symmetric QR | Divide-and-Conquer |
|--------|-----------|--------------|-------------------|
| Flops  | ~10n³     | ~4n³         | ~(4/3)n³          |

#### Code Example

```rust
use achronyme_linalg::eigen_symmetric;
use achronyme_types::tensor::RealTensor;

// Symmetric matrix
let a = RealTensor::matrix(3, 3, vec![
    4.0, 1.0, 0.0,
    1.0, 3.0, 1.0,
    0.0, 1.0, 2.0
]).unwrap();

// Verify symmetry
assert!(is_symmetric(&a, 1e-10));

let (eigs, vecs) = eigen_symmetric(&a, 100, 1e-10).unwrap();

// All eigenvalues should be real
for eig in &eigs {
    println!("λ = {}", eig);
}

// Verify orthogonality: V^T·V = I
let vt = vecs.transpose().unwrap();
let vtv = vt.matmul(&vecs).unwrap();
// vtv should be identity matrix
```

#### Error Conditions

- **Non-square matrix**: `Err("Symmetric eigendecomposition requires square matrix")`

**Best Practice**: Check symmetry before calling:
```rust
if !is_symmetric(&a, 1e-10) {
    return Err("Matrix is not symmetric".to_string());
}
```

---

## 🧪 Testing Strategy

### Current Tests

1. **Known Eigenvalues** (2×2 matrix):
   ```rust
   #[test]
   fn test_eigenvalues_2x2() {
       let a = RealTensor::matrix(2, 2, vec![
           4.0, 1.0,
           2.0, 3.0
       ]).unwrap();
       let eigs = eigenvalues(&a).unwrap();
       // Eigenvalues should be 5 and 2
       assert_relative_eq!(sorted_eigs[0], 2.0, epsilon = 1e-10);
       assert_relative_eq!(sorted_eigs[1], 5.0, epsilon = 1e-10);
   }
   ```

2. **Identity Matrix** (all eigenvalues = 1):
   ```rust
   #[test]
   fn test_eigenvalues_3x3() {
       let a = RealTensor::eye(3);
       let eigs = eigenvalues(&a).unwrap();
       for eig in eigs {
           assert_relative_eq!(eig.re, 1.0, epsilon = 1e-10);
       }
   }
   ```

3. **Error Handling**:
   ```rust
   #[test]
   fn test_eigenvalues_nonsquare_fails() {
       let a = RealTensor::matrix(2, 3, vec![...]).unwrap();
       assert!(eigenvalues(&a).is_err());
   }
   ```

### Future Test Enhancements

#### Eigenvector Verification
```rust
#[test]
fn test_eigenvector_property() {
    let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
    let (eigs, vecs) = eigenvectors(&a).unwrap();

    for i in 0..3 {
        let lambda = eigs[i].re;
        let v = extract_column(&vecs, i);

        let av = a.matmul(&v).unwrap();
        let lambda_v = v.scalar_mul(lambda);

        // A·v ≈ λ·v
        assert_vectors_approx_eq(&av, &lambda_v, 1e-10);
    }
}
```

#### Symmetric Matrix Properties
```rust
#[test]
fn test_symmetric_orthogonal_eigenvectors() {
    let a = create_symmetric_matrix(5);
    let (eigs, vecs) = eigen_symmetric(&a, 100, 1e-10).unwrap();

    // Check orthogonality: V^T·V = I
    let vt = vecs.transpose().unwrap();
    let vtv = vt.matmul(&vecs).unwrap();
    let identity = RealTensor::eye(5);

    assert_matrices_approx_eq(&vtv, &identity, 1e-10);
}
```

#### Power Iteration Convergence
```rust
#[test]
fn test_power_iteration_convergence() {
    let a = RealTensor::matrix(3, 3, vec![
        5.0, 0.0, 0.0,
        0.0, 3.0, 0.0,
        0.0, 0.0, 1.0
    ]).unwrap();

    let (lambda, _v) = power_iteration(&a, 100, 1e-10).unwrap();

    // Should find largest eigenvalue (5.0)
    assert_relative_eq!(lambda, 5.0, epsilon = 1e-10);
}
```

---

## 📊 Performance Characteristics

### Computational Complexity

| Algorithm | Time Complexity | Space Complexity | Convergence |
|-----------|----------------|------------------|-------------|
| Standard Eigendecomp | O(10n³) | O(n²) | Direct (non-iterative) |
| Eigenvectors | O(10n³) | O(n²) | Direct |
| Power Iteration | O(k·n²) | O(n) | Linear: (λ₂/λ₁)^k |
| QR Algorithm (custom) | O(k·n³) | O(n²) | Depends on spectrum |
| Symmetric (planned) | O(4n³) | O(n²) | Direct |

**k**: Number of iterations (typically 10-100)

### Algorithm Selection Guide

```
Need eigenvalues?
├─ Only largest eigenvalue?
│  └─ Power Iteration (fastest for single eigenvalue)
│
├─ Matrix is symmetric?
│  └─ Symmetric Eigendecomp (2× faster, more accurate)
│
├─ Need all eigenvalues?
│  ├─ Small matrix (n < 100)?
│  │  └─ Standard Eigendecomp (faer's optimized QR)
│  └─ Large matrix?
│     └─ Consider iterative methods (not implemented)
│
└─ Need eigenvectors too?
   └─ Use eigenvectors() instead of eigenvalues()
```

### When to Use Each

**Standard Eigendecomposition**:
- General-purpose, production code
- Need reliable, tested implementation
- Matrix size < 1000×1000

**Power Iteration**:
- Only need dominant eigenvalue
- Large sparse matrices (future optimization)
- PageRank-style applications

**QR Algorithm (custom)**:
- Educational purposes
- Understanding convergence behavior
- Fine-tuned convergence criteria

**Symmetric Eigendecomposition**:
- Symmetric matrices (correlation, covariance, graph Laplacians)
- Need guaranteed real eigenvalues and orthogonal eigenvectors

---

## 🔬 Numerical Stability Considerations

### Sources of Error

1. **Rounding Error**: Accumulated during QR iterations
2. **Cancellation**: Computing det(A - λI) near eigenvalue
3. **Ill-Conditioned Eigenvectors**: Small eigenvalue perturbations → large eigenvector changes

### Condition Numbers

**Eigenvalue Condition Number**:
```
κ(λ_i) = 1 / |w_i^H · v_i|
```
where w_i is left eigenvector, v_i is right eigenvector.

**Implications**:
- Well-conditioned: κ ≈ 1 (normal matrices)
- Ill-conditioned: κ >> 1 (defective matrices)
- Worst case: Repeated eigenvalues with geometric multiplicity < algebraic multiplicity

### Stability of Algorithms

| Algorithm | Stability | Notes |
|-----------|-----------|-------|
| Standard Eigendecomp | **Backward Stable** | faer uses Francis QR with shifts |
| Power Iteration | **Stable** | Simple operations, accumulation controlled |
| QR Algorithm (custom) | **Stable** | Orthogonal transformations preserve stability |
| Symmetric (faer) | **Very Stable** | Symmetric algorithms have better error bounds |

### Best Practices

1. **Check Symmetry**: For symmetric matrices, use specialized algorithms
   ```rust
   if is_symmetric(&a, 1e-10) {
       let (eigs, vecs) = eigen_symmetric(&a, ...)?;
   } else {
       let (eigs, vecs) = eigenvectors(&a)?;
   }
   ```

2. **Verify Eigenvalue-Eigenvector Pairs**:
   ```rust
   let residual = a.matmul(&v)?.sub(&v.scalar_mul(lambda))?;
   let error = residual.norm() / (a.norm() * v.norm());
   assert!(error < 1e-10, "Residual too large: {}", error);
   ```

3. **Handle Multiple Eigenvalues**: Be aware that eigenvector computation may be unstable

4. **Use Appropriate Tolerance**: Balance accuracy vs. convergence time

---

## 🔮 Future Enhancements

### Planned Algorithms

1. **Jacobi Algorithm** (for symmetric matrices):
   ```rust
   pub fn jacobi_eigenvalues(
       tensor: &RealTensor,
       max_sweeps: usize,
       tolerance: f64
   ) -> Result<(Vec<f64>, RealTensor), String> {
       // Iteratively apply Givens rotations to diagonalize
   }
   ```

2. **Lanczos Algorithm** (for large sparse symmetric matrices):
   ```rust
   pub fn lanczos_eigenvalues(
       tensor: &RealTensor,
       num_eigenvalues: usize,
       max_iterations: usize
   ) -> Result<Vec<f64>, String> {
       // Krylov subspace method for largest/smallest eigenvalues
   }
   ```

3. **Arnoldi Iteration** (for large general matrices):
   ```rust
   pub fn arnoldi_eigenvalues(
       tensor: &RealTensor,
       num_eigenvalues: usize,
       max_iterations: usize
   ) -> Result<Vec<Complex>, String> {
       // Krylov method for general matrices
   }
   ```

4. **Generalized Eigenvalue Problem**:
   ```rust
   pub fn generalized_eigenvalues(
       a: &RealTensor,
       b: &RealTensor
   ) -> Result<Vec<Complex>, String> {
       // Solve A·v = λ·B·v
   }
   ```

### Complex Matrix Support

```rust
use achronyme_types::tensor::ComplexTensor;

pub fn complex_eigenvalues(tensor: &ComplexTensor)
    -> Result<Vec<Complex>, String> { ... }

pub fn complex_eigenvectors(tensor: &ComplexTensor)
    -> Result<(Vec<Complex>, ComplexTensor), String> { ... }
```

### Parallel Eigenvalue Computation

```rust
pub fn parallel_eigenvalues(
    tensor: &RealTensor,
    num_threads: usize
) -> Result<Vec<Complex>, String> {
    // Parallelize QR algorithm or use spectrum slicing
}
```

---

## 📚 References

### Textbooks

1. **Golub & Van Loan** - "Matrix Computations" (4th ed.)
   - Chapter 7: Unsymmetric Eigenvalue Problems
   - Chapter 8: Symmetric Eigenvalue Problems
   - Section 7.5: The QR Algorithm

2. **Trefethen & Bau** - "Numerical Linear Algebra"
   - Lectures 25-27: Eigenvalue Problems
   - Lectures 28-30: Symmetric Eigenvalue Problems
   - Lecture 32: The QR Algorithm

3. **Watkins** - "The Matrix Eigenvalue Problem: GR and Krylov Subspace Methods"
   - Comprehensive treatment of modern eigenvalue algorithms

### Papers

- **Francis (1961)**: "The QR Transformation" - Original QR algorithm paper
- **Wilkinson (1965)**: "The Algebraic Eigenvalue Problem" - Classic reference
- **Parlett (1980)**: "The Symmetric Eigenvalue Problem" - Definitive text

### Online Resources

- faer eigendecomposition: https://docs.rs/faer/latest/faer/struct.Eigendecomposition.html
- LAPACK Eigenvalue Routines: https://netlib.org/lapack/lug/node31.html

---

## 🎓 Educational Notes

### Understanding Eigenvalues Geometrically

**Linear Transformation Perspective**:
```
A·v = λ·v

Matrix A transforms vector v by scaling it by factor λ,
without changing direction.
```

**Example**: Rotation matrix has complex eigenvalues (rotates, doesn't scale)

### Eigenvalue Spectrum

**Spectral Radius**: `ρ(A) = max|λ_i|`
- Determines convergence rate of iterative methods
- Matrix power convergence: `A^k → 0` iff `ρ(A) < 1`

**Trace and Determinant**:
```
tr(A) = Σ λ_i         (sum of eigenvalues)
det(A) = ∏ λ_i        (product of eigenvalues)
```

### Common Eigenvalue Patterns

**Diagonal Matrix**: Eigenvalues = diagonal elements
**Triangular Matrix**: Eigenvalues = diagonal elements
**Symmetric Matrix**: Real eigenvalues, orthogonal eigenvectors
**Orthogonal Matrix**: Eigenvalues on unit circle |λ| = 1
**Positive Definite**: All eigenvalues > 0

---

**Module Maintainer**: Achronyme Project Team
**Last Updated**: 2024-11-14
