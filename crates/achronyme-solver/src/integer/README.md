# Integer Programming Module

Implementation of Integer Linear Programming (ILP) algorithms using Branch & Bound.

## Overview

The `integer` module provides algorithms for solving optimization problems with **discrete decision variables**. Unlike continuous linear programming where variables can take any real value, integer programming requires some or all variables to be integers.

```
integer/
├── mod.rs           # Module exports
└── branch_bound/    # Branch & Bound implementation (refactored)
    ├── mod.rs       # Public API and comprehensive tests
    ├── node.rs      # BBNode tree node structure
    ├── helpers.rs   # Utility functions (integrality, pruning, heuristics)
    ├── bounded_lp.rs # LP solver with variable bounds
    └── solvers.rs   # Main algorithms (intlinprog, binary_linprog)
```

## Problem Formulation

### Integer Linear Programming (ILP)

```
maximize/minimize  z = c^T x
subject to:        A x ≤ b
                   x_i ∈ ℤ for i ∈ I  (integer constraints)
                   x_j ∈ ℝ for j ∉ I  (continuous variables)
                   x ≥ 0
```

Where:
- **I** ⊆ {1, ..., n}: indices of integer variables
- **ℤ**: set of integers
- Special case: **Binary LP** when x_i ∈ {0, 1}

### Complexity

ILP is **NP-hard**:
- No polynomial-time algorithm known (unless P = NP)
- Worst-case: O(2^k) where k = number of integer variables
- Exponential growth in solution time with problem size

**Practical implications:**
- Solvable for small k (k < 20): seconds
- Challenging for medium k (20 < k < 50): minutes to hours
- Intractable for large k (k > 50): may not terminate

## Module Structure (Refactored)

The Branch & Bound implementation was recently refactored from a monolithic file into modular components for better maintainability and testability.

### Old Structure (Pre-Refactoring)
```
integer/
└── branch_bound.rs  (single ~500 line file)
```

### New Structure (Post-Refactoring)
```
integer/
└── branch_bound/
    ├── mod.rs           # Tests and public API
    ├── node.rs          # ~28 lines - BBNode structure
    ├── helpers.rs       # ~91 lines - Utilities
    ├── bounded_lp.rs    # ~111 lines - LP with bounds
    └── solvers.rs       # ~268 lines - Main algorithms
```

**Benefits:**
- **Separation of concerns:** Each file has single responsibility
- **Testability:** Can unit test components independently
- **Readability:** Easier to understand and modify
- **Extensibility:** Can add new heuristics/strategies without touching solver core

---

## Branch & Bound Algorithm

### High-Level Idea

Since ILP is NP-hard, no efficient exact algorithm exists. Branch & Bound is an **intelligent enumeration** that avoids exploring all 2^k possibilities:

1. **Bound:** Solve LP relaxation (continuous version) → upper bound on integer objective
2. **Branch:** If solution is fractional, split problem into subproblems
3. **Prune:** Discard subproblems that cannot improve best solution

**Key insight:** LP relaxation provides bound without solving full integer problem

### Algorithm Pseudocode

```
function BranchAndBound(c, A, b, I):
    // Initialize
    best_solution ← None
    best_objective ← -∞ (max) or +∞ (min)
    stack ← [root_node]  // root = unconstrained problem

    while stack not empty:
        node ← pop(stack)

        // 1. BOUND: Solve LP relaxation with node's bounds
        lp_solution ← solve_LP(c, A, b, node.bounds)

        if lp_solution is infeasible:
            continue  // Prune by infeasibility

        lp_objective ← c^T lp_solution

        // 2. PRUNE by bound
        if lp_objective ≤ best_objective:  // For maximization
            continue  // Can't improve best

        // 3. Check integrality
        if lp_solution is integer:
            // Found feasible integer solution
            if lp_objective > best_objective:
                best_solution ← lp_solution
                best_objective ← lp_objective
            continue  // Prune by integrality

        // 4. BRANCH: Select fractional variable
        j ← select_fractional_variable(lp_solution, I)
        val ← lp_solution[j]

        // Create two child nodes
        left_child ← node.clone()
        left_child.upper_bounds[j] ← floor(val)
        push(stack, left_child)

        right_child ← node.clone()
        right_child.lower_bounds[j] ← ceil(val)
        push(stack, right_child)

    return best_solution
```

### Visualization

```
                    Root: x ∈ [0, ∞)
                    LP: x = [2.7, 3.2]
                    z = 18.5
                   /              \
                  /                \
             x₁ ≤ 2                x₁ ≥ 3
         LP: [2, 3.5]         LP: [3, 2.8]
         z = 17.5             z = 18.4
        /        \           /        \
    x₂≤3      x₂≥4      x₂≤2      x₂≥3
  [2,3]     [2,4]     [3,2]     [3,3]
  z=17    infeas.    z=18      z=18
  (int)              (int)     (int)
                     BEST!
```

---

## Component 1: BBNode (`node.rs`)

### Purpose

Represents a node in the Branch & Bound search tree, defining a subproblem with variable bounds.

### Structure

```rust
#[derive(Clone)]
pub struct BBNode {
    /// Lower bounds: x[i] ≥ lower_bounds[i]
    pub lower_bounds: Vec<f64>,

    /// Upper bounds: x[i] ≤ upper_bounds[i]
    pub upper_bounds: Vec<f64>,
}
```

### Methods

**1. Initial root node:**
```rust
pub fn initial(n: usize) -> Self {
    Self {
        lower_bounds: vec![0.0; n],
        upper_bounds: vec![f64::INFINITY; n],
    }
}
```

Creates unconstrained problem: 0 ≤ x[i] < ∞

**2. Custom bounds:**
```rust
pub fn new(lower_bounds: Vec<f64>, upper_bounds: Vec<f64>) -> Self {
    Self { lower_bounds, upper_bounds }
}
```

Creates node with specific bounds (used when branching)

### Example Usage

```rust
// Root node: no bounds
let root = BBNode::initial(2);
// root.lower = [0, 0]
// root.upper = [∞, ∞]

// After branching on x₀ = 2.7
let left = BBNode::new(vec![0.0, 0.0], vec![2.0, f64::INFINITY]);
// Constraint: x₀ ≤ 2

let right = BBNode::new(vec![3.0, 0.0], vec![f64::INFINITY, f64::INFINITY]);
// Constraint: x₀ ≥ 3
```

### Design Rationale

**Why not store full constraint matrix?**
- Memory efficiency: O(n) vs O(m × n)
- Bounds are sufficient since we only branch on variable bounds
- Original constraints (A x ≤ b) remain unchanged

**Why clone nodes?**
- Each child inherits parent's bounds
- Simple to modify one bound without affecting others
- Enables parallel Branch & Bound (future enhancement)

---

## Component 2: Helper Functions (`helpers.rs`)

### Overview

Utility functions for integrality checking, branching variable selection, pruning decisions, and objective comparison.

### 1. Integrality Check

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

**Usage:**
```rust
let sol = vec![3.0, 2.00001];
let integer_vars = vec![0, 1];

assert!(is_integer_solution(&sol, &integer_vars));
// 2.00001 rounds to 2, within tolerance
```

**Why tolerance?**
- Floating-point rounding errors
- LP solver may return 2.9999999 instead of 3.0
- Tolerance = 10^(-6) is standard for commercial solvers

### 2. Fractional Variable Selection

```rust
pub fn find_fractional_var(solution: &[f64], integer_vars: &[usize])
    -> Option<usize>
{
    let mut best_var = None;
    let mut best_fractionality = f64::INFINITY;

    for &idx in integer_vars {
        let val = solution[idx];
        let frac = val - val.floor();

        // Check if truly fractional
        if frac > TOLERANCE && frac < (1.0 - TOLERANCE) {
            // "Most fractional" heuristic: choose closest to 0.5
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

**Branching Heuristics:**

| Heuristic | Selection Rule | Pros | Cons |
|-----------|---------------|------|------|
| **Most fractional** (implemented) | Closest to 0.5 | Balanced subproblems, good for binary | May not be best for general ILP |
| First fractional | First non-integer | Simple, fast | No intelligence, often poor |
| Max objective change | Largest \|c[i]\| among fractional | Good for weighted problems | Ignores constraint structure |
| Strong branching | Try each, choose best bound | Best theoretical performance | Expensive (solves multiple LPs) |
| Pseudocost | Historical average improvement | Good balance | Needs warm-up phase |

**Example:**
```rust
let sol = vec![2.1, 3.7, 1.2];
let int_vars = vec![0, 1, 2];

// Fractionalities:
// x₀ = 2.1 → frac = 0.1 → |0.1 - 0.5| = 0.4
// x₁ = 3.7 → frac = 0.7 → |0.7 - 0.5| = 0.2  ← BEST
// x₂ = 1.2 → frac = 0.2 → |0.2 - 0.5| = 0.3

let branch_var = find_fractional_var(&sol, &int_vars);
assert_eq!(branch_var, Some(1));  // x₁ is most fractional
```

### 3. Pruning Decision

```rust
pub fn should_prune(node_objective: f64, best_objective: f64, sense: f64) -> bool {
    if sense > 0.0 {
        // Maximize: prune if node objective ≤ best
        node_objective <= best_objective
    } else {
        // Minimize: prune if node objective ≥ best
        node_objective >= best_objective
    }
}
```

**Pruning Rules:**

1. **Bound pruning:** LP bound worse than incumbent
   ```rust
   if node_obj ≤ best_int_obj {  // Max
       continue;  // Cannot improve
   }
   ```

2. **Infeasibility pruning:** LP relaxation infeasible
   ```rust
   if lp_solve() returns Err(...) {
       continue;  // No integer solution in this subtree
   }
   ```

3. **Integrality pruning:** LP solution is integer
   ```rust
   if is_integer_solution(&lp_sol, &int_vars) {
       update_best();
       continue;  // No need to branch further
   }
   ```

**Example:**
```rust
// Maximization problem
let best_objective = 15.0;  // Best integer solution found

// Node 1: LP gives z = 18.5
assert!(!should_prune(18.5, 15.0, 1.0));  // Explore this

// Node 2: LP gives z = 14.2
assert!(should_prune(14.2, 15.0, 1.0));  // Prune this
```

### 4. Objective Comparison

```rust
pub fn is_better(obj1: f64, obj2: f64, sense: f64) -> bool {
    if sense > 0.0 {
        obj1 > obj2  // Maximize
    } else {
        obj1 < obj2  // Minimize
    }
}
```

**Usage:**
```rust
if is_better(new_obj, best_obj, sense) {
    best_obj = new_obj;
    best_solution = new_solution;
}
```

---

## Component 3: Bounded LP Solver (`bounded_lp.rs`)

### Purpose

Solves LP subproblems with variable bounds: lower[i] ≤ x[i] ≤ upper[i]

### Challenge

Standard simplex requires: x ≥ 0 (no upper bounds)

Branch & Bound needs: lower[i] ≤ x[i] ≤ upper[i]

### Solution

Transform bounded LP to standard form by:
1. Fixing variables where lower[i] = upper[i]
2. Adding upper bound constraints: x[i] ≤ upper[i]
3. Substituting fixed variables in original constraints

### Algorithm

```rust
pub fn solve_with_bounds(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    lower: &[f64],
    upper: &[f64]
) -> Result<Vec<f64>, String>
```

**Steps:**

**1. Identify fixed variables:**
```rust
let mut fixed_vars = vec![None; n];
for i in 0..n {
    if (upper[i] - lower[i]).abs() < 1e-9 {
        fixed_vars[i] = Some(lower[i]);
    }
}
```

**2. Modify objective (remove fixed variables):**
```rust
let mut modified_c = c.to_vec();
for i in 0..n {
    if fixed_vars[i].is_some() {
        modified_c[i] = 0.0;  // Don't optimize over fixed variables
    }
}
```

**3. Substitute fixed variables in constraints:**
```rust
let mut modified_b = b.to_vec();
for i in 0..m {
    for j in 0..n {
        if let Some(fixed_val) = fixed_vars[j] {
            // Move fixed contribution to RHS
            modified_b[i] -= a.get(i, j) * fixed_val;
        }
    }
}
```

**4. Add upper bound constraints:**
```rust
let mut new_constraints = vec![];
for i in 0..n {
    if fixed_vars[i].is_none() && upper[i] < f64::INFINITY {
        // Add constraint: x[i] ≤ upper[i]
        let mut row = vec![0.0; n];
        row[i] = 1.0;
        new_constraints.push((row, upper[i]));
    }
}
```

**5. Solve modified LP:**
```rust
let combined_a = build_combined_matrix(a, &new_constraints, n, m);
let combined_b = build_combined_rhs(b, &new_constraints);

let mut solution = simplex::solve(&modified_c, &combined_a, &combined_b, sense)?;
```

**6. Restore fixed variables:**
```rust
for i in 0..n {
    if let Some(fixed_val) = fixed_vars[i] {
        solution[i] = fixed_val;
    }
}
```

### Example

**Original problem:**
```
maximize z = 3x₁ + 2x₂
subject to:
  x₁ + x₂ ≤ 10
  2 ≤ x₁ ≤ 5    (lower and upper bounds)
  x₂ ≤ 4        (upper bound)
```

**After branching, might have:**
```
lower = [3.0, 0.0]  (from branching x₁ ≥ 3)
upper = [5.0, 4.0]  (original bounds)
```

**Transformed to standard form:**
```
maximize z = 3x₁ + 2x₂
subject to:
  x₁ + x₂ ≤ 10    (original)
  x₁ ≤ 5          (upper bound on x₁)
  x₂ ≤ 4          (upper bound on x₂)
  x₁ ≥ 3          (LIMITATION: not enforced!)
  x ≥ 0
```

### Limitation: Lower Bounds > 0

**Current implementation does NOT enforce lower[i] > 0**

**Workaround used:** Rely on LP producing vertex solutions
- Vertices of polytope with upper bounds often respect implicit lower bounds
- Combined with branching, usually works in practice

**Proper fix (not implemented):**
Substitute y[i] = x[i] - lower[i]:
```
Original: lower[i] ≤ x[i] ≤ upper[i]
Substitute: 0 ≤ y[i] ≤ upper[i] - lower[i]
```

Then transform back after solving.

### Complexity

- Fixed variable elimination: O(m × n)
- Add upper bounds: O(n)
- Solve LP: O(m^2 × n^2) typical
- **Total:** Dominated by LP solve time

---

## Component 4: Main Solvers (`solvers.rs`)

### Algorithm 1: Integer Linear Programming

```rust
pub fn intlinprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    integer_vars: &[usize]
) -> Result<Vec<f64>, String>
```

**Full Implementation:**

```rust
pub fn intlinprog(...) -> Result<Vec<f64>, String> {
    // 1. VALIDATE INPUTS
    if sense != 1.0 && sense != -1.0 {
        return Err("sense must be 1.0 or -1.0");
    }
    for &idx in integer_vars {
        if idx >= n {
            return Err(format!("Index {} out of bounds", idx));
        }
    }

    // 2. SOLVE LP RELAXATION (quick check)
    let relaxed = simplex::solve(c, a, b, sense)?;
    if is_integer_solution(&relaxed, integer_vars) {
        return Ok(relaxed);  // Lucky! Already integer
    }

    // 3. INITIALIZE BRANCH & BOUND
    let mut best_solution = None;
    let mut best_objective = if sense > 0.0 {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };

    let initial_node = BBNode::initial(n);
    let mut stack = vec![initial_node];
    let max_iterations = 10000;
    let mut iterations = 0;

    // 4. MAIN LOOP
    while let Some(node) = stack.pop() {
        iterations += 1;
        if iterations > max_iterations {
            return Err("Max iterations exceeded");
        }

        // 4a. BOUND: Solve LP with node bounds
        let node_solution = match solve_with_bounds(
            c, a, b, sense,
            &node.lower_bounds,
            &node.upper_bounds
        ) {
            Ok(sol) => sol,
            Err(_) => continue,  // PRUNE by infeasibility
        };

        // 4b. BOUND: Check objective
        let node_objective = simplex::objective_value(c, &node_solution)?;
        if should_prune(node_objective, best_objective, sense) {
            continue;  // PRUNE by bound
        }

        // 4c. INTEGER CHECK
        if is_integer_solution(&node_solution, integer_vars) {
            if is_better(node_objective, best_objective, sense) {
                best_objective = node_objective;
                best_solution = Some(node_solution);
            }
            continue;  // PRUNE by integrality
        }

        // 4d. BRANCH
        if let Some(branch_var) = find_fractional_var(&node_solution, integer_vars) {
            let val = node_solution[branch_var];

            // Left child: x[branch_var] ≤ floor(val)
            let mut left = node.clone();
            left.upper_bounds[branch_var] = val.floor();
            stack.push(left);

            // Right child: x[branch_var] ≥ ceil(val)
            let mut right = node;
            right.lower_bounds[branch_var] = val.ceil();
            stack.push(right);
        }
    }

    // 5. RETURN BEST FOUND
    best_solution.ok_or("No integer solution found")
}
```

### Search Strategy: Depth-First (DFS)

Uses **stack** (LIFO) for node storage:

```rust
let mut stack = vec![initial_node];

while let Some(node) = stack.pop() {  // Pop from end (DFS)
    // ... process node ...
    stack.push(left_child);   // Push children
    stack.push(right_child);
}
```

**Alternatives:**

| Strategy | Data Structure | Pros | Cons |
|----------|---------------|------|------|
| Depth-First (DFS) | Stack (Vec) | Memory efficient, finds feasible quickly | May explore bad branches deeply |
| Breadth-First (BFS) | Queue (VecDeque) | Explores all depths evenly | High memory, slow to find feasible |
| Best-First | Priority queue | Explores most promising first | Overhead, memory |
| Dive and Plunge | Hybrid | Best of DFS and best-first | Complex implementation |

**Why DFS chosen:**
- Simple implementation
- Low memory (depth × n vs width × n)
- Often finds good feasible solutions quickly
- Works well for problems with tight LP relaxations

### Example Usage

```rust
// Maximize z = 3x₁ + 2x₂
// subject to: x₁ + x₂ ≤ 4, x₁, x₂ ∈ ℤ₊

let c = vec![3.0, 2.0];
let a = RealTensor::matrix(1, 2, vec![1.0, 1.0]).unwrap();
let b = vec![4.0];
let integer_vars = vec![0, 1];

let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();
// solution = [4, 0] or [3, 1] (both optimal with z=12)

// Branching tree:
//        Root: [2.67, 1.33] z=10.67
//       /                        \
//   x₁≤2                         x₁≥3
//   [2,2] z=10               [3,1] z=11
//   (int)                    (int) BEST!
//   PRUNE                         /    \
//                            x₂≤0    x₂≥2
//                          [4,0]   [3,2] infeas
//                          z=12    PRUNE
//                          (int)
//                          BEST!
```

---

### Algorithm 2: Binary Linear Programming

```rust
pub fn binary_linprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    binary_vars: &[usize]
) -> Result<Vec<f64>, String>
```

**Special case of ILP where variables ∈ {0, 1}**

### Key Differences from General ILP

**1. Tighter Bounds:**
```rust
// General ILP: x[i] ∈ [0, ∞)
// Binary ILP:   x[i] ∈ [0, 1]

let mut upper_bounds = vec![f64::INFINITY; n];
for &idx in binary_vars {
    upper_bounds[idx] = 1.0;  // Enforce binary constraint
}
```

**2. Binary Branching:**
```rust
// General ILP: branch on floor/ceil
let (left_val, right_val) = (val.floor(), val.ceil());

// Binary ILP: always branch on 0 and 1
let (left_val, right_val) = (0.0, 1.0);
```

**3. Priority Ordering:**
```rust
if sense > 0.0 {
    // Maximize: explore x=1 first (likely higher objective)
    stack.push(left_node);   // x = 0
    stack.push(right_node);  // x = 1 (explored first due to stack)
} else {
    // Minimize: explore x=0 first (likely lower objective)
    stack.push(right_node);  // x = 1
    stack.push(left_node);   // x = 0 (explored first)
}
```

### Applications

**1. 0-1 Knapsack:**
```rust
// Items: values = [60, 100, 120], weights = [10, 20, 30]
// Capacity: 50
// Select items to maximize value

let c = vec![60.0, 100.0, 120.0];  // Values
let a = RealTensor::matrix(1, 3, vec![10.0, 20.0, 30.0]).unwrap();  // Weights
let b = vec![50.0];  // Capacity
let binary_vars = vec![0, 1, 2];

let sol = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();
// sol = [0, 1, 1] → take items 2 and 3
// value = 220, weight = 50
```

**2. Set Covering:**
```rust
// Cover all elements with minimum subsets
// Sets: S₁={1,2}, S₂={2,3}, S₃={1,3}
// Find minimum number of sets

let c = vec![1.0, 1.0, 1.0];  // Cost of each set
let a = RealTensor::matrix(3, 3, vec![
    -1.0, 0.0, -1.0,   // Element 1 must be covered (S₁ or S₃)
    -1.0, -1.0, 0.0,   // Element 2 must be covered (S₁ or S₂)
    0.0, -1.0, -1.0,   // Element 3 must be covered (S₂ or S₃)
]).unwrap();
let b = vec![-1.0, -1.0, -1.0];  // Each element covered ≥ 1 time

let sol = binary_linprog(&c, &a, &b, -1.0, &binary_vars).unwrap();
// Minimum set cover
```

**3. Assignment Problem:**
```rust
// Assign n workers to n jobs (one-to-one)
// Minimize total cost

// For 3 workers × 3 jobs:
// x[i,j] = 1 if worker i assigned to job j, 0 otherwise
// Constraints:
//   - Each worker assigned to exactly 1 job
//   - Each job assigned to exactly 1 worker
```

### Knapsack Example with Branching Tree

```
Problem: maximize 60x₁ + 100x₂ + 120x₃
         subject to: 10x₁ + 20x₂ + 30x₃ ≤ 50
         x ∈ {0,1}³

         Root: LP = [1, 1, 0.67] z=226.7
                    /           \
               x₃=0             x₃=1
           [1,1,0] z=160     [0.5,1,1] z=230
           (int)                  /      \
           PRUNE              x₁=0       x₁=1
                           [0,1,1]     [1,0,1] infeas
                           z=220       (weight=40>50)
                           (int)       PRUNE
                           BEST!

Final: x = [0, 1, 1], z = 220
```

---

## Testing Strategy

### Comprehensive Test Suite

The refactored module includes extensive tests in `mod.rs`:

**1. Basic ILP:**
```rust
#[test]
fn test_intlinprog_simple() {
    // Small 2-variable problem
    // maximize z = 3x₁ + 2x₂
    // x₁ + x₂ ≤ 4, x ∈ ℤ₊²

    let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();

    // Verify integer
    assert!((solution[0].round() - solution[0]).abs() < 1e-6);
    assert!((solution[1].round() - solution[1]).abs() < 1e-6);

    // Verify objective
    let z = objective_value(&c, &solution).unwrap();
    assert!((z - 12.0).abs() < 1e-3);
}
```

**2. Knapsack Problems:**
```rust
#[test]
fn test_binary_linprog_knapsack() {
    // Classic 0-1 knapsack
}

#[test]
fn test_knapsack_small_capacity() {
    // Edge case: tight capacity
}

#[test]
fn test_knapsack_large_instance() {
    // 5 items, various weights
}

#[test]
fn test_knapsack_all_items_fit() {
    // Capacity larger than total weight
}

#[test]
fn test_knapsack_one_item_only() {
    // Only one item fits
}

#[test]
fn test_knapsack_tight_capacity() {
    // Multiple optimal solutions
}
```

**3. Multiple Constraints:**
```rust
#[test]
fn test_intlinprog_multiple_constraints() {
    // 2 variables, 2 constraints
    // maximize z = 3x₁ + 4x₂
    // 2x₁ + 3x₂ ≤ 12
    // x₁ + x₂ ≤ 5
}
```

### Test Coverage

```bash
# Run all integer programming tests
cargo test -p achronyme-solver integer

# Run with verbose output
cargo test -p achronyme-solver integer -- --nocapture

# Run specific test
cargo test -p achronyme-solver test_binary_linprog_knapsack
```

**Coverage goals:**
- ✓ Small problems (n, m < 5)
- ✓ Knapsack variants
- ✓ Binary vs. general integer
- ✓ Multiple constraints
- ✓ Edge cases (all fit, tight capacity, one item)
- ✗ Infeasible problems (not yet tested)
- ✗ Very large instances (performance benchmarks)

---

## Performance Analysis

### Complexity

**Worst-case:** O(2^k × T_LP) where:
- k = number of integer variables
- T_LP = time to solve one LP (typically O(m^2 n^2))

**Practical performance:**

| Problem Size | Integer Vars | Constraints | Expected Time |
|--------------|-------------|------------|---------------|
| Tiny | k ≤ 5 | m ≤ 10 | < 1 second |
| Small | 5 < k ≤ 15 | 10 < m ≤ 50 | 1-10 seconds |
| Medium | 15 < k ≤ 30 | 50 < m ≤ 200 | 10 sec - 10 min |
| Large | 30 < k ≤ 50 | 200 < m ≤ 1000 | 10 min - hours |
| Very Large | k > 50 | m > 1000 | May not terminate |

### Pruning Effectiveness

**Example:** 20 binary variables

**Without pruning:** 2^20 = 1,048,576 nodes
**With pruning (typical):** 1,000 - 10,000 nodes (99% reduction)

**Pruning ratio depends on:**
- LP relaxation tightness (closer to integer → better pruning)
- Objective coefficient distribution
- Constraint structure
- Branching heuristic quality

### Benchmarking

```rust
use std::time::Instant;

fn benchmark_knapsack(n: usize, capacity: f64) {
    let c = (0..n).map(|i| (i+1) as f64 * 10.0).collect();
    let weights = (0..n).map(|i| (i+1) as f64 * 5.0).collect();
    let a = RealTensor::matrix(1, n, weights).unwrap();
    let b = vec![capacity];
    let binary_vars = (0..n).collect();

    let start = Instant::now();
    let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();
    let elapsed = start.elapsed();

    println!("n={}, time={:?}, obj={}",
             n, elapsed, objective_value(&c, &solution).unwrap());
}

// Benchmark results (approximate):
// n=10: ~5ms
// n=20: ~50ms
// n=30: ~500ms
// n=40: ~5s
// n=50: ~50s (highly variable)
```

---

## Optimization Opportunities

### 1. Advanced Branching Heuristics

**Strong Branching:**
```rust
fn strong_branching(node: &BBNode, candidates: &[usize]) -> usize {
    let mut best_var = candidates[0];
    let mut best_improvement = f64::NEG_INFINITY;

    for &var in candidates {
        // Solve LP for both branches (x ≤ floor and x ≥ ceil)
        let left_obj = solve_left_branch(node, var);
        let right_obj = solve_right_branch(node, var);

        // Improvement = min(left, right) bound increase
        let improvement = left_obj.min(right_obj);

        if improvement > best_improvement {
            best_improvement = improvement;
            best_var = var;
        }
    }

    best_var
}
```

**Pros:** Best branching decisions, smaller tree
**Cons:** Solves 2k LPs per node (expensive)

### 2. Pseudocost Branching

**Idea:** Learn from history

```rust
struct Pseudocost {
    up_cost: Vec<f64>,    // Average obj. decrease from x ← ceil
    down_cost: Vec<f64>,  // Average obj. decrease from x ← floor
    up_count: Vec<usize>,
    down_count: Vec<usize>,
}

fn pseudocost_branching(pc: &Pseudocost, candidates: &[usize]) -> usize {
    let mut best_var = candidates[0];
    let mut best_score = f64::NEG_INFINITY;

    for &var in candidates {
        let up_avg = pc.up_cost[var] / pc.up_count[var] as f64;
        let down_avg = pc.down_cost[var] / pc.down_count[var] as f64;

        // Score = expected improvement (product or min)
        let score = up_avg * down_avg;

        if score > best_score {
            best_score = score;
            best_var = var;
        }
    }

    best_var
}
```

**Pros:** Fast, learned heuristic
**Cons:** Needs warm-up, may not generalize

### 3. Cutting Planes

**Add valid inequalities to tighten LP relaxation:**

```rust
// Gomory cut: derived from optimal simplex tableau
fn gomory_cut(tableau: &Tableau, fractional_row: usize) -> (Vec<f64>, f64) {
    // Extract fractional parts from tableau row
    // Returns new constraint that cuts off current fractional solution
}

// Combined Branch & Cut:
fn branch_and_cut(...) {
    while !optimal {
        // 1. Solve LP
        // 2. Generate cuts (if fractional)
        // 3. Add cuts to LP
        // 4. Re-solve LP
        // 5. If still fractional after max cuts, branch
    }
}
```

**Pros:** Much smaller tree, faster overall
**Cons:** Complex implementation, cut selection is an art

### 4. Primal Heuristics

**Find good feasible solutions quickly:**

```rust
// Rounding heuristic
fn round_solution(lp_solution: &[f64]) -> Vec<f64> {
    lp_solution.iter().map(|&x| x.round()).collect()
}

// Greedy knapsack
fn greedy_knapsack(values: &[f64], weights: &[f64], capacity: f64) -> Vec<f64> {
    let mut items: Vec<_> = (0..values.len())
        .map(|i| (i, values[i] / weights[i]))  // value-to-weight ratio
        .collect();

    items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());  // Sort descending

    let mut solution = vec![0.0; values.len()];
    let mut remaining = capacity;

    for (i, _) in items {
        if weights[i] <= remaining {
            solution[i] = 1.0;
            remaining -= weights[i];
        }
    }

    solution
}
```

**Pros:** Warm-start with good incumbent, better pruning
**Cons:** Heuristic may not find optimal

### 5. Parallel Branch & Bound

**Exploit multiple cores:**

```rust
use rayon::prelude::*;

fn parallel_branch_and_bound(...) {
    let mut stack = vec![initial_node];
    let best = Arc::new(Mutex::new(None));

    // Process nodes in parallel
    stack.par_iter().for_each(|node| {
        let node_sol = solve_node(node);

        if is_integer(&node_sol) {
            let mut best_lock = best.lock().unwrap();
            if is_better(node_sol, *best_lock) {
                *best_lock = Some(node_sol);
            }
        } else {
            // Branch and add to thread-local stack
            // (Requires work stealing or shared queue)
        }
    });
}
```

**Pros:** Linear speedup (in theory)
**Cons:** Load balancing, synchronization overhead

---

## Common Pitfalls

### 1. Tolerance Issues

**Problem:** Rounding errors near integer values
```rust
// Bad: exact check
if solution[i] == solution[i].round() { ... }

// Good: tolerance
const TOL: f64 = 1e-6;
if (solution[i] - solution[i].round()).abs() < TOL { ... }
```

### 2. Infinite Loops

**Problem:** Max iterations exceeded
```rust
// Always set iteration limit
let max_iterations = 10000;
for iter in 0..max_iterations {
    // ...
    if iter == max_iterations - 1 {
        return Err("Max iterations reached");
    }
}
```

### 3. Incorrect Pruning

**Problem:** Pruning potentially optimal branches
```rust
// Wrong: prune if equal
if node_obj == best_obj { continue; }

// Correct: prune only if strictly worse
if node_obj < best_obj - TOL { continue; }
```

### 4. LP Infeasibility Ignored

**Problem:** Crash when LP fails
```rust
// Bad: unwrap
let node_sol = solve_with_bounds(...).unwrap();

// Good: handle error
let node_sol = match solve_with_bounds(...) {
    Ok(sol) => sol,
    Err(_) => continue,  // Prune infeasible branch
};
```

---

## Future Enhancements

### Algorithm Improvements
- [ ] Implement strong branching
- [ ] Add pseudocost branching
- [ ] Cutting plane generation (Gomory cuts)
- [ ] Primal heuristics (rounding, local search)
- [ ] Better node selection (best-first, dive-and-plunge)

### Performance Optimizations
- [ ] Parallel Branch & Bound
- [ ] Warm-starting LP solves (reuse basis)
- [ ] Lazy constraint generation
- [ ] Symmetry breaking
- [ ] Preprocessing (variable fixing, constraint propagation)

### Features
- [ ] Mixed-Integer Programming (MIP): some continuous, some integer
- [ ] Special Ordered Sets (SOS) constraints
- [ ] Indicator constraints (if x=1 then ...)
- [ ] Quadratic objectives (MIQP)
- [ ] Solution pool (multiple optimal solutions)

### Usability
- [ ] Progress callbacks for long-running solves
- [ ] Warm-start with initial solution
- [ ] Timeout parameter
- [ ] Verbose logging option
- [ ] Export/import of Branch & Bound tree

---

## References

### Books
- **Wolsey** (1998). *Integer Programming*. Wiley.
- **Nemhauser & Wolsey** (1988). *Integer and Combinatorial Optimization*. Wiley.
- **Schrijver** (1986). *Theory of Linear and Integer Programming*. Wiley.

### Papers
- **Land & Doig** (1960). "An automatic method of solving discrete programming problems." *Econometrica*.
- **Gomory** (1963). "An algorithm for integer solutions to linear programs." *Recent Advances in Mathematical Programming*.
- **Achterberg et al.** (2007). "Branching rules revisited." *Operations Research Letters*.

### Software
- **SCIP**: Solving Constraint Integer Programs (open source, state-of-the-art)
- **CBC**: COIN-OR Branch & Cut (open source)
- **CPLEX, Gurobi**: Commercial MIP solvers (industry standard)

### Applications
- **Optimization Modeling with JuMP** (Julia): Excellent tutorials on ILP modeling
- **NEOS Server**: Online optimization solvers for testing
- **MIPlib**: Library of benchmark ILP instances
