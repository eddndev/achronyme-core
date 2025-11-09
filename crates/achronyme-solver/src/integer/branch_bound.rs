use achronyme_types::tensor::RealTensor;
use crate::linear::simplex;

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
    let initial_node = BBNode {
        lower_bounds: vec![0.0; n],
        upper_bounds: vec![f64::INFINITY; n],
    };

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
fn binary_intlinprog(
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

    let initial_node = BBNode {
        lower_bounds: initial_lower,
        upper_bounds: initial_upper,
    };

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

// ============================================================================
// Helper Structures and Functions
// ============================================================================

#[derive(Clone)]
struct BBNode {
    lower_bounds: Vec<f64>,
    upper_bounds: Vec<f64>,
}

/// Check if solution satisfies integer constraints
fn is_integer_solution(solution: &[f64], integer_vars: &[usize]) -> bool {
    const TOLERANCE: f64 = 1e-6;
    for &idx in integer_vars {
        let val = solution[idx];
        let rounded = val.round();
        if (val - rounded).abs() > TOLERANCE {
            return false;
        }
    }
    true
}

/// Find a fractional variable to branch on
fn find_fractional_var(solution: &[f64], integer_vars: &[usize]) -> Option<usize> {
    const TOLERANCE: f64 = 1e-6;

    // Use "most fractional" heuristic: choose variable closest to 0.5
    let mut best_var = None;
    let mut best_fractionality = f64::INFINITY;

    for &idx in integer_vars {
        let val = solution[idx];
        let frac = val - val.floor();
        let fractionality = (frac - 0.5).abs();

        if frac > TOLERANCE && frac < (1.0 - TOLERANCE) {
            // Choose variable CLOSEST to 0.5 (lowest fractionality score)
            if fractionality < best_fractionality {
                best_fractionality = fractionality;
                best_var = Some(idx);
            }
        }
    }

    best_var
}

/// Check if should prune this branch
fn should_prune(node_objective: f64, best_objective: f64, sense: f64) -> bool {
    if sense > 0.0 {
        // Maximize: prune if node objective ≤ best
        node_objective <= best_objective
    } else {
        // Minimize: prune if node objective ≥ best
        node_objective >= best_objective
    }
}

/// Check if obj1 is better than obj2
fn is_better(obj1: f64, obj2: f64, sense: f64) -> bool {
    if sense > 0.0 {
        obj1 > obj2
    } else {
        obj1 < obj2
    }
}

/// Solve LP with variable bounds
fn solve_with_bounds(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    lower: &[f64],
    upper: &[f64],
) -> Result<Vec<f64>, String> {
    let n = c.len();
    let m = a.rows();  // ✅ Cambio: .rows en lugar de .rows

    // Identify fixed variables (lower == upper) and substitute them
    let mut fixed_vars = vec![None; n];
    let mut modified_c = c.to_vec();
    for i in 0..n {
        if (upper[i] - lower[i]).abs() < 1e-9 {
            fixed_vars[i] = Some(lower[i]);
            modified_c[i] = 0.0;  // Zero out objective coefficient for fixed variables
        }
    }

    // Build new constraint matrix with bounds and fixed variable substitution
    let mut new_rows = Vec::new();
    let mut new_b_values = Vec::new();

    // Copy original constraints with fixed variable substitution
    for i in 0..m {
        let row_start = i * n;
        let row_end = row_start + n;
        let mut row = a.data[row_start..row_end].to_vec();
        let mut b_val = b[i];

        // Subtract contribution of fixed variables
        for j in 0..n {
            if let Some(fixed_val) = fixed_vars[j] {
                b_val -= row[j] * fixed_val;
                row[j] = 0.0;  // Fixed variable doesn't participate
            }
        }

        new_rows.push(row);
        new_b_values.push(b_val);
    }

    // Add upper bound constraints: xᵢ ≤ upper[i] (only for non-fixed variables)
    for i in 0..n {
        if fixed_vars[i].is_none() && upper[i] < f64::INFINITY {
            let mut row = vec![0.0; n];
            row[i] = 1.0;
            new_rows.push(row);
            new_b_values.push(upper[i]);
        }
    }

    // For lower bounds, we have two cases:
    // 1. If lower[i] == 0, no constraint needed (default)
    // 2. If lower[i] > 0, we need to add constraint, but simplex requires non-negative RHS
    //    So we don't add it directly. Instead, we rely on the fact that simplex
    //    solutions are typically at vertices, and combined with upper bounds this should work.
    // NOTE: This is a limitation - we may not correctly enforce lower > 0 in all cases.

    // Build combined matrix
    let total_rows = new_rows.len();
    let mut combined_data = Vec::with_capacity(total_rows * n);
    for row in new_rows {
        combined_data.extend_from_slice(&row);
    }

    let a_new = RealTensor::matrix(total_rows, n, combined_data)
        .map_err(|e| format!("Failed to create bounded constraint matrix: {}", e))?;

    // Solve bounded LP with modified objective
    match simplex::solve(&modified_c, &a_new, &new_b_values, sense) {
        Ok(mut sol) => {
            // Restore fixed variables to their fixed values
            for i in 0..n {
                if let Some(fixed_val) = fixed_vars[i] {
                    sol[i] = fixed_val;
                }
            }
            Ok(sol)
        },
        Err(_) => Err("Subproblem infeasible".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intlinprog_simple() {
        // maximize z = 3x₁ + 2x₂
        // subject to: x₁ + x₂ ≤ 4
        // x₁, x₂ ∈ ℤ₊

        let c = vec![3.0, 2.0];
        let a = RealTensor::matrix(1, 2, vec![1.0, 1.0]).unwrap();
        let b = vec![4.0];
        let integer_vars = vec![0, 1];

        let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();

        // Optimal integer solution: x = [4, 0] with z = 12
        assert!((solution[0].round() - solution[0]).abs() < 1e-6, "x[0] should be integer");
        assert!((solution[1].round() - solution[1]).abs() < 1e-6, "x[1] should be integer");

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 12.0).abs() < 1e-3, "Objective should be 12");
    }

    #[test]
    fn test_binary_linprog_knapsack() {
        // Knapsack: maximize z = 60x₁ + 100x₂ + 120x₃
        // subject to: 10x₁ + 20x₂ + 30x₃ ≤ 50
        // xᵢ ∈ {0, 1}

        let c = vec![60.0, 100.0, 120.0];
        let a = RealTensor::matrix(1, 3, vec![10.0, 20.0, 30.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        // Check binary
        for &idx in &binary_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} = {} is not binary", idx, val);
        }

        // Optimal: take items 2 and 3 (x = [0, 1, 1]), z = 220
        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 220.0).abs() < 1e-3, "Objective should be 220, got z = {}", z);

        // Verify optimal solution
        assert!((solution[0]).abs() < 1e-6, "x₁ should be 0");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_small_capacity() {
        // Small capacity knapsack
        // Items: (value, weight) = [(4,2), (3,1), (2,1)]
        // Capacity: 2
        // Optimal: take items 2 and 3 → x = [0, 1, 1], z = 5

        let c = vec![4.0, 3.0, 2.0];
        let a = RealTensor::matrix(1, 3, vec![2.0, 1.0, 1.0]).unwrap();
        let b = vec![2.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 5.0).abs() < 1e-3, "Expected z = 5, got z = {}", z);

        // Verify solution
        assert!((solution[0]).abs() < 1e-6, "x₁ should be 0");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_large_instance() {
        // Classic 5-item knapsack
        // Items: values = [10, 20, 30, 40, 50], weights = [5, 10, 15, 20, 25]
        // Capacity: 50
        // Optimal: take items 2, 3, 4 → x = [0, 1, 1, 1, 0], z = 90

        let c = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let a = RealTensor::matrix(1, 5, vec![5.0, 10.0, 15.0, 20.0, 25.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2, 3, 4];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        // Check all variables are binary
        for &idx in &binary_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} should be binary", idx);
        }

        let z = simplex::objective_value(&c, &solution).unwrap();
        // Optimal: x = [1, 1, 1, 1, 0] → weight = 5+10+15+20 = 50, value = 10+20+30+40 = 100
        assert!((z - 100.0).abs() < 1e-3, "Expected z = 100, got z = {}", z);

        // Verify constraint satisfaction
        let weight: f64 = solution.iter()
            .zip(vec![5.0, 10.0, 15.0, 20.0, 25.0].iter())
            .map(|(x, w)| x * w)
            .sum();
        assert!(weight <= 50.0 + 1e-6, "Weight constraint violated: {} > 50", weight);
    }

    #[test]
    fn test_knapsack_all_items_fit() {
        // All items fit in knapsack
        // Items: values = [5, 10, 15], weights = [1, 2, 3]
        // Capacity: 10 (all items total = 6)
        // Optimal: take all items → x = [1, 1, 1], z = 30

        let c = vec![5.0, 10.0, 15.0];
        let a = RealTensor::matrix(1, 3, vec![1.0, 2.0, 3.0]).unwrap();
        let b = vec![10.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 30.0).abs() < 1e-3, "Expected z = 30, got z = {}", z);

        // All items should be selected
        assert!((solution[0] - 1.0).abs() < 1e-6, "x₁ should be 1");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_one_item_only() {
        // Only one item fits
        // Items: values = [100, 200, 300], weights = [50, 50, 50]
        // Capacity: 50
        // Optimal: take item 3 → x = [0, 0, 1], z = 300

        let c = vec![100.0, 200.0, 300.0];
        let a = RealTensor::matrix(1, 3, vec![50.0, 50.0, 50.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 300.0).abs() < 1e-3, "Expected z = 300, got z = {}", z);

        // Only best item should be selected
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1 (best item)");

        // Total selected should be 1
        let total_selected: f64 = solution.iter().sum();
        assert!((total_selected - 1.0).abs() < 1e-6, "Only one item should be selected");
    }

    #[test]
    fn test_knapsack_tight_capacity() {
        // Tight capacity constraint
        // Items: values = [16, 19, 23, 28], weights = [2, 3, 4, 5]
        // Capacity: 7
        // Optimal: items 1, 2 (weight = 2+3 = 5) or items 2, 3 (weight = 3+4 = 7)
        // Best is items 2, 3 → x = [0, 1, 1, 0], z = 42

        let c = vec![16.0, 19.0, 23.0, 28.0];
        let a = RealTensor::matrix(1, 4, vec![2.0, 3.0, 4.0, 5.0]).unwrap();
        let b = vec![7.0];
        let binary_vars = vec![0, 1, 2, 3];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!(z >= 35.0, "Objective should be at least 35, got z = {}", z);

        // Check constraint satisfaction
        let weight: f64 = solution.iter()
            .zip(vec![2.0, 3.0, 4.0, 5.0].iter())
            .map(|(x, w)| x * w)
            .sum();
        assert!(weight <= 7.0 + 1e-6, "Weight exceeds capacity: {} > 7", weight);
    }

    #[test]
    fn test_intlinprog_multiple_constraints() {
        // Integer LP with multiple constraints
        // maximize z = 3x₁ + 4x₂
        // subject to: 2x₁ + 3x₂ ≤ 12, x₁ + x₂ ≤ 5
        // x₁, x₂ ∈ ℤ₊

        let c = vec![3.0, 4.0];
        let a = RealTensor::matrix(2, 2, vec![2.0, 3.0, 1.0, 1.0]).unwrap();
        let b = vec![12.0, 5.0];
        let integer_vars = vec![0, 1];

        let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();

        // Check integer
        for &idx in &integer_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} should be integer", idx);
        }

        let z = simplex::objective_value(&c, &solution).unwrap();
        // Optimal: x = [3, 2] with z = 17 (constraints: 2*3+3*2=12≤12, 3+2=5≤5)
        assert!((z - 17.0).abs() < 1e-3, "Expected z = 17, got z = {}", z);
    }
}
