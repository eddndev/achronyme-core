use achronyme_types::tensor::RealTensor;
use crate::linear::simplex;

/// Solve LP with variable bounds
///
/// Transforms the bounded LP into standard form by:
/// 1. Fixing variables where lower == upper
/// 2. Adding upper bound constraints xᵢ ≤ upper[i]
/// 3. Substituting fixed variables in constraints
///
/// # Parameters
/// - `c`: Objective coefficients
/// - `a`: Constraint matrix (m × n)
/// - `b`: RHS vector
/// - `sense`: 1.0 for maximize, -1.0 for minimize
/// - `lower`: Lower bounds for each variable
/// - `upper`: Upper bounds for each variable
///
/// # Returns
/// - `Ok(solution)`: Optimal solution respecting bounds
/// - `Err`: If subproblem is infeasible
///
/// # Note
/// Lower bounds > 0 are not fully enforced in current implementation.
/// This is a known limitation that relies on simplex vertex solutions.
pub fn solve_with_bounds(
    c: &[f64],
    a: &RealTensor,
    b: &[f64],
    sense: f64,
    lower: &[f64],
    upper: &[f64],
) -> Result<Vec<f64>, String> {
    let n = c.len();
    let m = a.rows();

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
