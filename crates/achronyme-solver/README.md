# achronyme-solver

Mathematical optimization and linear programming solver for the Achronyme project.

## Overview

The `achronyme-solver` crate provides production-ready implementations of optimization algorithms for solving Linear Programming (LP) and Integer Linear Programming (ILP) problems. This crate is a core component of the Achronyme computational stack, enabling users to solve real-world optimization problems in areas such as operations research, resource allocation, production planning, and decision analysis.

## Position in Architecture

```
User Code (SOC)
      ↓
achronyme-parser  →  AST
      ↓
achronyme-eval    →  Calls solver functions (linprog, simplex, intlinprog)
      ↓
achronyme-solver  →  Optimization algorithms (THIS CRATE)
      ↓
achronyme-types   →  Tensor, Value, etc.
```

**Dependencies:**
- `achronyme-types`: For `RealTensor` (matrix representation) and type definitions

**Used by:**
- `achronyme-eval`: Evaluates optimization function calls from SOC code
  - `simplex()` → `simplex_solve()`
  - `linprog()` → `linprog_solve()`
  - `intlinprog()` → `intlinprog()`
  - `binary_linprog()` → `binary_linprog()`
  - `shadow_price()` → `shadow_price()`
  - `sensitivity_b()`, `sensitivity_c()` → Sensitivity analysis

## Algorithms Implemented

### Linear Programming (LP)

**1. Primal Simplex Method** (`linear::simplex`)
- **Complexity**: O(2^n) worst-case, O(m × n) typical
- **Use case**: General LP problems with few constraints
- **Algorithm**: Iteratively moves along edges of feasible polytope to optimal vertex
- **Standard form**:
  ```
  maximize/minimize  z = c^T x
  subject to:        A x ≤ b
                     x ≥ 0
  ```

**2. Dual Simplex Method** (`linear::dual_simplex`)
- **Complexity**: O(2^n) worst-case, O(m × n) typical
- **Use case**: Problems with dual feasible starting point, adding constraints to solved problems
- **Algorithm**: Maintains dual feasibility, seeks primal feasibility
- **Advantage**: Useful when adding constraints post-optimization

**3. Two-Phase Simplex** (`linear::two_phase`)
- **Complexity**: 2 × O(m × n) for two phases
- **Use case**: Problems without obvious initial basic feasible solution (BFS)
- **Algorithm**:
  - **Phase 1**: Finds initial BFS using artificial variables
  - **Phase 2**: Solves original problem from Phase 1 BFS
- **Handles**: Equality constraints, ≥ constraints, negative RHS

**4. Revised Simplex Method** (`linear::revised_simplex`)
- **Complexity**: O(m^3 + m^2 × n) per iteration
- **Use case**: Problems with many variables but few constraints (n >> m)
- **Memory**: O(m^2) vs O(m × n) for standard simplex
- **Algorithm**: Maintains inverse basis matrix B^(-1), recalculates tableau columns on-demand
- **Advantage**: More efficient for sparse, large-scale problems

**5. Sensitivity Analysis** (`linear::sensitivity`)
- **Shadow Prices**: Marginal value of resources (∂z*/∂b_i)
- **Coefficient Ranging**: Valid range for objective coefficients (c_i)
- **RHS Ranging**: Valid range for constraint RHS (b_i)
- **Use case**: Post-optimality analysis, what-if scenarios

### Integer Programming (ILP)

**6. Branch & Bound for ILP** (`integer::branch_bound`)
- **Complexity**: O(2^k) where k = number of integer variables (NP-hard)
- **Use case**: Optimization with discrete decision variables
- **Algorithm**:
  1. Solve LP relaxation
  2. If integer → done
  3. Branch on fractional variable: x_i ≤ ⌊x_i⌋ and x_i ≥ ⌈x_i⌉
  4. Bound: prune branches worse than best integer solution
- **Heuristic**: "Most fractional" variable selection (closest to 0.5)
- **Applications**: Knapsack, assignment, scheduling, facility location

**7. Binary Linear Programming** (`integer::branch_bound::binary_linprog`)
- **Complexity**: O(2^n) (special case of ILP)
- **Use case**: Binary decision problems (yes/no, on/off, include/exclude)
- **Constraints**: x_i ∈ {0, 1}
- **Applications**: 0-1 Knapsack, set covering, job scheduling

## Module Structure

```
achronyme-solver/
├── src/
│   ├── lib.rs                    # Public API exports
│   ├── linear/                   # Linear Programming
│   │   ├── mod.rs
│   │   ├── tableau.rs            # Simplex tableau data structure
│   │   ├── simplex.rs            # Primal Simplex
│   │   ├── dual_simplex.rs       # Dual Simplex
│   │   ├── two_phase.rs          # Two-Phase Simplex
│   │   ├── revised_simplex.rs    # Revised Simplex
│   │   ├── sensitivity.rs        # Sensitivity analysis
│   │   └── linprog.rs            # Auto-selection wrapper
│   └── integer/                  # Integer Programming
│       ├── mod.rs
│       └── branch_bound/         # Branch & Bound (refactored)
│           ├── mod.rs
│           ├── node.rs           # BB tree node structure
│           ├── helpers.rs        # Integrality checks, heuristics
│           ├── bounded_lp.rs     # LP with variable bounds
│           └── solvers.rs        # intlinprog, binary_linprog
```

## Usage Examples

### 1. Simple Linear Program (Production Planning)

```rust
use achronyme_solver::{simplex_solve, objective_value};
use achronyme_types::tensor::RealTensor;

// Maximize profit: z = 40x₁ + 30x₂
// Subject to:
//   x₁ ≤ 40        (material A)
//   x₂ ≤ 50        (material B)
//   x₁ + x₂ ≤ 70   (labor hours)
//   x₁, x₂ ≥ 0

let c = vec![40.0, 30.0];
let a = RealTensor::matrix(3, 2, vec![
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0,
]).unwrap();
let b = vec![40.0, 50.0, 70.0];

let solution = simplex_solve(&c, &a, &b, 1.0).unwrap();
// solution = [40.0, 30.0]
// z* = 40*40 + 30*30 = 2500

let z = objective_value(&c, &solution).unwrap();
println!("Optimal production: x₁={}, x₂={}, profit={}",
         solution[0], solution[1], z);
```

### 2. Integer Linear Program (Resource Allocation)

```rust
use achronyme_solver::intlinprog;
use achronyme_types::tensor::RealTensor;

// Maximize: z = 3x₁ + 2x₂
// Subject to: x₁ + x₂ ≤ 4
// Integer constraints: x₁, x₂ ∈ ℤ₊

let c = vec![3.0, 2.0];
let a = RealTensor::matrix(1, 2, vec![1.0, 1.0]).unwrap();
let b = vec![4.0];
let integer_vars = vec![0, 1];  // Both variables must be integer

let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();
// solution = [4.0, 0.0] (integer)
// z* = 12
```

### 3. Knapsack Problem (Binary LP)

```rust
use achronyme_solver::binary_linprog;
use achronyme_types::tensor::RealTensor;

// 0-1 Knapsack: maximize value with weight constraint
// Items: values = [60, 100, 120], weights = [10, 20, 30]
// Capacity: 50

let c = vec![60.0, 100.0, 120.0];
let a = RealTensor::matrix(1, 3, vec![10.0, 20.0, 30.0]).unwrap();
let b = vec![50.0];
let binary_vars = vec![0, 1, 2];  // x_i ∈ {0, 1}

let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();
// solution = [0.0, 1.0, 1.0] (take items 2 and 3)
// z* = 220, weight = 50
```

### 4. Sensitivity Analysis

```rust
use achronyme_solver::{shadow_price, sensitivity_b};
use achronyme_types::tensor::RealTensor;

let c = vec![40.0, 30.0];
let a = RealTensor::matrix(3, 2, vec![
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0,
]).unwrap();
let b = vec![40.0, 50.0, 70.0];

// Shadow prices: value of one more unit of each resource
let shadow = shadow_price(&c, &a, &b, 1.0).unwrap();
// shadow[0] = 40 (material A is worth $40/unit)
// shadow[1] = 30 (material B is worth $30/unit)
// shadow[2] = 0  (labor not binding)

// RHS ranging: how much can we change b[i] without changing basis
let range = sensitivity_b(&c, &a, &b, 0).unwrap();
// Material A can vary in range [range[0], range[1]]
```

### 5. Two-Phase Simplex (Negative RHS)

```rust
use achronyme_solver::two_phase_solve;
use achronyme_types::tensor::RealTensor;

// Problem with negative RHS (needs artificial variables)
// Maximize: z = x₁ + x₂
// Subject to: -x₁ - x₂ ≤ -2  (i.e., x₁ + x₂ ≥ 2)
//             x₁ + x₂ ≤ 5

let c = vec![1.0, 1.0];
let a = RealTensor::matrix(2, 2, vec![
    -1.0, -1.0,
     1.0,  1.0,
]).unwrap();
let b = vec![-2.0, 5.0];  // Negative RHS

let solution = two_phase_solve(&c, &a, &b, 1.0).unwrap();
// Phase 1 finds BFS, Phase 2 optimizes
```

## Mathematical Foundations

### LP Standard Form

All LP solvers convert problems to standard form:

```
maximize    z = c^T x
subject to  A x ≤ b
            x ≥ 0
```

**Transformations:**
- Minimization: `min c^T x` → `max -c^T x`
- Equality: `a^T x = b` → `a^T x ≤ b` AND `a^T x ≥ b`
- ≥ constraint: `a^T x ≥ b` → `-a^T x ≤ -b`
- Free variable: `x_j free` → `x_j = x_j^+ - x_j^-` where `x_j^+, x_j^- ≥ 0`

### Simplex Tableau

The tableau is an augmented matrix representing the LP in canonical form:

```
     x₁  x₂  ...  xₙ | s₁  s₂  ...  sₘ | RHS
  ─────────────────────────────────────────────
  |  A matrix       |  Identity I     |  b   |  ← Constraints
  ─────────────────────────────────────────────
  | -c coefficients |  0 ... 0        |  z   |  ← Objective
```

**Pivot Operation:**
1. Select entering variable (most negative cost)
2. Select leaving variable (minimum ratio test)
3. Make pivot element = 1
4. Make pivot column = e_i (unit vector)

**Optimality:** All objective row coefficients ≥ 0

**Unbounded:** No valid leaving variable (all pivot column elements ≤ 0)

### Duality

Every LP has a dual problem:

**Primal:**
```
maximize    c^T x
subject to  A x ≤ b
            x ≥ 0
```

**Dual:**
```
minimize    b^T y
subject to  A^T y ≥ c
            y ≥ 0
```

**Strong Duality Theorem:** If primal has optimal solution x*, dual has optimal solution y* with c^T x* = b^T y*

**Shadow Prices:** Optimal dual variables y* are shadow prices (marginal resource values)

### Branch & Bound

For ILP: maximize c^T x subject to Ax ≤ b, x_i ∈ ℤ for i ∈ I

**Algorithm:**
1. **Bound:** Solve LP relaxation (continuous version)
2. **Integer Check:** If x_i ∈ ℤ for all i ∈ I → optimal integer solution
3. **Branch:** Select fractional variable x_j, create two subproblems:
   - Left: add constraint x_j ≤ ⌊x_j⌋
   - Right: add constraint x_j ≥ ⌈x_j⌉
4. **Prune:** Discard branches with bound worse than best integer solution
5. **Repeat** until all branches pruned or explored

**Pruning Rules:**
- **Bound pruning:** z_node ≤ z_best (maximization)
- **Infeasibility pruning:** Subproblem infeasible
- **Integrality pruning:** Found integer solution

**Branching Heuristics:**
- **Most fractional:** Choose x_j closest to 0.5
- **Strong branching:** Solve LP for each candidate, choose best bound improvement
- **Pseudocost:** Estimate objective degradation from historical branching

## Algorithm Complexity

| Algorithm | Time Complexity | Space | Best For |
|-----------|----------------|-------|----------|
| Primal Simplex | O(m × n) typical, O(2^n) worst | O(m × n) | General LP, m ≈ n |
| Dual Simplex | O(m × n) typical | O(m × n) | Adding constraints, reoptimization |
| Two-Phase Simplex | 2 × O(m × n) | O(m × n) | No obvious BFS, equality constraints |
| Revised Simplex | O(m^3 + m^2 n) per iter | O(m^2) | Many variables (n >> m), sparse matrices |
| Branch & Bound | O(2^k) where k = integer vars | O(k × tree depth) | ILP, NP-hard problems |
| Binary LP | O(2^n) | O(n × depth) | 0-1 decisions, knapsack |

**Notes:**
- m = number of constraints
- n = number of variables
- k = number of integer variables
- LP is polynomial in practice (interior point methods are O(n^3.5))
- ILP is NP-hard (exponential worst-case)

## Numerical Stability

The implementation uses several techniques for numerical robustness:

1. **Tolerance-based comparisons:** Uses ε = 10^(-6) to 10^(-10) for zero checks
2. **Pivot selection:** Minimum ratio test with tie-breaking to avoid degeneracy
3. **Bland's rule:** (Not yet implemented) Prevents cycling in degenerate LPs
4. **Basis reinversion:** (Revised Simplex) Periodically recompute B^(-1) to reduce rounding errors
5. **Gaussian elimination with partial pivoting:** For matrix inversion

**Known Limitations:**
- No presolve/preprocessing (simplification before solving)
- No scaling of constraints for numerical conditioning
- No advanced anti-cycling rules (Bland's rule, lexicographic ordering)
- Revised Simplex uses dense matrix inversion (not LU factorization)

## Applications

### Operations Research
- **Production planning:** Maximize profit given resource constraints
- **Transportation:** Minimize shipping cost between suppliers and customers
- **Blending:** Optimize ingredient mix (petroleum, food, chemicals)
- **Scheduling:** Assign tasks to minimize completion time

### Finance
- **Portfolio optimization:** Maximize return subject to risk constraints
- **Capital budgeting:** Select projects under budget constraint
- **Asset allocation:** Distribute investments across asset classes

### Engineering
- **Network flow:** Max flow, min cost flow problems
- **Facility location:** Place warehouses/factories to minimize cost
- **Cutting stock:** Minimize material waste in cutting operations
- **Resource allocation:** Assign limited resources to competing activities

### Combinatorial Optimization
- **Knapsack:** Select items to maximize value within weight limit
- **Assignment:** Match workers to jobs (one-to-one)
- **Set covering:** Choose minimum subsets to cover all elements
- **Bin packing:** Pack items into minimum number of bins

## Performance Characteristics

**Small problems (n, m < 100):**
- All methods perform well
- Primal Simplex is simplest choice

**Medium problems (100 < n, m < 1000):**
- Primal Simplex for balanced problems (m ≈ n)
- Dual Simplex for adding constraints
- Revised Simplex for n >> m

**Large problems (n, m > 1000):**
- Revised Simplex recommended
- Consider sparse matrix implementations
- Interior Point methods (not yet implemented) for very large scale

**Integer problems:**
- Branch & Bound efficient for k < 20 integer variables
- Use binary LP for 0-1 problems (better pruning)
- Consider heuristics/approximations for k > 30

## Testing

The crate includes comprehensive test coverage:

- **Unit tests:** Each algorithm module has dedicated tests
- **Integration tests:** End-to-end problem solving
- **Edge cases:** Unbounded, infeasible, degenerate problems
- **Classic problems:** Knapsack, production planning, transportation

Run tests:
```bash
cargo test -p achronyme-solver
```

## Future Enhancements

**Algorithms:**
- Interior Point Methods (Primal-Dual, Barrier)
- Network Simplex for network flow problems
- Cutting plane methods for ILP
- Branch & Cut (Branch & Bound + cutting planes)
- Heuristics: Greedy, local search, metaheuristics

**Optimizations:**
- Sparse matrix support (CSR/CSC format)
- LU factorization for basis updates
- Presolve: Remove redundant constraints, fix variables
- Scaling: Normalize constraints for numerical stability
- Warm start: Reoptimize from previous solution

**Features:**
- Quadratic Programming (QP)
- Mixed-Integer Linear Programming (MILP) with mixed variables
- Multi-objective optimization (Pareto frontier)
- Robust optimization (uncertain parameters)
- Stochastic programming

## References

### Books
- **Bertsimas & Tsitsiklis** (1997). *Introduction to Linear Optimization*. Athena Scientific.
- **Vanderbei** (2014). *Linear Programming: Foundations and Extensions*. Springer.
- **Wolsey** (1998). *Integer Programming*. Wiley.

### Papers
- **Dantzig** (1963). *Linear Programming and Extensions*. Princeton University Press.
- **Bland** (1977). "New finite pivoting rules for the simplex method." *Mathematics of Operations Research*.
- **Land & Doig** (1960). "An automatic method of solving discrete programming problems." *Econometrica*.

### Online Resources
- [NEOS Guide](https://neos-guide.org/): Optimization algorithms and software
- [OR-Tools Documentation](https://developers.google.com/optimization): Google's optimization library
- [COIN-OR](https://www.coin-or.org/): Open-source OR software

## License

Part of the Achronyme project. See the main repository for license information.
