/**
 * Linear Programming and Optimization Operations
 *
 * Provides methods for:
 * - Linear Programming (Simplex, Dual Simplex, Revised Simplex, Two-Phase)
 * - Integer Programming (Branch & Bound for Integer and Binary variables)
 * - Sensitivity Analysis (Shadow Prices, Parameter Ranges)
 * - Objective Value Calculation
 */

import type { RustWASM } from '../core/RustBindings';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

export class OptimizationOps {
  constructor(
    private bindings: RustWASM,
    private session: AchronymeSession
  ) {}

  // ============================================================================
  // Linear Programming Solvers
  // ============================================================================

  /**
   * Solve linear programming problem using Simplex method
   *
   * Solves: maximize/minimize z = c·x
   *         subject to: A·x ≤ b, x ≥ 0
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to solution vector x
   *
   * @example
   * ```typescript
   * // maximize z = 3x₁ + 5x₂
   * // subject to: x₁ ≤ 4, 2x₂ ≤ 12, 3x₁ + 2x₂ ≤ 18
   * const c = session.use(ctx => ctx.eval('[3, 5]'));
   * const A = session.use(ctx => ctx.eval('[[1, 0], [0, 2], [3, 2]]'));
   * const b = session.use(ctx => ctx.eval('[4, 12, 18]'));
   *
   * const solution = optimization.simplex(c, A, b, 1);
   * const z = optimization.objectiveValue(c, solution);
   * console.log('Optimal value:', z); // 36
   * ```
   */
  simplex(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.simplex(c, A, b, sense);
  }

  /**
   * Solve linear programming problem with automatic method selection
   *
   * Automatically chooses the best LP solver based on problem characteristics
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to solution vector x
   *
   * @example
   * ```typescript
   * // Production planning problem
   * const c = session.use(ctx => ctx.eval('[40, 30]')); // profits
   * const A = session.use(ctx => ctx.eval('[[1, 0], [0, 1], [1, 1]]'));
   * const b = session.use(ctx => ctx.eval('[40, 50, 70]')); // resources
   *
   * const solution = optimization.linprog(c, A, b, 1);
   * const profit = optimization.objectiveValue(c, solution);
   * console.log('Maximum profit:', profit); // 2500
   * ```
   */
  linprog(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.linprog(c, A, b, sense);
  }

  /**
   * Solve linear programming problem using Dual Simplex method
   *
   * Useful for sensitivity analysis and problems starting from dual feasibility
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to solution vector x
   *
   * @example
   * ```typescript
   * const solution = optimization.dualSimplex(c, A, b, 1);
   * ```
   */
  dualSimplex(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.dualSimplex(c, A, b, sense);
  }

  /**
   * Solve linear programming problem using Two-Phase Simplex method
   *
   * Handles problems with:
   * - Equality constraints (=)
   * - Greater-than-or-equal constraints (≥)
   * - Negative RHS values
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to solution vector x
   *
   * @example
   * ```typescript
   * // Problem with equality constraint
   * const c = session.use(ctx => ctx.eval('[2, 3]'));
   * const A = session.use(ctx => ctx.eval('[[1, 1], [2, 1]]'));
   * const b = session.use(ctx => ctx.eval('[5, 4]'));
   *
   * const solution = optimization.twoPhaseSimplex(c, A, b, -1);
   * ```
   */
  twoPhaseSimplex(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.twoPhaseSimplex(c, A, b, sense);
  }

  /**
   * Solve linear programming problem using Revised Simplex method
   *
   * Memory-efficient method for large-scale problems (n > 1000)
   * Stores only basis inverse instead of full tableau
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to solution vector x
   *
   * @example
   * ```typescript
   * // For large problems
   * const solution = optimization.revisedSimplex(c, A, b, 1);
   * ```
   */
  revisedSimplex(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.revisedSimplex(c, A, b, sense);
  }

  // ============================================================================
  // Objective Value Calculation
  // ============================================================================

  /**
   * Calculate objective value c·x
   *
   * Computes the dot product of objective coefficients and solution
   *
   * @param c - Handle to objective coefficients vector
   * @param x - Handle to solution vector
   * @returns The objective value z = c·x
   *
   * @example
   * ```typescript
   * const c = session.use(ctx => ctx.eval('[3, 5]'));
   * const x = session.use(ctx => ctx.eval('[2, 6]'));
   * const z = optimization.objectiveValue(c, x);
   * console.log(z); // 36 = 3*2 + 5*6
   * ```
   */
  objectiveValue(c: Handle, x: Handle): number {
    return this.bindings.objectiveValue(c, x);
  }

  // ============================================================================
  // Sensitivity Analysis
  // ============================================================================

  /**
   * Compute shadow prices (dual variables)
   *
   * Shadow prices indicate the marginal value of each resource:
   * - How much the objective improves per unit increase in constraint i
   * - If shadow_price[i] = 0, resource i is not binding (surplus exists)
   * - If shadow_price[i] > 0, resource i is fully utilized and valuable
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @returns Handle to vector of shadow prices (one per constraint)
   *
   * @example
   * ```typescript
   * // maximize z = 40x₁ + 30x₂
   * // subject to: x₁ ≤ 40 (material A), x₂ ≤ 50 (material B), x₁+x₂ ≤ 70 (hours)
   * const c = session.use(ctx => ctx.eval('[40, 30]'));
   * const A = session.use(ctx => ctx.eval('[[1, 0], [0, 1], [1, 1]]'));
   * const b = session.use(ctx => ctx.eval('[40, 50, 70]'));
   *
   * const shadows = optimization.shadowPrice(c, A, b, 1);
   * // shadows[0] = $10 per unit of material A
   * // shadows[1] = $0 (material B not binding)
   * // shadows[2] = $30 per hour of labor
   * ```
   */
  shadowPrice(c: Handle, A: Handle, b: Handle, sense: number): Handle {
    return this.bindings.shadowPrice(c, A, b, sense);
  }

  /**
   * Sensitivity analysis for objective coefficient c[index]
   *
   * Determines the range [c_min, c_max] within which c[index] can vary
   * without changing the optimal basis (solution structure remains the same)
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param index - Index of coefficient to analyze (0-based)
   * @returns Handle to vector [c_min, c_max]
   *
   * @example
   * ```typescript
   * // How much can the profit coefficient for product 1 vary?
   * const c = session.use(ctx => ctx.eval('[40, 30]'));
   * const A = session.use(ctx => ctx.eval('[[1, 0], [0, 1], [1, 1]]'));
   * const b = session.use(ctx => ctx.eval('[40, 50, 70]'));
   *
   * const range = optimization.sensitivityC(c, A, b, 0);
   * // range = [20, 80] means c[0] can vary between $20 and $80
   * // without changing which products to produce
   * ```
   */
  sensitivityC(c: Handle, A: Handle, b: Handle, index: number): Handle {
    return this.bindings.sensitivityC(c, A, b, index);
  }

  /**
   * Sensitivity analysis for RHS constraint b[index]
   *
   * Determines the range [b_min, b_max] within which b[index] can vary
   * without changing the optimal basis
   * Within this range, the shadow price remains valid
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param index - Index of constraint to analyze (0-based)
   * @returns Handle to vector [b_min, b_max]
   *
   * @example
   * ```typescript
   * // How much can available labor hours vary?
   * const c = session.use(ctx => ctx.eval('[40, 30]'));
   * const A = session.use(ctx => ctx.eval('[[1, 0], [0, 1], [1, 1]]'));
   * const b = session.use(ctx => ctx.eval('[40, 50, 70]'));
   *
   * const range = optimization.sensitivityB(c, A, b, 2);
   * // range = [35, 105] means hours can vary between 35 and 105
   * // and the shadow price remains $30/hour
   * ```
   */
  sensitivityB(c: Handle, A: Handle, b: Handle, index: number): Handle {
    return this.bindings.sensitivityB(c, A, b, index);
  }

  // ============================================================================
  // Integer Programming
  // ============================================================================

  /**
   * Integer Linear Programming using Branch & Bound
   *
   * Solves LP problems where specified variables must take integer values.
   * Uses Branch & Bound algorithm with LP relaxations.
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @param integerVars - Handle to vector of variable indices that must be integer (0-based)
   * @returns Handle to integer solution vector x
   *
   * @remarks
   * Algorithm: Branch & Bound
   * 1. Solve LP relaxation (without integer constraints)
   * 2. If solution is integer → done!
   * 3. Otherwise, branch on fractional variable
   * 4. Bound: prune branches worse than best integer solution found
   *
   * @example
   * ```typescript
   * // maximize z = 3x₁ + 2x₂
   * // subject to: x₁ + x₂ ≤ 4, x₁, x₂ ∈ ℤ₊
   * const c = session.use(ctx => ctx.eval('[3, 2]'));
   * const A = session.use(ctx => ctx.eval('[[1, 1]]'));
   * const b = session.use(ctx => ctx.eval('[4]'));
   * const intVars = session.use(ctx => ctx.eval('[0, 1]')); // Both variables must be integer
   *
   * const solution = optimization.intlinprog(c, A, b, 1, intVars);
   * const z = optimization.objectiveValue(c, solution);
   * // solution = [4, 0], z* = 12
   * ```
   */
  intlinprog(c: Handle, A: Handle, b: Handle, sense: number, integerVars: Handle): Handle {
    return this.bindings.intlinprog(c, A, b, sense, integerVars);
  }

  /**
   * Binary Linear Programming (0-1 Integer Programming)
   *
   * Solves LP problems where specified variables must be binary (0 or 1).
   * Optimized for binary variables using specialized Branch & Bound.
   *
   * @param c - Handle to objective coefficients vector
   * @param A - Handle to constraint matrix
   * @param b - Handle to RHS vector
   * @param sense - 1 for maximize, -1 for minimize
   * @param binaryVars - Handle to vector of variable indices that must be binary (0-based)
   * @returns Handle to binary solution vector x (all values are 0 or 1)
   *
   * @remarks
   * Common Applications:
   * - **0-1 Knapsack**: Select items to maximize value within weight limit
   * - **Set Covering**: Select minimum sets to cover all elements
   * - **Set Packing**: Select maximum non-overlapping sets
   * - **Assignment**: Assign tasks to workers (one-to-one)
   * - **Facility Location**: Choose which facilities to open
   * - **Capital Budgeting**: Select projects subject to budget
   * - **Scheduling**: Binary decisions for time slots
   *
   * @example
   * ```typescript
   * // 0-1 Knapsack Problem
   * // maximize z = 60x₁ + 100x₂ + 120x₃
   * // subject to: 10x₁ + 20x₂ + 30x₃ ≤ 50, xᵢ ∈ {0,1}
   *
   * const values = session.use(ctx => ctx.eval('[60, 100, 120]'));
   * const weights = session.use(ctx => ctx.eval('[[10, 20, 30]]'));
   * const capacity = session.use(ctx => ctx.eval('[50]'));
   * const binVars = session.use(ctx => ctx.eval('[0, 1, 2]')); // All variables are binary
   *
   * const solution = optimization.binaryLinprog(values, weights, capacity, 1, binVars);
   * const totalValue = optimization.objectiveValue(values, solution);
   * // solution = [0, 1, 1] → take items 2 and 3
   * // totalValue = 220 (weight = 50)
   * ```
   *
   * @example
   * ```typescript
   * // Assignment Problem: Assign 3 workers to 3 tasks
   * // Minimize cost while ensuring each worker gets exactly one task
   *
   * // Cost matrix (flattened): worker i to task j
   * const costs = session.use(ctx => ctx.eval('[9, 2, 7, 6, 4, 3, 5, 8, 1]'));
   *
   * // Constraints: each worker assigned once, each task assigned once
   * const A = session.use(ctx => ctx.eval(`[
   *   [1, 1, 1, 0, 0, 0, 0, 0, 0],  // worker 1
   *   [0, 0, 0, 1, 1, 1, 0, 0, 0],  // worker 2
   *   [0, 0, 0, 0, 0, 0, 1, 1, 1],  // worker 3
   *   [1, 0, 0, 1, 0, 0, 1, 0, 0],  // task 1
   *   [0, 1, 0, 0, 1, 0, 0, 1, 0],  // task 2
   *   [0, 0, 1, 0, 0, 1, 0, 0, 1]   // task 3
   * ]`));
   * const b = session.use(ctx => ctx.eval('[1, 1, 1, 1, 1, 1]'));
   * const binVars = session.use(ctx => ctx.eval('[0, 1, 2, 3, 4, 5, 6, 7, 8]'));
   *
   * const assignment = optimization.binaryLinprog(costs, A, b, -1, binVars);
   * // assignment = [0, 1, 0, 1, 0, 0, 0, 0, 1]
   * // Worker 1 → Task 2 (cost=2), Worker 2 → Task 1 (cost=6), Worker 3 → Task 3 (cost=1)
   * // Total cost = 9
   * ```
   */
  binaryLinprog(c: Handle, A: Handle, b: Handle, sense: number, binaryVars: Handle): Handle {
    return this.bindings.binaryLinprog(c, A, b, sense, binaryVars);
  }
}
