/// Tolerance for integer constraint checking
const TOLERANCE: f64 = 1e-6;

/// Check if solution satisfies integer constraints
///
/// A value is considered integer if |value - round(value)| < TOLERANCE
///
/// # Parameters
/// - `solution`: Candidate solution vector
/// - `integer_vars`: Indices of variables that must be integer
///
/// # Returns
/// `true` if all integer variables are within tolerance of an integer value
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

/// Find a fractional variable to branch on
///
/// Uses the "most fractional" heuristic: selects the variable
/// closest to 0.5 (i.e., furthest from an integer value).
///
/// # Parameters
/// - `solution`: Current LP solution
/// - `integer_vars`: Indices of variables that must be integer
///
/// # Returns
/// `Some(index)` of the most fractional variable, or `None` if all are integer
pub fn find_fractional_var(solution: &[f64], integer_vars: &[usize]) -> Option<usize> {
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

/// Check if should prune this branch based on bound
///
/// Pruning rules:
/// - Maximization: prune if node_objective ≤ best_objective
/// - Minimization: prune if node_objective ≥ best_objective
///
/// # Parameters
/// - `node_objective`: Objective value of current node
/// - `best_objective`: Best integer objective found so far
/// - `sense`: 1.0 for maximize, -1.0 for minimize
pub fn should_prune(node_objective: f64, best_objective: f64, sense: f64) -> bool {
    if sense > 0.0 {
        // Maximize: prune if node objective ≤ best
        node_objective <= best_objective
    } else {
        // Minimize: prune if node objective ≥ best
        node_objective >= best_objective
    }
}

/// Check if obj1 is better than obj2 according to optimization sense
///
/// # Parameters
/// - `obj1`: First objective value
/// - `obj2`: Second objective value
/// - `sense`: 1.0 for maximize, -1.0 for minimize
pub fn is_better(obj1: f64, obj2: f64, sense: f64) -> bool {
    if sense > 0.0 {
        obj1 > obj2
    } else {
        obj1 < obj2
    }
}
