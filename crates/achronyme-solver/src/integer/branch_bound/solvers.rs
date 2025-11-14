use achronyme_types::tensor::RealTensor;
use crate::linear::simplex;
use super::node::BBNode;
use super::helpers::{is_integer_solution, find_fractional_var, should_prune, is_better};
use super::bounded_lp::solve_with_bounds;

/// Integer Linear Programming using Branch & Bound
///
/// Solves LP with integer constraints on specified variables.
///
/// # Algorithm: Branch & Bound
/// 1. Solve LP relaxation (without integer constraints)
/// 2. If solution is integer → done!
/// 3. Otherwise, branch on fractional variable:
///    - Create two subproblems: xᵢ ≤ ⌊xᵢ⌋ and xᵢ ≥ ⌈xᵢ⌉
/// 4. Bound: prune branches with worse objective than best integer solution
///
/// # Parameters
/// - `c`: Objective coefficients (n elements)
/// - `a`: Constraint matrix (m × n)
/// - `b`: RHS vector (m elements)
/// - `sense`: 1.0 for maximize, -1.0 for minimize
/// - `integer_vars`: Indices of variables that must be integer
///
/// # Returns
/// - `Ok(x)`: Optimal integer solution
/// - `Err`: If problem is infeasible or unbounded
///
/// # Ejemplo
/// ```
/// // maximize z = 3x₁ + 2x₂
/// // subject to: x₁ + x₂ ≤ 4, x₁, x₂ ∈ ℤ₊
/// let c = vec![3.0, 2.0];
/// let a = RealTensor::matrix(1, 2, vec![1.0, 1.0]).unwrap();
/// let b = vec![4.0];
/// let integer_vars = vec![0, 1]; // Both variables must be integer
///
/// // let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();
/// // solution = [3.0, 1.0], z* = 11
/// ```
pub fn intlinprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    integer_vars: &[usize],
) -> Result<Vec<f64>, String> {
    // Validate inputs
    if sense != 1.0 && sense != -1.0 {
        return Err("sense must be 1.0 (maximize) or -1.0 (minimize)".to_string());
    }

    let n = c.len();
    for &idx in integer_vars {
        if idx >= n {
            return Err(format!("Integer variable index {} out of bounds (n={})", idx, n));
        }
    }

    // Solve LP relaxation
    let relaxed_solution = simplex::solve(c, a, b, sense)?;

    // Check if already integer
    if is_integer_solution(&relaxed_solution, integer_vars) {
        return Ok(relaxed_solution);
    }

    // Branch & Bound
    let mut best_solution: Option<Vec<f64>> = None;
    let mut best_objective = if sense > 0.0 { f64::NEG_INFINITY } else { f64::INFINITY };

    // Initial node: entire problem
    let initial_node = BBNode::initial(n);

    let mut stack = vec![initial_node];
    let max_iterations = 10000; // Prevent infinite loops
    let mut iterations = 0;

    while let Some(node) = stack.pop() {
        iterations += 1;
        if iterations > max_iterations {
            return Err("Branch & Bound exceeded maximum iterations".to_string());
        }

        // Solve LP with current bounds
        let node_solution = solve_with_bounds(c, a, b, sense, &node.lower_bounds, &node.upper_bounds)?;

        // Prune: check if this branch is worse than best found
        let node_objective = simplex::objective_value(c, &node_solution)?;
        if should_prune(node_objective, best_objective, sense) {
            continue; // Prune this branch
        }

        // Check if integer solution
        if is_integer_solution(&node_solution, integer_vars) {
            // Found integer solution - update best
            if is_better(node_objective, best_objective, sense) {
                best_objective = node_objective;
                best_solution = Some(node_solution);
            }
            continue;
        }

        // Branch: choose fractional variable
        if let Some(branch_var) = find_fractional_var(&node_solution, integer_vars) {
            let branch_value = node_solution[branch_var];

            // Create two child nodes
            let floor_val = branch_value.floor();
            let ceil_val = branch_value.ceil();

            // Left branch: xᵢ ≤ ⌊xᵢ⌋
            let mut left_node = node.clone();
            left_node.upper_bounds[branch_var] = floor_val;
            stack.push(left_node);

            // Right branch: xᵢ ≥ ⌈xᵢ⌉
            let mut right_node = node.clone();
            right_node.lower_bounds[branch_var] = ceil_val;
            stack.push(right_node);
        }
    }

    best_solution.ok_or_else(|| "No integer solution found".to_string())
}

/// Binary Linear Programming (special case where variables ∈ {0, 1})
///
/// # Example
/// ```
/// // Knapsack problem: maximize value with weight constraint
/// // maximize z = 60x₁ + 100x₂ + 120x₃
/// // subject to: 10x₁ + 20x₂ + 30x₃ ≤ 50, xᵢ ∈ {0,1}
/// let c = vec![60.0, 100.0, 120.0];
/// let a = RealTensor::matrix(1, 3, vec![10.0, 20.0, 30.0]).unwrap();
/// let b = vec![50.0];
/// let binary_vars = vec![0, 1, 2];
///
/// // let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();
/// ```
pub fn binary_linprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    binary_vars: &[usize],
) -> Result<Vec<f64>, String> {
    let n = c.len();

    // Add binary constraints: 0 ≤ xᵢ ≤ 1 for binary variables
    // We add these as explicit constraints to the LP
    let mut binary_upper_bounds = vec![f64::INFINITY; n];
    for &idx in binary_vars {
        binary_upper_bounds[idx] = 1.0;
    }

    // Solve with modified intlinprog that respects binary bounds
    binary_intlinprog(c, a, b, sense, binary_vars, &binary_upper_bounds)
}

/// Internal helper for binary IP with explicit upper bounds
pub fn binary_intlinprog(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    integer_vars: &[usize],
    var_upper_bounds: &[f64],
) -> Result<Vec<f64>, String> {
    let n = c.len();

    // Initial bounds: [0, upper_bound] for each variable
    let initial_lower = vec![0.0; n];
    let initial_upper = var_upper_bounds.to_vec();

    // Solve LP relaxation with binary bounds
    let relaxed_solution = solve_with_bounds(c, a, b, sense, &initial_lower, &initial_upper)?;

    // Check if already integer
    if is_integer_solution(&relaxed_solution, integer_vars) {
        return Ok(relaxed_solution);
    }

    // Branch & Bound with binary-aware bounds
    let mut best_solution: Option<Vec<f64>> = None;
    let mut best_objective = if sense > 0.0 { f64::NEG_INFINITY } else { f64::INFINITY };

    let initial_node = BBNode::new(initial_lower, initial_upper);

    let mut stack = vec![initial_node];
    let max_iterations = 50000;
    let mut iterations = 0;

    while let Some(node) = stack.pop() {
        iterations += 1;
        if iterations > max_iterations {
            // Return best found so far instead of error
            return best_solution.ok_or_else(||
                "Branch & Bound exceeded maximum iterations (no integer solution found)".to_string()
            );
        }

        // Solve LP with current bounds
        let node_solution = match solve_with_bounds(c, a, b, sense, &node.lower_bounds, &node.upper_bounds) {
            Ok(sol) => sol,
            Err(_) => continue, // Infeasible subproblem, prune
        };

        // Bound: check if this branch is worse than best found
        let node_objective = match simplex::objective_value(c, &node_solution) {
            Ok(obj) => obj,
            Err(_) => continue,
        };

        if should_prune(node_objective, best_objective, sense) {
            continue;
        }

        // Check if integer solution
        if is_integer_solution(&node_solution, integer_vars) {
            if is_better(node_objective, best_objective, sense) {
                best_objective = node_objective;
                best_solution = Some(node_solution);
            }
            continue;
        }

        // Branch on fractional variable
        if let Some(branch_var) = find_fractional_var(&node_solution, integer_vars) {
            let branch_value = node_solution[branch_var];

            // For binary variables, branch on 0 and 1
            // For general integer, branch on floor and ceil
            let (left_val, right_val) = if var_upper_bounds[branch_var] <= 1.0 {
                // Binary variable: branch on 0 and 1
                (0.0, 1.0)
            } else {
                // General integer: branch on floor and ceil
                (branch_value.floor(), branch_value.ceil())
            };

            // For maximization: explore xᵢ = 1 first (push last to stack)
            // For minimization: explore xᵢ = 0 first (push last to stack)
            if sense > 0.0 {
                // Maximization: push 0-branch first, then 1-branch (so 1 is explored first)
                let mut left_node = node.clone();
                left_node.upper_bounds[branch_var] = left_val;
                stack.push(left_node);

                let mut right_node = node;
                right_node.lower_bounds[branch_var] = right_val;
                stack.push(right_node);
            } else {
                // Minimization: push 1-branch first, then 0-branch (so 0 is explored first)
                let mut right_node = node.clone();
                right_node.lower_bounds[branch_var] = right_val;
                stack.push(right_node);

                let mut left_node = node;
                left_node.upper_bounds[branch_var] = left_val;
                stack.push(left_node);
            }
        }
    }

    best_solution.ok_or_else(|| "No integer solution found".to_string())
}
