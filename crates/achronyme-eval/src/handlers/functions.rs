use achronyme_parser::ast::AstNode;
use achronyme_types::function::Function;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Evaluate a lambda expression
pub fn evaluate_lambda(
    evaluator: &Evaluator,
    params: &[String],
    body: &AstNode,
) -> Result<Value, String> {
    // MAJOR PERFORMANCE OPTIMIZATION:
    // Instead of calling snapshot() which copies ALL variables in the environment,
    // we now just capture an Rc<Environment> which is a simple pointer increment.
    //
    // Before: O(n) where n = total variables, expensive deep clones of all Values
    // After:  O(1) just incrementing a reference counter
    //
    // This is especially critical when you have many variables defined (like in a REPL)
    // because each new lambda would copy all 20+ variables, and nested lambdas would
    // create a quadratic explosion of memory usage.
    let closure_env = evaluator.environment().to_rc();

    // Create a Function value using the optimized constructor
    let function = Function::new_with_env(params.to_vec(), body.clone(), closure_env);

    Ok(Value::Function(function))
}

/// Apply a lambda function to arguments
pub fn apply_lambda(
    evaluator: &mut Evaluator,
    function: &Function,
    args: Vec<Value>,
) -> Result<Value, String> {
    match function {
        Function::UserDefined { params, body, closure_env } => {
            // Check arity
            if args.len() != params.len() {
                return Err(format!(
                    "Lambda expects {} arguments, got {}",
                    params.len(),
                    args.len()
                ));
            }

            // OPTIMIZATION: Instead of cloning the entire environment,
            // we just save a reference to it and swap it with the closure environment.
            // This is much faster, especially with large environments.
            let saved_env = evaluator.environment().clone();

            // Check if 'self' is defined in the current environment (for record methods)
            let self_value = saved_env.get("self").ok();

            // Set environment to the closure's captured environment
            // Since closure_env is an Rc, this clone is just incrementing a counter
            *evaluator.environment_mut() = (**closure_env).clone();

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
            for (param, arg) in params.iter().zip(args.iter()) {
                evaluator.environment_mut().define(param.clone(), arg.clone())?;
            }

            // Evaluate the body
            let result = evaluator.evaluate(body);

            // Pop the parameter scope
            evaluator.environment_mut().pop_scope();

            // Restore environment
            *evaluator.environment_mut() = saved_env;

            result
        }
        Function::Builtin(name) => {
            // Built-in functions should be dispatched to their handlers
            // Create temporary AST nodes for the arguments
            let _arg_nodes: Vec<AstNode> = args.iter().enumerate().map(|(i, _)| {
                // We can't easily convert Values back to AstNodes, so we need a different approach
                // For now, error out - this will be fixed by making the function_call handler
                // work with evaluated Values instead of AST nodes
                AstNode::VariableRef(format!("__builtin_arg_{}", i))
            }).collect();

            // For now, return an error - we need to refactor function_call handlers
            // to accept Vec<Value> instead of Vec<&AstNode>
            Err(format!(
                "Built-in function '{}' cannot yet be used as a first-class value in this context. \
                This requires refactoring the function call handlers.",
                name
            ))
        }
    }
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
