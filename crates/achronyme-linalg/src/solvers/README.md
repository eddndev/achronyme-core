# Linear System Solvers and Matrix Analysis Module

Comprehensive linear system solving and matrix property analysis for the `achronyme-linalg` crate.

**Location**: `crates/achronyme-linalg/src/solvers.rs`
**Lines of Code**: 338
**Target Audience**: Contributors implementing or extending solver algorithms and matrix analysis tools

---

## 📋 Overview

This module provides five categories of operations:

1. **Linear System Solving** - Solve Ax = b for unknown vector x
2. **Matrix Inversion** - Compute multiplicative inverse A^(-1)
3. **Determinant Computation** - Calculate det(A) efficiently
4. **Matrix Property Checks** - Test for symmetry, positive-definiteness
5. **Vector Operations** - Support utilities for linear algebra

All solvers use LU decomposition via the `faer` library for numerical stability and efficiency.

---

## 🏗️ Module Structure

```rust
solvers.rs
│
├── Conversion Functions (Internal)
│   ├── tensor_to_faer_mat(&RealTensor) -> Mat<f64>
│   ├── faer_mat_to_tensor(Mat<f64>) -> Result<RealTensor, String>
│   ├── tensor_to_faer_col(&RealTensor) -> Col<f64>
│   └── faer_col_to_tensor(Col<f64>) -> RealTensor
│
├── Public Solver Functions
│   ├── determinant_nd(&RealTensor) -> Result<f64, String>
│   ├── inverse(&RealTensor) -> Result<RealTensor, String>
│   ├── solve_system(&RealTensor, &RealTensor) -> Result<RealTensor, String>
│   ├── is_symmetric(&RealTensor, f64) -> bool
│   └── is_positive_definite(&RealTensor) -> bool
│
└── Tests
    ├── test_determinant_2x2()
    ├── test_determinant_3x3()
    ├── test_inverse_2x2()
    ├── test_solve_system_2x2()
    ├── test_solve_system_dimension_mismatch()
    └── test_inverse_singular_fails()
```

---

## 🔧 Conversion Functions

### Matrix Conversions

#### `tensor_to_faer_mat`

**Purpose**: Convert Achronyme `RealTensor` (matrix) to faer `Mat<f64>`

**Signature**:
```rust
fn tensor_to_faer_mat(tensor: &RealTensor) -> Mat<f64>
```

**Preconditions**: `tensor.is_matrix()` (asserted)

**Implementation**: Same as other modules (duplicated for independence)

---

#### `faer_mat_to_tensor`

**Purpose**: Convert faer `Mat<f64>` to Achronyme `RealTensor`

**Signature**:
```rust
fn faer_mat_to_tensor(mat: Mat<f64>) -> Result<RealTensor, String>
```

**Algorithm**: Copy data row-by-row, create tensor

---

### Vector Conversions

#### `tensor_to_faer_col`

**Purpose**: Convert Achronyme vector (rank-1 tensor) to faer `Col<f64>`

**Signature**:
```rust
fn tensor_to_faer_col(tensor: &RealTensor) -> Col<f64>
```

**Preconditions**: `tensor.is_vector()` (asserted)

**Implementation**:
```rust
fn tensor_to_faer_col(tensor: &RealTensor) -> Col<f64> {
    assert!(tensor.is_vector());
    let data = tensor.data();
    Col::from_fn(data.len(), |i| data[i])
}
```

**Usage**: Converting right-hand side `b` in `Ax = b`

---

#### `faer_col_to_tensor`

**Purpose**: Convert faer `Col<f64>` to Achronyme vector

**Signature**:
```rust
fn faer_col_to_tensor(col: Col<f64>) -> RealTensor
```

**Note**: Cannot fail (always creates valid vector)

**Implementation**:
```rust
fn faer_col_to_tensor(col: Col<f64>) -> RealTensor {
    let data: Vec<f64> = (0..col.nrows()).map(|i| col.read(i)).collect();
    RealTensor::vector(data)  // Infallible for valid data
}
```

**Usage**: Converting solution `x` back to Achronyme type

---

## 🎯 Solver Algorithms

### 1. Determinant Computation

#### Mathematical Background

**Determinant** measures:
- Volume scaling factor of linear transformation
- Invertibility (det ≠ 0 ⟺ matrix invertible)
- Orientation (sign of det)

**Properties**:
```
det(AB) = det(A) · det(B)
det(A^T) = det(A)
det(A^(-1)) = 1 / det(A)
det(cA) = c^n · det(A)    (for n×n matrix)
```

**Geometric Interpretation** (2D):
```
det([a c]) = ad - bc = area of parallelogram
    [b d]
```

#### Function Signature

```rust
pub fn determinant_nd(tensor: &RealTensor) -> Result<f64, String>
```

**Returns**: Scalar determinant value

**Name**: `determinant_nd` = "n-dimensional" (works for any size n×n)

#### Algorithm (LU-based)

**Key Insight**: Determinant from LU decomposition:
```
det(A) = det(P) · det(L) · det(U)
       = (-1)^(swaps) · 1 · ∏ U[i,i]
```

**Detailed Steps**:

1. **LU Decomposition with Partial Pivoting**:
   ```rust
   let mat = tensor_to_faer_mat(tensor);
   let lu = mat.partial_piv_lu();
   ```

2. **Compute det(U)**: Product of diagonal elements
   ```rust
   let u = lu.compute_u();
   let mut det = 1.0;
   for i in 0..u.nrows() {
       det *= u.read(i, i);
   }
   ```

3. **Compute det(P)**: Permutation sign
   ```rust
   // Count number of swaps in permutation
   let perm = lu.row_permutation();
   let perm_indices = perm.arrays().0;

   // Analyze permutation cycles
   let mut swaps = 0;
   let mut visited = vec![false; n];

   for i in 0..n {
       if !visited[i] && perm_indices[i] != i {
           let mut j = i;
           while !visited[j] {
               visited[j] = true;
               j = perm_indices[j];
               swaps += 1;
           }
           swaps -= 1;  // Cycle of length k needs k-1 swaps
       }
   }

   // Apply sign
   if swaps % 2 == 1 {
       det = -det;
   }
   ```

4. **Return**: Final determinant value

#### Complexity

- **Time**: O(n³) for LU decomposition, O(n) for sign and product
- **Space**: O(n²) for L and U matrices
- **Total**: O(n³) - same as LU decomposition

#### Why LU Instead of Cofactor Expansion?

**Cofactor Expansion** (recursive):
```
det(A) = Σ a[0,j] · (-1)^j · det(M[0,j])
```
- **Complexity**: O(n!) - exponential, unusable for n > 10

**LU Decomposition**:
- **Complexity**: O(n³) - polynomial, scales to large matrices
- **Stability**: Partial pivoting ensures numerical accuracy

#### Numerical Considerations

**Underflow/Overflow**:
- Large matrices can have very large/small determinants
- Example: 100×100 identity has det = 1, but intermediate products may overflow

**Singularity Detection**:
- det ≈ 0 indicates near-singularity
- Use tolerance: `|det| < ε` for some ε (e.g., 1e-10)

**Condition Number**:
- Determinant alone doesn't indicate conditioning
- Small det ≠ ill-conditioned (e.g., 0.001·I has det = 10^(-9) but κ = 1)

#### Code Example

```rust
use achronyme_linalg::determinant_nd;
use achronyme_types::tensor::RealTensor;

// 2×2 matrix
let a = RealTensor::matrix(2, 2, vec![
    4.0, 7.0,
    2.0, 6.0
]).unwrap();

let det = determinant_nd(&a).unwrap();
// det = 4*6 - 7*2 = 24 - 14 = 10
println!("Determinant: {}", det);

// Check invertibility
if det.abs() > 1e-10 {
    println!("Matrix is invertible");
} else {
    println!("Matrix is singular or near-singular");
}

// 3×3 matrix
let b = RealTensor::matrix(3, 3, vec![
    1.0, 2.0, 3.0,
    0.0, 1.0, 4.0,
    5.0, 6.0, 0.0
]).unwrap();

let det_b = determinant_nd(&b).unwrap();
// det(B) = 1*(1*0 - 4*6) - 2*(0*0 - 4*5) + 3*(0*6 - 1*5)
//        = 1*(-24) - 2*(-20) + 3*(-5)
//        = -24 + 40 - 15 = 1
println!("Determinant of 3×3: {}", det_b);
```

#### Error Conditions

- **Non-square matrix**: `Err("Determinant requires square matrix")`
- **Numerical issues**: Very large/small determinants may lose precision

---

### 2. Matrix Inverse

#### Mathematical Background

**Inverse** of matrix A (denoted A^(-1)) satisfies:
```
A · A^(-1) = A^(-1) · A = I
```

**Existence**:
- Inverse exists ⟺ det(A) ≠ 0
- Unique when it exists

**Properties**:
```
(AB)^(-1) = B^(-1) · A^(-1)    (reverse order)
(A^T)^(-1) = (A^(-1))^T
(A^(-1))^(-1) = A
det(A^(-1)) = 1 / det(A)
```

#### Function Signature

```rust
pub fn inverse(tensor: &RealTensor) -> Result<RealTensor, String>
```

**Returns**: Inverse matrix A^(-1)

#### Algorithm (LU-based)

**Idea**: Solve `A·X = I` for X = A^(-1)

**Steps**:

1. **Convert to faer**:
   ```rust
   let mat = tensor_to_faer_mat(tensor);
   let n = mat.nrows();
   ```

2. **Create Identity Matrix**:
   ```rust
   let identity = Mat::<f64>::identity(n, n);
   ```

3. **LU Decomposition**:
   ```rust
   let lu = mat.partial_piv_lu();
   ```

4. **Solve System**:
   ```rust
   let inv = lu.solve(&identity);
   // Solves A·X = I column-by-column
   ```

5. **Convert Back**:
   ```rust
   faer_mat_to_tensor(inv)
   ```

#### Complexity

- **Time**: O(n³) for LU decomposition, O(n³) for solving n systems
- **Total**: O(n³) (constant factor ~2-3× single solve)
- **Space**: O(n²) for inverse matrix

#### Why Not Use Inverse Directly?

**Common Misconception**:
```rust
// ❌ Inefficient and less accurate
let x = inverse(&a)?.matmul(&b)?;

// ✅ Better: Direct solve
let x = solve_system(&a, &b)?;
```

**Reasons**:
1. **Efficiency**: Solving Ax = b is O(n³), computing A^(-1) then multiplying is 2·O(n³)
2. **Accuracy**: Direct solve has better numerical properties
3. **Stability**: Avoids forming A^(-1) explicitly

**When to Use Inverse**:
- Solving many systems with same A (but consider LU factorization instead)
- Theoretical analysis (e.g., Sherman-Morrison formula)
- Small matrices where convenience matters

#### Numerical Stability

**Condition Number Warning**:
```
||x_computed - x_exact|| / ||x_exact|| ≈ κ(A) · ε_machine
```

**Ill-Conditioned Matrices**:
- Large κ(A) → inverse is inaccurate
- Check condition number via SVD before inverting

**Singular/Near-Singular**:
- faer may not error on near-singular matrices
- Result will be numerically unreliable
- Check: `det(A)` or `cond(A)`

#### Code Example

```rust
use achronyme_linalg::inverse;
use achronyme_types::tensor::RealTensor;

let a = RealTensor::matrix(2, 2, vec![
    4.0, 7.0,
    2.0, 6.0
]).unwrap();

let a_inv = inverse(&a).unwrap();

println!("A^(-1):");
for i in 0..2 {
    for j in 0..2 {
        print!("{:.4} ", a_inv.get_matrix(i, j).unwrap());
    }
    println!();
}

// Verify: A * A^(-1) = I
let product = a.matmul(&a_inv).unwrap();
let identity = RealTensor::eye(2);

for i in 0..2 {
    for j in 0..2 {
        let diff = (product.get_matrix(i, j).unwrap() -
                    identity.get_matrix(i, j).unwrap()).abs();
        assert!(diff < 1e-10, "Verification failed at ({}, {})", i, j);
    }
}

println!("Verification: A * A^(-1) = I ✓");
```

#### Error Conditions

- **Non-square matrix**: `Err("Inverse requires square matrix")`
- **Singular matrix**: May succeed numerically but result is unreliable
- **Near-singular**: No explicit error, but inverse is inaccurate

**Best Practice**:
```rust
let det = determinant_nd(&a)?;
if det.abs() < 1e-10 {
    return Err("Matrix is singular or near-singular".to_string());
}
let a_inv = inverse(&a)?;
```

---

### 3. Linear System Solver

#### Mathematical Background

**Linear System**:
```
A·x = b
```
where:
- **A**: Coefficient matrix (n×n)
- **x**: Unknown vector (n×1)
- **b**: Right-hand side vector (n×1)

**Solution Existence**:
- **Unique solution**: det(A) ≠ 0 (non-singular)
- **No solution**: System inconsistent (overdetermined)
- **Infinite solutions**: Underdetermined or singular

#### Function Signature

```rust
pub fn solve_system(a: &RealTensor, b: &RealTensor)
    -> Result<RealTensor, String>
```

**Parameters**:
- `a`: Coefficient matrix (must be rank-2 tensor)
- `b`: Right-hand side (must be rank-1 tensor, vector)

**Returns**: Solution vector `x`

#### Algorithm (LU with Forward/Backward Substitution)

**Steps**:

1. **Validate Inputs**:
   ```rust
   if !a.is_matrix() || !b.is_vector() {
       return Err("`a` must be a matrix and `b` must be a vector".to_string());
   }
   if a.rows() != b.size() {
       return Err(format!(
           "Dimension mismatch: matrix has {} rows but vector has {} elements",
           a.rows(), b.size()
       ));
   }
   ```

2. **Convert to faer**:
   ```rust
   let a_mat = tensor_to_faer_mat(a);
   let b_col = tensor_to_faer_col(b);
   ```

3. **LU Decomposition**:
   ```rust
   let lu = a_mat.partial_piv_lu();
   ```

4. **Solve via Forward/Backward Substitution**:
   ```rust
   let x = lu.solve(&b_col);
   ```

   Internally:
   - **Forward substitution**: Solve `L·y = P·b` for `y`
   - **Backward substitution**: Solve `U·x = y` for `x`

5. **Convert Back**:
   ```rust
   Ok(faer_col_to_tensor(x))
   ```

#### Complexity

- **LU Decomposition**: O(n³) - one-time cost
- **Forward Substitution**: O(n²)
- **Backward Substitution**: O(n²)
- **Total**: O(n³) for single solve
- **Multiple RHS**: Reuse LU, only O(n²) per additional solve

#### Numerical Stability

**Error Bound**:
```
||x_computed - x_exact|| / ||x_exact|| ≤ κ(A) · ε_machine
```

**Condition Number κ(A)**:
- **Well-conditioned**: κ(A) ≈ 1-100 → accurate solution
- **Moderately conditioned**: κ(A) ≈ 10³-10⁶ → some precision loss
- **Ill-conditioned**: κ(A) > 10¹⁰ → unreliable solution

**Partial Pivoting**:
- Ensures stability for most matrices
- Grows factor typically O(n) in practice (O(2^n) worst case, rare)

#### Iterative Refinement (Future Enhancement)

**Idea**: Improve solution accuracy
```
1. Solve: A·x₀ = b
2. Compute residual: r = b - A·x₀
3. Solve correction: A·δx = r
4. Update: x₁ = x₀ + δx
5. Repeat until ||r|| < tolerance
```

#### Code Example

```rust
use achronyme_linalg::solve_system;
use achronyme_types::tensor::RealTensor;

// System: 3x + y = 9
//         x + 2y = 8
// Solution: x = 2, y = 3

let a = RealTensor::matrix(2, 2, vec![
    3.0, 1.0,
    1.0, 2.0
]).unwrap();

let b = RealTensor::vector(vec![9.0, 8.0]);

let x = solve_system(&a, &b).unwrap();

println!("Solution: x = {:?}", x.data());
// Should print: [2.0, 3.0]

// Verify solution: A·x = b
let ax = a.matmul(&x.to_column_matrix()).unwrap();  // Convert x to n×1 matrix
let b_mat = b.to_column_matrix();

for i in 0..2 {
    let diff = (ax.get_matrix(i, 0).unwrap() -
                b_mat.get_matrix(i, 0).unwrap()).abs();
    assert!(diff < 1e-10, "Verification failed at row {}", i);
}

println!("Verification: A·x = b ✓");
```

#### Advanced Example (Multiple RHS)

```rust
// Solve A·X = B where B has multiple columns
// Current API: solve one column at a time

let a = RealTensor::matrix(3, 3, vec![...]).unwrap();
let b1 = RealTensor::vector(vec![1.0, 2.0, 3.0]);
let b2 = RealTensor::vector(vec![4.0, 5.0, 6.0]);

// Solve for each column
let x1 = solve_system(&a, &b1)?;
let x2 = solve_system(&a, &b2)?;

// Future: Batch solve API
// let X = solve_system_multiple(&a, &B)?;
```

#### Error Conditions

- **Non-matrix A**: `Err("`a` must be a matrix and `b` must be a vector")`
- **Non-vector b**: Same error
- **Dimension mismatch**: `Err("Dimension mismatch: matrix has ... rows but vector has ... elements")`
- **Singular matrix**: May not error explicitly, but solution unreliable

**Robust Usage**:
```rust
// Check determinant before solving
let det = determinant_nd(&a)?;
if det.abs() < 1e-10 {
    return Err("Matrix is singular, system may not have unique solution".to_string());
}

let x = solve_system(&a, &b)?;

// Verify residual
let residual = a.matmul(&x)?.sub(&b)?;
let rel_error = residual.norm() / b.norm();
if rel_error > 1e-8 {
    eprintln!("Warning: Large residual error: {}", rel_error);
}
```

---

### 4. Matrix Property Checks

#### Symmetry Check (`is_symmetric`)

##### Mathematical Definition

**Symmetric Matrix**:
```
A = A^T  ⟺  A[i,j] = A[j,i]  for all i,j
```

##### Function Signature

```rust
pub fn is_symmetric(tensor: &RealTensor, tolerance: f64) -> bool
```

**Parameters**:
- `tolerance`: Maximum allowed difference `|A[i,j] - A[j,i]|`

**Returns**: `true` if symmetric within tolerance, `false` otherwise

**Note**: Returns `bool`, not `Result` (never fails)

##### Algorithm

```rust
pub fn is_symmetric(tensor: &RealTensor, tolerance: f64) -> bool {
    // Non-square matrices cannot be symmetric
    if !tensor.is_square() {
        return false;
    }

    // Check upper triangle against lower triangle
    for i in 0..tensor.rows() {
        for j in (i + 1)..tensor.cols() {
            let diff = (tensor.get_matrix(i, j).unwrap() -
                        tensor.get_matrix(j, i).unwrap()).abs();
            if diff > tolerance {
                return false;  // Early exit on first asymmetric element
            }
        }
    }

    true
}
```

##### Complexity

- **Best Case**: O(1) - non-square matrix
- **Worst Case**: O(n²) - check all off-diagonal elements
- **Early Exit**: Faster if asymmetry found early

##### Tolerance Selection

**Machine Epsilon**: ~2.22 × 10^(-16) for f64

**Recommended Tolerances**:
- **Strict**: `1e-15` (near machine precision)
- **Standard**: `1e-10` (accounting for rounding errors)
- **Relaxed**: `1e-6` (for matrices from numerical algorithms)

**Trade-offs**:
- Too small: False negatives due to rounding
- Too large: False positives for asymmetric matrices

##### Code Example

```rust
use achronyme_linalg::is_symmetric;
use achronyme_types::tensor::RealTensor;

// Symmetric matrix
let a = RealTensor::matrix(3, 3, vec![
    4.0, 1.0, 2.0,
    1.0, 5.0, 3.0,
    2.0, 3.0, 6.0
]).unwrap();

assert!(is_symmetric(&a, 1e-10));

// Non-symmetric matrix
let b = RealTensor::matrix(2, 2, vec![
    1.0, 2.0,
    3.0, 4.0
]).unwrap();

assert!(!is_symmetric(&b, 1e-10));

// Nearly symmetric (within tolerance)
let c = RealTensor::matrix(2, 2, vec![
    1.0, 2.0,
    2.0 + 1e-11, 3.0  // Slightly asymmetric
]).unwrap();

assert!(is_symmetric(&c, 1e-10));    // Within tolerance
assert!(!is_symmetric(&c, 1e-12));   // Outside tolerance
```

---

#### Positive Definiteness Check (`is_positive_definite`)

##### Mathematical Definition

**Positive Definite Matrix**:
```
x^T · A · x > 0  for all non-zero x
```

**Equivalent Conditions**:
1. All eigenvalues are positive
2. All leading principal minors are positive
3. Cholesky decomposition exists

##### Function Signature

```rust
pub fn is_positive_definite(tensor: &RealTensor) -> bool
```

**Returns**: `true` if positive definite, `false` otherwise

**Note**: Implicitly requires symmetry (not checked)

##### Algorithm (Cholesky Test)

**Key Insight**: Matrix is positive definite ⟺ Cholesky decomposition succeeds

```rust
pub fn is_positive_definite(tensor: &RealTensor) -> bool {
    if !tensor.is_square() {
        return false;
    }

    let mat = tensor_to_faer_mat(tensor);

    // Attempt Cholesky decomposition
    mat.cholesky(faer::Side::Lower).is_ok()
}
```

##### Complexity

- **Time**: O(n³/3) - full Cholesky decomposition
- **Fast Failure**: If not positive definite, fails early (before completion)

##### Properties

**Positive Definite**:
- Symmetric
- All eigenvalues > 0
- All diagonal elements > 0 (necessary but not sufficient)
- Invertible (det > 0)

**Related Concepts**:
- **Positive Semi-Definite**: x^T·A·x ≥ 0 (allows zero eigenvalues)
- **Negative Definite**: x^T·A·x < 0 (all eigenvalues < 0)

##### Use Cases

1. **Covariance Matrices**: Always positive semi-definite, positive definite if full rank
2. **Hessian Matrices**: Positive definite at local minimum (optimization)
3. **Gram Matrices**: X^T·X is always positive semi-definite
4. **Cholesky Eligibility**: Test before using Cholesky decomposition

##### Code Example

```rust
use achronyme_linalg::{is_positive_definite, is_symmetric};
use achronyme_types::tensor::RealTensor;

// Positive definite matrix
let a = RealTensor::matrix(3, 3, vec![
    4.0, 1.0, 0.0,
    1.0, 3.0, 1.0,
    0.0, 1.0, 2.0
]).unwrap();

// Check symmetry first
assert!(is_symmetric(&a, 1e-10));

// Check positive definiteness
assert!(is_positive_definite(&a));
println!("Matrix is positive definite");

// Non-positive definite (negative eigenvalue)
let b = RealTensor::matrix(2, 2, vec![
    1.0,  2.0,
    2.0, -1.0
]).unwrap();

assert!(is_symmetric(&b, 1e-10));
assert!(!is_positive_definite(&b));
println!("Matrix is symmetric but not positive definite");

// Use case: Choose solver
if is_positive_definite(&a) {
    // Use Cholesky (2× faster than LU)
    let l = cholesky_decomposition(&a).unwrap();
} else {
    // Use LU decomposition
    let (l, u, p) = lu_decomposition(&a).unwrap();
}
```

##### Limitations

**Symmetry Not Checked**:
```rust
// Asymmetric matrix
let c = RealTensor::matrix(2, 2, vec![
    1.0, 2.0,
    3.0, 4.0
]).unwrap();

// May return false (or true due to numerical issues)
// Better: Check symmetry first
if !is_symmetric(&c, 1e-10) {
    println!("Matrix is not symmetric, cannot be positive definite");
} else {
    if is_positive_definite(&c) {
        println!("Positive definite");
    }
}
```

**Numerical Precision**:
- Near-singular positive definite matrices may fail Cholesky numerically
- Very small positive eigenvalues → numerical issues

---

## 🧪 Testing Strategy

### Current Tests

1. **Determinant 2×2** (known value):
   ```rust
   #[test]
   fn test_determinant_2x2() {
       let a = RealTensor::matrix(2, 2, vec![
           4.0, 7.0,
           2.0, 6.0
       ]).unwrap();
       let det = determinant_nd(&a).unwrap();
       assert_relative_eq!(det, 10.0, epsilon = 1e-10);
   }
   ```

2. **Determinant 3×3** (known value):
   ```rust
   #[test]
   fn test_determinant_3x3() {
       let a = RealTensor::matrix(3, 3, vec![
           1.0, 2.0, 3.0,
           0.0, 1.0, 4.0,
           5.0, 6.0, 0.0
       ]).unwrap();
       let det = determinant_nd(&a).unwrap();
       assert_relative_eq!(det, 1.0, epsilon = 1e-10);
   }
   ```

3. **Inverse Verification** (A·A^(-1) = I):
   ```rust
   #[test]
   fn test_inverse_2x2() {
       let a = RealTensor::matrix(2, 2, vec![
           4.0, 7.0,
           2.0, 6.0
       ]).unwrap();
       let a_inv = inverse(&a).unwrap();
       let product = a.matmul(&a_inv).unwrap();
       let identity = RealTensor::eye(2);
       // Check each element
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

4. **Solve System** (known solution):
   ```rust
   #[test]
   fn test_solve_system_2x2() {
       // 3x + y = 9, x + 2y = 8 → x=2, y=3
       let a = RealTensor::matrix(2, 2, vec![
           3.0, 1.0,
           1.0, 2.0
       ]).unwrap();
       let b = RealTensor::vector(vec![9.0, 8.0]);
       let x = solve_system(&a, &b).unwrap();
       assert_relative_eq!(x.data()[0], 2.0, epsilon = 1e-10);
       assert_relative_eq!(x.data()[1], 3.0, epsilon = 1e-10);
   }
   ```

5. **Error Handling** (dimension mismatch):
   ```rust
   #[test]
   fn test_solve_system_dimension_mismatch() {
       let a = RealTensor::matrix(2, 2, vec![...]).unwrap();
       let b = RealTensor::vector(vec![1.0, 2.0, 3.0]);  // Wrong size
       let result = solve_system(&a, &b);
       assert!(result.is_err());
   }
   ```

### Future Test Enhancements

#### Residual Verification
```rust
#[test]
fn test_solve_system_residual() {
    let a = generate_random_matrix(10);
    let b = generate_random_vector(10);

    let x = solve_system(&a, &b).unwrap();

    // Compute residual: r = b - A·x
    let ax = a.matmul(&x.to_column_matrix()).unwrap();
    let b_mat = b.to_column_matrix();
    let residual = b_mat.sub(&ax).unwrap();

    // Check ||r|| / ||b|| < tolerance
    let rel_residual = residual.norm() / b_mat.norm();
    assert!(rel_residual < 1e-10, "Large residual: {}", rel_residual);
}
```

#### Condition Number Impact
```rust
#[test]
fn test_solve_system_ill_conditioned() {
    // Hilbert matrix (notoriously ill-conditioned)
    let a = create_hilbert_matrix(5);
    let b = RealTensor::vector(vec![1.0; 5]);

    let x = solve_system(&a, &b).unwrap();

    // Solution may be inaccurate
    // Verify we get *some* solution (may not be accurate)
    assert_eq!(x.size(), 5);
}
```

#### Performance Benchmarks
```rust
#[bench]
fn bench_solve_system_100x100(bencher: &mut Bencher) {
    let a = generate_random_matrix(100);
    let b = generate_random_vector(100);

    bencher.iter(|| {
        solve_system(&a, &b).unwrap()
    });
}
```

---

## 📊 Performance Characteristics

### Computational Complexity

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| Determinant | O(n³) | O(n²) | Via LU decomposition |
| Inverse | O(n³) | O(n²) | Solve A·X = I |
| Solve Ax=b | O(n³) | O(n²) | LU + substitution |
| Is Symmetric | O(n²) | O(1) | Early exit possible |
| Is Pos Def | O(n³/3) | O(n²) | Via Cholesky |

### Optimization Opportunities

1. **Batch Solving** (multiple RHS):
   - Current: Solve each b separately
   - Optimized: Reuse LU factorization, solve all b together
   - Speedup: ~n× for n right-hand sides

2. **In-Place Operations**:
   - Current: Always allocate new matrices
   - Optimized: Overwrite input (if permitted)
   - Benefit: Reduce memory allocations

3. **Sparse Matrices** (future):
   - Current: Dense matrix operations O(n³)
   - Optimized: Sparse LU O(n^1.5) to O(n²) depending on sparsity
   - Use case: Large systems with few non-zeros

---

## 🔬 Numerical Stability Considerations

### Condition Number

**Definition**:
```
κ(A) = ||A|| · ||A^(-1)||
```

**Impact on Solvers**:
```
Relative error ≈ κ(A) · ε_machine
```

**Examples**:
- Identity: κ(I) = 1 (perfectly conditioned)
- Hilbert matrix (5×5): κ ≈ 10⁶ (ill-conditioned)
- Random matrix: κ ≈ 10-100 (well-conditioned)

### Partial Pivoting

**Purpose**: Avoid small pivot elements (numerical stability)

**Growth Factor**:
```
ρ = max(|U[i,j]|) / max(|A[i,j]|)
```

**Typical**: ρ ≈ O(n) in practice
**Worst Case**: ρ ≈ 2^(n-1) (extremely rare)

### Iterative Refinement (Future)

**Algorithm**:
```
1. Solve: A·x₀ = b
2. Loop:
   a. Compute residual: r = b - A·x_k (in higher precision)
   b. Solve: A·δx = r
   c. Update: x_{k+1} = x_k + δx
   d. If ||r|| < tolerance, break
```

**Benefit**: Recover full precision even for ill-conditioned systems

---

## 🔮 Future Enhancements

### Planned Features

1. **Batch Solving**:
   ```rust
   pub fn solve_system_multiple(
       a: &RealTensor,
       b_matrix: &RealTensor  // Multiple RHS as columns
   ) -> Result<RealTensor, String> {
       // Reuse LU factorization for all columns
   }
   ```

2. **LU Factorization Object** (reusable):
   ```rust
   pub struct LuFactorization {
       l: RealTensor,
       u: RealTensor,
       p: Vec<usize>,
   }

   impl LuFactorization {
       pub fn new(a: &RealTensor) -> Result<Self, String> { ... }

       pub fn solve(&self, b: &RealTensor) -> Result<RealTensor, String> {
           // Reuse factorization, only O(n²) substitution
       }

       pub fn determinant(&self) -> f64 { ... }
       pub fn inverse(&self) -> Result<RealTensor, String> { ... }
   }
   ```

3. **Iterative Solvers** (for large sparse systems):
   ```rust
   pub fn conjugate_gradient(
       a: &RealTensor,
       b: &RealTensor,
       max_iter: usize,
       tolerance: f64
   ) -> Result<RealTensor, String> {
       // Iterative solver for SPD matrices
   }

   pub fn gmres(
       a: &RealTensor,
       b: &RealTensor,
       restart: usize,
       max_iter: usize
   ) -> Result<RealTensor, String> {
       // Generalized Minimal Residual for general matrices
   }
   ```

4. **Condition Number Estimation**:
   ```rust
   pub fn condition_number(tensor: &RealTensor) -> Result<f64, String> {
       // Estimate κ(A) via SVD or other methods
   }
   ```

5. **Matrix Norms**:
   ```rust
   pub fn matrix_norm(tensor: &RealTensor, norm_type: NormType) -> f64 {
       // Frobenius, spectral (2-norm), 1-norm, infinity-norm
   }
   ```

### Sparse Matrix Support

```rust
pub struct SparseMatrix {
    format: SparseFormat,  // CSR, CSC, COO
    data: Vec<f64>,
    indices: Vec<usize>,
    indptr: Vec<usize>,
    shape: (usize, usize),
}

pub fn sparse_solve(
    a: &SparseMatrix,
    b: &RealTensor,
    method: SparseSolverMethod
) -> Result<RealTensor, String> {
    match method {
        SparseSolverMethod::Direct => sparse_lu_solve(a, b),
        SparseSolverMethod::IterativeCG => sparse_cg_solve(a, b),
        SparseSolverMethod::IterativeGMRES => sparse_gmres_solve(a, b),
    }
}
```

---

## 📚 References

### Textbooks

1. **Golub & Van Loan** - "Matrix Computations" (4th ed.)
   - Chapter 3: General Linear Systems (LU, pivoting strategies)
   - Chapter 4: Special Linear Systems (Cholesky, banded, sparse)
   - Section 3.5: Iterative Refinement

2. **Trefethen & Bau** - "Numerical Linear Algebra"
   - Lectures 17-18: Gaussian Elimination and Stability
   - Lectures 22-23: Least Squares Problems
   - Lectures 38-40: Iterative Methods

3. **Demmel** - "Applied Numerical Linear Algebra"
   - Chapter 2: Linear System Solving
   - Chapter 6: Iterative Methods

### Papers

- **Wilkinson (1961)**: "Error Analysis of Direct Methods of Matrix Inversion"
- **Moler (2003)**: "Numerical Computing with MATLAB" (Chapter on linear systems)

### Online Resources

- LAPACK Users' Guide: https://netlib.org/lapack/lug/node38.html
- faer documentation: https://docs.rs/faer/latest/faer/

---

## 🎓 Educational Notes

### Understanding Linear System Solving

**Why LU Decomposition?**
- **Reusability**: Factor once, solve many times
- **Stability**: Partial pivoting ensures numerical accuracy
- **Efficiency**: O(n³) factorization + O(n²) per solve

**Direct vs Iterative**:

| Aspect | Direct (LU) | Iterative (CG, GMRES) |
|--------|-------------|-----------------------|
| Complexity | O(n³) | O(kn²) where k = iterations |
| Best For | Dense, small-medium | Sparse, large |
| Accuracy | Exact (within ε) | Approximate (tolerance) |
| Memory | O(n²) | O(n) |

### Common Pitfalls

1. **Computing Inverse Unnecessarily**:
   - ❌ `x = inv(A) @ b`
   - ✅ `x = solve(A, b)`

2. **Ignoring Condition Number**:
   - Always check κ(A) for ill-conditioned systems
   - Use SVD for extreme cases

3. **Not Verifying Solution**:
   - Compute residual: `r = b - A·x`
   - Check: `||r|| / ||b|| < tolerance`

4. **Wrong Matrix Type**:
   - Use Cholesky for SPD (2× faster than LU)
   - Check symmetry first: `is_symmetric(A, 1e-10)`

---

**Module Maintainer**: Achronyme Project Team
**Last Updated**: 2024-11-14
