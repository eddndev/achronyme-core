use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Function Call Dispatcher
///
/// This module acts as a central dispatcher for all function calls,
/// routing them to the appropriate handler based on the function name.

pub fn dispatch(evaluator: &mut Evaluator, name: &str, args: &[AstNode]) -> Result<Value, String> {
    // Check if it's a constant (zero arguments)
    if args.is_empty() {
        if evaluator.constants().has(name) {
            return Ok(Value::Number(evaluator.constants().get(name)?));
        }
    }

    // Check if it's a lambda stored in a variable
    if evaluator.environment().has(name) {
        let var_value = evaluator.environment().get(name)?;
        if let Value::Function(ref func) = var_value {
            // Evaluate arguments
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(evaluator.evaluate(arg)?);
            }
            // Apply lambda function
            let func_clone = func.clone();
            return evaluator.apply_lambda(&func_clone, arg_values);
        }
    }

    // Check for higher-order functions (need evaluator access)
    match name {
        "map" => return super::hof::handle_map(evaluator, args),
        "filter" => return super::hof::handle_filter(evaluator, args),
        "reduce" => return super::hof::handle_reduce(evaluator, args),
        "pipe" => return super::hof::handle_pipe(evaluator, args),
        _ => {}
    }

    // Check for numerical calculus functions (need evaluator access for lambdas)
    match name {
        "diff" => return super::numerical::handle_diff(evaluator, args),
        "diff2" => return super::numerical::handle_diff2(evaluator, args),
        "diff3" => return super::numerical::handle_diff3(evaluator, args),
        "gradient" => return super::numerical::handle_gradient(evaluator, args),
        "integral" | "trapz" => return super::numerical::handle_integral(evaluator, args),
        "simpson" => return super::numerical::handle_simpson(evaluator, args),
        "romberg" => return super::numerical::handle_romberg(evaluator, args),
        "quad" => return super::numerical::handle_quad(evaluator, args),
        "solve" | "bisect" => return super::numerical::handle_solve(evaluator, args),
        "newton" => return super::numerical::handle_newton(evaluator, args),
        "secant" => return super::numerical::handle_secant(evaluator, args),
        _ => {}
    }

    // Check for optimization functions
    match name {
        "simplex" => return super::optimization::handle_simplex(evaluator, args),
        "linprog" => return super::optimization::handle_linprog(evaluator, args),
        "dual_simplex" => return super::optimization::handle_dual_simplex(evaluator, args),
        "two_phase_simplex" => return super::optimization::handle_two_phase_simplex(evaluator, args),
        "revised_simplex" => return super::optimization::handle_revised_simplex(evaluator, args),
        "objective_value" => return super::optimization::handle_objective_value(evaluator, args),
        "shadow_price" => return super::optimization::handle_shadow_price(evaluator, args),
        "sensitivity_c" => return super::optimization::handle_sensitivity_c(evaluator, args),
        "sensitivity_b" => return super::optimization::handle_sensitivity_b(evaluator, args),
        _ => {}
    }

    // Otherwise, it's a built-in function call from FunctionRegistry
    if !evaluator.functions().has(name) {
        return Err(format!("Unknown function or constant: {}", name));
    }

    // Evaluate all arguments
    let mut arg_values = Vec::new();
    for arg in args {
        arg_values.push(evaluator.evaluate(arg)?);
    }

    // Check arity (if not variadic)
    if let Some(expected_arity) = evaluator.functions().arity(name) {
        if expected_arity >= 0 && arg_values.len() != expected_arity as usize {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                name,
                expected_arity,
                arg_values.len()
            ));
        }
    }

    // Call the function
    evaluator.functions_mut().call(name, &arg_values)
}
