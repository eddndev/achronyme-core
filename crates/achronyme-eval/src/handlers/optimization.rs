use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

use crate::evaluator::Evaluator;

/// Optimization Functions Handler
///
/// This module contains implementations of linear programming solvers.

/// Simplex method: simplex(c, A, b, sense)
///
/// Solves linear programming problems:
///   maximize/minimize z = c^T * x
///   subject to: A * x <= b, x >= 0
///
/// Args:
///   - c: objective coefficients (vector)
///   - A: constraint matrix
///   - b: constraint bounds (vector)
///   - sense: 1 for maximize, -1 for minimize
pub fn handle_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("simplex() requires 4 arguments: c, A, b, sense".to_string());
    }

    // Evaluate c (objective vector)
    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("simplex: c must be a vector".to_string()),
    };

    // Evaluate A (constraint matrix)
    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("simplex: A must be a matrix".to_string()),
    };

    // Evaluate b (bounds vector)
    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("simplex: b must be a vector".to_string()),
    };

    // Evaluate sense (1 or -1)
    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("simplex: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("simplex: sense must be a number".to_string()),
    };

    // Validations
    if a_mat.cols != c_vec.len() {
        return Err(format!(
            "simplex: matrix A has {} columns but c has {} elements",
            a_mat.cols, c_vec.len()
        ));
    }
    if a_mat.rows != b_vec.len() {
        return Err(format!(
            "simplex: matrix A has {} rows but b has {} elements",
            a_mat.rows, b_vec.len()
        ));
    }

    // Solve using Simplex
    use achronyme_solver::simplex_solve;
    let solution = simplex_solve(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(solution)))
}

/// Linear programming with auto-selection: linprog(c, A, b, sense)
///
/// Automatically selects the best method based on problem characteristics.
/// Currently uses Simplex, but will use Dual Simplex, Revised Simplex,
/// or Interior Point in the future.
pub fn handle_linprog(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("linprog() requires 4 arguments: c, A, b, sense".to_string());
    }

    // Evaluate c (objective vector)
    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("linprog: c must be a vector".to_string()),
    };

    // Evaluate A (constraint matrix)
    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("linprog: A must be a matrix".to_string()),
    };

    // Evaluate b (bounds vector)
    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("linprog: b must be a vector".to_string()),
    };

    // Evaluate sense (1 or -1)
    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("linprog: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("linprog: sense must be a number".to_string()),
    };

    // Validations
    if a_mat.cols != c_vec.len() {
        return Err(format!(
            "linprog: matrix A has {} columns but c has {} elements",
            a_mat.cols, c_vec.len()
        ));
    }
    if a_mat.rows != b_vec.len() {
        return Err(format!(
            "linprog: matrix A has {} rows but b has {} elements",
            a_mat.rows, b_vec.len()
        ));
    }

    // Solve using linprog (auto-selection)
    use achronyme_solver::linprog_solve;
    let solution = linprog_solve(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(solution)))
}

/// Calculate objective value: objective_value(c, x)
///
/// Computes z = c^T * x
pub fn handle_objective_value(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("objective_value() requires 2 arguments: c, x".to_string());
    }

    // Evaluate c
    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("objective_value: c must be a vector".to_string()),
    };

    // Evaluate x
    let x_vec = match evaluator.evaluate(&args[1])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("objective_value: x must be a vector".to_string()),
    };

    // Calculate z = c^T * x
    use achronyme_solver::objective_value;
    let z = objective_value(&c_vec, &x_vec)?;

    Ok(Value::Number(z))
}

/// Dual Simplex method: dual_simplex(c, A, b, sense)
///
/// Solves LP when you have dual feasibility but not primal feasibility.
/// Useful for:
/// - Problems with many variables but few constraints
/// - Sensitivity analysis (adding constraints)
/// - Branch-and-bound algorithms
pub fn handle_dual_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("dual_simplex() requires 4 arguments: c, A, b, sense".to_string());
    }

    // Evaluate arguments (same as simplex)
    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("dual_simplex: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("dual_simplex: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("dual_simplex: b must be a vector".to_string()),
    };

    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("dual_simplex: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("dual_simplex: sense must be a number".to_string()),
    };

    // Validations
    if a_mat.cols != c_vec.len() {
        return Err(format!(
            "dual_simplex: matrix A has {} columns but c has {} elements",
            a_mat.cols, c_vec.len()
        ));
    }
    if a_mat.rows != b_vec.len() {
        return Err(format!(
            "dual_simplex: matrix A has {} rows but b has {} elements",
            a_mat.rows, b_vec.len()
        ));
    }

    // Solve using Dual Simplex
    use achronyme_solver::dual_simplex_solve;
    let solution = dual_simplex_solve(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(solution)))
}

/// Two-Phase Simplex method: two_phase_simplex(c, A, b, sense)
///
/// Finds an initial BFS when it's not obvious.
/// Useful for problems with:
/// - Equality constraints (=)
/// - Greater-than constraints (≥)
/// - Negative RHS values
pub fn handle_two_phase_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("two_phase_simplex() requires 4 arguments: c, A, b, sense".to_string());
    }

    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("two_phase_simplex: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("two_phase_simplex: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("two_phase_simplex: b must be a vector".to_string()),
    };

    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("two_phase_simplex: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("two_phase_simplex: sense must be a number".to_string()),
    };

    // Validations
    if a_mat.cols != c_vec.len() {
        return Err(format!(
            "two_phase_simplex: matrix A has {} columns but c has {} elements",
            a_mat.cols, c_vec.len()
        ));
    }
    if a_mat.rows != b_vec.len() {
        return Err(format!(
            "two_phase_simplex: matrix A has {} rows but b has {} elements",
            a_mat.rows, b_vec.len()
        ));
    }

    // Solve using Two-Phase Simplex
    use achronyme_solver::two_phase_solve;
    let solution = two_phase_solve(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(solution)))
}

/// Revised Simplex method: revised_simplex(c, A, b, sense)
///
/// Memory-efficient version of Simplex.
/// Stores only the basis inverse B⁻¹ instead of full tableau.
/// Best for:
/// - Large problems (n > 1000)
/// - Sparse matrices
/// - Many variables, few constraints
pub fn handle_revised_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("revised_simplex() requires 4 arguments: c, A, b, sense".to_string());
    }

    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("revised_simplex: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("revised_simplex: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("revised_simplex: b must be a vector".to_string()),
    };

    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("revised_simplex: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("revised_simplex: sense must be a number".to_string()),
    };

    // Validations
    if a_mat.cols != c_vec.len() {
        return Err(format!(
            "revised_simplex: matrix A has {} columns but c has {} elements",
            a_mat.cols, c_vec.len()
        ));
    }
    if a_mat.rows != b_vec.len() {
        return Err(format!(
            "revised_simplex: matrix A has {} rows but b has {} elements",
            a_mat.rows, b_vec.len()
        ));
    }

    // Solve using Revised Simplex
    use achronyme_solver::revised_simplex_solve;
    let solution = revised_simplex_solve(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(solution)))
}

// ============================================================================
// Sensitivity Analysis Functions
// ============================================================================

/// Shadow prices: shadow_price(c, A, b, sense)
///
/// Calculate dual variables (shadow prices) for each constraint.
/// Indicates how much the objective improves per unit increase in b[i].
pub fn handle_shadow_price(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("shadow_price() requires 4 arguments: c, A, b, sense".to_string());
    }

    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("shadow_price: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("shadow_price: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("shadow_price: b must be a vector".to_string()),
    };

    let sense = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => {
            if n != 1.0 && n != -1.0 {
                return Err("shadow_price: sense must be 1 (maximize) or -1 (minimize)".to_string());
            }
            n
        },
        _ => return Err("shadow_price: sense must be a number".to_string()),
    };

    use achronyme_solver::shadow_price;
    let prices = shadow_price(&c_vec, &a_mat, &b_vec, sense)?;

    Ok(Value::Vector(Vector::new(prices)))
}

/// Sensitivity analysis for c: sensitivity_c(c, A, b, i)
///
/// Returns [c_min, c_max] range for c[i] that maintains optimal basis.
pub fn handle_sensitivity_c(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("sensitivity_c() requires 4 arguments: c, A, b, index".to_string());
    }

    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("sensitivity_c: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("sensitivity_c: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("sensitivity_c: b must be a vector".to_string()),
    };

    let index = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n as usize,
        _ => return Err("sensitivity_c: index must be a number".to_string()),
    };

    use achronyme_solver::sensitivity_c;
    let range = sensitivity_c(&c_vec, &a_mat, &b_vec, index)?;

    Ok(Value::Vector(Vector::new(range)))
}

/// Sensitivity analysis for b: sensitivity_b(c, A, b, i)
///
/// Returns [b_min, b_max] range for b[i] that maintains optimal basis.
pub fn handle_sensitivity_b(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("sensitivity_b() requires 4 arguments: c, A, b, index".to_string());
    }

    let c_vec = match evaluator.evaluate(&args[0])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("sensitivity_b: c must be a vector".to_string()),
    };

    let a_mat = match evaluator.evaluate(&args[1])? {
        Value::Matrix(m) => m,
        _ => return Err("sensitivity_b: A must be a matrix".to_string()),
    };

    let b_vec = match evaluator.evaluate(&args[2])? {
        Value::Vector(v) => v.data().to_vec(),
        _ => return Err("sensitivity_b: b must be a vector".to_string()),
    };

    let index = match evaluator.evaluate(&args[3])? {
        Value::Number(n) => n as usize,
        _ => return Err("sensitivity_b: index must be a number".to_string()),
    };

    use achronyme_solver::sensitivity_b;
    let range = sensitivity_b(&c_vec, &a_mat, &b_vec, index)?;

    Ok(Value::Vector(Vector::new(range)))
}
