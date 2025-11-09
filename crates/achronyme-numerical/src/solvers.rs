//! Equation Solvers and Root Finding
//!
//! Provides numerical methods for finding roots and solving equations.

use achronyme_types::function::Function;
use achronyme_types::LambdaEvaluator;

/// Bisection method for root finding
///
/// Finds a root of f(x) = 0 in the interval [a, b].
/// Requires f(a) and f(b) to have opposite signs.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function for which to find the root
/// * `a` - Left endpoint of interval
/// * `b` - Right endpoint of interval
/// * `tol` - Tolerance (stop when |b - a| < tol)
///
/// # Example
/// ```ignore
/// let root = bisect(&mut evaluator, &func, 0.0, 5.0, 1e-6)?;
/// ```
pub fn bisect<E>(
    evaluator: &mut E,
    func: &Function,
    mut a: f64,
    mut b: f64,
    tol: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let fa = evaluator.eval_at(func, a)?;
    let fb = evaluator.eval_at(func, b)?;

    if fa * fb > 0.0 {
        return Err("bisect: f(a) and f(b) must have opposite signs".to_string());
    }

    while (b - a).abs() > tol {
        let c = (a + b) / 2.0;
        let fc = evaluator.eval_at(func, c)?;

        if fc.abs() < tol {
            return Ok(c);
        }

        if fa * fc < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    Ok((a + b) / 2.0)
}

/// Newton-Raphson method for root finding
///
/// Finds a root of f(x) = 0 using Newton's method: x_{n+1} = x_n - f(x_n) / f'(x_n)
///
/// Converges quadratically near the root, but may diverge if the initial guess is poor.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function for which to find the root
/// * `dfunc` - Derivative of func
/// * `x0` - Initial guess
/// * `tol` - Tolerance (stop when |f(x)| < tol)
/// * `max_iter` - Maximum number of iterations
///
/// # Example
/// ```ignore
/// let root = newton(&mut evaluator, &func, &dfunc, 1.0, 1e-10, 100)?;
/// ```
pub fn newton<E>(
    evaluator: &mut E,
    func: &Function,
    dfunc: &Function,
    mut x: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    for _ in 0..max_iter {
        let fx = evaluator.eval_at(func, x)?;

        if fx.abs() < tol {
            return Ok(x);
        }

        let dfx = evaluator.eval_at(dfunc, x)?;

        if dfx.abs() < 1e-12 {
            // Derivative too small, cannot continue
            return Err("Newton: derivative too small, cannot continue".to_string());
        }

        x = x - fx / dfx;
    }

    Ok(x)
}

/// Secant method for root finding
///
/// Similar to Newton's method but doesn't require the derivative.
/// Uses two initial points instead.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function for which to find the root
/// * `x0` - First initial guess
/// * `x1` - Second initial guess
/// * `tol` - Tolerance (stop when |f(x)| < tol)
/// * `max_iter` - Maximum number of iterations
///
/// # Example
/// ```ignore
/// let root = secant(&mut evaluator, &func, 1.0, 2.0, 1e-10, 100)?;
/// ```
pub fn secant<E>(
    evaluator: &mut E,
    func: &Function,
    mut x0: f64,
    mut x1: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let mut fx0 = evaluator.eval_at(func, x0)?;

    for _ in 0..max_iter {
        let fx1 = evaluator.eval_at(func, x1)?;

        if fx1.abs() < tol {
            return Ok(x1);
        }

        if (fx1 - fx0).abs() < 1e-12 {
            // Denominator too small
            return Err("Secant: denominator too small, cannot continue".to_string());
        }

        let x2 = x1 - fx1 * (x1 - x0) / (fx1 - fx0);

        x0 = x1;
        fx0 = fx1;
        x1 = x2;
    }

    Ok(x1)
}

/// Fixed-point iteration
///
/// Finds a fixed point of g(x), i.e., a value x such that g(x) = x.
/// This can be used to solve f(x) = 0 by rewriting as g(x) = x.
///
/// # Arguments
/// * `g` - Function g where we seek x = g(x)
/// * `x0` - Initial guess
/// * `tol` - Tolerance (stop when |x_{n+1} - x_n| < tol)
/// * `max_iter` - Maximum number of iterations
///
/// # Example
/// ```
/// // Find fixed point of g(x) = cos(x)
/// let g = |x: f64| x.cos();
/// let fixed_point = fixed_point_iteration(g, 0.0, 1e-10, 100);
/// assert!((fixed_point - fixed_point.cos()).abs() < 1e-9);
/// ```
pub fn fixed_point_iteration<F>(mut g: F, mut x: f64, tol: f64, max_iter: usize) -> f64
where
    F: FnMut(f64) -> f64,
{
    for _ in 0..max_iter {
        let x_new = g(x);

        if (x_new - x).abs() < tol {
            return x_new;
        }

        x = x_new;
    }

    x
}

/// Newton's method for systems of equations (simplified 2D version)
///
/// Finds a solution to the system:
/// f1(x, y) = 0
/// f2(x, y) = 0
///
/// # Arguments
/// * `f1` - First equation
/// * `f2` - Second equation
/// * `x0` - Initial guess for x
/// * `y0` - Initial guess for y
/// * `tol` - Tolerance
/// * `max_iter` - Maximum iterations
///
/// # Returns
/// (x, y) solution as a tuple
pub fn newton_system_2d<F1, F2>(
    mut f1: F1,
    mut f2: F2,
    mut x: f64,
    mut y: f64,
    tol: f64,
    max_iter: usize,
) -> (f64, f64)
where
    F1: FnMut(f64, f64) -> f64,
    F2: FnMut(f64, f64) -> f64,
{
    let h = 1e-8;

    for _ in 0..max_iter {
        let f1_val = f1(x, y);
        let f2_val = f2(x, y);

        // Check convergence
        if f1_val.abs() < tol && f2_val.abs() < tol {
            return (x, y);
        }

        // Compute Jacobian using finite differences
        let df1_dx = (f1(x + h, y) - f1_val) / h;
        let df1_dy = (f1(x, y + h) - f1_val) / h;
        let df2_dx = (f2(x + h, y) - f2_val) / h;
        let df2_dy = (f2(x, y + h) - f2_val) / h;

        // Determinant of Jacobian
        let det = df1_dx * df2_dy - df1_dy * df2_dx;

        if det.abs() < 1e-12 {
            break; // Singular Jacobian
        }

        // Solve linear system: J * delta = -F
        let dx = (-f1_val * df2_dy + f2_val * df1_dy) / det;
        let dy = (f1_val * df2_dx - f2_val * df1_dx) / det;

        x += dx;
        y += dy;
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: These tests use the old closure-based API and need to be refactored
    // to use the new Evaluator + Function API. They are marked as #[ignore] for now.
    // TODO: Refactor tests to use achronyme_eval::Evaluator and Function objects

    #[test]
    #[ignore]
    fn test_bisect_quadratic() {
        // x² - 4 = 0, root at x = 2
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_bisect_cubic() {
        // x³ - x - 2 = 0, root at x ≈ 1.521
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_newton_quadratic() {
        // x² - 4 = 0, derivative = 2x
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_newton_trig() {
        // cos(x) = 0, root at x = π/2
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_secant() {
        // x³ - x - 2 = 0
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_fixed_point() {
        // Fixed point of cos(x)
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn test_newton_system_2d() {
        // System:
        // x² + y² = 25  (circle)
        // x - y = 1     (line)
        // Solution: (4, 3) and (-3, -4)
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_bisect_same_sign_panic() {
        // Both endpoints positive, should panic
        // TODO: Refactor to use Evaluator + Function
    }
}
