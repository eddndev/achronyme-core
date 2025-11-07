use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES};
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

// ============================================================================
// Linear Programming and Optimization
// ============================================================================

/// Simplex method for linear programming
/// simplex(c, A, b, sense) - solves LP using primal simplex
///
/// # Parameters
/// - c_handle: Handle to objective coefficients vector
/// - a_handle: Handle to constraint matrix A
/// - b_handle: Handle to RHS vector b
/// - sense: 1.0 for maximize, -1.0 for minimize
///
/// # Returns
/// Handle to solution vector x
#[wasm_bindgen(js_name = simplex)]
pub fn simplex(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        // Extract data from handles
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("simplex: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("simplex: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("simplex: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        // Solve using simplex
        use achronyme_solver::simplex_solve;
        let solution = simplex_solve(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        // Create handle for solution
        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}

/// Linear programming with auto-selection of method
/// linprog(c, A, b, sense) - automatically selects best LP method
#[wasm_bindgen(js_name = linprog)]
pub fn linprog(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("linprog: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("linprog: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("linprog: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::linprog_solve;
        let solution = linprog_solve(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}

/// Dual simplex method
/// dualSimplex(c, A, b, sense) - solves LP using dual simplex
#[wasm_bindgen(js_name = dualSimplex)]
pub fn dual_simplex(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("dualSimplex: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("dualSimplex: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("dualSimplex: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::dual_simplex_solve;
        let solution = dual_simplex_solve(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}

/// Two-phase simplex method
/// twoPhaseSimplex(c, A, b, sense) - handles difficult starting conditions
#[wasm_bindgen(js_name = twoPhaseSimplex)]
pub fn two_phase_simplex(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("twoPhaseSimplex: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("twoPhaseSimplex: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("twoPhaseSimplex: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::two_phase_solve;
        let solution = two_phase_solve(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}

/// Revised simplex method (memory efficient)
/// revisedSimplex(c, A, b, sense) - best for large problems
#[wasm_bindgen(js_name = revisedSimplex)]
pub fn revised_simplex(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("revisedSimplex: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("revisedSimplex: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("revisedSimplex: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::revised_simplex_solve;
        let solution = revised_simplex_solve(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}

/// Calculate objective value c·x
/// objectiveValue(c, x) - computes dot product
#[wasm_bindgen(js_name = objectiveValue)]
pub fn objective_value(
    c_handle: Handle,
    x_handle: Handle,
) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, x_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("objectiveValue: c must be a vector")),
            }?;

            let x = match handles.get(x_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("objectiveValue: x must be a vector")),
            }?;

            Ok::<_, JsValue>((c, x))
        }?;

        use achronyme_solver::objective_value;
        objective_value(&c_vec, &x_vec)
            .map_err(|e| JsValue::from_str(&e))
    })
}

/// Shadow prices (dual variables)
/// shadowPrice(c, A, b, sense) - marginal values of resources
///
/// # Returns
/// Handle to vector of shadow prices (one per constraint)
#[wasm_bindgen(js_name = shadowPrice)]
pub fn shadow_price(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("shadowPrice: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("shadowPrice: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("shadowPrice: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::shadow_price;
        let prices = shadow_price(&c_vec, &a_mat, &b_vec, sense)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(prices))))
    })
}

/// Sensitivity analysis for objective coefficient
/// sensitivityC(c, A, b, index) - range for c[index]
///
/// # Returns
/// Handle to vector [c_min, c_max]
#[wasm_bindgen(js_name = sensitivityC)]
pub fn sensitivity_c(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    index: usize,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("sensitivityC: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("sensitivityC: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("sensitivityC: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::sensitivity_c;
        let range = sensitivity_c(&c_vec, &a_mat, &b_vec, index)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(range))))
    })
}

/// Sensitivity analysis for RHS constraint
/// sensitivityB(c, A, b, index) - range for b[index]
///
/// # Returns
/// Handle to vector [b_min, b_max]
#[wasm_bindgen(js_name = sensitivityB)]
pub fn sensitivity_b(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    index: usize,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("sensitivityB: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("sensitivityB: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("sensitivityB: b must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b))
        }?;

        use achronyme_solver::sensitivity_b;
        let range = sensitivity_b(&c_vec, &a_mat, &b_vec, index)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(range))))
    })
}

// ============================================================================
// Integer Programming
// ============================================================================

/// Integer Linear Programming using Branch & Bound
/// intlinprog(c, A, b, sense, integerVars) - solves IP with integer constraints
///
/// # Parameters
/// - c_handle: Handle to objective coefficients vector
/// - a_handle: Handle to constraint matrix A
/// - b_handle: Handle to RHS vector b
/// - sense: 1.0 for maximize, -1.0 for minimize
/// - integer_vars_handle: Handle to vector of variable indices that must be integer
///
/// # Returns
/// Handle to integer solution vector x
///
/// # Algorithm
/// Uses Branch & Bound with LP relaxations
///
/// # Example
/// ```javascript
/// // maximize z = 3x₁ + 2x₂
/// // subject to: x₁ + x₂ ≤ 4, x₁, x₂ ∈ ℤ₊
/// const c = ach.vector([3, 2]);
/// const A = ach.matrix([[1, 1]]);
/// const b = ach.vector([4]);
/// const intVars = ach.vector([0, 1]); // Both variables must be integer
/// const solution = ach.optimization.intlinprog(c.handle, A.handle, b.handle, 1, intVars.handle);
/// ```
#[wasm_bindgen(js_name = intlinprog)]
pub fn intlinprog(
    c_handle: Handle,
    a_handle: Handle,
    b_handle: Handle,
    sense: f64,
    integer_vars_handle: Handle,
) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let (c_vec, a_mat, b_vec, int_vars) = {
            let handles = h.borrow();

            let c = match handles.get(c_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("intlinprog: c must be a vector")),
            }?;

            let a = match handles.get(a_handle) {
                Some(Value::Matrix(m)) => Ok(m.clone()),
                _ => Err(JsValue::from_str("intlinprog: A must be a matrix")),
            }?;

            let b = match handles.get(b_handle) {
                Some(Value::Vector(v)) => Ok(v.data().to_vec()),
                _ => Err(JsValue::from_str("intlinprog: b must be a vector")),
            }?;

            let integer_vars_vec = match handles.get(integer_vars_handle) {
                Some(Value::Vector(v)) => {
                    // Convert f64 to usize
                    let vars: Vec<usize> = v.data().iter().map(|&val| val as usize).collect();
                    Ok(vars)
                }
                _ => Err(JsValue::from_str("intlinprog: integer_vars must be a vector")),
            }?;

            Ok::<_, JsValue>((c, a, b, integer_vars_vec))
        }?;

        use achronyme_solver::intlinprog;
        let solution = intlinprog(&c_vec, &a_mat, &b_vec, sense, &int_vars)
            .map_err(|e| JsValue::from_str(e.as_str()))?;

        Ok(h.borrow_mut().create(Value::Vector(Vector::new(solution))))
    })
}
