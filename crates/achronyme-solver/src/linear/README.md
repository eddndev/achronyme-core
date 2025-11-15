# Linear Programming Module

Comprehensive implementation of linear programming algorithms for the Achronyme solver.

## Overview

The `linear` module provides multiple algorithms for solving Linear Programming (LP) problems, each optimized for different problem characteristics:

```
linear/
├── mod.rs              # Module exports and public API
├── tableau.rs          # Simplex tableau data structure (foundation)
├── simplex.rs          # Primal Simplex algorithm
├── dual_simplex.rs     # Dual Simplex algorithm
├── two_phase.rs        # Two-Phase Simplex for difficult problems
├── revised_simplex.rs  # Memory-efficient Revised Simplex
├── sensitivity.rs      # Post-optimality sensitivity analysis
└── linprog.rs          # Auto-selection wrapper
```

## Linear Programming Standard Form

All algorithms solve problems in standard form:

```
maximize/minimize  z = c^T x
subject to:        A x ≤ b
                   x ≥ 0
```

Where:
- **x** ∈ ℝ^n: decision variables (unknowns to find)
- **c** ∈ ℝ^n: objective coefficients (costs or profits)
- **A** ∈ ℝ^(m×n): constraint coefficients
- **b** ∈ ℝ^m: right-hand side (resource limits)
- **z**: objective value to optimize

## Module Selection Guide

| Problem Type | Recommended Algorithm | Reason |
|--------------|---------------------|---------|
| General LP | `simplex.rs` | Most robust, well-tested |
| Adding constraints to solved problem | `dual_simplex.rs` | Starts from dual feasible |
| Negative RHS or equality constraints | `two_phase.rs` | Handles non-standard forms |
| Many variables, few constraints (n >> m) | `revised_simplex.rs` | O(m²) space vs O(mn) |
| Any LP (auto-select) | `linprog.rs` | Chooses best method |
| Post-optimality analysis | `sensitivity.rs` | Shadow prices, ranging |

---

## Core Data Structure: Tableau (`tableau.rs`)

### Purpose

The `Tableau` is the fundamental data structure for all simplex-based algorithms. It represents the LP problem in **augmented matrix form** with slack variables.

### Structure

```rust
pub struct Tableau {
    /// Augmented matrix data: (m+1) rows × (n+m+1) columns
    /// Last row = objective function
    /// Last column = RHS (right-hand side)
    pub data: Vec<Vec<f64>>,

    /// Number of original decision variables
    pub num_vars: usize,

    /// Number of constraints (= number of slack variables)
    pub num_constraints: usize,

    /// Current basic variables (one per constraint)
    /// basis[i] = column index of basic variable in row i
    pub basis: Vec<usize>,
}
```

### Tableau Layout

For problem with n=2 variables and m=3 constraints:

```
Columns:  x₁   x₂  | s₁  s₂  s₃ | RHS
         ───────────┼────────────┼─────
Row 0:    a₁₁  a₁₂ | 1   0   0  | b₁     ← Constraint 1
Row 1:    a₂₁  a₂₂ | 0   1   0  | b₂     ← Constraint 2
Row 2:    a₃₁  a₃₂ | 0   0   1  | b₃     ← Constraint 3
         ───────────┼────────────┼─────
Row 3:   -c₁  -c₂  | 0   0   0  | z₀     ← Objective (maximize)
```

**Key properties:**
- Slack variables form identity matrix initially
- Objective row coefficients are negated for maximization
- RHS column contains constraint limits and current objective value

### Example

Problem:
```
maximize z = 3x₁ + 5x₂
subject to:
  x₁ ≤ 4
  2x₂ ≤ 12
  3x₁ + 2x₂ ≤ 18
  x₁, x₂ ≥ 0
```

Initial tableau:
```
     x₁  x₂  s₁  s₂  s₃ | RHS
s₁ [  1   0   1   0   0 |  4  ]
s₂ [  0   2   0   1   0 | 12  ]
s₃ [  3   2   0   0   1 | 18  ]
    ────────────────────┼─────
z  [ -3  -5   0   0   0 |  0  ]
```

### Operations

#### 1. Construction (`new`)

```rust
pub fn new(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Self, String>
```

**Steps:**
1. Validate dimensions (A is m×n, b is m, c is n)
2. Check b ≥ 0 (required for initial feasibility)
3. Allocate (m+1) × (n+m+1) matrix
4. Copy A into left part
5. Add identity matrix for slack variables
6. Copy b into RHS column
7. Set objective row: -sense × c for decision vars, 0 for slacks
8. Initialize basis with slack variables [n, n+1, ..., n+m-1]

**Complexity:** O(mn)

#### 2. Optimality Check (`is_optimal`)

```rust
pub fn is_optimal(&self) -> bool
```

**Rule:**
- For maximization: all objective coefficients ≥ 0
- For minimization: all objective coefficients ≤ 0

**Rationale:** Negative coefficient → can increase that variable to improve objective

**Complexity:** O(n+m)

#### 3. Entering Variable Selection (`find_entering_variable`)

```rust
pub fn find_entering_variable(&self) -> Option<usize>
```

**Rule:** Select column with most negative coefficient in objective row

**Why most negative?** Steepest ascent (maximize improvement per unit increase)

**Returns:**
- `Some(col)`: Column index to enter basis
- `None`: Already optimal

**Complexity:** O(n+m)

#### 4. Leaving Variable Selection (`find_leaving_variable`)

```rust
pub fn find_leaving_variable(&self, entering: usize) -> Result<usize, String>
```

**Minimum Ratio Test:**
```
For each row i:
  if a[i][entering] > 0:
    ratio[i] = RHS[i] / a[i][entering]

Select row with minimum ratio
```

**Why?** Determines how far we can increase entering variable before basic variable hits zero

**Edge cases:**
- All a[i][entering] ≤ 0 → Problem is unbounded
- Multiple rows with same ratio → Degeneracy (tie-breaking needed)

**Complexity:** O(m)

#### 5. Pivot Operation (`pivot`)

```rust
pub fn pivot(&mut self, entering: usize, leaving: usize)
```

**Goal:** Transform tableau so entering variable is basic and leaving variable is non-basic

**Steps:**
1. **Normalize pivot row:** Divide row by pivot element
   ```rust
   let pivot = data[leaving][entering];
   for j in 0..total_cols {
       data[leaving][j] /= pivot;
   }
   ```

2. **Eliminate pivot column in other rows:** Make pivot column = unit vector
   ```rust
   for i in 0..=m {
       if i != leaving {
           let factor = data[i][entering];
           for j in 0..total_cols {
               data[i][j] -= factor * data[leaving][j];
           }
       }
   }
   ```

3. **Update basis:**
   ```rust
   basis[leaving] = entering;
   ```

**Result:** Pivot column now has 1 in pivot row, 0 elsewhere

**Complexity:** O(m × n)

**Example pivot:**
Before (entering=1, leaving=1):
```
     x₁  x₂  s₁  s₂  s₃ | RHS
s₁ [  1   0   1   0   0 |  4  ]
s₂ [  0   2   0   1   0 | 12  ] ← leaving row
s₃ [  3   2   0   0   1 | 18  ]
    ────────────────────┼─────
z  [ -3  -5   0   0   0 |  0  ]
         ↑
      entering column
```

After pivot:
```
     x₁  x₂  s₁  s₂  s₃ | RHS
s₁ [  1   0   1   0   0 |  4  ]
x₂ [  0   1   0  0.5  0 |  6  ] ← x₂ enters basis
s₃ [  3   0   0  -1   1 |  6  ]
    ────────────────────┼─────
z  [ -3   0   0  2.5  0 | 30  ]
         ↑
      now zero
```

#### 6. Solution Extraction (`extract_solution`)

```rust
pub fn extract_solution(&self) -> Vec<f64>
```

**Rule:**
- Basic variables: Read value from RHS of their row
- Non-basic variables: Set to 0

**Implementation:**
```rust
let mut solution = vec![0.0; n];
for (i, &basic_var) in self.basis.iter().enumerate() {
    if basic_var < n {  // Not a slack variable
        solution[basic_var] = data[i][RHS_col];
    }
}
```

**Complexity:** O(n + m)

#### 7. Objective Value (`objective_value`)

```rust
pub fn objective_value(&self) -> f64
```

**Simply read RHS of objective row:**
```rust
data[m][n+m]  // Last row, last column
```

**Complexity:** O(1)

---

## Algorithm 1: Primal Simplex (`simplex.rs`)

### Overview

The classic simplex algorithm that iteratively moves along edges of the feasible polytope toward the optimal vertex.

### Algorithm

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
```

**Pseudocode:**
```
1. Create initial tableau
2. WHILE not optimal:
     a. Find entering variable (most negative cost)
     b. Find leaving variable (minimum ratio)
     c. Pivot
     d. Check iteration limit
3. Return solution
```

### Detailed Steps

**Initialization:**
```rust
let mut tableau = Tableau::new(c, a, b, sense)?;
```

**Main loop:**
```rust
for iteration in 0..max_iterations {
    // Optimality check
    if tableau.is_optimal() {
        return Ok(tableau.extract_solution());
    }

    // Pivot selection
    let entering = tableau.find_entering_variable()
        .ok_or("Already optimal")?;

    let leaving = tableau.find_leaving_variable(entering)?;

    // Pivot
    tableau.pivot(entering, leaving);
}
```

### Complexity

- **Iterations:** O(m × n) typical, O(2^n) worst-case
- **Per iteration:** O(m × n) for pivot
- **Total:** O(m^2 × n^2) typical, exponential worst-case

### Termination Conditions

1. **Optimal:** All objective coefficients ≥ 0 (max) or ≤ 0 (min)
2. **Unbounded:** No valid leaving variable (all pivot candidates ≤ 0)
3. **Max iterations:** Exceeded 10,000 iterations (safety limit)

### Example

```rust
// Production planning: maximize profit
// z = 40x₁ + 30x₂ (profit per unit)
// x₁ ≤ 40    (material A limit)
// x₂ ≤ 50    (material B limit)
// x₁ + x₂ ≤ 70  (labor hours limit)

let c = vec![40.0, 30.0];
let a = RealTensor::matrix(3, 2, vec![
    1.0, 0.0,   // x₁ ≤ 40
    0.0, 1.0,   // x₂ ≤ 50
    1.0, 1.0,   // x₁ + x₂ ≤ 70
]).unwrap();
let b = vec![40.0, 50.0, 70.0];

let solution = simplex::solve(&c, &a, &b, 1.0).unwrap();
// solution = [40.0, 30.0]
// profit = 40×40 + 30×30 = 2500
```

### Geometric Interpretation

The simplex method:
1. Starts at a vertex of the feasible polytope (origin with slacks)
2. Moves along an edge to an adjacent vertex (pivot)
3. Chooses edge that increases objective most (steepest ascent)
4. Stops when no improving edge exists (local = global optimum for LP)

---

## Algorithm 2: Dual Simplex (`dual_simplex.rs`)

### Overview

Maintains dual feasibility while seeking primal feasibility. Useful when starting solution is optimal for dual but not feasible for primal.

### When to Use

1. **Adding constraints:** After solving LP, add new constraint → solution may become infeasible
2. **Negative RHS:** Some b[i] < 0 (primal infeasible but dual may be feasible)
3. **Reoptimization:** Warm-start from previous solution

### Primal vs Dual Simplex

| Aspect | Primal Simplex | Dual Simplex |
|--------|---------------|--------------|
| Maintains | Primal feasibility (b ≥ 0) | Dual feasibility (costs ≥ 0) |
| Seeks | Optimality | Primal feasibility |
| Leaving | Minimum ratio (RHS/pivot) | Most negative RHS |
| Entering | Most negative cost | Dual ratio test |

### Algorithm

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
```

**Pseudocode:**
```
1. Create tableau (may have negative RHS)
2. WHILE not (primal feasible AND optimal):
     a. Find leaving variable (most negative RHS)
     b. Find entering variable (dual ratio test)
     c. Pivot
3. Return solution
```

### Key Differences

**Leaving variable selection:**
```rust
fn find_leaving_variable_dual(tableau: &Tableau) -> Option<usize> {
    // Find row with most negative RHS
    let mut min_rhs = 0.0;
    let mut leaving_row = None;

    for i in 0..m {
        let rhs = tableau.data[i][RHS];
        if rhs < min_rhs {
            min_rhs = rhs;
            leaving_row = Some(i);
        }
    }
    leaving_row
}
```

**Dual ratio test:**
```rust
fn find_entering_variable_dual(tableau: &Tableau, leaving: usize)
    -> Result<usize, String>
{
    // For each column j with a[leaving][j] < 0:
    //   ratio = |c[j]| / |a[leaving][j]|
    // Choose j with minimum ratio

    let mut min_ratio = f64::INFINITY;
    let mut entering = None;

    for j in 0..total_cols {
        let pivot_candidate = tableau.data[leaving][j];

        if pivot_candidate < -ε {
            let cost = tableau.data[m][j];
            let ratio = cost.abs() / pivot_candidate.abs();

            if ratio < min_ratio {
                min_ratio = ratio;
                entering = Some(j);
            }
        }
    }

    entering.ok_or("Dual infeasible")
}
```

### Complexity

Same as primal simplex: O(m^2 × n^2) typical

### Example Use Case

**Adding constraint to solved problem:**
```rust
// Original problem solved: x* = [10, 20]
// Add new constraint: x₁ + x₂ ≤ 25

// Original solution violates new constraint (10+20 > 25)
// Dual simplex can reoptimize efficiently from x*
```

---

## Algorithm 3: Two-Phase Simplex (`two_phase.rs`)

### Overview

Handles problems where initial basic feasible solution (BFS) is not obvious, particularly:
- Negative RHS (b[i] < 0)
- Equality constraints
- Greater-than constraints (≥)

### The Problem

Standard simplex requires:
1. Initial BFS (all b[i] ≥ 0)
2. Slack variables form identity matrix

What if b[i] < 0? No obvious initial basis!

### Solution: Two Phases

#### Phase 1: Find Initial BFS

**Auxiliary Problem:**
```
minimize w = Σ a[i]  (sum of artificial variables)
subject to:
  A x + a = b  (add artificial variables)
  x, a ≥ 0
```

**If min w = 0:** Found BFS for original problem (artificials can be removed)
**If min w > 0:** Original problem is infeasible

#### Phase 2: Solve Original Problem

Use BFS from Phase 1 as starting point for original objective.

### Algorithm

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
```

**Pseudocode:**
```
1. Check if Phase 1 needed:
     IF all b[i] ≥ 0:
       Use standard simplex
     ELSE:
       Proceed with two phases

2. PHASE 1:
     a. Build auxiliary problem with artificials
     b. Solve: minimize sum of artificials
     c. If objective > 0 → INFEASIBLE
     d. Extract basis (remove artificials)

3. PHASE 2:
     a. Build tableau with original objective
     b. Use basis from Phase 1
     c. Solve with simplex
     d. Return solution
```

### Phase 1 Tableau Construction

**Example:** Constraint with negative RHS
```
Original: 2x₁ + x₂ ≤ -5  (impossible with x ≥ 0)
Flip:    -2x₁ - x₂ ≤ 5   (multiply by -1)
Add artificial: -2x₁ - x₂ + a₁ = 5
```

**Implementation:**
```rust
fn build_phase1_tableau(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Tableau, String>
{
    let num_artificials = b.iter().filter(|&&bi| bi < 0).count();

    // Extended tableau: [x | slack | artificial | RHS]
    let mut data = vec![vec![0.0; n+m+num_artificials+1]; m+1];

    let mut artificial_idx = 0;
    for i in 0..m {
        if b[i] < 0.0 {
            // Flip constraint
            for j in 0..n {
                data[i][j] = -a.get(i, j);
            }
            data[i][n+i] = -1.0;  // Flip slack
            data[i][n+m+artificial_idx] = 1.0;  // Add artificial
            data[i][RHS] = -b[i];  // Now positive
            artificial_idx += 1;
        } else {
            // Normal constraint
            for j in 0..n {
                data[i][j] = a.get(i, j);
            }
            data[i][n+i] = 1.0;  // Slack
            data[i][RHS] = b[i];
        }
    }

    // Phase 1 objective: minimize Σ artificials
    for j in (n+m)..(n+m+num_artificials) {
        data[m][j] = 1.0;
    }

    // CRITICAL: Make objective compatible with basis
    // Subtract artificial rows from objective
    artificial_idx = 0;
    for i in 0..m {
        if b[i] < 0.0 {
            for j in 0..total_cols {
                data[m][j] -= data[i][j];
            }
            artificial_idx += 1;
        }
    }

    Ok(Tableau { data, ... })
}
```

**Why subtract artificial rows?**

Initial basis includes artificial variables. To maintain canonical form (basic variables have 0 in objective), we must eliminate them from objective row.

### Phase 2 Tableau Construction

```rust
fn build_phase2_tableau(phase1: &Tableau, c: &[f64], sense: f64, n: usize, m: usize)
    -> Result<Tableau, String>
{
    // Copy constraint rows (drop artificial columns)
    let mut data = vec![vec![0.0; n+m+1]; m+1];

    for i in 0..m {
        for j in 0..(n+m) {
            data[i][j] = phase1.data[i][j];
        }
        data[i][n+m] = phase1.data[i][RHS];
    }

    // Original objective
    for j in 0..n {
        data[m][j] = -sense * c[j];
    }

    // Copy basis (check no artificials)
    let basis = phase1.basis.clone();
    for &b in &basis {
        if b >= n+m {
            return Err("Artificial in basis - infeasible");
        }
    }

    Ok(Tableau { data, num_vars: n, num_constraints: m, basis })
}
```

### Complexity

- **Phase 1:** O(m^2 × (n+k)^2) where k = artificials
- **Phase 2:** O(m^2 × n^2)
- **Total:** Approximately 2× standard simplex

### Example

```rust
// Problem with negative RHS
// maximize z = x₁ + x₂
// subject to:
//   -x₁ - x₂ ≤ -2   (i.e., x₁ + x₂ ≥ 2)
//    x₁ + x₂ ≤  5

let c = vec![1.0, 1.0];
let a = RealTensor::matrix(2, 2, vec![
    -1.0, -1.0,
     1.0,  1.0,
]).unwrap();
let b = vec![-2.0, 5.0];

let solution = two_phase::solve(&c, &a, &b, 1.0).unwrap();
// Phase 1 finds BFS, Phase 2 optimizes
// solution ∈ {x : x₁+x₂ ≥ 2, x₁+x₂ ≤ 5}
```

---

## Algorithm 4: Revised Simplex (`revised_simplex.rs`)

### Overview

Memory-efficient variant that stores only basis inverse B^(-1) instead of full tableau.

### Motivation

**Standard simplex:** Stores (m+1) × (n+m+1) tableau = O(m × n) space

**Revised simplex:** Stores only m × m basis inverse = O(m^2) space

**When beneficial:** n >> m (many variables, few constraints)

### Key Idea

Don't store full tableau. Instead:
1. Store original matrix A
2. Store basis inverse B^(-1)
3. Recompute tableau columns on-demand

### Algorithm

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
```

**Pseudocode:**
```
1. Initialize: basis B = I (slack variables)
2. WHILE not optimal:
     a. Compute simplex multipliers: π = c_B^T B^(-1)
     b. For each non-basic variable j:
          Compute reduced cost: r_j = c_j - π^T A_j
     c. Select entering (most negative r_j)
     d. Compute direction: d = B^(-1) A_entering
     e. Minimum ratio test: θ = min{x_B[i]/d[i] : d[i] > 0}
     f. Update: x_B ← x_B - θd, basis ← entering
     g. Recompute B^(-1)
3. Return solution
```

### Data Structures

```rust
let aug_a: Vec<f64>;        // [A | I] augmented matrix
let c_aug: Vec<f64>;        // [c | 0] augmented costs
let basis: Vec<usize>;      // Basis column indices
let x_b: Vec<f64>;          // Basic variable values
```

### Key Operations

**1. Basis Inverse Computation:**
```rust
fn compute_basis_inverse(aug_a: &[f64], basis: &[usize], m: usize, n: usize)
    -> Result<Vec<f64>, String>
{
    // Extract basis columns → matrix B (m×m)
    let mut B = vec![0.0; m*m];
    for (i, &col) in basis.iter().enumerate() {
        for row in 0..m {
            B[row*m + i] = aug_a[row*n + col];
        }
    }

    // Invert using Gauss-Jordan
    invert_matrix(&B, m)
}
```

**2. Matrix Inversion (Gauss-Jordan):**
```rust
fn invert_matrix(mat: &[f64], n: usize) -> Result<Vec<f64>, String> {
    // Create augmented [A | I]
    let mut aug = vec![0.0; n * 2*n];
    for i in 0..n {
        for j in 0..n { aug[i*2*n + j] = mat[i*n + j]; }
        aug[i*2*n + (n+i)] = 1.0;  // Identity
    }

    // Gaussian elimination with partial pivoting
    for k in 0..n {
        // Find and swap pivot row
        let (pivot_row, pivot_val) = find_max_row(&aug, k, n);
        if pivot_val < ε { return Err("Singular matrix"); }
        swap_rows(&mut aug, k, pivot_row);

        // Normalize pivot row
        for j in 0..2*n {
            aug[k*2*n + j] /= aug[k*2*n + k];
        }

        // Eliminate column k
        for i in 0..n {
            if i != k {
                let factor = aug[i*2*n + k];
                for j in 0..2*n {
                    aug[i*2*n + j] -= factor * aug[k*2*n + j];
                }
            }
        }
    }

    // Extract inverse (right half)
    let mut inv = vec![0.0; n*n];
    for i in 0..n {
        for j in 0..n {
            inv[i*n + j] = aug[i*2*n + (n+j)];
        }
    }
    Ok(inv)
}
```

**3. Reduced Cost Calculation:**
```rust
// Simplex multipliers
let c_b: Vec<f64> = basis.iter().map(|&j| c_aug[j]).collect();
let pi = multiply_vector_matrix(&c_b, &b_inv, m);

// For each non-basic variable j
for j in 0..total_vars {
    if basis.contains(&j) { continue; }

    let a_j = get_column(&aug_a, j, m, total_vars);
    let pi_a_j = dot_product(&pi, &a_j);
    let r_j = c_aug[j] - pi_a_j;  // Reduced cost

    if r_j < min_cost { /* select as entering */ }
}
```

**4. Direction Computation:**
```rust
// Direction: d = B^(-1) A_entering
let a_entering = get_column(&aug_a, entering, m, total_vars);
let d = multiply_matrix_vector(&b_inv, &a_entering, m);
```

**5. Basis Update:**
```rust
// Minimum ratio test
let θ = min_ratio(&x_b, &d);

// Update basic variables
for i in 0..m {
    x_b[i] -= θ * d[i];
}
x_b[leaving_row] = θ;

// Update basis
basis[leaving_row] = entering;
```

### Complexity

**Per iteration:**
- Basis inversion: O(m^3)
- Reduced costs: O(n × m)
- Total: O(m^3 + n × m)

**Comparison:**
- Standard simplex: O(m × n) per iteration
- Revised simplex: O(m^3 + m × n) per iteration

**When revised is better:**
- If n >> m, then m^3 + mn ≈ mn << mn (for large n)
- Space: m^2 << mn

### Optimization: Product Form

Instead of full re-inversion, update B^(-1) incrementally:

```
B_new^(-1) = E × B_old^(-1)

where E is elementary matrix (differs from I in one column)
```

Store sequence of E matrices (product form). This reduces per-iteration cost to O(m^2).

### Example

```rust
// Large sparse problem: 1000 variables, 50 constraints
let c = vec![/* 1000 elements */];
let a = /* 50×1000 sparse matrix */;
let b = vec![/* 50 elements */];

// Revised simplex uses 50×50 = 2500 floats for B^(-1)
// Standard simplex uses 50×1000 = 50,000 floats for tableau
let solution = revised_simplex::solve(&c, &a, &b, 1.0).unwrap();
```

---

## Algorithm 5: Sensitivity Analysis (`sensitivity.rs`)

### Overview

Post-optimality analysis to understand how changes in parameters affect the optimal solution.

### Use Cases

1. **What-if analysis:** "What if material A increases to 50 units?"
2. **Pricing decisions:** "How much should we pay for extra labor?"
3. **Robustness:** "How sensitive is the solution to data errors?"

---

### 1. Shadow Prices

**Definition:** Marginal value of resources

```
shadow_price[i] = ∂z*/∂b[i]
```

**Interpretation:**
- shadow[i] = $15 → one more unit of resource i increases profit by $15
- shadow[i] = 0 → resource i is not binding (surplus exists)

**Mathematical Background:**

From duality theory:
```
Primal: max c^T x  s.t. Ax ≤ b, x ≥ 0
Dual:   min b^T y  s.t. A^T y ≥ c, y ≥ 0

At optimum: c^T x* = b^T y*
Shadow prices = y* (optimal dual variables)
```

**Implementation:**

```rust
pub fn shadow_price(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    // Solve to optimality
    let tableau = solve_to_optimality(c, a, b, sense)?;

    // Shadow prices are in slack columns of objective row
    let mut shadow = vec![0.0; m];
    for i in 0..m {
        let slack_col = n + i;
        shadow[i] = tableau.data[m][slack_col];
    }

    Ok(shadow)
}
```

**Why slack columns?**

In optimal tableau:
- Slack variable s_i corresponds to constraint i
- Its coefficient in objective row = dual variable y_i
- y_i = shadow price for constraint i

**Example:**

```rust
// Production problem
let c = vec![40.0, 30.0];  // Profit per unit
let a = RealTensor::matrix(3, 2, vec![
    1.0, 0.0,   // Material A: x₁ ≤ 40
    0.0, 1.0,   // Material B: x₂ ≤ 50
    1.0, 1.0,   // Labor: x₁+x₂ ≤ 70
]).unwrap();
let b = vec![40.0, 50.0, 70.0];

let shadow = shadow_price(&c, &a, &b, 1.0).unwrap();
// shadow[0] = 40  (Material A worth $40/unit)
// shadow[1] = 30  (Material B worth $30/unit)
// shadow[2] = 0   (Labor not binding, x₁+x₂ < 70)

// If we get 1 more unit of Material A (b[0] = 41):
// New profit ≈ 2500 + 40 = 2540
```

---

### 2. Objective Coefficient Sensitivity (c[i])

**Question:** How much can c[i] vary without changing the optimal solution?

**Answer:** Range [c_min, c_max] where basis remains optimal

**Interpretation:**
- Within range: solution x* unchanged, but z* changes proportionally
- Outside range: solution x* may change

**For Non-Basic Variables:**

If x[i] = 0 in optimal solution:

```rust
let reduced_cost = tableau.data[m][i];
let c_max = c[i] + reduced_cost;
return [f64::NEG_INFINITY, c_max];
```

**Rationale:** If c[i] increases beyond c_max, reduced cost becomes negative → x[i] enters basis

**For Basic Variables:**

More complex: must check dual feasibility

```rust
// Full implementation requires:
// 1. Compute optimal basis B
// 2. For each non-basic j: recompute reduced cost with new c[i]
// 3. Find range where all reduced costs ≥ 0

// Simplified (conservative estimate):
return [c[i] * 0.5, c[i] * 2.0];
```

**Implementation:**

```rust
pub fn sensitivity_c(c: &[f64], a: &RealTensor, b: &[f64], index: usize)
    -> Result<Vec<f64>, String>
{
    let tableau = solve_to_optimality(c, a, b, 1.0)?;

    if !tableau.basis.contains(&index) {
        // Non-basic: range is (-∞, c[i] + reduced_cost]
        let reduced_cost = tableau.data[m][index];
        return Ok(vec![f64::NEG_INFINITY, c[index] + reduced_cost]);
    } else {
        // Basic: conservative range
        return Ok(vec![c[index] * 0.5, c[index] * 2.0]);
    }
}
```

---

### 3. RHS Sensitivity (b[i])

**Question:** How much can b[i] vary without changing the basis?

**Answer:** Range [b_min, b_max] where all basic variables remain non-negative

**Importance:** Shadow prices remain valid within this range

**Mathematical Background:**

Basic variables: x_B = B^(-1) b

If b changes to b + Δb:
```
x_B_new = B^(-1) (b + Δb) = x_B + B^(-1) Δb
```

For basis to remain optimal: x_B_new ≥ 0

**Full Implementation:**

```rust
pub fn sensitivity_b_full(c: &[f64], a: &RealTensor, b: &[f64], index: usize)
    -> Result<Vec<f64>, String>
{
    let tableau = solve_to_optimality(c, a, b, 1.0)?;

    // Extract B^(-1) from tableau
    let b_inv = extract_basis_inverse(&tableau);

    // Column i of B^(-1) shows how basic vars change with b[i]
    let sensitivity_column = get_column(&b_inv, index);

    // For each basic variable j:
    //   x_B[j] + Δb[i] * sensitivity[j] ≥ 0
    //   Δb[i] ≥ -x_B[j] / sensitivity[j]  (if sensitivity[j] > 0)
    //   Δb[i] ≤ -x_B[j] / sensitivity[j]  (if sensitivity[j] < 0)

    let mut lower_bound = f64::NEG_INFINITY;
    let mut upper_bound = f64::INFINITY;

    for j in 0..m {
        let x_b_j = tableau.data[j][RHS];
        let sens_j = sensitivity_column[j];

        if sens_j > ε {
            // Upper bound constraint
            let bound = -x_b_j / sens_j;
            upper_bound = upper_bound.min(bound);
        } else if sens_j < -ε {
            // Lower bound constraint
            let bound = -x_b_j / sens_j;
            lower_bound = lower_bound.max(bound);
        }
    }

    Ok(vec![b[index] + lower_bound, b[index] + upper_bound])
}
```

**Current Implementation (Conservative):**

```rust
pub fn sensitivity_b(c: &[f64], a: &RealTensor, b: &[f64], index: usize)
    -> Result<Vec<f64>, String>
{
    // Conservative estimate: ±50% of current value
    let b_min = b[index] - b[index].abs() * 0.5;
    let b_max = b[index] + b[index].abs() * 0.5;
    Ok(vec![b_min.max(0.0), b_max])
}
```

---

## Auto-Selection Wrapper (`linprog.rs`)

### Purpose

Automatically select best algorithm based on problem characteristics.

### Current Implementation

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    // Currently: always use primal simplex
    simplex::solve(c, a, b, sense)
}
```

### Future Heuristics

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    let n = c.len();
    let m = a.rows();

    // Very large problems
    if n > 5000 || m > 5000 {
        return interior_point::solve(c, a, b, sense);
    }

    // Many variables, few constraints
    if m < n / 2 {
        return dual_simplex::solve(c, a, b, sense);
    }

    // Sparse matrix
    if is_sparse(a) {
        return revised_simplex::solve(c, a, b, sense);
    }

    // Negative RHS or equality constraints
    if has_negative_rhs(b) || has_equality_constraints() {
        return two_phase::solve(c, a, b, sense);
    }

    // Default: primal simplex
    simplex::solve(c, a, b, sense)
}
```

### Sparsity Detection

```rust
fn is_sparse(tensor: &RealTensor) -> bool {
    let total = tensor.size();
    let zeros = tensor.data()
        .iter()
        .filter(|&&x| x.abs() < 1e-10)
        .count();

    zeros > total / 2  // >50% zeros
}
```

---

## Numerical Considerations

### Tolerances

```rust
const ZERO_TOLERANCE: f64 = 1e-10;   // Optimality check
const PIVOT_TOLERANCE: f64 = 1e-8;   // Minimum pivot element
const INTEGER_TOLERANCE: f64 = 1e-6; // Integer feasibility (ILP)
```

### Pivot Selection

**Numerical stability:** Choose pivot with largest absolute value in column (partial pivoting)

```rust
fn find_pivot_with_partial_pivoting(col: &[f64], start_row: usize) -> (usize, f64) {
    let (max_row, max_val) = col[start_row..]
        .iter()
        .enumerate()
        .map(|(i, &val)| (i + start_row, val.abs()))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    (max_row, max_val)
}
```

### Degeneracy

**Problem:** Multiple basic variables = 0 → infinite loop (cycling)

**Detection:**
```rust
fn is_degenerate(tableau: &Tableau) -> bool {
    tableau.basis.iter()
        .filter(|&&i| tableau.data[i][RHS].abs() < ZERO_TOLERANCE)
        .count() > 0
}
```

**Prevention (not implemented):**
- Bland's rule: lexicographic pivot selection
- Perturbation: add small random ε to b
- Lexicographic ordering: tie-breaking by constraint index

### Ill-conditioning

**Problem:** Large condition number → numerical errors accumulate

**Detection:**
```rust
// Condition number ≈ ||B|| × ||B^(-1)||
fn condition_number(b: &[f64], b_inv: &[f64], n: usize) -> f64 {
    let norm_b = matrix_norm(b, n);
    let norm_b_inv = matrix_norm(b_inv, n);
    norm_b * norm_b_inv
}
```

**Mitigation:**
- Scaling: normalize rows/columns to similar magnitude
- Preconditioning: transform to better-conditioned problem
- Iterative refinement: improve solution accuracy

---

## Testing Strategy

### Unit Tests

Each algorithm has comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_2d_maximize() {
        // Small problem verifiable by hand
    }

    #[test]
    fn test_unbounded_problem() {
        // Should detect and report unbounded
    }

    #[test]
    fn test_infeasible_problem() {
        // Should detect infeasibility
    }

    #[test]
    fn test_degenerate_problem() {
        // Multiple optimal bases
    }
}
```

### Benchmark Problems

**Classic LP problems:**
1. **Diet problem:** Minimize cost subject to nutrition constraints
2. **Transportation:** Min cost shipping from suppliers to customers
3. **Production planning:** Maximize profit with resource limits
4. **Network flow:** Max flow from source to sink

### Validation

Compare against known optimal solutions:

```rust
fn assert_optimal(solution: &[f64], expected: &[f64], tolerance: f64) {
    for i in 0..solution.len() {
        assert!((solution[i] - expected[i]).abs() < tolerance,
                "Variable {}: expected {}, got {}",
                i, expected[i], solution[i]);
    }
}
```

---

## Performance Tips

### 1. Problem Reformulation

**Reduce dimensions:** Eliminate redundant constraints/variables

```rust
// Before: 100 variables, 80 constraints
// After presolve: 60 variables, 50 constraints
```

### 2. Scaling

**Normalize data:** Avoid mixing large and small numbers

```rust
fn scale_problem(c: &mut [f64], a: &mut RealTensor, b: &mut [f64]) {
    // Row scaling: divide each row by its max element
    for i in 0..m {
        let max_row = max_abs_in_row(a, i);
        if max_row > 1e-10 {
            scale_row(a, i, 1.0 / max_row);
            b[i] /= max_row;
        }
    }

    // Column scaling: divide each column by its max element
    for j in 0..n {
        let max_col = max_abs_in_col(a, j);
        if max_col > 1e-10 {
            scale_col(a, j, 1.0 / max_col);
            c[j] /= max_col;
        }
    }
}
```

### 3. Sparsity Exploitation

**Use sparse data structures for large sparse problems:**

```rust
// Dense: 1000×10000 = 10M floats (80 MB)
// Sparse (1% non-zero): ~100K entries (1.6 MB)

struct SparseMatrix {
    rows: usize,
    cols: usize,
    values: Vec<f64>,
    col_indices: Vec<usize>,
    row_ptrs: Vec<usize>,  // CSR format
}
```

---

## References

### Textbooks
- **Chvátal** (1983). *Linear Programming*. W.H. Freeman.
- **Bertsimas & Tsitsiklis** (1997). *Introduction to Linear Optimization*. Athena Scientific.
- **Dantzig** (1963). *Linear Programming and Extensions*. Princeton University Press.

### Papers
- **Dantzig** (1947). "Simplex method for linear programming." (Original paper)
- **Bland** (1977). "New finite pivoting rules for the simplex method." *Math. Oper. Res.*
- **Karmarkar** (1984). "A new polynomial-time algorithm for linear programming." (Interior point)

### Software
- **GLPK**: GNU Linear Programming Kit (open source)
- **CLP**: COIN-OR LP solver (open source)
- **CPLEX, Gurobi**: Commercial state-of-the-art solvers
