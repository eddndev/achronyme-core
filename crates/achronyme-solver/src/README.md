# achronyme-solver Implementation Guide

Technical documentation for the internal implementation of optimization algorithms in `achronyme-solver`.

## Architecture Overview

The solver is organized into two main modules:

```
src/
├── lib.rs                    # Public API and re-exports
├── linear/                   # Linear Programming algorithms
│   ├── mod.rs               # Module exports
│   ├── tableau.rs           # Core tableau data structure
│   ├── simplex.rs           # Primal Simplex algorithm
│   ├── dual_simplex.rs      # Dual Simplex algorithm
│   ├── two_phase.rs         # Two-Phase Simplex
│   ├── revised_simplex.rs   # Revised Simplex (memory-efficient)
│   ├── sensitivity.rs       # Post-optimality analysis
│   └── linprog.rs           # Auto-selection wrapper
└── integer/                  # Integer Programming algorithms
    ├── mod.rs               # Module exports
    └── branch_bound/        # Branch & Bound (modular refactoring)
        ├── mod.rs           # Submodule organization
        ├── node.rs          # BBNode structure
        ├── helpers.rs       # Utility functions
        ├── bounded_lp.rs    # LP with variable bounds
        └── solvers.rs       # intlinprog, binary_linprog
```

## Design Principles

### 1. Separation of Concerns

**Tableau Operations (tableau.rs):**
- Pure data structure representing simplex tableau
- No algorithm logic, only data manipulation
- Reusable across all simplex variants

**Algorithm Logic (simplex.rs, dual_simplex.rs, etc.):**
- High-level algorithm control flow
- Calls tableau operations
- Contains stopping criteria and iteration limits

**Public API (lib.rs):**
- Clean function signatures for eval crate
- Unified error handling
- Convenient re-exports

### 2. Type Safety

All matrix operations use `achronyme_types::tensor::RealTensor`:
- Static shape checking where possible
- Runtime validation for matrix dimensions
- No raw index arithmetic on flat vectors (except in performance-critical inner loops)

### 3. Error Handling

Consistent error reporting using `Result<T, String>`:
- **Infeasible problems:** "Problem is infeasible..."
- **Unbounded problems:** "Unbounded problem..."
- **Dimension mismatch:** "Matrix A has X rows but b has Y elements"
- **Invalid inputs:** "sense must be 1.0 or -1.0"

## Module Breakdown

---

## Linear Programming Module (`linear/`)

### Core: Tableau Data Structure (`tableau.rs`)

The `Tableau` struct is the foundation of all simplex methods:

```rust
pub struct Tableau {
    pub data: Vec<Vec<f64>>,      // (m+1) × (n+m+1) matrix
    pub num_vars: usize,           // n (decision variables)
    pub num_constraints: usize,    // m (constraints)
    pub basis: Vec<usize>,         // Current basis (m indices)
}
```

**Layout:**
```
Columns: [x₁ ... xₙ | s₁ ... sₘ | RHS]
Rows:    [  Constraint rows (m)        ]
         [  Objective row (1)          ]
```

**Key Methods:**

1. **`new(c, a, b, sense)`** - Construct initial tableau
   - Validates dimensions
   - Adds slack variables
   - Sets up initial basis (slack variables)
   - Transforms objective (negate for maximization)

2. **`is_optimal()`** - Check optimality
   - For max: all objective coefficients ≥ 0
   - For min: all objective coefficients ≤ 0
   - Tolerance: 10^(-10)

3. **`find_entering_variable()`** - Select column to enter basis
   - **Rule:** Most negative coefficient in objective row
   - **Ties:** First occurrence (could use Bland's rule)
   - Returns `None` if optimal

4. **`find_leaving_variable(entering)`** - Select row to leave basis
   - **Minimum ratio test:** min{b[i] / a[i][entering] : a[i][entering] > 0}
   - **Unbounded:** If no positive pivot candidates
   - **Degeneracy:** Multiple rows with same ratio (uses first)

5. **`pivot(entering, leaving)`** - Perform pivot operation
   - Normalize pivot row (make pivot element = 1)
   - Eliminate pivot column in all other rows
   - Update basis vector

6. **`extract_solution()`** - Read solution from tableau
   - Basic variables: Read from RHS
   - Non-basic variables: Set to 0
   - Returns only decision variables (not slacks)

7. **`objective_value()`** - Current objective value
   - Read from RHS of objective row

**Complexity:**
- `new`: O(m × n)
- `is_optimal`: O(n + m)
- `find_entering_variable`: O(n + m)
- `find_leaving_variable`: O(m)
- `pivot`: O((m+1) × (n+m+1)) = O(m × n)
- `extract_solution`: O(m × n) (searches basis)

---

### Algorithm 1: Primal Simplex (`simplex.rs`)

**High-level pseudocode:**
```
1. Create initial tableau with slack variables
2. WHILE not optimal:
     a. Find entering variable (most negative cost)
     b. Find leaving variable (minimum ratio test)
     c. Pivot
3. Extract solution
```

**Implementation details:**

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    // Validate inputs
    if sense != 1.0 && sense != -1.0 { return Err(...) }

    // Create initial tableau
    let mut tableau = Tableau::new(c, a, b, sense)?;

    // Main loop
    let max_iterations = 10000;
    for iteration in 0..max_iterations {
        // Check optimality
        if tableau.is_optimal() {
            return Ok(tableau.extract_solution());
        }

        // Select entering variable
        let entering = tableau.find_entering_variable()
            .ok_or("Optimal")?;

        // Select leaving variable
        let leaving = tableau.find_leaving_variable(entering)?;

        // Pivot
        tableau.pivot(entering, leaving);
    }

    Err("Max iterations reached")
}
```

**Termination:**
- **Optimal:** All costs reduced to ≥ 0
- **Unbounded:** No valid leaving variable
- **Max iterations:** Safety limit (10,000)

**Degeneracy handling:** Not implemented (uses first occurrence in ties)

**Cycling:** Rare in practice, could add Bland's rule or perturbation

---

### Algorithm 2: Dual Simplex (`dual_simplex.rs`)

**Key difference from primal:**
- Primal: Maintains primal feasibility, seeks optimality
- Dual: Maintains dual feasibility (optimal costs), seeks primal feasibility

**When to use:**
- Starting with dual feasible but primal infeasible solution
- Adding constraints to already-solved problem
- Problems with many variables, few constraints

**Algorithm:**
```
1. Create tableau (may have negative RHS)
2. WHILE not primal feasible:
     a. Find leaving variable (most negative RHS)
     b. Find entering variable (dual ratio test)
     c. Pivot
3. If optimal AND feasible → done
```

**Dual ratio test:**
```rust
fn find_entering_variable_dual(tableau: &Tableau, leaving_row: usize)
    -> Result<usize, String>
{
    // For each column j with a[leaving][j] < 0:
    //   ratio = |c[j]| / |a[leaving][j]|
    // Choose j with minimum ratio

    for j in 0..total_cols {
        if a[leaving][j] < -ε {
            let ratio = c[j].abs() / a[leaving][j].abs();
            if ratio < min_ratio {
                min_ratio = ratio;
                entering = j;
            }
        }
    }
}
```

**Primal feasibility check:**
```rust
fn is_primal_feasible(tableau: &Tableau) -> bool {
    // All RHS values must be non-negative
    for i in 0..m {
        if tableau.data[i][RHS] < -ε { return false; }
    }
    true
}
```

---

### Algorithm 3: Two-Phase Simplex (`two_phase.rs`)

**Problem:** Standard simplex requires initial BFS (all b[i] ≥ 0). What if:
- Some b[i] < 0
- Equality constraints (no obvious slack variables)
- ≥ constraints

**Solution:** Two-Phase Method

#### Phase 1: Find Initial BFS

**Objective:** Minimize sum of artificial variables

**Setup:**
1. For each constraint with b[i] < 0:
   - Multiply row by -1 (flip to b[i] > 0)
   - Add artificial variable a[i]
2. Objective: minimize Σ a[i]
3. Solve auxiliary LP

**Success:** If min Σ a[i] = 0, found BFS for original problem

**Failure:** If min Σ a[i] > 0, original problem is infeasible

**Implementation:**
```rust
fn build_phase1_tableau(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Tableau, String>
{
    // Count constraints needing artificials
    let num_artificials = b.iter().filter(|&&bi| bi < 0.0).count();

    // Build extended tableau: [x | s | a | RHS]
    let mut data = vec![vec![0.0; n+m+num_artificials+1]; m+1];

    // For each constraint:
    for i in 0..m {
        if b[i] < 0.0 {
            // Flip row: -a[i] x ≤ -b[i]  =>  a[i] x ≥ b[i]
            // Add artificial: a[i] x + artificial = b[i]
            for j in 0..n {
                data[i][j] = -a.get(i, j);
            }
            data[i][n+m+artificial_idx] = 1.0;
            data[i][RHS] = -b[i];
        } else {
            // Normal constraint with slack
            for j in 0..n {
                data[i][j] = a.get(i, j);
            }
            data[i][n+i] = 1.0;
            data[i][RHS] = b[i];
        }
    }

    // Objective: minimize Σ artificials
    for j in (n+m)..(n+m+num_artificials) {
        data[m][j] = 1.0;
    }

    // CRITICAL: Make objective row compatible with basis
    // (Subtract artificial variable rows from objective)
    for i in 0..m {
        if has_artificial[i] {
            for j in 0..total_cols {
                data[m][j] -= data[i][j];
            }
        }
    }
}
```

#### Phase 2: Solve Original Problem

**Setup:**
1. Use basis from Phase 1 (without artificials)
2. Replace Phase 1 objective with original objective
3. Continue simplex from this BFS

**Implementation:**
```rust
fn build_phase2_tableau(phase1: &Tableau, c: &[f64], sense: f64, n: usize, m: usize)
    -> Result<Tableau, String>
{
    // Copy constraint rows (without artificial columns)
    let mut data = vec![vec![0.0; n+m+1]; m+1];
    for i in 0..m {
        for j in 0..(n+m) {
            data[i][j] = phase1.data[i][j];
        }
        data[i][n+m] = phase1.data[i][RHS];
    }

    // Set original objective
    for j in 0..n {
        data[m][j] = -sense * c[j];
    }

    // Copy basis (check no artificials remain)
    let basis = phase1.basis.clone();
    for &b in &basis {
        if b >= n+m {
            return Err("Artificial variable in basis");
        }
    }
}
```

---

### Algorithm 4: Revised Simplex (`revised_simplex.rs`)

**Motivation:** Standard simplex stores full tableau (m × (n+m+1)) = O(m × n) space

**Revised Simplex:** Only store basis inverse B^(-1) (m × m) = O(m^2) space

**When beneficial:** n >> m (many variables, few constraints)

**Key idea:**
- Don't store full tableau
- Recompute tableau columns on-demand
- Maintain inverse basis matrix B^(-1)

**Algorithm:**
```
1. Initialize basis B = I (slack variables)
2. WHILE not optimal:
     a. Compute simplex multipliers: π = c_B^T B^(-1)
     b. FOR each non-basic variable j:
          compute reduced cost: r_j = c_j - π^T A_j
     c. Select entering variable (most negative r_j)
     d. Compute direction: d = B^(-1) A_entering
     e. Minimum ratio test: θ = min{x_B[i] / d[i] : d[i] > 0}
     f. Update basis, compute new B^(-1)
3. Extract solution
```

**Data structures:**
```rust
struct RevisedSimplex {
    aug_a: Vec<f64>,           // Augmented matrix [A | I]
    c_aug: Vec<f64>,            // Augmented costs [c | 0]
    basis: Vec<usize>,          // Basis indices
    x_b: Vec<f64>,              // Basic variable values
}
```

**Key operations:**

1. **Compute basis inverse:**
```rust
fn compute_basis_inverse(aug_a: &[f64], basis: &[usize], m: usize, n: usize)
    -> Result<Vec<f64>, String>
{
    // Extract basis columns into matrix B (m × m)
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

2. **Gauss-Jordan inversion:**
```rust
fn invert_matrix(mat: &[f64], n: usize) -> Result<Vec<f64>, String> {
    // Create augmented matrix [A | I]
    let mut aug = vec![0.0; n * 2*n];
    for i in 0..n {
        for j in 0..n {
            aug[i*2*n + j] = mat[i*n + j];
        }
        aug[i*2*n + (n+i)] = 1.0;  // Identity
    }

    // Gaussian elimination with partial pivoting
    for k in 0..n {
        // Find pivot
        let (max_row, max_val) = find_max_in_column(&aug, k, n);
        if max_val < ε { return Err("Singular matrix"); }

        // Swap rows
        swap_rows(&mut aug, k, max_row, 2*n);

        // Normalize pivot row
        let pivot = aug[k*2*n + k];
        for j in 0..2*n {
            aug[k*2*n + j] /= pivot;
        }

        // Eliminate column k in other rows
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
    extract_right_half(&aug, n)
}
```

3. **Reduced cost calculation:**
```rust
// For each non-basic variable j:
let a_j = get_column(&aug_a, j, m, total_vars);
let pi_a_j = dot_product(&pi, &a_j);
let r_j = c_aug[j] - pi_a_j;
```

4. **Basis update:**
```rust
// After selecting entering and leaving:
for i in 0..m {
    x_b[i] -= θ * d[i];
}
x_b[leaving_row] = θ;
basis[leaving_row] = entering;
```

**Complexity per iteration:**
- Basis inversion: O(m^3)
- Reduced cost for all variables: O(n × m)
- Total: O(m^3 + n × m)

**Optimization:** In practice, use LU factorization and incremental updates instead of full re-inversion

---

### Algorithm 5: Sensitivity Analysis (`sensitivity.rs`)

Post-optimality analysis to understand solution robustness.

#### Shadow Prices (Dual Variables)

**Definition:** ∂z*/∂b[i] = marginal value of resource i

**Interpretation:**
- shadow[i] = $15 → one more unit of resource i increases profit by $15
- shadow[i] = 0 → resource i is not binding (slack > 0)

**Calculation:**
```rust
pub fn shadow_price(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    // Solve to get optimal tableau
    let tableau = solve_to_optimality(c, a, b, sense)?;

    // Shadow prices are in slack variable columns of objective row
    let mut shadow = vec![0.0; m];
    for i in 0..m {
        let slack_col = n + i;
        shadow[i] = tableau.data[m][slack_col];  // Objective row
    }

    Ok(shadow)
}
```

**Duality:** Shadow prices are optimal dual variables y*

**Economic interpretation:**
- Resource i is **binding** if shadow[i] > 0 (fully used)
- Resource i has **slack** if shadow[i] = 0 (surplus capacity)

#### Coefficient Sensitivity (c[i] ranging)

**Question:** How much can c[i] vary without changing optimal solution?

**Answer:** Range [c_min, c_max] where basis remains optimal

**For non-basic variables:**
```rust
// If x[i] is non-basic (= 0 in optimal solution):
// Range: (-∞, c[i] + reduced_cost[i]]
// Because if c[i] increases beyond this, x[i] would enter basis

let reduced_cost = tableau.data[m][i];  // Objective row
let c_max = c[i] + reduced_cost;
return [f64::NEG_INFINITY, c_max];
```

**For basic variables:**
```rust
// If x[i] is basic (> 0 in optimal solution):
// More complex: need to check dual feasibility
// (Not fully implemented - returns conservative range)

// Conservative estimate:
return [c[i] * 0.5, c[i] * 2.0];
```

#### RHS Sensitivity (b[i] ranging)

**Question:** How much can b[i] vary without changing basis?

**Answer:** Range [b_min, b_max] where all basic variables remain non-negative

**Calculation:**
```rust
// Δb[i] causes change in basic variables: Δx_B = B^(-1) Δb
// For basis to remain optimal: x_B + B^(-1) Δb ≥ 0
// This gives bounds on Δb[i]

// Current implementation uses conservative estimate:
let b_min = b[i] - b[i].abs() * 0.5;
let b_max = b[i] + b[i].abs() * 0.5;
return [b_min.max(0.0), b_max];
```

**Full implementation would:**
1. Compute B^(-1)
2. Extract column i of B^(-1)
3. For each row j: compute max Δb such that x_B[j] + (B^(-1))[j,i] * Δb ≥ 0
4. Return intersection of all bounds

---

### Linprog Wrapper (`linprog.rs`)

Auto-selects best algorithm based on problem characteristics:

```rust
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    let n = c.len();
    let m = a.rows();

    // Selection heuristics (future):
    // - Large problems (n, m > 5000): Interior Point
    // - Many variables, few constraints (m < n/2): Dual Simplex
    // - Sparse matrix: Revised Simplex

    // Current: always use Primal Simplex
    simplex::solve(c, a, b, sense)
}
```

**Future enhancements:**
```rust
if n > 5000 || m > 5000 {
    interior_point::solve(c, a, b, sense)
} else if m < n / 2 {
    dual_simplex::solve(c, a, b, sense)
} else if is_sparse(a) {
    revised_simplex::solve(c, a, b, sense)
} else {
    simplex::solve(c, a, b, sense)
}
```

---

## Integer Programming Module (`integer/`)

### Refactored Structure (`branch_bound/`)

The Branch & Bound module was recently refactored for modularity:

**Old structure:** Single monolithic file
**New structure:** Modular organization

```
branch_bound/
├── mod.rs          # Public API, tests
├── node.rs         # BBNode data structure
├── helpers.rs      # Utility functions
├── bounded_lp.rs   # LP with variable bounds
└── solvers.rs      # Main algorithms
```

---

### Data Structure: BBNode (`node.rs`)

Represents a node in the Branch & Bound search tree:

```rust
#[derive(Clone)]
pub struct BBNode {
    pub lower_bounds: Vec<f64>,  // x[i] ≥ lower[i]
    pub upper_bounds: Vec<f64>,  // x[i] ≤ upper[i]
}

impl BBNode {
    // Create root node: x ∈ [0, ∞)^n
    pub fn initial(n: usize) -> Self {
        Self {
            lower_bounds: vec![0.0; n],
            upper_bounds: vec![f64::INFINITY; n],
        }
    }

    // Create node with custom bounds
    pub fn new(lower: Vec<f64>, upper: Vec<f64>) -> Self {
        Self {
            lower_bounds: lower,
            upper_bounds: upper,
        }
    }
}
```

**Interpretation:**
- Each node represents a subproblem with additional bound constraints
- Branching creates two child nodes with tighter bounds
- Example: x₁ = 2.7 → branch on x₁ ≤ 2 (left) and x₁ ≥ 3 (right)

---

### Utility Functions (`helpers.rs`)

**1. Integer feasibility check:**
```rust
const TOLERANCE: f64 = 1e-6;

pub fn is_integer_solution(solution: &[f64], integer_vars: &[usize]) -> bool {
    for &idx in integer_vars {
        let val = solution[idx];
        let rounded = val.round();
        if (val - rounded).abs() > TOLERANCE {
            return false;
        }
    }
    true
}
```

**2. Branching variable selection:**
```rust
pub fn find_fractional_var(solution: &[f64], integer_vars: &[usize])
    -> Option<usize>
{
    // "Most fractional" heuristic: choose variable closest to 0.5
    let mut best_var = None;
    let mut best_fractionality = f64::INFINITY;

    for &idx in integer_vars {
        let val = solution[idx];
        let frac = val - val.floor();

        if frac > TOLERANCE && frac < (1.0 - TOLERANCE) {
            let fractionality = (frac - 0.5).abs();
            if fractionality < best_fractionality {
                best_fractionality = fractionality;
                best_var = Some(idx);
            }
        }
    }

    best_var
}
```

**Heuristic rationale:**
- Variable at x = 0.5 is "most fractional" (furthest from integer)
- Branching on it creates most balanced subproblems
- Alternative: First fractional variable (simpler, less effective)

**3. Pruning check:**
```rust
pub fn should_prune(node_objective: f64, best_objective: f64, sense: f64) -> bool {
    if sense > 0.0 {
        // Maximize: prune if node_obj ≤ best_obj
        node_objective <= best_objective
    } else {
        // Minimize: prune if node_obj ≥ best_obj
        node_objective >= best_objective
    }
}
```

**4. Objective comparison:**
```rust
pub fn is_better(obj1: f64, obj2: f64, sense: f64) -> bool {
    if sense > 0.0 {
        obj1 > obj2  // Maximize
    } else {
        obj1 < obj2  // Minimize
    }
}
```

---

### LP with Bounds (`bounded_lp.rs`)

Solves LP subproblems with variable bounds: lower[i] ≤ x[i] ≤ upper[i]

**Approach:** Transform to standard form by adding bound constraints

```rust
pub fn solve_with_bounds(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    lower: &[f64],
    upper: &[f64]
) -> Result<Vec<f64>, String>
{
    let n = c.len();
    let m = a.rows();

    // 1. Handle fixed variables (lower[i] == upper[i])
    let mut fixed_vars = vec![None; n];
    let mut modified_c = c.to_vec();

    for i in 0..n {
        if (upper[i] - lower[i]).abs() < 1e-9 {
            fixed_vars[i] = Some(lower[i]);
            modified_c[i] = 0.0;  // Remove from objective
        }
    }

    // 2. Substitute fixed variables in constraints
    let mut new_b = b.to_vec();
    for i in 0..m {
        for j in 0..n {
            if let Some(fixed_val) = fixed_vars[j] {
                new_b[i] -= a.get(i, j) * fixed_val;
            }
        }
    }

    // 3. Add upper bound constraints: x[i] ≤ upper[i]
    let mut new_rows = Vec::new();
    for i in 0..n {
        if fixed_vars[i].is_none() && upper[i] < f64::INFINITY {
            let mut row = vec![0.0; n];
            row[i] = 1.0;
            new_rows.push(row);
            new_b.push(upper[i]);
        }
    }

    // 4. Build augmented constraint matrix
    let combined_a = build_combined_matrix(a, &new_rows, n, m);

    // 5. Solve LP
    let mut sol = simplex::solve(&modified_c, &combined_a, &new_b, sense)?;

    // 6. Restore fixed variables
    for i in 0..n {
        if let Some(fixed_val) = fixed_vars[i] {
            sol[i] = fixed_val;
        }
    }

    Ok(sol)
}
```

**Limitation:** Lower bounds > 0 are not fully enforced
- Relies on simplex producing vertex solutions
- Combined with upper bounds, usually works in practice
- Could be improved with variable substitution: y = x - lower

---

### Main Algorithms (`solvers.rs`)

#### Algorithm 1: Integer Linear Programming

```rust
pub fn intlinprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    integer_vars: &[usize]
) -> Result<Vec<f64>, String>
```

**Pseudocode:**
```
1. Solve LP relaxation (ignore integer constraints)
2. IF solution is integer → return it
3. Initialize:
     best_solution = None
     best_objective = -∞ (max) or +∞ (min)
     stack = [root_node]
4. WHILE stack not empty:
     a. Pop node from stack
     b. Solve LP with node bounds
     c. IF infeasible → prune (continue)
     d. IF objective worse than best → prune (bound)
     e. IF solution is integer:
          Update best_solution if better
          Continue (prune by integrality)
     f. ELSE (fractional solution):
          Select fractional variable
          Create two child nodes (floor and ceil)
          Push children to stack
5. Return best_solution
```

**Implementation:**
```rust
pub fn intlinprog(...) -> Result<Vec<f64>, String> {
    // Solve LP relaxation
    let relaxed = simplex::solve(c, a, b, sense)?;
    if is_integer_solution(&relaxed, integer_vars) {
        return Ok(relaxed);
    }

    // Branch & Bound
    let mut best_solution = None;
    let mut best_objective = if sense > 0.0 { f64::NEG_INFINITY } else { f64::INFINITY };
    let mut stack = vec![BBNode::initial(n)];

    while let Some(node) = stack.pop() {
        // Solve LP with bounds
        let node_sol = solve_with_bounds(c, a, b, sense,
                                          &node.lower_bounds,
                                          &node.upper_bounds)?;

        // Bound check
        let node_obj = objective_value(c, &node_sol)?;
        if should_prune(node_obj, best_objective, sense) {
            continue;
        }

        // Integer check
        if is_integer_solution(&node_sol, integer_vars) {
            if is_better(node_obj, best_objective, sense) {
                best_objective = node_obj;
                best_solution = Some(node_sol);
            }
            continue;
        }

        // Branch
        if let Some(branch_var) = find_fractional_var(&node_sol, integer_vars) {
            let val = node_sol[branch_var];

            // Left: x[i] ≤ floor(val)
            let mut left = node.clone();
            left.upper_bounds[branch_var] = val.floor();
            stack.push(left);

            // Right: x[i] ≥ ceil(val)
            let mut right = node;
            right.lower_bounds[branch_var] = val.ceil();
            stack.push(right);
        }
    }

    best_solution.ok_or("No integer solution found")
}
```

**Key features:**
- **Depth-First Search (DFS):** Uses stack (LIFO)
- **Pruning:** By bound, infeasibility, integrality
- **Branching:** On most fractional variable
- **Termination:** When all nodes pruned or explored

**Complexity:**
- Worst case: O(2^k × LP_solve_time) where k = integer variables
- Exponential in k (NP-hard problem)
- Pruning significantly reduces tree in practice

---

#### Algorithm 2: Binary Linear Programming

Special case: variables ∈ {0, 1}

```rust
pub fn binary_linprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    binary_vars: &[usize]
) -> Result<Vec<f64>, String>
```

**Differences from general ILP:**

1. **Tighter bounds:** 0 ≤ x[i] ≤ 1 instead of 0 ≤ x[i] < ∞
2. **Binary branching:** Always branch on 0 and 1 (not floor/ceil)
3. **Better pruning:** Stronger LP relaxation bounds

**Implementation:**
```rust
pub fn binary_linprog(...) -> Result<Vec<f64>, String> {
    // Set binary bounds
    let mut upper_bounds = vec![f64::INFINITY; n];
    for &idx in binary_vars {
        upper_bounds[idx] = 1.0;
    }

    // Solve with binary-aware intlinprog
    binary_intlinprog(c, a, b, sense, binary_vars, &upper_bounds)
}

fn binary_intlinprog(...) -> Result<Vec<f64>, String> {
    // Similar to intlinprog but:

    // Branching for binary variables:
    if var_upper_bounds[branch_var] <= 1.0 {
        // Binary: branch on 0 and 1
        (left_val, right_val) = (0.0, 1.0);
    } else {
        // General integer: branch on floor and ceil
        (left_val, right_val) = (val.floor(), val.ceil());
    }

    // Priority ordering for DFS:
    if sense > 0.0 {
        // Maximize: explore x=1 first (push last to stack)
        stack.push(left_node);   // x = 0
        stack.push(right_node);  // x = 1 (explored first)
    } else {
        // Minimize: explore x=0 first
        stack.push(right_node);  // x = 1
        stack.push(left_node);   // x = 0 (explored first)
    }
}
```

**Applications:**
- **Knapsack:** maximize Σ v[i] x[i] subject to Σ w[i] x[i] ≤ W, x[i] ∈ {0,1}
- **Assignment:** one-to-one matching
- **Set covering:** choose minimum subsets
- **Scheduling:** binary task selection

---

## Testing Strategy

### Unit Tests

Each module has dedicated tests:

**Tableau tests (`tableau.rs`):**
- Tableau creation
- Optimality check
- Entering/leaving variable selection
- Pivot operations
- Solution extraction

**Algorithm tests:**
- Simple 2D problems (easy to verify by hand)
- Classic problems (production planning, knapsack)
- Edge cases (unbounded, infeasible, degenerate)

### Integration Tests

**Linear programming (`mod.rs`):**
- Multi-constraint problems
- Different objective senses (max/min)
- Negative coefficients
- Tight constraints

**Integer programming (`branch_bound/mod.rs`):**
- Small ILP instances
- Knapsack variants (different capacities)
- Multiple constraints
- Binary vs. general integer

### Test Coverage

```bash
# Run all solver tests
cargo test -p achronyme-solver

# Run with coverage
cargo tarpaulin -p achronyme-solver

# Run specific module
cargo test -p achronyme-solver linear::simplex
```

**Coverage goals:**
- All public functions tested
- All error paths exercised
- Edge cases validated
- Numerical stability checks

---

## Performance Considerations

### Memory Usage

| Algorithm | Space Complexity | Notes |
|-----------|-----------------|-------|
| Primal Simplex | O(m × n) | Full tableau |
| Dual Simplex | O(m × n) | Full tableau |
| Two-Phase | O(m × (n+k)) | k = artificials |
| Revised Simplex | O(m^2) | Basis inverse only |
| Branch & Bound | O(tree depth × n) | Stack of nodes |

### Time Complexity (per iteration)

| Algorithm | Complexity | Bottleneck |
|-----------|-----------|-----------|
| Primal Simplex | O(m × n) | Pivot operation |
| Dual Simplex | O(m × n) | Pivot operation |
| Revised Simplex | O(m^3 + m^2 n) | Basis inversion |
| Branch & Bound | O(LP_solve × tree_size) | Exponential tree |

### Optimization Opportunities

**1. Sparse matrices:**
- Use CSR/CSC format instead of dense
- Only store non-zero elements
- Specialized sparse pivot operations

**2. Basis updates:**
- LU factorization instead of full inversion
- Incremental updates (product form)
- Periodic re-factorization for stability

**3. Presolve:**
- Remove redundant constraints
- Fix variables with forced values
- Tighten bounds
- Detect infeasibility early

**4. Branching heuristics:**
- Strong branching (try several variables)
- Pseudocost branching (historical performance)
- Reliability branching (hybrid)

**5. Cutting planes:**
- Add valid inequalities to strengthen relaxation
- Gomory cuts, cover inequalities
- Reduce Branch & Bound tree size

---

## Extension Guide

### Adding a New LP Algorithm

**Example: Interior Point Method**

1. Create `src/linear/interior_point.rs`:
```rust
use achronyme_types::tensor::RealTensor;

pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64)
    -> Result<Vec<f64>, String>
{
    // 1. Initialize primal-dual interior point
    // 2. Iterate: compute Newton direction
    // 3. Line search with barrier parameter
    // 4. Check convergence
    // 5. Return primal solution
}
```

2. Add to `src/linear/mod.rs`:
```rust
pub mod interior_point;
pub use interior_point::solve as interior_point_solve;
```

3. Update `linprog.rs` to use it:
```rust
if n > 5000 || m > 5000 {
    interior_point::solve(c, a, b, sense)
} else {
    simplex::solve(c, a, b, sense)
}
```

### Adding a New ILP Heuristic

**Example: Greedy Knapsack Heuristic**

1. Create `src/integer/heuristics.rs`:
```rust
pub fn greedy_knapsack(c: &[f64], weights: &[f64], capacity: f64)
    -> Vec<f64>
{
    // 1. Compute value/weight ratios
    // 2. Sort items by ratio (descending)
    // 3. Greedily pack items until full
    // 4. Return binary solution
}
```

2. Use in `branch_bound/solvers.rs`:
```rust
// Compute initial heuristic solution
let heuristic_sol = greedy_knapsack(c, weights, capacity);
if is_feasible(&heuristic_sol, a, b) {
    best_solution = Some(heuristic_sol);
    best_objective = objective_value(c, &heuristic_sol);
}
// Then run Branch & Bound with warm start
```

### Adding Presolve

Create `src/presolve.rs`:
```rust
pub struct Presolve {
    pub reduced_c: Vec<f64>,
    pub reduced_a: RealTensor,
    pub reduced_b: Vec<f64>,
    pub fixed_vars: Vec<Option<f64>>,
}

pub fn presolve(c: &[f64], a: &RealTensor, b: &[f64])
    -> Result<Presolve, String>
{
    // 1. Detect and remove redundant constraints
    // 2. Fix variables with forced values
    // 3. Tighten variable bounds
    // 4. Return reduced problem
}

pub fn postsolve(reduced_sol: &[f64], presolve: &Presolve)
    -> Vec<f64>
{
    // Reconstruct full solution from reduced solution
}
```

---

## Debugging Tips

### Common Issues

**1. Numerical instability:**
- **Symptom:** Incorrect solution, negative values where should be zero
- **Cause:** Floating-point rounding errors
- **Fix:** Increase tolerance, use perturbation, re-factor basis

**2. Cycling:**
- **Symptom:** Simplex iterates forever with same objective
- **Cause:** Degenerate problem (multiple optimal bases)
- **Fix:** Implement Bland's rule, lexicographic pivoting

**3. Slow Branch & Bound:**
- **Symptom:** Exceeds iteration limit, explores too many nodes
- **Cause:** Weak LP relaxation, poor branching heuristic
- **Fix:** Add cutting planes, use strong branching, tighten bounds

**4. Infeasibility not detected:**
- **Symptom:** "No solution found" but problem should be feasible
- **Cause:** Numerical errors, incorrect transformation
- **Fix:** Check constraint conversion, validate input data

### Debugging Techniques

**1. Enable verbose logging:**
```rust
fn solve(...) -> Result<Vec<f64>, String> {
    for iteration in 0..max_iterations {
        eprintln!("Iteration {}: obj = {}", iteration, tableau.objective_value());
        eprintln!("  Basis: {:?}", tableau.basis);
        eprintln!("  Solution: {:?}", tableau.extract_solution());

        // ... algorithm ...
    }
}
```

**2. Validate intermediate states:**
```rust
// After pivot:
assert!(tableau.is_canonical(), "Basis not canonical after pivot");
assert!(all_non_negative(&tableau.extract_solution()), "Negative basic variable");
```

**3. Compare with known solution:**
```rust
#[test]
fn test_against_known_optimal() {
    let solution = simplex::solve(&c, &a, &b, 1.0).unwrap();
    let expected = vec![2.0, 6.0];
    for i in 0..solution.len() {
        assert!((solution[i] - expected[i]).abs() < 1e-6);
    }
}
```

**4. Visualize Branch & Bound tree:**
```rust
fn solve_with_trace(...) {
    eprintln!("Node {}: bounds {:?}", node_id, node.bounds);
    eprintln!("  LP objective: {}", node_obj);
    eprintln!("  Branching on x[{}] = {}", branch_var, val);
    eprintln!("  Children: [{}, {}]", left_id, right_id);
}
```

---

## References

### Implementation Guides
- **Chvátal** (1983). *Linear Programming*. W.H. Freeman. (Detailed simplex description)
- **Nocedal & Wright** (2006). *Numerical Optimization*. Springer. (Revised simplex, interior point)
- **Nemhauser & Wolsey** (1988). *Integer and Combinatorial Optimization*. Wiley. (Branch & Bound)

### Research Papers
- **Dantzig** (1963). *Linear Programming and Extensions*. (Original simplex algorithm)
- **Karmarkar** (1984). "A new polynomial-time algorithm for linear programming." *Combinatorica*. (Interior point)
- **Bland** (1977). "New finite pivoting rules for the simplex method." (Anti-cycling)

### Software
- **GLPK** (GNU Linear Programming Kit): Open-source LP/MIP solver
- **COIN-OR CLP**: High-performance LP solver
- **Gurobi, CPLEX**: Commercial state-of-the-art solvers

Compare implementation with these for validation and performance benchmarking.
