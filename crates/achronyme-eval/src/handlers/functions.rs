use achronyme_parser::ast::AstNode;
use achronyme_types::function::Function;
use achronyme_types::value::Value;

use crate::environment::Environment;
use crate::evaluator::Evaluator;

/// Evaluate a lambda expression
pub fn evaluate_lambda(
    evaluator: &Evaluator,
    params: &[String],
    body: &AstNode,
) -> Result<Value, String> {
    // Capture the current environment (closure)
    let captured_vars = evaluator.environment().snapshot();

    // Create a Function value
    let function = Function::new(params.to_vec(), body.clone(), captured_vars);

    Ok(Value::Function(function))
}

/// Apply a lambda function to arguments
pub fn apply_lambda(
    evaluator: &mut Evaluator,
    function: &Function,
    args: Vec<Value>,
) -> Result<Value, String> {
    // Check arity
    if args.len() != function.arity() {
        return Err(format!(
            "Lambda expects {} arguments, got {}",
            function.arity(),
            args.len()
        ));
    }

    // Save current environment
    let saved_env = evaluator.environment().clone();

    // Check if 'self' is defined in the current environment (for record methods)
    let self_value = saved_env.get("self").ok();

    // Create new environment from closure
    *evaluator.environment_mut() = Environment::from_snapshot(function.captured_vars.clone());

    // Inject the current function as 'rec' for recursive calls
    // This allows any function to reference itself using 'rec'
    evaluator.environment_mut().define("rec".to_string(), Value::Function(function.clone()))?;

    // If 'self' was available in the calling context, inject it (for record methods)
    if let Some(self_val) = self_value {
        evaluator.environment_mut().define("self".to_string(), self_val)?;
    }

    // Push a new scope for lambda parameters (enables shadowing)
    evaluator.environment_mut().push_scope();

    // Bind parameters to arguments in the new scope
    for (param, arg) in function.params.iter().zip(args.iter()) {
        evaluator.environment_mut().define(param.clone(), arg.clone())?;
    }

    // Evaluate the body
    let result = evaluator.evaluate(&function.body);

    // Pop the parameter scope
    evaluator.environment_mut().pop_scope();

    // Restore environment
    *evaluator.environment_mut() = saved_env;

    result
}

/// Helper to evaluate a lambda function at a single point
/// Used by LambdaEvaluator trait implementation
pub fn eval_lambda_at(evaluator: &mut Evaluator, func: &Function, x: f64) -> Result<f64, String> {
    let result = apply_lambda(evaluator, func, vec![Value::Number(x)])?;
    match result {
        Value::Number(n) => Ok(n),
        _ => Err("Lambda must return a number".to_string()),
    }
}
