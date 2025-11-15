# achronyme-linalg Implementation Guide

Internal architecture and implementation details for the `achronyme-linalg` crate.

**Target audience**: Contributors, maintainers, and developers extending the linear algebra functionality.

---

## 🏛️ Internal Architecture

The `achronyme-linalg` crate is organized into three primary modules, each handling a distinct category of linear algebra operations:

```
src/
├── lib.rs                  # Public API, module declarations, re-exports
├── decompositions.rs       # Matrix factorization algorithms (272 lines)
├── eigenvalues.rs          # Eigenvalue/eigenvector computations (312 lines)
└── solvers.rs              # Linear system solving, inversions (338 lines)
```

**Total Implementation**: ~940 lines of Rust code

### Design Philosophy

1. **Separation of Concerns**: Each module handles one category of operations
2. **Thin Wrapper Layer**: Minimal abstraction over `faer` while providing type conversions
3. **Type Safety**: Leverage Rust's type system for compile-time correctness
4. **Error Propagation**: Explicit `Result` types for all fallible operations
5. **Zero-Copy When Possible**: Minimize allocations through careful API design

### Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Code                               │
│                    (SOC or Rust API)                            │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Public Functions                             │
│   (lu_decomposition, eigenvalues, solve_system, etc.)           │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Type Conversion Layer                          │
│                                                                 │
│  tensor_to_faer_mat()  ─────►  Achronyme → faer               │
│  faer_mat_to_tensor()  ◄─────  faer → Achronyme               │
│  tensor_to_faer_col()  ─────►  Vector conversion               │
│  faer_col_to_tensor()  ◄─────  Vector conversion               │
│  faer_mat_ref_to_tensor() ◄──  Reference conversion            │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    faer Library                                 │
│                                                                 │
│  • PartialPivLu      (LU decomposition)                        │
│  • Qr                (QR decomposition)                         │
│  • Cholesky          (Cholesky decomposition)                   │
│  • SvdRef            (SVD decomposition)                        │
│  • Eigendecomposition (eigenvalue computation)                  │
│  • Mat/Col           (matrix/vector storage)                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Structure Breakdown

### `lib.rs` - Public API Surface

**Responsibilities**:
- Module declarations (`pub mod decompositions`, etc.)
- Re-exports for convenience (flatten API for users)
- Crate-level documentation
- Integration tests entry point

**Key Exports**:
```rust
// Decompositions
pub use decompositions::{
    lu_decomposition,
    qr_decomposition,
    cholesky_decomposition,
    svd_decomposition
};

// Eigenvalues
pub use eigenvalues::{
    eigenvalues,
    eigenvectors,
    power_iteration,
    qr_eigenvalues,
    eigen_symmetric
};

// Solvers
pub use solvers::{
    inverse,
    solve_system,
    determinant_nd,
    is_symmetric,
    is_positive_definite
};
```

**Design Decisions**:
- **Flat API**: Users can import functions directly without module prefixes
- **Explicit Names**: No ambiguous short names (e.g., `determinant_nd` not just `det`)
- **Consistency**: All functions return `Result<T, String>` for error handling

---

### `decompositions.rs` - Matrix Factorizations

**Module Organization**:
```rust
// Conversion utilities (internal)
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
fn faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String>
fn faer_mat_ref_to_tensor(mat: MatRef<f64>) -> Result<RealTensor, String>

// Public decomposition functions
pub fn lu_decomposition(...) -> Result<(RealTensor, RealTensor, Vec<usize>), String>
pub fn cholesky_decomposition(...) -> Result<RealTensor, String>
pub fn qr_decomposition(...) -> Result<(RealTensor, RealTensor), String>
pub fn svd_decomposition(...) -> Result<(RealTensor, Vec<f64>, RealTensor), String>

// Tests module
#[cfg(test)]
mod tests { ... }
```

**Key Algorithms**:

1. **LU Decomposition** (`lu_decomposition`)
   - Uses `faer::PartialPivLu`
   - Returns `(L, U, P)` where `P·A = L·U`
   - Permutation represented as `Vec<usize>` (row indices)

2. **Cholesky Decomposition** (`cholesky_decomposition`)
   - Uses `faer::Cholesky` with `Side::Lower`
   - Only accepts symmetric positive-definite matrices
   - Returns lower triangular `L` where `A = L·L^T`
   - Error handling for non-SPD matrices

3. **QR Decomposition** (`qr_decomposition`)
   - Uses `faer::Qr`
   - Computes thin QR (not full)
   - Returns `(Q, R)` where `A = Q·R`
   - Works for rectangular matrices (m ≥ n)

4. **SVD** (`svd_decomposition`)
   - Uses `faer::thin_svd()`
   - Returns `(U, singular_values, V^T)`
   - Singular values as `Vec<f64>`, not diagonal matrix
   - Automatically transposes V for user convenience

**Conversion Strategy**:
```rust
// Achronyme tensors are row-major, faer expects column-major semantically
// But we handle this transparently in conversion functions

fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64> {
    assert!(tensor.is_matrix());
    let rows = tensor.rows();
    let cols = tensor.cols();
    // Create faer matrix using function initialization
    Mat::from_fn(rows, cols, |i, j| tensor.get_matrix(i, j).unwrap())
}
```

**Error Handling**:
- Non-square matrices for LU/Cholesky → `Err("requires square matrix")`
- Non-positive-definite for Cholesky → `Err("not positive definite?")`
- Tensor creation failures → propagate with `.map_err(|e| e.to_string())`

**Testing Strategy**:
- Basic functionality: smoke tests with small known matrices
- Dimension verification: check output shapes
- Reconstruction tests: verify `A ≈ L·U`, `A ≈ Q·R`, etc. (future enhancement)
- Edge cases: identity matrices, diagonal matrices

---

### `eigenvalues.rs` - Eigenvalue Computations

**Module Organization**:
```rust
// Conversion utilities
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>

// Public eigenvalue functions
pub fn eigenvalues(...) -> Result<Vec<Complex>, String>
pub fn eigenvectors(...) -> Result<(Vec<Complex>, RealTensor), String>
pub fn power_iteration(...) -> Result<(f64, RealTensor), String>
pub fn qr_eigenvalues(...) -> Result<Vec<f64>, String>
pub fn eigen_symmetric(...) -> Result<(Vec<f64>, RealTensor), String>

// Tests module
#[cfg(test)]
mod tests { ... }
```

**Algorithm Details**:

1. **Standard Eigenvalue Decomposition** (`eigenvalues`, `eigenvectors`)
   - Uses `faer::eigendecomposition::<c64>()`
   - Supports complex eigenvalues (even for real matrices)
   - Eigenvalues as `Vec<Complex>` (Achronyme type)
   - Eigenvectors extracted as real parts (limitation for now)
   - **Future**: Full complex eigenvector support

2. **Power Iteration** (`power_iteration`)
   - Custom implementation (not delegated to faer)
   - Finds dominant eigenvalue by magnitude
   - Iterative algorithm:
     ```
     Initialize: v₀ = [1, 0, 0, ...]
     Loop:
       v_{k+1} = A·v_k / ||A·v_k||
       λ_{k+1} = ||A·v_k||
     Until: |λ_{k+1} - λ_k| < tolerance
     ```
   - Convergence rate: O(|λ₁/λ₂|^k)
   - Returns: `(eigenvalue, eigenvector)`

3. **QR Algorithm** (`qr_eigenvalues`)
   - Custom iterative implementation
   - Repeatedly factors `A_k = Q_k·R_k`, then `A_{k+1} = R_k·Q_k`
   - Diagonal converges to eigenvalues
   - Convergence check: off-diagonal elements < tolerance
   - Returns only real parts (eigenvalues may be complex)
   - **Limitation**: Doesn't handle complex conjugate pairs explicitly

4. **Symmetric Eigendecomposition** (`eigen_symmetric`)
   - Currently delegates to general eigendecomposition
   - Extracts real parts (symmetric matrices have real eigenvalues)
   - **TODO**: Implement specialized symmetric algorithm (Jacobi, Divide-and-Conquer)
   - Symmetric algorithms are ~2× faster and more stable

**Complex Number Handling**:
```rust
use faer::complex_native::c64;  // faer's complex type
use achronyme_types::complex::Complex;  // Achronyme's complex type

// Conversion from faer c64 to Achronyme Complex
let val: c64 = eigenvals.read(i);
let achronyme_val = Complex::new(val.re, val.im);
```

**Numerical Considerations**:
- **Non-symmetric matrices**: Can have complex eigenvalues
- **Symmetric matrices**: Always have real eigenvalues, orthogonal eigenvectors
- **Defective matrices**: Multiple eigenvalues may not have independent eigenvectors
- **Convergence**: Power iteration fails if |λ₁| = |λ₂| (multiple dominant eigenvalues)

**Testing Strategy**:
- Known eigenvalues: test 2×2 matrix with calculable eigenvalues
- Identity matrix: all eigenvalues = 1
- Symmetry: verify symmetric matrices return real eigenvalues
- Error cases: non-square matrices
- Precision: use `approx::assert_relative_eq!` for floating-point comparison

---

### `solvers.rs` - Linear Systems and Matrix Analysis

**Module Organization**:
```rust
// Conversion utilities
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
fn faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String>
fn tensor_to_faer_col(tensor: &RealTensor) -> Col<f64>
fn faer_col_to_tensor(col: Col<f64>) -> RealTensor

// Public solver functions
pub fn determinant_nd(...) -> Result<f64, String>
pub fn inverse(...) -> Result<RealTensor, String>
pub fn solve_system(...) -> Result<RealTensor, String>
pub fn is_symmetric(...) -> bool
pub fn is_positive_definite(...) -> bool

// Tests module
#[cfg(test)]
mod tests { ... }
```

**Algorithm Details**:

1. **Determinant Computation** (`determinant_nd`)
   - Uses LU decomposition approach
   - `det(A) = det(P) · det(L) · det(U)`
   - `det(L) = 1` (unit diagonal)
   - `det(U) = ∏ u_ii` (product of diagonal)
   - `det(P) = (-1)^(number of swaps)`
   - **Permutation sign calculation**:
     ```rust
     // Count swaps by analyzing permutation cycles
     let perm_indices = perm.arrays().0;
     let mut swaps = 0;
     let mut visited = vec![false; n];
     // Detect cycles, count swaps
     // Each cycle of length k needs k-1 swaps
     ```
   - Complexity: O(n³) for LU + O(n) for sign

2. **Matrix Inverse** (`inverse`)
   - Solves `A·X = I` for `X = A^(-1)`
   - Uses `faer::PartialPivLu` for stability
   - Returns error if matrix is singular (numerically)
   - **Note**: Direct solving `Ax = b` is preferred over computing `A^(-1)·b`
   - Complexity: O(n³)

3. **Linear System Solver** (`solve_system`)
   - Solves `A·x = b` for `x`
   - Uses LU decomposition with forward/backward substitution
   - **Steps**:
     1. Validate: `A` is matrix, `b` is vector, dimensions match
     2. Convert: `A` → faer `Mat`, `b` → faer `Col`
     3. Decompose: LU with partial pivoting
     4. Solve: `lu.solve(&b_col)`
     5. Convert: result → Achronyme `RealTensor`
   - Complexity: O(n³) for decomposition, O(n²) for solving
   - Error handling: dimension mismatch, singular matrix

4. **Symmetry Check** (`is_symmetric`)
   - Compares `A[i,j]` with `A[j,i]` for all i,j
   - Tolerance parameter for floating-point comparison
   - Early exit on first asymmetric element
   - Complexity: O(n²) worst case, O(1) best case
   - Returns `bool` (no error, always succeeds)

5. **Positive Definiteness Check** (`is_positive_definite`)
   - Attempts Cholesky decomposition
   - Matrix is SPD ⟺ Cholesky succeeds
   - Uses `faer::cholesky()` result (`Ok` or `Err`)
   - **Fast test**: O(n³/3) - same as Cholesky
   - Returns `bool`
   - **Limitation**: Requires symmetry (should check first)

**Vector Handling**:
```rust
// Achronyme vectors are rank-1 tensors
// faer vectors are Col<f64> or Row<f64>

fn tensor_to_faer_col(tensor: &RealTensor) -> Col<f64> {
    assert!(tensor.is_vector());
    let data = tensor.data();
    Col::from_fn(data.len(), |i| data[i])
}

fn faer_col_to_tensor(col: Col<f64>) -> RealTensor {
    let data: Vec<f64> = (0..col.nrows()).map(|i| col.read(i)).collect();
    RealTensor::vector(data)  // Cannot fail for valid data
}
```

**Error Messages**:
```rust
// Descriptive error messages for debugging
"Dimension mismatch: matrix has {} rows but vector has {} elements"
"Determinant requires square matrix"
"`a` must be a matrix and `b` must be a vector"
"Inverse requires square matrix"
```

**Testing Strategy**:
- **Determinant**: Test known values (2×2, 3×3 matrices)
- **Inverse**: Verify `A·A^(-1) = I`
- **Solve**: Test system with known solution
- **Symmetry**: Test symmetric and non-symmetric matrices
- **Dimension errors**: Invalid input shapes
- **Numerical accuracy**: Use `approx` for FP comparisons

---

## 🔄 Module Interactions

### Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                         lib.rs                              │
│                   (orchestrates modules)                    │
└──────────┬──────────────────┬────────────────┬──────────────┘
           │                  │                │
           ▼                  ▼                ▼
┌──────────────────┐ ┌────────────────┐ ┌─────────────────┐
│ decompositions.rs│ │ eigenvalues.rs │ │   solvers.rs    │
│                  │ │                │ │                 │
│ Independent      │ │ Independent    │ │ Independent     │
│ (no cross-deps)  │ │ (no cross-deps)│ │ (no cross-deps) │
└────────┬─────────┘ └────────┬───────┘ └────────┬────────┘
         │                    │                   │
         └────────────────────┼───────────────────┘
                              │
                              ▼
                 ┌────────────────────────┐
                 │   achronyme-types       │
                 │   (RealTensor, Complex) │
                 └────────────────────────┘
                              │
                              ▼
                 ┌────────────────────────┐
                 │        faer            │
                 │   (linear algebra)     │
                 └────────────────────────┘
```

**Key Insight**: Modules are intentionally independent to allow:
- Parallel development
- Easy testing in isolation
- Future extraction to separate crates if needed
- No circular dependencies

### Type Conversion Patterns

Each module implements its own conversion functions (duplicated):
- **Reason**: Encapsulation, no shared internal utilities
- **Trade-off**: Some code duplication vs. internal coupling
- **Future**: Could extract to `conversions.rs` if duplication becomes problematic

**Common Conversion Functions**:
```rust
// Matrix conversions
tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String>
faer_mat_ref_to_tensor(mat: MatRef<f64>) -> Result<RealTensor, String>

// Vector conversions (only in solvers.rs)
tensor_to_faer_col(tensor: &RealTensor) -> Col<f64>
faer_col_to_tensor(col: Col<f64>) -> RealTensor
```

**Ownership Semantics**:
- `tensor_to_faer_mat`: Borrows `&RealTensor`, copies data to `Mat` (owned)
- `faer_mat_to_tensor`: Consumes `Mat`, produces `RealTensor` (owned)
- `faer_mat_ref_to_tensor`: Borrows `MatRef`, copies to `RealTensor` (owned)

**Why No Zero-Copy?**
- Achronyme tensors and faer matrices have different memory layouts
- Row-major vs. column-major considerations
- Safety: ownership transfer would be complex and error-prone
- Performance: copies are acceptable for typical matrix sizes

---

## 🎯 Design Patterns

### 1. Type Conversion Adapter Pattern

**Problem**: Bridge Achronyme's `RealTensor` with faer's `Mat<f64>`

**Solution**: Explicit conversion functions at module boundaries

```rust
// Input conversion
let faer_matrix = tensor_to_faer_mat(&input_tensor);

// Algorithm execution
let result = faer_matrix.some_operation();

// Output conversion
let output_tensor = faer_mat_to_tensor(result)?;
```

**Benefits**:
- Clear separation between Achronyme and faer types
- Type safety at compile time
- Easy to test conversion correctness
- Flexible for future optimizations

### 2. Error Propagation Pattern

**Problem**: Handle errors from multiple sources (faer, tensor creation, validation)

**Solution**: Consistent `Result<T, String>` return types with descriptive messages

```rust
pub fn cholesky_decomposition(tensor: &RealTensor) -> Result<RealTensor, String> {
    // Validation
    if !tensor.is_square() {
        return Err("Cholesky decomposition requires square matrix".to_string());
    }

    // Conversion
    let mat = tensor_to_faer_mat(tensor);

    // Algorithm (may fail)
    let chol = mat
        .cholesky(faer::Side::Lower)
        .map_err(|_| "Cholesky decomposition failed (matrix not positive definite?)".to_string())?;

    // Conversion with error mapping
    let l = chol.compute_l();
    faer_mat_to_tensor(l)  // Returns Result<RealTensor, String>
}
```

**Error Categories**:
1. **Validation Errors**: Wrong tensor shape, dimension mismatch
2. **Algorithm Errors**: Singular matrix, non-convergence
3. **Conversion Errors**: Tensor creation failures (rare)

### 3. Thin Wrapper Pattern

**Problem**: Expose faer functionality without reimplementing algorithms

**Solution**: Delegate to faer with minimal logic, focus on API design

```rust
pub fn qr_decomposition(tensor: &RealTensor) -> Result<(RealTensor, RealTensor), String> {
    let mat = tensor_to_faer_mat(tensor);  // Convert
    let qr = mat.qr();                     // Delegate to faer
    let q = qr.compute_thin_q();           // Extract results
    let r = qr.compute_thin_r();
    let q_tensor = faer_mat_to_tensor(q)?; // Convert back
    let r_tensor = faer_mat_to_tensor(r)?;
    Ok((q_tensor, r_tensor))               // Return
}
```

**Benefits**:
- Leverage faer's optimized implementations
- Minimal maintenance burden
- Easy to update when faer improves
- Small API surface to document

### 4. Assert-Based Preconditions

**Problem**: Ensure invariants at conversion boundaries

**Solution**: Use `assert!` for programming errors, `Result` for runtime errors

```rust
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64> {
    assert!(tensor.is_matrix());  // Programming error if violated
    // ... conversion logic
}

pub fn solve_system(a: &RealTensor, b: &RealTensor) -> Result<RealTensor, String> {
    if !a.is_matrix() || !b.is_vector() {  // Runtime check
        return Err("`a` must be a matrix and `b` must be a vector".to_string());
    }
    // ... solver logic
}
```

**Guideline**:
- `assert!`: Internal invariants, should never fail in correct usage
- `Result::Err`: External inputs, user-facing errors

---

## 🧪 Testing Patterns

### Test Organization

Each module has its own `#[cfg(test)] mod tests` section:
- Co-located with implementation (easier maintenance)
- Uses `use super::*` for private function access
- Consistent naming: `test_<function>_<scenario>`

### Testing Strategies

#### 1. Smoke Tests (Basic Functionality)
```rust
#[test]
fn test_lu_decomposition() {
    let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
    let result = lu_decomposition(&a);
    assert!(result.is_ok());
    let (l, u, _p) = result.unwrap();
    assert_eq!(l.rows(), 3);
    assert_eq!(u.cols(), 3);
}
```

#### 2. Known-Value Tests
```rust
#[test]
fn test_determinant_2x2() {
    let a = RealTensor::matrix(2, 2, vec![
        4.0, 7.0,
        2.0, 6.0
    ]).unwrap();
    let det = determinant_nd(&a).unwrap();
    // det = 4*6 - 7*2 = 10
    assert_relative_eq!(det, 10.0, epsilon = 1e-10);
}
```

#### 3. Reconstruction Tests (Future)
```rust
// TODO: Add reconstruction tests
#[test]
fn test_lu_reconstruction() {
    let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
    let (l, u, p) = lu_decomposition(&a).unwrap();
    // Apply permutation and check P*A ≈ L*U
}
```

#### 4. Error Handling Tests
```rust
#[test]
fn test_eigenvalues_nonsquare_fails() {
    let a = RealTensor::matrix(2, 3, vec![...]).unwrap();
    let result = eigenvalues(&a);
    assert!(result.is_err());
}

#[test]
fn test_solve_system_dimension_mismatch() {
    let a = RealTensor::matrix(2, 2, vec![...]).unwrap();
    let b = RealTensor::vector(vec![1.0, 2.0, 3.0]); // Wrong size
    let result = solve_system(&a, &b);
    assert!(result.is_err());
}
```

#### 5. Numerical Accuracy Tests
```rust
#[test]
fn test_inverse_2x2() {
    let a = RealTensor::matrix(2, 2, vec![...]).unwrap();
    let a_inv = inverse(&a).unwrap();
    let product = a.matmul(&a_inv).unwrap();
    let identity = RealTensor::eye(2);

    for i in 0..2 {
        for j in 0..2 {
            assert_relative_eq!(
                product.get_matrix(i, j).unwrap(),
                identity.get_matrix(i, j).unwrap(),
                epsilon = 1e-10
            );
        }
    }
}
```

### Testing Tools

- **`approx` crate**: Floating-point comparison with tolerances
  ```rust
  use approx::assert_relative_eq;
  assert_relative_eq!(actual, expected, epsilon = 1e-10);
  ```
- **`unwrap()` vs `?`**: Tests use `unwrap()` for clarity (panics are test failures)
- **Test data**: Small matrices (2×2, 3×3) for hand-verification

### Coverage Goals

- ✅ All public functions have at least one test
- ✅ Error paths are tested (invalid inputs)
- ⚠️ Edge cases partially covered (identity, diagonal matrices)
- ❌ Reconstruction tests not implemented yet
- ❌ Performance benchmarks not implemented

---

## 🔧 How to Extend the System

### Adding a New Decomposition

**Example**: Adding Schur decomposition

1. **Add function to `decompositions.rs`**:
```rust
/// Schur Decomposition
///
/// Decomposes matrix A into Q * T * Q^H where T is upper triangular
pub fn schur_decomposition(tensor: &RealTensor) -> Result<(RealTensor, RealTensor), String> {
    if !tensor.is_square() {
        return Err("Schur decomposition requires square matrix".to_string());
    }

    let mat = tensor_to_faer_mat(tensor);

    // Use faer's Schur decomposition (if available)
    let schur = mat.schur_decomposition();

    let q = schur.q();
    let t = schur.t();

    let q_tensor = faer_mat_ref_to_tensor(q)?;
    let t_tensor = faer_mat_to_tensor(t)?;

    Ok((q_tensor, t_tensor))
}
```

2. **Add tests**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_schur_decomposition() {
        let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
        let result = schur_decomposition(&a);
        assert!(result.is_ok());
        let (q, t) = result.unwrap();
        // Verify properties: Q is orthogonal, T is upper triangular
    }
}
```

3. **Export in `lib.rs`**:
```rust
pub use decompositions::{
    lu_decomposition,
    qr_decomposition,
    cholesky_decomposition,
    svd_decomposition,
    schur_decomposition,  // Add here
};
```

4. **Document in crate README**: Add to "Key Algorithms Provided" section

### Adding a New Eigenvalue Algorithm

**Example**: Adding Jacobi algorithm for symmetric matrices

1. **Add to `eigenvalues.rs`**:
```rust
/// Jacobi Algorithm for Symmetric Eigenvalues
///
/// Iteratively diagonalizes symmetric matrix using Givens rotations.
/// More stable and accurate than QR for symmetric matrices.
pub fn jacobi_eigenvalues(
    tensor: &RealTensor,
    max_iterations: usize,
    tolerance: f64
) -> Result<(Vec<f64>, RealTensor), String> {
    if !tensor.is_square() {
        return Err("Jacobi requires square matrix".to_string());
    }

    // Check symmetry
    for i in 0..tensor.rows() {
        for j in (i+1)..tensor.cols() {
            if (tensor.get_matrix(i,j).unwrap() - tensor.get_matrix(j,i).unwrap()).abs() > tolerance {
                return Err("Jacobi requires symmetric matrix".to_string());
            }
        }
    }

    // Implement Jacobi iteration
    // ...
}
```

2. **Export and document** as above

### Adding Iterative Solvers

**Example**: Adding Conjugate Gradient solver

1. **Add to `solvers.rs`**:
```rust
/// Conjugate Gradient Solver
///
/// Iteratively solves Ax = b for symmetric positive-definite A.
/// More efficient than direct methods for large sparse matrices.
pub fn conjugate_gradient(
    a: &RealTensor,
    b: &RealTensor,
    max_iterations: usize,
    tolerance: f64
) -> Result<RealTensor, String> {
    // Validate inputs
    if !a.is_square() {
        return Err("CG requires square matrix".to_string());
    }

    // Initialize: x₀ = 0, r₀ = b, p₀ = r₀
    let mut x = RealTensor::zeros(&[b.size()]);
    let mut r = b.clone();
    let mut p = r.clone();

    for _iter in 0..max_iterations {
        // CG iteration steps
        // α_k = (r_k^T r_k) / (p_k^T A p_k)
        // x_{k+1} = x_k + α_k p_k
        // r_{k+1} = r_k - α_k A p_k
        // β_k = (r_{k+1}^T r_{k+1}) / (r_k^T r_k)
        // p_{k+1} = r_{k+1} + β_k p_k

        // Check convergence: ||r|| < tolerance
        let residual_norm = /* compute norm */;
        if residual_norm < tolerance {
            break;
        }
    }

    Ok(x)
}
```

2. **Consider**: May need tensor operations (dot product, axpy, etc.)
3. **Alternative**: Wait for sparse matrix support

### Integration Guidelines

When adding new functionality:

1. **Check faer documentation**: See if algorithm is already implemented
2. **Match existing API style**: `Result<T, String>`, descriptive errors
3. **Add comprehensive tests**: Functionality, errors, edge cases
4. **Document thoroughly**: Algorithm description, complexity, use cases
5. **Update crate README**: Add to algorithms list, usage examples
6. **Consider performance**: Benchmark against alternatives if applicable

---

## 📊 Complexity Analysis

### Space Complexity

| Operation | Input | Output | Temporary | Total |
|-----------|-------|--------|-----------|-------|
| LU | n² | 2n² | n² (during) | O(n²) |
| QR | mn | m²+mn | mn (during) | O(mn) |
| Cholesky | n² | n² | n² (during) | O(n²) |
| SVD | mn | m²+n²+min(m,n) | O(mn) | O(m²+n²) |
| Eigendecomp | n² | n²+n | n² (during) | O(n²) |
| Solve Ax=b | n²+n | n | n² (LU) | O(n²) |
| Inverse | n² | n² | n² (LU) | O(n²) |

**Memory Optimization Opportunities**:
- In-place algorithms (modify input matrix)
- Lazy evaluation (compute on demand)
- Streaming for large matrices (future)

### Time Complexity Summary

| Category | Operation | Best Case | Average | Worst Case | Notes |
|----------|-----------|-----------|---------|------------|-------|
| **Decompositions** |
| | LU | O(n³) | O(n³) | O(n³) | Dominated by elimination |
| | QR | O(mn²) | O(mn²) | O(mn²) | m ≥ n |
| | Cholesky | O(n³/3) | O(n³/3) | O(n³/3) | 2× faster than LU |
| | SVD | O(min(m²n, mn²)) | O(min(m²n, mn²)) | O(min(m²n, mn²)) | Expensive |
| **Eigenvalues** |
| | Standard | O(n³) | O(n³) | O(n³) | QR algorithm internally |
| | Power Iter | O(kn²) | O(kn²) | O(kn²) | k = iterations |
| | QR Algo | O(kn³) | O(kn³) | O(kn³) | k = iterations |
| **Solvers** |
| | Determinant | O(n³) | O(n³) | O(n³) | Via LU |
| | Inverse | O(n³) | O(n³) | O(n³) | Via LU + solve |
| | Solve Ax=b | O(n³) | O(n³) | O(n³) | LU + substitution |
| | Is Symmetric | O(1) | O(n²) | O(n²) | Early exit possible |
| | Is Pos Def | O(n³/3) | O(n³/3) | O(n³/3) | Via Cholesky |

---

## 🎯 Numerical Stability Considerations

### Sources of Instability

1. **Small Pivot Elements** (LU without pivoting)
   - **Problem**: Division by near-zero values amplifies errors
   - **Solution**: Partial pivoting (swap rows to maximize pivot)
   - **faer**: Uses partial pivoting by default

2. **Catastrophic Cancellation** (subtraction)
   - **Problem**: `a - b` loses precision when `a ≈ b`
   - **Example**: Solving `(A - λI)x = 0` near eigenvalue
   - **Mitigation**: Use stable algorithms (QR, SVD)

3. **Condition Number**
   - **Definition**: `κ(A) = ||A|| · ||A⁻¹||`
   - **Impact**: Relative error amplified by factor κ(A)
   - **Example**: If `κ(A) = 10⁶` and input error is `10⁻¹⁰`, output error ≈ `10⁻⁴`

4. **Accumulation in Iterative Methods**
   - **Problem**: Errors compound over iterations
   - **Example**: Power iteration, QR algorithm
   - **Solution**: Monitor convergence, use residual checks

### Stability of Implemented Algorithms

| Algorithm | Stability | Reason |
|-----------|-----------|--------|
| LU with partial pivoting | **Stable** | Pivoting prevents small divisors |
| QR (Householder) | **Very Stable** | Orthogonal transformations preserve norms |
| Cholesky | **Stable** | Exploits positive definiteness |
| SVD | **Most Stable** | Computes all singular values reliably |
| Eigendecomp (QR) | **Stable** | Orthogonal similarity transformations |
| Power Iteration | **Conditionally Stable** | Depends on eigenvalue gap |
| Direct Gaussian Elim | **Unstable** | Not implemented (no pivoting) |

### Best Practices for Users

1. **Check Condition Number**:
   ```rust
   let (u, s, vt) = svd_decomposition(&a)?;
   let cond = s[0] / s[s.len()-1];  // σ_max / σ_min
   if cond > 1e10 {
       println!("Warning: Matrix is ill-conditioned (κ = {})", cond);
   }
   ```

2. **Use Appropriate Decomposition**:
   - Well-conditioned, square → LU
   - Symmetric positive definite → Cholesky (fastest)
   - Ill-conditioned, least squares → SVD (most stable)
   - Eigenvalues → Standard eigendecomp or symmetric specialized

3. **Validate Results**:
   ```rust
   let x = solve_system(&a, &b)?;
   let b_check = a.matmul(&x)?;
   let residual = b.sub(&b_check)?;
   let error = residual.norm() / b.norm();
   assert!(error < 1e-10, "Solution accuracy: {}", error);
   ```

4. **Avoid Explicit Inversion**:
   - ❌ `x = inverse(a)?.matmul(&b)?;`
   - ✅ `x = solve_system(&a, &b)?;`

---

## 📚 Implementation References

### faer Library
- **Documentation**: https://docs.rs/faer/
- **GitHub**: https://github.com/sarah-ek/faer-rs
- **Benchmarks**: Competitive with OpenBLAS, MKL

### Algorithms Implemented in faer

- **LU**: Blocked algorithm with partial pivoting
- **QR**: Householder reflections (thin and full)
- **Cholesky**: Blocked Cholesky algorithm
- **SVD**: Golub-Reinsch algorithm with implicit shifts
- **Eigendecomp**: QR algorithm with Francis shifts

### Academic References

1. **Golub & Van Loan** - "Matrix Computations" (4th ed.)
   - Chapter 3: LU and Cholesky
   - Chapter 5: QR and Least Squares
   - Chapter 8: Symmetric Eigenvalue Problems
   - Chapter 9: QR Algorithm

2. **Trefethen & Bau** - "Numerical Linear Algebra"
   - Lecture 16: Stability of LU
   - Lectures 19-20: QR Factorization
   - Lectures 31-32: Eigenvalue Algorithms

3. **Demmel** - "Applied Numerical Linear Algebra"
   - Chapter 2: Floating Point Arithmetic
   - Chapter 4: Stability
   - Chapter 5: Conditioning

### Key Concepts from Literature

**Backward Stability** (Wilkinson):
> An algorithm is backward stable if the computed result is the exact result for a slightly perturbed input.

**Error Bounds**:
- LU: `||x_computed - x_exact|| / ||x_exact|| ≈ κ(A) · ε_machine`
- QR: `||Q_computed^T Q_computed - I|| ≈ ε_machine` (near-perfect orthogonality)

**Pivoting Strategies**:
- Partial: Swap rows, O(n²) overhead, sufficient for most cases
- Complete: Swap rows and columns, O(n³) overhead, theoretically better
- Rook: Compromise between partial and complete

---

## 🔍 Detailed Algorithm Walkthroughs

### LU Decomposition with Partial Pivoting

**Goal**: Factorize `A` into `P·A = L·U`

**Algorithm** (simplified):
```
Input: A (n×n matrix)
Output: L (lower triangular), U (upper triangular), P (permutation)

Initialize:
  L = zeros(n, n), U = A.copy(), P = identity_permutation(n)

For k = 0 to n-1:
  # Partial pivoting: find largest element in column k
  pivot_row = argmax(|U[k:n, k]|)
  if pivot_row != k:
    swap U[k, :] ↔ U[pivot_row, :]
    swap L[k, :k] ↔ L[pivot_row, :k]
    swap P[k] ↔ P[pivot_row]

  # Gaussian elimination
  L[k, k] = 1
  for i = k+1 to n-1:
    L[i, k] = U[i, k] / U[k, k]  # Multiplier
    U[i, k:n] -= L[i, k] * U[k, k:n]  # Row reduction

Return (L, U, P)
```

**Complexity**:
- Pivoting: O(n) per iteration × n iterations = O(n²)
- Elimination: O(n²) per iteration × n iterations = O(n³)
- **Total**: O(n³)

**Numerical Properties**:
- Partial pivoting ensures `|L[i,j]| ≤ 1`
- Growth factor `max(|U|) / max(|A|)` typically small in practice
- Guarantees: `||L||_∞ ≤ n`, but U can grow exponentially (rare)

### QR Decomposition (Householder)

**Goal**: Factorize `A = Q·R` where `Q` is orthogonal, `R` is upper triangular

**Algorithm** (Householder reflections):
```
Input: A (m×n matrix, m ≥ n)
Output: Q (m×m orthogonal), R (m×n upper triangular)

Initialize:
  Q = I, R = A

For k = 0 to n-1:
  # Compute Householder reflector to zero out R[k+1:m, k]
  x = R[k:m, k]
  v = x + sign(x[0]) * ||x|| * e_1  # Householder vector
  v = v / ||v||

  # Apply reflector: H = I - 2vv^T
  R[k:m, k:n] -= 2 * v * (v^T * R[k:m, k:n])
  Q[:, k:m] -= 2 * (Q[:, k:m] * v) * v^T

Return (Q, R)
```

**Why Householder?**
- Numerically stable (orthogonal transformations)
- Faster than Gram-Schmidt (O(2mn²) vs O(2mn² + mn))
- Can be implemented in blocked form for cache efficiency

**Properties**:
- `||Q||_2 = 1` (preserves norms)
- `Q^T Q = I` (exactly, up to machine precision)
- Backward stable: computed `Q` and `R` satisfy `A + E = Q·R` where `||E|| ≈ ε_machine ||A||`

### Power Iteration

**Goal**: Find dominant eigenvalue `λ_1` (largest magnitude)

**Algorithm**:
```
Input: A (n×n matrix), max_iterations, tolerance
Output: λ (dominant eigenvalue), v (corresponding eigenvector)

Initialize:
  v = random_vector(n)  # Or [1, 0, 0, ..., 0]
  λ = 0

For iter = 0 to max_iterations:
  w = A · v                    # Matrix-vector product
  λ_new = ||w||                # Rayleigh quotient approximation
  v = w / ||w||                # Normalize

  if |λ_new - λ| < tolerance:
    break
  λ = λ_new

Return (λ, v)
```

**Convergence Rate**:
- Linear convergence: `error_k ≈ (|λ_2| / |λ_1|)^k · error_0`
- Faster when `|λ_1| >> |λ_2|` (well-separated eigenvalues)
- Fails when `|λ_1| = |λ_2|` (multiple dominant eigenvalues)

**Improvements** (not implemented):
- Rayleigh quotient: `λ = (v^T A v) / (v^T v)` (more accurate)
- Inverse iteration: Use `A^(-1)` to find smallest eigenvalue
- Shift-and-invert: `(A - σI)^(-1)` to find eigenvalue near σ

---

## 🧬 Future Architectural Improvements

### 1. Sparse Matrix Support
**Current**: Only dense matrices (`Mat<f64>`)
**Planned**: Sparse formats (CSR, CSC, COO)

```rust
// Future API
pub enum MatrixFormat {
    Dense(RealTensor),
    Sparse(SparseMatrix),
}

pub fn sparse_solve(
    a: &SparseMatrix,
    b: &RealTensor,
    method: SolverMethod,
) -> Result<RealTensor, String> {
    match method {
        SolverMethod::ConjugateGradient => cg_solver(a, b),
        SolverMethod::BiCGStab => bicgstab_solver(a, b),
        SolverMethod::GMRES(restart) => gmres_solver(a, b, restart),
    }
}
```

### 2. Parallel Decompositions
**Current**: Single-threaded
**Planned**: Rayon-based parallelism

```rust
// Future parallel LU
use rayon::prelude::*;

pub fn parallel_lu_decomposition(tensor: &RealTensor, num_threads: usize)
    -> Result<(RealTensor, RealTensor, Vec<usize>), String>
{
    // Use blocked algorithm with parallel panel factorization
}
```

### 3. Complex Matrix Support
**Current**: Real matrices only, complex eigenvalues extracted
**Planned**: Full complex matrix operations

```rust
// Future complex support
use achronyme_types::tensor::ComplexTensor;

pub fn complex_eigenvalues(tensor: &ComplexTensor)
    -> Result<Vec<Complex>, String> { ... }

pub fn complex_svd(tensor: &ComplexTensor)
    -> Result<(ComplexTensor, Vec<f64>, ComplexTensor), String> { ... }
```

### 4. Builder Pattern for Fine-Tuned Algorithms
**Current**: Fixed algorithm parameters
**Planned**: Configurable solvers

```rust
// Future configurable API
pub struct LuDecomposer {
    pivoting: PivotStrategy,
    threshold: f64,
    block_size: usize,
}

impl LuDecomposer {
    pub fn new() -> Self { /* defaults */ }

    pub fn with_pivoting(mut self, strategy: PivotStrategy) -> Self {
        self.pivoting = strategy;
        self
    }

    pub fn decompose(&self, tensor: &RealTensor)
        -> Result<LuFactorization, String> { ... }
}

// Usage
let lu = LuDecomposer::new()
    .with_pivoting(PivotStrategy::Complete)
    .with_threshold(1e-12)
    .decompose(&matrix)?;
```

### 5. Incremental/Streaming Algorithms
**Current**: Batch processing only
**Planned**: Update decompositions incrementally

```rust
// Future incremental API
pub struct IncrementalQr {
    q: RealTensor,
    r: RealTensor,
}

impl IncrementalQr {
    pub fn new(initial_matrix: &RealTensor) -> Result<Self, String> { ... }

    pub fn add_column(&mut self, col: &RealTensor) -> Result<(), String> {
        // Update QR factorization with new column
    }

    pub fn get_qr(&self) -> (&RealTensor, &RealTensor) {
        (&self.q, &self.r)
    }
}
```

---

## 📝 Maintenance Checklist

### When Updating faer Version

- [ ] Review faer changelog for API changes
- [ ] Update conversion functions if needed
- [ ] Run full test suite: `cargo test -p achronyme-linalg`
- [ ] Benchmark performance (if benchmarks exist)
- [ ] Update `Cargo.toml` version constraint
- [ ] Document any breaking changes

### When Adding New Functions

- [ ] Implement in appropriate module (decompositions, eigenvalues, solvers)
- [ ] Add conversion helpers if needed
- [ ] Write comprehensive documentation (algorithm, complexity, use cases)
- [ ] Add unit tests (functionality, errors, edge cases)
- [ ] Export in `lib.rs`
- [ ] Update crate README with usage examples
- [ ] Update this implementation README

### When Fixing Bugs

- [ ] Write failing test that reproduces bug
- [ ] Implement fix
- [ ] Verify test passes
- [ ] Check for related issues in other modules
- [ ] Document fix in commit message
- [ ] Consider adding regression test

### Performance Optimization

- [ ] Profile with `cargo flamegraph`
- [ ] Identify bottlenecks (conversion vs. algorithm)
- [ ] Consider:
  - In-place operations
  - Blocked algorithms
  - SIMD optimizations
  - Parallelization
- [ ] Benchmark before and after
- [ ] Document performance characteristics

---

## 🎓 Learning Resources for Contributors

### Understanding Linear Algebra Algorithms
1. Start with Trefethen & Bau (most accessible)
2. Consult Golub & Van Loan for implementation details
3. Read faer source code for Rust-specific patterns

### Understanding faer
1. Read faer documentation: https://docs.rs/faer/
2. Study examples in faer repository
3. Join faer discussions on GitHub

### Rust Patterns Used
1. **Type conversions**: `From`, `Into` traits (not used here, but could be)
2. **Error handling**: `Result`, `?` operator, `map_err`
3. **Borrowing**: Understand when to use `&` vs owned values
4. **Testing**: `#[cfg(test)]`, `assert!`, `approx::assert_relative_eq!`

---

**Last Updated**: 2024-11-14
**Maintainer**: Achronyme Project Team
