# Matrix Decompositions Module

Comprehensive matrix factorization algorithms for the `achronyme-linalg` crate.

**Location**: `crates/achronyme-linalg/src/decompositions.rs`
**Lines of Code**: 272
**Target Audience**: Contributors implementing or extending decomposition algorithms

---

## 📋 Overview

This module provides four fundamental matrix decompositions:

1. **LU Decomposition** - General-purpose factorization for linear systems
2. **Cholesky Decomposition** - Efficient factorization for symmetric positive-definite matrices
3. **QR Decomposition** - Orthogonal-triangular factorization for least squares
4. **SVD (Singular Value Decomposition)** - Universal factorization revealing matrix structure

All decompositions are implemented as thin wrappers around the high-performance `faer` library, with type conversions to/from Achronyme's `RealTensor` type.

---

## 🏗️ Module Structure

```rust
decompositions.rs
│
├── Conversion Functions (Internal)
│   ├── tensor_to_faer_mat(&RealTensor) -> Mat<f64>
│   ├── faer_mat_to_tensor(Mat<f64>) -> Result<RealTensor, String>
│   └── faer_mat_ref_to_tensor(MatRef<f64>) -> Result<RealTensor, String>
│
├── Public Decomposition Functions
│   ├── lu_decomposition(&RealTensor) -> Result<(L, U, P), String>
│   ├── cholesky_decomposition(&RealTensor) -> Result<L, String>
│   ├── qr_decomposition(&RealTensor) -> Result<(Q, R), String>
│   └── svd_decomposition(&RealTensor) -> Result<(U, S, VT), String>
│
└── Tests
    ├── test_lu_decomposition()
    ├── test_cholesky_decomposition()
    ├── test_qr_decomposition()
    └── test_svd_decomposition()
```

---

## 🔧 Conversion Functions

### `tensor_to_faer_mat`

**Purpose**: Convert Achronyme `RealTensor` (matrix) to faer `Mat<f64>`

**Signature**:
```rust
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
```

**Preconditions**:
- `tensor.is_matrix()` must be true (asserted)

**Algorithm**:
```rust
Mat::from_fn(rows, cols, |i, j| tensor.get_matrix(i, j).unwrap())
```

**Performance**:
- Time: O(mn) for m×n matrix
- Space: Allocates new m×n matrix
- No zero-copy optimization (different memory layouts)

**Design Note**: Uses `assert!` because this is an internal function - public APIs validate before calling.

---

### `faer_mat_to_tensor`

**Purpose**: Convert faer `Mat<f64>` (owned) to Achronyme `RealTensor`

**Signature**:
```rust
fn faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String>
```

**Algorithm**:
1. Extract dimensions: `rows`, `cols`
2. Copy data row-by-row into `Vec<f64>`
3. Call `RealTensor::matrix(rows, cols, data)`
4. Propagate errors with `.map_err(|e| e.to_string())`

**Performance**:
- Time: O(mn)
- Space: Allocates Vec<f64> of size mn

**Error Handling**: Wraps tensor creation errors (rare, only on allocation failure)

---

### `faer_mat_ref_to_tensor`

**Purpose**: Convert faer `MatRef<f64>` (borrowed) to Achronyme `RealTensor`

**Signature**:
```rust
fn faer_mat_ref_to_tensor(mat: MatRef<f64>) -> Result<RealTensor, String>
```

**Difference from `faer_mat_to_tensor`**:
- Takes reference instead of owned value
- Used for SVD where faer returns references (U, V)
- Identical implementation (always copies data)

**Use Case**: When faer decomposition returns `MatRef` instead of owned `Mat`

---

## 🎯 Decomposition Algorithms

### 1. LU Decomposition

#### Overview

**Mathematical Definition**:
```
P·A = L·U
```
where:
- **P**: Permutation matrix (as vector of row indices)
- **L**: Lower triangular with 1s on diagonal
- **U**: Upper triangular

**Visual Representation**:
```
    ┌         ┐       ┌         ┐   ┌         ┐
    │ 2  1  1 │       │ 1  0  0 │   │ 8  7  9 │
P · │ 4  3  3 │   =   │ 0  1  0 │ × │ 0 -1 -3 │
    │ 8  7  9 │       │ 0  0  1 │   │ 0  0  0 │
    └         ┘       └         ┘   └         ┘
       A                  L              U

(After row permutation P)
```

#### Function Signature

```rust
pub fn lu_decomposition(tensor: &RealTensor)
    -> Result<(RealTensor, RealTensor, Vec<usize>), String>
```

**Returns**: `(L, U, P)` where:
- `L`: Lower triangular matrix (n×n)
- `U`: Upper triangular matrix (n×n)
- `P`: Permutation vector (length n), where `P[i]` is original row index

#### Algorithm (faer implementation)

1. **Partial Pivoting**:
   - For each column k, find row with largest absolute value
   - Swap rows to bring largest element to pivot position
   - Record swap in permutation matrix

2. **Gaussian Elimination**:
   - For each row below pivot: compute multiplier `L[i,k] = A[i,k] / A[k,k]`
   - Subtract multiplier × pivot row from current row
   - Store multipliers in L, update A to form U

3. **Complexity**:
   - Time: O(n³) - specifically ~(2/3)n³ flops
   - Space: O(n²) for L and U matrices

#### Numerical Stability

- **With Partial Pivoting**: Stable in practice
  - Growth factor typically small (~n in worst case, rare)
  - Ensures `|L[i,j]| ≤ 1` for all i,j

- **Error Bound**:
  ```
  ||A - L·U||_∞ ≤ n³ · ε · ||A||_∞
  ```
  where ε is machine epsilon (~2.22×10⁻¹⁶ for f64)

#### Use Cases

1. **Solving Linear Systems**: `Ax = b`
   - Decompose: `A = P^T·L·U`
   - Solve: `L·y = P·b` (forward substitution)
   - Then: `U·x = y` (backward substitution)

2. **Matrix Inversion**: `A^(-1)`
   - Solve `A·X = I` column-by-column

3. **Determinant**:
   - `det(A) = det(P) · det(L) · det(U) = (-1)^(swaps) · ∏ U[i,i]`

4. **Matrix Rank**:
   - Count non-zero diagonal elements in U

#### Code Example

```rust
use achronyme_linalg::lu_decomposition;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(3, 3, vec![
    2.0, 1.0, 1.0,
    4.0, 3.0, 3.0,
    8.0, 7.0, 9.0
]).unwrap();

let (l, u, p) = lu_decomposition(&a).unwrap();

// Verify dimensions
assert_eq!(l.rows(), 3);
assert_eq!(l.cols(), 3);
assert_eq!(u.rows(), 3);
assert_eq!(u.cols(), 3);

// P is permutation: [original_row_0, original_row_1, original_row_2]
println!("Permutation: {:?}", p);
```

#### Error Conditions

- **Non-square matrix**: Returns `Err("LU decomposition requires square matrix")`
- **Singular matrix**: Completes but U has zero on diagonal
- **Tensor creation failure**: Propagates error from `faer_mat_to_tensor`

---

### 2. Cholesky Decomposition

#### Overview

**Mathematical Definition**:
```
A = L·L^T
```
where:
- **L**: Lower triangular matrix with positive diagonal
- **A**: Symmetric positive-definite matrix

**Visual Representation**:
```
┌         ┐       ┌         ┐   ┌         ┐
│ 4  2  1 │       │ 2  0  0 │   │ 2  1  0.5│
│ 2  3  1 │   =   │ 1  1.41 0│ × │ 0  1.41 0.35│
│ 1  1  2 │       │ 0.5 0.35 1.22│   │ 0  0  1.22│
└         ┘       └         ┘   └         ┘
   A                  L              L^T
```

#### Function Signature

```rust
pub fn cholesky_decomposition(tensor: &RealTensor)
    -> Result<RealTensor, String>
```

**Returns**: `L` (lower triangular matrix)

#### Algorithm (faer implementation)

1. **Check Positive Definiteness**: Implicitly during factorization
2. **Compute L**:
   ```
   For j = 0 to n-1:
     L[j,j] = sqrt(A[j,j] - Σ(k=0 to j-1) L[j,k]²)
     For i = j+1 to n-1:
       L[i,j] = (A[i,j] - Σ(k=0 to j-1) L[i,k]·L[j,k]) / L[j,j]
   ```

3. **Complexity**:
   - Time: O(n³/3) - exactly half of LU (exploits symmetry)
   - Space: O(n²) for L

#### Numerical Stability

- **Stable for SPD matrices**: No pivoting needed
- **Requires**: All leading principal minors must be positive
- **Fails if**: Matrix not positive definite (sqrt of negative number)
- **Condition Number**: `κ(L) = √κ(A)` (better conditioned than A)

#### Use Cases

1. **Efficient Linear Solves** (SPD systems):
   - 2× faster than LU for symmetric positive-definite A

2. **Monte Carlo Simulation**:
   - Generate correlated random variables: `x = L·z` where z ~ N(0,I)

3. **Optimization**:
   - Hessian matrices in Newton's method (if positive definite)

4. **Positive Definiteness Test**:
   - Matrix is SPD ⟺ Cholesky succeeds

#### Code Example

```rust
use achronyme_linalg::cholesky_decomposition;
use achronyme_types::tensor::RealTensor;

// Symmetric positive-definite matrix
let a = RealTensor::matrix(3, 3, vec![
    4.0, 2.0, 1.0,
    2.0, 3.0, 1.0,
    1.0, 1.0, 2.0
]).unwrap();

let l = cholesky_decomposition(&a).unwrap();

// Verify A ≈ L·L^T (future reconstruction test)
let lt = l.transpose().unwrap();
let reconstructed = l.matmul(&lt).unwrap();
```

#### Error Conditions

- **Non-square matrix**: `Err("Cholesky decomposition requires square matrix")`
- **Not positive definite**: `Err("Cholesky decomposition failed (matrix not positive definite?)")`
- **Not symmetric**: May succeed or fail depending on numerical properties

**Best Practice**: Check `is_symmetric(a, 1e-10)` before calling Cholesky

---

### 3. QR Decomposition

#### Overview

**Mathematical Definition**:
```
A = Q·R
```
where:
- **Q**: Orthogonal matrix (Q^T·Q = I), size m×m or m×n (thin)
- **R**: Upper triangular matrix, size m×n or n×n (thin)

**Visual Representation** (thin QR for tall matrix):
```
┌     ┐       ┌       ┐   ┌     ┐
│ 1 1 │       │ -0.58 │   │ -1.73 -3.46│
│ 1 2 │   =   │ -0.58 │ × │  0    1.15│
│ 1 3 │       │ -0.58 │   └     ┘
└     ┘       └       ┘      R (2×2)
A (3×2)       Q (3×2)
```

#### Function Signature

```rust
pub fn qr_decomposition(tensor: &RealTensor)
    -> Result<(RealTensor, RealTensor), String>
```

**Returns**: `(Q, R)` where:
- `Q`: Orthogonal matrix (thin QR: m×n for m≥n)
- `R`: Upper triangular (thin QR: n×n)

#### Algorithm (faer - Householder reflections)

1. **Householder Reflections**:
   - For each column k, construct reflector to zero out elements below diagonal
   - Reflector: `H_k = I - 2·v_k·v_k^T` where `v_k` is Householder vector

2. **Thin vs Full QR**:
   - **Thin** (implemented): Q is m×n, R is n×n (for m≥n)
   - **Full**: Q is m×m, R is m×n

3. **Complexity**:
   - Time: O(2mn² - (2/3)n³) for m×n matrix (m ≥ n)
   - Space: O(mn) for Q and R

#### Numerical Stability

- **Excellent stability**: Householder reflections are orthogonal transformations
- **Backward stable**: Computed Q and R satisfy `A + E = Q·R` where `||E|| ≈ ε||A||`
- **Preserves Norms**: `||Q·x||_2 = ||x||_2` (isometry)
- **Orthogonality**: `||Q^T·Q - I||_F ≈ ε` (machine precision orthogonality)

#### Use Cases

1. **Least Squares Problems**:
   - Minimize `||Ax - b||_2` → Solve `Rx = Q^T·b`

2. **Eigenvalue Computation**:
   - QR algorithm iterates `A_k = Q_k·R_k`, `A_{k+1} = R_k·Q_k`

3. **Orthogonalization**:
   - Gram-Schmidt alternative (more stable)

4. **Rank Determination**:
   - Diagonal of R reveals rank (count non-zero elements)

#### Code Example

```rust
use achronyme_linalg::qr_decomposition;
use achronyme_types::tensor::RealTensor;

// Tall matrix (3×2)
let a = RealTensor::matrix(3, 2, vec![
    1.0, 1.0,
    1.0, 2.0,
    1.0, 3.0
]).unwrap();

let (q, r) = qr_decomposition(&a).unwrap();

// Q is 3×2 (thin), R is 2×2
assert_eq!(q.rows(), 3);
assert_eq!(q.cols(), 2);
assert_eq!(r.rows(), 2);
assert_eq!(r.cols(), 2);

// Verify orthogonality: Q^T·Q ≈ I (future test)
let qt = q.transpose().unwrap();
let qtq = qt.matmul(&q).unwrap();
// Should be 2×2 identity matrix
```

#### Error Conditions

- **Tensor creation failure**: Only error is from `faer_mat_to_tensor`
- **Works for all matrices**: No mathematical preconditions (unlike Cholesky)

**Note**: Rectangular matrices (m < n) will return Q with fewer columns than rows

---

### 4. SVD (Singular Value Decomposition)

#### Overview

**Mathematical Definition**:
```
A = U·Σ·V^T
```
where:
- **U**: Left singular vectors (m×m or m×r for thin SVD)
- **Σ**: Singular values (diagonal, r×r where r = min(m,n))
- **V^T**: Right singular vectors transposed (n×n or r×n for thin SVD)

**Visual Representation**:
```
┌       ┐       ┌       ┐   ┌     ┐   ┌       ┐
│ 1  2  │       │ -0.23 │   │ 9.52│   │ -0.62 -0.78│
│ 3  4  │   =   │ -0.52 │ × │ 0   │ × └       ┘
│ 5  6  │       │ -0.82 │   │ 0.51│      V^T
└       ┘       └       ┘   └     ┘
A (3×2)         U (3×2)    Σ (2)

(Thin SVD)
```

#### Function Signature

```rust
pub fn svd_decomposition(tensor: &RealTensor)
    -> Result<(RealTensor, Vec<f64>, RealTensor), String>
```

**Returns**: `(U, singular_values, V^T)` where:
- `U`: Left singular vectors (m×r)
- `singular_values`: Vec<f64> of length r = min(m,n), **sorted descending**
- `V^T`: Right singular vectors transposed (r×n)

**Note**: Returns `V^T` directly (user-friendly), not V

#### Algorithm (faer - Golub-Reinsch)

1. **Bidiagonalization**:
   - Reduce A to bidiagonal form using Householder reflections
   - `A = U_1·B·V_1^T` where B is bidiagonal

2. **QR Iteration**:
   - Iteratively diagonalize B using implicit QR with shifts
   - Produces diagonal Σ and updates U, V

3. **Complexity**:
   - Time: O(min(m²n, mn²)) - typically slower than LU/QR
   - Space: O(m² + n²) for full SVD, O(mr + nr) for thin SVD

#### Numerical Stability

- **Most Stable Decomposition**: Gold standard for accuracy
- **Relative Accuracy**: Computes small singular values accurately
- **Condition Number**: `κ(A) = σ_max / σ_min` (from SVD)
- **Backward Stable**: `A + E = U·Σ·V^T` where `||E|| ≈ ε||A||`

#### Use Cases

1. **Principal Component Analysis (PCA)**:
   - U columns are principal components
   - σ_i² are variances explained by each component

2. **Low-Rank Approximation**:
   - Best rank-k approximation: `A_k = U_k·Σ_k·V_k^T` (first k singular values)

3. **Pseudoinverse**:
   - `A^† = V·Σ^†·U^T` where `Σ^†[i,i] = 1/σ_i` if `σ_i > 0`, else 0

4. **Condition Number**:
   - `κ(A) = σ_max / σ_min` directly from singular values

5. **Matrix Rank**:
   - Count singular values > tolerance

6. **Image Compression**:
   - Truncate small singular values for lossy compression

#### Code Example

```rust
use achronyme_linalg::svd_decomposition;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(3, 2, vec![
    1.0, 2.0,
    3.0, 4.0,
    5.0, 6.0
]).unwrap();

let (u, s, vt) = svd_decomposition(&a).unwrap();

// U is 3×2, S has 2 values, VT is 2×2
assert_eq!(u.rows(), 3);
assert_eq!(u.cols(), 2);
assert_eq!(s.len(), 2);
assert_eq!(vt.rows(), 2);
assert_eq!(vt.cols(), 2);

// Singular values are sorted descending
assert!(s[0] >= s[1]);

// Compute condition number
let cond_number = s[0] / s[s.len()-1];
println!("Condition number: {}", cond_number);

// Low-rank approximation: keep largest singular value
// A_1 ≈ σ_0 · u_0 · v_0^T
```

#### Error Conditions

- **Tensor creation failure**: Only errors from `faer_mat_ref_to_tensor`
- **Works for all matrices**: No preconditions (most general decomposition)

---

## 🧪 Testing Strategy

### Current Tests

1. **Smoke Tests**: Verify decompositions don't crash
   ```rust
   #[test]
   fn test_lu_decomposition() {
       let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
       let result = lu_decomposition(&a);
       assert!(result.is_ok());
   }
   ```

2. **Dimension Verification**: Check output shapes
   ```rust
   let (l, u, _p) = result.unwrap();
   assert_eq!(l.rows(), 3);
   assert_eq!(l.cols(), 3);
   ```

### Future Test Enhancements

#### Reconstruction Tests
```rust
#[test]
fn test_lu_reconstruction() {
    let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
    let (l, u, p) = lu_decomposition(&a).unwrap();

    // Apply permutation to A
    let pa = apply_permutation(&a, &p);

    // Verify P·A ≈ L·U
    let lu = l.matmul(&u).unwrap();
    assert_matrices_approx_eq(&pa, &lu, 1e-10);
}
```

#### Property-Based Tests
```rust
#[test]
fn test_cholesky_properties() {
    let a = generate_spd_matrix(5);
    let l = cholesky_decomposition(&a).unwrap();

    // L should be lower triangular
    assert!(is_lower_triangular(&l, 1e-10));

    // Diagonal elements should be positive
    for i in 0..5 {
        assert!(l.get_matrix(i, i).unwrap() > 0.0);
    }
}
```

#### Numerical Accuracy Tests
```rust
#[test]
fn test_qr_orthogonality() {
    let a = RealTensor::matrix(5, 3, vec![...]).unwrap();
    let (q, _r) = qr_decomposition(&a).unwrap();

    // Q^T·Q ≈ I
    let qt = q.transpose().unwrap();
    let qtq = qt.matmul(&q).unwrap();
    let identity = RealTensor::eye(3);

    assert_matrices_approx_eq(&qtq, &identity, 1e-10);
}
```

---

## 📊 Performance Characteristics

### Computational Complexity

| Decomposition | Time Complexity | Space Complexity | Relative Speed |
|---------------|----------------|------------------|----------------|
| LU            | O(2n³/3)       | O(n²)           | 1.0× (baseline) |
| Cholesky      | O(n³/3)        | O(n²)           | 2.0× (fastest) |
| QR (Householder) | O(2mn² - 2n³/3) | O(mn)        | 0.7× |
| SVD           | O(min(4m²n, 4mn²)) | O(m²+n²)     | 0.2× (slowest) |

### When to Use Each

**Decision Tree**:
```
Is matrix square?
├─ Yes
│  ├─ Is it symmetric positive-definite?
│  │  └─ Yes → Cholesky (fastest, most efficient)
│  └─ No → LU (general-purpose, stable with pivoting)
│
└─ No (rectangular)
   ├─ Need orthogonal factorization?
   │  └─ Yes → QR (least squares, orthogonalization)
   └─ Need complete spectral information?
      └─ Yes → SVD (most stable, most expensive)

Special cases:
- Ill-conditioned matrix → SVD (most accurate)
- Rank determination → SVD or QR
- Eigenvalue computation → QR algorithm (iterative)
```

### Optimization Opportunities

1. **Blocked Algorithms**: faer uses cache-friendly blocked implementations
2. **SIMD**: faer leverages SIMD when available (x86_64, aarch64)
3. **In-Place**: Some decompositions can overwrite input (not exposed yet)
4. **Parallelization**: Could parallelize panel factorization (future)

---

## 🔮 Future Enhancements

### Planned Additions

1. **Eigenvalue Decomposition** (currently in `eigenvalues.rs`)
   - Could be viewed as special decomposition: `A = V·Λ·V^(-1)`

2. **Schur Decomposition**: `A = Q·T·Q^T`
   - T is upper triangular (complex) or quasi-upper triangular (real)
   - Useful for eigenvalue computation

3. **Hessenberg Reduction**: `A = Q·H·Q^T`
   - H is Hessenberg (upper triangular + one subdiagonal)
   - Preprocessing for eigenvalue algorithms

4. **Polar Decomposition**: `A = U·P`
   - U is orthogonal, P is symmetric positive-definite
   - Useful in continuum mechanics

5. **Rank-Revealing QR**: RRQR with column pivoting
   - Better rank determination than standard QR

### API Enhancements

```rust
// Builder pattern for fine-tuned decompositions
pub struct SvdDecomposer {
    thin: bool,
    compute_u: bool,
    compute_v: bool,
}

impl SvdDecomposer {
    pub fn new() -> Self { /* defaults */ }
    pub fn thin(mut self, thin: bool) -> Self { ... }
    pub fn compute_u(mut self, compute: bool) -> Self { ... }

    pub fn decompose(&self, tensor: &RealTensor)
        -> Result<SvdResult, String> { ... }
}
```

### Complex Matrix Support

```rust
use achronyme_types::tensor::ComplexTensor;

pub fn complex_lu_decomposition(tensor: &ComplexTensor)
    -> Result<(ComplexTensor, ComplexTensor, Vec<usize>), String> { ... }

pub fn complex_svd_decomposition(tensor: &ComplexTensor)
    -> Result<(ComplexTensor, Vec<f64>, ComplexTensor), String> { ... }
```

---

## 📚 References

### Textbooks
1. **Golub & Van Loan** - "Matrix Computations" (4th ed.)
   - Chapter 3: General Linear Systems (LU)
   - Chapter 4: Special Linear Systems (Cholesky, Banded)
   - Chapter 5: Orthogonalization and Least Squares (QR)
   - Chapter 8: Symmetric Eigenvalue Problems
   - Chapter 9: The SVD

2. **Trefethen & Bau** - "Numerical Linear Algebra"
   - Lectures 20-21: QR Factorization (Householder)
   - Lectures 22-23: Least Squares
   - Lectures 31-32: Eigenvalue Algorithms
   - Lectures 36-37: SVD

### Papers
- Golub & Reinsch (1970): "Singular value decomposition and least squares solutions"
- Wilkinson (1965): "The Algebraic Eigenvalue Problem"

### Online Resources
- faer documentation: https://docs.rs/faer/
- LAPACK Users' Guide: https://netlib.org/lapack/lug/

---

**Module Maintainer**: Achronyme Project Team
**Last Updated**: 2024-11-14
