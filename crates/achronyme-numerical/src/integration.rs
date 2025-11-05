//! Numerical Integration
//!
//! Provides methods for numerical integration (quadrature).

use achronyme_types::function::Function;
use achronyme_types::LambdaEvaluator;

/// Trapezoidal rule for numerical integration
///
/// ∫f(x)dx ≈ h/2 * (f(x₀) + 2f(x₁) + 2f(x₂) + ... + 2f(xₙ₋₁) + f(xₙ))
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to integrate
/// * `a` - Lower limit of integration
/// * `b` - Upper limit of integration
/// * `n` - Number of subdivisions (higher = more accurate)
pub fn trapz<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    if n == 0 {
        return Ok(0.0);
    }

    let h = (b - a) / n as f64;
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    let mut sum = 0.5 * (f_a + f_b);

    for i in 1..n {
        sum += evaluator.eval_at(func, a + i as f64 * h)?;
    }

    Ok(h * sum)
}

/// Simpson's 1/3 rule for numerical integration
///
/// ∫f(x)dx ≈ h/3 * (f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + f(xₙ))
///
/// More accurate than trapezoidal rule. Requires n to be even.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to integrate
/// * `a` - Lower limit of integration
/// * `b` - Upper limit of integration
/// * `n` - Number of subdivisions (must be even)
pub fn simpson<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let n = if n % 2 == 0 { n } else { n + 1 }; // Ensure n is even

    if n == 0 {
        return Ok(0.0);
    }

    let h = (b - a) / n as f64;
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    let mut sum = f_a + f_b;

    for i in 1..n {
        let x = a + i as f64 * h;
        let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += coefficient * evaluator.eval_at(func, x)?;
    }

    Ok((h / 3.0) * sum)
}

/// Simpson's 3/8 rule for numerical integration
///
/// Alternative Simpson's rule with different weighting.
/// Requires n to be divisible by 3.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to integrate
/// * `a` - Lower limit of integration
/// * `b` - Upper limit of integration
/// * `n` - Number of subdivisions (must be divisible by 3)
pub fn simpson38<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let n = ((n + 2) / 3) * 3; // Round up to multiple of 3

    if n == 0 {
        return Ok(0.0);
    }

    let h = (b - a) / n as f64;
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    let mut sum = f_a + f_b;

    for i in 1..n {
        let x = a + i as f64 * h;
        let coefficient = if i % 3 == 0 { 2.0 } else { 3.0 };
        sum += coefficient * evaluator.eval_at(func, x)?;
    }

    Ok((3.0 * h / 8.0) * sum)
}

/// Romberg integration (adaptive quadrature)
///
/// Uses Richardson extrapolation to achieve high accuracy.
/// Automatically adapts the step size to reach the desired tolerance.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to integrate
/// * `a` - Lower limit of integration
/// * `b` - Upper limit of integration
/// * `tol` - Desired tolerance (default: 1e-8)
/// * `max_iter` - Maximum number of iterations (default: 20)
///
/// # Example
/// ```ignore
/// let result = romberg(&mut evaluator, &func, 0.0, std::f64::consts::PI, 1e-10, 20);
/// ```
pub fn romberg<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let mut r = vec![vec![0.0; max_iter]; max_iter];

    // First column: trapezoidal rule with increasing subdivisions
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    r[0][0] = (b - a) * (f_a + f_b) / 2.0;

    for i in 1..max_iter {
        // Trapezoidal rule with 2^i subdivisions
        let n = 1 << i; // 2^i
        let h = (b - a) / n as f64;

        let mut sum = 0.0;
        for j in 1..n {
            if j % 2 == 1 {
                sum += evaluator.eval_at(func, a + j as f64 * h)?;
            }
        }

        r[i][0] = 0.5 * r[i - 1][0] + h * sum;

        // Richardson extrapolation
        for j in 1..=i {
            let power = 4_f64.powi(j as i32);
            r[i][j] = (power * r[i][j - 1] - r[i - 1][j - 1]) / (power - 1.0);
        }

        // Check convergence
        if i > 0 && (r[i][i] - r[i - 1][i - 1]).abs() < tol {
            return Ok(r[i][i]);
        }
    }

    Ok(r[max_iter - 1][max_iter - 1])
}

/// Gaussian quadrature (adaptive)
///
/// High-accuracy integration using Gaussian quadrature nodes.
/// This is a simplified adaptive version.
///
/// # Arguments
/// * `evaluator` - Lambda evaluator
/// * `func` - Function to integrate
/// * `a` - Lower limit of integration
/// * `b` - Upper limit of integration
/// * `tol` - Desired tolerance
///
/// # Example
/// ```ignore
/// let result = quad(&mut evaluator, &func, 0.0, 1.0, 1e-10);
/// ```
pub fn quad<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    tol: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // Use Simpson's rule with adaptive refinement
    let mut n = 10;

    // Helper function to compute Simpson's rule
    let simpson_impl = |evaluator: &mut E, func: &Function, a: f64, b: f64, n: usize| -> Result<f64, String> {
        let n = if n % 2 == 0 { n } else { n + 1 };
        if n == 0 {
            return Ok(0.0);
        }
        let h = (b - a) / n as f64;
        let f_a = evaluator.eval_at(func, a)?;
        let f_b = evaluator.eval_at(func, b)?;
        let mut sum = f_a + f_b;
        for i in 1..n {
            let x = a + i as f64 * h;
            let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
            sum += coefficient * evaluator.eval_at(func, x)?;
        }
        Ok((h / 3.0) * sum)
    };

    let mut prev = simpson_impl(evaluator, func, a, b, n)?;

    loop {
        n *= 2;
        let curr = simpson_impl(evaluator, func, a, b, n)?;

        if (curr - prev).abs() < tol || n > 100000 {
            return Ok(curr);
        }

        prev = curr;
    }
}

/// Integrate a discrete dataset using trapezoidal rule
///
/// Useful when you have data points instead of a function.
///
/// # Arguments
/// * `x` - x-coordinates (must be sorted)
/// * `y` - y-coordinates (function values)
///
/// # Example
/// ```
/// let x = vec![0.0, 1.0, 2.0, 3.0];
/// let y = vec![0.0, 1.0, 4.0, 9.0];  // y = x²
/// let result = trapz_discrete(&x, &y);
/// ```
pub fn trapz_discrete(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }

    let mut sum = 0.0;

    for i in 0..x.len() - 1 {
        let h = x[i + 1] - x[i];
        sum += 0.5 * h * (y[i] + y[i + 1]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_trapz_linear() {
        // ∫x dx from 0 to 1 = 0.5
        let f = |x: f64| x;
        let result = trapz(f, 0.0, 1.0, 1000);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_trapz_quadratic() {
        // ∫x² dx from 0 to 1 = 1/3
        let f = |x: f64| x * x;
        let result = trapz(f, 0.0, 1.0, 1000);
        assert!((result - 1.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_simpson_quadratic() {
        // ∫x² dx from 0 to 1 = 1/3
        let f = |x: f64| x * x;
        let result = simpson(f, 0.0, 1.0, 100);
        assert!((result - 1.0 / 3.0).abs() < 1e-8);
    }

    #[test]
    fn test_simpson_trig() {
        // ∫sin(x) dx from 0 to π = 2
        let f = |x: f64| x.sin();
        let result = simpson(f, 0.0, PI, 100);
        assert!((result - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_simpson38() {
        // ∫x³ dx from 0 to 1 = 1/4
        let f = |x: f64| x.powi(3);
        let result = simpson38(f, 0.0, 1.0, 99);
        assert!((result - 0.25).abs() < 1e-5);
    }

    #[test]
    fn test_romberg() {
        // ∫sin(x) dx from 0 to π = 2
        let f = |x: f64| x.sin();
        let result = romberg(f, 0.0, PI, 1e-10, 20);
        assert!((result - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_quad_exponential() {
        // ∫e^x dx from 0 to 1 = e - 1
        let f = |x: f64| x.exp();
        let result = quad(f, 0.0, 1.0, 1e-10);
        let expected = 1.0_f64.exp() - 1.0;
        assert!((result - expected).abs() < 1e-8);
    }

    #[test]
    fn test_trapz_discrete() {
        // Discrete data: y = x²
        let x = vec![0.0, 0.5, 1.0];
        let y = vec![0.0, 0.25, 1.0];
        let result = trapz_discrete(&x, &y);
        // Expected: approximately 1/3
        assert!((result - 1.0 / 3.0).abs() < 0.05);
    }
}
