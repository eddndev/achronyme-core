//! Lambda Evaluator Trait
//!
//! Defines the interface for evaluating lambda functions at specific points.
//! This trait is implemented by the evaluator and used by numerical calculus functions.

use crate::function::Function;

/// Trait for evaluating lambda functions
///
/// This trait allows numerical calculus functions to evaluate lambdas
/// without directly depending on the Evaluator type, enabling better
/// testability and avoiding borrow checker issues.
///
/// # Example
///
/// ```ignore
/// impl LambdaEvaluator for MyEvaluator {
///     fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String> {
///         // Evaluate the lambda function at point x
///         self.apply_lambda(func, vec![Value::Number(x)])
///     }
/// }
/// ```
pub trait LambdaEvaluator {
    /// Evaluate a lambda function at a single point
    ///
    /// # Arguments
    /// * `func` - The lambda function to evaluate
    /// * `x` - The point at which to evaluate
    ///
    /// # Returns
    /// The numeric result of evaluating `func(x)`
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String>;

    /// Evaluate a lambda function at a vector point (for multivariate functions)
    ///
    /// # Arguments
    /// * `func` - The lambda function to evaluate
    /// * `point` - The point (as a slice) at which to evaluate
    ///
    /// # Returns
    /// The numeric result of evaluating `func(point)`
    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String>;

    /// Evaluate a lambda function with multiple scalar arguments
    ///
    /// This is different from `eval_vec_at` which passes a single Vector argument.
    /// This method passes multiple Number arguments.
    ///
    /// # Arguments
    /// * `func` - The lambda function to evaluate
    /// * `args` - The arguments as individual numbers
    ///
    /// # Returns
    /// The numeric result of evaluating `func(args[0], args[1], ...)`
    ///
    /// # Example
    /// ```ignore
    /// // For a function like: (x, y) => x^2 + y^2
    /// let result = evaluator.eval_at_nd(&func, &[3.0, 4.0])?;
    /// // result = 25.0
    /// ```
    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String>;
}
