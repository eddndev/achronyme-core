//! Achronyme Numerical Calculus
//!
//! Provides numerical methods for differentiation, integration, and equation solving.
//!
//! # Modules
//! - `differentiation` - Numerical derivatives (forward, backward, central differences)
//! - `integration` - Numerical integration (trapezoid, Simpson, Romberg)
//! - `solvers` - Root finding and equation solvers (bisection, Newton, secant)

pub mod differentiation;
pub mod integration;
pub mod solvers;

// Re-exports for convenience
pub use differentiation::*;
pub use integration::*;
pub use solvers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_derivative() {
        // f(x) = x^2, f'(x) = 2x
        let f = |x: f64| x * x;
        let derivative = diff_central(&f, 2.0, 1e-5);
        assert!((derivative - 4.0).abs() < 1e-4);
    }

    #[test]
    fn basic_integral() {
        // ∫x dx from 0 to 1 = 0.5
        let f = |x: f64| x;
        let result = trapz(f, 0.0, 1.0, 1000);
        assert!((result - 0.5).abs() < 1e-3);
    }

    #[test]
    fn basic_root() {
        // f(x) = x^2 - 4, root at x = 2
        let f = |x: f64| x * x - 4.0;
        let root = bisect(f, 0.0, 5.0, 1e-6);
        assert!((root - 2.0).abs() < 1e-5);
    }
}
