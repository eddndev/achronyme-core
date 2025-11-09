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

    // NOTE: These tests use the old closure-based API and need to be refactored
    // to use the new Evaluator + Function API. They are marked as #[ignore] for now.
    // TODO: Refactor tests to use achronyme_eval::Evaluator and Function objects

    #[test]
    #[ignore]
    fn basic_derivative() {
        // f(x) = x^2, f'(x) = 2x
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn basic_integral() {
        // ∫x dx from 0 to 1 = 0.5
        // TODO: Refactor to use Evaluator + Function
    }

    #[test]
    #[ignore]
    fn basic_root() {
        // f(x) = x^2 - 4, root at x = 2
        // TODO: Refactor to use Evaluator + Function
    }
}
