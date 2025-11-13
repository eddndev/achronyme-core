use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Function Call Dispatcher
///
/// This module acts as a central dispatcher for all function calls,
/// routing them to the appropriate handler based on the function name.

pub fn dispatch(evaluator: &mut Evaluator, name: &str, args: &[AstNode]) -> Result<Value, String> {
    // Check if name contains a dot (field access like "record.field")
    if name.contains('.') {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() >= 2 {
            // Get the record from the first part
            let record_name = parts[0];

            if evaluator.environment().has(record_name) {
                let mut current_value = evaluator.environment().get(record_name)?;
                let mut parent_record = None; // Track the record containing the method

                // Navigate through nested fields
                for &field_name in parts[1..].iter() {
                    match current_value {
                        Value::Record(ref map) => {
                            // This record might be the parent of the method we're calling
                            parent_record = Some(Value::Record(map.clone()));

                            current_value = map.get(field_name)
                                .cloned()
                                .ok_or_else(|| format!("Field '{}' not found in record", field_name))?;
                        }
                        _ => return Err(format!("Cannot access field '{}' on non-record value", field_name)),
                    }
                }

                // If the final value is a function, call it with 'self' injected
                if let Value::Function(ref func) = current_value {
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(evaluator.evaluate(arg)?);
                    }

                    // If we have a parent record, inject it as 'self'
                    if let Some(record) = parent_record {
                        // Push a new scope, define 'self', call function, then pop scope
                        evaluator.environment_mut().push_scope();
                        evaluator.environment_mut().define("self".to_string(), record)?;

                        let func_clone = func.clone();
                        let result = evaluator.apply_lambda(&func_clone, arg_values);

                        evaluator.environment_mut().pop_scope();
                        return result;
                    } else {
                        // No parent record, call normally
                        let func_clone = func.clone();
                        return evaluator.apply_lambda(&func_clone, arg_values);
                    }
                }

                return Err(format!("'{}' is not a function", name));
            }
        }
    }

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
        // Tier 2 predicates
        "any" => return super::hof::handle_any(evaluator, args),
        "all" => return super::hof::handle_all(evaluator, args),
        "find" => return super::hof::handle_find(evaluator, args),
        "findIndex" => return super::hof::handle_find_index(evaluator, args),
        "count" => return super::hof::handle_count(evaluator, args),
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

    // Check for debug functions
    match name {
        "describe" => return super::debug::handle_describe(evaluator, args),
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

    // Try to resolve function through module system
    // This checks: 1) prelude, 2) imported modules, 3) global FunctionRegistry (backward compat)
    let function_info = if let Some((func, arity)) = evaluator.module_registry().resolve(name, evaluator.imported_modules()) {
        Some((func, arity))
    } else if evaluator.functions().has(name) {
        // Fallback to FunctionRegistry for backward compatibility
        evaluator.functions().get(name)
    } else {
        None
    };

    if function_info.is_none() {
        return Err(format!("Unknown function or constant: {}", name));
    }

    let (func, expected_arity) = function_info.unwrap();

    // Evaluate all arguments
    let mut arg_values = Vec::new();
    for arg in args {
        arg_values.push(evaluator.evaluate(arg)?);
    }

    // Check arity (if not variadic)
    if expected_arity >= 0 && arg_values.len() != expected_arity as usize {
        return Err(format!(
            "Function {} expects {} arguments, got {}",
            name,
            expected_arity,
            arg_values.len()
        ));
    }

    // Call the resolved function directly
    func(&arg_values)
}
