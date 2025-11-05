//! Numerical Differentiation
//!
//! Provides finite difference methods for calculating derivatives numerically.

use achronyme_types::function::Function;
use achronyme_types::LambdaEvaluator;

/// Numerical derivative using forward difference
///
/// f'(x) ≈ (f(x + h) - f(x)) / h
///
/// # Arguments
/// * `f` - Function to differentiate
/// * `x` - Point at which to calculate derivative
/// * `h` - Step size (default: 1e-5)
///
/// # Example
/// ```
/// let f = |x: f64| x * x;  // f(x) = x²
/// let derivative = diff_forward(f, 2.0, 1e-5);  // f'(2) ≈ 4
/// ```
pub fn diff_forward<F>(mut f: F, x: f64, h: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}

/// Numerical derivative using backward difference
///
/// f'(x) ≈ (f(x) - f(x - h)) / h
///
/// # Arguments
/// * `f` - Function to differentiate
/// * `x` - Point at which to calculate derivative
/// * `h` - Step size (default: 1e-5)
pub fn diff_backward<F>(mut f: F, x: f64, h: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    (f(x) - f(x - h)) / h
}

/// Numerical derivative using central difference (most accurate)
///
/// f'(x) ≈ (f(x + h) - f(x - h)) / (2h)
///
/// More accurate than forward or backward differences (O(h²) vs O(h) error).
///
/// # Arguments
/// * `evaluator` - Lambda evaluator (usually the Evaluator)
/// * `func` - Function to differentiate
/// * `x` - Point at which to calculate derivative
/// * `h` - Step size (default: 1e-5)
///
/// # Example
/// ```ignore
/// let derivative = diff_central(&mut evaluator, &func, 2.0, 1e-5);
/// ```
pub fn diff_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let f_plus = evaluator.eval_at(func, x + h)?;
    let f_minus = evaluator.eval_at(func, x - h)?;
    Ok((f_plus - f_minus) / (2.0 * h))
}

/// Second derivative using central difference
///
/// f''(x) ≈ (f(x + h) - 2f(x) + f(x - h)) / h²
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to differentiate
/// * `x` - Point at which to calculate second derivative
/// * `h` - Step size (default: 1e-3)
pub fn diff2_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let f_plus = evaluator.eval_at(func, x + h)?;
    let f_center = evaluator.eval_at(func, x)?;
    let f_minus = evaluator.eval_at(func, x - h)?;
    Ok((f_plus - 2.0 * f_center + f_minus) / (h * h))
}

/// Third derivative using finite differences
///
/// Calculates d³f/dx³ at point x using central difference formula.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to differentiate
/// * `x` - Point at which to calculate third derivative
/// * `h` - Step size (default: 1e-2)
pub fn diff3_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let h3 = h * h * h;
    let f_2h = evaluator.eval_at(func, x + 2.0 * h)?;
    let f_h = evaluator.eval_at(func, x + h)?;
    let f_minus_h = evaluator.eval_at(func, x - h)?;
    let f_minus_2h = evaluator.eval_at(func, x - 2.0 * h)?;
    Ok((f_2h - 2.0 * f_h + 2.0 * f_minus_h - f_minus_2h) / (2.0 * h3))
}

/// Gradient of a multivariate function
///
/// Calculates ∇f = [∂f/∂x₁, ∂f/∂x₂, ..., ∂f/∂xₙ]
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Multivariate function f(x₁, x₂, ..., xₙ)
/// * `point` - Point at which to calculate gradient
/// * `h` - Step size (default: 1e-5)
pub fn gradient<E>(
    evaluator: &mut E,
    func: &Function,
    point: &[f64],
    h: f64,
) -> Result<Vec<f64>, String>
where
    E: LambdaEvaluator,
{
    let n = point.len();
    let mut grad = vec![0.0; n];

    for i in 0..n {
        let mut point_plus = point.to_vec();
        let mut point_minus = point.to_vec();

        point_plus[i] += h;
        point_minus[i] -= h;

        let f_plus = evaluator.eval_vec_at(func, &point_plus)?;
        let f_minus = evaluator.eval_vec_at(func, &point_minus)?;

        grad[i] = (f_plus - f_minus) / (2.0 * h);
    }

    Ok(grad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_difference() {
        // f(x) = x², f'(x) = 2x
        let f = |x: f64| x * x;
        let derivative = diff_forward(f, 2.0, 1e-5);
        assert!((derivative - 4.0).abs() < 1e-4);
    }

    #[test]
    fn test_backward_difference() {
        // f(x) = x², f'(x) = 2x
        let f = |x: f64| x * x;
        let derivative = diff_backward(f, 2.0, 1e-5);
        assert!((derivative - 4.0).abs() < 1e-4);
    }

    #[test]
    fn test_central_difference() {
        // f(x) = x³, f'(x) = 3x²
        let f = |x: f64| x.powi(3);
        let derivative = diff_central(f, 2.0, 1e-5);
        assert!((derivative - 12.0).abs() < 1e-4);
    }

    #[test]
    fn test_second_derivative() {
        // f(x) = x³, f''(x) = 6x
        let f = |x: f64| x.powi(3);
        let second_derivative = diff2_central(f, 2.0, 1e-3);
        assert!((second_derivative - 12.0).abs() < 1e-2);
    }

    #[test]
    fn test_third_derivative() {
        // f(x) = x⁴, f'''(x) = 24x
        let f = |x: f64| x.powi(4);
        let third_derivative = diff3_central(f, 2.0, 1e-2);
        assert!((third_derivative - 48.0).abs() < 1.0);
    }

    #[test]
    fn test_gradient() {
        // f(x, y) = x² + y², ∇f = [2x, 2y]
        let f = |coords: &[f64]| coords[0].powi(2) + coords[1].powi(2);
        let grad = gradient(f, &[1.0, 2.0], 1e-5);

        assert!((grad[0] - 2.0).abs() < 1e-3);
        assert!((grad[1] - 4.0).abs() < 1e-3);
    }

    #[test]
    fn test_trig_functions() {
        // f(x) = sin(x), f'(x) = cos(x)
        let f = |x: f64| x.sin();
        let derivative = diff_central(f, std::f64::consts::PI / 4.0, 1e-6);
        let expected = (std::f64::consts::PI / 4.0).cos();
        assert!((derivative - expected).abs() < 1e-5);
    }
}
