use achronyme_types::function::Function;
use achronyme_types::value::Value;
use achronyme_types::LambdaEvaluator;
use crate::handlers;

use super::Evaluator;

/// LambdaEvaluator trait implementation
impl LambdaEvaluator for Evaluator {
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String> {
        handlers::functions::eval_lambda_at(self, func, x)
    }

    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String> {
        // Create a vector value and apply the lambda
        let vec_arg = Value::Vector(point.iter().map(|&n| Value::Number(n)).collect());
        let result = self.apply_lambda(func, vec![vec_arg])?;

        // Extract the numeric result
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda function must return a number".to_string()),
        }
    }

    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String> {
        // Convert each f64 to a Value::Number
        let value_args: Vec<Value> = args.iter().map(|&x| Value::Number(x)).collect();

        // Apply the lambda with multiple arguments
        let result = self.apply_lambda(func, value_args)?;

        // Extract the numeric result
        match result {
            Value::Number(n) => Ok(n),
            _ => Err("Lambda function must return a number".to_string()),
        }
    }
}

/// Lambda application methods
impl Evaluator {
    /// Apply a lambda function to arguments
    pub fn apply_lambda(
        &mut self,
        function: &Function,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        handlers::functions::apply_lambda(self, function, args)
    }
}
